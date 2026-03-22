import { describe, expect, it } from "vitest";

describe("Design System Migration", () => {
  it("should have zero *.module.css files under src/", () => {
    // Vite's import.meta.glob resolves at build/test time — an empty record means no matches
    const cssModuleFiles = import.meta.glob("../**/*.module.css", {
      eager: true,
    });
    const paths = Object.keys(cssModuleFiles);
    expect(paths, `Found CSS Module files:\n${paths.join("\n")}`).toHaveLength(
      0,
    );
  });

  it("should have zero CSS Modules import statements in src/**/*.{ts,tsx}", () => {
    // Grab the raw source of every TS/TSX file at test time
    const sourceFiles = import.meta.glob("../**/*.{ts,tsx}", {
      eager: true,
      query: "?raw",
      import: "default",
    });

    const filesWithCssModuleImports: string[] = [];
    for (const [path, raw] of Object.entries(sourceFiles)) {
      // Skip this test file itself
      if (path.includes("design-system.test")) continue;
      if (typeof raw === "string" && /import\s.*\.module\.css/.test(raw)) {
        filesWithCssModuleImports.push(path);
      }
    }

    expect(
      filesWithCssModuleImports,
      `Found CSS Modules imports in:\n${filesWithCssModuleImports.join("\n")}`,
    ).toHaveLength(0);
  });
});
