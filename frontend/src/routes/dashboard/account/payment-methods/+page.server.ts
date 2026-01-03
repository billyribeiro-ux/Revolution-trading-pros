import { error, fail } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export const load = async ({ locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		// Fetch payment methods from your Stripe/payment provider API
		const response = await fetch('/api/user/payment-methods', {
			headers: {
				'Content-Type': 'application/json'
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
	delete: async ({ request, locals, fetch }: RequestEvent) => {
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
			const response = await fetch(`/api/user/payment-methods/${paymentMethodId}`, {
				method: 'DELETE',
				headers: {
					'Content-Type': 'application/json'
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
