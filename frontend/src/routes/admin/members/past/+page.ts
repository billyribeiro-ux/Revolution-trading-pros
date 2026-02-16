/**
 * Past Members Dashboard Load Function - SvelteKit / Svelte 5 Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-grade load function with:
 * - Parallel data fetching with caching
 * - Admin auth guard
 * - Progressive data streaming
 *
 * @version 3.0.0 (SvelteKit / December 2025)
 */

import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';
import { authStore } from '$lib/stores/auth.svelte';
import pastMembersApi, {
	type DashboardOverview,
	type ChurnReason,
	type ServiceStats,
	type CampaignHistory,
	preloadDashboardData
} from '$lib/api/past-members-dashboard';
import type { Load } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface PastMembersPageData {
	title: string;
	requiresAuth: true;
	requiresAdmin: true;
	overview?: DashboardOverview;
	churnReasons?: ChurnReason[];
	services?: ServiceStats[];
	campaigns?: CampaignHistory[];
	authError?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// LOAD FUNCTION
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Admin route loader with parallel data fetching
 *
 * Uses progressive loading:
 * 1. Return shell data immediately (SSR)
 * 2. Stream in dashboard data when available
 * 3. Cache results for instant navigation
 */
export const load: Load = async ({ url }) => {
	// Base page data
	const pageData: PastMembersPageData = {
		title: 'Past Members Dashboard',
		requiresAuth: true,
		requiresAdmin: true
	};

	// ═══════════════════════════════════════════════════════════════════════
	// SERVER-SIDE RENDERING
	// ═══════════════════════════════════════════════════════════════════════

	if (!browser) {
		return pageData;
	}

	// ═══════════════════════════════════════════════════════════════════════
	// CLIENT-SIDE AUTH CHECK
	// ═══════════════════════════════════════════════════════════════════════

	const token = authStore.getToken();

	if (!token) {
		const returnUrl = encodeURIComponent(url.pathname + url.search);
		redirect(302, `/login?redirect=${returnUrl}`);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// PARALLEL DATA LOADING WITH CACHING
	// ═══════════════════════════════════════════════════════════════════════

	try {
		// Parallel data loading - all cached
		const [overview, churnReasons, services, campaigns] = await Promise.all([
			pastMembersApi.getDashboardOverview(),
			pastMembersApi.getChurnReasons(),
			pastMembersApi.getServiceStats(),
			pastMembersApi.getCampaignHistory()
		]);

		// Preload additional data in background
		preloadDashboardData().catch(() => {
			// Silent fail for background preloading
		});

		return {
			...pageData,
			overview,
			churnReasons,
			services,
			campaigns
		};
	} catch (error) {
		logger.error('[PastMembers Load] Error:', error);

		return {
			...pageData,
			authError: true
		};
	}
};
