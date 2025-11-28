/**
 * SvelteKit Server Hooks
 *
 * Handles server-side request processing, including:
 * - Consent cookie reading for SSR
 * - Security headers
 * - Request logging (development)
 *
 * @see https://kit.svelte.dev/docs/hooks
 * @module hooks.server
 * @version 1.0.0
 */

import type { Handle } from '@sveltejs/kit';
import type { ConsentState } from '$lib/consent/types';
import { CONSENT_SCHEMA_VERSION, DEFAULT_CONSENT_STATE } from '$lib/consent/types';

/**
 * Cookie name for consent storage (must match client-side).
 */
const CONSENT_COOKIE_NAME = 'rtp_consent';

/**
 * Parse and validate consent from a cookie value.
 */
function parseConsentCookie(cookieValue: string | undefined): ConsentState | null {
	if (!cookieValue) return null;

	try {
		const decoded = decodeURIComponent(cookieValue);
		const parsed = JSON.parse(decoded);

		// Validate structure
		if (
			typeof parsed !== 'object' ||
			typeof parsed.necessary !== 'boolean' ||
			typeof parsed.analytics !== 'boolean' ||
			typeof parsed.marketing !== 'boolean' ||
			typeof parsed.preferences !== 'boolean'
		) {
			return null;
		}

		// Return validated consent state
		return {
			necessary: true, // Always true
			analytics: Boolean(parsed.analytics),
			marketing: Boolean(parsed.marketing),
			preferences: Boolean(parsed.preferences),
			updatedAt: parsed.updatedAt || new Date().toISOString(),
			hasInteracted: Boolean(parsed.hasInteracted),
			version: parsed.version || CONSENT_SCHEMA_VERSION,
		};
	} catch {
		return null;
	}
}

/**
 * Main server hook handler.
 */
export const handle: Handle = async ({ event, resolve }) => {
	// Parse consent cookie for SSR
	const cookieHeader = event.request.headers.get('cookie') || '';
	const cookies = Object.fromEntries(
		cookieHeader.split(';').map((cookie) => {
			const [key, ...valueParts] = cookie.trim().split('=');
			return [key, valueParts.join('=')];
		})
	);

	const consentCookie = cookies[CONSENT_COOKIE_NAME];
	const consent = parseConsentCookie(consentCookie);

	// Store consent in locals for access in load functions
	event.locals.consent = consent || { ...DEFAULT_CONSENT_STATE };
	event.locals.hasConsentInteraction = consent?.hasInteracted ?? false;

	// Resolve the request
	const response = await resolve(event, {
		// Transform the HTML to inject consent mode defaults
		// This ensures Google Consent Mode is set before any scripts load
		transformPageChunk: ({ html }) => {
			// Only inject if consent hasn't been given yet
			// This sets restrictive defaults that will be updated client-side
			const consentDefaults = consent || DEFAULT_CONSENT_STATE;

			const consentModeScript = `
<script>
// Google Consent Mode v2 - Server-rendered defaults
window.dataLayer = window.dataLayer || [];
function gtag(){dataLayer.push(arguments);}
gtag('consent', 'default', {
  'ad_storage': '${consentDefaults.marketing ? 'granted' : 'denied'}',
  'analytics_storage': '${consentDefaults.analytics ? 'granted' : 'denied'}',
  'ad_user_data': '${consentDefaults.marketing ? 'granted' : 'denied'}',
  'ad_personalization': '${consentDefaults.marketing ? 'granted' : 'denied'}',
  'functionality_storage': '${consentDefaults.preferences ? 'granted' : 'denied'}',
  'personalization_storage': '${consentDefaults.preferences ? 'granted' : 'denied'}',
  'security_storage': 'granted'
});
</script>`;

			// Inject the consent mode script at the very beginning of <head>
			return html.replace('<head>', `<head>${consentModeScript}`);
		},
	});

	return response;
};
