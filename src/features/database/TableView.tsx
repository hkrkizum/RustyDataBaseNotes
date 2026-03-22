import { invoke } from "@tauri-apps/api/core";
import { type FormEvent, useCallback, useEffect, useState } from "react";
import { toast } from "sonner";
import type { Page } from "../pages/types";
import { AddPageModal } from "./AddPageModal";
import { TableHeader } from "./TableHeader";
import { TableRow } from "./TableRow";
import styles from "./TableView.module.css";
import type {
  DatabaseDto,
  PropertyValueInputDto,
  SortConditionDto,
} from "./types";
import { useTableData } from "./useTableData";

interface TableViewProps {
  database: DatabaseDto;
  onNavigateBack: () => void;
  onPageClick?: (page: Page) => void;
  onDatabaseDeleted?: () => void;
}

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as { message: string }).message;
  }
  return String(err);
}

export function TableView({
  database,
  onNavigateBack,
  onPageClick,
  onDatabaseDeleted,
}: TableViewProps) {
  const [newTitle, setNewTitle] = useState("");
  const [showModal, setShowModal] = useState(false);
  const [editingTitle, setEditingTitle] = useState(false);
  const [dbTitle, setDbTitle] = useState(database.title);
  const [showDeleteDbConfirm, setShowDeleteDbConfirm] = useState(false);

  const {
    properties,
    tableData,
    loading,
    addProperty,
    updatePropertyName,
    updatePropertyConfig,
    deleteProperty,
    resetSelectOption,
    addPageToDatabase,
    addExistingPageToDatabase,
    listStandalonePages,
    loadTableData,
    setPropertyValue,
    clearPropertyValue,
    removePageFromDatabase,
    updateSortConditions,
    resetView,
  } = useTableData(database.id);

  useEffect(() => {
    void loadTableData();
  }, [loadTableData]);

  const handleCreatePage = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();
      const trimmed = newTitle.trim();
      if (!trimmed) return;
      const page = await addPageToDatabase(trimmed);
      if (page) {
        setNewTitle("");
        void loadTableData();
      }
    },
    [newTitle, addPageToDatabase, loadTableData],
  );

  const handleAddExisting = useCallback(
    async (pageId: string): Promise<Page | null> => {
      const page = await addExistingPageToDatabase(pageId);
      if (page) {
        void loadTableData();
      }
      return page;
    },
    [addExistingPageToDatabase, loadTableData],
  );

  const handleAddProperty = useCallback(
    async (
      name: string,
      propertyType: Parameters<typeof addProperty>[1],
      config?: Parameters<typeof addProperty>[2],
    ) => {
      const result = await addProperty(name, propertyType, config);
      if (result) {
        void loadTableData();
      }
      return result;
    },
    [addProperty, loadTableData],
  );

  const handleSaveValue = useCallback(
    async (
      pageId: string,
      propertyId: string,
      value: PropertyValueInputDto,
    ) => {
      return setPropertyValue(pageId, propertyId, value);
    },
    [setPropertyValue],
  );

  const handleClearValue = useCallback(
    async (pageId: string, propertyId: string) => {
      return clearPropertyValue(pageId, propertyId);
    },
    [clearPropertyValue],
  );

  const handlePageClick = useCallback(
    (page: Page) => {
      if (onPageClick) {
        onPageClick(page);
      }
    },
    [onPageClick],
  );

  const handleTitleSave = useCallback(async () => {
    const trimmed = dbTitle.trim();
    if (!trimmed || trimmed === database.title) {
      setDbTitle(database.title);
      setEditingTitle(false);
      return;
    }
    try {
      await invoke("update_database_title", {
        id: database.id,
        title: trimmed,
      });
      toast.success("データベース名を更新しました");
    } catch (err) {
      toast.error(errorMessage(err));
      setDbTitle(database.title);
    }
    setEditingTitle(false);
  }, [dbTitle, database.id, database.title]);

  const handleDeleteDatabase = useCallback(async () => {
    try {
      await invoke("delete_database", { id: database.id });
      toast.success("データベースを削除しました");
      setShowDeleteDbConfirm(false);
      if (onDatabaseDeleted) {
        onDatabaseDeleted();
      } else {
        onNavigateBack();
      }
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, [database.id, onDatabaseDeleted, onNavigateBack]);

  const handleRemoveFromDatabase = useCallback(
    async (pageId: string): Promise<boolean> => {
      return removePageFromDatabase(pageId);
    },
    [removePageFromDatabase],
  );

  const handleDeletePage = useCallback(
    async (pageId: string): Promise<boolean> => {
      try {
        await invoke("delete_page", { id: pageId });
        void loadTableData();
        toast.success("ページを削除しました");
        return true;
      } catch (err) {
        toast.error(errorMessage(err));
        return false;
      }
    },
    [loadTableData],
  );

  const handleSortClick = useCallback(
    async (conditions: SortConditionDto[]) => {
      await updateSortConditions(conditions);
    },
    [updateSortConditions],
  );

  const rows = tableData?.rows ?? [];
  const view = tableData?.view ?? null;
  const sortCount = view?.sortConditions?.length ?? 0;
  const filterCount = view?.filterConditions?.length ?? 0;

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <button
          type="button"
          className={styles.backBtn}
          onClick={onNavigateBack}
        >
          &larr; 戻る
        </button>
        {editingTitle ? (
          <input
            className={styles.titleInput}
            type="text"
            value={dbTitle}
            onChange={(e) => setDbTitle(e.target.value)}
            onBlur={handleTitleSave}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                handleTitleSave();
              }
              if (e.key === "Escape") {
                setDbTitle(database.title);
                setEditingTitle(false);
              }
            }}
            // biome-ignore lint/a11y/noAutofocus: title editing requires immediate focus
            autoFocus
          />
        ) : (
          <button
            type="button"
            className={styles.titleBtn}
            onClick={() => setEditingTitle(true)}
            title="クリックして名前を編集"
          >
            <h2 className={styles.title}>{dbTitle}</h2>
          </button>
        )}
        <button
          type="button"
          className={styles.deleteDatabaseBtn}
          onClick={() => setShowDeleteDbConfirm(true)}
          title="データベースを削除"
        >
          削除
        </button>
      </div>
      <div className={styles.actions}>
        <form className={styles.form} onSubmit={handleCreatePage}>
          <input
            className={styles.input}
            type="text"
            placeholder="新しいページ名"
            value={newTitle}
            onChange={(e) => setNewTitle(e.target.value)}
          />
          <button type="submit" className={styles.submitBtn}>
            追加
          </button>
        </form>
        <button
          type="button"
          className={styles.existingBtn}
          onClick={() => setShowModal(true)}
        >
          既存ページを追加
        </button>
        {sortCount > 0 && (
          <span className={styles.toolbarBadge}>ソート: {sortCount}件</span>
        )}
        {filterCount > 0 && (
          <span className={styles.toolbarBadge}>フィルタ: {filterCount}件</span>
        )}
        {(sortCount > 0 || filterCount > 0 || view?.groupCondition) && (
          <button
            type="button"
            className={styles.existingBtn}
            onClick={() => void resetView()}
          >
            設定をリセット
          </button>
        )}
      </div>

      {loading ? (
        <div className={styles.emptyState}>
          <p>読み込み中...</p>
        </div>
      ) : rows.length === 0 && properties.length === 0 ? (
        <div className={styles.emptyState}>
          <p>ページがありません</p>
          <p className={styles.hint}>
            ページを追加してテーブルを構築しましょう
          </p>
        </div>
      ) : (
        <div className={styles.tableWrapper}>
          <div className={styles.table}>
            <div className={styles.tableHeaderRow}>
              <div className={styles.titleHeader}>タイトル</div>
              <TableHeader
                properties={properties}
                view={view}
                onAddProperty={handleAddProperty}
                onUpdatePropertyName={updatePropertyName}
                onUpdatePropertyConfig={updatePropertyConfig}
                onDeleteProperty={deleteProperty}
                onResetSelectOption={resetSelectOption}
                onSortClick={handleSortClick}
              />
            </div>
            {rows.length === 0 ? (
              <div className={styles.emptyRow}>
                <p>ページがありません</p>
              </div>
            ) : (
              rows.map((row) => (
                <TableRow
                  key={row.page.id}
                  page={row.page}
                  properties={properties}
                  values={row.values}
                  onPageClick={handlePageClick}
                  onSaveValue={handleSaveValue}
                  onClearValue={handleClearValue}
                  onRemoveFromDatabase={handleRemoveFromDatabase}
                  onDeletePage={handleDeletePage}
                />
              ))
            )}
          </div>
        </div>
      )}

      {showModal && (
        <AddPageModal
          onAddPage={handleAddExisting}
          listStandalonePages={listStandalonePages}
          onClose={() => setShowModal(false)}
        />
      )}

      {showDeleteDbConfirm && (
        /* biome-ignore lint/a11y/noStaticElementInteractions: confirm overlay */
        <div
          className={styles.confirmOverlay}
          role="presentation"
          onClick={() => setShowDeleteDbConfirm(false)}
          onKeyDown={(e) => {
            if (e.key === "Escape") setShowDeleteDbConfirm(false);
          }}
        >
          {/* biome-ignore lint/a11y/noStaticElementInteractions: confirm dialog */}
          {/* biome-ignore lint/a11y/useKeyWithClickEvents: confirm dialog */}
          <div
            className={styles.confirmDialog}
            onClick={(e) => e.stopPropagation()}
          >
            <p className={styles.confirmMessage}>
              データベース「{dbTitle}
              」を削除しますか？プロパティと値はすべて削除されます。ページ自体は残ります。
            </p>
            <div className={styles.confirmActions}>
              <button
                type="button"
                className={styles.cancelBtn}
                onClick={() => setShowDeleteDbConfirm(false)}
              >
                キャンセル
              </button>
              <button
                type="button"
                className={styles.confirmDeleteBtn}
                onClick={handleDeleteDatabase}
              >
                削除する
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
