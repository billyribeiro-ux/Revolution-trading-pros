/**
 * Type definitions for Account page
 * Manually defined until SvelteKit generates $types
 */

export interface Profile {
	firstName: string;
	lastName: string;
	email: string;
	avatarUrl: string | null;
}

export interface Membership {
	id: string;
	name: string;
	slug: string;
	type: string;
	status: string;
	startDate: string;
	endDate: string | null;
	price?: number;
	interval?: string;
	autoRenew?: boolean;
	canCancel?: boolean;
	accessUrl?: string;
}

export interface BillingAddress {
	line1: string;
	line2?: string;
	city: string;
	state: string;
	postalCode: string;
	country: string;
}

export interface PaymentMethod {
	type: string;
	last4: string;
	brand: string;
	expMonth: number;
	expYear: number;
}

export interface Billing {
	email: string | null;
	phone: string | null;
	address: BillingAddress | null;
	paymentMethod: PaymentMethod | null;
}

export interface AccountPageData {
	profile: Profile;
	memberships: {
		active: Membership[];
		expired: Membership[];
	};
	billing: Billing;
	error?: string;
}
