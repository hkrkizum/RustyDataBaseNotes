import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useAutoSave } from "../useAutoSave";

// Mock @tauri-apps/api/core
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// Mock sonner
vi.mock("sonner", () => ({
  toast: {
    warning: vi.fn(),
  },
}));

import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";

const mockInvoke = vi.mocked(invoke);

describe("useAutoSave", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.clearAllMocks();
    mockInvoke.mockResolvedValue(undefined);
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("should fire save_editor after 500ms debounce", async () => {
    const { result } = renderHook(() => useAutoSave("page-1"));

    act(() => {
      result.current.scheduleSave();
    });

    // Should not fire immediately
    expect(mockInvoke).not.toHaveBeenCalled();

    // Advance 500ms
    await act(async () => {
      vi.advanceTimersByTime(500);
    });

    expect(mockInvoke).toHaveBeenCalledWith("save_editor", {
      pageId: "page-1",
    });
    expect(mockInvoke).toHaveBeenCalledTimes(1);
  });

  it("should debounce multiple calls within 500ms", async () => {
    const { result } = renderHook(() => useAutoSave("page-1"));

    act(() => {
      result.current.scheduleSave();
    });

    await act(async () => {
      vi.advanceTimersByTime(200);
    });

    // Schedule again before debounce fires
    act(() => {
      result.current.scheduleSave();
    });

    await act(async () => {
      vi.advanceTimersByTime(500);
    });

    // Should only fire once (the second schedule reset the timer)
    expect(mockInvoke).toHaveBeenCalledTimes(1);
  });

  it("should retry with exponential backoff on failure", async () => {
    mockInvoke.mockRejectedValue({ kind: "storage", message: "DB locked" });

    const { result } = renderHook(() => useAutoSave("page-1"));

    act(() => {
      result.current.scheduleSave();
    });

    // First attempt after 500ms debounce
    await act(async () => {
      vi.advanceTimersByTime(500);
    });
    expect(mockInvoke).toHaveBeenCalledTimes(1);

    // Retry 1 after 1s
    await act(async () => {
      vi.advanceTimersByTime(1000);
    });
    expect(mockInvoke).toHaveBeenCalledTimes(2);

    // Retry 2 after 2s
    await act(async () => {
      vi.advanceTimersByTime(2000);
    });
    expect(mockInvoke).toHaveBeenCalledTimes(3);

    // Retry 3 after 4s
    await act(async () => {
      vi.advanceTimersByTime(4000);
    });
    expect(mockInvoke).toHaveBeenCalledTimes(4);
  });

  it("should show toast.warning after all 3 retries exhausted", async () => {
    mockInvoke.mockRejectedValue({ kind: "storage", message: "DB locked" });

    const { result } = renderHook(() => useAutoSave("page-1"));

    act(() => {
      result.current.scheduleSave();
    });

    // Debounce + 3 retries (500 + 1000 + 2000 + 4000 = 7500ms)
    await act(async () => {
      vi.advanceTimersByTime(500);
    });
    await act(async () => {
      vi.advanceTimersByTime(1000);
    });
    await act(async () => {
      vi.advanceTimersByTime(2000);
    });
    await act(async () => {
      vi.advanceTimersByTime(4000);
    });

    expect(toast.warning).toHaveBeenCalledWith("保存に失敗しました", {
      duration: 5000,
    });
  });

  it("should skip retry on permanent error (NotFound)", async () => {
    mockInvoke.mockRejectedValue({
      kind: "notFound",
      message: "page not found",
    });

    const { result } = renderHook(() => useAutoSave("page-1"));

    act(() => {
      result.current.scheduleSave();
    });

    await act(async () => {
      vi.advanceTimersByTime(500);
    });

    // Should not retry — only 1 call total
    expect(mockInvoke).toHaveBeenCalledTimes(1);

    // Should show toast immediately
    expect(toast.warning).toHaveBeenCalledWith("保存に失敗しました", {
      duration: 5000,
    });

    // Even after waiting, no more calls
    await act(async () => {
      vi.advanceTimersByTime(10000);
    });
    expect(mockInvoke).toHaveBeenCalledTimes(1);
  });

  it("should flush immediately on unmount", async () => {
    const { result, unmount } = renderHook(() => useAutoSave("page-1"));

    act(() => {
      result.current.scheduleSave();
    });

    // Unmount before debounce fires
    unmount();

    // The flush should attempt a synchronous save
    expect(mockInvoke).toHaveBeenCalledWith("save_editor", {
      pageId: "page-1",
    });
  });

  it("should cancel pending retries when page changes", async () => {
    mockInvoke.mockRejectedValue({ kind: "storage", message: "DB locked" });

    const { result, rerender } = renderHook(
      ({ pageId }: { pageId: string }) => useAutoSave(pageId),
      { initialProps: { pageId: "page-1" } },
    );

    act(() => {
      result.current.scheduleSave();
    });

    await act(async () => {
      vi.advanceTimersByTime(500);
    });

    // First attempt failed, retry scheduled
    expect(mockInvoke).toHaveBeenCalledTimes(1);

    // Change page — should cancel retries
    rerender({ pageId: "page-2" });

    // Wait for retry period — should not retry old page
    await act(async () => {
      vi.advanceTimersByTime(10000);
    });

    // The flush on cleanup may call once more, but no retries for old page
    const page1Calls = mockInvoke.mock.calls.filter(
      (call) =>
        call[0] === "save_editor" &&
        (call[1] as { pageId: string }).pageId === "page-1",
    );
    // At most 2: initial attempt + flush on unmount
    expect(page1Calls.length).toBeLessThanOrEqual(2);
  });
});
