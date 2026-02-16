/**
 * SPX Profit Pulse Alerts API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT Level 7 Principal Engineer Implementation
 * Paginated alerts/videos endpoint
 *
 * @version 1.0.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

interface SPXAlert {
	id: number;
	slug: string;
	type: 'daily_video' | 'trade_alert' | 'market_update';
	title: string;
	date: string;
	excerpt: string;
	trader: {
		id: number;
		name: string;
		slug: string;
		photo_url?: string;
	};
	video?: {
		url: string;
		platform: string;
		thumbnail: string;
		duration?: number;
	};
	href: string;
	image: string;
	is_video: boolean;
	created_at: string;
	updated_at: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// DATA
// ═══════════════════════════════════════════════════════════════════════════

const TRADERS = [
	{
		id: 1,
		name: 'Billy Ribeiro',
		slug: 'billy-ribeiro',
		photo_url: 'https://cdn.simplertrading.com/traders/billy-ribeiro.jpg'
	},
	{
		id: 2,
		name: 'Freddie Ferber',
		slug: 'freddie-ferber',
		photo_url: 'https://cdn.simplertrading.com/traders/freddie-ferber.jpg'
	},
	{
		id: 3,
		name: 'Shao Wan',
		slug: 'shao-wan',
		photo_url: 'https://cdn.simplertrading.com/traders/shao-wan.jpg'
	},
	{
		id: 4,
		name: 'Melissa Beegle',
		slug: 'melissa-beegle',
		photo_url: 'https://cdn.simplertrading.com/traders/melissa-beegle.jpg'
	},
	{
		id: 5,
		name: 'Jonathan McKeever',
		slug: 'jonathan-mckeever',
		photo_url: 'https://cdn.simplertrading.com/traders/jonathan-mckeever.jpg'
	}
];

function generateAlerts(count: number = 30): SPXAlert[] {
	const alerts: SPXAlert[] = [];
	const baseDate = new Date();

	for (let i = 0; i < count; i++) {
		const date = new Date(baseDate);
		date.setDate(date.getDate() - i);

		// Skip weekends
		const dayOfWeek = date.getDay();
		if (dayOfWeek === 0) date.setDate(date.getDate() - 2);
		if (dayOfWeek === 6) date.setDate(date.getDate() - 1);

		const trader = TRADERS[i % TRADERS.length];
		const dateStr = date.toLocaleDateString('en-US', {
			month: 'long',
			day: 'numeric',
			year: 'numeric'
		});
		const slug = dateStr.toLowerCase().replace(/[,\s]+/g, '-');

		alerts.push({
			id: i + 1,
			slug,
			type: 'daily_video',
			title: dateStr,
			date: dateStr,
			excerpt: `With ${trader.name}`,
			trader: {
				id: trader.id,
				name: trader.name,
				slug: trader.slug,
				photo_url: trader.photo_url
			},
			video: {
				url: `https://simpler-options.s3.amazonaws.com/Tr3ndy/spx-daily-${slug}.mp4`,
				platform: 'direct',
				thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
				duration: 1800 + Math.floor(Math.random() * 1200)
			},
			href: `/dashboard/spx-profit-pulse/alerts/${slug}`,
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			is_video: true,
			created_at: date.toISOString(),
			updated_at: date.toISOString()
		});
	}

	return alerts;
}

// Cache alerts for performance
let cachedAlerts: SPXAlert[] | null = null;
let cacheTime = 0;
const CACHE_TTL = 60000; // 1 minute

function getAlerts(): SPXAlert[] {
	const now = Date.now();
	if (!cachedAlerts || now - cacheTime > CACHE_TTL) {
		cachedAlerts = generateAlerts(30);
		cacheTime = now;
	}
	return cachedAlerts;
}

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST HANDLER
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ url }) => {
	try {
		const page = parseInt(url.searchParams.get('page') || '1');
		const perPage = parseInt(url.searchParams.get('per_page') || '6');
		const type = url.searchParams.get('type');

		let alerts = getAlerts();

		// Filter by type if specified
		if (type) {
			alerts = alerts.filter((a) => a.type === type);
		}

		const total = alerts.length;
		const lastPage = Math.ceil(total / perPage);
		const start = (page - 1) * perPage;
		const items = alerts.slice(start, start + perPage);

		return json(
			{
				success: true,
				data: {
					items,
					current_page: page,
					last_page: lastPage,
					per_page: perPage,
					total
				}
			},
			{
				headers: {
					'Cache-Control': 'public, max-age=60, stale-while-revalidate=300'
				}
			}
		);
	} catch (error) {
		logger.error('[SPX Alerts API] Error:', error);
		return json({ success: false, error: 'Failed to fetch alerts' }, { status: 500 });
	}
};
