/**
 * Watchlist Rundown Archive - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Fetches watchlist rundown videos from the backend API
 */

import type { ServerLoadEvent } from '@sveltejs/kit';

export async function load({ fetch }: ServerLoadEvent) {
	try {
		// Fetch watchlist entries from backend
		const response = await fetch('https://revolution-trading-pros-api.fly.dev/api/watchlist/entries');
		
		if (!response.ok) {
			console.error('Failed to fetch watchlist entries:', response.statusText);
			return {
				videos: []
			};
		}

		const data = await response.json();
		
		// Filter and map to video format
		const videos = (data.entries || []).map((entry: any) => ({
			id: entry.id,
			slug: entry.slug,
			title: entry.title,
			date: entry.publish_date,
			weekOf: entry.week_of,
			image: entry.thumbnail_url || entry.image_url,
			href: `/watchlist/${entry.slug}`,
			description: entry.description || entry.week_of
		}));

		return {
			videos
		};
	} catch (error) {
		console.error('Error loading watchlist rundown archive:', error);
		return {
			videos: []
		};
	}
};
