/**
 * R25-C extraction (2026-05-20): hoisted types for the coupon create page so
 * children can share the same shapes without re-importing from the parent.
 *
 * Distinct from the sister `/admin/coupons/edit/[id]` types — the create page
 * speaks the canonical backend `CreateCouponRequest` field names
 * (`discount_type`/`discount_value`/`min_purchase`/`max_discount`/`starts_at`/
 * `expires_at`/`applicable_plans`) per the FIX-2026-04-26 P0-3 audit fix.
 */

export type CouponDiscountType = 'percentage' | 'fixed';

export type CouponDuration = 'once' | 'forever' | 'repeating';

export type RestrictionTab = 'include' | 'exclude';

export interface CouponFormData {
	code: string;
	description: string;
	discount_type: CouponDiscountType;
	discount_value: number;
	min_purchase: number | null;
	max_discount: number | null;
	usage_limit: number | null;
	usage_limit_per_user: number | null;
	starts_at: string;
	expires_at: string;
	is_active: boolean;
	applicable_products: number[];
	applicable_plans: number[];
	// Batch 3.5+: how long the discount applies on subscriptions.
	// 'once': first billing period only.
	// 'forever': every billing period.
	// 'repeating': next N billing periods (duration_in_months required).
	duration: CouponDuration;
	duration_in_months: number | null;
}
