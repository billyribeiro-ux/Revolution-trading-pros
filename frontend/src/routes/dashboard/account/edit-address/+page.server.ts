import { error, fail } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';

export const load: PageServerLoad = async ({ locals, fetch }) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		const response = await fetch('/api/woocommerce/address', {
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch address');
		}

		const address = await response.json();

		return {
			address: address || {}
		};
	} catch (err) {
		console.error('Error loading address:', err);
		return {
			address: {}
		};
	}
};

export const actions: Actions = {
	default: async ({ request, locals, fetch }) => {
		const session = await locals.auth();

		if (!session?.user) {
			return fail(401, { message: 'Unauthorized' });
		}

		const formData = await request.formData();
		const addressData = {
			firstName: formData.get('firstName'),
			lastName: formData.get('lastName'),
			company: formData.get('company'),
			address1: formData.get('address1'),
			address2: formData.get('address2'),
			city: formData.get('city'),
			state: formData.get('state'),
			postcode: formData.get('postcode'),
			country: formData.get('country'),
			email: formData.get('email'),
			phone: formData.get('phone')
		};

		try {
			const response = await fetch('/api/woocommerce/address', {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(addressData)
			});

			if (!response.ok) {
				return fail(response.status, { message: 'Failed to update address' });
			}

			return { success: true, message: 'Address updated successfully' };
		} catch (err) {
			console.error('Error updating address:', err);
			return fail(500, { message: 'An error occurred while updating address' });
		}
	}
};
