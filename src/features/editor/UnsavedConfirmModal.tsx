interface UnsavedConfirmModalProps {
  onDiscard: () => void;
  onCancel: () => void;
}

export function UnsavedConfirmModal({
  onDiscard,
  onCancel,
}: UnsavedConfirmModalProps) {
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
          className="bg-card rounded-lg p-6 max-w-[400px] w-[90%] shadow-[0_4px_24px_rgba(0,0,0,0.15)]"
          onClick={(e) => e.stopPropagation()}
        >
          <h3 className="m-0 mb-3 text-base">未保存の変更があります</h3>
          <p className="text-muted-foreground text-sm m-0 mb-5">
            未保存の変更があります。破棄しますか？
          </p>
          <div className="flex gap-2 justify-end">
            <button
              type="button"
              className="py-2 px-4 bg-secondary border-none rounded cursor-pointer text-sm hover:bg-secondary/80"
              onClick={onCancel}
            >
              キャンセル
            </button>
            <button
              type="button"
              className="py-2 px-4 bg-destructive text-white border-none rounded cursor-pointer text-sm hover:bg-destructive/80"
              onClick={onDiscard}
            >
              破棄
            </button>
          </div>
        </div>
      </div>
    </>
  );
}
