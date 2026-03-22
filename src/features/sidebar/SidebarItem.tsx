import {
  draggable,
  dropTargetForElements,
} from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import type { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item";
import {
  attachInstruction,
  extractInstruction,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item";
import { ChevronRight, FileText, MoreHorizontal, Table2 } from "lucide-react";
import { type KeyboardEvent, useEffect, useRef, useState } from "react";
import { SidebarMenuButton, SidebarMenuItem } from "@/components/ui/sidebar";
import { cn } from "@/lib/utils";
import type { DragItemData, SidebarTreeNode } from "./types";

const INDENT_PER_LEVEL = 12;
const MAX_DEPTH = 5;

interface SidebarItemProps {
  node: SidebarTreeNode;
  isActive: boolean;
  hasChildren: boolean;
  isExpanded: boolean;
  isRenaming: boolean;
  isDragging: boolean;
  onClick: () => void;
  onToggleExpanded: () => void;
  onRenameSubmit: (newTitle: string) => void;
  onRenameCancel: () => void;
  onContextMenu?: (e: React.MouseEvent) => void;
  onMoreClick?: (e: React.MouseEvent) => void;
}

function isDragItemData(data: Record<string, unknown>): data is DragItemData {
  return data.type === "sidebar-item";
}

export function SidebarItem({
  node,
  isActive,
  hasChildren,
  isExpanded,
  isRenaming,
  isDragging: isDraggingGlobal,
  onClick,
  onToggleExpanded,
  onRenameSubmit,
  onRenameCancel,
  onContextMenu,
  onMoreClick,
}: SidebarItemProps) {
  const [editTitle, setEditTitle] = useState(node.title);
  const inputRef = useRef<HTMLInputElement>(null);
  const isSubmittingRef = useRef(false);
  const itemRef = useRef<HTMLLIElement>(null);
  const [dragState, setDragState] = useState<"idle" | "dragging" | "over">(
    "idle",
  );
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  const canDrag = node.itemType === "page" && !node.databaseId;

  useEffect(() => {
    if (isRenaming) {
      setEditTitle(node.title);
      isSubmittingRef.current = false;
      requestAnimationFrame(() => {
        inputRef.current?.focus();
        inputRef.current?.select();
      });
    }
  }, [isRenaming, node.title]);

  // Attach draggable
  useEffect(() => {
    const el = itemRef.current;
    if (!el || !canDrag) return;

    return draggable({
      element: el,
      getInitialData: (): DragItemData => ({
        type: "sidebar-item",
        pageId: node.id,
        parentId: node.parentId,
        depth: node.depth,
        itemType: node.itemType,
      }),
      onDragStart: () => setDragState("dragging"),
      onDrop: () => setDragState("idle"),
    });
  }, [canDrag, node.id, node.parentId, node.depth, node.itemType]);

  // Attach drop target
  useEffect(() => {
    const el = itemRef.current;
    if (!el) return;

    // Only standalone pages can be drop targets for "make-child"
    const canBeParent = node.itemType === "page" && !node.databaseId;

    return dropTargetForElements({
      element: el,
      canDrop: ({ source }) => {
        if (!isDragItemData(source.data)) return false;
        // Cannot drop on self
        if (source.data.pageId === node.id) return false;
        // Cannot drop on a database or DB-owned page
        if (node.itemType === "database" || node.databaseId) return false;
        return true;
      },
      getData: ({ input, element }) => {
        const blockedStates: string[] = [];

        // Block if this item is a database or DB-owned page
        if (!canBeParent) {
          blockedStates.push("database-page");
        }

        // Block if depth would exceed max
        if (node.depth >= MAX_DEPTH) {
          blockedStates.push("max-depth");
        }

        return attachInstruction(
          {
            pageId: node.id,
            parentId: node.parentId,
            depth: node.depth,
            itemType: node.itemType,
          },
          {
            input,
            element,
            currentLevel: node.depth - 1,
            indentPerLevel: INDENT_PER_LEVEL,
            mode: "standard",
            block:
              blockedStates.length > 0
                ? blockedStates.map(() => "make-child" as const)
                : [],
          },
        );
      },
      onDragEnter: ({ self }) => {
        setDragState("over");
        setInstruction(extractInstruction(self.data));
      },
      onDrag: ({ self }) => {
        setInstruction(extractInstruction(self.data));
      },
      onDragLeave: () => {
        setDragState("idle");
        setInstruction(null);
      },
      onDrop: () => {
        setDragState("idle");
        setInstruction(null);
      },
    });
  }, [node.id, node.parentId, node.depth, node.itemType, node.databaseId]);

  function handleConfirm() {
    if (isSubmittingRef.current) return;
    isSubmittingRef.current = true;
    const trimmed = editTitle.trim();
    if (!trimmed || trimmed === node.title) {
      onRenameCancel();
      return;
    }
    onRenameSubmit(trimmed);
  }

  function handleKeyDown(e: KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Enter") {
      e.preventDefault();
      handleConfirm();
    } else if (e.key === "Escape") {
      e.preventDefault();
      onRenameCancel();
    }
    // Prevent Cmd/Ctrl+B from toggling sidebar during inline edit
    if (e.key === "b" && (e.metaKey || e.ctrlKey)) {
      e.stopPropagation();
    }
  }

  const Icon = node.itemType === "database" ? Table2 : FileText;
  const isDragSource = dragState === "dragging";

  if (isRenaming) {
    return (
      <SidebarMenuItem ref={itemRef}>
        <div className="flex w-full items-center gap-2 rounded-md px-2 py-1.5">
          <Icon className="size-4 shrink-0 text-sidebar-foreground" />
          <input
            ref={inputRef}
            type="text"
            value={editTitle}
            onChange={(e) => setEditTitle(e.target.value)}
            onBlur={handleConfirm}
            onKeyDown={handleKeyDown}
            maxLength={255}
            className="h-6 min-w-0 flex-1 rounded border border-border bg-background px-1 text-sm outline-none focus:ring-1 focus:ring-ring"
          />
        </div>
      </SidebarMenuItem>
    );
  }

  return (
    <SidebarMenuItem
      ref={itemRef}
      data-sidebar-item-id={node.id}
      className={cn(
        "group/item relative",
        isDragSource && "opacity-40",
        instruction?.type === "make-child" && "bg-accent/50 rounded-md",
      )}
      onContextMenu={onContextMenu}
    >
      <SidebarMenuButton isActive={isActive} onClick={onClick}>
        {hasChildren ? (
          // biome-ignore lint/a11y/noStaticElementInteractions: chevron toggle is nested inside SidebarMenuButton (<button>); using <button> would create invalid nested buttons
          <span
            tabIndex={-1}
            onClick={(e) => {
              e.stopPropagation();
              onToggleExpanded();
            }}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.stopPropagation();
                e.preventDefault();
                onToggleExpanded();
              }
            }}
            className="flex size-4 shrink-0 items-center justify-center"
          >
            <ChevronRight
              className={cn(
                "size-3.5 transition-transform duration-200",
                isExpanded && "rotate-90",
              )}
            />
          </span>
        ) : (
          <span className="w-4 shrink-0" />
        )}
        <Icon className="size-4 shrink-0" />
        <span className="truncate">{node.title}</span>
      </SidebarMenuButton>
      {!isDraggingGlobal && onMoreClick && (
        // biome-ignore lint/a11y/noStaticElementInteractions: the "..." button is mouse-triggered, keyboard users use context menu
        // biome-ignore lint/a11y/useKeyWithClickEvents: the "..." button is mouse-triggered, keyboard users use context menu
        <span
          className="absolute top-1/2 right-1 flex -translate-y-1/2 opacity-0 group-hover/item:opacity-100"
          onClick={(e) => {
            e.stopPropagation();
            onMoreClick(e);
          }}
        >
          <MoreHorizontal className="size-4 text-muted-foreground hover:text-foreground" />
        </span>
      )}
    </SidebarMenuItem>
  );
}
