/**
 * SPX Profit Pulse Premium Videos - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * ICT 11+ Principal Engineer Grade
 * Fetches videos from backend API with pagination support
 * 
 * SSR Configuration (SvelteKit Official Docs):
 * - ssr: true - Server-side render for SEO and fast initial load
 * - csr: true - Enable client-side hydration for interactivity
 * - prerender: false - Dynamic content, cannot be prerendered
 * 
 * @version 1.0.0 - January 2026
 */

import type { RequestEvent } from '@sveltejs/kit';

// SSR/SSG Configuration - Per SvelteKit Official Docs
export const ssr = true;      // Enable server-side rendering
export const csr = true;      // Enable client-side hydration
export const prerender = false; // Dynamic content - cannot prerender

// ICT 7 FIX: VITE_API_URL does NOT include /api suffix - we add it here
const API_ROOT = import.meta.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = `${API_ROOT}/api`;

export interface PremiumVideo {
	id: string;
	title: string;
	slug: string;
	date: string;
	trader: string;
	excerpt: string;
	thumbnail: string;
	isVideo: boolean;
}

export interface PageData {
	videos: PremiumVideo[];
	pagination: {
		page: number;
		perPage: number;
		total: number;
		totalPages: number;
	};
	error?: string;
}

export const load = async ({ url, fetch, cookies }: RequestEvent) => {
	const page = parseInt(url.searchParams.get('page') || '1');
	const search = url.searchParams.get('search') || '';
	const perPage = 12;

	try {
		// Try to fetch from backend API
		const apiUrl = new URL(`${BACKEND_URL}/trading-rooms/spx-profit-pulse/videos`);
		apiUrl.searchParams.set('page', page.toString());
		apiUrl.searchParams.set('per_page', perPage.toString());
		if (search) {
			apiUrl.searchParams.set('search', search);
		}

		// Get auth token if available
		const token = cookies.get('auth_token');
		const headers: HeadersInit = {
			'Accept': 'application/json',
			'Content-Type': 'application/json',
		};
		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const response = await fetch(apiUrl.toString(), { headers });

		if (response.ok) {
			const data = await response.json();
			
			// Transform backend response to frontend format
			const videos: PremiumVideo[] = (data.data?.videos || []).map((v: any) => ({
				id: v.id,
				title: v.title,
				slug: v.slug,
				date: formatDate(v.published_at),
				trader: v.trader?.name || 'Trading Team',
				excerpt: v.description || '',
				thumbnail: v.thumbnail_url || 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
				isVideo: true,
			}));

			return {
				videos,
				pagination: {
					page: data.data?.pagination?.page || page,
					perPage: data.data?.pagination?.per_page || perPage,
					total: data.data?.pagination?.total || 0,
					totalPages: data.data?.pagination?.total_pages || 0,
				},
			} satisfies PageData;
		}

		// If API fails, return fallback mock data for development
		console.warn('Backend API unavailable, using mock data');
		return getMockData(page, perPage, search);

	} catch (error) {
		console.error('Failed to fetch videos:', error);
		// Return mock data as fallback
		return getMockData(page, perPage, search);
	}
};

function formatDate(dateStr: string): string {
	try {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: '2-digit',
		});
	} catch {
		return dateStr;
	}
}

function getMockData(page: number, perPage: number, search: string): PageData {
	// Fallback mock data matching WordPress reference
	const allVideos: PremiumVideo[] = [
		{
			id: '1',
			title: 'Can the Nasdaq Finally Make it to New All Time Highs?',
			slug: 'can-the-nasdaq',
			date: 'January 12, 2026',
			trader: 'Melissa Beegle',
			excerpt: 'I am bullish going into this week, with NQ making higher highs above last week\'s high. If the NQ can hold 25880 as support this week, then I am looking for it to trade into 26120- 26400. I am watching NVDA, ASTS, and GLD for pullback buys.',
			thumbnail: 'https://cdn.simplertrading.com/2026/01/12162748/MTT-MB.jpg',
			isVideo: true
		},
		{
			id: '2',
			title: 'Get Ready For Banks To Report',
			slug: 'get-ready-for-banks-to-report',
			date: 'January 10, 2026',
			trader: 'Tr3ndy Jon',
			excerpt: 'Here Tr3ndy Jon covers the XLF and his trades on GS and JPM.',
			thumbnail: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
			isVideo: true
		},
		{
			id: '3',
			title: 'Can $PLTR Sustain Its Bullish Trajectory?',
			slug: 'can-pltr-sustain-its-bullish-trajectory',
			date: 'January 08, 2026',
			trader: 'Tr3ndy Jon',
			excerpt: 'Looking at the EMA cloud, the gap up from earnings, and the 9 EMA. Will this be a stair stepper?',
			thumbnail: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
			isVideo: true
		},
		{
			id: '4',
			title: 'How Tr3ndy Jon Is Trading TESLA Into Earnings',
			slug: 'trading-tesla-into-earnings',
			date: 'January 06, 2026',
			trader: 'Tr3ndy Jon',
			excerpt: 'How Tr3ndy Jon is trading TESLA into earnings.',
			thumbnail: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
			isVideo: true
		},
		{
			id: '5',
			title: 'Master Your Week',
			slug: 'master-your-week',
			date: 'January 05, 2026',
			trader: 'Tr3ndy Jon',
			excerpt: 'Navigating ES Futures, SPX, NVDA, and TSLA for Stock Market Success.',
			thumbnail: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
			isVideo: true
		},
		{
			id: '6',
			title: 'Is The Semiconductor Space Ready To Push Higher?',
			slug: 'is-the-semiconductor-space-ready-to-push-higher',
			date: 'January 03, 2026',
			trader: 'Tr3ndy Jon',
			excerpt: 'Tr3ndy Jon covers: SMH, NVDA, AMD.',
			thumbnail: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
			isVideo: true
		},
		// Generate more mock videos for pagination testing
		...Array.from({ length: 6 }, (_, i) => ({
			id: String(i + 7),
			title: `SPX Premium Analysis ${i + 7}`,
			slug: `spx-analysis-${i + 7}`,
			date: `December ${25 - i}, 2025`,
			trader: i % 2 === 0 ? 'Tr3ndy Jon' : 'Melissa Beegle',
			excerpt: 'Expert SPX analysis and premium trading insights for today\'s market conditions.',
			thumbnail: i % 2 === 0 
				? 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg'
				: 'https://cdn.simplertrading.com/2026/01/12162748/MTT-MB.jpg',
			isVideo: true
		}))
	];

	// Filter by search
	let filtered = allVideos;
	if (search) {
		const searchLower = search.toLowerCase();
		filtered = allVideos.filter(v => 
			v.title.toLowerCase().includes(searchLower) ||
			v.trader.toLowerCase().includes(searchLower)
		);
	}

	// Paginate
	const total = filtered.length;
	const totalPages = Math.ceil(total / perPage);
	const start = (page - 1) * perPage;
	const videos = filtered.slice(start, start + perPage);

	return {
		videos,
		pagination: {
			page,
			perPage,
			total,
			totalPages,
		},
	};
}
