import type { Page } from "./types";

interface DeleteConfirmModalProps {
  page: Page;
  onConfirm: () => void;
  onCancel: () => void;
}

export function DeleteConfirmModal({
  page,
  onConfirm,
  onCancel,
}: DeleteConfirmModalProps) {
  return (
    <>
      {/* biome-ignore lint/a11y/noStaticElementInteractions: overlay backdrop dismiss */}
      <div
        className="fixed inset-0 bg-black/40 flex items-center justify-center z-[1000]"
        role="presentation"
        onClick={onCancel}
        onKeyDown={(e) => {
          if (e.key === "Escape") onCancel();
        }}
      >
        {/* biome-ignore lint/a11y/noStaticElementInteractions: stopPropagation on modal container */}
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: stopPropagation on modal container */}
        <div
          className="bg-card rounded-lg p-6 max-w-[400px] w-[90%] shadow-lg"
          onClick={(e) => e.stopPropagation()}
        >
          <h3 className="m-0 mb-3 text-base">ページを削除しますか？</h3>
          <p className="text-muted-foreground text-sm m-0 mb-5 break-words">
            「{page.title}」を削除します。この操作は元に戻せません。
          </p>
          <div className="flex gap-2 justify-end">
            <button
              type="button"
              className="px-4 py-2 bg-secondary border-none rounded-md cursor-pointer text-sm hover:bg-secondary/80"
              onClick={onCancel}
            >
              キャンセル
            </button>
            <button
              type="button"
              className="px-4 py-2 bg-destructive text-white border-none rounded-md cursor-pointer text-sm hover:bg-destructive/85"
              onClick={onConfirm}
            >
              削除
            </button>
          </div>
        </div>
      </div>
    </>
  );
}
