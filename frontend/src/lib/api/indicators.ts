/**
 * Indicators API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * API client for managing user indicators (purchased/licensed).
 *
 * @version 1.0.0 - December 2025
 */

import { api } from './config';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface Indicator {
	id: number;
	name: string;
	slug: string;
	description?: string;
	short_description?: string;
	price: number;
	sale_price?: number;
	thumbnail_url?: string;
	platform: 'TradingView' | 'ThinkorSwim' | 'MetaTrader' | 'NinjaTrader';
	platform_slug: string;
	type: 'indicator' | 'scanner' | 'strategy';
	features?: string[];
	version?: string;
	is_active: boolean;
	created_at: string;
	updated_at: string;
}

export interface PurchasedIndicator {
	id: number;
	indicator_id: number;
	user_id: number;
	name: string;
	slug: string;
	description?: string;
	platform: 'TradingView' | 'ThinkorSwim' | 'MetaTrader' | 'NinjaTrader';
	platform_slug: string;
	status: 'active' | 'expiring' | 'expired' | 'trial';
	expires_at: string;
	days_until_expiry?: number;
	download_url: string;
	download_id: string;
	version: string;
	last_updated: string;
	purchase_date: string;
	license_key?: string;
	indicator?: Indicator;
}

export interface IndicatorDownload {
	id: number;
	indicator_id: number;
	version: string;
	platform: string;
	file_url: string;
	file_name: string;
	file_size: number;
	release_notes?: string;
	created_at: string;
}

export interface ApiResponse<T> {
	success: boolean;
	data: T;
	message?: string;
}

export interface PaginatedResponse<T> {
	success: boolean;
	data: {
		data: T[];
		current_page: number;
		last_page: number;
		per_page: number;
		total: number;
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// API ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

// Updated for Rust API - December 2025
const ENDPOINTS = {
	// Public (store) routes - Rust API
	list: '/api/indicators',
	single: (slug: string) => `/api/indicators/${slug}`,

	// User routes (purchased indicators) - Rust API
	my: '/api/indicators/my',
	download: (id: number) => `/api/indicators/${id}/download`,

	// Admin routes - Rust API
	admin: {
		list: '/api/indicators',
		single: (slug: string) => `/api/indicators/${slug}`,
		create: '/api/indicators',
		update: (slug: string) => `/api/indicators/${slug}`,
		delete: (slug: string) => `/api/indicators/${slug}`
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC/STORE API
// ═══════════════════════════════════════════════════════════════════════════

export const indicatorsApi = {
	/**
	 * List all available indicators for purchase
	 */
	list: async (params?: {
		platform?: string;
		type?: string;
		search?: string;
		per_page?: number;
		page?: number;
	}): Promise<PaginatedResponse<Indicator>> => {
		return api.get(ENDPOINTS.list, params);
	},

	/**
	 * Get a single indicator by slug
	 */
	getBySlug: async (slug: string): Promise<ApiResponse<Indicator>> => {
		return api.get(ENDPOINTS.single(slug));
	},

};

// ═══════════════════════════════════════════════════════════════════════════
// USER/PURCHASED INDICATORS API - Updated for Rust API
// ═══════════════════════════════════════════════════════════════════════════

export const userIndicatorsApi = {
	/**
	 * Get user's purchased/licensed indicators
	 */
	getPurchased: async (params?: {
		platform?: string;
		status?: 'active' | 'expiring' | 'expired';
		search?: string;
	}): Promise<ApiResponse<PurchasedIndicator[]>> => {
		return api.get(ENDPOINTS.my, params);
	},

	/**
	 * Get download URL for an indicator
	 */
	getDownload: async (indicatorId: number): Promise<ApiResponse<{
		download_url: string;
		documentation_url: string;
		version: string;
	}>> => {
		return api.get(ENDPOINTS.download(indicatorId));
	},

	/**
	 * Request download (generates secure download link)
	 */
	requestDownload: async (indicatorId: number): Promise<ApiResponse<{
		download_url: string;
		expires_in: number;
	}>> => {
		return api.get(ENDPOINTS.download(indicatorId));
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN API - Updated for Rust API
// ═══════════════════════════════════════════════════════════════════════════

export const adminIndicatorsApi = {
	/**
	 * List all indicators (admin)
	 */
	list: async (params?: {
		search?: string;
		platform?: string;
		is_active?: boolean;
		per_page?: number;
		page?: number;
	}): Promise<PaginatedResponse<Indicator>> => {
		return api.get(ENDPOINTS.admin.list, params);
	},

	/**
	 * Get a single indicator (admin)
	 */
	get: async (slug: string): Promise<ApiResponse<Indicator>> => {
		return api.get(ENDPOINTS.admin.single(slug));
	},

	/**
	 * Create a new indicator (admin)
	 */
	create: async (data: Partial<Indicator>): Promise<ApiResponse<Indicator>> => {
		return api.post(ENDPOINTS.admin.create, data);
	},

	/**
	 * Update an indicator (admin)
	 */
	update: async (slug: string, data: Partial<Indicator>): Promise<ApiResponse<Indicator>> => {
		return api.put(ENDPOINTS.admin.update(slug), data);
	},

	/**
	 * Delete an indicator (admin)
	 */
	delete: async (slug: string): Promise<ApiResponse<void>> => {
		return api.delete(ENDPOINTS.admin.delete(slug));
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// COMBINED EXPORT
// ═══════════════════════════════════════════════════════════════════════════

export const indicatorApi = {
	store: indicatorsApi,
	user: userIndicatorsApi,
	admin: adminIndicatorsApi
};

export default indicatorApi;
