import { invoke } from "@tauri-apps/api/core";
import { type FormEvent, useCallback, useEffect, useState } from "react";
import { toast } from "sonner";
import type { Page } from "../pages/types";
import { AddPageModal } from "./AddPageModal";
import { FilterPanel } from "./FilterPanel";
import { GroupHeader } from "./GroupHeader";
import { GroupPanel } from "./GroupPanel";
import { SortPanel } from "./SortPanel";
import { TableHeader } from "./TableHeader";
import { TableRow } from "./TableRow";
import type {
  DatabaseDto,
  FilterConditionDto,
  GroupConditionDto,
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
    updateFilterConditions,
    updateGroupCondition,
    toggleGroupCollapsed,
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

  const [showFilterPanel, setShowFilterPanel] = useState(false);
  const [showSortPanel, setShowSortPanel] = useState(false);
  const [showGroupPanel, setShowGroupPanel] = useState(false);

  const handleFilterApply = useCallback(
    async (conditions: FilterConditionDto[]) => {
      await updateFilterConditions(conditions);
    },
    [updateFilterConditions],
  );

  const handleSortClick = useCallback(
    async (conditions: SortConditionDto[]) => {
      await updateSortConditions(conditions);
    },
    [updateSortConditions],
  );

  const handleSortPanelApply = useCallback(
    async (conditions: SortConditionDto[]) => {
      await updateSortConditions(conditions);
    },
    [updateSortConditions],
  );

  const handleGroupApply = useCallback(
    async (condition: GroupConditionDto | null) => {
      await updateGroupCondition(condition);
    },
    [updateGroupCondition],
  );

  const handleGroupToggle = useCallback(
    async (groupValue: string | null) => {
      await toggleGroupCollapsed(groupValue);
    },
    [toggleGroupCollapsed],
  );

  const rows = tableData?.rows ?? [];
  const view = tableData?.view ?? null;
  const groups = tableData?.groups ?? null;
  const sortCount = view?.sortConditions?.length ?? 0;
  const filterCount = view?.filterConditions?.length ?? 0;
  const hasGroup = view?.groupCondition != null;

  return (
    <div className="w-full">
      <div className="flex items-center gap-4 mb-6">
        <button
          type="button"
          className="px-3 py-1.5 border border-border rounded cursor-pointer text-sm bg-transparent hover:bg-accent"
          onClick={onNavigateBack}
        >
          &larr; 戻る
        </button>
        {editingTitle ? (
          <input
            className="text-2xl font-bold border border-ring rounded px-1.5 py-0.5 outline-none"
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
            className="bg-transparent border-none cursor-pointer p-0 font-inherit text-left rounded hover:bg-accent"
            onClick={() => setEditingTitle(true)}
            title="クリックして名前を編集"
          >
            <h2 className="m-0 text-2xl">{dbTitle}</h2>
          </button>
        )}
        <button
          type="button"
          className="ml-auto px-3 py-1.5 border border-border rounded bg-transparent cursor-pointer text-sm text-muted-foreground hover:text-destructive hover:border-destructive"
          onClick={() => setShowDeleteDbConfirm(true)}
          title="データベースを削除"
        >
          削除
        </button>
      </div>
      <div className="flex items-center gap-3 mb-6">
        <form className="flex gap-2" onSubmit={handleCreatePage}>
          <input
            className="px-3 py-1.5 border border-border rounded text-sm w-[200px]"
            type="text"
            placeholder="新しいページ名"
            value={newTitle}
            onChange={(e) => setNewTitle(e.target.value)}
          />
          <button
            type="submit"
            className="px-3 py-1.5 border border-border rounded bg-transparent cursor-pointer text-sm hover:bg-accent"
          >
            追加
          </button>
        </form>
        <button
          type="button"
          className="px-3 py-1.5 border border-border rounded bg-transparent cursor-pointer text-sm hover:bg-accent"
          onClick={() => setShowModal(true)}
        >
          既存ページを追加
        </button>
        <button
          type="button"
          className="px-3 py-1.5 border border-border rounded bg-transparent cursor-pointer text-sm hover:bg-accent"
          onClick={() => {
            setShowSortPanel((v) => !v);
            setShowFilterPanel(false);
          }}
        >
          ソート{sortCount > 0 ? ` (${sortCount})` : ""}
        </button>
        <button
          type="button"
          className="px-3 py-1.5 border border-border rounded bg-transparent cursor-pointer text-sm hover:bg-accent"
          onClick={() => {
            setShowFilterPanel((v) => !v);
            setShowSortPanel(false);
          }}
        >
          フィルタ{filterCount > 0 ? ` (${filterCount})` : ""}
        </button>
        <button
          type="button"
          className="px-3 py-1.5 border border-border rounded bg-transparent cursor-pointer text-sm hover:bg-accent"
          onClick={() => {
            setShowGroupPanel((v) => !v);
            setShowSortPanel(false);
            setShowFilterPanel(false);
          }}
        >
          グループ{hasGroup ? " ●" : ""}
        </button>
        {(sortCount > 0 || filterCount > 0 || hasGroup) && (
          <button
            type="button"
            className="px-3 py-1.5 border border-border rounded bg-transparent cursor-pointer text-sm hover:bg-accent"
            onClick={() => void resetView()}
          >
            設定をリセット
          </button>
        )}
      </div>

      {showSortPanel && (
        <SortPanel
          properties={properties}
          conditions={view?.sortConditions ?? []}
          onApply={handleSortPanelApply}
          onClose={() => setShowSortPanel(false)}
        />
      )}

      {showGroupPanel && (
        <GroupPanel
          properties={properties}
          currentCondition={view?.groupCondition ?? null}
          onApply={handleGroupApply}
          onClose={() => setShowGroupPanel(false)}
        />
      )}

      {showFilterPanel && (
        <FilterPanel
          properties={properties}
          conditions={view?.filterConditions ?? []}
          onApply={handleFilterApply}
          onClose={() => setShowFilterPanel(false)}
        />
      )}

      {loading ? (
        <div className="text-center p-12 text-muted-foreground">
          <p>読み込み中...</p>
        </div>
      ) : rows.length === 0 && properties.length === 0 ? (
        <div className="text-center p-12 text-muted-foreground">
          <p>ページがありません</p>
          <p className="text-sm text-muted-foreground/70">
            ページを追加してテーブルを構築しましょう
          </p>
        </div>
      ) : (
        <div className="overflow-x-auto border border-border rounded">
          <div className="min-w-full">
            <div className="flex items-stretch bg-muted border-b-2 border-border font-semibold text-sm text-muted-foreground">
              <div className="min-w-[200px] flex-[0_0_200px] px-2.5 py-2 flex items-center border-r border-border">
                タイトル
              </div>
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
              <div className="p-8 text-center text-muted-foreground text-sm">
                <p>
                  {filterCount > 0
                    ? "条件に一致するページがありません"
                    : "ページがありません"}
                </p>
                {filterCount > 0 && (
                  <button
                    type="button"
                    className="mt-2 px-3 py-1 border border-ring rounded bg-transparent text-ring cursor-pointer text-sm hover:bg-ring/10"
                    onClick={() => void updateFilterConditions([])}
                  >
                    すべてのフィルタを解除
                  </button>
                )}
              </div>
            ) : groups ? (
              (() => {
                const elements: React.ReactNode[] = [];
                let rowCursor = 0;
                for (const group of groups) {
                  elements.push(
                    <GroupHeader
                      key={`group-${group.value ?? "__null__"}`}
                      group={group}
                      onToggle={handleGroupToggle}
                    />,
                  );
                  if (!group.isCollapsed) {
                    for (let j = 0; j < group.count; j++) {
                      const row = rows[rowCursor];
                      if (row) {
                        elements.push(
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
                          />,
                        );
                      }
                      rowCursor++;
                    }
                  }
                }
                return elements;
              })()
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
          className="fixed inset-0 bg-black/40 flex items-center justify-center z-[100]"
          role="presentation"
          onClick={() => setShowDeleteDbConfirm(false)}
          onKeyDown={(e) => {
            if (e.key === "Escape") setShowDeleteDbConfirm(false);
          }}
        >
          {/* biome-ignore lint/a11y/noStaticElementInteractions: confirm dialog */}
          {/* biome-ignore lint/a11y/useKeyWithClickEvents: confirm dialog */}
          <div
            className="bg-card rounded-lg p-6 w-[360px] shadow-lg"
            onClick={(e) => e.stopPropagation()}
          >
            <p className="m-0 mb-4 text-[0.95rem] leading-relaxed">
              データベース「{dbTitle}
              」を削除しますか？プロパティと値はすべて削除されます。ページ自体は残ります。
            </p>
            <div className="flex justify-end gap-2">
              <button
                type="button"
                className="px-4 py-2 border border-border rounded cursor-pointer text-sm bg-transparent hover:bg-accent"
                onClick={() => setShowDeleteDbConfirm(false)}
              >
                キャンセル
              </button>
              <button
                type="button"
                className="px-4 py-2 border-none rounded bg-destructive text-white cursor-pointer text-sm hover:bg-destructive/80"
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
