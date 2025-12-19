/**
 * User Indicators API Service
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * API service for fetching and managing user's purchased indicators.
 *
 * @version 1.0.0 - December 2025
 */

import { browser } from '$app/environment';
import { authStore } from '$lib/stores/auth';
import { apiCache, buildCacheKey, invalidateCache } from './cache';

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

// Production fallback - NEVER use localhost in production
const PROD_API = 'https://revolution-backend.fly.dev/api';
const API_BASE = browser ? import.meta.env.VITE_API_URL || PROD_API : '';
const CACHE_TTL = 3 * 60 * 1000; // 3 minutes

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type IndicatorStatus = 'active' | 'expiring' | 'expired';
export type Platform = 'TradingView' | 'ThinkorSwim' | 'MetaTrader';

export interface PurchasedIndicator {
	id: number;
	name: string;
	description: string;
	platform: Platform;
	platformSlug: string;
	status: IndicatorStatus;
	expiresAt: string | null;
	daysUntilExpiry: number | null;
	downloadUrl: string;
	version: string;
	slug: string;
	lastUpdated: string;
	purchasedAt: string | null;
	licenseKey?: string;
	downloadCount: number;
	documentationUrl: string;
	features: string[];
	thumbnail?: string;
}

export interface IndicatorStats {
	total: number;
	active: number;
	expiring: number;
	expired: number;
}

export interface UserIndicatorsResponse {
	success: boolean;
	data: {
		indicators: PurchasedIndicator[];
		stats: IndicatorStats;
	};
}

export interface IndicatorDownloadResponse {
	success: boolean;
	data: {
		download_url: string;
		filename: string;
		version: string;
	};
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
		public status: number,
		public errorCode?: string
	) {
		super(message);
		this.name = 'ApiError';
	}
}

async function handleResponse<T>(response: Response): Promise<T> {
	if (!response.ok) {
		const error = await response.json().catch(() => ({ message: 'Request failed' }));
		throw new ApiError(error.message || 'Request failed', response.status, error.error_code);
	}

	const data = await response.json();
	return data;
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get all indicators owned by the current user
 */
export async function getUserIndicators(options?: {
	skipCache?: boolean;
}): Promise<UserIndicatorsResponse> {
	if (!browser) {
		return {
			success: true,
			data: { indicators: [], stats: { total: 0, active: 0, expiring: 0, expired: 0 } }
		};
	}

	const token = authStore.getToken();
	if (!token) {
		throw new Error('Not authenticated');
	}

	const url = `${API_BASE}/user/indicators`;
	const cacheKey = buildCacheKey(url);

	if (options?.skipCache) {
		apiCache.delete(cacheKey);
	}

	try {
		return await apiCache.getOrFetch<UserIndicatorsResponse>(
			cacheKey,
			async () => {
				const response = await fetch(url, {
					method: 'GET',
					headers: await getAuthHeaders(),
					credentials: 'include'
				});

				return handleResponse<UserIndicatorsResponse>(response);
			},
			{ ttl: CACHE_TTL, persist: true }
		);
	} catch (error) {
		console.error('[UserIndicators] Error fetching indicators:', error);

		// Return mock data for development/demo
		if (import.meta.env.DEV) {
			return getMockIndicators();
		}

		throw error;
	}
}

/**
 * Get details for a specific indicator
 */
export async function getIndicatorDetails(indicatorId: number): Promise<PurchasedIndicator> {
	if (!browser) {
		throw new Error('Cannot fetch indicator details on server');
	}

	const url = `${API_BASE}/user/indicators/${indicatorId}`;
	const response = await fetch(url, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	const data = await handleResponse<{ success: boolean; data: PurchasedIndicator }>(response);
	return data.data;
}

/**
 * Download an indicator
 */
export async function downloadIndicator(indicatorId: number): Promise<IndicatorDownloadResponse> {
	if (!browser) {
		throw new Error('Cannot download on server');
	}

	const url = `${API_BASE}/user/indicators/${indicatorId}/download`;
	const response = await fetch(url, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	const data = await handleResponse<IndicatorDownloadResponse>(response);

	// Open download URL
	if (data.success && data.data.download_url) {
		window.open(data.data.download_url, '_blank');
	}

	return data;
}

/**
 * Invalidate indicators cache
 */
export function invalidateIndicatorsCache(): void {
	invalidateCache(/user\/indicators/);
}

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA (Development Only)
// ═══════════════════════════════════════════════════════════════════════════

function getMockIndicators(): UserIndicatorsResponse {
	const indicators: PurchasedIndicator[] = [
		{
			id: 1,
			name: 'Revolution Pro Scanner',
			description: 'Advanced market scanner with real-time alerts for momentum trades.',
			platform: 'TradingView',
			platformSlug: 'tradingview',
			status: 'active',
			expiresAt: 'Dec 31, 2025',
			daysUntilExpiry: 25,
			downloadUrl: '#',
			version: '2.4.1',
			slug: 'revolution-pro-scanner',
			lastUpdated: 'Dec 1, 2025',
			purchasedAt: 'Jan 15, 2025',
			downloadCount: 3,
			documentationUrl: '/dashboard/indicators/revolution-pro-scanner/docs',
			features: ['Real-time scanning', 'Momentum alerts', 'Custom filters']
		},
		{
			id: 2,
			name: 'Momentum Tracker Elite',
			description: 'Identify momentum shifts before they happen with advanced algorithms.',
			platform: 'ThinkorSwim',
			platformSlug: 'thinkorswim',
			status: 'active',
			expiresAt: 'Jun 15, 2026',
			daysUntilExpiry: 190,
			downloadUrl: '#',
			version: '1.8.0',
			slug: 'momentum-tracker-elite',
			lastUpdated: 'Nov 15, 2025',
			purchasedAt: 'Mar 1, 2025',
			downloadCount: 5,
			documentationUrl: '/dashboard/indicators/momentum-tracker-elite/docs',
			features: ['Momentum detection', 'Multi-timeframe', 'Audio alerts']
		},
		{
			id: 3,
			name: 'Support & Resistance Zones',
			description: 'Automatic S/R level detection with multi-timeframe confluence.',
			platform: 'TradingView',
			platformSlug: 'tradingview',
			status: 'expiring',
			expiresAt: 'Dec 15, 2025',
			daysUntilExpiry: 8,
			downloadUrl: '#',
			version: '3.1.2',
			slug: 'support-resistance-zones',
			lastUpdated: 'Oct 20, 2025',
			purchasedAt: 'Dec 15, 2024',
			downloadCount: 2,
			documentationUrl: '/dashboard/indicators/support-resistance-zones/docs',
			features: ['Auto S/R detection', 'Confluence zones', 'Volume confirmation']
		}
	];

	return {
		success: true,
		data: {
			indicators,
			stats: {
				total: indicators.length,
				active: indicators.filter((i) => i.status === 'active').length,
				expiring: indicators.filter((i) => i.status === 'expiring').length,
				expired: indicators.filter((i) => i.status === 'expired').length
			}
		}
	};
}

export default {
	getUserIndicators,
	getIndicatorDetails,
	downloadIndicator,
	invalidateIndicatorsCache
};
