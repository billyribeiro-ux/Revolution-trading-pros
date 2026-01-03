import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export const load = async ({ params, locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	const orderId = params.id;

	try {
		// Fetch order details from API
		const response = await fetch(`/api/woocommerce/orders/${orderId}`, {
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch order details');
		}

		const order = await response.json();

		return {
			order: order || null
		};
	} catch (err) {
		console.error('Error loading order:', err);
		throw error(404, 'Order not found');
	}
};
