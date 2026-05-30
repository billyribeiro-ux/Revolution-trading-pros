// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

import type { PayPalNamespace } from '@paypal/paypal-js';
import type { StripeConstructor } from '@stripe/stripe-js';
import type { Square } from '@square/web-payments-sdk-types';

/**
 * Consent State interface for server-side access.
 */
export interface ConsentState {
	necessary: boolean;
	analytics: boolean;
	marketing: boolean;
	preferences: boolean;
	updatedAt: string;
	hasInteracted: boolean;
	version: number;
}

/**
 * Meta Pixel (fbq) function type.
 */
interface Fbq {
	(method: 'init', pixelId: string): void;
	(method: 'track', eventName: string, params?: Record<string, unknown>): void;
	(method: 'trackCustom', eventName: string, params?: Record<string, unknown>): void;
	(method: 'consent', action: 'grant' | 'revoke'): void;
	(method: 'dataProcessingOptions', options: string[], country?: number, state?: number): void;
	callMethod?: (...args: unknown[]) => void;
	queue: unknown[];
	loaded: boolean;
	version: string;
	push: (...args: unknown[]) => void;
}

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			/** User's consent state from cookie */
			consent: ConsentState;
			/** Whether the user has interacted with the consent banner */
			hasConsentInteraction: boolean;
			/** Current authenticated user (if any) */
			user?: {
				id: number; // Matches Rust backend i64
				email: string;
				name?: string;
				role?: string;
			} | null;
			/** Access token for server-side API calls (from httpOnly cookie) */
			accessToken?: string | null;
			/** Authentication method to get current session */
			auth: () => Promise<{
				user: { id: number; email: string; name?: string; role?: string } | null;
			} | null>;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	/**
	 * Hand-written minimal types for checkout SDK globals with no official npm
	 * typings matching our usage (loaded via <script>). Modelled on each
	 * vendor's official docs, covering only the surface our components touch.
	 */
	// Paddle Classic (Paddle.js)
	interface PaddleEventData {
		event: string;
		eventData?: { error?: string; [key: string]: unknown };
		checkoutData?: {
			checkout?: { id?: string; coupon?: { coupon_code?: string } };
			product?: { id?: number };
			subscription?: { plan_id?: number };
			user?: { email?: string; country?: string };
			[key: string]: unknown;
		};
		[key: string]: unknown;
	}
	interface PaddleClassic {
		Setup(options: { vendor?: number; eventCallback?: (data: PaddleEventData) => void }): void;
		Environment: { set(environment: string): void };
		Checkout: { open(options: Record<string, unknown>): void };
	}

	// Razorpay Checkout (checkout.js)
	interface RazorpaySuccessResponse {
		razorpay_payment_id: string;
		razorpay_order_id?: string;
		razorpay_signature?: string;
	}
	interface RazorpayFailureResponse {
		error?: { code?: string; description?: string; reason?: string; [key: string]: unknown };
	}
	interface RazorpayInstance {
		on(event: 'payment.failed', handler: (response: RazorpayFailureResponse) => void): void;
		open(): void;
	}
	interface RazorpayOptions {
		key: string;
		amount: number;
		currency?: string;
		name?: string;
		description?: string;
		order_id?: string;
		handler?: (response: RazorpaySuccessResponse) => void;
		prefill?: Record<string, string>;
		theme?: { color?: string };
		[key: string]: unknown;
	}
	interface RazorpayConstructor {
		new (options: RazorpayOptions): RazorpayInstance;
	}

	// Paystack Inline (inline.js)
	interface PaystackResponse {
		reference: string;
		transaction?: string;
		status?: string;
		channel?: string;
		[key: string]: unknown;
	}
	interface PaystackHandler {
		openIframe(): void;
	}
	interface PaystackPop {
		setup(options: {
			key: string;
			email: string;
			amount: number;
			currency?: string;
			ref?: string;
			callback?: (response: PaystackResponse) => void;
			onClose?: () => void;
			[key: string]: unknown;
		}): PaystackHandler;
	}

	interface Window {
		/** YouTube IFrame API (consumers cast `Player` to a local ctor type) */
		YT?: {
			Player: unknown;
			PlayerState: {
				UNSTARTED: number;
				ENDED: number;
				PLAYING: number;
				PAUSED: number;
				BUFFERING: number;
				CUED: number;
			};
			ready: (callback: () => void) => void;
		};
		onYouTubeIframeAPIReady?: () => void;

		/** Google Tag (gtag.js) */
		dataLayer?: unknown[];
		gtag?: (...args: unknown[]) => void;

		/** Plausible Analytics */
		plausible?: (event: string, options?: { props?: Record<string, unknown> }) => void;

		/** Meta Pixel (fbq) */
		fbq?: Fbq;
		_fbq?: Fbq;

		/** Payment Providers */
		Stripe?: StripeConstructor;
		paypal?: PayPalNamespace | null;
		Square?: Square;
		Paddle?: PaddleClassic;
		Razorpay?: RazorpayConstructor;
		PaystackPop?: PaystackPop;
	}
}

export {};
