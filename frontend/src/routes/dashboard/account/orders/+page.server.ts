import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

/**
 * Orders Page Server Load
 * ICT 11 Protocol: Enterprise-grade order retrieval with auth verification
 */
export const load = async ({ locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		// Fetch orders from real API endpoint
		const response = await fetch('/api/my/orders', {
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include'
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch orders');
		}

		const data = await response.json();

		return {
			orders: data.orders || []
		};
	} catch (err) {
		console.error('Error loading orders:', err);
		return {
			orders: []
		};
	}
};
