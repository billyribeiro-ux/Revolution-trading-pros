/**
 * Subscription Plans — Remote Functions (queries + commands)
 * ─────────────────────────────────────────────────────────────────────────────
 * Typed wrappers over the admin subscription-plan proxies. Consumed
 * **imperatively** (the page keeps `loadPlans()` and re-loads after each
 * mutation), so the commands do NOT single-flight-refresh — `loadPlans()` is the
 * refresh path. NOTE: `getPlans()` takes no argument, so it is cached under a
 * single key; the page MUST re-read it via `.refresh()` after a mutation (a bare
 * `await getPlans()` would return the stale cached list).
 *
 * SURGICAL INTENT: this is a billing-sensitive page. The commands are thin
 * forwarders — all money normalization (`Math.round(price * 100)` → `price_cents`
 * / `amount_cents`) and the migration-detail messaging stay in the component,
 * verbatim. The commands only move the `fetch` to the server and surface the
 * backend's `{ error }`. `changePlanPrice` returns the backend payload
 * (`subscriptions_migrated` / `subscriptions_failed`) the page reports.
 *
 * Auth: `getRequestEvent().fetch` forwards the request cookies to the admin
 * proxies (`requireAdmin`).
 */
import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { command, getRequestEvent, query } from '$app/server';
import type { PriceChangeResult, PriceHistoryEntry, SubscriptionPlan } from './plans.types';

const IdSchema = v.pipe(v.number(), v.integer(), v.minValue(1));

async function failFrom(res: Response, fallback: string): Promise<never> {
	let message = fallback;
	try {
		const body = await res.json();
		message = body?.error || message;
	} catch {
		/* non-JSON body */
	}
	error(res.status, message);
}

export const getPlans = query(async (): Promise<SubscriptionPlan[]> => {
	const { fetch } = getRequestEvent();
	const res = await fetch('/api/admin/subscriptions/plans?per_page=100');
	if (!res.ok) error(res.status, `Failed to load plans: ${res.status}`);
	const body = await res.json();
	return Array.isArray(body?.data) ? (body.data as SubscriptionPlan[]) : [];
});

export const getPriceHistory = query(IdSchema, async (planId): Promise<PriceHistoryEntry[]> => {
	const { fetch } = getRequestEvent();
	const res = await fetch(`/api/admin/subscriptions/plans/${planId}/price-history`);
	if (!res.ok) return [];
	const body = await res.json();
	return Array.isArray(body?.data) ? (body.data as PriceHistoryEntry[]) : [];
});

/** Change a plan's price (Stripe-syncing). Returns the backend migration counts. */
export const changePlanPrice = command(
	v.object({
		planId: IdSchema,
		amount_cents: v.pipe(v.number(), v.integer(), v.minValue(1)),
		currency: v.string(),
		billing_interval: v.picklist(['month', 'year', 'one_time']),
		apply_to: v.picklist(['new_only', 'next_renewal', 'immediate_proration'])
	}),
	async ({ planId, amount_cents, currency, billing_interval, apply_to }): Promise<PriceChangeResult> => {
		const { fetch } = getRequestEvent();
		const res = await fetch(`/api/admin/subscriptions/plans/${planId}/price`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ amount_cents, currency, billing_interval, apply_to })
		});
		if (!res.ok) await failFrom(res, `Failed to change price: ${res.status}`);
		return (await res.json()) as PriceChangeResult;
	}
);

/** Update a plan. `payload` is built (and money-normalized) by the caller. */
export const updatePlan = command(
	v.object({ planId: IdSchema, payload: v.record(v.string(), v.unknown()) }),
	async ({ planId, payload }) => {
		const { fetch } = getRequestEvent();
		const res = await fetch(`/api/admin/subscriptions/plans/${planId}`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(payload)
		});
		if (!res.ok) await failFrom(res, `Failed to save: ${res.status}`);
	}
);

/** Toggle a plan's active state. */
export const setPlanActive = command(
	v.object({ planId: IdSchema, is_active: v.boolean() }),
	async ({ planId, is_active }) => {
		const { fetch } = getRequestEvent();
		const res = await fetch(`/api/admin/subscriptions/plans/${planId}`, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ is_active })
		});
		if (!res.ok) error(res.status, 'Failed to toggle status');
	}
);
