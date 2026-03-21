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

  return {
    databases,
    loading,
    createDatabase,
    getDatabase,
    refreshDatabases,
  };
}
