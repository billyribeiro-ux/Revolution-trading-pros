/**
 * LinkedIn Insight Tag Vendor Integration
 *
 * Integrates LinkedIn's conversion tracking with consent-aware loading.
 *
 * @see https://www.linkedin.com/help/lms/answer/a423304
 * @module consent/vendors/linkedin
 * @version 1.0.0
 */

import { browser, dev } from '$app/environment';
import type { VendorConfig } from '../types';
import { logger } from '$lib/utils/logger';

declare global {
	interface Window {
		_linkedin_data_partner_ids?: string[];
		lintrk?: {
			(action: string, data?: Record<string, unknown>): void;
			q?: unknown[][];
		};
	}
}

const PUBLIC_LINKEDIN_PARTNER_ID = import.meta.env['PUBLIC_LINKEDIN_PARTNER_ID'] || '';
let linkedinReady = false;
const eventQueue: Array<{ conversionId: string; data?: Record<string, unknown> }> = [];

/**
 * Initialize LinkedIn Insight Tag
 */
function initializeLinkedIn(partnerId: string): void {
	if (!browser || !partnerId) return;

	// Set partner ID
	window._linkedin_data_partner_ids = window._linkedin_data_partner_ids || [];
	window._linkedin_data_partner_ids.push(partnerId);

	// Create lintrk stub if not present
	if (!window.lintrk) {
		window.lintrk = function (action: string, data?: Record<string, unknown>) {
			window.lintrk!.q = window.lintrk!.q || [];
			window.lintrk!.q.push([action, data]);
		};
	}

	// Load LinkedIn script
	const script = document.createElement('script');
	script.type = 'text/javascript';
	script.async = true;
	script.src = 'https://snap.licdn.com/li.lms-analytics/insight.min.js';
	script.onload = () => {
		linkedinReady = true;
		processEventQueue();
		logger.debug('[LinkedIn] Insight Tag initialized:', partnerId);
	};

	document.head.appendChild(script);
}

/**
 * Process queued events
 */
function processEventQueue(): void {
	if (!window.lintrk || !linkedinReady) return;

	while (eventQueue.length > 0) {
		const { conversionId, data } = eventQueue.shift()!;
		window.lintrk('track', { conversion_id: conversionId, ...data });
	}
}

/**
 * Track a LinkedIn conversion event
 */
export function trackLinkedInConversion(
	conversionId: string,
	data?: Record<string, unknown>
): void {
	if (!browser) return;

	if (!linkedinReady) {
		eventQueue.push({ conversionId, ...(data && { data }) });
		return;
	}

	window.lintrk?.('track', { conversion_id: conversionId, ...data });
	logger.debug('[LinkedIn] Tracked conversion:', conversionId, data);
}

/**
 * Track page view (automatically tracked by Insight Tag)
 */
export function trackLinkedInPageView(): void {
	// Page views are automatically tracked by the Insight Tag
	logger.debug('[LinkedIn] Page view auto-tracked by Insight Tag');
}

/**
 * Check if LinkedIn Insight Tag is ready
 */
export function isLinkedInReady(): boolean {
	return linkedinReady && !!window.lintrk;
}

/**
 * LinkedIn Insight Tag vendor configuration
 */
export const linkedinVendor: VendorConfig = {
	id: 'linkedin',
	name: 'LinkedIn Insight Tag',
	description:
		'LinkedIn Insight Tag enables conversion tracking and audience insights for LinkedIn advertising.',
	requiredCategories: ['marketing'],
	privacyPolicyUrl: 'https://www.linkedin.com/legal/privacy-policy',
	cookies: [
		{
			name: 'li_*',
			purpose: 'LinkedIn tracking and advertising',
			duration: '6 months',
			type: 'third-party'
		},
		{
			name: 'bcookie',
			purpose: 'LinkedIn browser identifier',
			duration: '1 year',
			type: 'third-party'
		},
		{
			name: 'lidc',
			purpose: 'LinkedIn data center selection',
			duration: '24 hours',
			type: 'third-party'
		},
		{
			name: 'UserMatchHistory',
			purpose: 'LinkedIn Ads ID syncing',
			duration: '30 days',
			type: 'third-party'
		},
		{
			name: 'AnalyticsSyncHistory',
			purpose: 'LinkedIn analytics sync',
			duration: '30 days',
			type: 'third-party'
		}
	],
	dataLocations: ['United States', 'Ireland'],
	supportsRevocation: true,

	load: () => {
		const partnerId = PUBLIC_LINKEDIN_PARTNER_ID;
		if (!partnerId) {
			if (!dev) logger.warn('[LinkedIn] Missing PUBLIC_LINKEDIN_PARTNER_ID environment variable');
			return;
		}
		initializeLinkedIn(partnerId);
	},

	onConsentRevoked: () => {
		linkedinReady = false;
		eventQueue.length = 0;
		logger.debug('[LinkedIn] Consent revoked');
	}
};
