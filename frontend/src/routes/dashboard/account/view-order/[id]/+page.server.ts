import type { PageServerLoad } from './$types';

/**
 * View Order Page Server Load
 * ICT 11+ Protocol: Uses parent layout auth pattern
 */
export const load: PageServerLoad = async ({ params, parent }) => {
	// Get user from parent layout (already authenticated by hooks.server.ts)
	const parentData = await parent();

	// Return mock order data for now - will be populated from API
	return {
		order: {
			id: parseInt(params.id || '0'),
			number: params.id || '0',
			date: new Date().toISOString(),
			status: 'Completed',
			items: [
				{
					id: 1,
					name: 'Mastering the Trade Room (1 Month Trial)',
					quantity: 1,
					total: '247.00'
				}
			],
			subtotal: '247.00',
			discount: '240.00',
			tax: '0.00',
			total: '7.00',
			paymentMethod: 'Credit Card (Stripe)',
			billingAddress: {
				name: parentData.user?.name || 'Member',
				address: '123 Main Street\nCity, ST 12345',
				phone: '',
				email: parentData.user?.email || ''
			},
			subscriptions: [
				{
					id: parseInt(params.id || '0') + 1,
					status: 'Active',
					nextPayment: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
					total: '247.00'
				}
			]
		},
		user: parentData.user
	};
};
