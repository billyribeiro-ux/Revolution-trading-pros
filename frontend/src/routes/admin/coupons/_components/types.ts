/**
 * R27-C extraction (2026-05-20): shared types for the admin coupons list
 * page. Re-export of the canonical `Coupon` type from `$lib/api/admin`,
 * plus the local filter-status discriminator the page UI uses.
 */
export type { Coupon } from '$lib/api/admin';

export type FilterStatus = 'all' | 'active' | 'inactive';
