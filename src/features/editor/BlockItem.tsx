import { useEffect, useRef, useState } from "react";
import type { Block } from "./types";

interface BlockItemProps {
  block: Block;
  isFirst: boolean;
  isLast: boolean;
  shouldFocus?: boolean;
  onEditContent: (blockId: string, content: string) => void;
  onMoveUp: (blockId: string) => void;
  onMoveDown: (blockId: string) => void;
  onRemove: (blockId: string) => void;
}

export function BlockItem({
  block,
  isFirst,
  isLast,
  shouldFocus,
  onEditContent,
  onMoveUp,
  onMoveDown,
  onRemove,
}: BlockItemProps) {
  const [localContent, setLocalContent] = useState(block.content);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (shouldFocus && textareaRef.current) {
      textareaRef.current.focus();
    }
  }, [shouldFocus]);

  function handleBlur() {
    if (localContent !== block.content) {
      onEditContent(block.id, localContent);
    }
  }

  // Sync local state when backend state changes (e.g. after error recovery)
  if (
    localContent !== block.content &&
    document.activeElement !== textareaRef.current
  ) {
    setLocalContent(block.content);
  }

  return (
    <div className="flex items-start gap-2 px-3 py-2 border border-border rounded mb-2">
      <textarea
        ref={textareaRef}
        className="flex-1 min-h-12 p-1 border border-input rounded-sm font-[inherit] resize-y leading-relaxed focus:border-primary focus:outline-none"
        value={localContent}
        onChange={(e) => setLocalContent(e.target.value)}
        onBlur={handleBlur}
        maxLength={10000}
        placeholder="テキストを入力..."
      />
      <div className="flex gap-1 shrink-0">
        <button
          type="button"
          className="bg-transparent border border-input rounded-sm py-0.5 px-1.5 text-xs cursor-pointer leading-none hover:enabled:border-primary hover:enabled:text-primary disabled:opacity-30 disabled:cursor-not-allowed"
          onClick={() => onMoveUp(block.id)}
          disabled={isFirst}
          title="上に移動"
        >
          ↑
        </button>
        <button
          type="button"
          className="bg-transparent border border-input rounded-sm py-0.5 px-1.5 text-xs cursor-pointer leading-none hover:enabled:border-primary hover:enabled:text-primary disabled:opacity-30 disabled:cursor-not-allowed"
          onClick={() => onMoveDown(block.id)}
          disabled={isLast}
          title="下に移動"
        >
          ↓
        </button>
        <button
          type="button"
          className="bg-transparent border border-input rounded-sm py-0.5 px-1.5 text-sm cursor-pointer text-muted-foreground leading-none hover:border-destructive hover:text-destructive"
          onClick={() => onRemove(block.id)}
          title="削除"
        >
          &times;
        </button>
      </div>
    </div>
  );
}
