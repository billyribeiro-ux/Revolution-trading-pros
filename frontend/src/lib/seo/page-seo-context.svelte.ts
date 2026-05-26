/**
 * @deprecated As of May 2026 SEO audit, the legacy SeoHead component (which
 * this bridge served) has zero importers. All routes now write SEO directly
 * into `page.data.seo` from their `load()`. The layout still wires this
 * context to avoid a breaking change in `+layout.svelte`, but it is inert.
 *
 * Page SEO Context — SSR-safe component-to-layout SEO bridge
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Allows a page-level component (e.g. legacy `SEOHead`) to feed its SEO
 * data into the layout's unified `Seo.svelte` renderer WITHOUT emitting
 * duplicate `<svelte:head>` tags.
 *
 * Architecture:
 *   1. Layout creates the context via `createPageSeoContext()` BEFORE
 *      rendering `<Seo />` and `{@render children()}`.
 *   2. Page components mutate `pageSeoState.value` synchronously in their
 *      `<script>` body (NOT in `$effect`, which is client-only).
 *   3. Layout's `<Seo>` consumes the merged state via `$derived` and emits
 *      the single source of truth for meta/canonical/og/twitter tags.
 *
 * SSR correctness:
 *   - The mutation happens during Svelte 5 component initialization, which
 *     runs synchronously inline with parent template evaluation. By the time
 *     SvelteKit serializes the collected `<svelte:head>` output, all
 *     descendant components have already written their values.
 *   - Context is per-component-tree (per-request), so no cross-request leaks.
 *
 * @author Revolution Trading Pros — Principal Engineer ICT 7+
 * @since May 2026
 */

import { getContext, setContext } from 'svelte';
import type { SEOInput } from './types';

const KEY = Symbol('rtp.page-seo-state');

export interface PageSeoState {
	/**
	 * Lazy SEO input factory contributed by the deepest page-level component.
	 * Using a function ensures reactive prop reads occur inside the layout's
	 * $derived (reactive context), eliminating state_referenced_locally warnings.
	 */
	value: (() => SEOInput) | null;
}

/**
 * Create the page-SEO context. Call this ONCE in the root layout BEFORE
 * rendering children. Returns the reactive state container.
 */
export function createPageSeoContext(): PageSeoState {
	const state = $state<PageSeoState>({ value: null });
	setContext(KEY, state);
	return state;
}

/**
 * Retrieve the page-SEO context from a child component. Returns `undefined`
 * if no ancestor layout has called `createPageSeoContext()` — components
 * MUST tolerate this case (no-op gracefully).
 */
export function getPageSeoContext(): PageSeoState | undefined {
	return getContext<PageSeoState | undefined>(KEY);
}
