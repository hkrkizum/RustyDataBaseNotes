import { useEffect, useRef } from "react";
import { useAutoSave } from "../../hooks/useAutoSave";
import { BlockItem } from "./BlockItem";
import { EditorToolbar } from "./EditorToolbar";
import { useEditor } from "./useEditor";

interface BlockEditorProps {
  pageId: string;
  pageTitle: string;
}

export function BlockEditor({ pageId, pageTitle }: BlockEditorProps) {
  const {
    editorState,
    loading,
    openEditor,
    addBlock,
    editBlockContent,
    moveBlockUp,
    moveBlockDown,
    removeBlock,
  } = useEditor();

  const { scheduleSave } = useAutoSave(pageId);
  const prevBlockCount = useRef(0);

  useEffect(() => {
    openEditor(pageId);
  }, [pageId, openEditor]);

  // Track block count for auto-focus on new block
  useEffect(() => {
    if (editorState) {
      prevBlockCount.current = editorState.blocks.length;
    }
  }, [editorState]);

  // Suppress Ctrl+S / Cmd+S browser default save dialog
  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      if ((e.ctrlKey || e.metaKey) && e.key === "s") {
        e.preventDefault();
      }
    }
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  async function handleAddBlock() {
    await addBlock(pageId);
    scheduleSave();
  }

  async function handleEditContent(blockId: string, content: string) {
    await editBlockContent(pageId, blockId, content);
    scheduleSave();
  }

  async function handleMoveUp(blockId: string) {
    await moveBlockUp(pageId, blockId);
    scheduleSave();
  }

  async function handleMoveDown(blockId: string) {
    await moveBlockDown(pageId, blockId);
    scheduleSave();
  }

  async function handleRemove(blockId: string) {
    await removeBlock(pageId, blockId);
    scheduleSave();
  }

  if (loading || !editorState) {
    return (
      <div className="text-center p-8 text-muted-foreground">読み込み中...</div>
    );
  }

  const blocks = editorState.blocks;
  const justAdded = blocks.length > prevBlockCount.current;

  return (
    <div className="text-left">
      <EditorToolbar pageTitle={pageTitle} />
      {blocks.length === 0 ? (
        <div className="text-center p-8 text-muted-foreground">
          <p>ブロックがありません</p>
          <p className="text-sm mt-2">
            下のボタンからブロックを追加してください
          </p>
        </div>
      ) : (
        <div className="mb-4">
          {blocks.map((block, index) => (
            <BlockItem
              key={block.id}
              block={block}
              isFirst={index === 0}
              isLast={index === blocks.length - 1}
              shouldFocus={justAdded && index === blocks.length - 1}
              onEditContent={handleEditContent}
              onMoveUp={handleMoveUp}
              onMoveDown={handleMoveDown}
              onRemove={handleRemove}
            />
          ))}
        </div>
      )}
      <button
        type="button"
        className="w-full p-2 text-sm border border-dashed border-border rounded bg-transparent cursor-pointer text-muted-foreground hover:border-primary hover:text-primary"
        onClick={handleAddBlock}
      >
        + ブロック追加
      </button>
    </div>
  );
}
