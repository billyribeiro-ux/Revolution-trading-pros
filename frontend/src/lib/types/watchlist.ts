/**
 * Weekly Watchlist Types
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Type definitions for the Weekly Watchlist system
 * 
 * @version 1.0.0
 */

export interface WatchlistDate {
	date: string;
	spreadsheetUrl: string;
}

export interface WatchlistEntry {
	id: string;
	slug: string;
	title: string;
	trader: string;
	weekOf: string;
	publishedAt: string;
	poster: string;
	videoUrl: string;
	spreadsheetUrl: string;
	isLatest: boolean;
	watchlistDates?: WatchlistDate[];
}

export interface CreateWatchlistRequest {
	trader: string;
	weekOf: string;
	publishedAt: string;
	posterUrl: string;
	videoUrl: string;
	spreadsheetUrl: string;
	watchlistDates?: WatchlistDate[];
}

export interface WatchlistVideo {
	title: string;
	video_url: string;
	thumbnail_url?: string;
	formatted_duration?: string;
	formatted_date?: string;
}

export interface WatchlistResponse {
	entries: WatchlistEntry[];
	total: number;
	video?: WatchlistVideo;
	week_title?: string;
}
