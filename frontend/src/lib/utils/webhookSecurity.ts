/**
 * Webhook Security Utilities
 * ==========================
 * Cryptographically secure secret generation + SSRF-aware URL validation
 * for webhook configuration UIs.
 *
 * @security Critical — secrets are used to HMAC-sign outbound payloads,
 * URLs are dispatched server-side and must not reach private/loopback hosts.
 */

/**
 * Generate a cryptographically secure webhook secret.
 *
 * Uses `crypto.getRandomValues()` (CSPRNG) — never `Math.random()`, which
 * is seeded predictably and can let an attacker forge HMAC signatures.
 *
 * @param byteLength Number of random bytes to draw (default 32 → ~43 chars base64url).
 * @returns URL-safe base64 string with no padding.
 */
export function generateWebhookSecret(byteLength: number = 32): string {
	if (typeof crypto === 'undefined' || typeof crypto.getRandomValues !== 'function') {
		throw new Error('Web Crypto API is unavailable — cannot generate a secure secret');
	}
	const bytes = new Uint8Array(byteLength);
	crypto.getRandomValues(bytes);
	let binary = '';
	for (let i = 0; i < bytes.length; i++) {
		binary += String.fromCharCode(bytes[i]);
	}
	// btoa → base64; convert to base64url and strip padding for compactness.
	return btoa(binary).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

/**
 * Hostname classifier — returns true if the hostname resolves to a private,
 * loopback, link-local, or otherwise non-public address. SSRF guard.
 *
 * Note: this is a best-effort frontend check. The backend MUST also enforce
 * its own DNS-resolution-based check at dispatch time. Bypassing the
 * frontend check by submitting a hostname that resolves to private IPs at
 * runtime (DNS rebinding) is the backend's responsibility.
 */
export function isPrivateOrLoopbackHost(hostname: string): boolean {
	const host = hostname.toLowerCase().trim();
	if (!host) return true;

	// Loopback hostnames
	if (host === 'localhost' || host.endsWith('.localhost')) return true;
	if (host === 'broadcasthost') return true;

	// IPv6 loopback / link-local / unique-local
	if (host === '::1' || host === '[::1]') return true;
	if (host.startsWith('fe80:') || host.startsWith('[fe80:')) return true;
	if (host.startsWith('fc') || host.startsWith('[fc')) return true;
	if (host.startsWith('fd') || host.startsWith('[fd')) return true;

	// IPv4 numeric checks
	const ipv4Match = host.match(/^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/);
	if (ipv4Match) {
		const [, a, b, c, d] = ipv4Match.map((n) => parseInt(n, 10));
		// Each octet must be 0-255
		if ([a, b, c, d].some((n) => n > 255 || Number.isNaN(n))) return true;

		// 127.0.0.0/8 — loopback
		if (a === 127) return true;
		// 10.0.0.0/8 — private
		if (a === 10) return true;
		// 172.16.0.0/12 — private
		if (a === 172 && b >= 16 && b <= 31) return true;
		// 192.168.0.0/16 — private
		if (a === 192 && b === 168) return true;
		// 169.254.0.0/16 — link-local (incl. AWS IMDS 169.254.169.254)
		if (a === 169 && b === 254) return true;
		// 0.0.0.0/8 — current network / "this host"
		if (a === 0) return true;
		// 100.64.0.0/10 — CGNAT
		if (a === 100 && b >= 64 && b <= 127) return true;
		// 224.0.0.0/4 — multicast
		if (a >= 224 && a <= 239) return true;
		// 240.0.0.0/4 — reserved
		if (a >= 240) return true;
	}

	return false;
}

export type WebhookUrlValidationError =
	| 'invalid_url'
	| 'must_be_https'
	| 'private_or_loopback'
	| 'invalid_host';

export interface WebhookUrlValidationResult {
	ok: boolean;
	error?: WebhookUrlValidationError;
	message?: string;
}

/**
 * Validate a webhook URL: must parse, must be `https:`, hostname must not
 * be loopback / RFC1918 / link-local. Returns `{ ok: true }` on success,
 * `{ ok: false, error, message }` otherwise.
 */
export function validateWebhookUrl(urlString: string): WebhookUrlValidationResult {
	let url: URL;
	try {
		url = new URL(urlString);
	} catch {
		return { ok: false, error: 'invalid_url', message: 'Please enter a valid URL' };
	}

	if (url.protocol !== 'https:') {
		return {
			ok: false,
			error: 'must_be_https',
			message: 'Webhook URL must start with https:// (http is not allowed)'
		};
	}

	const host = url.hostname;
	if (!host) {
		return { ok: false, error: 'invalid_host', message: 'URL is missing a hostname' };
	}

	if (isPrivateOrLoopbackHost(host)) {
		return {
			ok: false,
			error: 'private_or_loopback',
			message:
				'Webhook URL cannot point at private/loopback/link-local addresses (localhost, 10.x.x.x, 172.16-31.x.x, 192.168.x.x, 169.254.x.x)'
		};
	}

	return { ok: true };
}

/**
 * Convenience boolean wrapper matching the legacy `isValidUrl` shape used
 * across the existing webhook forms. Prefer `validateWebhookUrl` for new
 * code so you get a usable error message.
 */
export function isValidWebhookUrl(urlString: string): boolean {
	return validateWebhookUrl(urlString).ok;
}
