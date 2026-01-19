/**
 * Thank You Page Load Configuration
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Fetches complete order details from API for display on thank-you page.
 * Handles both authenticated and unauthenticated states gracefully.
 *
 * NOTE: This is a server-side load function (+page.server.ts) because it needs
 * access to cookies for authentication. The data is passed to the client via PageData.
 *
 * @version 2.0.0 - Complete Order Details Integration
 */
import type { ServerLoad } from '@sveltejs/kit';
import { API_BASE_URL } from '$lib/api/config';
import type { OrderDetail } from './types';

// Disable prerendering - this page requires dynamic URL parameters
export const prerender = false;

// Enable SSR for initial page load
export const ssr = true;

export const load: ServerLoad = async ({ url, fetch, cookies }) => {
	// Extract order details from URL parameters
	const orderNumber = url.searchParams.get('order');
	const productName = url.searchParams.get('product');
	const paymentId = url.searchParams.get('payment_intent');
	const sessionId = url.searchParams.get('session_id');

	let orderDetail: OrderDetail | null = null;
	let fetchError: string | null = null;

	// Fetch order details if order number is provided
	if (orderNumber) {
		try {
			const apiUrl = API_BASE_URL;
			const token = cookies.get('auth_token');

			const response = await fetch(`${apiUrl}/api/my/orders/by-number/${encodeURIComponent(orderNumber)}`, {
				headers: {
					'Content-Type': 'application/json',
					...(token ? { Authorization: `Bearer ${token}` } : {})
				},
				credentials: 'include'
			});

			if (response.ok) {
				const data = await response.json();
				if (data.success && data.data) {
					orderDetail = data.data as OrderDetail;
				}
			} else if (response.status === 401) {
				// User not authenticated - order details will be shown without API data
				fetchError = 'Please log in to view complete order details.';
			} else if (response.status === 404) {
				// Order not found or doesn't belong to user
				fetchError = 'Order details not found. Please check your order number.';
			} else {
				fetchError = 'Unable to load order details. Your order was successful.';
			}
		} catch (error) {
			console.error('Failed to fetch order details:', error);
			fetchError = 'Unable to load order details. Your order was successful.';
		}
	}

	return {
		orderNumber,
		productName,
		paymentId,
		sessionId,
		orderDetail,
		fetchError,
		// Metadata for tracking
		purchaseTimestamp: Date.now()
	};
};
