import { fail } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';

/**
 * View Subscription Page Server
 * ICT 11+ Protocol: Uses parent layout auth pattern
 */
export const load: PageServerLoad = async ({ params, parent }) => {
	// Get user from parent layout (already authenticated by hooks.server.ts)
	const parentData = await parent();

	// Return mock subscription data for now - will be populated from API
	return {
		subscription: {
			id: params.id || '0',
			userId: parentData.user?.id || '',
			productId: '1',
			productName: 'Mastering the Trade Room',
			planId: 'monthly',
			status: 'active',
			interval: 'month',
			price: 247,
			currency: 'USD',
			startDate: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString(),
			nextPaymentDate: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
			lastPaymentDate: new Date().toISOString(),
			totalPaid: 247,
			failedPayments: 0,
			successfulPayments: 1,
			paymentHistory: [
				{
					id: '1',
					amount: 7,
					status: 'completed',
					paymentDate: new Date().toISOString(),
					dueDate: new Date().toISOString(),
					paymentMethod: 'Credit Card'
				}
			],
			renewalCount: 0,
			autoRenew: true,
			isTrialing: false,
			paymentMethod: {
				type: 'card',
				last4: '4242',
				brand: 'Visa'
			},
			mrr: 247,
			arr: 2964,
			ltv: 247
		},
		user: parentData.user
	};
};

export const actions: Actions = {
	cancel: async ({ request }) => {
		const formData = await request.formData();
		const reason = formData.get('reason') as string;

		// TODO: Call actual API when backend is ready
		console.log('Cancel subscription requested with reason:', reason);

		return {
			success: true,
			message: 'Subscription cancellation requested. You will receive a confirmation email.'
		};
	},

	pause: async ({ request }) => {
		const formData = await request.formData();
		const reason = formData.get('reason') as string;

		// TODO: Call actual API when backend is ready
		console.log('Pause subscription requested with reason:', reason);

		return {
			success: true,
			message: 'Subscription paused successfully.'
		};
	}
};
