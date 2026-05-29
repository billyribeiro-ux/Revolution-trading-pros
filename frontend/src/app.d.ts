// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

import type { PayPalNamespace } from '@paypal/paypal-js';
import type { StripeConstructor } from '@stripe/stripe-js';

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
		// Deferred: these checkout globals have no official npm types matching
		// our current usage. Square in particular needs an SDK-API review — the
		// Apple/Google Pay flow (`payments.applePay({...})` + `.attach()`) targets
		// an older Web Payments SDK than the current `payments.paymentRequest()`.
		Square?: any;
		Paddle?: any;
		Razorpay?: any;
		PaystackPop?: any;
	}
}

export {};
