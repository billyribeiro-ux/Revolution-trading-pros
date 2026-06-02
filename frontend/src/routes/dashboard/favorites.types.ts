/**
 * Shared favorites types.
 * ─────────────────────────────────────────────────────────────────────────────
 * Lives outside `favorites.remote.ts` on purpose: a `.remote.ts` module may only
 * export remote functions (`query`/`command`/`form`/`prerender`) and schemas
 * cannot be exported from it, so any shape the remote functions *and* their
 * consumers need to agree on is hoisted here (see the SvelteKit remote-functions
 * docs, "Validation" note). Keeps one source of truth for the favorite row.
 */

/** A single bookmarked alert/video/resource as returned by the backend. */
export interface Favorite {
	id: number;
	item_type: string;
	item_id: number;
	title: string | null;
	excerpt: string | null;
	href: string | null;
	thumbnail_url: string | null;
	created_at: string;
}
