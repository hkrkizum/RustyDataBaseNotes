import { invoke } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";
import { toast } from "sonner";
import type { Page } from "../pages/types";
import type {
  CommandError,
  PropertyConfigDto,
  PropertyDto,
  PropertyTypeDto,
  PropertyValueDto,
  PropertyValueInputDto,
  TableDataDto,
} from "./types";

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as CommandError).message;
  }
  return String(err);
}

export function useTableData(databaseId: string) {
  const [properties, setProperties] = useState<PropertyDto[]>([]);
  const [tableData, setTableData] = useState<TableDataDto | null>(null);
  const [loading, setLoading] = useState(false);

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

  const loadTableData = useCallback(async () => {
    setLoading(true);
    try {
      const data = await invoke<TableDataDto>("get_table_data", {
        databaseId,
      });
      setTableData(data);
      setProperties(data.properties);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setLoading(false);
    }
  }, [databaseId]);

  const setPropertyValue = useCallback(
    async (
      pageId: string,
      propertyId: string,
      value: PropertyValueInputDto,
    ): Promise<PropertyValueDto | null> => {
      try {
        const result = await invoke<PropertyValueDto>("set_property_value", {
          pageId,
          propertyId,
          value,
        });
        // Update local state
        setTableData((prev) => {
          if (!prev) return prev;
          return {
            ...prev,
            rows: prev.rows.map((row) =>
              row.page.id === pageId
                ? {
                    ...row,
                    values: { ...row.values, [propertyId]: result },
                  }
                : row,
            ),
          };
        });
        return result;
      } catch (err) {
        toast.error(errorMessage(err));
        return null;
      }
    },
    [],
  );

  const clearPropertyValue = useCallback(
    async (pageId: string, propertyId: string): Promise<boolean> => {
      try {
        await invoke("clear_property_value", { pageId, propertyId });
        // Update local state
        setTableData((prev) => {
          if (!prev) return prev;
          return {
            ...prev,
            rows: prev.rows.map((row) => {
              if (row.page.id !== pageId) return row;
              const { [propertyId]: _, ...rest } = row.values;
              return { ...row, values: rest };
            }),
          };
        });
        return true;
      } catch (err) {
        toast.error(errorMessage(err));
        return false;
      }
    },
    [],
  );

  return {
    properties,
    tableData,
    loading,
    listProperties,
    addProperty,
    addPageToDatabase,
    addExistingPageToDatabase,
    listStandalonePages,
    loadTableData,
    setPropertyValue,
    clearPropertyValue,
  };
}
