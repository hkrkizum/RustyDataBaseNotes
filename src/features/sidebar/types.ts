/** DTO returned from the `list_sidebar_items` backend command. */
export interface SidebarItem {
  id: string;
  title: string;
  itemType: "page" | "database";
  parentId: string | null;
  databaseId: string | null;
  createdAt: string;
}

/** Tree node used internally by the sidebar for rendering. */
export interface SidebarTreeNode {
  id: string;
  title: string;
  itemType: "page" | "database";
  parentId: string | null;
  databaseId: string | null;
  createdAt: string;
  children: SidebarTreeNode[];
  /** Depth in the tree (root = 1). Computed during tree building. */
  depth: number;
}

/** Data attached to a draggable sidebar item. */
export interface DragItemData {
  type: "sidebar-item";
  pageId: string;
  parentId: string | null;
  depth: number;
  itemType: "page" | "database";
  [key: string]: unknown;
}

/** Shape of the last-opened-item stored in localStorage. */
export interface LastOpenedItem {
  id: string;
  type: "page" | "database";
  title: string;
}
