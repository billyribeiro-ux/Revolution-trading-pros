/**
 * Banner Template Store
 *
 * Manages active template selection, customizations, and persistence.
 * Now connects to backend API for persistence across devices.
 *
 * @module consent/templates/store
 * @version 2.0.0 - Backend API Integration
 */

import { browser } from '$app/environment';
import { writable, derived, get } from 'svelte/store';
import type { BannerTemplate, TemplateCustomization, ActiveTemplateConfig } from './types';
import { BANNER_TEMPLATES, getTemplate, DEFAULT_TEMPLATE_ID } from './registry';
import { logger } from '$lib/utils/logger';

// =============================================================================
// API CONFIGURATION
// =============================================================================

const API_BASE = '/api/admin/consent';

function getAuthHeaders(): HeadersInit {
	const token = browser ? localStorage.getItem('access_token') : '';
	return {
		'Content-Type': 'application/json',
		Accept: 'application/json',
		...(token ? { Authorization: `Bearer ${token}` } : {})
	};
}

// =============================================================================
// STORES
// =============================================================================

/**
 * Active template configuration
 */
const activeTemplateConfig = writable<ActiveTemplateConfig>({
	templateId: DEFAULT_TEMPLATE_ID,
	updatedAt: new Date().toISOString()
});

/**
 * Custom templates created by user (from backend)
 */
const customTemplates = writable<Map<string, BannerTemplate>>(new Map());

/**
 * Backend templates (from database)
 */
const backendTemplates = writable<BannerTemplate[]>([]);

/**
 * Active template ID from backend
 */
const backendActiveId = writable<number | null>(null);

/**
 * Loading state
 */
export const isLoading = writable<boolean>(false);

/**
 * Preview mode template (for admin preview)
 */
export const previewTemplate = writable<BannerTemplate | null>(null);

/**
 * Is preview mode active
 */
export const isPreviewMode = writable<boolean>(false);

// =============================================================================
// DERIVED STORES
// =============================================================================

/**
 * Current active template (fully resolved with customizations)
 */
export const activeTemplate = derived(
	[activeTemplateConfig, customTemplates, backendTemplates, previewTemplate, isPreviewMode],
	([$config, $custom, $backend, $preview, $isPreview]) => {
		// If preview mode, return preview template
		if ($isPreview && $preview) {
			return $preview;
		}

		// Check backend templates first
		const backendTemplate = $backend.find((t) => t.id === $config.templateId);
		if (backendTemplate) {
			return applyCustomization(backendTemplate, $config.customization);
		}

		// Check custom templates
		const customTemplate = $custom.get($config.templateId);
		if (customTemplate) {
			return applyCustomization(customTemplate, $config.customization);
		}

		// Get from registry
		const baseTemplate = getTemplate($config.templateId);
		if (!baseTemplate) {
			return getTemplate(DEFAULT_TEMPLATE_ID)!;
		}

		return applyCustomization(baseTemplate, $config.customization);
	}
);

/**
 * All available templates (built-in + backend + custom)
 */
export const allTemplates = derived([customTemplates, backendTemplates], ([$custom, $backend]) => {
	const templates = [...BANNER_TEMPLATES];

	// Add backend templates
	$backend.forEach((template) => {
		if (!templates.find((t) => t.id === template.id)) {
			templates.push(template);
		}
	});

	// Add custom templates
	$custom.forEach((template) => {
		if (!templates.find((t) => t.id === template.id)) {
			templates.push(template);
		}
	});

	return templates;
});

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/**
 * Apply customizations to a template
 */
function applyCustomization(
	template: BannerTemplate,
	customization?: TemplateCustomization
): BannerTemplate {
	if (!customization) return template;

	const customCSS = customization.customCSS || template.customCSS;
	return {
		...template,
		name: customization.name || template.name,
		position: customization.position || template.position,
		colors: { ...template.colors, ...customization.colors },
		typography: { ...template.typography, ...customization.typography },
		spacing: { ...template.spacing, ...customization.spacing },
		copy: { ...template.copy, ...customization.copy },
		...(customCSS && { customCSS })
	};
}

/**
 * Backend template format (different from frontend BannerTemplate)
 * This is the format returned by the Rust API
 */
interface BackendBannerTemplate {
	id: string;
	name: string;
	description: string;
	category: string;
	layout: string;
	position: string;
	isEditable: boolean;
	colors: {
		background: string;
		text: string;
		link: string;
		title: string;
		border: string;
		acceptBg: string;
		acceptText: string;
		rejectBg: string;
		rejectText: string;
		settingsBg: string;
		settingsText: string;
		toggleActive: string;
		toggleInactive: string;
	};
	typography: {
		fontFamily: string;
		titleSize: number;
		titleWeight: number;
		bodySize: number;
		bodyWeight: number;
		buttonSize: number;
		buttonWeight: number;
	};
	spacing: {
		padding: { top: number; bottom: number; left: number; right: number };
		buttonPadding: { x: number; y: number };
		buttonMargin: number;
		borderRadius: number;
		containerRadius: number;
		maxWidth: number;
	};
	animation: {
		type: string;
		duration: number;
	};
	copy: {
		title: string;
		acceptButton: string;
		rejectButton: string;
		settingsButton: string;
		privacyLinkText: string;
		cookieLinkText: string;
	};
	options: {
		showRejectButton: boolean;
		showSettingsButton: boolean;
		showPrivacyLink: boolean;
		showCookieLink: boolean;
		closeOnScroll: boolean;
		closeOnScrollDistance: number;
		showCloseButton: boolean;
		blockPageScroll: boolean;
		showPoweredBy: boolean;
	};
}

/**
 * Convert backend template format to frontend format
 * Note: Returns BackendBannerTemplate which has a different structure than BannerTemplate
 */
function convertBackendTemplate(backendTemplate: any): BackendBannerTemplate {
	return {
		id: backendTemplate.id.toString(),
		name: backendTemplate.name,
		description: backendTemplate.description || '',
		category: backendTemplate.category || 'custom',
		layout: backendTemplate.layout_type || 'bar',
		position: backendTemplate.position || 'bottom',
		isEditable: !backendTemplate.is_system,
		colors: {
			background: backendTemplate.background_color || '#1e293b',
			text: backendTemplate.text_color || '#e2e8f0',
			link: backendTemplate.link_color || '#0ea5e9',
			title: backendTemplate.title_color || '#f8fafc',
			border: backendTemplate.border_color || '#334155',
			acceptBg: backendTemplate.accept_btn_bg || '#0ea5e9',
			acceptText: backendTemplate.accept_btn_text || '#ffffff',
			rejectBg: backendTemplate.reject_btn_bg || 'transparent',
			rejectText: backendTemplate.reject_btn_text || '#94a3b8',
			settingsBg: backendTemplate.settings_btn_bg || 'transparent',
			settingsText: backendTemplate.settings_btn_text || '#94a3b8',
			toggleActive: backendTemplate.toggle_active_color || '#0ea5e9',
			toggleInactive: backendTemplate.toggle_inactive_color || '#475569'
		},
		typography: {
			fontFamily: backendTemplate.font_family || 'Inter, system-ui, sans-serif',
			titleSize: backendTemplate.title_font_size || 18,
			titleWeight: backendTemplate.title_font_weight || 600,
			bodySize: backendTemplate.body_font_size || 14,
			bodyWeight: backendTemplate.body_font_weight || 400,
			buttonSize: backendTemplate.btn_font_size || 14,
			buttonWeight: backendTemplate.btn_font_weight || 500
		},
		spacing: {
			padding: {
				top: backendTemplate.padding_top || 20,
				bottom: backendTemplate.padding_bottom || 20,
				left: backendTemplate.padding_left || 24,
				right: backendTemplate.padding_right || 24
			},
			buttonPadding: {
				x: backendTemplate.btn_padding_x || 16,
				y: backendTemplate.btn_padding_y || 10
			},
			buttonMargin: backendTemplate.btn_margin || 8,
			borderRadius: backendTemplate.btn_border_radius || 6,
			containerRadius: backendTemplate.container_border_radius || 12,
			maxWidth: backendTemplate.container_max_width || 1200
		},
		animation: {
			type: backendTemplate.animation_type || 'slide',
			duration: backendTemplate.animation_duration || 300
		},
		copy: {
			title: backendTemplate.title || 'We value your privacy',
			acceptButton: backendTemplate.accept_btn_label || 'Accept All',
			rejectButton: backendTemplate.reject_btn_label || 'Reject All',
			settingsButton: backendTemplate.settings_btn_label || 'Manage Preferences',
			privacyLinkText: backendTemplate.privacy_link_text || 'Privacy Policy',
			cookieLinkText: backendTemplate.cookie_link_text || 'Cookie Policy'
		},
		options: {
			showRejectButton: backendTemplate.show_reject_btn ?? true,
			showSettingsButton: backendTemplate.show_settings_btn ?? true,
			showPrivacyLink: backendTemplate.show_privacy_link ?? true,
			showCookieLink: backendTemplate.show_cookie_link ?? false,
			closeOnScroll: backendTemplate.close_on_scroll ?? false,
			closeOnScrollDistance: backendTemplate.close_on_scroll_distance || 60,
			showCloseButton: backendTemplate.show_close_btn ?? false,
			blockPageScroll: backendTemplate.block_page_scroll ?? false,
			showPoweredBy: backendTemplate.show_powered_by ?? false
		}
	};
}

/**
 * Convert frontend template to backend API format
 * Accepts any template-like object and extracts relevant properties
 */
function convertToBackendFormat(
	template: Partial<BackendBannerTemplate> | BannerTemplate | any
): any {
	return {
		name: template.name,
		description: template.description,
		category: template.category,
		layout_type: template.layout,
		position: template.position,
		background_color: template.colors?.background,
		text_color: template.colors?.text,
		link_color: template.colors?.link,
		title_color: template.colors?.title,
		border_color: template.colors?.border,
		accept_btn_bg: template.colors?.acceptBg,
		accept_btn_text: template.colors?.acceptText,
		reject_btn_bg: template.colors?.rejectBg,
		reject_btn_text: template.colors?.rejectText,
		settings_btn_bg: template.colors?.settingsBg,
		settings_btn_text: template.colors?.settingsText,
		toggle_active_color: template.colors?.toggleActive,
		toggle_inactive_color: template.colors?.toggleInactive,
		font_family: template.typography?.fontFamily,
		title_font_size: template.typography?.titleSize,
		title_font_weight: template.typography?.titleWeight,
		body_font_size: template.typography?.bodySize,
		body_font_weight: template.typography?.bodyWeight,
		btn_font_size: template.typography?.buttonSize,
		btn_font_weight: template.typography?.buttonWeight,
		padding_top: template.spacing?.padding?.top,
		padding_bottom: template.spacing?.padding?.bottom,
		padding_left: template.spacing?.padding?.left,
		padding_right: template.spacing?.padding?.right,
		btn_padding_x: template.spacing?.buttonPadding?.x,
		btn_padding_y: template.spacing?.buttonPadding?.y,
		btn_margin: template.spacing?.buttonMargin,
		btn_border_radius: template.spacing?.borderRadius,
		container_border_radius: template.spacing?.containerRadius,
		container_max_width: template.spacing?.maxWidth,
		animation_type: template.animation?.type,
		animation_duration: template.animation?.duration,
		title: template.copy?.title,
		accept_btn_label: template.copy?.acceptButton,
		reject_btn_label: template.copy?.rejectButton,
		settings_btn_label: template.copy?.settingsButton,
		privacy_link_text: template.copy?.privacyLinkText,
		cookie_link_text: template.copy?.cookieLinkText,
		show_reject_btn: template.options?.showRejectButton,
		show_settings_btn: template.options?.showSettingsButton,
		show_privacy_link: template.options?.showPrivacyLink,
		show_cookie_link: template.options?.showCookieLink,
		close_on_scroll: template.options?.closeOnScroll,
		close_on_scroll_distance: template.options?.closeOnScrollDistance,
		show_close_btn: template.options?.showCloseButton,
		block_page_scroll: template.options?.blockPageScroll,
		show_powered_by: template.options?.showPoweredBy
	};
}

// =============================================================================
// API FUNCTIONS
// =============================================================================

/**
 * Fetch templates from backend
 */
async function fetchTemplatesFromBackend(): Promise<void> {
	if (!browser) return;

	try {
		const response = await fetch(`${API_BASE}/templates`, {
			headers: getAuthHeaders()
		});

		if (response.ok) {
			const data = await response.json();
			if (data.success && data.data) {
				const templates = (data.data.templates || []).map(convertBackendTemplate);
				backendTemplates.set(templates);

				if (data.data.activeId) {
					backendActiveId.set(data.data.activeId);
					activeTemplateConfig.update((config) => ({
						...config,
						templateId: data.data.activeId.toString()
					}));
				}
			}
		}
	} catch (error) {
		logger.error('[TemplateStore] Failed to fetch templates from backend:', error);
	}
}

/**
 * Create template in backend
 */
async function createTemplateInBackend(template: BannerTemplate): Promise<string | null> {
	if (!browser) return null;

	try {
		const response = await fetch(`${API_BASE}/templates`, {
			method: 'POST',
			headers: getAuthHeaders(),
			body: JSON.stringify(convertToBackendFormat(template))
		});

		if (response.ok) {
			const data = await response.json();
			if (data.success && data.data) {
				await fetchTemplatesFromBackend();
				return data.data.id.toString();
			}
		}
	} catch (error) {
		logger.error('[TemplateStore] Failed to create template:', error);
	}
	return null;
}

/**
 * Update template in backend
 */
async function updateTemplateInBackend(
	id: string,
	template: Partial<BannerTemplate>
): Promise<boolean> {
	if (!browser) return false;

	try {
		const response = await fetch(`${API_BASE}/templates/${id}`, {
			method: 'PUT',
			headers: getAuthHeaders(),
			body: JSON.stringify(convertToBackendFormat(template as BannerTemplate))
		});

		if (response.ok) {
			await fetchTemplatesFromBackend();
			return true;
		}
	} catch (error) {
		logger.error('[TemplateStore] Failed to update template:', error);
	}
	return false;
}

/**
 * Delete template from backend
 */
async function deleteTemplateFromBackend(id: string): Promise<boolean> {
	if (!browser) return false;

	try {
		const response = await fetch(`${API_BASE}/templates/${id}`, {
			method: 'DELETE',
			headers: getAuthHeaders()
		});

		if (response.ok) {
			await fetchTemplatesFromBackend();
			return true;
		}
	} catch (error) {
		logger.error('[TemplateStore] Failed to delete template:', error);
	}
	return false;
}

/**
 * Activate template in backend
 */
async function activateTemplateInBackend(id: string): Promise<boolean> {
	if (!browser) return false;

	try {
		const response = await fetch(`${API_BASE}/templates/${id}/activate`, {
			method: 'POST',
			headers: getAuthHeaders()
		});

		if (response.ok) {
			backendActiveId.set(parseInt(id));
			return true;
		}
	} catch (error) {
		logger.error('[TemplateStore] Failed to activate template:', error);
	}
	return false;
}

// =============================================================================
// STORE METHODS
// =============================================================================

/**
 * Initialize template store (load from backend)
 * Only fetches from backend if user is authenticated
 */
export async function initializeTemplateStore(): Promise<void> {
	if (!browser) return;

	// Only fetch from backend if user has an auth token
	const token = localStorage.getItem('access_token');
	if (!token) {
		logger.debug('[TemplateStore] Skipping backend fetch - user not authenticated');
		return;
	}

	isLoading.set(true);

	try {
		await fetchTemplatesFromBackend();
	} catch (error) {
		logger.error('[TemplateStore] Failed to initialize:', error);
	} finally {
		isLoading.set(false);
	}

	logger.debug('[TemplateStore] Initialized with backend data');
}

/**
 * Set active template
 */
export async function setActiveTemplate(templateId: string): Promise<void> {
	const template =
		getTemplate(templateId) ||
		get(customTemplates).get(templateId) ||
		get(backendTemplates).find((t) => t.id === templateId);

	if (!template) {
		logger.warn('[TemplateStore] Template not found:', templateId);
		return;
	}

	// Update local state immediately
	const { customization: _removed, ...rest } = get(activeTemplateConfig);
	activeTemplateConfig.set({
		...rest,
		templateId,
		updatedAt: new Date().toISOString()
	});

	// Sync to backend if it's a backend template
	const backendTemplate = get(backendTemplates).find((t) => t.id === templateId);
	if (backendTemplate) {
		await activateTemplateInBackend(templateId);
	}

	logger.debug('[TemplateStore] Active template set:', templateId);
}

/**
 * Update template customization
 */
export function updateCustomization(customization: Partial<TemplateCustomization>): void {
	activeTemplateConfig.update((config) => {
		const existing = config.customization || {
			templateId: config.templateId,
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString()
		};

		return {
			...config,
			customization: {
				...existing,
				...customization,
				updatedAt: new Date().toISOString()
			},
			updatedAt: new Date().toISOString()
		};
	});
}

/**
 * Clear customizations
 */
export function clearCustomization(): void {
	const { customization: _removed, ...rest } = get(activeTemplateConfig);
	activeTemplateConfig.set({
		...rest,
		updatedAt: new Date().toISOString()
	});
}

/**
 * Save as custom template (to backend)
 */
export async function saveAsCustomTemplate(name: string): Promise<string> {
	const current = get(activeTemplate);

	const customTemplate: BannerTemplate = {
		...current,
		id: `custom-${Date.now().toString(36)}`,
		name,
		category: 'custom',
		isEditable: true
	};

	// Try to save to backend first
	const backendId = await createTemplateInBackend(customTemplate);

	if (backendId) {
		logger.debug('[TemplateStore] Custom template saved to backend:', backendId);
		return backendId;
	}

	// Fallback to local storage
	customTemplates.update((map) => {
		map.set(customTemplate.id, customTemplate);
		return new Map(map);
	});

	logger.debug('[TemplateStore] Custom template saved locally:', customTemplate.id);
	return customTemplate.id;
}

/**
 * Update custom template
 */
export async function updateCustomTemplate(
	id: string,
	updates: Partial<BannerTemplate>
): Promise<void> {
	// Try backend first
	const backendTemplate = get(backendTemplates).find((t) => t.id === id);
	if (backendTemplate) {
		await updateTemplateInBackend(id, updates);
		return;
	}

	// Fallback to local
	customTemplates.update((map) => {
		const existing = map.get(id);
		if (existing) {
			map.set(id, { ...existing, ...updates });
		}
		return new Map(map);
	});
}

/**
 * Delete custom template
 */
export async function deleteCustomTemplate(id: string): Promise<void> {
	// Try backend first
	const backendTemplate = get(backendTemplates).find((t) => t.id === id);
	if (backendTemplate) {
		await deleteTemplateFromBackend(id);
	}

	// Remove from local
	customTemplates.update((map) => {
		map.delete(id);
		return new Map(map);
	});

	// If active template was deleted, reset to default
	const config = get(activeTemplateConfig);
	if (config.templateId === id) {
		await setActiveTemplate(DEFAULT_TEMPLATE_ID);
	}
}

/**
 * Enter preview mode
 */
export function enterPreviewMode(template: BannerTemplate): void {
	previewTemplate.set(template);
	isPreviewMode.set(true);
}

/**
 * Exit preview mode
 */
export function exitPreviewMode(): void {
	previewTemplate.set(null);
	isPreviewMode.set(false);
}

/**
 * Apply preview as active
 */
export async function applyPreview(): Promise<void> {
	const preview = get(previewTemplate);
	if (preview) {
		await setActiveTemplate(preview.id);
	}
	exitPreviewMode();
}

/**
 * Get active template config
 */
export function getActiveTemplateConfig(): ActiveTemplateConfig {
	return get(activeTemplateConfig);
}

/**
 * Get current active template
 */
export function getCurrentTemplate(): BannerTemplate {
	return get(activeTemplate);
}

/**
 * Export template configuration
 */
export function exportTemplateConfig(): string {
	return JSON.stringify(
		{
			activeConfig: get(activeTemplateConfig),
			customTemplates: [...get(customTemplates).values()],
			backendTemplates: get(backendTemplates),
			exportedAt: new Date().toISOString()
		},
		null,
		2
	);
}

/**
 * Import template configuration
 */
export async function importTemplateConfig(json: string): Promise<boolean> {
	try {
		const data = JSON.parse(json);

		if (data.activeConfig) {
			activeTemplateConfig.set(data.activeConfig);
		}

		if (data.customTemplates && Array.isArray(data.customTemplates)) {
			// Try to import to backend
			for (const template of data.customTemplates) {
				await createTemplateInBackend(template);
			}
		}

		// Refresh from backend
		await fetchTemplatesFromBackend();

		logger.debug('[TemplateStore] Configuration imported');
		return true;
	} catch (e) {
		logger.error('[TemplateStore] Failed to import configuration:', e);
		return false;
	}
}

/**
 * Refresh templates from backend
 */
export async function refreshTemplates(): Promise<void> {
	await fetchTemplatesFromBackend();
}
