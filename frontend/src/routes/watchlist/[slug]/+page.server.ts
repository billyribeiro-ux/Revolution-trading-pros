/**
 * Weekly Watchlist Single Item - Server Load
 * ═══════════════════════════════════════════════════════════════════════════
 * Handles data loading for individual watchlist items
 * @version 1.0.0 - December 2025
 */

import type { PageServerLoad } from './$types';

interface WatchlistItem {
	slug: string;
	title: string;
	subtitle: string;
	trader: string;
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

// Mock data - replace with actual API call in production
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

export const load: PageServerLoad = async ({ params }) => {
	const { slug } = params;

	// Try to find the watchlist item
	let watchlistItem = mockWatchlistItems[slug];

	// If not found, return a default/fallback
	if (!watchlistItem) {
		// Parse the slug to extract date and trader name
		const parts = slug.split('-');
		const dateStr = parts[0] || '';
		const traderName = parts.slice(1).map(p => p.charAt(0).toUpperCase() + p.slice(1)).join(' ');

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
