import type { CouponType, CouponValidationResponse } from '$lib/api/coupons';

export interface AppliedCoupon {
	code: string;
	discountAmount: number;
	type: CouponType;
}

export function getCouponDiscountAmount(
	result: CouponValidationResponse,
	cartTotal: number
): number {
	const explicitAmount =
		typeof result.discountAmount === 'number'
			? result.discountAmount
			: typeof result.finalPrice === 'number'
				? cartTotal - result.finalPrice
				: undefined;

	const amount =
		explicitAmount ??
		(result.type === 'percentage' ? (cartTotal * result.discount) / 100 : result.discount);

	if (!Number.isFinite(amount)) return 0;
	return Math.min(Math.max(amount, 0), cartTotal);
}

export function createAppliedCoupon(
	code: string,
	result: CouponValidationResponse,
	cartTotal: number
): AppliedCoupon {
	return {
		code,
		discountAmount: getCouponDiscountAmount(result, cartTotal),
		type: result.type ?? 'fixed'
	};
}
