import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export const load = async ({ locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		const response = await fetch('/api/woocommerce/coupons', {
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch coupons');
		}

		const coupons = await response.json();

		return {
			coupons: coupons || []
		};
	} catch (err) {
		console.error('Error loading coupons:', err);
		return {
			coupons: []
		};
	}
};
