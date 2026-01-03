import { error, fail } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

/**
 * Edit Address Page Server Load
 * ICT 11 Protocol: Secure billing address management with auth verification
 */
export const load = async ({ locals, fetch }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		// Fetch user profile which includes basic info
		const response = await fetch('/api/user/profile', {
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include'
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch user profile');
		}

		const profile = await response.json();

		// Map profile data to address format
		return {
			address: {
				firstName: profile.firstName || profile.first_name || '',
				lastName: profile.lastName || profile.last_name || '',
				company: profile.company || '',
				address1: profile.address1 || profile.billing_address?.address1 || '',
				address2: profile.address2 || profile.billing_address?.address2 || '',
				city: profile.city || profile.billing_address?.city || '',
				state: profile.state || profile.billing_address?.state || '',
				postcode: profile.postcode || profile.billing_address?.postcode || '',
				country: profile.country || profile.billing_address?.country || 'US',
				email: profile.email || '',
				phone: profile.phone || profile.billing_address?.phone || ''
			}
		};
	} catch (err) {
		console.error('Error loading address:', err);
		return {
			address: {
				firstName: '',
				lastName: '',
				company: '',
				address1: '',
				address2: '',
				city: '',
				state: '',
				postcode: '',
				country: 'US',
				email: '',
				phone: ''
			}
		};
	}
};

export const actions = {
	default: async ({ request, locals, fetch }: RequestEvent) => {
		const session = await locals.auth();

		if (!session?.user) {
			return fail(401, { message: 'Unauthorized' });
		}

		const formData = await request.formData();
		const addressData = {
			firstName: formData.get('billing_first_name') as string,
			lastName: formData.get('billing_last_name') as string,
			company: formData.get('billing_company') as string,
			address1: formData.get('billing_address_1') as string,
			address2: formData.get('billing_address_2') as string,
			city: formData.get('billing_city') as string,
			state: formData.get('billing_state') as string,
			postcode: formData.get('billing_postcode') as string,
			country: formData.get('billing_country') as string,
			email: formData.get('billing_email') as string,
			phone: formData.get('billing_phone') as string
		};

		try {
			// Update user profile with billing address
			const response = await fetch('/api/user/profile', {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json'
				},
				credentials: 'include',
				body: JSON.stringify({
					firstName: addressData.firstName,
					lastName: addressData.lastName,
					email: addressData.email,
					billing_address: {
						company: addressData.company,
						address1: addressData.address1,
						address2: addressData.address2,
						city: addressData.city,
						state: addressData.state,
						postcode: addressData.postcode,
						country: addressData.country,
						phone: addressData.phone
					}
				})
			});

			if (!response.ok) {
				const errorData = await response.json();
				return fail(response.status, { 
					message: errorData.message || 'Failed to update address' 
				});
			}

			return { success: true, message: 'Address updated successfully' };
		} catch (err) {
			console.error('Error updating address:', err);
			return fail(500, { message: 'An error occurred while updating address' });
		}
	}
};
