import { useCallback, useState } from "react";

/**
 * Generic localStorage hook with JSON serialize/deserialize.
 *
 * Falls back to `defaultValue` when:
 * - The key doesn't exist in localStorage
 * - The stored value fails to parse as JSON
 * - localStorage is unavailable (e.g. in tests)
 */
export function useLocalStorage<T>(
  key: string,
  defaultValue: T,
): [T, (value: T | ((prev: T) => T)) => void] {
  const [storedValue, setStoredValue] = useState<T>(() => {
    return readFromStorage(key, defaultValue);
  });

  const setValue = useCallback(
    (value: T | ((prev: T) => T)) => {
      setStoredValue((prev) => {
        const nextValue =
          typeof value === "function" ? (value as (prev: T) => T)(prev) : value;
        try {
          window.localStorage.setItem(key, JSON.stringify(nextValue));
        } catch {
          // localStorage unavailable — silently ignore
        }
        return nextValue;
      });
    },
    [key],
  );

  return [storedValue, setValue];
}

/**
 * Read a value from localStorage with JSON parse, returning `defaultValue`
 * on any failure (missing key, parse error, unavailable storage).
 */
export function readFromStorage<T>(key: string, defaultValue: T): T {
  try {
    const item = window.localStorage.getItem(key);
    if (item === null) return defaultValue;
    return JSON.parse(item) as T;
  } catch {
    return defaultValue;
  }
}
