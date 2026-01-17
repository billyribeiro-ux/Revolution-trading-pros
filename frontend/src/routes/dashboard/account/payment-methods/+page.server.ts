import { error, fail } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

/**
 * Payment Methods Page Server Load
 * ICT 11 Protocol: Secure Stripe payment method management
 */
export const load = async ({ locals, fetch, cookies }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		// Get auth token from cookies
		const token = cookies.get('rtp_access_token');

		// Fetch payment methods from your Stripe/payment provider API
		const response = await fetch('/api/user/payment-methods', {
			headers: {
				'Content-Type': 'application/json',
				...(token && { Authorization: `Bearer ${token}` })
			},
			credentials: 'include'
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch payment methods');
		}

		const data = await response.json();

		return {
			paymentMethods: data.paymentMethods || []
		};
	} catch (err) {
		console.error('Error loading payment methods:', err);
		return {
			paymentMethods: []
		};
	}
};

export const actions = {
	delete: async ({ request, locals, fetch, cookies }: RequestEvent) => {
		const session = await locals.auth();

		if (!session?.user) {
			return fail(401, { error: 'Unauthorized' });
		}

		const formData = await request.formData();
		const paymentMethodId = formData.get('payment_method_id') as string;

		if (!paymentMethodId) {
			return fail(400, { error: 'Payment method ID is required' });
		}

		try {
			// Get auth token from cookies
			const token = cookies.get('rtp_access_token');

			const response = await fetch(`/api/user/payment-methods/${paymentMethodId}`, {
				method: 'DELETE',
				headers: {
					'Content-Type': 'application/json',
					...(token && { Authorization: `Bearer ${token}` })
				},
				credentials: 'include'
			});

			if (!response.ok) {
				const errorData = await response.json();
				return fail(response.status, {
					error: errorData.message || 'Failed to delete payment method'
				});
			}

			return {
				success: true,
				message: 'Payment method deleted successfully'
			};
		} catch (err) {
			console.error('Error deleting payment method:', err);
			return fail(500, { error: 'An error occurred while deleting the payment method' });
		}
	}
};
