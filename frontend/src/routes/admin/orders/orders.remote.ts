/**
 * Admin Orders — Remote Functions (queries)
 * ─────────────────────────────────────────────────────────────────────────────
 * Replaces the page's raw `fetch('/api/admin/orders…')` calls with typed,
 * valibot-validated `query` functions. The page consumes them imperatively via
 * `await getOrders(...)` / `await getOrderDetail(...)`; SvelteKit 2.61+ supports
 * awaiting remote queries in event handlers and async callbacks with cache
 * deduping across identical active consumers.
 *
 * Auth: `getRequestEvent().fetch('/api/admin/orders…')` forwards the request
 * cookies to the existing admin proxy, which enforces `requireAdmin` and
 * re-signs `rtp_access_token` → `Bearer` for the Axum backend — the identical
 * path the old client `fetch` took.
 *
 * Note: CSV export stays a client `fetch` in the page — it streams a `Blob`
 * download, which isn't a (devalue-serialized) remote-function return value.
 */
import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { query, getRequestEvent } from '$app/server';
import type { OrderDetail, OrderListResult } from './orders.types';

const OrdersArgsSchema = v.object({
	page: v.optional(v.pipe(v.number(), v.integer(), v.minValue(1)), 1),
	perPage: v.optional(v.pipe(v.number(), v.integer(), v.minValue(1), v.maxValue(100)), 25),
	status: v.optional(v.string(), ''),
	search: v.optional(v.string(), '')
});

/**
 * Paginated, filterable order list (+ stats + pagination). Throws on a non-OK
 * response so the page's loader catch block drives the error banner (matching
 * the previous `throw new Error(...)` behaviour).
 */
export const getOrders = query(OrdersArgsSchema, async (args): Promise<OrderListResult> => {
	const { fetch } = getRequestEvent();

	const params = new URLSearchParams();
	params.set('page', String(args.page));
	params.set('per_page', String(args.perPage));
	if (args.status) params.set('status', args.status);
	if (args.search) params.set('search', args.search);

	const res = await fetch(`/api/admin/orders?${params}`);
	if (!res.ok) error(res.status, 'Failed to load orders');

	const body = await res.json();
	const rawOrders: Record<string, unknown>[] = Array.isArray(body?.data)
		? body.data
		: Array.isArray(body?.orders)
			? body.orders
			: [];
	const data = rawOrders.map((o) => ({
		...(o as unknown as OrderListResult['data'][number]),
		total:
			typeof o.total === 'number' && !Number.isNaN(o.total)
				? o.total
				: typeof o.total_cents === 'number'
					? o.total_cents / 100
					: Number(o.total ?? 0) || 0
	}));
	const rawStats = (body?.stats ?? null) as Record<string, unknown> | null;
	const stats = rawStats
		? {
				total_orders: Number(rawStats.total_orders ?? 0) || 0,
				completed_orders: Number(rawStats.completed_orders ?? 0) || 0,
				pending_orders: Number(rawStats.pending_orders ?? 0) || 0,
				refunded_orders: Number(rawStats.refunded_orders ?? 0) || 0,
				total_revenue:
					typeof rawStats.total_revenue === 'number' && !Number.isNaN(rawStats.total_revenue)
						? rawStats.total_revenue
						: typeof rawStats.total_revenue_cents === 'number'
							? rawStats.total_revenue_cents / 100
							: Number(rawStats.total_revenue ?? 0) || 0,
				revenue_this_month:
					typeof rawStats.revenue_this_month === 'number' &&
					!Number.isNaN(rawStats.revenue_this_month)
						? rawStats.revenue_this_month
						: typeof rawStats.revenue_this_month_cents === 'number'
							? rawStats.revenue_this_month_cents / 100
							: Number(rawStats.revenue_this_month ?? 0) || 0,
				average_order_value:
					typeof rawStats.average_order_value === 'number' &&
					!Number.isNaN(rawStats.average_order_value)
						? rawStats.average_order_value
						: typeof rawStats.average_order_value_cents === 'number'
							? rawStats.average_order_value_cents / 100
							: Number(rawStats.average_order_value ?? 0) || 0
			}
		: null;
	return {
		data,
		stats,
		pagination: body?.pagination ?? null
	};
});

/** Full detail for a single order (loaded on demand when the modal opens). */
export const getOrderDetail = query(
	v.pipe(v.number(), v.integer(), v.minValue(1)),
	async (id): Promise<OrderDetail | null> => {
		const { fetch } = getRequestEvent();

		const res = await fetch(`/api/admin/orders/${id}`);
		if (!res.ok) error(res.status, 'Failed to load order details');

		const body = await res.json();
		return body?.data ?? null;
	}
);
