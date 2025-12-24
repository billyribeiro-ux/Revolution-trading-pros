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
import { isSuperadminEmail } from '$lib/config/roles';
import { get } from 'svelte/store';

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

// Production fallback - NEVER use localhost in production
// API routes are nested under /api in the backend
const PROD_API = 'https://revolution-trading-pros-api.fly.dev/api';
const API_BASE = browser ? (import.meta.env.VITE_API_URL || PROD_API) : '';

// Cache TTLs (in milliseconds)
const CACHE_TTL = {
	memberships: 3 * 60 * 1000,       // 3 minutes
	membershipDetails: 5 * 60 * 1000, // 5 minutes
	tradingRooms: 2 * 60 * 1000       // 2 minutes (frequently accessed)
} as const;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type MembershipType = 'trading-room' | 'alert-service' | 'course' | 'indicator' | 'weekly-watchlist' | 'premium-report';
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
	startDate?: string;
	nextBillingDate?: string;
	expiresAt?: string;
	price?: number;
	interval?: BillingInterval;
	daysUntilExpiry?: number;
	accessUrl?: string;
	roomLabel?: string;
	features?: string[];
}

export interface UserMembershipsResponse {
	memberships: UserMembership[];
	tradingRooms: UserMembership[];
	alertServices: UserMembership[];
	courses: UserMembership[];
	indicators: UserMembership[];
	weeklyWatchlist: UserMembership[];
	premiumReports: UserMembership[];
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
	const premiumReports = memberships.filter((m) => m.type === 'premium-report');

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
		premiumReports,
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
		'weekly-watchlist': '/dashboard/weekly-watchlist',
		'premium-report': '/dashboard/premium-reports'
	};

	return `${baseUrls[type]}/${slug}`;
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get current user's active memberships (cached)
 *
 * SUPERADMIN AUTO-UNLOCK: Superadmin users get automatic access to ALL memberships for testing
 * Regular users get their actual purchased memberships from the backend
 */
export async function getUserMemberships(options?: {
	skipCache?: boolean;
	includeExpired?: boolean;
}): Promise<UserMembershipsResponse> {
	if (!browser) {
		return categorizeMemberships([]);
	}

	// Check auth - if no token, return empty (user needs to login)
	const token = authStore.getToken();
	if (!token) {
		console.log('[UserMemberships] No auth token - user not authenticated');
		return categorizeMemberships([]);
	}

	// SUPERADMIN AUTO-UNLOCK: Check if user is superadmin
	const { user } = get(authStore);
	if (user && isSuperadminEmail(user.email)) {
		console.log('[UserMemberships] Superadmin detected - unlocking all memberships');
		// Skip cache for superadmin to always get latest products
		return await getSuperadminMemberships();
	}

	const params = new URLSearchParams();
	if (options?.includeExpired) params.append('include_expired', 'true');

	const url = `${API_BASE}/user/memberships${params.toString() ? '?' + params : ''}`;
	const cacheKey = buildCacheKey(url);

	if (options?.skipCache) {
		apiCache.delete(cacheKey);
	}

	try {
		const result = await apiCache.getOrFetch<UserMembershipsResponse>(
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

		return result;
	} catch (error) {
		console.error('[UserMemberships] Error fetching memberships:', error);
		// Return empty - let UI show appropriate message
		return categorizeMemberships([]);
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
// SUPERADMIN AUTO-UNLOCK
// ═══════════════════════════════════════════════════════════════════════════

/**
 * ICT 11+ ENTERPRISE PATTERN: Superadmin Auto-Unlock System
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Superadmin gets automatic access to ALL available memberships for testing.
 * This function fetches all products and transforms them into active memberships.
 * 
 * FALLBACK STRATEGY:
 * 1. Try /admin/products endpoint (primary)
 * 2. Try /products endpoint (fallback)
 * 3. Return mock data if both fail (development safety)
 * 
 * @returns UserMembershipsResponse with all available memberships
 */
async function getSuperadminMemberships(): Promise<UserMembershipsResponse> {
	console.log('[Superadmin] 🔓 Fetching all available memberships for superadmin...');
	
	try {
		// STRATEGY 1: Try membership plans endpoint (correct admin endpoint)
		let response = await fetch(`${API_BASE}/admin/membership-plans`, {
			method: 'GET',
			headers: await getAuthHeaders(),
			credentials: 'include'
		});

		// STRATEGY 2: Fallback to public products endpoint with correct parameter
		if (!response.ok) {
			console.warn('[Superadmin] Admin membership-plans endpoint failed, trying public products endpoint...');
			response = await fetch(`${API_BASE}/products?product_type=membership&per_page=100`, {
				method: 'GET',
				headers: await getAuthHeaders(),
				credentials: 'include'
			});
		}

		if (!response.ok) {
			console.error('[Superadmin] ❌ Both API endpoints failed:', response.status, response.statusText);
			// STRATEGY 3: Return mock data for development
			return getSuperadminMockMemberships();
		}

		const data = await response.json();
		const products = data.data || data || [];
		
		console.log(`[Superadmin] ✅ Fetched ${products.length} products from API`);

		if (products.length === 0) {
			console.warn('[Superadmin] ⚠️ No products returned from API, using mock data');
			return getSuperadminMockMemberships();
		}

		// Transform membership plans/products into active memberships for superadmin
		const memberships: UserMembership[] = products.map((item: any) => {
			// Handle both membership_plans and products structures
			const name = item.name || item.title || 'Unnamed Membership';
			const slug = item.slug || `membership-${item.id}`;
			const price = item.price || item.price_monthly || 0;
			
			// Determine membership type from name, slug, or metadata
			let membershipType: MembershipType = 'trading-room';
			const searchText = `${name} ${slug} ${item.description || ''}`.toLowerCase();
			
			if (searchText.includes('course') || searchText.includes('class') || searchText.includes('mastery')) {
				membershipType = 'course';
			} else if (searchText.includes('indicator')) {
				membershipType = 'indicator';
			} else if (searchText.includes('alert')) {
				membershipType = 'alert-service';
			} else if (searchText.includes('watchlist') || searchText.includes('weekly')) {
				membershipType = 'weekly-watchlist';
			} else if (searchText.includes('report') || searchText.includes('premium')) {
				membershipType = 'premium-report';
			}
			
			// Extract icon from metadata if available
			let icon = getDefaultIcon(membershipType);
			if (item.metadata && typeof item.metadata === 'object') {
				icon = item.metadata.icon || icon;
			} else if (item.features && typeof item.features === 'object') {
				icon = item.features.icon || icon;
			}

			return {
				id: String(item.id),
				name,
				type: membershipType,
				slug,
				status: 'active' as MembershipStatus,
				membershipType: 'complimentary' as MembershipSubscriptionType,
				icon,
				startDate: new Date().toISOString(),
				nextBillingDate: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString(),
				price,
				interval: 'monthly' as BillingInterval,
				roomLabel: name,
				features: Array.isArray(item.features) ? item.features : []
			};
		});

		console.log('[Superadmin] 📊 Membership breakdown:', {
			total: memberships.length,
			tradingRooms: memberships.filter(m => m.type === 'trading-room').length,
			courses: memberships.filter(m => m.type === 'course').length,
			indicators: memberships.filter(m => m.type === 'indicator').length,
			weeklyWatchlist: memberships.filter(m => m.type === 'weekly-watchlist').length,
			premiumReports: memberships.filter(m => m.type === 'premium-report').length
		});

		const enhanced = enhanceMemberships(memberships);
		const categorized = categorizeMemberships(enhanced);
		
		console.log('[Superadmin] ✅ Successfully loaded all memberships');
		return categorized;
	} catch (error) {
		console.error('[Superadmin] ❌ Critical error fetching memberships:', error);
		// STRATEGY 3: Return mock data on error
		return getSuperadminMockMemberships();
	}
}

/**
 * ICT 11+ PATTERN: Get default icon for membership type
 */
function getDefaultIcon(type: MembershipType): string {
	const iconMap: Record<MembershipType, string> = {
		'trading-room': 'chart-line',
		'alert-service': 'bell',
		'course': 'book',
		'indicator': 'chart-candle',
		'weekly-watchlist': 'calendar',
		'premium-report': 'file-text'
	};
	return iconMap[type] || 'chart-line';
}

/**
 * ICT 11+ PATTERN: Mock memberships for development/fallback
 * Ensures superadmin always has access even if API is down
 */
function getSuperadminMockMemberships(): UserMembershipsResponse {
	console.log('[Superadmin] 🔧 Using mock membership data (API unavailable)');
	
	const mockMemberships: UserMembership[] = [
		// Trading Rooms
		{
			id: 'mock-1',
			name: 'Options Day Trading Room',
			type: 'trading-room',
			slug: 'options-day-trading-room',
			status: 'active',
			membershipType: 'complimentary',
			icon: 'chart-line',
			startDate: new Date().toISOString(),
			nextBillingDate: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString(),
			price: 0,
			interval: 'monthly'
		},
		{
			id: 'mock-2',
			name: 'Simpler Showcase',
			type: 'trading-room',
			slug: 'simpler-showcase',
			status: 'active',
			membershipType: 'complimentary',
			icon: 'trophy',
			startDate: new Date().toISOString(),
			nextBillingDate: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString(),
			price: 0,
			interval: 'monthly'
		},
		// Weekly Watchlist
		{
			id: 'mock-3',
			name: 'Weekly Watchlist',
			type: 'weekly-watchlist',
			slug: 'weekly-watchlist',
			status: 'active',
			membershipType: 'complimentary',
			icon: 'calendar',
			startDate: new Date().toISOString(),
			nextBillingDate: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString(),
			price: 0,
			interval: 'monthly'
		}
	];

	const enhanced = enhanceMemberships(mockMemberships);
	return categorizeMemberships(enhanced);
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
// SUBSCRIPTION MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

export interface CancelSubscriptionRequest {
	cancel_immediately?: boolean;
	reason?: string;
}

export interface CancelSubscriptionResponse {
	success: boolean;
	message: string;
	status: string;
	cancel_at?: string;
}

/**
 * Cancel a subscription
 * Members can only cancel their own subscriptions
 */
export async function cancelSubscription(
	membershipId: string,
	options?: CancelSubscriptionRequest
): Promise<CancelSubscriptionResponse> {
	if (!browser) {
		throw new Error('Cannot cancel subscription on server');
	}

	const url = `${API_BASE}/user/memberships/${membershipId}/cancel`;

	const response = await fetch(url, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify({
			cancel_immediately: options?.cancel_immediately ?? false,
			reason: options?.reason
		})
	});

	const data = await handleResponse<CancelSubscriptionResponse>(response);

	// Invalidate cache after cancellation
	invalidateMembershipCache();

	return data;
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
	getOwnedProducts,
	cancelSubscription
};
