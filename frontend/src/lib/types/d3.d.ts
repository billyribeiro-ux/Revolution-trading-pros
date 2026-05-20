/**
 * d3 Type Declarations — INTENTIONALLY NEUTRALISED (R24-A, 2026-05-20)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * This file previously declared a hand-rolled `declare module 'd3' { ... }`
 * with `any` for every member (`scaleLinear(): any`, `arc<_T>(): any`, etc.).
 * That shim *shadowed* the real `@types/d3` typings (already a project
 * dependency) for every consumer — defeating the entire purpose of the
 * d3 type packages. The single consumer (`ChartIcons.svelte`) had to use
 * `as any` casts everywhere to compile against the shadowed signatures.
 *
 * R24-A neutralises the override by emptying the module body, restoring
 * `@types/d3` as the authoritative declaration. The file itself is kept
 * (rather than deleted) to:
 *
 *   1. Preserve git blame on the original commit that introduced the shim,
 *   2. Mark the slot as intentionally-empty so a future contributor doesn't
 *      "fix the missing d3 types" by re-introducing the override,
 *   3. Match the project's hard rule against file deletion without an
 *      explicit per-file approval.
 *
 * If d3 ever needs project-specific augmentations (e.g., extending
 * `Selection.prototype` via a plugin), declare them inside this empty
 * `export {}` module — augmentations merge with `@types/d3`, they do not
 * shadow it.
 */
export {};
