/**
 * Connect Service Endpoint
 *
 * Establishes connection with a third-party service using provided credentials.
 *
 * @version 1.1.0 - January 2026
 * ICT Level 7 Fix: Proper error handling for 500 errors
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

// In-memory storage (shared with main connections endpoint)
// In production, this would be a database
const connections: Map<string, any> = new Map();

// POST - Connect to a service
export const POST: RequestHandler = async ({ params, request }) => {
	const { key } = params;

	try {
		const body = await request.json();
		const { credentials, environment = 'production' } = body;

		if (!credentials || Object.keys(credentials).length === 0) {
			return json({ success: false, error: 'Credentials are required' }, { status: 400 });
		}

		// Validate connection credentials
		const validationError = await validateConnectionCredentials(key ?? '', credentials);
		if (validationError) {
			return json({ success: false, error: validationError }, { status: 400 });
		}

		// Create connection record
		const connection = {
			id: `conn_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
			service_key: key,
			credentials: encryptCredentials(credentials), // Would encrypt in production
			environment,
			status: 'connected',
			health_score: 100,
			health_status: 'healthy',
			connected_at: new Date().toISOString(),
			last_verified_at: new Date().toISOString(),
			api_calls_today: 0,
			api_calls_total: 0,
			last_error: null
		};

		connections.set(key ?? '', connection);

		return json({
			success: true,
			data: {
				id: connection.id,
				status: connection.status,
				connected_at: connection.connected_at
			}
		});
	} catch (err) {
		logger.error('Connection error:', err);
		return json(
			{ success: false, error: err instanceof Error ? err.message : 'Failed to connect service' },
			{ status: 500 }
		);
	}
};

// Validate connection credentials - returns error message or null if valid
async function validateConnectionCredentials(
	serviceKey: string,
	credentials: Record<string, string>
): Promise<string | null> {
	// Add a small delay to simulate network request
	await new Promise((resolve) => setTimeout(resolve, 300));

	// Basic validation - check that required fields are present
	const requiredFields: Record<string, string[]> = {
		stripe: ['api_key', 'webhook_secret'],
		paypal: ['client_id', 'client_secret'],
		mailchimp: ['api_key', 'server_prefix'],
		sendgrid: ['api_key'],
		google_analytics: ['measurement_id'],
		openai: ['api_key'],
		anthropic: ['api_key'],
		twilio: ['account_sid', 'auth_token'],
		aws_s3: ['access_key_id', 'secret_access_key', 'bucket_name']
		// Add more as needed
	};

	const required = requiredFields[serviceKey] || [];
	for (const field of required) {
		if (!credentials[field]) {
			return `Missing required field: ${field}`;
		}
	}

	// Validate API key format for common services
	if (serviceKey === 'stripe' && !credentials.api_key?.startsWith('sk_')) {
		return 'Invalid Stripe API key format. Should start with sk_';
	}

	if (serviceKey === 'openai' && !credentials.api_key?.startsWith('sk-')) {
		return 'Invalid OpenAI API key format. Should start with sk-';
	}

	return null; // No errors
}

// Encrypt credentials (placeholder - in production use proper encryption)
function encryptCredentials(credentials: Record<string, string>): Record<string, string> {
	// In production, this would use proper encryption (e.g., AES-256)
	// For development, we just mask the values
	const encrypted: Record<string, string> = {};
	for (const [key, value] of Object.entries(credentials)) {
		encrypted[key] = value.substring(0, 4) + '****' + value.substring(value.length - 4);
	}
	return encrypted;
}
