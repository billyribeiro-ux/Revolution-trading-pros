/**
 * SPX Profit Pulse Dashboard API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT Level 7 Principal Engineer Implementation
 * Server-side API endpoint for SPX Profit Pulse dashboard data
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
		video_id?: string;
		thumbnail: string;
		duration?: number;
	};
	href: string;
	image: string;
	is_video: boolean;
	created_at: string;
	updated_at: string;
}

interface SPXPerformanceMetrics {
	win_rate: number;
	total_profit: number;
	total_trades: number;
	avg_risk_reward: number;
	monthly_return?: number;
	ytd_return?: number;
	streak?: {
		current: number;
		type: 'win' | 'loss';
	};
	last_updated: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// DATA GENERATORS
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

function generateAlerts(count: number = 6): SPXAlert[] {
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
				duration: 1800 + Math.floor(Math.random() * 1200) // 30-50 min
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

function getPerformanceMetrics(): SPXPerformanceMetrics {
	// In production, this would fetch from database/analytics service
	return {
		win_rate: 87,
		total_profit: 12450,
		total_trades: 42,
		avg_risk_reward: 1.8,
		monthly_return: 8.5,
		ytd_return: 34.2,
		streak: { current: 5, type: 'win' },
		last_updated: new Date().toISOString()
	};
}

function getTradingRooms() {
	return [
		{
			name: 'Day Trading Room',
			slug: 'day-trading-room',
			href: '/dashboard/day-trading-room',
			icon: 'chart-line'
		},
		{
			name: 'Swing Trading Room',
			slug: 'swing-trading-room',
			href: '/dashboard/swing-trading-room',
			icon: 'trending-up'
		},
		{
			name: 'Small Account Mentorship',
			slug: 'small-account-mentorship',
			href: '/dashboard/small-account-mentorship',
			icon: 'dollar-sign'
		}
	];
}

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ request, locals }) => {
	const token =
		request.headers.get('Authorization')?.replace('Bearer ', '') ||
		(locals as any).accessToken;
	if (!token) return json({ success: false, error: 'Unauthorized' }, { status: 401 });
	try {
		// Build full dashboard response
		const dashboardData = {
			alerts: generateAlerts(6),
			performance: getPerformanceMetrics(),
			traders: TRADERS.map((t) => ({
				id: t.id,
				name: t.name,
				slug: t.slug,
				title: 'SPX Options Strategist',
				bio: `Expert trader specializing in SPX options strategies.`,
				photo_url: t.photo_url,
				specialties: ['SPX Options', '0DTE Strategies', 'Technical Analysis']
			})),
			featured_video: {
				url: 'https://simpler-options.s3.amazonaws.com/Tr3ndy/TrendySPXQuickstart2025.mp4',
				poster: 'https://cdn.simplertrading.com/2025/07/02134158/FTR-Jonathan.png',
				title: 'SPX Profit Pulse Quickstart Guide'
			},
			trading_rooms: getTradingRooms()
		};

		return json(
			{
				success: true,
				data: dashboardData
			},
			{
				headers: {
					'Cache-Control': 'public, max-age=60, stale-while-revalidate=300'
				}
			}
		);
	} catch (error) {
		logger.error('[SPX API] Error:', error);
		return json({ success: false, error: 'Failed to fetch dashboard data' }, { status: 500 });
	}
};
