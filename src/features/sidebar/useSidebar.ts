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
    nodeMap.set(item.id, { ...item, children: [] });
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

  // Sort each level by createdAt DESC
  const sortDesc = (a: SidebarTreeNode, b: SidebarTreeNode) =>
    b.createdAt.localeCompare(a.createdAt);

  function sortRecursive(nodes: SidebarTreeNode[]) {
    nodes.sort(sortDesc);
    for (const node of nodes) {
      if (node.children.length > 0) {
        sortRecursive(node.children);
      }
    }
  }

  sortRecursive(roots);

  return roots;
}

/**
 * Hook for sidebar data management: fetches items, builds tree, manages
 * expand/collapse state and active item tracking.
 */
export function useSidebar() {
  const [items, setItems] = useState<SidebarItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeItemId, setActiveItemId] = useState<string | null>(null);
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
