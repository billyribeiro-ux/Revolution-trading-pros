/**
 * Stripe Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Centralized Stripe configuration for the application.
 * Uses environment variables for security.
 *
 * @version 1.0.0 - January 2026
 */

import { browser } from '$app/environment';
import { PUBLIC_STRIPE_PUBLISHABLE_KEY } from '$env/static/public';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get Stripe publishable key from environment
 * This is safe to use on the client side
 */
export function getStripePublishableKey(): string {
	const key = PUBLIC_STRIPE_PUBLISHABLE_KEY;
	
	if (!key) {
		console.error('[Stripe] PUBLIC_STRIPE_PUBLISHABLE_KEY not configured in environment');
		throw new Error('Stripe publishable key not configured');
	}
	
	return key;
}

/**
 * Check if Stripe is configured
 */
export function isStripeConfigured(): boolean {
	return !!PUBLIC_STRIPE_PUBLISHABLE_KEY;
}

/**
 * Check if running in test mode
 */
export function isStripeTestMode(): boolean {
	const key = PUBLIC_STRIPE_PUBLISHABLE_KEY;
	return key?.startsWith('pk_test_') ?? false;
}

/**
 * Get Stripe mode label
 */
export function getStripeMode(): 'test' | 'live' | 'unconfigured' {
	if (!isStripeConfigured()) return 'unconfigured';
	return isStripeTestMode() ? 'test' : 'live';
}

// ═══════════════════════════════════════════════════════════════════════════
// Stripe Configuration Object
// ═══════════════════════════════════════════════════════════════════════════

export const stripeConfig = {
	get publishableKey() {
		return getStripePublishableKey();
	},
	get isConfigured() {
		return isStripeConfigured();
	},
	get isTestMode() {
		return isStripeTestMode();
	},
	get mode() {
		return getStripeMode();
	}
} as const;

// ═══════════════════════════════════════════════════════════════════════════
// Type Exports
// ═══════════════════════════════════════════════════════════════════════════

export interface StripePaymentResult {
	paymentMethodId: string;
	paymentIntentId?: string;
	last4: string;
	cardBrand: string;
	expiryMonth: number;
	expiryYear: number;
}

export interface StripePaymentOptions {
	amount: number;
	currency?: string;
	description?: string;
	customerEmail?: string;
	metadata?: Record<string, string>;
}
