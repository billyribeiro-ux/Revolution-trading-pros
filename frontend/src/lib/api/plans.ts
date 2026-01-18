/**
 * Subscription Plans API Service
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 7+ Principal Engineer Grade - January 2026
 *
 * Fetches subscription plans from the backend API for dynamic checkout flows.
 * Supports room-based plan queries with monthly/quarterly/annual variants.
 */

import { getAuthToken } from '$lib/stores/auth.svelte';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface SubscriptionPlan {
	id: number;
	name: string;
	slug: string;
	display_name?: string;
	description?: string;
	price: number;
	billing_cycle: 'monthly' | 'quarterly' | 'annual' | 'yearly';
	interval_count?: number;
	savings_percent?: number;
	is_popular?: boolean;
	is_active: boolean;
	stripe_price_id?: string;
	stripe_product_id?: string;
	features?: string[] | Record<string, unknown>;
	trial_days?: number;
	sort_order?: number;
	room_id?: number;
	room_name?: string;
	room_slug?: string;
	metadata?: Record<string, unknown>;
}

export interface RoomPlansResponse {
	room_slug: string;
	room_name: string;
	plans: SubscriptionPlan[];
	total: number;
}

export interface PlansListResponse {
	data: SubscriptionPlan[];
	meta?: {
		current_page: number;
		per_page: number;
		total: number;
		total_pages: number;
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Cache
// ═══════════════════════════════════════════════════════════════════════════

const cache = new Map<string, { data: unknown; expiry: number }>();
const CACHE_TTL = 5 * 60 * 1000; // 5 minutes

function getFromCache<T>(key: string): T | null {
	const cached = cache.get(key);
	if (cached && Date.now() < cached.expiry) {
		return cached.data as T;
	}
	cache.delete(key);
	return null;
}

function setCache(key: string, data: unknown): void {
	cache.set(key, { data, expiry: Date.now() + CACHE_TTL });
}

// ═══════════════════════════════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch all subscription plans for a specific trading room
 * @param roomSlug - The room's URL slug (e.g., 'explosive-swings')
 * @returns Plans with pricing variants (monthly, quarterly, annual)
 */
export async function getPlansByRoom(roomSlug: string): Promise<RoomPlansResponse> {
	const cacheKey = `room-plans:${roomSlug}`;
	const cached = getFromCache<RoomPlansResponse>(cacheKey);
	if (cached) return cached;

	const response = await fetch(`/api/subscriptions/room/${roomSlug}/plans`, {
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	if (!response.ok) {
		const error = await response.json().catch(() => ({}));
		throw new Error(error.error || `Failed to fetch plans for room: ${roomSlug}`);
	}

	const data: RoomPlansResponse = await response.json();
	setCache(cacheKey, data);
	return data;
}

/**
 * Fetch a single plan by its slug
 * @param slug - The plan's URL slug (e.g., 'explosive-swing-quarterly')
 */
export async function getPlanBySlug(slug: string): Promise<SubscriptionPlan> {
	const cacheKey = `plan:${slug}`;
	const cached = getFromCache<SubscriptionPlan>(cacheKey);
	if (cached) return cached;

	const response = await fetch(`/api/subscriptions/plans/${slug}`, {
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	if (!response.ok) {
		const error = await response.json().catch(() => ({}));
		throw new Error(error.error || `Plan not found: ${slug}`);
	}

	const data: SubscriptionPlan = await response.json();
	setCache(cacheKey, data);
	return data;
}

/**
 * Fetch all active subscription plans
 */
export async function getAllPlans(): Promise<SubscriptionPlan[]> {
	const cacheKey = 'all-plans';
	const cached = getFromCache<SubscriptionPlan[]>(cacheKey);
	if (cached) return cached;

	const response = await fetch('/api/subscriptions/plans', {
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch plans');
	}

	const data: SubscriptionPlan[] = await response.json();
	setCache(cacheKey, data);
	return data;
}

/**
 * Admin: Fetch all plans (including inactive)
 */
export async function getAdminPlans(): Promise<PlansListResponse> {
	const token = getAuthToken();

	const response = await fetch('/api/admin/subscriptions/plans?per_page=100', {
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch admin plans');
	}

	return response.json();
}

/**
 * Admin: Update a plan
 */
export async function updatePlan(
	planId: number,
	updates: Partial<SubscriptionPlan>
): Promise<SubscriptionPlan> {
	const token = getAuthToken();

	const response = await fetch(`/api/admin/subscriptions/plans/${planId}`, {
		method: 'PUT',
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify(updates)
	});

	if (!response.ok) {
		const error = await response.json().catch(() => ({}));
		throw new Error(error.error || 'Failed to update plan');
	}

	const result = await response.json();

	// Invalidate cache
	cache.clear();

	return result.data;
}

// ═══════════════════════════════════════════════════════════════════════════
// Utility Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get billing cycle display label
 */
export function getBillingCycleLabel(cycle: string): string {
	switch (cycle) {
		case 'monthly':
			return 'Monthly';
		case 'quarterly':
			return 'Quarterly';
		case 'annual':
		case 'yearly':
			return 'Annual';
		default:
			return cycle;
	}
}

/**
 * Get billing interval description
 */
export function getBillingIntervalText(cycle: string): string {
	switch (cycle) {
		case 'monthly':
			return '/ month';
		case 'quarterly':
			return 'every 3 months';
		case 'annual':
		case 'yearly':
			return '/ year';
		default:
			return '';
	}
}

/**
 * Calculate monthly equivalent price for comparison
 */
export function getMonthlyEquivalent(plan: SubscriptionPlan): number {
	switch (plan.billing_cycle) {
		case 'monthly':
			return plan.price;
		case 'quarterly':
			return plan.price / 3;
		case 'annual':
		case 'yearly':
			return plan.price / 12;
		default:
			return plan.price;
	}
}

/**
 * Format price for display
 */
export function formatPlanPrice(price: number): string {
	return new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency: 'USD',
		minimumFractionDigits: 0,
		maximumFractionDigits: 2
	}).format(price);
}

/**
 * Sort plans by billing cycle (monthly first, then quarterly, then annual)
 */
export function sortPlansByBillingCycle(plans: SubscriptionPlan[]): SubscriptionPlan[] {
	const order: Record<string, number> = {
		monthly: 1,
		quarterly: 2,
		annual: 3,
		yearly: 3
	};

	return [...plans].sort((a, b) => {
		const orderA = a.sort_order ?? order[a.billing_cycle] ?? 99;
		const orderB = b.sort_order ?? order[b.billing_cycle] ?? 99;
		return orderA - orderB;
	});
}

/**
 * Find the recommended/popular plan from a list
 */
export function getRecommendedPlan(plans: SubscriptionPlan[]): SubscriptionPlan | undefined {
	return plans.find((p) => p.is_popular) || plans.find((p) => p.billing_cycle === 'quarterly');
}

/**
 * Convert plan to cart item format
 */
export function planToCartItem(plan: SubscriptionPlan) {
	return {
		id: `plan-${plan.id}`,
		planId: plan.id,
		name: plan.display_name || plan.name,
		description: plan.description || '',
		price: plan.price,
		type: 'subscription' as const,
		interval: plan.billing_cycle,
		stripePriceId: plan.stripe_price_id
	};
}

/**
 * Clear the plans cache (useful after admin updates)
 */
export function clearPlansCache(): void {
	cache.clear();
}
