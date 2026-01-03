import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, fetch }) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		// Fetch orders from API
		const response = await fetch('/api/woocommerce/orders', {
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch orders');
		}

		const orders = await response.json();

		return {
			orders: orders || []
		};
	} catch (err) {
		console.error('Error loading orders:', err);
		return {
			orders: []
		};
	}
};
