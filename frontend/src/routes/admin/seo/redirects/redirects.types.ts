/**
 * Redirect Manager — shared types.
 * ─────────────────────────────────────────────────────────────────────────────
 * Hoisted out of the page and out of `redirects.remote.ts` (a `.remote.ts` may
 * only export remote functions).
 */

export interface Redirect {
	id: number;
	source_url: string;
	destination_url: string;
	redirect_type: string;
	is_regex?: boolean;
	is_active?: boolean;
	hits?: number;
	notes?: string;
}

export interface RedirectStats {
	total: number;
	active: number;
	inactive: number;
	total_hits?: number;
}
