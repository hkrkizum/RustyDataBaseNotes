import { invoke } from "@tauri-apps/api/core";
import { FileText, Plus, Table2 } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

interface SidebarCreateButtonProps {
  onPageCreated: (page: { id: string; title: string }) => void;
  onDatabaseCreated: (db: { id: string; title: string }) => void;
}

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as { message: string }).message;
  }
  return String(err);
}

export function SidebarCreateButton({
  onPageCreated,
  onDatabaseCreated,
}: SidebarCreateButtonProps) {
  async function handleCreatePage() {
    try {
      const page = await invoke<{ id: string; title: string }>("create_page", {
        title: "無題",
      });
      onPageCreated(page);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }

  async function handleCreateDatabase() {
    try {
      const db = await invoke<{ id: string; title: string }>(
        "create_database",
        { title: "無題" },
      );
      onDatabaseCreated(db);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={
          <Button variant="ghost" size="icon-sm">
            <Plus className="size-4" />
            <span className="sr-only">新規作成</span>
          </Button>
        }
      />
      <DropdownMenuContent side="bottom" align="start">
        <DropdownMenuItem onClick={handleCreatePage}>
          <FileText className="size-4" />
          ページ
        </DropdownMenuItem>
        <DropdownMenuItem onClick={handleCreateDatabase}>
          <Table2 className="size-4" />
          データベース
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
