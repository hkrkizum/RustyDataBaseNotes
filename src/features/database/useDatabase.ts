import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";
import { toast } from "sonner";
import type { CommandError, DatabaseDto } from "./types";

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as CommandError).message;
  }
  return String(err);
}

export function useDatabase() {
  const [databases, setDatabases] = useState<DatabaseDto[]>([]);
  const [loading, setLoading] = useState(true);

  const refreshDatabases = useCallback(async () => {
    try {
      const result = await invoke<DatabaseDto[]>("list_databases");
      setDatabases(result);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refreshDatabases();
  }, [refreshDatabases]);

  const createDatabase = useCallback(
    async (title: string): Promise<DatabaseDto | null> => {
      try {
        const db = await invoke<DatabaseDto>("create_database", { title });
        setDatabases((prev) => [db, ...prev]);
        toast.success("データベースを作成しました");
        return db;
      } catch (err) {
        toast.error(errorMessage(err));
        return null;
      }
    },
    [],
  );

  const getDatabase = useCallback(
    async (id: string): Promise<DatabaseDto | null> => {
      try {
        return await invoke<DatabaseDto>("get_database", { id });
      } catch (err) {
        toast.error(errorMessage(err));
        return null;
      }
    },
    [],
  );

  const updateDatabaseTitle = useCallback(
    async (id: string, title: string): Promise<DatabaseDto | null> => {
      try {
        const db = await invoke<DatabaseDto>("update_database_title", {
          id,
          title,
        });
        setDatabases((prev) => prev.map((d) => (d.id === id ? db : d)));
        toast.success("データベース名を更新しました");
        return db;
      } catch (err) {
        toast.error(errorMessage(err));
        return null;
      }
    },
    [],
  );

  const deleteDatabase = useCallback(async (id: string): Promise<boolean> => {
    try {
      await invoke("delete_database", { id });
      setDatabases((prev) => prev.filter((d) => d.id !== id));
      toast.success("データベースを削除しました");
      return true;
    } catch (err) {
      toast.error(errorMessage(err));
      return false;
    }
  }, []);

  return {
    databases,
    loading,
    createDatabase,
    getDatabase,
    updateDatabaseTitle,
    deleteDatabase,
    refreshDatabases,
  };
}
