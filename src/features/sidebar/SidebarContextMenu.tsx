import { invoke } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";
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
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import type { SidebarTreeNode } from "./types";

const MAX_DEPTH = 5;

interface SidebarContextMenuProps {
  node: SidebarTreeNode;
  isDragging: boolean;
  open: boolean;
  position: { x: number; y: number };
  onOpenChange: (open: boolean) => void;
  onChildCreated: (
    page: { id: string; title: string },
    parentId: string,
  ) => void;
  onRenameStart: (id: string) => void;
  onDeleted: (id: string) => void;
}

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as { message: string }).message;
  }
  return String(err);
}

export function SidebarContextMenu({
  node,
  isDragging,
  open,
  position,
  onOpenChange,
  onChildCreated,
  onRenameStart,
  onDeleted,
}: SidebarContextMenuProps) {
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);

  const canCreateChild =
    node.itemType === "page" && !node.databaseId && node.depth < MAX_DEPTH;

  const handleCreateChild = useCallback(async () => {
    onOpenChange(false);
    try {
      const page = await invoke<{ id: string; title: string }>(
        "create_child_page",
        { parentId: node.id, title: "無題" },
      );
      onChildCreated(page, node.id);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, [node.id, onChildCreated, onOpenChange]);

  const handleRename = useCallback(() => {
    onOpenChange(false);
    onRenameStart(node.id);
  }, [node.id, onRenameStart, onOpenChange]);

  const handleDeleteClick = useCallback(() => {
    onOpenChange(false);
    setDeleteDialogOpen(true);
  }, [onOpenChange]);

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

  const hasChildren = node.children.length > 0;
  const deleteMessage = hasChildren
    ? `「${node.title}」を削除しますか？子ページはルートレベルに昇格されます。`
    : `「${node.title}」を削除しますか？`;

  if (isDragging) return null;

  return (
    <>
      <DropdownMenu open={open} onOpenChange={onOpenChange}>
        <DropdownMenuTrigger
          render={
            <span
              className="pointer-events-none fixed h-0 w-0 opacity-0"
              style={{ top: position.y, left: position.x }}
            />
          }
        />
        <DropdownMenuContent align="start" side="bottom">
          {canCreateChild && (
            <DropdownMenuItem onClick={handleCreateChild}>
              子ページ作成
            </DropdownMenuItem>
          )}
          <DropdownMenuItem onClick={handleRename}>名前変更</DropdownMenuItem>
          <DropdownMenuItem
            onClick={handleDeleteClick}
            className="text-destructive focus:text-destructive"
          >
            削除
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
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
