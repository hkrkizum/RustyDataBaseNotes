import { useCallback, useEffect, useRef } from "react";
import styles from "./BlockEditor.module.css";
import { BlockItem } from "./BlockItem";
import { EditorToolbar } from "./EditorToolbar";
import { UnsavedConfirmModal } from "./UnsavedConfirmModal";
import { useEditor } from "./useEditor";

interface BlockEditorProps {
  pageId: string;
  pageTitle: string;
  onNavigateBack: () => void;
}

export function BlockEditor({
  pageId,
  pageTitle,
  onNavigateBack,
}: BlockEditorProps) {
  const {
    editorState,
    loading,
    openEditor,
    closeEditor,
    addBlock,
    editBlockContent,
    moveBlockUp,
    moveBlockDown,
    removeBlock,
    saveEditor,
    showUnsavedConfirm,
    setShowUnsavedConfirm,
  } = useEditor();

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

  // Ctrl+S keyboard shortcut
  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      if ((e.ctrlKey || e.metaKey) && e.key === "s") {
        e.preventDefault();
        // Blur active element to sync any pending textarea content
        if (document.activeElement instanceof HTMLElement) {
          document.activeElement.blur();
        }
        // Use setTimeout to allow blur handler to fire first
        setTimeout(() => {
          saveEditor(pageId);
        }, 0);
      }
    }
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [pageId, saveEditor]);

  const handleBack = useCallback(() => {
    if (editorState?.isDirty) {
      setShowUnsavedConfirm(true);
      return;
    }
    closeEditor(pageId);
    onNavigateBack();
  }, [editorState, pageId, closeEditor, onNavigateBack, setShowUnsavedConfirm]);

  function handleDiscard() {
    setShowUnsavedConfirm(false);
    closeEditor(pageId);
    onNavigateBack();
  }

  function handleCancelConfirm() {
    setShowUnsavedConfirm(false);
  }

  function handleSave() {
    // Blur active element to sync any pending textarea content
    if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
    setTimeout(() => {
      saveEditor(pageId);
    }, 0);
  }

  function handleAddBlock() {
    addBlock(pageId);
  }

  if (loading || !editorState) {
    return <div className={styles.loading}>読み込み中...</div>;
  }

  const blocks = editorState.blocks;
  const justAdded = blocks.length > prevBlockCount.current;

  return (
    <div className={styles.editor}>
      <EditorToolbar
        pageTitle={pageTitle}
        isDirty={editorState.isDirty}
        onBack={handleBack}
        onSave={handleSave}
      />
      {blocks.length === 0 ? (
        <div className={styles.empty}>
          <p>ブロックがありません</p>
          <p className={styles.hint}>
            下のボタンからブロックを追加してください
          </p>
        </div>
      ) : (
        <div className={styles.blockList}>
          {blocks.map((block, index) => (
            <BlockItem
              key={block.id}
              block={block}
              isFirst={index === 0}
              isLast={index === blocks.length - 1}
              shouldFocus={justAdded && index === blocks.length - 1}
              onEditContent={(blockId, content) =>
                editBlockContent(pageId, blockId, content)
              }
              onMoveUp={(blockId) => moveBlockUp(pageId, blockId)}
              onMoveDown={(blockId) => moveBlockDown(pageId, blockId)}
              onRemove={(blockId) => removeBlock(pageId, blockId)}
            />
          ))}
        </div>
      )}
      <button
        type="button"
        className={styles.addButton}
        onClick={handleAddBlock}
      >
        + ブロック追加
      </button>
      {showUnsavedConfirm && (
        <UnsavedConfirmModal
          onDiscard={handleDiscard}
          onCancel={handleCancelConfirm}
        />
      )}
    </div>
  );
}
