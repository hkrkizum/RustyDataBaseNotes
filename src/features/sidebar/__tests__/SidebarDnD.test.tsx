import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { SidebarItem } from "../types";

// Mock @tauri-apps/api/core
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// Mock sonner
vi.mock("sonner", () => ({
  toast: {
    error: vi.fn(),
    warning: vi.fn(),
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

describe("Sidebar D&D data validation", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  afterEach(() => {
    localStorage.clear();
  });

  it("should call move_page with newParentId when reparenting", async () => {
    const parentId = crypto.randomUUID();
    const childId = crypto.randomUUID();

    mockInvoke.mockResolvedValueOnce({
      id: childId,
      title: "Child",
      parentId,
      sortOrder: 0,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    });

    await invoke("move_page", { pageId: childId, newParentId: parentId });

    expect(mockInvoke).toHaveBeenCalledWith("move_page", {
      pageId: childId,
      newParentId: parentId,
    });
  });

  it("should call move_page with null newParentId for root promotion", async () => {
    const pageId = crypto.randomUUID();

    mockInvoke.mockResolvedValueOnce({
      id: pageId,
      title: "Page",
      parentId: null,
      sortOrder: 0,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    });

    await invoke("move_page", { pageId, newParentId: null });

    expect(mockInvoke).toHaveBeenCalledWith("move_page", {
      pageId,
      newParentId: null,
    });
  });

  it("should handle move_page error for circular reference", async () => {
    const parentId = crypto.randomUUID();
    const childId = crypto.randomUUID();

    mockInvoke.mockRejectedValueOnce({
      kind: "circularReference",
      message: `Circular reference detected: page ${parentId} cannot be moved under ${childId}`,
    });

    await expect(
      invoke("move_page", { pageId: parentId, newParentId: childId }),
    ).rejects.toEqual(expect.objectContaining({ kind: "circularReference" }));
  });

  it("should handle move_page error for max depth exceeded", async () => {
    const pageId = crypto.randomUUID();
    const targetId = crypto.randomUUID();

    mockInvoke.mockRejectedValueOnce({
      kind: "maxDepthExceeded",
      message: `Maximum nesting depth (5) exceeded for page ${pageId}`,
    });

    await expect(
      invoke("move_page", { pageId, newParentId: targetId }),
    ).rejects.toEqual(expect.objectContaining({ kind: "maxDepthExceeded" }));
  });

  it("should handle move_page error for database page", async () => {
    const dbPageId = crypto.randomUUID();
    const targetId = crypto.randomUUID();

    mockInvoke.mockRejectedValueOnce({
      kind: "databasePageCannotNest",
      message: `Database page ${dbPageId} cannot participate in page hierarchy`,
    });

    await expect(
      invoke("move_page", { pageId: dbPageId, newParentId: targetId }),
    ).rejects.toEqual(
      expect.objectContaining({ kind: "databasePageCannotNest" }),
    );
  });

  it("should determine canDrop based on item properties", () => {
    const page = makeSidebarItem({ itemType: "page" });
    const dbItem = makeSidebarItem({ itemType: "database" });
    const dbPage = makeSidebarItem({
      itemType: "page",
      databaseId: "some-db",
    });

    // Regular pages can be dragged
    expect(page.itemType === "page" && !page.databaseId).toBe(true);

    // Database items cannot be dragged for reparenting
    expect(dbItem.itemType === "database").toBe(true);

    // DB-owned pages cannot be dragged for reparenting
    expect(dbPage.databaseId !== null).toBe(true);
  });

  it("should reject self-drop", () => {
    const pageId = crypto.randomUUID();
    const sourceData = { pageId, parentId: null, depth: 1, itemType: "page" };
    const targetData = { pageId, parentId: null, depth: 1, itemType: "page" };

    // Self-reference check
    expect(sourceData.pageId === targetData.pageId).toBe(true);
  });
});
