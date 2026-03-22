import { useCallback, useEffect, useState } from "react";
import type { Page } from "../pages/types";

interface AddPageModalProps {
  onAddPage: (pageId: string) => Promise<Page | null>;
  listStandalonePages: () => Promise<Page[]>;
  onClose: () => void;
}

export function AddPageModal({
  onAddPage,
  listStandalonePages,
  onClose,
}: AddPageModalProps) {
  const [standalonePages, setStandalonePages] = useState<Page[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    listStandalonePages().then((pages) => {
      setStandalonePages(pages);
      setLoading(false);
    });
  }, [listStandalonePages]);

  const handleAdd = useCallback(
    async (pageId: string) => {
      const result = await onAddPage(pageId);
      if (result) {
        setStandalonePages((prev) => prev.filter((p) => p.id !== pageId));
      }
    },
    [onAddPage],
  );

  return (
    <>
      {/* biome-ignore lint/a11y/noStaticElementInteractions: overlay backdrop dismiss */}
      <div
        className="fixed inset-0 bg-black/40 flex items-center justify-center z-[100]"
        role="presentation"
        onClick={onClose}
        onKeyDown={(e) => {
          if (e.key === "Escape") onClose();
        }}
      >
        {/* biome-ignore lint/a11y/noStaticElementInteractions: stopPropagation on modal container */}
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: stopPropagation on modal container */}
        <div
          className="bg-card rounded-lg w-[400px] max-h-[70vh] flex flex-col shadow-lg"
          onClick={(e) => e.stopPropagation()}
        >
          <div className="flex items-center justify-between px-5 py-4 border-b border-border">
            <h3 className="m-0 text-lg">既存ページを追加</h3>
            <button
              type="button"
              className="bg-transparent border-none text-xl cursor-pointer text-muted-foreground px-2 py-0.5 hover:text-foreground"
              onClick={onClose}
            >
              x
            </button>
          </div>
          <div className="px-5 py-4 overflow-y-auto">
            {loading && (
              <p className="text-muted-foreground text-sm text-center">
                読み込み中...
              </p>
            )}
            {!loading && standalonePages.length === 0 && (
              <p className="text-muted-foreground text-sm text-center">
                追加可能なページがありません
              </p>
            )}
            {!loading && standalonePages.length > 0 && (
              <ul className="list-none m-0 p-0">
                {standalonePages.map((page) => (
                  <li
                    key={page.id}
                    className="flex items-center justify-between py-2 border-b border-border/50 last:border-b-0"
                  >
                    <span className="flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-[0.95rem]">
                      {page.title}
                    </span>
                    <button
                      type="button"
                      className="px-3 py-1 border border-border rounded bg-transparent cursor-pointer text-sm ml-2 hover:bg-accent"
                      onClick={() => handleAdd(page.id)}
                    >
                      追加
                    </button>
                  </li>
                ))}
              </ul>
            )}
          </div>
        </div>
      </div>
    </>
  );
}
