/**
 * Dashboard Memberships API - Real Membership Management
 *
 * Manages user memberships including:
 * - Active memberships
 * - Membership tiers and access levels
 * - Subscription status
 * - Member benefits
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Membership Types
interface Membership {
	id: string;
	slug: string;
	name: string;
	description: string;
	short_description: string;
	thumbnail_url: string;
	banner_url?: string;
	tier: 'basic' | 'pro' | 'elite' | 'lifetime';
	category: 'trading-room' | 'alerts' | 'mastery' | 'tools' | 'bundle';
	features: string[];
	price_monthly?: number;
	price_annual?: number;
	price_lifetime?: number;
	has_trading_room: boolean;
	has_alerts: boolean;
	has_learning_center: boolean;
	has_daily_videos: boolean;
	instructor?: {
		id: string;
		name: string;
		avatar_url?: string;
	};
	trading_room_schedule?: {
		morning?: string;
		afternoon?: string;
		evening?: string;
	};
	is_active: boolean;
	is_featured: boolean;
	sort_order: number;
	created_at: string;
	updated_at: string;
}

interface UserMembership {
	id: string;
	user_id: string;
	membership_id: string;
	membership: Membership;
	status: 'active' | 'paused' | 'cancelled' | 'expired';
	subscription_type: 'monthly' | 'annual' | 'lifetime';
	started_at: string;
	expires_at?: string;
	cancelled_at?: string;
	auto_renew: boolean;
	payment_method?: string;
}

// In-memory storage
const memberships: Map<string, Membership> = new Map();
const userMemberships: Map<string, UserMembership[]> = new Map();

// Initialize sample data
function initializeSampleData() {
	if (memberships.size > 0) return;

	const sampleMemberships: Membership[] = [
		{
			id: 'mem_1',
			slug: 'mastering-the-trade',
			name: 'Mastering the Trade',
			description: 'Learn to master the markets with our comprehensive trading room and educational content.',
			short_description: 'Live trading room with daily analysis and education',
			thumbnail_url: '/images/memberships/mastering-the-trade.jpg',
			banner_url: '/images/memberships/mastering-the-trade-banner.jpg',
			tier: 'pro',
			category: 'trading-room',
			features: [
				'Daily Live Trading Room (9:00 AM - 4:00 PM ET)',
				'Real-time Trade Alerts',
				'Access to Learning Center (400+ Videos)',
				'Weekly Member Webinars',
				'Discord Community Access',
				'Mobile App Access'
			],
			price_monthly: 297,
			price_annual: 2497,
			price_lifetime: 4997,
			has_trading_room: true,
			has_alerts: true,
			has_learning_center: true,
			has_daily_videos: true,
			instructor: {
				id: 'inst_1',
				name: 'John Carter',
				avatar_url: '/images/instructors/john-carter.jpg'
			},
			trading_room_schedule: {
				morning: '9:00 AM - 11:30 AM ET',
				afternoon: '1:00 PM - 4:00 PM ET'
			},
			is_active: true,
			is_featured: true,
			sort_order: 1,
			created_at: '2020-01-01T00:00:00Z',
			updated_at: '2024-06-01T00:00:00Z'
		},
		{
			id: 'mem_2',
			slug: 'options-alerts',
			name: 'Options Alert Service',
			description: 'Get real-time options trade alerts directly to your phone.',
			short_description: 'Real-time options trade alerts',
			thumbnail_url: '/images/memberships/options-alerts.jpg',
			tier: 'basic',
			category: 'alerts',
			features: [
				'Real-time SMS/Email Alerts',
				'Entry, Target, and Stop Prices',
				'Weekly Trade Recap',
				'Mobile App Access'
			],
			price_monthly: 97,
			price_annual: 797,
			has_trading_room: false,
			has_alerts: true,
			has_learning_center: false,
			has_daily_videos: false,
			is_active: true,
			is_featured: false,
			sort_order: 2,
			created_at: '2020-01-01T00:00:00Z',
			updated_at: '2024-06-01T00:00:00Z'
		},
		{
			id: 'mem_3',
			slug: 'futures-trading-room',
			name: 'Futures Trading Room',
			description: 'Live futures trading room with expert analysis.',
			short_description: 'Live futures trading and analysis',
			thumbnail_url: '/images/memberships/futures-room.jpg',
			tier: 'pro',
			category: 'trading-room',
			features: [
				'Pre-market Analysis (8:30 AM ET)',
				'Live Trading Room (9:30 AM - 4:00 PM ET)',
				'Futures Trade Alerts',
				'Weekly Market Outlook',
				'ES, NQ, CL Focus'
			],
			price_monthly: 247,
			price_annual: 1997,
			has_trading_room: true,
			has_alerts: true,
			has_learning_center: true,
			has_daily_videos: true,
			instructor: {
				id: 'inst_2',
				name: 'Chris Brecher',
				avatar_url: '/images/instructors/chris-brecher.jpg'
			},
			trading_room_schedule: {
				morning: '8:30 AM - 12:00 PM ET',
				afternoon: '1:00 PM - 4:00 PM ET'
			},
			is_active: true,
			is_featured: true,
			sort_order: 3,
			created_at: '2020-06-01T00:00:00Z',
			updated_at: '2024-06-01T00:00:00Z'
		},
		{
			id: 'mem_4',
			slug: 'squeeze-pro',
			name: 'Squeeze Pro Indicator',
			description: 'The legendary TTM Squeeze indicator with enhanced features.',
			short_description: 'Professional squeeze indicator',
			thumbnail_url: '/images/memberships/squeeze-pro.jpg',
			tier: 'basic',
			category: 'tools',
			features: [
				'TTM Squeeze Pro Indicator',
				'Multi-timeframe Analysis',
				'Works on TradingView, ThinkOrSwim',
				'Setup Guide and Tutorials',
				'Lifetime Updates'
			],
			price_lifetime: 497,
			has_trading_room: false,
			has_alerts: false,
			has_learning_center: false,
			has_daily_videos: false,
			is_active: true,
			is_featured: false,
			sort_order: 4,
			created_at: '2018-01-01T00:00:00Z',
			updated_at: '2024-06-01T00:00:00Z'
		},
		{
			id: 'mem_5',
			slug: 'options-mastery',
			name: 'Options Mastery Course',
			description: 'Complete options trading education from beginner to advanced.',
			short_description: 'Comprehensive options trading course',
			thumbnail_url: '/images/memberships/options-mastery.jpg',
			tier: 'pro',
			category: 'mastery',
			features: [
				'50+ Hours of Video Content',
				'Options Basics to Advanced',
				'Greeks Deep Dive',
				'Strategy Playbook',
				'Certificate of Completion'
			],
			price_lifetime: 1497,
			has_trading_room: false,
			has_alerts: false,
			has_learning_center: true,
			has_daily_videos: false,
			instructor: {
				id: 'inst_3',
				name: 'Taylor Horton',
				avatar_url: '/images/instructors/taylor-horton.jpg'
			},
			is_active: true,
			is_featured: true,
			sort_order: 5,
			created_at: '2021-01-01T00:00:00Z',
			updated_at: '2024-06-01T00:00:00Z'
		}
	];

	sampleMemberships.forEach(m => memberships.set(m.id, m));

	// Sample user memberships (for demo user)
	const demoUserMemberships: UserMembership[] = [
		{
			id: 'umem_1',
			user_id: 'demo_user',
			membership_id: 'mem_1',
			membership: sampleMemberships[0],
			status: 'active',
			subscription_type: 'annual',
			started_at: '2024-01-15T00:00:00Z',
			expires_at: '2025-01-15T00:00:00Z',
			auto_renew: true,
			payment_method: 'Visa ****4242'
		},
		{
			id: 'umem_2',
			user_id: 'demo_user',
			membership_id: 'mem_4',
			membership: sampleMemberships[3],
			status: 'active',
			subscription_type: 'lifetime',
			started_at: '2023-06-01T00:00:00Z',
			auto_renew: false
		}
	];

	userMemberships.set('demo_user', demoUserMemberships);
}

// GET - List memberships or user's memberships
export const GET: RequestHandler = async ({ url }) => {
	initializeSampleData();

	const user_id = url.searchParams.get('user_id');
	const category = url.searchParams.get('category');
	const tier = url.searchParams.get('tier');
	const featured = url.searchParams.get('featured');
	const all = url.searchParams.get('all') === 'true';

	// If user_id provided, return user's memberships
	if (user_id) {
		const userMems = userMemberships.get(user_id) || [];
		return json({
			success: true,
			data: {
				memberships: userMems,
				total: userMems.length
			}
		});
	}

	// Otherwise, list all available memberships
	let filtered = Array.from(memberships.values());

	// Only active unless all requested
	if (!all) {
		filtered = filtered.filter(m => m.is_active);
	}

	// Filter by category
	if (category) {
		filtered = filtered.filter(m => m.category === category);
	}

	// Filter by tier
	if (tier) {
		filtered = filtered.filter(m => m.tier === tier);
	}

	// Filter by featured
	if (featured === 'true') {
		filtered = filtered.filter(m => m.is_featured);
	}

	// Sort by order
	filtered.sort((a, b) => a.sort_order - b.sort_order);

	return json({
		success: true,
		data: {
			memberships: filtered,
			categories: ['trading-room', 'alerts', 'mastery', 'tools', 'bundle'],
			tiers: ['basic', 'pro', 'elite', 'lifetime']
		}
	});
};

// POST - Add membership to user
export const POST: RequestHandler = async ({ request }) => {
	initializeSampleData();

	try {
		const body = await request.json();

		if (!body.user_id || !body.membership_id) {
			throw error(400, 'user_id and membership_id are required');
		}

		const membership = memberships.get(body.membership_id);
		if (!membership) {
			throw error(404, 'Membership not found');
		}

		const userMems = userMemberships.get(body.user_id) || [];

		// Check if user already has this membership
		const existing = userMems.find(um => um.membership_id === body.membership_id);
		if (existing && existing.status === 'active') {
			throw error(400, 'User already has an active subscription to this membership');
		}

		const newUserMembership: UserMembership = {
			id: `umem_${Date.now()}`,
			user_id: body.user_id,
			membership_id: body.membership_id,
			membership,
			status: 'active',
			subscription_type: body.subscription_type || 'monthly',
			started_at: new Date().toISOString(),
			auto_renew: body.auto_renew ?? true,
			payment_method: body.payment_method
		};

		// Set expiration based on subscription type
		if (body.subscription_type === 'monthly') {
			const expires = new Date();
			expires.setMonth(expires.getMonth() + 1);
			newUserMembership.expires_at = expires.toISOString();
		} else if (body.subscription_type === 'annual') {
			const expires = new Date();
			expires.setFullYear(expires.getFullYear() + 1);
			newUserMembership.expires_at = expires.toISOString();
		}

		userMems.push(newUserMembership);
		userMemberships.set(body.user_id, userMems);

		return json({
			success: true,
			data: newUserMembership
		}, { status: 201 });
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to add membership');
	}
};
