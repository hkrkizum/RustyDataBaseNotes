import { invoke } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";
import { toast } from "sonner";
import type { Page } from "../pages/types";
import type {
  CommandError,
  PropertyConfigDto,
  PropertyDto,
  PropertyTypeDto,
} from "./types";

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as CommandError).message;
  }
  return String(err);
}

export function useTableData(databaseId: string) {
  const [properties, setProperties] = useState<PropertyDto[]>([]);

  const listProperties = useCallback(async () => {
    try {
      const result = await invoke<PropertyDto[]>("list_properties", {
        databaseId,
      });
      setProperties(result);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, [databaseId]);

  const addProperty = useCallback(
    async (
      name: string,
      propertyType: PropertyTypeDto,
      config?: PropertyConfigDto,
    ): Promise<PropertyDto | null> => {
      try {
        const prop = await invoke<PropertyDto>("add_property", {
          databaseId,
          name,
          propertyType,
          config: config ?? null,
        });
        setProperties((prev) => [...prev, prop]);
        toast.success("プロパティを追加しました");
        return prop;
      } catch (err) {
        toast.error(errorMessage(err));
        return null;
      }
    },
    [databaseId],
  );

  const addPageToDatabase = useCallback(
    async (title: string): Promise<Page | null> => {
      try {
        const page = await invoke<Page>("add_page_to_database", {
          databaseId,
          title,
        });
        toast.success("ページを追加しました");
        return page;
      } catch (err) {
        toast.error(errorMessage(err));
        return null;
      }
    },
    [databaseId],
  );

  const addExistingPageToDatabase = useCallback(
    async (pageId: string): Promise<Page | null> => {
      try {
        const page = await invoke<Page>("add_existing_page_to_database", {
          databaseId,
          pageId,
        });
        toast.success("既存ページを追加しました");
        return page;
      } catch (err) {
        toast.error(errorMessage(err));
        return null;
      }
    },
    [databaseId],
  );

  const listStandalonePages = useCallback(async (): Promise<Page[]> => {
    try {
      return await invoke<Page[]>("list_standalone_pages");
    } catch (err) {
      toast.error(errorMessage(err));
      return [];
    }
  }, []);

  return {
    properties,
    listProperties,
    addProperty,
    addPageToDatabase,
    addExistingPageToDatabase,
    listStandalonePages,
  };
}
