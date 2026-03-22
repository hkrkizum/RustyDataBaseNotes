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
}

/** Shape of the last-opened-item stored in localStorage. */
export interface LastOpenedItem {
  id: string;
  type: "page" | "database";
  title: string;
}
