import {
  dropTargetForElements,
  monitorForElements,
} from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import type { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item";
import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item";
import { useEffect, useRef, useState } from "react";
import { Collapsible, CollapsibleContent } from "@/components/ui/collapsible";
import { SidebarMenuSub } from "@/components/ui/sidebar";
import { SidebarItem } from "./SidebarItem";
import type { DragItemData, SidebarTreeNode } from "./types";

interface SidebarTreeProps {
  nodes: SidebarTreeNode[];
  activeItemId: string | null;
  expandedState: Record<string, boolean>;
  renamingItemId: string | null;
  isDragging: boolean;
  onToggleExpanded: (id: string) => void;
  onItemClick: (node: SidebarTreeNode) => void;
  onRenameSubmit: (id: string, newTitle: string) => void;
  onRenameCancel: () => void;
  onMovePage: (pageId: string, newParentId: string | null) => void;
  onContextMenu?: (node: SidebarTreeNode, e: React.MouseEvent) => void;
  onMoreClick?: (node: SidebarTreeNode, e: React.MouseEvent) => void;
}

function isDragItemData(data: Record<string, unknown>): data is DragItemData {
  return data.type === "sidebar-item";
}

function SidebarTreeNodes({
  nodes,
  activeItemId,
  expandedState,
  renamingItemId,
  isDragging,
  onToggleExpanded,
  onItemClick,
  onRenameSubmit,
  onRenameCancel,
  onContextMenu,
  onMoreClick,
}: Omit<SidebarTreeProps, "onMovePage">) {
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
                isDragging={isDragging}
                onClick={() => onItemClick(node)}
                onToggleExpanded={() => onToggleExpanded(node.id)}
                onRenameSubmit={(title) => onRenameSubmit(node.id, title)}
                onRenameCancel={onRenameCancel}
                onContextMenu={
                  onContextMenu ? (e) => onContextMenu(node, e) : undefined
                }
                onMoreClick={
                  onMoreClick ? (e) => onMoreClick(node, e) : undefined
                }
              />
              <CollapsibleContent>
                <SidebarMenuSub>
                  <SidebarTreeNodes
                    nodes={node.children}
                    activeItemId={activeItemId}
                    expandedState={expandedState}
                    renamingItemId={renamingItemId}
                    isDragging={isDragging}
                    onToggleExpanded={onToggleExpanded}
                    onItemClick={onItemClick}
                    onRenameSubmit={onRenameSubmit}
                    onRenameCancel={onRenameCancel}
                    onContextMenu={onContextMenu}
                    onMoreClick={onMoreClick}
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
            isDragging={isDragging}
            onClick={() => onItemClick(node)}
            onToggleExpanded={() => {}}
            onRenameSubmit={(title) => onRenameSubmit(node.id, title)}
            onRenameCancel={onRenameCancel}
            onContextMenu={
              onContextMenu ? (e) => onContextMenu(node, e) : undefined
            }
            onMoreClick={onMoreClick ? (e) => onMoreClick(node, e) : undefined}
          />
        );
      })}
    </>
  );
}

export function SidebarTree({
  nodes,
  activeItemId,
  expandedState,
  renamingItemId,
  isDragging,
  onToggleExpanded,
  onItemClick,
  onRenameSubmit,
  onRenameCancel,
  onMovePage,
  onContextMenu,
  onMoreClick,
}: SidebarTreeProps) {
  const rootDropRef = useRef<HTMLDivElement>(null);
  const [rootDropActive, setRootDropActive] = useState(false);

  // Monitor for drop events across the entire tree.
  useEffect(() => {
    return monitorForElements({
      canMonitor: ({ source }) => isDragItemData(source.data),
      onDrop: ({ source, location }) => {
        if (!isDragItemData(source.data)) return;

        const target = location.current.dropTargets[0];
        if (!target) return;

        const targetData = target.data as Record<string, unknown>;

        // Check for root drop zone
        if (targetData.rootDropZone === true) {
          onMovePage(source.data.pageId, null);
          return;
        }

        const instruction: Instruction | null = extractInstruction(targetData);
        if (!instruction) return;

        const targetPageId = targetData.pageId as string | undefined;
        const targetParentId = targetData.parentId as string | null | undefined;

        if (!targetPageId) return;

        switch (instruction.type) {
          case "make-child":
            onMovePage(source.data.pageId, targetPageId);
            break;
          case "reorder-above":
          case "reorder-below":
            // Reparent to target's parent (same-parent reorder is future scope)
            onMovePage(source.data.pageId, targetParentId ?? null);
            break;
          case "instruction-blocked":
            // Do nothing — drop was blocked
            break;
        }
      },
    });
  }, [onMovePage]);

  // Root-level drop zone for promoting pages to root.
  useEffect(() => {
    const el = rootDropRef.current;
    if (!el) return;

    return dropTargetForElements({
      element: el,
      canDrop: ({ source }) => isDragItemData(source.data),
      getData: () => ({ rootDropZone: true }),
      onDragEnter: () => setRootDropActive(true),
      onDragLeave: () => setRootDropActive(false),
      onDrop: () => setRootDropActive(false),
    });
  }, []);

  return (
    <>
      <SidebarTreeNodes
        nodes={nodes}
        activeItemId={activeItemId}
        expandedState={expandedState}
        renamingItemId={renamingItemId}
        isDragging={isDragging}
        onToggleExpanded={onToggleExpanded}
        onItemClick={onItemClick}
        onRenameSubmit={onRenameSubmit}
        onRenameCancel={onRenameCancel}
        onContextMenu={onContextMenu}
        onMoreClick={onMoreClick}
      />
      <div
        ref={rootDropRef}
        className={`min-h-8 flex-1 ${rootDropActive ? "border-t-2 border-primary" : ""}`}
      />
    </>
  );
}
