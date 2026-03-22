import { useEffect } from "react";

/**
 * Watches the OS color scheme preference and toggles the `dark` class
 * on `document.documentElement` accordingly.
 *
 * Uses `prefers-color-scheme: dark` media query. No manual toggle —
 * always follows the OS setting (spec FR-016).
 */
export function useSystemTheme(): void {
  useEffect(() => {
    const mql = window.matchMedia("(prefers-color-scheme: dark)");

    function apply(isDark: boolean) {
      document.documentElement.classList.toggle("dark", isDark);
    }

    apply(mql.matches);

    function onChange(e: MediaQueryListEvent) {
      apply(e.matches);
    }

    mql.addEventListener("change", onChange);
    return () => mql.removeEventListener("change", onChange);
  }, []);
}
