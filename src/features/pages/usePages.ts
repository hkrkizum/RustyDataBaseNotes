import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import type { Page, CommandError } from "./types";

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as CommandError).message;
  }
  return String(err);
}

export function usePages() {
  const [pages, setPages] = useState<Page[]>([]);
  const [loading, setLoading] = useState(true);

  const refreshPages = useCallback(async () => {
    try {
      const result = await invoke<Page[]>("list_pages");
      setPages(result);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refreshPages();
  }, [refreshPages]);

  const createPage = useCallback(
    async (title: string): Promise<Page | null> => {
      try {
        const page = await invoke<Page>("create_page", { title });
        setPages((prev) => [page, ...prev]);
        toast.success("ページを作成しました");
        return page;
      } catch (err) {
        toast.error(errorMessage(err));
        return null;
      }
    },
    [],
  );

  const updatePageTitle = useCallback(
    async (id: string, title: string): Promise<Page | null> => {
      try {
        const page = await invoke<Page>("update_page_title", { id, title });
        setPages((prev) => prev.map((p) => (p.id === id ? page : p)));
        toast.success("タイトルを更新しました");
        return page;
      } catch (err) {
        toast.error(errorMessage(err));
        return null;
      }
    },
    [],
  );

  const deletePage = useCallback(async (id: string): Promise<boolean> => {
    try {
      await invoke("delete_page", { id });
      setPages((prev) => prev.filter((p) => p.id !== id));
      toast.success("ページを削除しました");
      return true;
    } catch (err) {
      toast.error(errorMessage(err));
      return false;
    }
  }, []);

  return {
    pages,
    loading,
    createPage,
    updatePageTitle,
    deletePage,
    refreshPages,
  };
}
