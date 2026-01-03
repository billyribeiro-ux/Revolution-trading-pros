import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

/**
 * Subscriptions Page Server Load
 * ICT 11 Protocol: Enterprise-grade subscription retrieval with auth verification
 */
export const load = async ({ locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		// Fetch subscriptions from real API endpoint
		const response = await fetch('/api/my/subscriptions', {
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include'
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch subscriptions');
		}

		const data = await response.json();

		return {
			subscriptions: data.subscriptions || []
		};
	} catch (err) {
		console.error('Error loading subscriptions:', err);
		return {
			subscriptions: []
		};
	}
};
