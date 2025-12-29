/**
 * Thank You Page Load Configuration
 * Enterprise Pattern: Disable prerendering for dynamic order confirmation pages
 *
 * This page requires runtime access to URL parameters (order number, product)
 * and cannot be statically prerendered.
 *
 * @version 1.0.0 - L8 Principal Engineer
 */
import type { Load } from '@sveltejs/kit';

// Disable prerendering - this page requires dynamic URL parameters
export const prerender = false;

// Enable client-side rendering for maximum flexibility
export const ssr = true;

export const load: Load = async ({ url }) => {
	// Extract order details from URL parameters
	const orderNumber = url.searchParams.get('order');
	const productName = url.searchParams.get('product');
	const paymentId = url.searchParams.get('payment_intent');
	const sessionId = url.searchParams.get('session_id');

	return {
		orderNumber,
		productName,
		paymentId,
		sessionId,
		// Metadata for tracking
		purchaseTimestamp: Date.now()
	};
};
