/**
 * R23-C extraction (2026-05-20): hoisted types for the coupon edit page so
 * children can share the same shapes without re-importing from the parent.
 */

export interface ValidationError {
	field: string;
	message: string;
	severity: 'error' | 'warning' | 'info';
}

export type CouponDiscountType =
	| 'fixed'
	| 'percentage'
	| 'bogo'
	| 'free_shipping'
	| 'tiered'
	| 'bundle'
	| 'cashback'
	| 'points';

export type CouponDuration = 'once' | 'forever' | 'repeating';

export interface CouponFormData {
	code: string;
	type: CouponDiscountType;
	value: number;
	description: string;
	minimum_amount: number | null;
	max_discount_amount: number | null;
	usage_limit: number | null;
	valid_from: string;
	valid_until: string;
	is_active: boolean;
	stackable: boolean;
	// Batch 3.5+ subscription duration (Stripe Coupon `duration`).
	duration: CouponDuration;
	duration_in_months: number | null;
}
