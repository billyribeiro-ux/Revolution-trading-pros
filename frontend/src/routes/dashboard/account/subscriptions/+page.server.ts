import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

/**
 * Subscriptions Page Server Load
 * ICT 11 Protocol: Enterprise-grade subscription retrieval with auth verification
 */
export const load = async ({ locals, fetch, cookies }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		// Get auth token from cookies
		const token = cookies.get('rtp_access_token');
		
		// Fetch subscriptions from real API endpoint
		const response = await fetch('/api/my/subscriptions', {
			headers: {
				'Content-Type': 'application/json',
				...(token && { 'Authorization': `Bearer ${token}` })
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
