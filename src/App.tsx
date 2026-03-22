import { useState } from "react";
import { Toaster } from "./components/toast/Toaster";
import { DatabaseListView } from "./features/database/DatabaseListView";
import { TableView } from "./features/database/TableView";
import type { DatabaseDto } from "./features/database/types";
import { useDatabase } from "./features/database/useDatabase";
import { BlockEditor } from "./features/editor/BlockEditor";
import { CreatePageForm } from "./features/pages/CreatePageForm";
import { DeleteConfirmModal } from "./features/pages/DeleteConfirmModal";
import type { Page } from "./features/pages/types";
import { usePages } from "./features/pages/usePages";
import { useSystemTheme } from "./hooks/useSystemTheme";

type CurrentView =
  | { type: "list" }
  | { type: "editor"; pageId: string; pageTitle: string }
  | { type: "table"; database: DatabaseDto };

function App() {
  useSystemTheme();
  const { pages, loading, createPage, deletePage } = usePages();
  const {
    databases,
    loading: dbLoading,
    createDatabase,
    deleteDatabase,
    refreshDatabases,
  } = useDatabase();
  const [deleteTarget, setDeleteTarget] = useState<Page | null>(null);
  const [currentView, setCurrentView] = useState<CurrentView>({
    type: "list",
  });

  async function handleConfirmDelete() {
    if (!deleteTarget) return;
    const success = await deletePage(deleteTarget.id);
    if (success) {
      setDeleteTarget(null);
    }
  }

  function handlePageClick(page: Page) {
    setCurrentView({
      type: "editor",
      pageId: page.id,
      pageTitle: page.title,
    });
  }

  function handleDatabaseClick(database: DatabaseDto) {
    setCurrentView({ type: "table", database });
  }

  function handleNavigateBack() {
    setCurrentView({ type: "list" });
    refreshDatabases();
  }

  if (currentView.type === "editor") {
    return (
      <main className="flex min-h-screen flex-col">
        <BlockEditor
          pageId={currentView.pageId}
          pageTitle={currentView.pageTitle}
          onNavigateBack={handleNavigateBack}
        />
        <Toaster />
      </main>
    );
  }

  if (currentView.type === "table") {
    return (
      <main className="flex min-h-screen flex-col">
        <TableView
          database={currentView.database}
          onNavigateBack={handleNavigateBack}
          onPageClick={handlePageClick}
          onDatabaseDeleted={handleNavigateBack}
        />
        <Toaster />
      </main>
    );
  }

  return (
    <main className="flex min-h-screen flex-col items-center pt-[10vh]">
      <h1 className="text-2xl font-bold text-foreground">RustyDataBaseNotes</h1>
      <CreatePageForm onSubmit={createPage} />
      <div className="mt-2 mb-4">
        <button
          type="button"
          className="rounded-lg border border-border bg-card px-5 py-2.5 font-medium text-card-foreground shadow-sm transition-colors hover:bg-accent"
          onClick={async () => {
            const title = prompt("データベースのタイトルを入力してください");
            if (title) {
              await createDatabase(title);
            }
          }}
        >
          + データベースを作成
        </button>
      </div>
      <DatabaseListView
        pages={pages}
        databases={databases}
        loading={loading || dbLoading}
        onPageClick={handlePageClick}
        onDatabaseClick={handleDatabaseClick}
        onRequestDeletePage={setDeleteTarget}
        onRequestDeleteDatabase={async (db) => {
          await deleteDatabase(db.id);
        }}
      />
      {deleteTarget && (
        <DeleteConfirmModal
          page={deleteTarget}
          onConfirm={handleConfirmDelete}
          onCancel={() => setDeleteTarget(null)}
        />
      )}
      <Toaster />
    </main>
  );
}

export default App;
