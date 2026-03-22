import { ChevronRight, FileText, Table2 } from "lucide-react";
import { type KeyboardEvent, useEffect, useRef, useState } from "react";
import { SidebarMenuButton, SidebarMenuItem } from "@/components/ui/sidebar";
import { cn } from "@/lib/utils";
import type { SidebarTreeNode } from "./types";

interface SidebarItemProps {
  node: SidebarTreeNode;
  isActive: boolean;
  hasChildren: boolean;
  isExpanded: boolean;
  isRenaming: boolean;
  onClick: () => void;
  onToggleExpanded: () => void;
  onRenameSubmit: (newTitle: string) => void;
  onRenameCancel: () => void;
}

export function SidebarItem({
  node,
  isActive,
  hasChildren,
  isExpanded,
  isRenaming,
  onClick,
  onToggleExpanded,
  onRenameSubmit,
  onRenameCancel,
}: SidebarItemProps) {
  const [editTitle, setEditTitle] = useState(node.title);
  const inputRef = useRef<HTMLInputElement>(null);
  const isSubmittingRef = useRef(false);

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

  if (isRenaming) {
    return (
      <SidebarMenuItem>
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
    <SidebarMenuItem>
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
    </SidebarMenuItem>
  );
}
