/**
 * Disconnect Service Endpoint
 *
 * Removes connection and credentials for a third-party service.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// In-memory storage (shared)
const connections: Map<string, any> = new Map();

// POST - Disconnect a service
export const POST: RequestHandler = async ({ params }) => {
	const { key } = params;

	try {
		const connection = connections.get(key);

		if (!connection) {
			throw error(404, `No connection found for service '${key}'`);
		}

		// Remove connection
		connections.delete(key);

		// In production, you would also:
		// 1. Revoke OAuth tokens if applicable
		// 2. Clean up any webhooks or subscriptions
		// 3. Log the disconnection for audit purposes

		return json({
			success: true,
			data: {
				service_key: key,
				disconnected_at: new Date().toISOString()
			}
		});
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to disconnect service');
	}
};
