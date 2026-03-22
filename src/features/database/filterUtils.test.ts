import { describe, expect, test } from "vitest";
import {
  getDefaultFilterValue,
  getFilterDisplayValue,
  isValueTypeCompatible,
  parseFilterValue,
} from "./filterUtils";

describe("getDefaultFilterValue", () => {
  test("number → { type: 'number', value: 0 }", () => {
    expect(getDefaultFilterValue("number")).toEqual({
      type: "number",
      value: 0,
    });
  });
  test("text → { type: 'text', value: '' }", () => {
    expect(getDefaultFilterValue("text")).toEqual({
      type: "text",
      value: "",
    });
  });
  test("select → { type: 'selectOption', value: '' }", () => {
    expect(getDefaultFilterValue("select")).toEqual({
      type: "selectOption",
      value: "",
    });
  });
  test("date → { type: 'date' } with ISO string", () => {
    const r = getDefaultFilterValue("date");
    expect(r.type).toBe("date");
    expect(typeof r.value).toBe("string");
  });
});

describe("parseFilterValue", () => {
  test("number: '1' → { type: 'number', value: 1 }", () => {
    expect(parseFilterValue("1", "number")).toEqual({
      type: "number",
      value: 1,
    });
  });
  test("number: '0' → { type: 'number', value: 0 }（0 は有効値）", () => {
    expect(parseFilterValue("0", "number")).toEqual({
      type: "number",
      value: 0,
    });
  });
  test("number: '' → null（空欄は値なし・B3 修正）", () => {
    expect(parseFilterValue("", "number")).toBeNull();
  });
  test("number: 'abc' → null", () => {
    expect(parseFilterValue("abc", "number")).toBeNull();
  });
  test("text: 'hello' → { type: 'text', value: 'hello' }", () => {
    expect(parseFilterValue("hello", "text")).toEqual({
      type: "text",
      value: "hello",
    });
  });
  test("select: UUID → { type: 'selectOption', value: UUID }", () => {
    const uuid = "550e8400-e29b-41d4-a716-446655440000";
    expect(parseFilterValue(uuid, "select")).toEqual({
      type: "selectOption",
      value: uuid,
    });
  });
});

describe("isValueTypeCompatible", () => {
  test("number + number → true", () => {
    expect(isValueTypeCompatible({ type: "number", value: 1 }, "number")).toBe(
      true,
    );
  });
  test("number + text → false", () => {
    expect(isValueTypeCompatible({ type: "text", value: "1" }, "number")).toBe(
      false,
    );
  });
  test("select + selectOption → true", () => {
    expect(
      isValueTypeCompatible({ type: "selectOption", value: "x" }, "select"),
    ).toBe(true);
  });
});

describe("getFilterDisplayValue", () => {
  test("null → ''", () => {
    expect(getFilterDisplayValue(null)).toBe("");
  });
  test("number → 文字列化", () => {
    expect(getFilterDisplayValue({ type: "number", value: 42 })).toBe("42");
  });
});
