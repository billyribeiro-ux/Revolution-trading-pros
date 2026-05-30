/**
 * DOM Type Declarations for Client-Side Code
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Ensures DOM types are available in all client-side modules
 * Fixes TypeScript errors when checking files that use window/document
 *
 * @version 1.0.0
 */

/// <reference lib="dom" />
/// <reference lib="dom.iterable" />

// `window.gtag` is declared canonically in src/app.d.ts (typed with
// `unknown[]`); no Window augmentation needed here. This file exists only to
// pull in the DOM libs above for client-side modules.

export {};
