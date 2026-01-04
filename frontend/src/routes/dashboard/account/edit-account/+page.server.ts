import { error, fail } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

/**
 * Edit Account Page Server Load
 * ICT 11 Protocol: Secure profile management with auth verification
 */
export const load = async ({ locals, fetch, cookies }: RequestEvent) => {
	const session = await locals.auth();

	if (!session?.user) {
		throw error(401, 'Unauthorized');
	}

	try {
		// Get auth token from cookies
		const token = cookies.get('rtp_access_token');
		
		// Fetch user account details from your auth system
		const response = await fetch('/api/user/profile', {
			headers: {
				'Content-Type': 'application/json',
				...(token && { 'Authorization': `Bearer ${token}` })
			},
			credentials: 'include'
		});

		if (!response.ok) {
			throw error(response.status, 'Failed to fetch account details');
		}

		const profile = await response.json();

		return {
			profile: profile || {
				firstName: '',
				lastName: '',
				displayName: '',
				email: ''
			}
		};
	} catch (err) {
		console.error('Error loading account details:', err);
		return {
			profile: {
				firstName: '',
				lastName: '',
				displayName: '',
				email: ''
			}
		};
	}
};

export const actions = {
	default: async ({ request, locals, fetch, cookies }: RequestEvent) => {
		const session = await locals.auth();

		if (!session?.user) {
			return fail(401, { error: 'Unauthorized' });
		}

		const formData = await request.formData();
		const firstName = formData.get('account_first_name') as string;
		const lastName = formData.get('account_last_name') as string;
		const displayName = formData.get('account_display_name') as string;
		const email = formData.get('account_email') as string;
		const currentPassword = formData.get('password_current') as string;
		const newPassword = formData.get('password_1') as string;
		const confirmPassword = formData.get('password_2') as string;

		// Validate required fields
		if (!firstName || !lastName || !displayName || !email) {
			return fail(400, { error: 'Please fill in all required fields' });
		}

		// Validate password change if provided
		if (currentPassword || newPassword || confirmPassword) {
			if (!currentPassword) {
				return fail(400, { error: 'Current password is required to change password' });
			}
			if (newPassword !== confirmPassword) {
				return fail(400, { error: 'New passwords do not match' });
			}
			if (newPassword && newPassword.length < 8) {
				return fail(400, { error: 'New password must be at least 8 characters' });
			}
		}

		try {
			// Get auth token from cookies
			const token = cookies.get('rtp_access_token');
			
			// Update account details using your auth system
			const response = await fetch('/api/user/profile', {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
					...(token && { 'Authorization': `Bearer ${token}` })
				},
				credentials: 'include',
				body: JSON.stringify({
					firstName,
					lastName,
					displayName,
					email,
					...(currentPassword && newPassword ? {
						currentPassword,
						newPassword
					} : {})
				})
			});

			if (!response.ok) {
				const errorData = await response.json();
				return fail(response.status, { 
					error: errorData.message || 'Failed to update account details' 
				});
			}

			return {
				success: true,
				message: 'Account details updated successfully'
			};
		} catch (err) {
			console.error('Error updating account:', err);
			return fail(500, { error: 'An error occurred while updating your account' });
		}
	}
};
