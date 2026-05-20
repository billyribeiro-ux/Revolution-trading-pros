/**
 * webhookSecurity — Security-Critical Unit Tests (R25-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * This module enforces two security boundaries:
 *
 *   1. SECRET GENERATION — uses `crypto.getRandomValues` (CSPRNG). A
 *      refactor that swaps in `Math.random()` (predictable seed) would
 *      let an attacker forge HMAC signatures on outbound webhooks. We
 *      defend by:
 *        a) Spying on `crypto.getRandomValues` and asserting it is
 *           called with a Uint8Array of the requested length.
 *        b) Asserting the output is base64url (no '+', '/', or '='
 *           characters survive the post-process).
 *
 *   2. SSRF GUARD — `isPrivateOrLoopbackHost` must return true for
 *      every RFC1918 / loopback / link-local / CGNAT / multicast /
 *      reserved range. Missing any one of these lets a customer
 *      configure a webhook pointing at AWS metadata (169.254.169.254),
 *      Docker bridges (172.17.x.x), or kube-internal IPs (10.x.x.x),
 *      which would then be dispatched by the backend → SSRF.
 *      This is a backend-mirrored check; we test every documented bucket.
 *
 *   3. URL VALIDATION — composition of (1) parse + (2) https-only +
 *      (3) hostname → SSRF guard. Each branch's error code is part
 *      of the public contract (the UI maps `error` strings to
 *      localized messages).
 *
 * NOTE on prod-branch coverage: in jsdom, `crypto.getRandomValues` is
 * implemented by Node's webcrypto; we exercise the real CSPRNG and ALSO
 * spy on it to verify the call shape. The "crypto unavailable" branch is
 * exercised by temporarily deleting `globalThis.crypto`.
 */

import { afterEach, describe, expect, it, vi } from 'vitest';
import {
	generateWebhookSecret,
	isPrivateOrLoopbackHost,
	isValidWebhookUrl,
	validateWebhookUrl
} from '../webhookSecurity';

// ═══════════════════════════════════════════════════════════════════════════
// generateWebhookSecret — CSPRNG + base64url shape
// ═══════════════════════════════════════════════════════════════════════════

describe('generateWebhookSecret', () => {
	afterEach(() => {
		vi.restoreAllMocks();
	});

	it('calls crypto.getRandomValues with a Uint8Array of the requested byte length', () => {
		const spy = vi.spyOn(crypto, 'getRandomValues');
		generateWebhookSecret(32);
		expect(spy).toHaveBeenCalledTimes(1);
		const arg = spy.mock.calls[0][0];
		expect(arg).toBeInstanceOf(Uint8Array);
		expect((arg as Uint8Array).length).toBe(32);
	});

	it('defaults to 32 bytes when no argument is passed', () => {
		const spy = vi.spyOn(crypto, 'getRandomValues');
		generateWebhookSecret();
		const arg = spy.mock.calls[0][0] as Uint8Array;
		expect(arg.length).toBe(32);
	});

	it('returns a base64url string (no +, /, or = padding survives)', () => {
		// 100 samples — flush out any post-process branch that forgot to
		// strip a character. base64url alphabet is [A-Za-z0-9_-].
		for (let i = 0; i < 100; i++) {
			const s = generateWebhookSecret(32);
			expect(s).toMatch(/^[A-Za-z0-9_-]+$/);
			expect(s).not.toContain('+');
			expect(s).not.toContain('/');
			expect(s).not.toContain('=');
		}
	});

	it('produces distinct values across two calls (CSPRNG is not seeded constant)', () => {
		const a = generateWebhookSecret(32);
		const b = generateWebhookSecret(32);
		expect(a).not.toBe(b);
	});

	it('throws a clear error when crypto is undefined (Web Crypto unavailable)', () => {
		const realCrypto = globalThis.crypto;
		// @ts-expect-error — intentionally erasing for the test
		delete globalThis.crypto;
		try {
			expect(() => generateWebhookSecret(32)).toThrow(/Web Crypto API is unavailable/i);
		} finally {
			globalThis.crypto = realCrypto;
		}
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// isPrivateOrLoopbackHost — SSRF bucket coverage
// ═══════════════════════════════════════════════════════════════════════════

describe('isPrivateOrLoopbackHost — loopback hostnames', () => {
	it('blocks bare localhost', () => {
		expect(isPrivateOrLoopbackHost('localhost')).toBe(true);
	});

	it('blocks *.localhost (e.g. api.localhost)', () => {
		expect(isPrivateOrLoopbackHost('api.localhost')).toBe(true);
	});

	it('blocks the case-variant LOCALHOST (lowercased internally)', () => {
		expect(isPrivateOrLoopbackHost('LOCALHOST')).toBe(true);
	});

	it('blocks empty / whitespace-only hostnames (fail-closed)', () => {
		expect(isPrivateOrLoopbackHost('')).toBe(true);
		expect(isPrivateOrLoopbackHost('   ')).toBe(true);
	});
});

describe('isPrivateOrLoopbackHost — IPv6', () => {
	it('blocks ::1 loopback', () => {
		expect(isPrivateOrLoopbackHost('::1')).toBe(true);
		expect(isPrivateOrLoopbackHost('[::1]')).toBe(true);
	});

	it('blocks fe80:: link-local', () => {
		expect(isPrivateOrLoopbackHost('fe80::1')).toBe(true);
	});

	it('blocks fc00::/7 unique-local prefix (fc / fd)', () => {
		expect(isPrivateOrLoopbackHost('fc00::1')).toBe(true);
		expect(isPrivateOrLoopbackHost('fd00::1')).toBe(true);
	});
});

describe('isPrivateOrLoopbackHost — IPv4 RFC1918 / reserved', () => {
	it('blocks 127.0.0.0/8 loopback', () => {
		expect(isPrivateOrLoopbackHost('127.0.0.1')).toBe(true);
		expect(isPrivateOrLoopbackHost('127.255.255.255')).toBe(true);
	});

	it('blocks 10.0.0.0/8 private', () => {
		expect(isPrivateOrLoopbackHost('10.0.0.1')).toBe(true);
		expect(isPrivateOrLoopbackHost('10.255.255.255')).toBe(true);
	});

	it('blocks 172.16.0.0/12 private (16-31, NOT 15 or 32)', () => {
		expect(isPrivateOrLoopbackHost('172.16.0.1')).toBe(true);
		expect(isPrivateOrLoopbackHost('172.31.255.255')).toBe(true);
		// Boundary check — 172.15.x.x is public, 172.32.x.x is public.
		expect(isPrivateOrLoopbackHost('172.15.0.1')).toBe(false);
		expect(isPrivateOrLoopbackHost('172.32.0.1')).toBe(false);
	});

	it('blocks 192.168.0.0/16 private', () => {
		expect(isPrivateOrLoopbackHost('192.168.0.1')).toBe(true);
		expect(isPrivateOrLoopbackHost('192.168.255.255')).toBe(true);
	});

	it('blocks 169.254.0.0/16 link-local (includes AWS IMDS 169.254.169.254)', () => {
		// THIS is the SSRF nightmare scenario — AWS instance metadata at
		// 169.254.169.254 is the most common token-exfil vector. Pin it.
		expect(isPrivateOrLoopbackHost('169.254.169.254')).toBe(true);
	});

	it('blocks 0.0.0.0/8 ("this host") range', () => {
		expect(isPrivateOrLoopbackHost('0.0.0.0')).toBe(true);
		expect(isPrivateOrLoopbackHost('0.1.2.3')).toBe(true);
	});

	it('blocks 100.64.0.0/10 CGNAT range', () => {
		expect(isPrivateOrLoopbackHost('100.64.0.1')).toBe(true);
		expect(isPrivateOrLoopbackHost('100.127.255.255')).toBe(true);
		// 100.63.x.x is public, 100.128.x.x is public.
		expect(isPrivateOrLoopbackHost('100.63.0.1')).toBe(false);
		expect(isPrivateOrLoopbackHost('100.128.0.1')).toBe(false);
	});

	it('blocks 224.0.0.0/4 multicast', () => {
		expect(isPrivateOrLoopbackHost('224.0.0.1')).toBe(true);
		expect(isPrivateOrLoopbackHost('239.255.255.255')).toBe(true);
	});

	it('blocks 240.0.0.0/4 reserved', () => {
		expect(isPrivateOrLoopbackHost('240.0.0.1')).toBe(true);
		expect(isPrivateOrLoopbackHost('255.255.255.255')).toBe(true);
	});

	it('blocks invalid IPv4 with out-of-range octets (>255)', () => {
		// `parseInt('999')` → 999, the documented contract treats any
		// 999.x.x.x as invalid → fail-closed → return true.
		expect(isPrivateOrLoopbackHost('999.1.1.1')).toBe(true);
	});

	it('ALLOWS public IPv4 (e.g. Cloudflare DNS, Google DNS)', () => {
		expect(isPrivateOrLoopbackHost('1.1.1.1')).toBe(false);
		expect(isPrivateOrLoopbackHost('8.8.8.8')).toBe(false);
		expect(isPrivateOrLoopbackHost('52.84.0.1')).toBe(false); // AWS public IP
	});

	it('ALLOWS public hostnames', () => {
		expect(isPrivateOrLoopbackHost('example.com')).toBe(false);
		expect(isPrivateOrLoopbackHost('api.stripe.com')).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// validateWebhookUrl — composition + error-code contract
// ═══════════════════════════════════════════════════════════════════════════

describe('validateWebhookUrl', () => {
	it('accepts a public https URL', () => {
		expect(validateWebhookUrl('https://hooks.example.com/incoming')).toEqual({ ok: true });
	});

	it('rejects malformed URLs with error=invalid_url', () => {
		const r = validateWebhookUrl('not a url');
		expect(r.ok).toBe(false);
		expect(r.error).toBe('invalid_url');
		expect(r.message).toBeTruthy();
	});

	it('rejects http:// URLs with error=must_be_https', () => {
		// HTTP must be rejected even for public hosts — secrets get sent
		// in clear text otherwise.
		const r = validateWebhookUrl('http://hooks.example.com/incoming');
		expect(r.ok).toBe(false);
		expect(r.error).toBe('must_be_https');
	});

	it('rejects ftp:// / file:// schemes', () => {
		expect(validateWebhookUrl('ftp://example.com/').error).toBe('must_be_https');
		// file:// has empty hostname → URL ctor accepts it; we get
		// must_be_https first (protocol check runs before host check).
		expect(validateWebhookUrl('file:///etc/passwd').error).toBe('must_be_https');
	});

	it('rejects loopback hostnames with error=private_or_loopback', () => {
		const r = validateWebhookUrl('https://localhost:8080/hook');
		expect(r.ok).toBe(false);
		expect(r.error).toBe('private_or_loopback');
	});

	it('rejects RFC1918 IPv4 with error=private_or_loopback', () => {
		expect(validateWebhookUrl('https://10.0.0.1/hook').error).toBe('private_or_loopback');
		expect(validateWebhookUrl('https://192.168.1.1/hook').error).toBe('private_or_loopback');
	});

	it('rejects AWS metadata IP', () => {
		const r = validateWebhookUrl('https://169.254.169.254/latest/meta-data/');
		expect(r.ok).toBe(false);
		expect(r.error).toBe('private_or_loopback');
	});

	it('isValidWebhookUrl is the boolean shorthand', () => {
		expect(isValidWebhookUrl('https://example.com/')).toBe(true);
		expect(isValidWebhookUrl('http://example.com/')).toBe(false);
		expect(isValidWebhookUrl('https://localhost/')).toBe(false);
	});
});
