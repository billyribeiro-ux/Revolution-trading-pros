/**
 * Banner Template Store
 *
 * Manages active template selection, customizations, and persistence.
 *
 * @module consent/templates/store
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import { writable, derived, get } from 'svelte/store';
import type { BannerTemplate, TemplateCustomization, ActiveTemplateConfig } from './types';
import { BANNER_TEMPLATES, getTemplate, DEFAULT_TEMPLATE_ID } from './registry';

// =============================================================================
// STORAGE KEYS
// =============================================================================

const ACTIVE_TEMPLATE_KEY = 'rtp_active_template';
const CUSTOM_TEMPLATES_KEY = 'rtp_custom_templates';

// =============================================================================
// STORES
// =============================================================================

/**
 * Active template configuration
 */
const activeTemplateConfig = writable<ActiveTemplateConfig>({
	templateId: DEFAULT_TEMPLATE_ID,
	updatedAt: new Date().toISOString(),
});

/**
 * Custom templates created by user
 */
const customTemplates = writable<Map<string, BannerTemplate>>(new Map());

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
	[activeTemplateConfig, customTemplates, previewTemplate, isPreviewMode],
	([$config, $custom, $preview, $isPreview]) => {
		// If preview mode, return preview template
		if ($isPreview && $preview) {
			return $preview;
		}

		// Check custom templates first
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
 * All available templates (built-in + custom)
 */
export const allTemplates = derived(customTemplates, ($custom) => {
	const templates = [...BANNER_TEMPLATES];
	$custom.forEach((template) => {
		// Add custom templates that don't override built-in ones
		if (!BANNER_TEMPLATES.find((t) => t.id === template.id)) {
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

	return {
		...template,
		name: customization.name || template.name,
		position: customization.position || template.position,
		colors: { ...template.colors, ...customization.colors },
		typography: { ...template.typography, ...customization.typography },
		spacing: { ...template.spacing, ...customization.spacing },
		copy: { ...template.copy, ...customization.copy },
		customCSS: customization.customCSS || template.customCSS,
	};
}

/**
 * Deep merge objects
 */
function deepMerge<T extends Record<string, any>>(target: T, source: Partial<T>): T {
	const result = { ...target };
	for (const key in source) {
		if (source[key] && typeof source[key] === 'object' && !Array.isArray(source[key])) {
			result[key] = deepMerge(result[key] as any, source[key] as any);
		} else if (source[key] !== undefined) {
			result[key] = source[key] as any;
		}
	}
	return result;
}

// =============================================================================
// STORE METHODS
// =============================================================================

/**
 * Initialize template store (load from storage)
 */
export function initializeTemplateStore(): void {
	if (!browser) return;

	// Load active template config
	try {
		const stored = localStorage.getItem(ACTIVE_TEMPLATE_KEY);
		if (stored) {
			const config = JSON.parse(stored) as ActiveTemplateConfig;
			activeTemplateConfig.set(config);
		}
	} catch (e) {
		console.debug('[TemplateStore] Failed to load active template:', e);
	}

	// Load custom templates
	try {
		const stored = localStorage.getItem(CUSTOM_TEMPLATES_KEY);
		if (stored) {
			const templates = JSON.parse(stored) as BannerTemplate[];
			const map = new Map<string, BannerTemplate>();
			templates.forEach((t) => map.set(t.id, t));
			customTemplates.set(map);
		}
	} catch (e) {
		console.debug('[TemplateStore] Failed to load custom templates:', e);
	}

	console.debug('[TemplateStore] Initialized');
}

/**
 * Save store to localStorage
 */
function saveToStorage(): void {
	if (!browser) return;

	const config = get(activeTemplateConfig);
	localStorage.setItem(ACTIVE_TEMPLATE_KEY, JSON.stringify(config));

	const custom = get(customTemplates);
	localStorage.setItem(CUSTOM_TEMPLATES_KEY, JSON.stringify([...custom.values()]));
}

/**
 * Set active template
 */
export function setActiveTemplate(templateId: string): void {
	const template = getTemplate(templateId) || get(customTemplates).get(templateId);
	if (!template) {
		console.warn('[TemplateStore] Template not found:', templateId);
		return;
	}

	activeTemplateConfig.update((config) => ({
		...config,
		templateId,
		customization: undefined,
		updatedAt: new Date().toISOString(),
	}));

	saveToStorage();
	console.debug('[TemplateStore] Active template set:', templateId);
}

/**
 * Update template customization
 */
export function updateCustomization(customization: Partial<TemplateCustomization>): void {
	activeTemplateConfig.update((config) => {
		const existing = config.customization || {
			templateId: config.templateId,
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString(),
		};

		return {
			...config,
			customization: {
				...existing,
				...customization,
				updatedAt: new Date().toISOString(),
			},
			updatedAt: new Date().toISOString(),
		};
	});

	saveToStorage();
}

/**
 * Clear customizations
 */
export function clearCustomization(): void {
	activeTemplateConfig.update((config) => ({
		...config,
		customization: undefined,
		updatedAt: new Date().toISOString(),
	}));

	saveToStorage();
}

/**
 * Save as custom template
 */
export function saveAsCustomTemplate(name: string): string {
	const current = get(activeTemplate);
	const customId = `custom-${Date.now().toString(36)}`;

	const customTemplate: BannerTemplate = {
		...current,
		id: customId,
		name,
		category: 'custom',
		isEditable: true,
	};

	customTemplates.update((map) => {
		map.set(customId, customTemplate);
		return new Map(map);
	});

	saveToStorage();
	console.debug('[TemplateStore] Custom template saved:', customId);

	return customId;
}

/**
 * Update custom template
 */
export function updateCustomTemplate(id: string, updates: Partial<BannerTemplate>): void {
	customTemplates.update((map) => {
		const existing = map.get(id);
		if (existing) {
			map.set(id, { ...existing, ...updates });
		}
		return new Map(map);
	});

	saveToStorage();
}

/**
 * Delete custom template
 */
export function deleteCustomTemplate(id: string): void {
	customTemplates.update((map) => {
		map.delete(id);
		return new Map(map);
	});

	// If active template was deleted, reset to default
	const config = get(activeTemplateConfig);
	if (config.templateId === id) {
		setActiveTemplate(DEFAULT_TEMPLATE_ID);
	}

	saveToStorage();
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
export function applyPreview(): void {
	const preview = get(previewTemplate);
	if (preview) {
		setActiveTemplate(preview.id);
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
			exportedAt: new Date().toISOString(),
		},
		null,
		2
	);
}

/**
 * Import template configuration
 */
export function importTemplateConfig(json: string): boolean {
	try {
		const data = JSON.parse(json);

		if (data.activeConfig) {
			activeTemplateConfig.set(data.activeConfig);
		}

		if (data.customTemplates && Array.isArray(data.customTemplates)) {
			const map = new Map<string, BannerTemplate>();
			data.customTemplates.forEach((t: BannerTemplate) => map.set(t.id, t));
			customTemplates.set(map);
		}

		saveToStorage();
		console.debug('[TemplateStore] Configuration imported');
		return true;
	} catch (e) {
		console.error('[TemplateStore] Failed to import configuration:', e);
		return false;
	}
}
