/**
 * Weekly Watchlist Single Item - Server Load
 * ═══════════════════════════════════════════════════════════════════════════
 * Fetches watchlist data from API with mock fallback
 * @version 2.0.0 - December 2025 - Connected to API
 */

import type { ServerLoad } from '@sveltejs/kit';

interface WatchlistItem {
	slug: string;
	title: string;
	subtitle: string;
	trader: string;
	traderImage?: string;
	datePosted: string;
	video: {
		src: string;
		poster: string;
		title: string;
	};
	spreadsheet: {
		src: string;
	};
	description: string;
	previous: {
		slug: string;
		title: string;
	} | null;
	next: {
		slug: string;
		title: string;
	} | null;
}

// Fallback mock data - used when API is unavailable
const mockWatchlistItems: Record<string, WatchlistItem> = {
	'12222025-tg-watkins': {
		slug: '12222025-tg-watkins',
		title: 'Weekly Watchlist with TG Watkins',
		subtitle: 'Week of December 22, 2025',
		trader: 'TG Watkins',
		datePosted: 'December 22, 2025',
		video: {
			src: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-TG-12222025.mp4',
			poster: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			title: 'Weekly Watchlist with TG Watkins'
		},
		spreadsheet: {
			src: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml'
		},
		description: 'Week of December 22, 2025.',
		previous: {
			slug: '12152025-allison-ostrander',
			title: 'Weekly Watchlist with Allison Ostrander'
		},
		next: null
	},
	'12152025-allison-ostrander': {
		slug: '12152025-allison-ostrander',
		title: 'Weekly Watchlist with Allison Ostrander',
		subtitle: 'Week of December 15, 2025',
		trader: 'Allison Ostrander',
		datePosted: 'December 15, 2025',
		video: {
			src: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-AO-12152025.mp4',
			poster: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg',
			title: 'Weekly Watchlist with Allison Ostrander'
		},
		spreadsheet: {
			src: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml'
		},
		description: 'Week of December 15, 2025.',
		previous: {
			slug: '12082025-taylor-horton',
			title: 'Weekly Watchlist with Taylor Horton'
		},
		next: {
			slug: '12222025-tg-watkins',
			title: 'Weekly Watchlist with TG Watkins'
		}
	},
	'12082025-taylor-horton': {
		slug: '12082025-taylor-horton',
		title: 'Weekly Watchlist with Taylor Horton',
		subtitle: 'Week of December 8, 2025',
		trader: 'Taylor Horton',
		datePosted: 'December 8, 2025',
		video: {
			src: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-TH-12082025.mp4',
			poster: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Taylor-Watchlist-Rundown.jpg',
			title: 'Weekly Watchlist with Taylor Horton'
		},
		spreadsheet: {
			src: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml'
		},
		description: 'Week of December 8, 2025.',
		previous: null,
		next: {
			slug: '12152025-allison-ostrander',
			title: 'Weekly Watchlist with Allison Ostrander'
		}
	}
};

/**
 * Fetch watchlist item from internal API
 */
async function fetchFromAPI(slug: string, origin: string): Promise<WatchlistItem | null> {
	try {
		const response = await fetch(`${origin}/api/watchlist/${slug}`, {
			headers: {
				'Accept': 'application/json'
			}
		});

		if (!response.ok) return null;

		const data = await response.json();
		if (!data.success) return null;

		// Map API response to expected format
		const item = data.data;
		return {
			slug: item.slug,
			title: item.title,
			subtitle: item.subtitle,
			trader: item.trader,
			traderImage: item.traderImage,
			datePosted: item.datePosted,
			video: item.video,
			spreadsheet: item.spreadsheet,
			description: item.description,
			previous: item.previous || null,
			next: item.next || null
		};
	} catch {
		return null;
	}
}

export const load: ServerLoad = async ({ params, url }) => {
	const { slug } = params;

	// Try to fetch from API first
	const origin = url.origin;
	const apiData = await fetchFromAPI(slug, origin);

	if (apiData) {
		return { watchlist: apiData };
	}

	// Fallback to mock data
	let watchlistItem = mockWatchlistItems[slug];

	// If not found, create a fallback from slug
	if (!watchlistItem) {
		const parts = slug.split('-');
		const dateStr = parts[0] || '';
		const traderName = parts.slice(1).map((p: string) => p.charAt(0).toUpperCase() + p.slice(1)).join(' ');

		watchlistItem = {
			slug,
			title: `Weekly Watchlist with ${traderName || 'Unknown Trader'}`,
			subtitle: `Week of ${formatDate(dateStr)}`,
			trader: traderName || 'Unknown',
			datePosted: formatDate(dateStr),
			video: {
				src: '',
				poster: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
				title: `Weekly Watchlist with ${traderName || 'Unknown Trader'}`
			},
			spreadsheet: {
				src: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml'
			},
			description: `Week of ${formatDate(dateStr)}.`,
			previous: null,
			next: null
		};
	}

	return {
		watchlist: watchlistItem
	};
};

function formatDate(dateStr: string): string {
	if (!dateStr || dateStr.length !== 8) return 'Unknown Date';

	const month = dateStr.substring(0, 2);
	const day = dateStr.substring(2, 4);
	const year = dateStr.substring(4, 8);

	const months = [
		'January', 'February', 'March', 'April', 'May', 'June',
		'July', 'August', 'September', 'October', 'November', 'December'
	];

	const monthName = months[parseInt(month, 10) - 1] || 'Unknown';
	return `${monthName} ${parseInt(day, 10)}, ${year}`;
}
