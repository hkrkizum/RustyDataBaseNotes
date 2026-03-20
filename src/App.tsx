import { useState } from "react";
import "./App.css";
import { Toaster } from "./components/toast/Toaster";
import { CreatePageForm } from "./features/pages/CreatePageForm";
import { PageListView } from "./features/pages/PageListView";
import { DeleteConfirmModal } from "./features/pages/DeleteConfirmModal";
import { usePages } from "./features/pages/usePages";
import type { Page } from "./features/pages/types";

function App() {
  const { pages, loading, createPage, updatePageTitle, deletePage } =
    usePages();
  const [deleteTarget, setDeleteTarget] = useState<Page | null>(null);

  async function handleConfirmDelete() {
    if (!deleteTarget) return;
    const success = await deletePage(deleteTarget.id);
    if (success) {
      setDeleteTarget(null);
    }
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
