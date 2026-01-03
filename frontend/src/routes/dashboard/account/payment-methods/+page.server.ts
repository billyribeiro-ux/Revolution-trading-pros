import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export const load = async ({ locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		const response = await fetch('/api/woocommerce/payment-methods', {
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch payment methods');
		}

		const paymentMethods = await response.json();

		return {
			paymentMethods: paymentMethods || []
		};
	} catch (err) {
		console.error('Error loading payment methods:', err);
		return {
			paymentMethods: []
		};
	}
};
