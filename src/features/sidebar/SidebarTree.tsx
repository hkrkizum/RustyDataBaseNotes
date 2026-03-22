import { Collapsible, CollapsibleContent } from "@/components/ui/collapsible";
import { SidebarMenuSub } from "@/components/ui/sidebar";
import { SidebarItem } from "./SidebarItem";
import type { SidebarTreeNode } from "./types";

interface SidebarTreeProps {
  nodes: SidebarTreeNode[];
  activeItemId: string | null;
  expandedState: Record<string, boolean>;
  renamingItemId: string | null;
  onToggleExpanded: (id: string) => void;
  onItemClick: (node: SidebarTreeNode) => void;
  onRenameSubmit: (id: string, newTitle: string) => void;
  onRenameCancel: () => void;
}

export function SidebarTree({
  nodes,
  activeItemId,
  expandedState,
  renamingItemId,
  onToggleExpanded,
  onItemClick,
  onRenameSubmit,
  onRenameCancel,
}: SidebarTreeProps) {
  return (
    <>
      {nodes.map((node) => {
        const hasChildren = node.children.length > 0;
        const isExpanded = expandedState[node.id] ?? false;
        const isActive = activeItemId === node.id;
        const isRenaming = renamingItemId === node.id;

        if (hasChildren) {
          return (
            <Collapsible key={node.id} open={isExpanded}>
              <SidebarItem
                node={node}
                isActive={isActive}
                hasChildren
                isExpanded={isExpanded}
                isRenaming={isRenaming}
                onClick={() => onItemClick(node)}
                onToggleExpanded={() => onToggleExpanded(node.id)}
                onRenameSubmit={(title) => onRenameSubmit(node.id, title)}
                onRenameCancel={onRenameCancel}
              />
              <CollapsibleContent>
                <SidebarMenuSub>
                  <SidebarTree
                    nodes={node.children}
                    activeItemId={activeItemId}
                    expandedState={expandedState}
                    renamingItemId={renamingItemId}
                    onToggleExpanded={onToggleExpanded}
                    onItemClick={onItemClick}
                    onRenameSubmit={onRenameSubmit}
                    onRenameCancel={onRenameCancel}
                  />
                </SidebarMenuSub>
              </CollapsibleContent>
            </Collapsible>
          );
        }

        return (
          <SidebarItem
            key={node.id}
            node={node}
            isActive={isActive}
            hasChildren={false}
            isExpanded={false}
            isRenaming={isRenaming}
            onClick={() => onItemClick(node)}
            onToggleExpanded={() => {}}
            onRenameSubmit={(title) => onRenameSubmit(node.id, title)}
            onRenameCancel={onRenameCancel}
          />
        );
      })}
    </>
  );
}
