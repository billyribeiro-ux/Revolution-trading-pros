import { error, fail, redirect } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export const load = async ({ locals }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	return {
		user: session.user
	};
};

export const actions = {
	default: async ({ request, locals, fetch }: RequestEvent) => {
		const session = await locals.auth();

		if (!session?.user) {
			return fail(401, { error: 'Unauthorized' });
		}

		const formData = await request.formData();
		const paymentMethodId = formData.get('payment_method_id') as string;
		const setAsDefault = formData.get('set_as_default') === 'true';

		if (!paymentMethodId) {
			return fail(400, { error: 'Payment method is required' });
		}

		try {
			// Add payment method via your Stripe API
			const response = await fetch('/api/user/payment-methods', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				credentials: 'include',
				body: JSON.stringify({
					paymentMethodId,
					setAsDefault
				})
			});

			if (!response.ok) {
				const errorData = await response.json();
				return fail(response.status, { 
					error: errorData.message || 'Failed to add payment method' 
				});
			}

			// Redirect to payment methods page on success
			throw redirect(303, '/dashboard/account/payment-methods');
		} catch (err) {
			// If it's a redirect, re-throw it
			if (err instanceof Response && err.status === 303) {
				throw err;
			}
			
			console.error('Error adding payment method:', err);
			return fail(500, { error: 'An error occurred while adding the payment method' });
		}
	}
};
