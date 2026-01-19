/**
 * Thank You Page Types
 * Shared types for order details display
 *
 * @version 1.0.0
 */

export interface OrderItem {
	id: number;
	product_id: number | null;
	plan_id: number | null;
	name: string;
	quantity: number;
	unit_price: number;
	total: number;
	product_type: string | null;
	product_slug: string | null;
	thumbnail: string | null;
}

export interface OrderDetail {
	id: number;
	order_number: string;
	status: string;
	subtotal: number;
	discount: number;
	tax: number;
	total: number;
	currency: string;
	billing_name: string | null;
	billing_email: string | null;
	billing_address: Record<string, unknown> | null;
	payment_provider: string | null;
	coupon_code: string | null;
	items: OrderItem[];
	created_at: string;
	completed_at: string | null;
}
