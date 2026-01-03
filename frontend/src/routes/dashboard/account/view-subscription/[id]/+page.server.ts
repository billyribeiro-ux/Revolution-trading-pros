import { error, fail } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export const load = async ({ params, locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	const subscriptionId = params.id;

	try {
		// Fetch subscription details from your subscription API
		const response = await fetch(`/api/my/subscriptions/${subscriptionId}`, {
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include'
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch subscription details');
		}

		const data = await response.json();

		return {
			subscription: data.subscription || null
		};
	} catch (err) {
		console.error('Error loading subscription:', err);
		throw error(404, 'Subscription not found');
	}
};

export const actions = {
	cancel: async ({ request, params, locals, fetch }: RequestEvent) => {
		const session = await locals.auth();

		if (!session?.user) {
			return fail(401, { error: 'Unauthorized' });
		}

		const formData = await request.formData();
		const reason = formData.get('reason') as string;
		const immediate = formData.get('immediate') === 'true';

		try {
			const response = await fetch(`/api/my/subscriptions/${params.id}/cancel`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				credentials: 'include',
				body: JSON.stringify({
					reason: reason || null,
					immediate: immediate
				})
			});

			if (!response.ok) {
				const errorData = await response.json();
				return fail(response.status, { 
					error: errorData.message || 'Failed to cancel subscription' 
				});
			}

			const data = await response.json();

			return {
				success: true,
				message: data.message || 'Subscription cancelled successfully'
			};
		} catch (err) {
			console.error('Error cancelling subscription:', err);
			return fail(500, { error: 'An error occurred while cancelling the subscription' });
		}
	},

	pause: async ({ request, params, locals, fetch }: RequestEvent) => {
		const session = await locals.auth();

		if (!session?.user) {
			return fail(401, { error: 'Unauthorized' });
		}

		const formData = await request.formData();
		const reason = formData.get('reason') as string;

		try {
			const response = await fetch(`/api/my/subscriptions/${params.id}/pause`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				credentials: 'include',
				body: JSON.stringify({
					reason: reason || null
				})
			});

			if (!response.ok) {
				const errorData = await response.json();
				return fail(response.status, { 
					error: errorData.message || 'Failed to pause subscription' 
				});
			}

			const data = await response.json();

			return {
				success: true,
				message: data.message || 'Subscription paused successfully'
			};
		} catch (err) {
			console.error('Error pausing subscription:', err);
			return fail(500, { error: 'An error occurred while pausing the subscription' });
		}
	}
};
