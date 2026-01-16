/**
 * Indicator Management System API
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * Full-service indicator management with:
 * - Multi-platform file downloads (ThinkorSwim, TradingView, etc.)
 * - Secure hash-based download URLs
 * - Video management via Bunny Stream
 * - Ownership verification
 */

import { apiClient } from './client.svelte';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface Indicator {
	id: string;
	name: string;
	slug: string;
	tagline?: string;
	description?: string;
	price_cents: number;
	is_free?: boolean;
	sale_price_cents?: number;
	sale_ends_at?: string;
	logo_url?: string;
	card_image_url?: string;
	banner_image_url?: string;
	featured_video_url?: string;
	bunny_video_guid?: string;
	short_description?: string;
	long_description?: string;
	features?: string[];
	requirements?: string[];
	compatibility?: string[];
	supported_platforms?: string[];
	version?: string;
	release_date?: string;
	last_update?: string;
	meta_title?: string;
	meta_description?: string;
	og_image_url?: string;
	status?: string;
	is_published?: boolean;
	published_at?: string;
	view_count?: number;
	download_count?: number;
	purchase_count?: number;
	creator_id?: number;
	product_id?: number;
	created_at?: string;
	updated_at?: string;
}

export interface IndicatorListItem {
	id: string;
	name: string;
	slug: string;
	tagline?: string;
	price_cents: number;
	is_free?: boolean;
	sale_price_cents?: number;
	logo_url?: string;
	card_image_url?: string;
	status?: string;
	is_published?: boolean;
	view_count?: number;
	download_count?: number;
	created_at?: string;
}

export interface IndicatorFile {
	id: number;
	indicator_id: string;
	file_name: string;
	original_name?: string;
	file_path: string;
	file_size_bytes?: number;
	file_type?: string;
	mime_type?: string;
	platform: string;
	platform_version?: string;
	cdn_url?: string;
	version?: string;
	is_current_version?: boolean;
	changelog?: string;
	display_name?: string;
	display_order?: number;
	is_active?: boolean;
	download_count?: number;
	uploaded_at?: string;
}

export interface IndicatorVideo {
	id: number;
	indicator_id: string;
	title: string;
	description?: string;
	bunny_video_guid: string;
	bunny_library_id?: string;
	embed_url?: string;
	play_url?: string;
	thumbnail_url?: string;
	duration_seconds?: number;
	display_order?: number;
	is_featured?: boolean;
	is_preview?: boolean;
	encoding_status?: string;
	is_published?: boolean;
	view_count?: number;
}

export interface UserIndicatorOwnership {
	id: number;
	user_id: number;
	indicator_id: string;
	order_id?: number;
	price_paid_cents?: number;
	access_granted_at?: string;
	access_expires_at?: string;
	is_lifetime_access?: boolean;
	source?: string;
	is_active?: boolean;
}

export interface TradingPlatform {
	id: number;
	name: string;
	slug: string;
	display_name: string;
	icon_url?: string;
	file_extension?: string;
	install_instructions?: string;
	is_active?: boolean;
	display_order?: number;
}

export interface SecureDownloadUrl {
	download_url: string;
	token: string;
	expires_at: string;
	file_name: string;
	file_size_bytes?: number;
	platform: string;
}

export interface IndicatorWithDetails {
	indicator: Indicator;
	files: IndicatorFile[];
	videos: IndicatorVideo[];
	platforms: string[];
}

// Legacy type for backward compatibility
export interface LegacyIndicator {
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
		return apiClient.get(ENDPOINTS.list, { params });
	},

	/**
	 * Get a single indicator by slug
	 */
	getBySlug: async (slug: string): Promise<ApiResponse<Indicator>> => {
		return apiClient.get(ENDPOINTS.single(slug));
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
		return apiClient.get(ENDPOINTS.my, { params });
	},

	/**
	 * Get download URL for an indicator
	 */
	getDownload: async (indicatorId: number): Promise<ApiResponse<{
		download_url: string;
		documentation_url: string;
		version: string;
	}>> => {
		return apiClient.get(ENDPOINTS.download(indicatorId));
	},

	/**
	 * Request download (generates secure download link)
	 */
	requestDownload: async (indicatorId: number): Promise<ApiResponse<{
		download_url: string;
		expires_in: number;
	}>> => {
		return apiClient.get(ENDPOINTS.download(indicatorId));
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
		return apiClient.get(ENDPOINTS.admin.list, { params });
	},

	/**
	 * Get a single indicator (admin)
	 */
	get: async (slug: string): Promise<ApiResponse<Indicator>> => {
		return apiClient.get(ENDPOINTS.admin.single(slug));
	},

	/**
	 * Create a new indicator (admin)
	 */
	create: async (data: Partial<Indicator>): Promise<ApiResponse<Indicator>> => {
		return apiClient.post(ENDPOINTS.admin.create, data);
	},

	/**
	 * Update an indicator (admin)
	 */
	update: async (slug: string, data: Partial<Indicator>): Promise<ApiResponse<Indicator>> => {
		return apiClient.put(ENDPOINTS.admin.update(slug), data);
	},

	/**
	 * Delete an indicator (admin)
	 */
	delete: async (slug: string): Promise<ApiResponse<void>> => {
		return apiClient.delete(ENDPOINTS.admin.delete(slug));
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
