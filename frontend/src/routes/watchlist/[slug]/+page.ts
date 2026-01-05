/**
 * Watchlist Page Data Loader
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Fetches watchlist entry data from API
 * 
 * @version 1.0.0
 */

import type { WatchlistItem } from '$lib/api/watchlist';

export const load = async ({ params, fetch }: { params: { slug: string }, fetch: typeof globalThis.fetch }) => {
	try {
		// Fetch all watchlist entries
		const response = await fetch('/api/watchlist?status=published');
		const data = await response.json();
		
		if (!data.success || !data.data) {
			return {
				entries: [],
				currentEntry: null,
				previousEntry: null,
				nextEntry: null
			};
		}
		
		const entries: WatchlistItem[] = data.data;
		
		// Handle "latest" or "current" slug
		let slug = params.slug;
		if (slug === 'latest' || slug === 'current') {
			slug = entries[0]?.slug || '';
		}
		
		// Find current entry
		const currentIndex = entries.findIndex(e => e.slug === slug);
		
		if (currentIndex === -1) {
			return {
				entries,
				currentEntry: null,
				previousEntry: null,
				nextEntry: null
			};
		}
		
		const currentEntry = entries[currentIndex];
		
		// Previous = older (higher index)
		const previousEntry = currentIndex < entries.length - 1 
			? entries[currentIndex + 1] 
			: null;
		
		// Next = newer (lower index)
		const nextEntry = currentIndex > 0 
			? entries[currentIndex - 1] 
			: null;
		
		return {
			entries,
			currentEntry,
			previousEntry,
			nextEntry
		};
	} catch (error) {
		console.error('Failed to load watchlist data:', error);
		return {
			entries: [],
			currentEntry: null,
			previousEntry: null,
			nextEntry: null
		};
	}
};
