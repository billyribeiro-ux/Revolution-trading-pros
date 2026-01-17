/**
 * Checkout API Service
 *
 * Handles payment processing with Stripe integration.
 * Connects to backend PaymentController endpoints.
 */

import { browser } from '$app/environment';
import { authStore } from '$lib/stores/auth.svelte';

// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB
const API_BASE = '/api';

export interface StripeConfig {
	publishable_key: string;
	test_mode: boolean;
	supported_currencies: string[];
}

export interface PaymentIntent {
	id: string;
	client_secret: string;
	amount: number;
	currency: string;
	status: string;
}

export interface CheckoutSession {
	id: string;
	url: string;
	payment_status: string;
	expires_at: number;
}

export interface Order {
	id: string;
	user_id: string;
	status: string;
	subtotal: number;
	discount_total: number;
	tax_total: number;
	total: number;
	currency: string;
	items: OrderItem[];
}

export interface OrderItem {
	id: string;
	product_id: string;
	name: string;
	quantity: number;
	price: number;
	total: number;
}

/**
 * Get Stripe configuration
 */
export async function getStripeConfig(): Promise<StripeConfig> {
	const response = await fetch(`${API_BASE}/payment/config`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		}
	});

	if (!response.ok) {
		throw new Error('Failed to get Stripe configuration');
	}

	return response.json();
}

/**
 * Create an order from cart items
 */
export async function createOrder(cartData: {
	items: Array<{
		id: string;
		name: string;
		price: number;
		quantity: number;
		interval?: string;
	}>;
	couponCode?: string;
	discountAmount?: number;
}): Promise<Order> {
	const token = authStore.getToken();

	const response = await fetch(`${API_BASE}/orders`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			...(token ? { Authorization: `Bearer ${token}` } : {})
		},
		credentials: 'include',
		body: JSON.stringify({
			items: cartData.items.map((item) => ({
				product_id: item.id,
				name: item.name,
				price: item.price,
				quantity: item.quantity,
				billing_interval: item.interval
			})),
			coupon_code: cartData.couponCode,
			discount_amount: cartData.discountAmount
		})
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(error.message || 'Failed to create order');
	}

	return response.json();
}

/**
 * Create a payment intent for an order
 */
export async function createPaymentIntent(orderId: string): Promise<PaymentIntent> {
	const token = authStore.getToken();

	const response = await fetch(`${API_BASE}/payment/intent`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			...(token ? { Authorization: `Bearer ${token}` } : {})
		},
		credentials: 'include',
		body: JSON.stringify({ order_id: orderId })
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(error.message || 'Failed to create payment intent');
	}

	const data = await response.json();
	return data.payment_intent;
}

/**
 * Create a Stripe Checkout session
 */
export async function createCheckoutSession(
	orderId: string,
	successUrl?: string,
	cancelUrl?: string
): Promise<CheckoutSession> {
	const token = authStore.getToken();

	const response = await fetch(`${API_BASE}/payment/checkout-session`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			...(token ? { Authorization: `Bearer ${token}` } : {})
		},
		credentials: 'include',
		body: JSON.stringify({
			order_id: orderId,
			success_url: successUrl || `${browser ? window.location.origin : ''}/checkout/success`,
			cancel_url: cancelUrl || `${browser ? window.location.origin : ''}/cart`
		})
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(error.message || 'Failed to create checkout session');
	}

	const data = await response.json();
	return data.session;
}

/**
 * Confirm payment intent
 */
export async function confirmPayment(
	paymentIntentId: string,
	paymentMethodId: string
): Promise<{ success: boolean; status: string }> {
	const token = authStore.getToken();

	const response = await fetch(`${API_BASE}/payment/confirm`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			...(token ? { Authorization: `Bearer ${token}` } : {})
		},
		credentials: 'include',
		body: JSON.stringify({
			payment_intent_id: paymentIntentId,
			payment_method_id: paymentMethodId
		})
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(error.message || 'Failed to confirm payment');
	}

	return response.json();
}

/**
 * Get order status
 */
export async function getOrderStatus(orderId: string): Promise<Order> {
	const token = authStore.getToken();

	const response = await fetch(`${API_BASE}/orders/${orderId}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			...(token ? { Authorization: `Bearer ${token}` } : {})
		},
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to get order status');
	}

	return response.json();
}

/**
 * Process refund for an order
 */
export async function requestRefund(
	orderId: string,
	reason?: string,
	amount?: number
): Promise<{ success: boolean; refund_id: string }> {
	const token = authStore.getToken();

	const response = await fetch(`${API_BASE}/payment/refund`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			...(token ? { Authorization: `Bearer ${token}` } : {})
		},
		credentials: 'include',
		body: JSON.stringify({
			order_id: orderId,
			reason,
			amount
		})
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(error.message || 'Failed to process refund');
	}

	return response.json();
}
