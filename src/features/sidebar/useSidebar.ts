import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";
import { toast } from "sonner";
import { readFromStorage } from "../../hooks/useLocalStorage";
import type { SidebarItem, SidebarTreeNode } from "./types";

const EXPANDED_KEY = "sidebar-expanded";

type ExpandedState = Record<string, boolean>;

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as { message: string }).message;
  }
  return String(err);
}

/**
 * Build a tree from a flat list of sidebar items.
 *
 * - Standalone pages with `parentId` become children of their parent page.
 * - Pages with `databaseId` become children of the matching database item.
 * - Root-level items (no parentId, no databaseId) stay at the top level.
 * - Items are sorted by `createdAt` DESC within each level.
 */
function buildTree(items: SidebarItem[]): SidebarTreeNode[] {
  const nodeMap = new Map<string, SidebarTreeNode>();

  // Create nodes
  for (const item of items) {
    nodeMap.set(item.id, { ...item, children: [], depth: 1 });
  }

  const roots: SidebarTreeNode[] = [];

  for (const item of items) {
    const node = nodeMap.get(item.id);
    if (!node) continue;

    // DB-owned pages go under their database
    if (item.databaseId && nodeMap.has(item.databaseId)) {
      nodeMap.get(item.databaseId)?.children.push(node);
      continue;
    }

    // Pages with a parent go under their parent
    if (item.parentId && nodeMap.has(item.parentId)) {
      nodeMap.get(item.parentId)?.children.push(node);
      continue;
    }

    // Everything else is root-level
    roots.push(node);
  }

  // Sort each level by createdAt DESC and assign depth
  const sortDesc = (a: SidebarTreeNode, b: SidebarTreeNode) =>
    b.createdAt.localeCompare(a.createdAt);

  function sortAndAssignDepth(nodes: SidebarTreeNode[], depth: number) {
    nodes.sort(sortDesc);
    for (const node of nodes) {
      node.depth = depth;
      if (node.children.length > 0) {
        sortAndAssignDepth(node.children, depth + 1);
      }
    }
  }

  sortAndAssignDepth(roots, 1);

  return roots;
}

/**
 * Hook for sidebar data management: fetches items, builds tree, manages
 * expand/collapse state and active item tracking.
 */
export function useSidebar(initialActiveItemId?: string | null) {
  const [items, setItems] = useState<SidebarItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeItemId, setActiveItemId] = useState<string | null>(
    initialActiveItemId ?? null,
  );
  const [expandedState, setExpandedState] = useState<ExpandedState>(() =>
    readFromStorage<ExpandedState>(EXPANDED_KEY, {}),
  );

  const fetchItems = useCallback(async () => {
    try {
      const result = await invoke<SidebarItem[]>("list_sidebar_items");
      setItems(result);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void fetchItems();
  }, [fetchItems]);

  const tree = buildTree(items);

  const toggleExpanded = useCallback((id: string) => {
    setExpandedState((prev) => {
      const next = { ...prev, [id]: !prev[id] };
      try {
        window.localStorage.setItem(EXPANDED_KEY, JSON.stringify(next));
      } catch {
        // localStorage unavailable
      }
      return next;
    });
  }, []);

  const setExpanded = useCallback((id: string, expanded: boolean) => {
    setExpandedState((prev) => {
      const next = { ...prev, [id]: expanded };
      try {
        window.localStorage.setItem(EXPANDED_KEY, JSON.stringify(next));
      } catch {
        // localStorage unavailable
      }
      return next;
    });
  }, []);

  const refreshItems = useCallback(async () => {
    try {
      const result = await invoke<SidebarItem[]>("list_sidebar_items");
      setItems(result);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, []);

  // Auto-expand ancestors of the active item on startup
  useEffect(() => {
    if (loading || !initialActiveItemId) return;

    // Find all ancestors of the active item and expand them
    const ancestors = findAncestorIds(items, initialActiveItemId);
    if (ancestors.length === 0) return;

    setExpandedState((prev) => {
      const next = { ...prev };
      let changed = false;
      for (const ancestorId of ancestors) {
        if (!next[ancestorId]) {
          next[ancestorId] = true;
          changed = true;
        }
      }
      if (!changed) return prev;
      try {
        window.localStorage.setItem(EXPANDED_KEY, JSON.stringify(next));
      } catch {
        // localStorage unavailable
      }
      return next;
    });

    // Scroll to the active item after expansion
    requestAnimationFrame(() => {
      const el = document.querySelector(
        `[data-sidebar-item-id="${initialActiveItemId}"]`,
      );
      el?.scrollIntoView({ behavior: "smooth", block: "nearest" });
    });
  }, [loading, initialActiveItemId, items]);

  return {
    items,
    tree,
    loading,
    activeItemId,
    setActiveItemId,
    expandedState,
    toggleExpanded,
    setExpanded,
    refreshItems,
    setItems,
  };
}

/**
 * Walk the flat items list to find all ancestor IDs of a given item,
 * from the immediate parent up to the root.
 */
function findAncestorIds(items: SidebarItem[], itemId: string): string[] {
  const ancestors: string[] = [];
  const itemMap = new Map<string, SidebarItem>();
  for (const item of items) {
    itemMap.set(item.id, item);
  }

  let currentId = itemId;
  for (let i = 0; i < 10; i++) {
    const item = itemMap.get(currentId);
    if (!item) break;

    // Check parentId (page hierarchy)
    if (item.parentId && itemMap.has(item.parentId)) {
      ancestors.push(item.parentId);
      currentId = item.parentId;
      continue;
    }
    // Check databaseId (DB-owned page)
    if (item.databaseId && itemMap.has(item.databaseId)) {
      ancestors.push(item.databaseId);
      break;
    }
    break;
  }

  return ancestors;
}
