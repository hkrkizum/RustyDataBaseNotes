import { type FormEvent, useCallback, useEffect, useState } from "react";
import type { Page } from "../pages/types";
import { AddPageModal } from "./AddPageModal";
import { TableHeader } from "./TableHeader";
import { TableRow } from "./TableRow";
import styles from "./TableView.module.css";
import type { DatabaseDto, PropertyValueInputDto } from "./types";
import { useTableData } from "./useTableData";

interface TableViewProps {
  database: DatabaseDto;
  onNavigateBack: () => void;
  onPageClick?: (page: Page) => void;
}

export function TableView({
  database,
  onNavigateBack,
  onPageClick,
}: TableViewProps) {
  const [newTitle, setNewTitle] = useState("");
  const [showModal, setShowModal] = useState(false);

  const {
    properties,
    tableData,
    loading,
    addProperty,
    addPageToDatabase,
    addExistingPageToDatabase,
    listStandalonePages,
    loadTableData,
    setPropertyValue,
    clearPropertyValue,
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

  const rows = tableData?.rows ?? [];

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
        <h2 className={styles.title}>{database.title}</h2>
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
                onAddProperty={handleAddProperty}
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
    </div>
  );
}
