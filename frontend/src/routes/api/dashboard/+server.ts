/**
 * Dashboard API - Main Dashboard Data Endpoint
 *
 * Provides aggregated dashboard data including:
 * - User's active memberships
 * - Recent activity
 * - Featured content
 * - Quick links
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Dashboard data types
interface DashboardData {
	user: {
		id: string;
		name: string;
		email: string;
		avatar_url?: string;
		member_since: string;
	};
	memberships: DashboardMembership[];
	recent_activity: ActivityItem[];
	featured_content: FeaturedContent[];
	announcements: Announcement[];
	stats: DashboardStats;
}

interface DashboardMembership {
	id: string;
	slug: string;
	name: string;
	thumbnail_url: string;
	category: string;
	status: string;
	expires_at?: string;
	has_trading_room: boolean;
	next_session?: {
		title: string;
		starts_at: string;
	};
}

interface ActivityItem {
	id: string;
	type: 'video_watched' | 'trade_alert' | 'webinar_attended' | 'course_progress';
	title: string;
	description: string;
	timestamp: string;
	link?: string;
}

interface FeaturedContent {
	id: string;
	title: string;
	description: string;
	thumbnail_url: string;
	type: string;
	link: string;
}

interface Announcement {
	id: string;
	title: string;
	content: string;
	type: 'info' | 'warning' | 'success' | 'urgent';
	link?: string;
	created_at: string;
}

interface DashboardStats {
	videos_watched: number;
	trading_room_hours: number;
	courses_completed: number;
	alerts_received: number;
}

// GET - Get dashboard data
export const GET: RequestHandler = async ({ url, cookies }) => {
	// In production, get user from session/JWT
	const userId = url.searchParams.get('user_id') || 'demo_user';

	// Simulated dashboard data
	const dashboardData: DashboardData = {
		user: {
			id: userId,
			name: 'Demo Trader',
			email: 'demo@revolutiontrading.com',
			avatar_url: '/images/avatars/default.jpg',
			member_since: '2024-01-15T00:00:00Z'
		},
		memberships: [
			{
				id: 'mem_1',
				slug: 'day-trading-room',
				name: 'Day Trading Room',
				thumbnail_url: '/images/memberships/day-trading-room.jpg',
				category: 'trading-room',
				status: 'active',
				expires_at: '2025-01-15T00:00:00Z',
				has_trading_room: true,
				next_session: {
					title: 'Morning Trading Session',
					starts_at: getNextTradingSession()
				}
			},
			{
				id: 'mem_4',
				slug: 'squeeze-pro',
				name: 'Squeeze Pro Indicator',
				thumbnail_url: '/images/memberships/squeeze-pro.jpg',
				category: 'tools',
				status: 'active',
				has_trading_room: false
			}
		],
		recent_activity: [
			{
				id: 'act_1',
				type: 'video_watched',
				title: 'Completed: Analysis of A Trade',
				description: 'You watched 70 minutes of educational content',
				timestamp: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(),
				link: '/dashboard/day-trading-room/learning-center/analysis-of-a-trade'
			},
			{
				id: 'act_2',
				type: 'trade_alert',
				title: 'Trade Alert: SPY Call Options',
				description: 'New trade alert received',
				timestamp: new Date(Date.now() - 4 * 60 * 60 * 1000).toISOString(),
				link: '/dashboard/day-trading-room/alerts'
			},
			{
				id: 'act_3',
				type: 'webinar_attended',
				title: 'Attended: Weekly Market Outlook',
				description: 'Live webinar with John Carter',
				timestamp: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString()
			}
		],
		featured_content: [
			{
				id: 'fc_1',
				title: 'Weekly Watchlist',
				description: 'This week\'s top trading opportunities',
				thumbnail_url: '/images/featured/weekly-watchlist.jpg',
				type: 'watchlist',
				link: '/dashboard/day-trading-room/watchlist'
			},
			{
				id: 'fc_2',
				title: 'New: Options Flow Analysis',
				description: 'Learn to read options flow like a pro',
				thumbnail_url: '/images/featured/options-flow.jpg',
				type: 'video',
				link: '/dashboard/day-trading-room/learning-center/options-flow-analysis'
			}
		],
		announcements: [
			{
				id: 'ann_1',
				title: 'Holiday Trading Schedule',
				content: 'Trading room hours adjusted for the upcoming holiday. Check schedule for details.',
				type: 'info',
				link: '/dashboard/schedule',
				created_at: new Date(Date.now() - 48 * 60 * 60 * 1000).toISOString()
			}
		],
		stats: {
			videos_watched: 47,
			trading_room_hours: 156,
			courses_completed: 3,
			alerts_received: 234
		}
	};

	return json({
		success: true,
		data: dashboardData
	});
};

// Helper function to get next trading session time
function getNextTradingSession(): string {
	const now = new Date();
	const marketOpen = new Date(now);
	marketOpen.setHours(9, 0, 0, 0);

	const marketClose = new Date(now);
	marketClose.setHours(16, 0, 0, 0);

	// If it's before market open today, next session is today
	if (now < marketOpen) {
		return marketOpen.toISOString();
	}

	// If market is currently open, return current time
	if (now >= marketOpen && now < marketClose) {
		return now.toISOString();
	}

	// Otherwise, next session is tomorrow
	const tomorrow = new Date(now);
	tomorrow.setDate(tomorrow.getDate() + 1);

	// Skip weekends
	if (tomorrow.getDay() === 0) {
		tomorrow.setDate(tomorrow.getDate() + 1); // Sunday -> Monday
	} else if (tomorrow.getDay() === 6) {
		tomorrow.setDate(tomorrow.getDate() + 2); // Saturday -> Monday
	}

	tomorrow.setHours(9, 0, 0, 0);
	return tomorrow.toISOString();
}
