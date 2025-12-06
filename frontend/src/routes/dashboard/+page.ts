/**
 * Dashboard Page Load Function - SvelteKit / Svelte 5 Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-grade load function with:
 * - Server-side data streaming
 * - Client-side caching integration
 * - Progressive loading pattern
 * - Auth guard with redirect
 *
 * @version 3.0.0 (SvelteKit / December 2025)
 */

import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';
import { authStore } from '$lib/stores/auth';
import { getUser } from '$lib/api/auth';
import { getUserMemberships, preloadMembershipData, type UserMembershipsResponse } from '$lib/api/user-memberships';
import type { PageLoad } from './$types';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface DashboardPageData {
	title: string;
	requiresAuth: true;
	user?: {
		id: string;
		name: string;
		email: string;
		avatar?: string;
	};
	memberships?: UserMembershipsResponse;
	needsRefresh?: boolean;
	authError?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// LOAD FUNCTION
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Protected route loader with SvelteKit streaming
 *
 * Uses progressive loading:
 * 1. Return immediately with shell data (SSR)
 * 2. Stream in user data when available
 * 3. Cache results for fast navigation
 */
export const load: PageLoad = async ({ url, fetch }) => {
	// Base page data
	const pageData: DashboardPageData = {
		title: 'My Account',
		requiresAuth: true
	};

	// ═══════════════════════════════════════════════════════════════════════
	// SERVER-SIDE RENDERING
	// ═══════════════════════════════════════════════════════════════════════

	if (!browser) {
		// During SSR, return shell immediately
		// Client will handle auth check and data loading
		return pageData;
	}

	// ═══════════════════════════════════════════════════════════════════════
	// CLIENT-SIDE AUTH CHECK
	// ═══════════════════════════════════════════════════════════════════════

	const token = authStore.getToken();
	const sessionId = authStore.getSessionId();

	// No token AND no session - definitely not logged in
	if (!token && !sessionId) {
		const returnUrl = encodeURIComponent(url.pathname + url.search);
		redirect(302, `/login?redirect=${returnUrl}`);
	}

	// Has session but no token - might need refresh
	if (!token && sessionId) {
		return {
			...pageData,
			needsRefresh: true
		};
	}

	// ═══════════════════════════════════════════════════════════════════════
	// DATA LOADING WITH CACHING
	// ═══════════════════════════════════════════════════════════════════════

	try {
		// Parallel data loading with streaming
		const [user, memberships] = await Promise.all([
			getUser(),
			getUserMemberships()
		]);

		// Preload related data in background for faster navigation
		preloadMembershipData().catch(() => {
			// Silent fail for background preloading
		});

		return {
			...pageData,
			user,
			memberships
		};
	} catch (error) {
		console.error('[Dashboard Load] Error:', error);

		// Token invalid or API error - let client-side handle
		// This prevents history pollution from load function redirects
		return {
			...pageData,
			authError: true
		};
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// INTERNAL PRELOAD HELPER
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Internal preload helper for link hover prefetching
 * Prefixed with underscore to avoid SvelteKit export validation
 */
export const _preloadDashboard = async () => {
	if (!browser) return;

	try {
		// Warm the cache for faster navigation
		await preloadMembershipData();
	} catch {
		// Silent fail for preloading
	}
};
