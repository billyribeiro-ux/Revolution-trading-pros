/**
 * FluentForm Pro Components
 *
 * Advanced form field components ported from FluentForm Pro plugin.
 * These components provide enhanced functionality for forms including:
 * - Payment processing
 * - Address fields with autocomplete
 * - International phone numbers
 * - NPS surveys
 * - Calculators
 * - GDPR/Terms compliance
 * - Save & Resume functionality
 */

// Payment Components
export { default as PaymentField } from './PaymentField.svelte';
export { default as PaymentMethodSelector } from './PaymentMethodSelector.svelte';
export { default as PaymentSummary } from './PaymentSummary.svelte';
export { default as CouponField } from './CouponField.svelte';

// Pro Input Fields
export { default as AddressField } from './AddressField.svelte';
export { default as PhoneIntlField } from './PhoneIntlField.svelte';
export { default as NPSField } from './NPSField.svelte';
export { default as RangeSliderField } from './RangeSliderField.svelte';
export { default as ToggleField } from './ToggleField.svelte';
export { default as CalculatorField } from './CalculatorField.svelte';

// Compliance Fields
export { default as GDPRField } from './GDPRField.svelte';

// Form Actions
export { default as SaveProgressButton } from './SaveProgressButton.svelte';

// PDF Generation
export { default as PdfDownload } from './PdfDownload.svelte';

// Type definitions
export interface PaymentItem {
	id: string;
	label: string;
	price: number;
	quantity?: number;
	image?: string;
}

export interface AddressValue {
	address_line_1: string;
	address_line_2: string;
	city: string;
	state: string;
	zip: string;
	country: string;
}

export interface PhoneValue {
	country_code: string;
	dial_code: string;
	number: string;
	full: string;
}

export interface CouponResult {
	valid: boolean;
	code: string;
	discount_type: 'percentage' | 'fixed';
	discount_value: number;
	message: string;
}

export interface LineItem {
	id: string;
	name: string;
	price: number;
	quantity: number;
	subtotal: number;
}

export interface DiscountInfo {
	code: string;
	type: 'percentage' | 'fixed';
	value: number;
	amount: number;
}

export interface TaxInfo {
	rate: number;
	amount: number;
}
