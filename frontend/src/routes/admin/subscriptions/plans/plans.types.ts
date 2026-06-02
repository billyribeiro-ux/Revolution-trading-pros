/**
 * Subscription Plans — shared types.
 * ─────────────────────────────────────────────────────────────────────────────
 * Hoisted out of the page and out of `plans.remote.ts` (a `.remote.ts` may only
 * export remote functions).
 */

export type ApplyTo = 'new_only' | 'next_renewal' | 'immediate_proration';

export interface SubscriptionPlan {
	id: number;
	name: string;
	slug: string;
	display_name?: string;
	description?: string;
	price: number;
	billing_cycle: string;
	interval_count?: number;
	is_active: boolean;
	stripe_price_id?: string;
	stripe_product_id?: string;
	features?: string[];
	trial_days?: number;
	trial_period_days?: number | null;
	trial_requires_payment_method?: boolean;
	room_id?: number;
	room_name?: string;
	savings_percent?: number;
	is_popular?: boolean;
	sort_order?: number;
	created_at: string;
	updated_at: string;
}

export interface PriceHistoryEntry {
	id: number;
	old_stripe_price_id: string | null;
	new_stripe_price_id: string;
	old_amount_cents: number | null;
	new_amount_cents: number;
	currency: string;
	billing_interval: string;
	apply_to: ApplyTo;
	subscriptions_migrated: number;
	subscriptions_failed: number;
	changed_at: string;
}

/** Result of a price change — the page uses the migration counts in its toast.
 *  Counts are typed as required to match the page's existing un-guarded usage
 *  (the backend returns them on the migration paths). */
export interface PriceChangeResult {
	subscriptions_migrated: number;
	subscriptions_failed: number;
	[key: string]: unknown;
}
