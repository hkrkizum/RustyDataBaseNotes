---
name: use-pnpm-not-npx
description: Always use pnpm (not npm/npx) for running JS/TS tools and scripts
type: feedback
---

Use `pnpm` for all JavaScript/TypeScript commands, not `npx` or `npm`.

**Why:** Project uses pnpm as the package manager (documented in constitution Technical Standards).

**How to apply:** Use `pnpm vitest` instead of `npx vitest`, `pnpm exec` instead of `npx`, etc.
