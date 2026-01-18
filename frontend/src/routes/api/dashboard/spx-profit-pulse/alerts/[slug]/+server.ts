/**
 * SPX Profit Pulse Single Alert API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT Level 7 Principal Engineer Implementation
 * Get single alert/video by slug
 *
 * @version 1.0.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

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
	description: string;
	trader: {
		id: number;
		name: string;
		slug: string;
		title: string;
		photo_url: string;
		bio: string;
	};
	video: {
		url: string;
		platform: string;
		thumbnail: string;
		duration: number;
	};
	href: string;
	image: string;
	is_video: boolean;
	related_alerts: Array<{
		id: number;
		slug: string;
		title: string;
		date: string;
		image: string;
		href: string;
	}>;
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
		title: 'Lead SPX Strategist',
		photo_url: 'https://cdn.simplertrading.com/traders/billy-ribeiro.jpg',
		bio: 'Billy Ribeiro is the lead SPX strategist with over 15 years of experience in options trading. He specializes in 0DTE SPX strategies and has developed proprietary methods for consistent profits.'
	},
	{ 
		id: 2, 
		name: 'Freddie Ferber', 
		slug: 'freddie-ferber',
		title: 'Senior Options Trader',
		photo_url: 'https://cdn.simplertrading.com/traders/freddie-ferber.jpg',
		bio: 'Freddie brings 20+ years of market experience to SPX Profit Pulse. His methodical approach to risk management has helped thousands of traders improve their win rates.'
	},
	{ 
		id: 3, 
		name: 'Shao Wan', 
		slug: 'shao-wan',
		title: 'Technical Analysis Expert',
		photo_url: 'https://cdn.simplertrading.com/traders/shao-wan.jpg',
		bio: 'Shao specializes in technical analysis and chart patterns. Her insights on market structure help traders identify high-probability setups.'
	},
	{ 
		id: 4, 
		name: 'Melissa Beegle', 
		slug: 'melissa-beegle',
		title: 'Options Educator',
		photo_url: 'https://cdn.simplertrading.com/traders/melissa-beegle.jpg',
		bio: 'Melissa is passionate about educating traders on options strategies. Her clear explanations make complex concepts accessible to all skill levels.'
	},
	{ 
		id: 5, 
		name: 'Jonathan McKeever', 
		slug: 'jonathan-mckeever',
		title: 'Market Strategist',
		photo_url: 'https://cdn.simplertrading.com/traders/jonathan-mckeever.jpg',
		bio: 'Jonathan combines fundamental and technical analysis to provide comprehensive market insights. His daily briefings are a must-watch for serious traders.'
	}
];

function parseSlugToDate(slug: string): Date | null {
	// Parse slug like "january-15-2026" back to date
	const parts = slug.split('-');
	if (parts.length < 3) return null;
	
	const months: Record<string, number> = {
		january: 0, february: 1, march: 2, april: 3, may: 4, june: 5,
		july: 6, august: 7, september: 8, october: 9, november: 10, december: 11
	};
	
	const month = months[parts[0].toLowerCase()];
	const day = parseInt(parts[1]);
	const year = parseInt(parts[2]);
	
	if (month === undefined || isNaN(day) || isNaN(year)) return null;
	
	return new Date(year, month, day);
}

function generateRelatedAlerts(currentSlug: string): Array<{
	id: number;
	slug: string;
	title: string;
	date: string;
	image: string;
	href: string;
}> {
	const related = [];
	const currentDate = parseSlugToDate(currentSlug) || new Date();
	
	for (let i = 1; i <= 3; i++) {
		const date = new Date(currentDate);
		date.setDate(date.getDate() - i);
		
		// Skip weekends
		if (date.getDay() === 0) date.setDate(date.getDate() - 2);
		if (date.getDay() === 6) date.setDate(date.getDate() - 1);
		
		const dateStr = date.toLocaleDateString('en-US', {
			month: 'long',
			day: 'numeric',
			year: 'numeric'
		});
		const slug = dateStr.toLowerCase().replace(/[,\s]+/g, '-');
		
		related.push({
			id: i + 100,
			slug,
			title: dateStr,
			date: dateStr,
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			href: `/dashboard/spx-profit-pulse/alerts/${slug}`
		});
	}
	
	return related;
}

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST HANDLER
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params }) => {
	try {
		const { slug } = params;
		
		if (!slug) {
			return json({ success: false, error: 'Slug is required' }, { status: 400 });
		}

		const date = parseSlugToDate(slug);
		
		if (!date) {
			return json({ success: false, error: 'Invalid alert slug' }, { status: 404 });
		}

		const dateStr = date.toLocaleDateString('en-US', {
			month: 'long',
			day: 'numeric',
			year: 'numeric'
		});

		// Determine trader based on date
		const dayOfYear = Math.floor((date.getTime() - new Date(date.getFullYear(), 0, 0).getTime()) / 86400000);
		const trader = TRADERS[dayOfYear % TRADERS.length];

		const alert: SPXAlert = {
			id: dayOfYear,
			slug,
			type: 'daily_video',
			title: dateStr,
			date: dateStr,
			excerpt: `Daily SPX analysis and trade alerts with ${trader.name}`,
			description: `Join ${trader.name} for today's comprehensive SPX market analysis. In this session, we cover key levels, potential trade setups, and real-time alerts to help you navigate the markets with confidence. ${trader.bio}`,
			trader: {
				id: trader.id,
				name: trader.name,
				slug: trader.slug,
				title: trader.title,
				photo_url: trader.photo_url,
				bio: trader.bio
			},
			video: {
				url: `https://simpler-options.s3.amazonaws.com/Tr3ndy/spx-daily-${slug}.mp4`,
				platform: 'direct',
				thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
				duration: 1800 + (dayOfYear % 1200) // 30-50 min
			},
			href: `/dashboard/spx-profit-pulse/alerts/${slug}`,
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			is_video: true,
			related_alerts: generateRelatedAlerts(slug),
			created_at: date.toISOString(),
			updated_at: date.toISOString()
		};

		return json({
			success: true,
			data: alert
		}, {
			headers: {
				'Cache-Control': 'public, max-age=3600, stale-while-revalidate=86400'
			}
		});
	} catch (error) {
		console.error('[SPX Alert API] Error:', error);
		return json(
			{ success: false, error: 'Failed to fetch alert' },
			{ status: 500 }
		);
	}
};
