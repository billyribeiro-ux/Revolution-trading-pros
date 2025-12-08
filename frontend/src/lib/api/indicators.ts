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
	status: 'active' | 'expiring' | 'expired';
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

const ENDPOINTS = {
	// Public (store) routes
	list: '/api/indicators',
	single: (slug: string) => `/api/indicators/${slug}`,
	categories: '/api/indicators/categories',

	// User routes (purchased indicators)
	purchased: '/api/user/indicators/purchased',
	download: (slug: string, downloadId: string) => `/api/indicators/${slug}/download/${downloadId}`,

	// Admin routes
	admin: {
		list: '/api/admin/indicators',
		single: (id: number) => `/api/admin/indicators/${id}`,
		create: '/api/admin/indicators',
		update: (id: number) => `/api/admin/indicators/${id}`,
		delete: (id: number) => `/api/admin/indicators/${id}`,
		downloads: (id: number) => `/api/admin/indicators/${id}/downloads`,
		uploadVersion: (id: number) => `/api/admin/indicators/${id}/versions`
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

	/**
	 * Get indicator categories
	 */
	getCategories: async (): Promise<ApiResponse<string[]>> => {
		return api.get(ENDPOINTS.categories);
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// USER/PURCHASED INDICATORS API
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
		return api.get(ENDPOINTS.purchased, params);
	},

	/**
	 * Get download URL for an indicator
	 */
	getDownload: async (slug: string, downloadId: string): Promise<ApiResponse<{
		download_url: string;
		expires_at: string;
	}>> => {
		return api.get(ENDPOINTS.download(slug, downloadId));
	},

	/**
	 * Request download (generates secure download link)
	 */
	requestDownload: async (indicatorId: number): Promise<ApiResponse<{
		download_url: string;
		expires_in: number;
	}>> => {
		return api.post(`/api/user/indicators/${indicatorId}/request-download`);
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN API
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
	get: async (id: number): Promise<ApiResponse<Indicator>> => {
		return api.get(ENDPOINTS.admin.single(id));
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
	update: async (id: number, data: Partial<Indicator>): Promise<ApiResponse<Indicator>> => {
		return api.put(ENDPOINTS.admin.update(id), data);
	},

	/**
	 * Delete an indicator (admin)
	 */
	delete: async (id: number): Promise<ApiResponse<void>> => {
		return api.delete(ENDPOINTS.admin.delete(id));
	},

	/**
	 * Get downloads/versions for an indicator (admin)
	 */
	getDownloads: async (id: number): Promise<ApiResponse<IndicatorDownload[]>> => {
		return api.get(ENDPOINTS.admin.downloads(id));
	},

	/**
	 * Upload a new version (admin)
	 */
	uploadVersion: async (id: number, data: {
		version: string;
		platform: string;
		file: File;
		release_notes?: string;
	}): Promise<ApiResponse<IndicatorDownload>> => {
		const formData = new FormData();
		formData.append('version', data.version);
		formData.append('platform', data.platform);
		formData.append('file', data.file);
		if (data.release_notes) formData.append('release_notes', data.release_notes);

		return api.post(ENDPOINTS.admin.uploadVersion(id), formData);
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
