/**
 * Consent Record API - Save user consent preferences
 *
 * Records user consent choices for GDPR/CCPA compliance.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

interface ConsentRecord {
	id: string;
	consents: Record<string, boolean>;
	timestamp: string;
	user_agent?: string;
	ip_address?: string;
}

// In-memory storage (in production, this would be a database)
const consentRecords: Map<string, ConsentRecord> = new Map();

// POST - Record consent
export const POST: RequestHandler = async ({ request, getClientAddress }) => {
	try {
		const body = await request.json();

		if (!body.consents || typeof body.consents !== 'object') {
			throw error(400, 'Consents object is required');
		}

		const id = `consent_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

		const record: ConsentRecord = {
			id,
			consents: body.consents,
			timestamp: body.timestamp || new Date().toISOString(),
			user_agent: body.user_agent,
			ip_address: getClientAddress()
		};

		consentRecords.set(id, record);

		// In production, you would:
		// 1. Store this in a database
		// 2. Associate with user ID if logged in
		// 3. Maintain audit trail for compliance

		return json({
			success: true,
			data: {
				id,
				recorded_at: record.timestamp
			}
		}, { status: 201 });
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to record consent');
	}
};

// GET - Get consent record by ID
export const GET: RequestHandler = async ({ url }) => {
	const id = url.searchParams.get('id');

	if (!id) {
		throw error(400, 'Consent ID is required');
	}

	const record = consentRecords.get(id);

	if (!record) {
		throw error(404, 'Consent record not found');
	}

	return json({
		success: true,
		data: record
	});
};
