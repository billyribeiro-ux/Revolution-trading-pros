/**
 * User Memberships API Service - Svelte 5 / SvelteKit Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-grade API service with:
 * - Multi-tier caching (memory + localStorage)
 * - Request deduplication
 * - Stale-while-revalidate
 * - TypeScript strict typing
 *
 * @version 3.0.0 (SvelteKit / December 2025)
 */

import { browser } from '$app/environment';
import { authStore } from '$lib/stores/auth';
import { apiCache, buildCacheKey, invalidateCache } from './cache';

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

// Production fallback - NEVER use localhost in production
const PROD_API = 'https://revolution-trading-pros-api.fly.dev/api';
const API_BASE = browser ? import.meta.env.VITE_API_URL || PROD_API : '';

// Cache TTLs (in milliseconds)
const CACHE_TTL = {
	memberships: 3 * 60 * 1000,       // 3 minutes
	membershipDetails: 5 * 60 * 1000, // 5 minutes
	tradingRooms: 2 * 60 * 1000       // 2 minutes (frequently accessed)
} as const;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type MembershipType = 'trading-room' | 'alert-service' | 'course' | 'indicator' | 'weekly-watchlist';
export type MembershipStatus = 'active' | 'pending' | 'cancelled' | 'expired' | 'expiring';
export type BillingInterval = 'monthly' | 'quarterly' | 'yearly' | 'lifetime';
export type MembershipSubscriptionType = 'trial' | 'active' | 'paused' | 'complimentary' | null;

export interface UserMembership {
	id: string;
	name: string;
	type: MembershipType;
	slug: string;
	status: MembershipStatus;
	membershipType?: MembershipSubscriptionType;
	icon?: string;
	startDate: string;
	nextBillingDate?: string;
	expiresAt?: string;
	price?: number;
	interval?: BillingInterval;
	daysUntilExpiry?: number;
	accessUrl?: string;
	features?: string[];
}

export interface UserMembershipsResponse {
	memberships: UserMembership[];
	tradingRooms: UserMembership[];
	alertServices: UserMembership[];
	courses: UserMembership[];
	indicators: UserMembership[];
	weeklyWatchlist: UserMembership[];
	stats?: {
		totalActive: number;
		totalValue: number;
		expiringCount: number;
	};
}

export interface MembershipDetails extends UserMembership {
	description?: string;
	billingHistory?: BillingRecord[];
	relatedMemberships?: UserMembership[];
	accessLevel?: 'basic' | 'premium' | 'vip';
}

export interface BillingRecord {
	id: string;
	date: string;
	amount: number;
	status: 'paid' | 'pending' | 'failed' | 'refunded';
	invoiceUrl?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// HTTP UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

async function getAuthHeaders(): Promise<Record<string, string>> {
	const token = authStore.getToken();
	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	};

	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	return headers;
}

class ApiError extends Error {
	constructor(
		message: string,
		public status: number
	) {
		super(message);
		this.name = 'ApiError';
	}
}

async function handleResponse<T>(response: Response): Promise<T> {
	if (!response.ok) {
		const error = await response.json().catch(() => ({ message: 'Request failed' }));
		throw new ApiError(error.message || 'Request failed', response.status);
	}

	const data = await response.json();
	return data.data ?? data;
}

// ═══════════════════════════════════════════════════════════════════════════
// CATEGORY HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Categorize memberships by type with computed stats
 */
function categorizeMemberships(memberships: UserMembership[]): UserMembershipsResponse {
	const tradingRooms = memberships.filter((m) => m.type === 'trading-room');
	const alertServices = memberships.filter((m) => m.type === 'alert-service');
	const courses = memberships.filter((m) => m.type === 'course');
	const indicators = memberships.filter((m) => m.type === 'indicator');
	const weeklyWatchlist = memberships.filter((m) => m.type === 'weekly-watchlist');

	const activeMembers = memberships.filter((m) => m.status === 'active' || m.status === 'expiring');
	const expiringMembers = memberships.filter((m) => m.status === 'expiring');
	const totalValue = activeMembers.reduce((sum, m) => sum + (m.price || 0), 0);

	return {
		memberships,
		tradingRooms,
		alertServices,
		courses,
		indicators,
		weeklyWatchlist,
		stats: {
			totalActive: activeMembers.length,
			totalValue,
			expiringCount: expiringMembers.length
		}
	};
}

/**
 * Enhance memberships with computed fields
 */
function enhanceMemberships(memberships: UserMembership[]): UserMembership[] {
	const now = new Date();

	return memberships.map((membership) => {
		const enhanced = { ...membership };

		// Calculate days until expiry
		if (membership.nextBillingDate || membership.expiresAt) {
			const expiryDate = new Date(membership.nextBillingDate || membership.expiresAt!);
			const daysUntil = Math.ceil((expiryDate.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));
			enhanced.daysUntilExpiry = daysUntil;

			// Auto-set status to expiring if within 7 days
			if (daysUntil <= 7 && daysUntil > 0 && membership.status === 'active') {
				enhanced.status = 'expiring';
			}
		}

		// Generate access URL based on type and slug
		enhanced.accessUrl = getAccessUrl(membership.type, membership.slug);

		return enhanced;
	});
}

/**
 * Get access URL for a membership
 */
function getAccessUrl(type: MembershipType, slug: string): string {
	const baseUrls: Record<MembershipType, string> = {
		'trading-room': '/dashboard/trading-rooms',
		'alert-service': '/dashboard/alerts',
		course: '/dashboard/courses',
		indicator: '/dashboard/indicators',
		'weekly-watchlist': '/dashboard/weekly-watchlist'
	};

	return `${baseUrls[type]}/${slug}`;
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get current user's active memberships (cached)
 */
export async function getUserMemberships(options?: {
	skipCache?: boolean;
	includeExpired?: boolean;
}): Promise<UserMembershipsResponse> {
	if (!browser) {
		return categorizeMemberships([]);
	}

	const token = authStore.getToken();
	if (!token) {
		throw new Error('Not authenticated');
	}

	const params = new URLSearchParams();
	if (options?.includeExpired) params.append('include_expired', 'true');

	const url = `${API_BASE}/user/memberships${params.toString() ? '?' + params : ''}`;
	const cacheKey = buildCacheKey(url);

	if (options?.skipCache) {
		apiCache.delete(cacheKey);
	}

	try {
		return await apiCache.getOrFetch<UserMembershipsResponse>(
			cacheKey,
			async () => {
				const response = await fetch(url, {
					method: 'GET',
					headers: await getAuthHeaders(),
					credentials: 'include'
				});

				const data = await handleResponse<{ memberships: UserMembership[] }>(response);
				const enhanced = enhanceMemberships(data.memberships || []);
				return categorizeMemberships(enhanced);
			},
			{ ttl: CACHE_TTL.memberships, persist: true }
		);
	} catch (error) {
		console.error('[UserMemberships] Error fetching memberships:', error);

		// Return mock data for development/demo
		if (import.meta.env.DEV) {
			const mock = getMockMemberships();
			return categorizeMemberships(enhanceMemberships(mock.memberships));
		}

		throw error;
	}
}

/**
 * Get details for a specific membership (cached)
 */
export async function getMembershipDetails(
	membershipId: string,
	options?: { skipCache?: boolean }
): Promise<MembershipDetails> {
	if (!browser) {
		throw new Error('Cannot fetch membership details on server');
	}

	const url = `${API_BASE}/user/memberships/${membershipId}`;
	const cacheKey = buildCacheKey(url);

	if (options?.skipCache) {
		apiCache.delete(cacheKey);
	}

	return apiCache.getOrFetch<MembershipDetails>(
		cacheKey,
		async () => {
			const response = await fetch(url, {
				method: 'GET',
				headers: await getAuthHeaders(),
				credentials: 'include'
			});

			return handleResponse<MembershipDetails>(response);
		},
		{ ttl: CACHE_TTL.membershipDetails, persist: true }
	);
}

/**
 * Get trading room access (cached with shorter TTL)
 */
export async function getTradingRoomAccess(
	slug: string,
	options?: { skipCache?: boolean }
): Promise<{
	hasAccess: boolean;
	membership?: UserMembership;
	accessUrl?: string;
	discordInvite?: string;
}> {
	if (!browser) {
		return { hasAccess: false };
	}

	const url = `${API_BASE}/trading-rooms/${slug}/access`;
	const cacheKey = buildCacheKey(url);

	if (options?.skipCache) {
		apiCache.delete(cacheKey);
	}

	return apiCache.getOrFetch(
		cacheKey,
		async () => {
			const response = await fetch(url, {
				method: 'GET',
				headers: await getAuthHeaders(),
				credentials: 'include'
			});

			return handleResponse(response);
		},
		{ ttl: CACHE_TTL.tradingRooms }
	);
}

/**
 * Invalidate user membership cache
 */
export function invalidateMembershipCache(): void {
	invalidateCache(/user\/memberships/);
	invalidateCache(/trading-rooms/);
}

/**
 * Preload membership data for faster navigation
 */
export async function preloadMembershipData(): Promise<void> {
	await Promise.allSettled([getUserMemberships()]);
}

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA (Development Only)
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Mock data for development/demo purposes
 */
function getMockMemberships(): UserMembershipsResponse {
	const tradingRooms: UserMembership[] = [
		{
			id: 'day-trading',
			name: 'Day Trading Room',
			type: 'trading-room',
			slug: 'day-trading',
			status: 'active',
			membershipType: 'active',
			icon: 'day-trading',
			startDate: '2024-01-15',
			nextBillingDate: '2025-01-15',
			price: 197,
			interval: 'monthly',
			features: ['Live Trading Sessions', 'Discord Access', 'Trade Alerts']
		},
		{
			id: 'swing-trading',
			name: 'Swing Trading Room',
			type: 'trading-room',
			slug: 'swing-trading',
			status: 'active',
			membershipType: 'trial',
			icon: 'swing-trading',
			startDate: '2024-03-01',
			nextBillingDate: '2025-03-01',
			price: 147,
			interval: 'monthly',
			features: ['Weekly Analysis', 'Discord Access', 'Swing Alerts']
		},
		{
			id: 'small-accounts',
			name: 'Small Accounts Room',
			type: 'trading-room',
			slug: 'small-accounts',
			status: 'active',
			membershipType: 'complimentary',
			icon: 'small-accounts',
			startDate: '2024-06-01',
			nextBillingDate: '2025-06-01',
			price: 97,
			interval: 'monthly',
			features: ['Beginner Friendly', 'Small Position Sizing', 'Discord Access']
		}
	];

	const alertServices: UserMembership[] = [
		{
			id: 'spx-profit-pulse',
			name: 'SPX Profit Pulse',
			type: 'alert-service',
			slug: 'spx-profit-pulse',
			status: 'active',
			membershipType: 'active',
			icon: 'spx-profit-pulse',
			startDate: '2024-02-01',
			nextBillingDate: '2025-02-01',
			price: 297,
			interval: 'monthly',
			features: ['Real-time SPX Alerts', 'Options Strategies', 'Entry/Exit Points']
		},
		{
			id: 'explosive-swings',
			name: 'Explosive Swings',
			type: 'alert-service',
			slug: 'explosive-swings',
			status: 'expiring',
			membershipType: 'active',
			icon: 'explosive-swings',
			startDate: '2024-04-15',
			nextBillingDate: '2024-12-10',
			daysUntilExpiry: 4,
			price: 197,
			interval: 'monthly',
			features: ['High-Impact Swing Trades', 'Technical Analysis', 'Risk Management']
		}
	];

	const allMemberships = [...tradingRooms, ...alertServices];

	return categorizeMemberships(allMemberships);
}

// ═══════════════════════════════════════════════════════════════════════════
// OWNERSHIP & SUBSCRIPTION CONFLICT CHECKING
// ═══════════════════════════════════════════════════════════════════════════

export interface OwnershipCheckResult {
	owned: boolean;
	membership?: UserMembership;
	conflictType?: 'already_subscribed' | 'upgrade' | 'downgrade' | 'already_owned' | null;
	message?: string;
}

/**
 * Interval priority for upgrade/downgrade detection
 */
const INTERVAL_PRIORITY: Record<string, number> = {
	monthly: 1,
	quarterly: 2,
	yearly: 3,
	lifetime: 4
};

/**
 * Check if user already owns a product or has an active subscription
 * Returns ownership status and conflict information
 */
export async function checkProductOwnership(
	productId: string,
	productSlug: string,
	productType: MembershipType,
	newInterval?: BillingInterval
): Promise<OwnershipCheckResult> {
	try {
		const membershipsData = await getUserMemberships();
		const allMemberships = membershipsData.memberships || [];

		// Find matching membership by slug or ID
		const existingMembership = allMemberships.find(
			(m) =>
				(m.slug === productSlug || m.id === productId) &&
				(m.status === 'active' || m.status === 'expiring')
		);

		if (!existingMembership) {
			return { owned: false };
		}

		// For one-time purchases (courses, indicators), simply return owned
		if (productType === 'course' || productType === 'indicator') {
			return {
				owned: true,
				membership: existingMembership,
				conflictType: 'already_owned',
				message: `You already own "${existingMembership.name}"`
			};
		}

		// For subscriptions, check interval changes
		const currentInterval = existingMembership.interval;

		if (!currentInterval || !newInterval) {
			// If no intervals, it's an exact match
			return {
				owned: true,
				membership: existingMembership,
				conflictType: 'already_subscribed',
				message: `You're already subscribed to "${existingMembership.name}"`
			};
		}

		if (currentInterval === newInterval) {
			return {
				owned: true,
				membership: existingMembership,
				conflictType: 'already_subscribed',
				message: `You're already subscribed to "${existingMembership.name}"`
			};
		}

		const currentPriority = INTERVAL_PRIORITY[currentInterval] || 0;
		const newPriority = INTERVAL_PRIORITY[newInterval] || 0;

		if (newPriority > currentPriority) {
			return {
				owned: true,
				membership: existingMembership,
				conflictType: 'upgrade',
				message: `You're upgrading from ${getIntervalDisplayLabel(currentInterval)} to ${getIntervalDisplayLabel(newInterval)}`
			};
		} else {
			return {
				owned: true,
				membership: existingMembership,
				conflictType: 'downgrade',
				message: `You're downgrading from ${getIntervalDisplayLabel(currentInterval)} to ${getIntervalDisplayLabel(newInterval)}`
			};
		}
	} catch (error) {
		console.error('[UserMemberships] Error checking ownership:', error);
		return { owned: false };
	}
}

/**
 * Get display label for billing interval
 */
function getIntervalDisplayLabel(interval: BillingInterval): string {
	switch (interval) {
		case 'monthly':
			return 'Monthly';
		case 'quarterly':
			return 'Quarterly';
		case 'yearly':
			return 'Annual';
		case 'lifetime':
			return 'Lifetime';
		default:
			return interval;
	}
}

/**
 * Check if user owns any variant of a product (any billing interval)
 */
export async function checkProductVariantOwnership(
	productSlug: string,
	productType: MembershipType
): Promise<UserMembership | null> {
	try {
		const membershipsData = await getUserMemberships();
		const allMemberships = membershipsData.memberships || [];

		return (
			allMemberships.find(
				(m) =>
					m.slug === productSlug &&
					m.type === productType &&
					(m.status === 'active' || m.status === 'expiring')
			) || null
		);
	} catch (error) {
		console.error('[UserMemberships] Error checking variant ownership:', error);
		return null;
	}
}

/**
 * Get all products the user currently owns (for blocking duplicate purchases)
 */
export async function getOwnedProducts(): Promise<{
	courses: string[];
	indicators: string[];
	subscriptions: Array<{ slug: string; interval: BillingInterval }>;
}> {
	try {
		const membershipsData = await getUserMemberships();
		const allMemberships = membershipsData.memberships || [];
		const activeMemberships = allMemberships.filter(
			(m) => m.status === 'active' || m.status === 'expiring'
		);

		return {
			courses: activeMemberships.filter((m) => m.type === 'course').map((m) => m.slug),
			indicators: activeMemberships.filter((m) => m.type === 'indicator').map((m) => m.slug),
			subscriptions: activeMemberships
				.filter((m) => m.type === 'trading-room' || m.type === 'alert-service')
				.map((m) => ({
					slug: m.slug,
					interval: m.interval || 'monthly'
				}))
		};
	} catch (error) {
		console.error('[UserMemberships] Error getting owned products:', error);
		return { courses: [], indicators: [], subscriptions: [] };
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT
// ═══════════════════════════════════════════════════════════════════════════

export default {
	getUserMemberships,
	getMembershipDetails,
	getTradingRoomAccess,
	invalidateMembershipCache,
	preloadMembershipData,
	checkProductOwnership,
	checkProductVariantOwnership,
	getOwnedProducts
};
