/**
 * Template Configuration API
 *
 * REST API for managing banner template configurations.
 * Supports:
 * - GET: Retrieve active template configuration
 * - POST: Save/update template configuration
 * - DELETE: Reset to default template
 *
 * @module api/consent/templates
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

/**
 * In-memory store for demo. Replace with database in production.
 */
interface StoredTemplateConfig {
	templateId: string;
	customization?: Record<string, unknown>;
	customTemplates: Record<string, unknown>[];
	updatedAt: string;
	updatedBy?: string;
}

const templateStore = new Map<string, StoredTemplateConfig>();

// Default configuration
const DEFAULT_CONFIG: StoredTemplateConfig = {
	templateId: 'minimal-dark',
	customTemplates: [],
	updatedAt: new Date().toISOString(),
};

/**
 * GET: Retrieve template configuration
 */
export const GET: RequestHandler = async ({ url }) => {
	try {
		const siteId = url.searchParams.get('siteId') || 'default';

		const config = templateStore.get(siteId) || DEFAULT_CONFIG;

		return json({
			success: true,
			config,
		});
	} catch (err) {
		console.error('[TemplatesAPI] GET error:', err);
		throw error(500, 'Internal server error');
	}
};

/**
 * POST: Save template configuration
 */
export const POST: RequestHandler = async ({ request, url }) => {
	try {
		const siteId = url.searchParams.get('siteId') || 'default';
		const body = await request.json();

		const { templateId, customization, customTemplates, updatedBy } = body;

		if (!templateId) {
			throw error(400, 'Missing required field: templateId');
		}

		const config: StoredTemplateConfig = {
			templateId,
			customization,
			customTemplates: customTemplates || [],
			updatedAt: new Date().toISOString(),
			updatedBy,
		};

		templateStore.set(siteId, config);

		console.log(`[TemplatesAPI] Saved template config for site: ${siteId}`);

		return json({
			success: true,
			message: 'Template configuration saved',
			config,
		});
	} catch (err) {
		console.error('[TemplatesAPI] POST error:', err);

		if (err && typeof err === 'object' && 'status' in err) {
			throw err;
		}

		throw error(500, 'Internal server error');
	}
};

/**
 * PUT: Update specific fields
 */
export const PUT: RequestHandler = async ({ request, url }) => {
	try {
		const siteId = url.searchParams.get('siteId') || 'default';
		const body = await request.json();

		const existing = templateStore.get(siteId) || DEFAULT_CONFIG;

		const updated: StoredTemplateConfig = {
			...existing,
			...body,
			updatedAt: new Date().toISOString(),
		};

		templateStore.set(siteId, updated);

		return json({
			success: true,
			message: 'Template configuration updated',
			config: updated,
		});
	} catch (err) {
		console.error('[TemplatesAPI] PUT error:', err);
		throw error(500, 'Internal server error');
	}
};

/**
 * DELETE: Reset to default
 */
export const DELETE: RequestHandler = async ({ url }) => {
	try {
		const siteId = url.searchParams.get('siteId') || 'default';

		templateStore.delete(siteId);

		console.log(`[TemplatesAPI] Reset template config for site: ${siteId}`);

		return json({
			success: true,
			message: 'Template configuration reset to default',
		});
	} catch (err) {
		console.error('[TemplatesAPI] DELETE error:', err);
		throw error(500, 'Internal server error');
	}
};
