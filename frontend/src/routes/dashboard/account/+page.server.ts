/**
 * Account Dashboard Server - Load & Form Actions
 * Handles profile updates, password changes
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { fail } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';

/**
 * Membership type for display
 */
interface Membership {
	name: string;
	slug: string;
	startDate: string;
	endDate: string | null;
}

/**
 * Billing information type
 */
interface BillingInfo {
	email: string | null;
	phone: string | null;
	address: string | null;
}

/**
 * Page load function
 * Parent layout already handles auth - we just add account-specific data
 */
export const load: PageServerLoad = async ({ parent }) => {
	// Get user data from parent layout
	const parentData = await parent();

	// Mock memberships data (replace with actual API call)
	const memberships: { active: Membership[]; expired: Membership[] } = {
		active: [
			{
				name: 'Options Gold',
				slug: 'options-gold',
				startDate: 'January 15, 2024',
				endDate: 'January 15, 2025'
			}
		],
		expired: []
	};

	// Mock billing data (replace with actual API call)
	const billing: BillingInfo = {
		email: parentData.user?.email || null,
		phone: null,
		address: null
	};

	return {
		memberships,
		billing
	};
};

/**
 * Form actions for account management
 */
export const actions: Actions = {
	/**
	 * Update user profile (name, email)
	 */
	updateProfile: async ({ request, locals: _locals }) => {
		const formData = await request.formData();
		const firstName = formData.get('first_name')?.toString().trim();
		const lastName = formData.get('last_name')?.toString().trim();
		const email = formData.get('email')?.toString().trim();

		// Validation
		if (!firstName) {
			return fail(400, { error: 'First name is required' });
		}
		if (!lastName) {
			return fail(400, { error: 'Last name is required' });
		}
		if (!email) {
			return fail(400, { error: 'Email is required' });
		}

		// Email format validation
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!emailRegex.test(email)) {
			return fail(400, { error: 'Please enter a valid email address' });
		}

		try {
			// TODO: Implement actual API call to update profile
			// await updateUserProfile(locals.user.id, { firstName, lastName, email });

			console.log('[Account] Profile update:', { firstName, lastName, email });

			return {
				success: true,
				message: 'Profile updated successfully!'
			};
		} catch (error) {
			console.error('[Account] Profile update failed:', error);
			return fail(500, { error: 'Failed to update profile. Please try again.' });
		}
	},

	/**
	 * Update user password
	 */
	updatePassword: async ({ request, locals: _locals }) => {
		const formData = await request.formData();
		const currentPassword = formData.get('current_password')?.toString();
		const newPassword = formData.get('new_password')?.toString();
		const confirmPassword = formData.get('confirm_password')?.toString();

		// Validation
		if (!currentPassword) {
			return fail(400, { error: 'Current password is required' });
		}
		if (!newPassword) {
			return fail(400, { error: 'New password is required' });
		}
		if (!confirmPassword) {
			return fail(400, { error: 'Please confirm your new password' });
		}

		// Password match validation
		if (newPassword !== confirmPassword) {
			return fail(400, { error: 'New passwords do not match' });
		}

		// Password strength validation
		if (newPassword.length < 8) {
			return fail(400, { error: 'Password must be at least 8 characters long' });
		}

		try {
			// TODO: Implement actual API call to verify current password and update
			// await verifyAndUpdatePassword(locals.user.id, currentPassword, newPassword);

			console.log('[Account] Password update requested');

			return {
				success: true,
				message: 'Password changed successfully!'
			};
		} catch (error) {
			console.error('[Account] Password update failed:', error);
			return fail(500, { error: 'Failed to change password. Please try again.' });
		}
	}
};
