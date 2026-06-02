/**
 * Admin Orders — shared types.
 * ─────────────────────────────────────────────────────────────────────────────
 * Single source of truth for the order shapes, hoisted out of the page and out
 * of `orders.remote.ts` (a `.remote.ts` may only export remote functions). The
 * `OrderDetail`/`OrderItem` shapes mirror what `OrderDetailModal` consumes.
 */

export interface Order {
	id: number;
	order_number: string;
	status: string;
	total: number;
	currency: string;
	user_email: string;
	user_name: string | null;
	payment_provider: string | null;
	item_count: number;
	created_at: string;
	completed_at: string | null;
}

export interface OrderStats {
	total_orders: number;
	completed_orders: number;
	pending_orders: number;
	refunded_orders: number;
	total_revenue: number;
	revenue_this_month: number;
	average_order_value: number;
}

export interface Pagination {
	page: number;
	per_page: number;
	total: number;
	total_pages: number;
}

export interface OrderItem {
	name: string;
	quantity: number;
	unit_price: number;
	total: number;
}

export interface OrderDetail {
	status: string;
	total: number;
	currency?: string;
	subtotal: number;
	discount: number;
	created_at: string;
	completed_at?: string;
	billing_name?: string;
	billing_email?: string;
	coupon_code?: string;
	items?: OrderItem[];
}

/** The `/api/admin/orders` list payload (list + stats + pagination). */
export interface OrderListResult {
	data: Order[];
	stats: OrderStats | null;
	pagination: Pagination | null;
}
