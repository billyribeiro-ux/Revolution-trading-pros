/**
 * Consent Sync API Endpoint
 *
 * REST API for syncing consent data to backend.
 * Supports:
 * - POST: Save/update consent
 * - GET: Retrieve consent by userId
 * - DELETE: Remove consent (GDPR right to erasure)
 *
 * @module api/consent/sync
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

/**
 * In-memory store for demo purposes.
 * In production, replace with database (PostgreSQL, MongoDB, etc.)
 */
const consentStore = new Map<
	string,
	{
		consentId: string;
		userId?: string;
		consent: {
			necessary: boolean;
			analytics: boolean;
			marketing: boolean;
			preferences: boolean;
		};
		metadata: Record<string, unknown>;
		auditLog?: unknown[];
		createdAt: string;
		updatedAt: string;
	}
>();

/**
 * POST: Save or update consent
 */
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		const { consentId, userId, sessionId, consent, metadata, auditLog } = body;

		if (!consentId || !consent) {
			error(400, 'Missing required fields: consentId, consent');
		}

		const now = new Date().toISOString();

		// Store consent (keyed by consentId)
		const record = {
			consentId,
			userId,
			sessionId,
			consent: {
				necessary: consent.necessary ?? true,
				analytics: consent.analytics ?? false,
				marketing: consent.marketing ?? false,
				preferences: consent.preferences ?? false
			},
			metadata: metadata || {},
			auditLog: auditLog || [],
			createdAt: consentStore.get(consentId)?.createdAt || now,
			updatedAt: now
		};

		consentStore.set(consentId, record);

		// Also index by userId if provided
		if (userId) {
			consentStore.set(`user:${userId}`, record);
		}

		logger.info(`[ConsentAPI] Saved consent: ${consentId} for user: ${userId || 'anonymous'}`);

		return json({
			success: true,
			consentId,
			serverTimestamp: now,
			message: 'Consent saved successfully'
		});
	} catch (err) {
		logger.error('[ConsentAPI] POST error:', err);

		if (err && typeof err === 'object' && 'status' in err) {
			throw err;
		}

		error(500, 'Internal server error');
	}
};

/**
 * GET: Retrieve consent by userId or consentId
 */
export const GET: RequestHandler = async ({ url }) => {
	try {
		const userId = url.searchParams.get('userId');
		const consentId = url.searchParams.get('consentId');

		let record = null;

		if (userId) {
			record = consentStore.get(`user:${userId}`);
		} else if (consentId) {
			record = consentStore.get(consentId);
		} else {
			error(400, 'Missing required parameter: userId or consentId');
		}

		if (!record) {
			error(404, 'Consent not found');
		}

		return json({
			success: true,
			consent: {
				consentId: record.consentId,
				necessary: record.consent.necessary,
				analytics: record.consent.analytics,
				marketing: record.consent.marketing,
				preferences: record.consent.preferences,
				updatedAt: record.updatedAt,
				...record.metadata
			}
		});
	} catch (err) {
		logger.error('[ConsentAPI] GET error:', err);

		if (err && typeof err === 'object' && 'status' in err) {
			throw err;
		}

		error(500, 'Internal server error');
	}
};

/**
 * DELETE: Remove consent (GDPR right to erasure)
 */
export const DELETE: RequestHandler = async ({ url }) => {
	try {
		const userId = url.searchParams.get('userId');
		const consentId = url.searchParams.get('consentId');

		let deleted = false;

		if (userId) {
			const record = consentStore.get(`user:${userId}`);
			if (record) {
				consentStore.delete(`user:${userId}`);
				consentStore.delete(record.consentId);
				deleted = true;
			}
		} else if (consentId) {
			deleted = consentStore.delete(consentId);
		} else {
			error(400, 'Missing required parameter: userId or consentId');
		}

		if (!deleted) {
			error(404, 'Consent not found');
		}

		logger.info(`[ConsentAPI] Deleted consent for: ${userId || consentId}`);

		return json({
			success: true,
			message: 'Consent deleted successfully'
		});
	} catch (err) {
		logger.error('[ConsentAPI] DELETE error:', err);

		if (err && typeof err === 'object' && 'status' in err) {
			throw err;
		}

		error(500, 'Internal server error');
	}
};
