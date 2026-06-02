/**
 * 404 Monitor — shared types.
 * ─────────────────────────────────────────────────────────────────────────────
 * Hoisted out of the page and out of `monitor.remote.ts` (a `.remote.ts` may
 * only export remote functions).
 */

export interface Seo404Log {
	id: number;
	url: string;
	hits: number;
	last_hit_at: string;
	referer?: string | null;
	is_resolved?: boolean;
	is_ignored?: boolean;
}

export interface Seo404Stats {
	total: number;
	unresolved: number;
	resolved: number;
	total_hits?: number;
}

/** Sort orders accepted by the logs list. */
export type Seo404Sort = 'hits' | 'latest' | 'oldest';
