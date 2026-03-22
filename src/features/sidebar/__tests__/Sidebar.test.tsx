import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { SidebarItem } from "../types";
import { useSidebar } from "../useSidebar";

// Mock @tauri-apps/api/core
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// Mock sonner
vi.mock("sonner", () => ({
  toast: {
    error: vi.fn(),
    success: vi.fn(),
  },
}));

import { invoke } from "@tauri-apps/api/core";

const mockInvoke = vi.mocked(invoke);

function makeSidebarItem(overrides: Partial<SidebarItem> = {}): SidebarItem {
  return {
    id: crypto.randomUUID(),
    title: "Untitled",
    itemType: "page",
    parentId: null,
    databaseId: null,
    createdAt: new Date().toISOString(),
    ...overrides,
  };
}

describe("useSidebar", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
    mockInvoke.mockResolvedValue([]);
  });

  afterEach(() => {
    localStorage.clear();
  });

  it("should fetch sidebar items on mount", async () => {
    const items: SidebarItem[] = [
      makeSidebarItem({ title: "Page A", itemType: "page" }),
      makeSidebarItem({ title: "DB 1", itemType: "database" }),
    ];
    mockInvoke.mockResolvedValueOnce(items);

    const { result } = renderHook(() => useSidebar());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(mockInvoke).toHaveBeenCalledWith("list_sidebar_items");
    expect(result.current.tree).toHaveLength(2);
  });

  it("should build tree from flat items using parentId", async () => {
    const parentId = crypto.randomUUID();
    const items: SidebarItem[] = [
      makeSidebarItem({
        id: parentId,
        title: "Parent",
        itemType: "page",
        parentId: null,
      }),
      makeSidebarItem({
        title: "Child",
        itemType: "page",
        parentId: parentId,
      }),
    ];
    mockInvoke.mockResolvedValueOnce(items);

    const { result } = renderHook(() => useSidebar());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    // Root level should have 1 item (the parent)
    expect(result.current.tree).toHaveLength(1);
    expect(result.current.tree[0].title).toBe("Parent");
    expect(result.current.tree[0].children).toHaveLength(1);
    expect(result.current.tree[0].children[0].title).toBe("Child");
  });

  it("should group DB-owned pages under their database", async () => {
    const dbId = crypto.randomUUID();
    const items: SidebarItem[] = [
      makeSidebarItem({ id: dbId, title: "Tasks DB", itemType: "database" }),
      makeSidebarItem({
        title: "Task Row",
        itemType: "page",
        databaseId: dbId,
      }),
    ];
    mockInvoke.mockResolvedValueOnce(items);

    const { result } = renderHook(() => useSidebar());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    // Root should have 1 item (the database)
    expect(result.current.tree).toHaveLength(1);
    expect(result.current.tree[0].itemType).toBe("database");
    expect(result.current.tree[0].children).toHaveLength(1);
    expect(result.current.tree[0].children[0].title).toBe("Task Row");
  });

  it("should show empty state when no items exist", async () => {
    mockInvoke.mockResolvedValueOnce([]);

    const { result } = renderHook(() => useSidebar());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.tree).toHaveLength(0);
  });

  it("should track active item ID", async () => {
    const items: SidebarItem[] = [
      makeSidebarItem({ id: "p1", title: "Page 1" }),
    ];
    mockInvoke.mockResolvedValueOnce(items);

    const { result } = renderHook(() => useSidebar());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    act(() => {
      result.current.setActiveItemId("p1");
    });

    expect(result.current.activeItemId).toBe("p1");
  });

  it("should manage expand/collapse state in localStorage", async () => {
    const items: SidebarItem[] = [
      makeSidebarItem({ id: "p1", title: "Parent" }),
    ];
    mockInvoke.mockResolvedValueOnce(items);

    const { result } = renderHook(() => useSidebar());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    // Default: collapsed
    expect(result.current.expandedState.p1).toBeFalsy();

    // Toggle expand
    act(() => {
      result.current.toggleExpanded("p1");
    });
    expect(result.current.expandedState.p1).toBe(true);

    // Verify localStorage
    const stored = JSON.parse(localStorage.getItem("sidebar-expanded") ?? "{}");
    expect(stored.p1).toBe(true);

    // Toggle collapse
    act(() => {
      result.current.toggleExpanded("p1");
    });
    expect(result.current.expandedState.p1).toBe(false);
  });

  it("should sort items by createdAt DESC within each level", async () => {
    const items: SidebarItem[] = [
      makeSidebarItem({
        id: "p1",
        title: "Older",
        createdAt: "2026-01-01T00:00:00Z",
      }),
      makeSidebarItem({
        id: "p2",
        title: "Newer",
        createdAt: "2026-03-01T00:00:00Z",
      }),
    ];
    mockInvoke.mockResolvedValueOnce(items);

    const { result } = renderHook(() => useSidebar());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    // Newer should come first (DESC)
    expect(result.current.tree[0].title).toBe("Newer");
    expect(result.current.tree[1].title).toBe("Older");
  });
});
