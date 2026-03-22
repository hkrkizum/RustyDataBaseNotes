import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Sidebar as ShadcnSidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { SidebarCreateButton } from "./SidebarCreateButton";
import { SidebarTree } from "./SidebarTree";
import type { SidebarItem, SidebarTreeNode } from "./types";
import { useSidebar as useSidebarData } from "./useSidebar";

interface AppSidebarProps {
  initialActiveItemId?: string | null;
  onPageClick: (pageId: string, pageTitle: string) => void;
  onDatabaseClick: (databaseId: string, databaseTitle: string) => void;
  onItemsLoaded?: (items: SidebarItem[]) => void;
}

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as { message: string }).message;
  }
  return String(err);
}

export function AppSidebar({
  initialActiveItemId,
  onPageClick,
  onDatabaseClick,
  onItemsLoaded,
}: AppSidebarProps) {
  const {
    items,
    tree,
    loading,
    activeItemId,
    setActiveItemId,
    expandedState,
    toggleExpanded,
    refreshItems,
    setItems,
  } = useSidebarData(initialActiveItemId);

  const [renamingItemId, setRenamingItemId] = useState<string | null>(null);
  const itemsLoadedRef = useRef(false);

  // Notify parent when items finish loading (runs once)
  useEffect(() => {
    if (!loading && !itemsLoadedRef.current) {
      itemsLoadedRef.current = true;
      onItemsLoaded?.(items);
    }
  }, [loading, items, onItemsLoaded]);

  const handleItemClick = useCallback(
    (node: SidebarTreeNode) => {
      setActiveItemId(node.id);
      if (node.itemType === "page") {
        onPageClick(node.id, node.title);
      } else {
        onDatabaseClick(node.id, node.title);
      }
    },
    [setActiveItemId, onPageClick, onDatabaseClick],
  );

  const handleRenameSubmit = useCallback(
    async (id: string, newTitle: string) => {
      setRenamingItemId(null);

      // Determine which IPC to call based on item type
      const item = findItemInTree(tree, id);
      const command =
        item?.itemType === "database"
          ? "update_database_title"
          : "update_page_title";

      try {
        await invoke(command, { id, title: newTitle });
        // Optimistic update
        setItems((prev) =>
          prev.map((i) => (i.id === id ? { ...i, title: newTitle } : i)),
        );
      } catch (err) {
        toast.error(errorMessage(err));
        await refreshItems();
      }
    },
    [tree, setItems, refreshItems],
  );

  const handleRenameCancel = useCallback(() => {
    setRenamingItemId(null);
  }, []);

  const handlePageCreated = useCallback(
    async (page: { id: string; title: string }) => {
      await refreshItems();
      setActiveItemId(page.id);
      setRenamingItemId(page.id);
      onPageClick(page.id, page.title);
    },
    [refreshItems, setActiveItemId, onPageClick],
  );

  const handleDatabaseCreated = useCallback(
    async (db: { id: string; title: string }) => {
      await refreshItems();
      setActiveItemId(db.id);
      setRenamingItemId(db.id);
      onDatabaseClick(db.id, db.title);
    },
    [refreshItems, setActiveItemId, onDatabaseClick],
  );

  return (
    <ShadcnSidebar>
      <SidebarHeader>
        <div className="flex items-center justify-between">
          <span className="text-sm font-semibold">RustyDataBaseNotes</span>
          <div className="flex items-center gap-1">
            <SidebarCreateButton
              onPageCreated={handlePageCreated}
              onDatabaseCreated={handleDatabaseCreated}
            />
            <SidebarTrigger />
          </div>
        </div>
      </SidebarHeader>
      <SidebarContent>
        <ScrollArea className="flex-1">
          <SidebarGroup>
            <SidebarGroupContent>
              <SidebarMenu>
                {loading ? (
                  <li className="px-2 py-4 text-center text-sm text-muted-foreground">
                    読み込み中...
                  </li>
                ) : (
                  <SidebarTree
                    nodes={tree}
                    activeItemId={activeItemId}
                    expandedState={expandedState}
                    renamingItemId={renamingItemId}
                    onToggleExpanded={toggleExpanded}
                    onItemClick={handleItemClick}
                    onRenameSubmit={handleRenameSubmit}
                    onRenameCancel={handleRenameCancel}
                  />
                )}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </ScrollArea>
      </SidebarContent>
    </ShadcnSidebar>
  );
}

/** Find a node by ID in a nested tree structure. */
function findItemInTree(
  nodes: SidebarTreeNode[],
  id: string,
): SidebarTreeNode | undefined {
  for (const node of nodes) {
    if (node.id === id) return node;
    if (node.children.length > 0) {
      const found = findItemInTree(node.children, id);
      if (found) return found;
    }
  }
  return undefined;
}
