import { error } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

/**
 * Coupons Page Server Load
 * ICT 11 Protocol: Secure coupon retrieval with user verification
 */
export const load = async ({ locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		// Fetch user's available coupons from backend
		const response = await fetch('/api/coupons/user/available', {
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include'
		});

		if (!response.ok) {
			console.error('Failed to fetch coupons:', response.status);
			return {
				coupons: []
			};
		}

		const data = await response.json();

		return {
			coupons: data.coupons || []
		};
	} catch (err) {
		console.error('Error loading coupons:', err);
		return {
			coupons: []
		};
	}
};
