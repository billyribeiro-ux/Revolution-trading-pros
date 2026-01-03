import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, fetch }) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		const response = await fetch('/api/woocommerce/subscriptions', {
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch subscriptions');
		}

		const subscriptions = await response.json();

		return {
			subscriptions: subscriptions || []
		};
	} catch (err) {
		console.error('Error loading subscriptions:', err);
		return {
			subscriptions: []
		};
	}
};
