/**
 * Analytics Tracking
 * ═══════════════════════════════════════════════════════════════════════════
 * Wrapper for analytics providers (Google Analytics, Plausible, etc.)
 */

import { dev } from '$app/environment';

interface AnalyticsEvent {
	action: string;
	category: string;
	label?: string;
	value?: number;
}

/**
 * Track page view
 */
export function trackPageView(url: string): void {
	if (dev) {
		console.log('[Analytics] Page view:', url);
		return;
	}

	// Google Analytics
	if (typeof window !== 'undefined' && (window as any).gtag) {
		(window as any).gtag('config', 'GA_MEASUREMENT_ID', {
			page_path: url
		});
	}

	// Plausible
	if (typeof window !== 'undefined' && (window as any).plausible) {
		(window as any).plausible('pageview');
	}
}

/**
 * Track custom event
 */
export function trackEvent({ action, category, label, value }: AnalyticsEvent): void {
	if (dev) {
		console.log('[Analytics] Event:', { action, category, label, value });
		return;
	}

	// Google Analytics
	if (typeof window !== 'undefined' && (window as any).gtag) {
		(window as any).gtag('event', action, {
			event_category: category,
			event_label: label,
			value: value
		});
	}

	// Plausible
	if (typeof window !== 'undefined' && (window as any).plausible) {
		(window as any).plausible(action, {
			props: { category, label, value }
		});
	}
}

/**
 * Track block interaction
 */
export function trackBlockInteraction(blockType: string, action: string): void {
	trackEvent({
		action,
		category: 'Block Interaction',
		label: blockType
	});
}

/**
 * Track error
 */
export function trackError(error: Error, fatal = false): void {
	trackEvent({
		action: 'error',
		category: 'Error',
		label: error.message,
		value: fatal ? 1 : 0
	});
}
