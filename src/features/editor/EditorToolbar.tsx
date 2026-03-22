interface EditorToolbarProps {
  pageTitle: string;
  isDirty: boolean;
  onBack: () => void;
  onSave: () => void;
}

export function EditorToolbar({
  pageTitle,
  isDirty,
  onBack,
  onSave,
}: EditorToolbarProps) {
  return (
    <div className="flex items-center gap-3 px-3 py-2 border-b border-border mb-4">
      <button type="button" className="py-1.5 px-3 text-sm" onClick={onBack}>
        ← 戻る
      </button>
      <h2 className="flex-1 m-0 text-lg overflow-hidden text-ellipsis whitespace-nowrap">
        {pageTitle}
      </h2>
      <span className="text-destructive text-xs whitespace-nowrap">
        {isDirty ? "未保存" : ""}
      </span>
      <button
        type="button"
        className="py-1.5 px-3 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
        onClick={onSave}
        disabled={!isDirty}
      >
        保存
      </button>
    </div>
  );
}
