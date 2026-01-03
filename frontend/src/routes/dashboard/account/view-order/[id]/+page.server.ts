import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

/**
 * View Order Page Server Load
 * ICT 11 Protocol: Secure order detail retrieval with ownership verification
 */
export const load = async ({ params, locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	const orderId = params.id;

	try {
		// Fetch order details from real API endpoint
		const response = await fetch(`/api/my/orders/${orderId}`, {
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include'
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch order details');
		}

		const data = await response.json();

		return {
			order: data.order || null
		};
	} catch (err) {
		console.error('Error loading order:', err);
		throw error(404, 'Order not found');
	}
};
