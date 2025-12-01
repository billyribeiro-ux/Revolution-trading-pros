/**
 * Behavior Tracker Consent Integration
 *
 * Wires the BehaviorTracker to respect consent preferences.
 * Only enables tracking when the user has granted analytics consent.
 *
 * @module consent/behavior-integration
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState } from './types';
import { consentStore } from './store';
import { get } from 'svelte/store';

/**
 * Track if the behavior tracker has been initialized.
 */
let trackerInitialized = false;
let trackerEnabled = false;

/**
 * Reference to the behavior tracker instance.
 */
let behaviorTracker: any = null;

/**
 * Store the unsubscribe function for cleanup.
 */
let consentUnsubscribe: (() => void) | null = null;

/**
 * Initialize behavior tracking with consent awareness.
 *
 * This replaces the auto-initialization in behavior/init.ts
 * to ensure tracking only starts when consent is granted.
 */
export async function initConsentAwareBehaviorTracking(): Promise<void> {
	if (!browser) return;

	// Avoid double initialization
	if (trackerInitialized) {
		console.debug('[BehaviorIntegration] Already initialized');
		return;
	}

	trackerInitialized = true;

	// Get initial consent state
	const consent = get(consentStore);

	// Only start tracking if analytics consent is granted
	if (consent.analytics) {
		await enableTracking();
	} else {
		console.debug('[BehaviorIntegration] Analytics consent not granted, tracking disabled');
	}

	// Subscribe to consent changes
	consentUnsubscribe = consentStore.subscribe(handleConsentChange);

	console.debug('[BehaviorIntegration] Initialized with consent:', consent.analytics);
}

/**
 * Handle consent state changes.
 */
async function handleConsentChange(consent: ConsentState): Promise<void> {
	if (!browser) return;

	if (consent.analytics && !trackerEnabled) {
		// Consent granted - enable tracking
		await enableTracking();
	} else if (!consent.analytics && trackerEnabled) {
		// Consent revoked - disable tracking
		disableTracking();
	}
}

/**
 * Enable the behavior tracker.
 */
async function enableTracking(): Promise<void> {
	if (!browser || trackerEnabled) return;

	try {
		// Dynamically import the behavior tracker
		const { BehaviorTracker } = await import('$lib/behavior/tracker');

		behaviorTracker = new BehaviorTracker({
			apiEndpoint: '/api/behavior/events',
			trackScrollDepth: true,
			trackRageClicks: true,
			trackHoverIntent: true,
			trackFormBehavior: true,
			trackIdleTime: true,
			sampleRate: 1.0,
			// Enhanced privacy settings
			maskPII: true,
			respectDNT: false, // We handle DNT via consent
			anonymizeIP: true,
		});

		// Expose for consent analytics integration
		(window as any).__behaviorTracker = behaviorTracker;

		trackerEnabled = true;
		console.debug('[BehaviorIntegration] Tracking enabled');
	} catch (e) {
		console.error('[BehaviorIntegration] Failed to enable tracking:', e);
	}
}

/**
 * Disable the behavior tracker.
 */
function disableTracking(): void {
	if (!browser || !trackerEnabled) return;

	try {
		if (behaviorTracker && typeof behaviorTracker.destroy === 'function') {
			behaviorTracker.destroy();
		}

		behaviorTracker = null;
		(window as any).__behaviorTracker = null;
		trackerEnabled = false;

		console.debug('[BehaviorIntegration] Tracking disabled');
	} catch (e) {
		console.error('[BehaviorIntegration] Failed to disable tracking:', e);
	}
}

/**
 * Set the user ID on the behavior tracker.
 */
export function setBehaviorUserId(userId: string | null): void {
	if (!browser || !behaviorTracker) return;

	if (userId && typeof behaviorTracker.setUserId === 'function') {
		behaviorTracker.setUserId(userId);
		console.debug('[BehaviorIntegration] Set user ID');
	}
}

/**
 * Track a custom event (if consent is granted).
 */
export function trackBehaviorEvent(
	eventType: string,
	metadata?: Record<string, unknown>
): void {
	if (!browser || !trackerEnabled || !behaviorTracker) {
		console.debug('[BehaviorIntegration] Cannot track event - tracking not enabled');
		return;
	}

	try {
		behaviorTracker.trackEvent({
			event_type: eventType,
			timestamp: Date.now(),
			page_url: window.location.href,
			event_metadata: metadata,
		});
	} catch (e) {
		console.error('[BehaviorIntegration] Failed to track event:', e);
	}
}

/**
 * Check if behavior tracking is currently enabled.
 */
export function isBehaviorTrackingEnabled(): boolean {
	return trackerEnabled;
}

/**
 * Get the current behavior tracker instance (if any).
 */
export function getBehaviorTracker(): any {
	return behaviorTracker;
}

/**
 * Cleanup - call on unmount.
 */
export function cleanupBehaviorIntegration(): void {
	if (consentUnsubscribe) {
		consentUnsubscribe();
		consentUnsubscribe = null;
	}

	disableTracking();
	trackerInitialized = false;

	console.debug('[BehaviorIntegration] Cleaned up');
}

/**
 * Force flush any pending behavior events.
 */
export async function flushBehaviorEvents(): Promise<void> {
	if (!browser || !behaviorTracker) return;

	try {
		if (typeof behaviorTracker.flush === 'function') {
			await behaviorTracker.flush();
			console.debug('[BehaviorIntegration] Flushed pending events');
		}
	} catch (e) {
		console.error('[BehaviorIntegration] Failed to flush events:', e);
	}
}
