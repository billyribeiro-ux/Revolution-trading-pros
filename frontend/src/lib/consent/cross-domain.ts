/**
 * Cross-Domain Consent Sharing
 *
 * Share consent across subdomains and related domains using:
 * - postMessage communication
 * - Shared localStorage (same origin)
 * - URL parameters (for cross-origin)
 * - Cookie with domain scope
 *
 * @module consent/cross-domain
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState } from './types';
import { consentStore } from './store.svelte';
import { loadConsent } from './storage';
import { logger } from '$lib/utils/logger';

/**
 * Cross-domain configuration
 */
export interface CrossDomainConfig {
	/** List of allowed origins for postMessage */
	allowedOrigins: string[];
	/** Whether to sync via URL parameters */
	useUrlParams: boolean;
	/** Whether to use domain-wide cookie */
	useDomainCookie: boolean;
	/** Cookie domain (e.g., '.example.com' for all subdomains) */
	cookieDomain?: string;
	/** Encryption key for secure transmission (optional) */
	encryptionKey?: string;
}

/**
 * Default configuration
 */
const DEFAULT_CONFIG: CrossDomainConfig = {
	allowedOrigins: [],
	useUrlParams: false,
	useDomainCookie: true
};

let config: CrossDomainConfig = { ...DEFAULT_CONFIG };
let messageListener: ((event: MessageEvent) => void) | null = null;

/**
 * Message types for postMessage
 */
type CrossDomainMessageType =
	| 'consent_request'
	| 'consent_response'
	| 'consent_update'
	| 'consent_sync';

/**
 * Cross-domain message structure
 */
interface CrossDomainMessage {
	type: CrossDomainMessageType;
	source: 'rtp-consent';
	version: string;
	payload: {
		consent?: Partial<ConsentState>;
		consentId?: string;
		timestamp: string;
	};
}

/**
 * Configure cross-domain sharing
 */
export function configureCrossDomain(newConfig: Partial<CrossDomainConfig>): void {
	config = { ...config, ...newConfig };
	logger.debug('[CrossDomain] Configured:', config);
}

/**
 * Get current configuration
 */
export function getCrossDomainConfig(): CrossDomainConfig {
	return { ...config };
}

/**
 * Initialize cross-domain consent sharing
 */
export function initializeCrossDomain(): () => void {
	if (!browser) return () => {};

	// Set up domain cookie if configured
	if (config.useDomainCookie && config.cookieDomain) {
		setupDomainCookie();
	}

	// Set up postMessage listener
	messageListener = handleIncomingMessage;
	window.addEventListener('message', messageListener);

	// Check for consent in URL parameters
	if (config.useUrlParams) {
		checkUrlParameters();
	}

	// Subscribe to consent changes and broadcast
	const unsubscribe = consentStore.subscribe((consent) => {
		if (consent.hasInteracted) {
			broadcastConsentUpdate(consent);
		}
	});

	logger.debug('[CrossDomain] Initialized');

	// Return cleanup function
	return () => {
		if (messageListener) {
			window.removeEventListener('message', messageListener);
			messageListener = null;
		}
		unsubscribe();
	};
}

/**
 * Set up domain-wide cookie
 */
function setupDomainCookie(): void {
	if (!browser || !config.cookieDomain) return;

	const consent = loadConsent();
	if (consent.hasInteracted && consent.consentId) {
		setDomainCookie(consent);
	}
}

/**
 * Set domain cookie
 */
function setDomainCookie(consent: ConsentState): void {
	if (!browser || !config.cookieDomain) return;

	const value = encodeURIComponent(
		JSON.stringify({
			consentId: consent.consentId,
			analytics: consent.analytics,
			marketing: consent.marketing,
			preferences: consent.preferences,
			updatedAt: consent.updatedAt
		})
	);

	const expires = consent.expiresAt ? `expires=${new Date(consent.expiresAt).toUTCString()};` : '';

	document.cookie = `rtp_consent_shared=${value}; ${expires} domain=${config.cookieDomain}; path=/; SameSite=Lax; Secure`;

	logger.debug('[CrossDomain] Set domain cookie');
}

/**
 * Get consent from domain cookie
 */
function getDomainCookie(): Partial<ConsentState> | null {
	if (!browser) return null;

	const cookies = document.cookie.split(';');
	for (const cookie of cookies) {
		const [name, value] = cookie.trim().split('=');
		if (name === 'rtp_consent_shared' && value) {
			try {
				return JSON.parse(decodeURIComponent(value));
			} catch (_e) {
				logger.debug('[CrossDomain] Failed to parse domain cookie');
			}
		}
	}
	return null;
}

/**
 * Handle incoming postMessage
 */
function handleIncomingMessage(event: MessageEvent): void {
	// Validate origin
	if (!config.allowedOrigins.includes(event.origin) && event.origin !== window.location.origin) {
		return;
	}

	// Validate message structure
	if (!event.data || event.data.source !== 'rtp-consent') {
		return;
	}

	const message = event.data as CrossDomainMessage;

	switch (message.type) {
		case 'consent_request':
			// Another domain is requesting our consent state
			handleConsentRequest(event.source as Window, event.origin);
			break;

		case 'consent_response':
		case 'consent_sync':
			// Received consent from another domain
			handleConsentSync(message.payload.consent);
			break;

		case 'consent_update':
			// Another domain updated consent
			handleConsentSync(message.payload.consent);
			break;
	}
}

/**
 * Handle consent request from another domain
 */
function handleConsentRequest(source: Window, origin: string): void {
	const consent = consentStore.getState();

	const response: CrossDomainMessage = {
		type: 'consent_response',
		source: 'rtp-consent',
		version: '1.0.0',
		payload: {
			consent: {
				...(consent.consentId && { consentId: consent.consentId }),
				analytics: consent.analytics,
				marketing: consent.marketing,
				preferences: consent.preferences,
				updatedAt: consent.updatedAt,
				hasInteracted: consent.hasInteracted
			},
			timestamp: new Date().toISOString()
		}
	};

	source.postMessage(response, origin);
	logger.debug('[CrossDomain] Sent consent response to:', origin);
}

/**
 * Handle synced consent from another domain
 */
function handleConsentSync(consent?: Partial<ConsentState>): void {
	if (!consent) return;

	const currentConsent = consentStore.getState();

	// Only sync if we don't have consent yet
	if (!currentConsent.hasInteracted && consent.hasInteracted) {
		logger.debug('[CrossDomain] Syncing consent from another domain');

		consentStore.updateCategories(
			{
				analytics: consent.analytics ?? false,
				marketing: consent.marketing ?? false,
				preferences: consent.preferences ?? false
			},
			'api'
		);
	}
}

/**
 * Broadcast consent update to other domains
 */
function broadcastConsentUpdate(consent: ConsentState): void {
	if (!browser) return;

	// Update domain cookie
	if (config.useDomainCookie && config.cookieDomain) {
		setDomainCookie(consent);
	}

	// Broadcast via postMessage to iframes
	const message: CrossDomainMessage = {
		type: 'consent_update',
		source: 'rtp-consent',
		version: '1.0.0',
		payload: {
			consent: {
				...(consent.consentId && { consentId: consent.consentId }),
				analytics: consent.analytics,
				marketing: consent.marketing,
				preferences: consent.preferences,
				updatedAt: consent.updatedAt,
				hasInteracted: consent.hasInteracted
			},
			timestamp: new Date().toISOString()
		}
	};

	// Send to parent if in iframe
	if (window.parent !== window) {
		window.parent.postMessage(message, '*');
	}

	// Send to all iframes
	const iframes = document.querySelectorAll('iframe');
	iframes.forEach((iframe) => {
		try {
			if (iframe.contentWindow) {
				iframe.contentWindow.postMessage(message, '*');
			}
		} catch (_e) {
			// Cross-origin iframe, skip
		}
	});
}

/**
 * Request consent from parent window (for iframes)
 */
export function requestConsentFromParent(): void {
	if (!browser || window.parent === window) return;

	const message: CrossDomainMessage = {
		type: 'consent_request',
		source: 'rtp-consent',
		version: '1.0.0',
		payload: {
			timestamp: new Date().toISOString()
		}
	};

	window.parent.postMessage(message, '*');
	logger.debug('[CrossDomain] Requested consent from parent');
}

/**
 * Check URL parameters for consent data
 */
function checkUrlParameters(): void {
	if (!browser) return;

	const url = new URL(window.location.href);
	const consentParam = url.searchParams.get('rtp_consent');

	if (consentParam) {
		try {
			const decoded = decodeConsentParam(consentParam);
			if (decoded) {
				logger.debug('[CrossDomain] Found consent in URL params');
				handleConsentSync(decoded);

				// Clean up URL
				url.searchParams.delete('rtp_consent');
				window.history.replaceState({}, '', url.toString());
			}
		} catch (_e) {
			logger.debug('[CrossDomain] Failed to decode URL consent param');
		}
	}
}

/**
 * Encode consent for URL parameter
 */
export function encodeConsentParam(consent: ConsentState): string {
	const data = {
		a: consent.analytics ? 1 : 0,
		m: consent.marketing ? 1 : 0,
		p: consent.preferences ? 1 : 0,
		i: consent.consentId
	};

	return btoa(JSON.stringify(data));
}

/**
 * Decode consent from URL parameter
 */
function decodeConsentParam(param: string): Partial<ConsentState> | null {
	try {
		const data = JSON.parse(atob(param));
		return {
			analytics: data.a === 1,
			marketing: data.m === 1,
			preferences: data.p === 1,
			consentId: data.i,
			hasInteracted: true
		};
	} catch (_e) {
		return null;
	}
}

/**
 * Generate URL with consent parameter
 */
export function generateConsentUrl(baseUrl: string, consent: ConsentState): string {
	const url = new URL(baseUrl);
	url.searchParams.set('rtp_consent', encodeConsentParam(consent));
	return url.toString();
}

/**
 * Sync consent from domain cookie on page load
 */
export function syncFromDomainCookie(): boolean {
	if (!browser) return false;

	const cookieConsent = getDomainCookie();
	if (!cookieConsent) return false;

	const currentConsent = loadConsent();

	// If we don't have consent but cookie does, sync it
	if (!currentConsent.hasInteracted && cookieConsent.consentId) {
		logger.debug('[CrossDomain] Syncing from domain cookie');

		consentStore.updateCategories(
			{
				analytics: cookieConsent.analytics ?? false,
				marketing: cookieConsent.marketing ?? false,
				preferences: cookieConsent.preferences ?? false
			},
			'api'
		);

		return true;
	}

	return false;
}
