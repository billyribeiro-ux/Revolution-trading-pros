/**
 * Keyword Tracking — shared types.
 * ─────────────────────────────────────────────────────────────────────────────
 * Hoisted out of the page and out of `keywords.remote.ts` (a `.remote.ts` may
 * only export remote functions).
 */

export interface Keyword {
	id: number;
	keyword: string;
	current_rank?: number;
	rank_change?: number | null;
	search_volume?: number;
	competition?: number | null;
	target_url?: string;
}

export interface KeywordStats {
	total?: number;
	top_3?: number;
	top_10?: number;
	avg_position?: number;
	top_keywords?: Keyword[];
	opportunity_keywords?: Keyword[];
}
