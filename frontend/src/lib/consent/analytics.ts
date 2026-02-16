/**
 * Consent Analytics
 *
 * Tracks consent interaction metrics for optimizing the consent experience.
 * All data is stored locally and can be synced to the analytics backend.
 *
 * @module consent/analytics
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentCategory, ConsentAnalytics, ConsentInteractionEvent } from './types';
import { logger } from '$lib/utils/logger';

const ANALYTICS_STORAGE_KEY = 'rtp_consent_analytics';
const EVENTS_STORAGE_KEY = 'rtp_consent_events';

/**
 * Default analytics structure.
 */
function getDefaultAnalytics(): ConsentAnalytics {
	return {
		totalInteractions: 0,
		acceptAllRate: 0,
		rejectAllRate: 0,
		customRate: 0,
		categoryRates: {
			necessary: 1, // Always 100%
			analytics: 0,
			marketing: 0,
			preferences: 0
		},
		avgTimeToDecision: 0,
		bannerImpressions: 0,
		modalOpens: 0
	};
}

/**
 * Get stored analytics data.
 */
export function getConsentAnalytics(): ConsentAnalytics {
	if (!browser) return getDefaultAnalytics();

	try {
		const stored = localStorage.getItem(ANALYTICS_STORAGE_KEY);
		if (stored) {
			return { ...getDefaultAnalytics(), ...JSON.parse(stored) };
		}
	} catch (e) {
		logger.debug('[ConsentAnalytics] Failed to load analytics:', e);
	}

	return getDefaultAnalytics();
}

/**
 * Save analytics data.
 */
function saveAnalytics(analytics: ConsentAnalytics): void {
	if (!browser) return;

	try {
		localStorage.setItem(ANALYTICS_STORAGE_KEY, JSON.stringify(analytics));
	} catch (e) {
		logger.debug('[ConsentAnalytics] Failed to save analytics:', e);
	}
}

/**
 * Get stored interaction events.
 */
export function getInteractionEvents(): ConsentInteractionEvent[] {
	if (!browser) return [];

	try {
		const stored = localStorage.getItem(EVENTS_STORAGE_KEY);
		if (stored) {
			return JSON.parse(stored);
		}
	} catch (e) {
		logger.debug('[ConsentAnalytics] Failed to load events:', e);
	}

	return [];
}

/**
 * Save interaction events.
 */
function saveEvents(events: ConsentInteractionEvent[]): void {
	if (!browser) return;

	try {
		// Keep last 1000 events
		const trimmed = events.slice(-1000);
		localStorage.setItem(EVENTS_STORAGE_KEY, JSON.stringify(trimmed));
	} catch (e) {
		logger.debug('[ConsentAnalytics] Failed to save events:', e);
	}
}

/**
 * Track banner impression timestamp.
 */
let bannerShownAt: number | null = null;

/**
 * Record a consent interaction event.
 */
export function trackConsentInteraction(
	type: ConsentInteractionEvent['type'],
	categories?: Partial<Record<ConsentCategory, boolean>>
): void {
	if (!browser) return;

	const now = Date.now();

	// Calculate time to decision
	let timeToDecision: number | undefined;
	if (type === 'accept_all' || type === 'reject_all' || type === 'save_preferences') {
		if (bannerShownAt) {
			timeToDecision = now - bannerShownAt;
			bannerShownAt = null;
		}
	}

	// Track banner shown timestamp
	if (type === 'banner_shown') {
		bannerShownAt = now;
	}

	const event: ConsentInteractionEvent = {
		type,
		timestamp: now,
		...(categories && { categories }),
		...(timeToDecision !== undefined && { timeToDecision }),
		pageUrl: window.location.href
	};

	// Add to events
	const events = getInteractionEvents();
	events.push(event);
	saveEvents(events);

	// Update aggregate analytics
	updateAnalytics(event);

	// Send to behavior tracker if available
	sendToBehaviorTracker(event);

	logger.debug('[ConsentAnalytics] Tracked interaction:', event);
}

/**
 * Update aggregate analytics based on new event.
 */
function updateAnalytics(event: ConsentInteractionEvent): void {
	const analytics = getConsentAnalytics();

	switch (event.type) {
		case 'banner_shown':
			analytics.bannerImpressions++;
			break;

		case 'modal_opened':
			analytics.modalOpens++;
			break;

		case 'accept_all':
		case 'reject_all':
		case 'save_preferences':
			analytics.totalInteractions++;

			// Update decision rates
			const allEvents = getInteractionEvents();
			const decisions = allEvents.filter(
				(e) => e.type === 'accept_all' || e.type === 'reject_all' || e.type === 'save_preferences'
			);

			const acceptCount = decisions.filter((e) => e.type === 'accept_all').length;
			const rejectCount = decisions.filter((e) => e.type === 'reject_all').length;
			const customCount = decisions.filter((e) => e.type === 'save_preferences').length;

			analytics.acceptAllRate = acceptCount / decisions.length;
			analytics.rejectAllRate = rejectCount / decisions.length;
			analytics.customRate = customCount / decisions.length;

			// Update average time to decision
			const decisionsWithTime = decisions.filter((e) => e.timeToDecision);
			if (decisionsWithTime.length > 0) {
				const totalTime = decisionsWithTime.reduce((sum, e) => sum + (e.timeToDecision || 0), 0);
				analytics.avgTimeToDecision = totalTime / decisionsWithTime.length;
			}

			// Update category rates from custom preferences
			const preferencesEvents = decisions.filter(
				(e) => e.type === 'save_preferences' && e.categories
			);
			if (preferencesEvents.length > 0) {
				const categoryTotals: Record<ConsentCategory, number> = {
					necessary: preferencesEvents.length, // Always 100%
					analytics: 0,
					marketing: 0,
					preferences: 0
				};

				preferencesEvents.forEach((e) => {
					if (e.categories?.analytics) categoryTotals.analytics++;
					if (e.categories?.marketing) categoryTotals.marketing++;
					if (e.categories?.preferences) categoryTotals.preferences++;
				});

				// Include accept all in rates
				const acceptAllCount = decisions.filter((e) => e.type === 'accept_all').length;
				const totalDecisions = decisions.length;

				analytics.categoryRates = {
					necessary: 1,
					analytics: (categoryTotals.analytics + acceptAllCount) / totalDecisions,
					marketing: (categoryTotals.marketing + acceptAllCount) / totalDecisions,
					preferences: (categoryTotals.preferences + acceptAllCount) / totalDecisions
				};
			}
			break;
	}

	saveAnalytics(analytics);
}

/**
 * Send consent event to behavior tracker for unified analytics.
 */
function sendToBehaviorTracker(event: ConsentInteractionEvent): void {
	if (!browser) return;

	try {
		// Check if behavior tracker is available
		const tracker = (window as any).__behaviorTracker;
		if (tracker && typeof tracker.trackEvent === 'function') {
			tracker.trackEvent({
				event_type: `consent_${event.type}`,
				timestamp: event.timestamp,
				page_url: event.pageUrl,
				event_metadata: {
					categories: event.categories,
					timeToDecision: event.timeToDecision
				}
			});
		}
	} catch (e) {
		logger.debug('[ConsentAnalytics] Failed to send to behavior tracker:', e);
	}
}

/**
 * Get analytics summary for dashboard.
 */
export function getAnalyticsSummary(): {
	analytics: ConsentAnalytics;
	recentEvents: ConsentInteractionEvent[];
	insights: string[];
} {
	const analytics = getConsentAnalytics();
	const events = getInteractionEvents();
	const recentEvents = events.slice(-10);

	// Generate insights
	const insights: string[] = [];

	if (analytics.acceptAllRate > 0.7) {
		insights.push('High accept rate indicates trust in your privacy practices.');
	} else if (analytics.rejectAllRate > 0.5) {
		insights.push(
			'High reject rate may indicate privacy concerns. Consider reviewing your messaging.'
		);
	}

	if (analytics.avgTimeToDecision > 30000) {
		insights.push('Users take over 30 seconds to decide. The banner may be too complex.');
	} else if (analytics.avgTimeToDecision < 3000) {
		insights.push('Quick decisions suggest users may not be reading the consent notice.');
	}

	if (analytics.modalOpens > analytics.bannerImpressions * 0.3) {
		insights.push('30%+ users open preferences. Consider showing more options upfront.');
	}

	if (analytics.categoryRates.analytics > analytics.categoryRates.marketing) {
		insights.push(
			'Users prefer analytics over marketing. Consider highlighting privacy-friendly analytics.'
		);
	}

	return {
		analytics,
		recentEvents,
		insights
	};
}

/**
 * Export analytics data for reporting.
 */
export function exportAnalyticsData(): string {
	return JSON.stringify(
		{
			exportDate: new Date().toISOString(),
			analytics: getConsentAnalytics(),
			events: getInteractionEvents()
		},
		null,
		2
	);
}

/**
 * Clear all analytics data.
 */
export function clearAnalytics(): void {
	if (!browser) return;

	try {
		localStorage.removeItem(ANALYTICS_STORAGE_KEY);
		localStorage.removeItem(EVENTS_STORAGE_KEY);
		logger.debug('[ConsentAnalytics] Cleared analytics data');
	} catch (e) {
		logger.debug('[ConsentAnalytics] Failed to clear analytics:', e);
	}
}

/**
 * Sync analytics to server.
 */
export async function syncAnalyticsToServer(endpoint: string): Promise<boolean> {
	if (!browser) return false;

	try {
		const response = await fetch(endpoint, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				analytics: getConsentAnalytics(),
				events: getInteractionEvents(),
				syncTimestamp: new Date().toISOString()
			})
		});

		return response.ok;
	} catch (e) {
		logger.error('[ConsentAnalytics] Failed to sync to server:', e);
		return false;
	}
}
