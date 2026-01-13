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

// Ensure global types are available
declare global {
	interface Window {
		gtag?: (...args: any[]) => void;
	}
}

export {};
