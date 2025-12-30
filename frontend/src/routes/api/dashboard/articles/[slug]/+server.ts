/**
 * Dashboard Articles by Room Slug API Endpoint
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Returns combined articles (daily videos + chatroom archives) for a trading room.
 * Provides a unified content feed for the dashboard.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

interface Article {
	id: number;
	type: 'daily-video' | 'chatroom-archive';
	label?: string;
	title: string;
	date: string;
	excerpt?: string;
	traderName?: string;
	href: string;
	image: string;
	videoUrl?: string;
	videoPlatform?: string;
	videoId?: string;
	duration?: number;
	isVideo?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// ROOM DATA
// ═══════════════════════════════════════════════════════════════════════════

const roomSlugToId: Record<string, number> = {
	'mastering-the-trade': 1,
	'simpler-central': 1,
	'simpler-showcase': 2,
	'day-trading-room': 3,
	'swing-trading-room': 4,
	'small-account-mentorship': 5,
	'spx-profit-pulse': 6,
	'explosive-swing': 7
};

const traders: Record<string, { name: string; initials: string; photo: string }> = {
	'hg': { name: 'Henry Gambell', initials: 'HG', photo: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg' },
	'dshay': { name: 'Danielle Shay', initials: 'DS', photo: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg' },
	'ss': { name: 'Sam Shames', initials: 'SS', photo: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg' },
	'bm': { name: 'Bruce Marshall', initials: 'BM', photo: 'https://cdn.simplertrading.com/2025/05/07135055/SimplerCentral_BMarshall.jpg' },
	'jc': { name: 'John Carter', initials: 'JC', photo: 'https://cdn.simplertrading.com/2025/05/07134409/SimplerCentral_JC.jpg' },
	'tg': { name: 'TG Watkins', initials: 'TG', photo: 'https://cdn.simplertrading.com/2025/05/07135326/SimplerCentral_TG.jpg' }
};

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA GENERATOR
// ═══════════════════════════════════════════════════════════════════════════

function generateMockArticles(slug: string): Article[] {
	const FALLBACK_PLACEHOLDER = 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg';
	const articles: Article[] = [];
	const today = new Date();

	// Generate articles for the last 7 days
	for (let i = 0; i < 7; i++) {
		const date = new Date(today);
		date.setDate(date.getDate() - i);

		// Skip weekends
		if (date.getDay() === 0 || date.getDay() === 6) continue;

		const dateStr = date.toLocaleDateString('en-US', {
			month: 'long',
			day: 'numeric',
			year: 'numeric'
		});
		const dateSlug = `${String(date.getMonth() + 1).padStart(2, '0')}${String(date.getDate()).padStart(2, '0')}${date.getFullYear()}`;

		// Rotate through traders
		const traderKeys = Object.keys(traders);
		const traderKey = traderKeys[i % traderKeys.length];
		const trader = traders[traderKey];

		// Daily Video
		articles.push({
			id: i * 2 + 1,
			type: 'daily-video',
			label: 'Daily Video',
			title: getDailyVideoTitle(i),
			date: `${dateStr} with ${trader.initials}`,
			excerpt: getDailyVideoExcerpt(i),
			href: `/dashboard/${slug}/daily-videos/${dateSlug}`,
			image: trader.photo,
			isVideo: true
		});

		// Chatroom Archive
		articles.push({
			id: i * 2 + 2,
			type: 'chatroom-archive',
			title: dateStr,
			date: dateStr,
			traderName: trader.name,
			href: `/dashboard/${slug}/chatroom-archives/${dateSlug}`,
			image: FALLBACK_PLACEHOLDER,
			isVideo: true
		});
	}

	return articles;
}

function getDailyVideoTitle(index: number): string {
	const titles = [
		"Santa's On His Way",
		'Setting Up for the Santa Rally',
		'Holiday Weekend Market Review',
		'Year-End Portfolio Strategies',
		'Final Trading Week of 2025',
		'Key Levels for the New Year',
		'Market Outlook December Review'
	];
	return titles[index % titles.length];
}

function getDailyVideoExcerpt(index: number): string {
	const excerpts = [
		"Things can always change, but given how the market closed, it looks like Santa's on his way. Let's look at the facts, then also some preferences and opinions.",
		"Everything looks good for a potential rally, as the indexes are consolidating and breaking higher. Let's take a look at TSLA, GOOGL, AMZN, AVGO, MSFT, and more.",
		"Indexes continue to churn sideways as we approach the holiday trade. Bulls usually take over in low volume. Can they do it again?",
		'Review your positions before year end. Tax loss harvesting and portfolio rebalancing are key strategies to consider.',
		"Last week of trading for 2025. Volume will be light but that doesn't mean opportunities won't present themselves.",
		'Key support and resistance levels to watch as we transition into the new year.',
		'December has been a solid month. Here is a recap of the major market moves and what they mean.'
	];
	return excerpts[index % excerpts.length];
}

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND FETCH
// ═══════════════════════════════════════════════════════════════════════════

async function fetchFromBackend(endpoint: string, authHeader?: string): Promise<any | null> {
	const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			'Accept': 'application/json'
		};

		if (authHeader) {
			headers['Authorization'] = authHeader;
		}

		const response = await fetch(`${BACKEND_URL}${endpoint}`, { headers });

		if (!response.ok) return null;
		return await response.json();
	} catch {
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// GET HANDLER
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, url, request }) => {
	const { slug } = params;

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	const roomId = roomSlugToId[slug];
	if (!roomId) {
		throw error(404, `Trading room '${slug}' not found`);
	}

	// Parse query params
	const page = parseInt(url.searchParams.get('page') || '1');
	const perPage = parseInt(url.searchParams.get('per_page') || '10');
	const type = url.searchParams.get('type') as 'daily-video' | 'chatroom-archive' | null;
	const search = url.searchParams.get('search');

	// Try backend first
	const authHeader = request.headers.get('Authorization') || undefined;
	const backendData = await fetchFromBackend(`/api/dashboard/articles/${slug}?${url.searchParams.toString()}`, authHeader);

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	let articles = generateMockArticles(slug);

	// Filter by type
	if (type) {
		articles = articles.filter(a => a.type === type);
	}

	// Search filter
	if (search) {
		const searchLower = search.toLowerCase();
		articles = articles.filter(a =>
			a.title.toLowerCase().includes(searchLower) ||
			a.excerpt?.toLowerCase().includes(searchLower) ||
			a.traderName?.toLowerCase().includes(searchLower)
		);
	}

	// Paginate
	const total = articles.length;
	const start = (page - 1) * perPage;
	const paginatedArticles = articles.slice(start, start + perPage);

	return json({
		success: true,
		data: paginatedArticles,
		pagination: {
			current_page: page,
			per_page: perPage,
			total,
			last_page: Math.ceil(total / perPage)
		},
		_mock: true
	});
};
