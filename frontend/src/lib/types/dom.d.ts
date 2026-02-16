/**
 * DOM Type Declarations for Client-Side Code
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Ensures DOM types are available in all client-side modules
 * Fixes TypeScript errors when checking files that use window/document
 *
 * @version 2.0.0 - ICT 7+ Strict Type Safety
 */

/// <reference lib="dom" />
/// <reference lib="dom.iterable" />

// ═══════════════════════════════════════════════════════════════════════════
// Google Analytics gtag.js Types
// ═══════════════════════════════════════════════════════════════════════════

type GtagCommand = 'config' | 'event' | 'set' | 'get' | 'consent';
type GtagConfigParams = {
	page_path?: string;
	page_title?: string;
	page_location?: string;
	send_page_view?: boolean;
	[key: string]: string | number | boolean | undefined;
};
type GtagEventParams = {
	event_category?: string;
	event_label?: string;
	value?: number;
	[key: string]: string | number | boolean | undefined;
};

// Ensure global types are available
declare global {
	interface Window {
		/** Google Analytics gtag function - properly typed */
		gtag?: {
			(command: 'config', targetId: string, config?: GtagConfigParams): void;
			(command: 'event', eventName: string, eventParams?: GtagEventParams): void;
			(command: 'set', params: Record<string, unknown>): void;
			(command: GtagCommand, ...args: unknown[]): void;
		};
	}
}

export {};
