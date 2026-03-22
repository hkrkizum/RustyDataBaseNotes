import { useCallback, useMemo, useState } from "react";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { Toaster } from "./components/toast/Toaster";
import { TableView } from "./features/database/TableView";
import type { DatabaseDto } from "./features/database/types";
import { BlockEditor } from "./features/editor/BlockEditor";
import type { Page } from "./features/pages/types";
import { AppSidebar } from "./features/sidebar/Sidebar";
import type { LastOpenedItem, SidebarItem } from "./features/sidebar/types";
import { readFromStorage } from "./hooks/useLocalStorage";
import { useSystemTheme } from "./hooks/useSystemTheme";

const LAST_OPENED_KEY = "last-opened-item";

type CurrentView =
  | { type: "empty" }
  | { type: "editor"; pageId: string; pageTitle: string }
  | { type: "table"; databaseId: string; databaseTitle: string };

function lastOpenedToView(item: LastOpenedItem): CurrentView {
  if (item.type === "page") {
    return { type: "editor", pageId: item.id, pageTitle: item.title };
  }
  return { type: "table", databaseId: item.id, databaseTitle: item.title };
}

function saveLastOpened(item: LastOpenedItem): void {
  try {
    window.localStorage.setItem(LAST_OPENED_KEY, JSON.stringify(item));
  } catch {
    // localStorage unavailable
  }
}

function App() {
  useSystemTheme();

  const storedItem = useMemo(
    () => readFromStorage<LastOpenedItem | null>(LAST_OPENED_KEY, null),
    [],
  );

  const [currentView, setCurrentView] = useState<CurrentView>(() => {
    if (storedItem) {
      return lastOpenedToView(storedItem);
    }
    return { type: "empty" };
  });

  const handlePageClick = useCallback((pageId: string, pageTitle: string) => {
    setCurrentView({ type: "editor", pageId, pageTitle });
    saveLastOpened({ id: pageId, type: "page", title: pageTitle });
  }, []);

  const handleDatabaseClick = useCallback(
    (databaseId: string, databaseTitle: string) => {
      setCurrentView({ type: "table", databaseId, databaseTitle });
      saveLastOpened({
        id: databaseId,
        type: "database",
        title: databaseTitle,
      });
    },
    [],
  );

  const handlePageClickFromTable = useCallback((page: Page) => {
    setCurrentView({
      type: "editor",
      pageId: page.id,
      pageTitle: page.title,
    });
    saveLastOpened({ id: page.id, type: "page", title: page.title });
  }, []);

  const handleItemsLoaded = useCallback((items: SidebarItem[]) => {
    setCurrentView((prev) => {
      // If we're already showing something, validate it still exists
      if (prev.type !== "empty") {
        const viewId = prev.type === "editor" ? prev.pageId : prev.databaseId;
        const exists = items.some((item) => item.id === viewId);
        if (exists) return prev;
        // Item was deleted — fall through to find first root item
      }

      // Navigate to the first root-level item (no parentId, no databaseId)
      const rootItems = items.filter(
        (item) => !item.parentId && !item.databaseId,
      );
      if (rootItems.length === 0) {
        // No items at all — clear localStorage and show empty state
        try {
          window.localStorage.removeItem(LAST_OPENED_KEY);
        } catch {
          // localStorage unavailable
        }
        return { type: "empty" };
      }

      // Sort by createdAt DESC (same as sidebar) and pick first
      rootItems.sort((a, b) => b.createdAt.localeCompare(a.createdAt));
      const first = rootItems[0];
      if (first.itemType === "page") {
        saveLastOpened({ id: first.id, type: "page", title: first.title });
        return { type: "editor", pageId: first.id, pageTitle: first.title };
      }
      saveLastOpened({
        id: first.id,
        type: "database",
        title: first.title,
      });
      return {
        type: "table",
        databaseId: first.id,
        databaseTitle: first.title,
      };
    });
  }, []);

  const handleDatabaseDeleted = useCallback(() => {
    try {
      window.localStorage.removeItem(LAST_OPENED_KEY);
    } catch {
      // localStorage unavailable
    }
    setCurrentView({ type: "empty" });
  }, []);

  function renderContent() {
    if (currentView.type === "editor") {
      return (
        <BlockEditor
          pageId={currentView.pageId}
          pageTitle={currentView.pageTitle}
        />
      );
    }

    if (currentView.type === "table") {
      const database: DatabaseDto = {
        id: currentView.databaseId,
        title: currentView.databaseTitle,
        createdAt: "",
        updatedAt: "",
      };
      return (
        <TableView
          database={database}
          onNavigateBack={handleDatabaseDeleted}
          onPageClick={handlePageClickFromTable}
          onDatabaseDeleted={handleDatabaseDeleted}
        />
      );
    }

    return (
      <div className="flex flex-1 items-center justify-center text-muted-foreground">
        <p>サイドバーからページまたはデータベースを選択してください</p>
      </div>
    );
  }

  return (
    <SidebarProvider>
      <AppSidebar
        initialActiveItemId={storedItem?.id}
        onPageClick={handlePageClick}
        onDatabaseClick={handleDatabaseClick}
        onItemsLoaded={handleItemsLoaded}
      />
      <SidebarInset>
        <div className="flex min-h-screen flex-col">{renderContent()}</div>
      </SidebarInset>
      <Toaster />
    </SidebarProvider>
  );
}

export default App;
