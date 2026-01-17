/**
 * Consent Settings API Client
 *
 * API client for managing consent settings from the Rust backend.
 * Mirrors Consent Magic Pro WordPress plugin functionality.
 *
 * @module api/consent-settings
 * @version 1.0.0
 */

import { browser } from '$app/environment';

const API_BASE = '/api';

/**
 * Get auth headers for API requests
 */
function getHeaders(): HeadersInit {
	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	};

	if (browser) {
		const token = localStorage.getItem('access_token');
		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}
	}

	return headers;
}

// =============================================================================
// TYPES
// =============================================================================

export interface ConsentSettingsData {
	// General
	consent_enabled: boolean;
	test_mode: boolean;
	expire_days: number;
	consent_version: number;
	policy_version: string;

	// Banner
	banner_enabled: boolean;
	banner_position: 'top' | 'bottom' | 'center';
	banner_layout: 'bar' | 'popup' | 'floating' | 'drawer';
	show_reject_button: boolean;
	show_settings_button: boolean;
	close_on_scroll: boolean;
	close_on_scroll_distance: number;

	// Script Blocking
	script_blocking_enabled: boolean;
	block_google_analytics: boolean;
	block_google_tag_manager: boolean;
	block_facebook_pixel: boolean;
	block_tiktok_pixel: boolean;
	block_twitter_pixel: boolean;
	block_linkedin_pixel: boolean;
	block_pinterest_tag: boolean;
	block_reddit_pixel: boolean;
	block_hotjar: boolean;
	block_youtube_embeds: boolean;
	block_vimeo_embeds: boolean;
	block_google_maps: boolean;

	// Integrations
	google_consent_mode_enabled: boolean;
	bing_consent_mode_enabled: boolean;

	// Geolocation
	geolocation_enabled: boolean;
	geo_default_strict: boolean;

	// Proof
	proof_consent_enabled: boolean;
	proof_retention_days: number;
	proof_auto_delete: boolean;

	[key: string]: any;
}

export interface BannerTemplate {
	id: number;
	name: string;
	slug: string;
	category: string;
	description?: string;
	is_active: boolean;
	is_default: boolean;
	is_system: boolean;
	layout_type: string;
	position: string;
	position_horizontal: string;

	// Colors
	background_color: string;
	text_color: string;
	link_color: string;
	title_color: string;
	border_color: string;
	border_style: string;
	border_width: number;
	accept_btn_bg: string;
	accept_btn_text: string;
	accept_btn_hover_bg: string;
	reject_btn_bg: string;
	reject_btn_text: string;
	reject_btn_hover_bg: string;
	settings_btn_bg: string;
	settings_btn_text: string;
	settings_btn_border: string;
	toggle_active_color: string;
	toggle_inactive_color: string;

	// Typography
	font_family: string;
	title_font_size: number;
	title_font_weight: number;
	body_font_size: number;
	body_font_weight: number;
	btn_font_size: number;
	btn_font_weight: number;

	// Spacing
	padding_top: number;
	padding_bottom: number;
	padding_left: number;
	padding_right: number;
	btn_padding_x: number;
	btn_padding_y: number;
	btn_margin: number;
	btn_border_radius: number;
	container_border_radius: number;
	container_max_width: number;

	// Animation
	animation_type: string;
	animation_duration: number;

	// Content
	title: string;
	message_text?: string;
	accept_btn_label: string;
	reject_btn_label: string;
	settings_btn_label: string;
	privacy_link_text: string;
	privacy_link_url?: string;
	cookie_link_text: string;
	cookie_link_url?: string;

	// Behavior
	show_reject_btn: boolean;
	show_settings_btn: boolean;
	show_privacy_link: boolean;
	show_cookie_link: boolean;
	close_on_scroll: boolean;
	close_on_scroll_distance: number;
	show_close_btn: boolean;
	block_page_scroll: boolean;
	show_powered_by: boolean;

	// Logo
	logo_url?: string;
	logo_size: number;
	logo_position: string;
}

export interface PublicConsentConfig {
	settings: Partial<ConsentSettingsData>;
	template?: Partial<BannerTemplate>;
	cssVariables?: Record<string, string>;
}

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

// =============================================================================
// PUBLIC CONFIG (No Auth Required)
// =============================================================================

/**
 * Get public consent configuration for frontend banner
 */
export async function getPublicConsentConfig(): Promise<PublicConsentConfig | null> {
	try {
		const response = await fetch(`${API_BASE}/consent/config`, {
			headers: {
				Accept: 'application/json'
			}
		});

		if (!response.ok) {
			console.warn('Failed to fetch consent config, using defaults');
			return null;
		}

		const data: ApiResponse<PublicConsentConfig> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error fetching consent config:', error);
		return null;
	}
}

// =============================================================================
// ADMIN SETTINGS (Auth Required)
// =============================================================================

/**
 * Get all consent settings (admin)
 */
export async function getConsentSettings(group?: string): Promise<ConsentSettingsData | null> {
	try {
		const url = group
			? `${API_BASE}/admin/consent/settings?group=${group}`
			: `${API_BASE}/admin/consent/settings`;

		const response = await fetch(url, {
			headers: getHeaders()
		});

		if (!response.ok) {
			throw new Error('Failed to fetch settings');
		}

		const data: ApiResponse<ConsentSettingsData> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error fetching consent settings:', error);
		return null;
	}
}

/**
 * Update a single setting
 */
export async function updateConsentSetting(
	key: string,
	value: any,
	options?: { type?: string; group?: string; description?: string; is_public?: boolean }
): Promise<boolean> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/settings/${key}`, {
			method: 'PUT',
			headers: getHeaders(),
			body: JSON.stringify({ value, ...options })
		});

		return response.ok;
	} catch (error) {
		console.error('Error updating consent setting:', error);
		return false;
	}
}

/**
 * Bulk update settings
 */
export async function bulkUpdateConsentSettings(
	settings: Partial<ConsentSettingsData>
): Promise<boolean> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/settings/bulk`, {
			method: 'POST',
			headers: getHeaders(),
			body: JSON.stringify({ settings })
		});

		return response.ok;
	} catch (error) {
		console.error('Error bulk updating consent settings:', error);
		return false;
	}
}

/**
 * Reset settings to defaults
 */
export async function resetConsentSettings(): Promise<ConsentSettingsData | null> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/settings/reset`, {
			method: 'POST',
			headers: getHeaders()
		});

		if (!response.ok) {
			throw new Error('Failed to reset settings');
		}

		const data: ApiResponse<ConsentSettingsData> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error resetting consent settings:', error);
		return null;
	}
}

// =============================================================================
// BANNER TEMPLATES (Admin)
// =============================================================================

/**
 * Get all banner templates
 */
export async function getBannerTemplates(): Promise<{
	templates: BannerTemplate[];
	activeId: number | null;
} | null> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates`, {
			headers: getHeaders()
		});

		if (!response.ok) {
			throw new Error('Failed to fetch templates');
		}

		const data: ApiResponse<{ templates: BannerTemplate[]; activeId: number | null }> =
			await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error fetching banner templates:', error);
		return null;
	}
}

/**
 * Get active banner template
 */
export async function getActiveTemplate(): Promise<BannerTemplate | null> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates/active`, {
			headers: getHeaders()
		});

		if (!response.ok) {
			return null;
		}

		const data: ApiResponse<BannerTemplate> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error fetching active template:', error);
		return null;
	}
}

/**
 * Get a specific template
 */
export async function getTemplate(id: number): Promise<BannerTemplate | null> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates/${id}`, {
			headers: getHeaders()
		});

		if (!response.ok) {
			throw new Error('Failed to fetch template');
		}

		const data: ApiResponse<BannerTemplate> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error fetching template:', error);
		return null;
	}
}

/**
 * Create a new template
 */
export async function createTemplate(
	template: Partial<BannerTemplate>
): Promise<BannerTemplate | null> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates`, {
			method: 'POST',
			headers: getHeaders(),
			body: JSON.stringify(template)
		});

		if (!response.ok) {
			throw new Error('Failed to create template');
		}

		const data: ApiResponse<BannerTemplate> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error creating template:', error);
		return null;
	}
}

/**
 * Update a template
 */
export async function updateTemplate(
	id: number,
	template: Partial<BannerTemplate>
): Promise<BannerTemplate | null> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates/${id}`, {
			method: 'PUT',
			headers: getHeaders(),
			body: JSON.stringify(template)
		});

		if (!response.ok) {
			throw new Error('Failed to update template');
		}

		const data: ApiResponse<BannerTemplate> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error updating template:', error);
		return null;
	}
}

/**
 * Delete a template
 */
export async function deleteTemplate(id: number): Promise<boolean> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates/${id}`, {
			method: 'DELETE',
			headers: getHeaders()
		});

		return response.ok;
	} catch (error) {
		console.error('Error deleting template:', error);
		return false;
	}
}

/**
 * Activate a template
 */
export async function activateTemplate(id: number): Promise<boolean> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates/${id}/activate`, {
			method: 'POST',
			headers: getHeaders()
		});

		return response.ok;
	} catch (error) {
		console.error('Error activating template:', error);
		return false;
	}
}

/**
 * Duplicate a template
 */
export async function duplicateTemplate(id: number): Promise<BannerTemplate | null> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates/${id}/duplicate`, {
			method: 'POST',
			headers: getHeaders()
		});

		if (!response.ok) {
			throw new Error('Failed to duplicate template');
		}

		const data: ApiResponse<BannerTemplate> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error duplicating template:', error);
		return null;
	}
}

/**
 * Export a template
 */
export async function exportTemplate(id: number): Promise<Partial<BannerTemplate> | null> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates/${id}/export`, {
			headers: getHeaders()
		});

		if (!response.ok) {
			throw new Error('Failed to export template');
		}

		const data: ApiResponse<Partial<BannerTemplate>> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error exporting template:', error);
		return null;
	}
}

/**
 * Import a template
 */
export async function importTemplate(
	template: Partial<BannerTemplate>
): Promise<BannerTemplate | null> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/templates/import`, {
			method: 'POST',
			headers: getHeaders(),
			body: JSON.stringify({ template })
		});

		if (!response.ok) {
			throw new Error('Failed to import template');
		}

		const data: ApiResponse<BannerTemplate> = await response.json();
		return data.success ? (data.data ?? null) : null;
	} catch (error) {
		console.error('Error importing template:', error);
		return null;
	}
}

/**
 * Initialize consent system (creates defaults)
 */
export async function initializeConsentSystem(): Promise<boolean> {
	try {
		const response = await fetch(`${API_BASE}/admin/consent/initialize`, {
			method: 'POST',
			headers: getHeaders()
		});

		return response.ok;
	} catch (error) {
		console.error('Error initializing consent system:', error);
		return false;
	}
}
