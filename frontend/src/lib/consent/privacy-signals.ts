/**
 * Privacy Signals Detection
 *
 * Detects browser privacy signals:
 * - Do Not Track (DNT)
 * - Global Privacy Control (GPC)
 * - Geo-based consent requirements
 *
 * @module consent/privacy-signals
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { PrivacySignals } from './types';

/**
 * EU/EEA country codes requiring strict GDPR consent.
 */
const EU_COUNTRIES = new Set([
	'AT', 'BE', 'BG', 'HR', 'CY', 'CZ', 'DK', 'EE', 'FI', 'FR',
	'DE', 'GR', 'HU', 'IE', 'IT', 'LV', 'LT', 'LU', 'MT', 'NL',
	'PL', 'PT', 'RO', 'SK', 'SI', 'ES', 'SE', // EU members
	'IS', 'LI', 'NO', // EEA
	'GB', 'UK', // UK (post-Brexit still has strict rules)
	'CH', // Switzerland (similar requirements)
]);


/**
 * Check if Do Not Track is enabled.
 */
export function isDNTEnabled(): boolean {
	if (!browser) return false;

	// Check navigator.doNotTrack
	if (navigator.doNotTrack === '1' || navigator.doNotTrack === 'yes') {
		return true;
	}

	// Check older IE implementation
	const msDoNotTrack = (navigator as any).msDoNotTrack;
	if (msDoNotTrack === '1') {
		return true;
	}

	// Check window.doNotTrack (older spec)
	const windowDNT = (window as any).doNotTrack;
	if (windowDNT === '1' || windowDNT === 'yes') {
		return true;
	}

	return false;
}

/**
 * Check if Global Privacy Control is enabled.
 * @see https://globalprivacycontrol.org/
 */
export function isGPCEnabled(): boolean {
	if (!browser) return false;

	// Standard GPC signal
	const gpc = (navigator as any).globalPrivacyControl;
	if (gpc === true || gpc === '1') {
		return true;
	}

	// Some browsers set it on the DOM
	const domGPC = (document as any).globalPrivacyControl;
	if (domGPC === true || domGPC === '1') {
		return true;
	}

	return false;
}

/**
 * Attempt to detect user's region from browser settings.
 * Falls back to timezone-based detection.
 */
export function detectRegion(): string | undefined {
	if (!browser) return undefined;

	try {
		// Try to get from Intl API
		const locale = Intl.DateTimeFormat().resolvedOptions().locale;
		if (locale) {
			// Extract country code from locale (e.g., 'en-US' -> 'US')
			const parts = locale.split('-');
			if (parts.length >= 2) {
				const lastPart = parts[parts.length - 1];
			const country = lastPart?.toUpperCase();
				if (country && country.length === 2) {
					return country;
				}
			}
		}

		// Fallback to timezone-based detection
		const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
		if (timezone) {
			return getCountryFromTimezone(timezone);
		}
	} catch (e) {
		console.debug('[PrivacySignals] Failed to detect region:', e);
	}

	return undefined;
}

/**
 * Map timezones to country codes (simplified).
 */
function getCountryFromTimezone(timezone: string): string | undefined {
	const tzToCountry: Record<string, string> = {
		// Europe
		'Europe/London': 'GB',
		'Europe/Paris': 'FR',
		'Europe/Berlin': 'DE',
		'Europe/Rome': 'IT',
		'Europe/Madrid': 'ES',
		'Europe/Amsterdam': 'NL',
		'Europe/Brussels': 'BE',
		'Europe/Vienna': 'AT',
		'Europe/Warsaw': 'PL',
		'Europe/Prague': 'CZ',
		'Europe/Stockholm': 'SE',
		'Europe/Oslo': 'NO',
		'Europe/Copenhagen': 'DK',
		'Europe/Helsinki': 'FI',
		'Europe/Dublin': 'IE',
		'Europe/Lisbon': 'PT',
		'Europe/Athens': 'GR',
		'Europe/Zurich': 'CH',

		// North America
		'America/New_York': 'US',
		'America/Chicago': 'US',
		'America/Denver': 'US',
		'America/Los_Angeles': 'US',
		'America/Toronto': 'CA',
		'America/Vancouver': 'CA',
		'America/Mexico_City': 'MX',

		// Asia Pacific
		'Asia/Tokyo': 'JP',
		'Asia/Shanghai': 'CN',
		'Asia/Hong_Kong': 'HK',
		'Asia/Singapore': 'SG',
		'Asia/Seoul': 'KR',
		'Asia/Mumbai': 'IN',
		'Asia/Dubai': 'AE',
		'Australia/Sydney': 'AU',
		'Australia/Melbourne': 'AU',
		'Pacific/Auckland': 'NZ',

		// South America
		'America/Sao_Paulo': 'BR',
		'America/Buenos_Aires': 'AR',

		// Africa
		'Africa/Johannesburg': 'ZA',
		'Africa/Cairo': 'EG',
	};

	return tzToCountry[timezone];
}

/**
 * Check if a region requires strict consent (GDPR-style).
 */
export function requiresStrictConsent(region?: string): boolean {
	if (!region) return false;

	// EU/EEA/UK/Swiss require strict opt-in consent
	if (EU_COUNTRIES.has(region)) {
		return true;
	}

	// Brazil's LGPD
	if (region === 'BR') {
		return true;
	}

	return false;
}

/**
 * Check if a region has specific privacy laws (may affect defaults).
 */
export function hasPrivacyLaw(region?: string): boolean {
	if (!region) return false;

	// Strict consent regions
	if (requiresStrictConsent(region)) {
		return true;
	}

	// US states with privacy laws
	// Note: For US, we'd need state-level detection which is harder
	// For now, we treat all US as potentially having privacy laws
	if (region === 'US') {
		return true;
	}

	// Canada's PIPEDA
	if (region === 'CA') {
		return true;
	}

	// Japan's APPI
	if (region === 'JP') {
		return true;
	}

	// Australia's Privacy Act
	if (region === 'AU') {
		return true;
	}

	return false;
}

/**
 * Detect all privacy signals.
 */
export function detectPrivacySignals(): PrivacySignals {
	const dnt = isDNTEnabled();
	const gpc = isGPCEnabled();
	const region = detectRegion();
	const requiresStrict = requiresStrictConsent(region);

	const signals: PrivacySignals = {
		dnt,
		gpc,
		...(region && { region }),
		requiresStrictConsent: requiresStrict,
	};

	console.debug('[PrivacySignals] Detected:', signals);

	return signals;
}

/**
 * Get recommended consent defaults based on privacy signals.
 */
export function getSignalBasedDefaults(signals: PrivacySignals): {
	analytics: boolean;
	marketing: boolean;
	preferences: boolean;
} {
	// If GPC is enabled, respect it (legally required in some jurisdictions)
	if (signals.gpc) {
		console.debug('[PrivacySignals] GPC enabled - denying all non-essential');
		return {
			analytics: false,
			marketing: false,
			preferences: false,
		};
	}

	// If DNT is enabled, respect it as a strong signal
	if (signals.dnt) {
		console.debug('[PrivacySignals] DNT enabled - denying marketing and analytics');
		return {
			analytics: false,
			marketing: false,
			preferences: true, // Preferences are okay with DNT
		};
	}

	// Strict consent regions - require opt-in
	if (signals.requiresStrictConsent) {
		console.debug('[PrivacySignals] Strict consent region - requiring opt-in');
		return {
			analytics: false,
			marketing: false,
			preferences: false,
		};
	}

	// Non-strict regions - can use legitimate interest for analytics
	// But still require opt-in for marketing
	console.debug('[PrivacySignals] Non-strict region - legitimate interest for analytics');
	return {
		analytics: false, // Still default to false for safety
		marketing: false,
		preferences: false,
	};
}

/**
 * Check if we should show a consent banner.
 * In some cases (GPC enabled + non-EU), we might auto-apply restrictive defaults.
 */
export function shouldShowBanner(_signals: PrivacySignals, hasInteracted: boolean): boolean {
	// Always show if user hasn't interacted
	if (!hasInteracted) {
		return true;
	}

	return false;
}

/**
 * Get the appropriate consent UI mode based on region.
 */
export function getConsentUIMode(signals: PrivacySignals): 'strict' | 'notice' | 'minimal' {
	// EU/EEA/UK - full GDPR mode with explicit consent
	if (signals.requiresStrictConsent) {
		return 'strict';
	}

	// US with privacy laws - notice mode (can use legitimate interest)
	if (signals.region === 'US') {
		return 'notice';
	}

	// Other regions - minimal mode
	return 'minimal';
}

/**
 * Subscribe to privacy signal changes (for GPC).
 * Some browsers may update GPC dynamically.
 */
export function onPrivacySignalChange(callback: (signals: PrivacySignals) => void): () => void {
	if (!browser) return () => {};

	// Currently no standard event for GPC changes
	// But we can poll periodically as a fallback
	let lastGPC = isGPCEnabled();
	let lastDNT = isDNTEnabled();

	const interval = setInterval(() => {
		const currentGPC = isGPCEnabled();
		const currentDNT = isDNTEnabled();

		if (currentGPC !== lastGPC || currentDNT !== lastDNT) {
			lastGPC = currentGPC;
			lastDNT = currentDNT;
			callback(detectPrivacySignals());
		}
	}, 5000); // Check every 5 seconds

	return () => clearInterval(interval);
}
