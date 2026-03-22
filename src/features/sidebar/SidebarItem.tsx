import {
  draggable,
  dropTargetForElements,
} from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import type { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item";
import {
  attachInstruction,
  extractInstruction,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item";
import { invoke } from "@tauri-apps/api/core";
import { ChevronRight, FileText, MoreHorizontal, Table2 } from "lucide-react";
import {
  type KeyboardEvent,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import { toast } from "sonner";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
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
  onChildCreated: (
    page: { id: string; title: string },
    parentId: string,
  ) => void;
  onRenameStart: (id: string) => void;
  onDeleted: (id: string) => void;
}

function isDragItemData(data: Record<string, unknown>): data is DragItemData {
  return data.type === "sidebar-item";
}

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as { message: string }).message;
  }
  return String(err);
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
  onChildCreated,
  onRenameStart,
  onDeleted,
}: SidebarItemProps) {
  const [editTitle, setEditTitle] = useState(node.title);
  const inputRef = useRef<HTMLInputElement>(null);
  const isSubmittingRef = useRef(false);
  const itemRef = useRef<HTMLLIElement>(null);
  const [dragState, setDragState] = useState<"idle" | "dragging" | "over">(
    "idle",
  );
  const [instruction, setInstruction] = useState<Instruction | null>(null);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);

  const canDrag = node.itemType === "page" && !node.databaseId;
  const canCreateChild =
    node.itemType === "page" && !node.databaseId && node.depth < MAX_DEPTH;

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

    const canBeParent = node.itemType === "page" && !node.databaseId;

    return dropTargetForElements({
      element: el,
      canDrop: ({ source }) => {
        if (!isDragItemData(source.data)) return false;
        if (source.data.pageId === node.id) return false;
        if (node.itemType === "database" || node.databaseId) return false;
        return true;
      },
      getData: ({ input, element }) => {
        const blockedInstructions: Instruction["type"][] = [];

        if (!canBeParent || node.depth >= MAX_DEPTH) {
          blockedInstructions.push("make-child");
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
            mode: "expanded",
            block: blockedInstructions,
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

  // --- Menu action handlers ---

  const handleCreateChild = useCallback(async () => {
    try {
      const page = await invoke<{ id: string; title: string }>(
        "create_child_page",
        { parentId: node.id, title: "無題" },
      );
      onChildCreated(page, node.id);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, [node.id, onChildCreated]);

  const handleRename = useCallback(() => {
    onRenameStart(node.id);
  }, [node.id, onRenameStart]);

  const handleDeleteConfirm = useCallback(async () => {
    setDeleteDialogOpen(false);
    try {
      const command =
        node.itemType === "database" ? "delete_database" : "delete_page";
      await invoke(command, { id: node.id });
      onDeleted(node.id);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, [node.id, node.itemType, onDeleted]);

  // --- Rename handlers ---

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
    if (e.key === "b" && (e.metaKey || e.ctrlKey)) {
      e.stopPropagation();
    }
  }

  const Icon = node.itemType === "database" ? Table2 : FileText;
  const isDragSource = dragState === "dragging";
  // During a drag, items that cannot accept children are dimmed.
  const canAcceptDrop =
    node.itemType === "page" && !node.databaseId && node.depth < MAX_DEPTH;
  const isDimmed = isDraggingGlobal && !isDragSource && !canAcceptDrop;

  const deleteMessage =
    node.children.length > 0
      ? `「${node.title}」を削除しますか？子ページはルートレベルに昇格されます。`
      : `「${node.title}」を削除しますか？`;

  const menuItems = (
    <>
      {canCreateChild && (
        <ContextMenuItem onClick={handleCreateChild}>
          子ページ作成
        </ContextMenuItem>
      )}
      <ContextMenuItem onClick={handleRename}>名前変更</ContextMenuItem>
      <ContextMenuItem
        onClick={() => setDeleteDialogOpen(true)}
        className="text-destructive focus:text-destructive"
      >
        削除
      </ContextMenuItem>
    </>
  );

  const dropdownMenuItems = (
    <>
      {canCreateChild && (
        <DropdownMenuItem onClick={handleCreateChild}>
          子ページ作成
        </DropdownMenuItem>
      )}
      <DropdownMenuItem onClick={handleRename}>名前変更</DropdownMenuItem>
      <DropdownMenuItem
        onClick={() => setDeleteDialogOpen(true)}
        className="text-destructive focus:text-destructive"
      >
        削除
      </DropdownMenuItem>
    </>
  );

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
    <>
      <ContextMenu>
        <ContextMenuTrigger>
          <SidebarMenuItem
            ref={itemRef}
            data-sidebar-item-id={node.id}
            className={cn(
              "group/item relative transition-opacity duration-150",
              isDragSource && "opacity-30",
              isDimmed && "opacity-30",
              instruction?.type === "make-child" &&
                "bg-accent/60 rounded-md opacity-100! ring-1 ring-primary/30",
            )}
          >
            <SidebarMenuButton isActive={isActive} onClick={onClick}>
              {hasChildren ? (
                // biome-ignore lint/a11y/noStaticElementInteractions: chevron toggle nested inside button
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
            {!isDraggingGlobal && (
              <DropdownMenu>
                <DropdownMenuTrigger
                  render={
                    // biome-ignore lint/a11y/noStaticElementInteractions: mouse-only trigger
                    // biome-ignore lint/a11y/useKeyWithClickEvents: mouse-only trigger
                    <span
                      className="absolute top-1/2 right-1 flex -translate-y-1/2 cursor-pointer opacity-0 group-hover/item:opacity-100"
                      onClick={(e) => e.stopPropagation()}
                    >
                      <MoreHorizontal className="size-4 text-muted-foreground hover:text-foreground" />
                    </span>
                  }
                />
                <DropdownMenuContent align="start" side="bottom">
                  {dropdownMenuItems}
                </DropdownMenuContent>
              </DropdownMenu>
            )}
          </SidebarMenuItem>
        </ContextMenuTrigger>
        <ContextMenuContent>{menuItems}</ContextMenuContent>
      </ContextMenu>
      <AlertDialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>削除の確認</AlertDialogTitle>
            <AlertDialogDescription>{deleteMessage}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>キャンセル</AlertDialogCancel>
            <AlertDialogAction onClick={handleDeleteConfirm}>
              削除
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}
