interface EditorToolbarProps {
  pageTitle: string;
}

export function EditorToolbar({ pageTitle }: EditorToolbarProps) {
  return (
    <div className="flex items-center gap-3 px-3 py-2 border-b border-border mb-4">
      <h2 className="flex-1 m-0 text-lg overflow-hidden text-ellipsis whitespace-nowrap">
        {pageTitle}
      </h2>
    </div>
  );
}
