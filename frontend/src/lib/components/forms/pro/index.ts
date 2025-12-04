/**
 * FluentForm Pro Components (December 2025)
 *
 * Advanced form field components ported from FluentForm Pro plugin.
 * Includes all features from FluentForms 6.1.10 (December 2025):
 * - Payment processing (Stripe, PayPal, Square, Razorpay, Mollie)
 * - Address fields with HTML5 geolocation (6.1.0 Pro)
 * - International phone numbers
 * - NPS surveys & Quiz/Survey scoring
 * - Calculators
 * - GDPR/Terms compliance
 * - Save & Resume with one-time links (6.1.0 Pro)
 * - PDF Generation with templates
 * - Accordion/Tab Input Fields (6.1.5)
 * - Enhanced Checkbox with "Others" option (6.1.5)
 * - Form Reports with PDF export (6.1.0 Pro)
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

// FluentForms 6.1.5 (November 2025) New Fields
export { default as AccordionTabField } from './AccordionTabField.svelte';
export { default as EnhancedCheckbox } from './EnhancedCheckbox.svelte';

// FluentForms 6.1.0 Pro (August 2025) Features
export { default as GeolocationAddress } from './GeolocationAddress.svelte';
export { default as FormReport } from './FormReport.svelte';

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

// FluentForms 6.1.5 (November 2025) Types
export interface AccordionSection {
	id: string;
	title: string;
	icon?: string;
	content?: string;
}

export interface CheckboxOption {
	value: string;
	label: string;
}

// FluentForms 6.1.0 Pro (August 2025) Types
export interface GeolocationAddressValue {
	address_line_1: string;
	address_line_2: string;
	city: string;
	state: string;
	zip: string;
	country: string;
	latitude?: number;
	longitude?: number;
	formatted_address?: string;
}

export interface FormReportData {
	form_id: number;
	form_title: string;
	total_submissions: number;
	date_range: {
		start: string;
		end: string;
	};
	field_reports: FieldReport[];
	conversion_rate?: number;
	avg_completion_time?: string;
	top_sources?: { source: string; count: number }[];
}

export interface FieldReport {
	field_name: string;
	field_label: string;
	field_type: string;
	responses: number;
	data: { value: string; count: number; percentage: number }[];
}
