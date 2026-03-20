import { invoke } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";
import { toast } from "sonner";
import type { CommandError, EditorState } from "./types";

function errorMessage(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    return (err as CommandError).message;
  }
  return String(err);
}

export function useEditor() {
  const [editorState, setEditorState] = useState<EditorState | null>(null);
  const [loading, setLoading] = useState(false);

  const openEditor = useCallback(async (pageId: string) => {
    setLoading(true);
    try {
      const state = await invoke<EditorState>("open_editor", { pageId });
      setEditorState(state);
    } catch (err) {
      toast.error(errorMessage(err));
    } finally {
      setLoading(false);
    }
  }, []);

  const closeEditor = useCallback(async (pageId: string) => {
    try {
      await invoke("close_editor", { pageId });
      setEditorState(null);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, []);

  const addBlock = useCallback(async (pageId: string) => {
    try {
      const state = await invoke<EditorState>("add_block", { pageId });
      setEditorState(state);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, []);

  const editBlockContent = useCallback(
    async (pageId: string, blockId: string, content: string) => {
      try {
        const state = await invoke<EditorState>("edit_block_content", {
          pageId,
          blockId,
          content,
        });
        setEditorState(state);
        return state;
      } catch (err) {
        const error = err as CommandError;
        if (error.kind === "contentTooLong") {
          toast.error(error.message);
          // Return the state from the error response if available
        }
        toast.error(errorMessage(err));
        return null;
      }
    },
    [],
  );

  const moveBlockUp = useCallback(async (pageId: string, blockId: string) => {
    try {
      const state = await invoke<EditorState>("move_block_up", {
        pageId,
        blockId,
      });
      setEditorState(state);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, []);

  const moveBlockDown = useCallback(async (pageId: string, blockId: string) => {
    try {
      const state = await invoke<EditorState>("move_block_down", {
        pageId,
        blockId,
      });
      setEditorState(state);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, []);

  const removeBlock = useCallback(async (pageId: string, blockId: string) => {
    try {
      const state = await invoke<EditorState>("remove_block", {
        pageId,
        blockId,
      });
      setEditorState(state);
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, []);

  const saveEditor = useCallback(async (pageId: string) => {
    try {
      const state = await invoke<EditorState>("save_editor", { pageId });
      setEditorState(state);
      toast.success("保存しました");
    } catch (err) {
      toast.error(errorMessage(err));
    }
  }, []);

  return {
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
  };
}
