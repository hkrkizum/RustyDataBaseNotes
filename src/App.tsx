import { useState } from "react";
import "./App.css";
import { Toaster } from "./components/toast/Toaster";
import { BlockEditor } from "./features/editor/BlockEditor";
import { CreatePageForm } from "./features/pages/CreatePageForm";
import { DeleteConfirmModal } from "./features/pages/DeleteConfirmModal";
import { PageListView } from "./features/pages/PageListView";
import type { Page } from "./features/pages/types";
import { usePages } from "./features/pages/usePages";

type CurrentView =
  | { type: "list" }
  | { type: "editor"; pageId: string; pageTitle: string };

function App() {
  const { pages, loading, createPage, updatePageTitle, deletePage } =
    usePages();
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
    setCurrentView({ type: "editor", pageId: page.id, pageTitle: page.title });
  }

  function handleNavigateBack() {
    setCurrentView({ type: "list" });
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

  return (
    <main className="container">
      <h1>RustyDataBaseNotes</h1>
      <CreatePageForm onSubmit={createPage} />
      <PageListView
        pages={pages}
        loading={loading}
        onUpdateTitle={updatePageTitle}
        onRequestDelete={setDeleteTarget}
        onPageClick={handlePageClick}
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
