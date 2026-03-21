import { useState } from "react";
import "./App.css";
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

type CurrentView =
  | { type: "list" }
  | { type: "editor"; pageId: string; pageTitle: string }
  | { type: "table"; database: DatabaseDto };

function App() {
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
      <main className="container">
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
      <main className="container">
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
    <main className="container">
      <h1>RustyDataBaseNotes</h1>
      <CreatePageForm onSubmit={createPage} />
      <div style={{ marginTop: "0.5rem", marginBottom: "1rem" }}>
        <button
          type="button"
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
