import { useCallback, useState } from "react";
import type { Page } from "../pages/types";
import { PropertyCell } from "./PropertyCell";
import type {
  PropertyDto,
  PropertyValueDto,
  PropertyValueInputDto,
} from "./types";

interface TableRowProps {
  page: Page;
  properties: PropertyDto[];
  values: Record<string, PropertyValueDto>;
  onPageClick: (page: Page) => void;
  onSaveValue: (
    pageId: string,
    propertyId: string,
    value: PropertyValueInputDto,
  ) => Promise<unknown>;
  onClearValue: (pageId: string, propertyId: string) => Promise<unknown>;
  onRemoveFromDatabase?: (pageId: string) => Promise<boolean>;
  onDeletePage?: (pageId: string) => Promise<boolean>;
}

export function TableRow({
  page,
  properties,
  values,
  onPageClick,
  onSaveValue,
  onClearValue,
  onRemoveFromDatabase,
  onDeletePage,
}: TableRowProps) {
  const [showMenu, setShowMenu] = useState(false);
  const [showConfirm, setShowConfirm] = useState<"remove" | "delete" | null>(
    null,
  );

  const handleTitleClick = useCallback(() => {
    onPageClick(page);
  }, [page, onPageClick]);

  const handleRemove = useCallback(async () => {
    if (onRemoveFromDatabase) {
      await onRemoveFromDatabase(page.id);
    }
    setShowConfirm(null);
    setShowMenu(false);
  }, [page.id, onRemoveFromDatabase]);

  const handleDelete = useCallback(async () => {
    if (onDeletePage) {
      await onDeletePage(page.id);
    }
    setShowConfirm(null);
    setShowMenu(false);
  }, [page.id, onDeletePage]);

  return (
    <div className="flex border-b border-border/50 items-stretch group/row hover:bg-muted/50">
      <div className="min-w-[200px] flex-[0_0_200px] px-2.5 py-1.5 flex items-center border-r border-border/50 relative">
        <button
          type="button"
          className="cursor-pointer text-foreground no-underline font-medium bg-transparent border-none p-0 text-inherit font-inherit text-left flex-1 hover:underline hover:text-primary"
          onClick={handleTitleClick}
        >
          {page.title}
        </button>
        {(onRemoveFromDatabase || onDeletePage) && (
          <button
            type="button"
            className="bg-transparent border-none cursor-pointer px-1.5 py-0.5 text-sm text-muted-foreground rounded-sm invisible group-hover/row:visible tracking-wide hover:bg-accent hover:text-accent-foreground"
            onClick={() => setShowMenu((prev) => !prev)}
            title="操作"
          >
            ...
          </button>
        )}
        {showMenu && (
          <div className="absolute top-full right-1.5 bg-card border border-border rounded-md shadow-lg z-50 min-w-[160px] overflow-hidden">
            {onRemoveFromDatabase && (
              <button
                type="button"
                className="block w-full px-3 py-2 border-none bg-transparent cursor-pointer text-sm text-left font-inherit hover:bg-accent"
                onClick={() => {
                  setShowConfirm("remove");
                  setShowMenu(false);
                }}
              >
                データベースから除外
              </button>
            )}
            {onDeletePage && (
              <button
                type="button"
                className="block w-full px-3 py-2 border-none bg-transparent cursor-pointer text-sm text-left text-destructive font-inherit hover:bg-destructive/10"
                onClick={() => {
                  setShowConfirm("delete");
                  setShowMenu(false);
                }}
              >
                完全に削除
              </button>
            )}
          </div>
        )}
      </div>
      {properties.map((prop) => (
        <div
          key={prop.id}
          className="min-w-[150px] flex-1 px-1.5 py-0.5 flex items-center border-r border-border/50 last:border-r-0"
        >
          <PropertyCell
            property={prop}
            value={values[prop.id]}
            pageId={page.id}
            onSave={onSaveValue}
            onClear={onClearValue}
          />
        </div>
      ))}

      {showConfirm && (
        /* biome-ignore lint/a11y/noStaticElementInteractions: confirm overlay */
        <div
          className="fixed inset-0 bg-black/40 flex items-center justify-center z-[100]"
          role="presentation"
          onClick={() => setShowConfirm(null)}
          onKeyDown={(e) => {
            if (e.key === "Escape") setShowConfirm(null);
          }}
        >
          {/* biome-ignore lint/a11y/noStaticElementInteractions: confirm dialog */}
          {/* biome-ignore lint/a11y/useKeyWithClickEvents: confirm dialog */}
          <div
            className="bg-card rounded-lg p-6 w-[340px] shadow-lg"
            onClick={(e) => e.stopPropagation()}
          >
            <p className="m-0 mb-4 text-[0.95rem] leading-relaxed">
              {showConfirm === "remove"
                ? `ページ「${page.title}」をデータベースから除外しますか？ページ自体は残りますが、プロパティ値は削除されます。`
                : `ページ「${page.title}」を完全に削除しますか？この操作は取り消せません。`}
            </p>
            <div className="flex justify-end gap-2">
              <button
                type="button"
                className="px-4 py-2 border border-border rounded cursor-pointer text-sm bg-transparent hover:bg-accent"
                onClick={() => setShowConfirm(null)}
              >
                キャンセル
              </button>
              <button
                type="button"
                className="px-4 py-2 border-none rounded bg-destructive text-white cursor-pointer text-sm hover:bg-destructive/80"
                onClick={showConfirm === "remove" ? handleRemove : handleDelete}
              >
                {showConfirm === "remove" ? "除外する" : "削除する"}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
