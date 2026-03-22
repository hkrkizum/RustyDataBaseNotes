import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef } from "react";
import { toast } from "sonner";

/** Debounce interval in milliseconds. */
const DEBOUNCE_MS = 500;

/** Maximum number of retry attempts. */
const MAX_RETRIES = 3;

/** Base interval for exponential backoff (doubles each retry). */
const BASE_RETRY_MS = 1000;

/** Error kinds that indicate a permanent failure (no retry). */
const PERMANENT_ERROR_KINDS = new Set(["notFound"]);

interface CommandError {
  kind: string;
  message: string;
}

function isPermanentError(err: unknown): boolean {
  if (typeof err === "object" && err !== null && "kind" in err) {
    return PERMANENT_ERROR_KINDS.has((err as CommandError).kind);
  }
  return false;
}

/**
 * Auto-save hook for the editor. Debounces save calls and retries on failure
 * with exponential backoff.
 *
 * @param pageId - The page ID to save.
 * @returns An object with `scheduleSave` to trigger a debounced save.
 */
export function useAutoSave(pageId: string) {
  const debounceTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const retryTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const pendingRef = useRef(false);
  const mountedRef = useRef(true);
  const pageIdRef = useRef(pageId);

  // Keep pageId ref in sync
  pageIdRef.current = pageId;

  const clearTimers = useCallback(() => {
    if (debounceTimerRef.current !== null) {
      clearTimeout(debounceTimerRef.current);
      debounceTimerRef.current = null;
    }
    if (retryTimerRef.current !== null) {
      clearTimeout(retryTimerRef.current);
      retryTimerRef.current = null;
    }
  }, []);

  const doSave = useCallback(
    async (retriesLeft: number) => {
      try {
        await invoke("save_editor", { pageId: pageIdRef.current });
        pendingRef.current = false;
      } catch (err) {
        if (isPermanentError(err)) {
          pendingRef.current = false;
          toast.warning("保存に失敗しました", { duration: 5000 });
          return;
        }

        if (retriesLeft > 0 && mountedRef.current) {
          const delay = BASE_RETRY_MS * 2 ** (MAX_RETRIES - retriesLeft);
          retryTimerRef.current = setTimeout(() => {
            void doSave(retriesLeft - 1);
          }, delay);
        } else {
          pendingRef.current = false;
          toast.warning("保存に失敗しました", { duration: 5000 });
        }
      }
    },
    [], // pageIdRef is a ref — no dep needed
  );

  const scheduleSave = useCallback(() => {
    pendingRef.current = true;

    // Clear any existing debounce timer
    if (debounceTimerRef.current !== null) {
      clearTimeout(debounceTimerRef.current);
    }

    debounceTimerRef.current = setTimeout(() => {
      debounceTimerRef.current = null;
      void doSave(MAX_RETRIES);
    }, DEBOUNCE_MS);
  }, [doSave]);

  // Cleanup on unmount or pageId change: cancel timers + flush.
  // `pageId` is intentionally in deps to trigger cleanup when the page changes,
  // even though the value is read via pageIdRef inside the cleanup.
  // biome-ignore lint/correctness/useExhaustiveDependencies: pageId triggers flush on page change
  useEffect(() => {
    mountedRef.current = true;

    return () => {
      mountedRef.current = false;
      clearTimers();

      if (pendingRef.current) {
        // Best-effort flush — fire and forget
        void invoke("save_editor", { pageId: pageIdRef.current });
        pendingRef.current = false;
      }
    };
  }, [pageId, clearTimers]);

  return { scheduleSave };
}
