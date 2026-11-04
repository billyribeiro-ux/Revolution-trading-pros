import { expect, it } from 'vite-plus/test';
import { appendStaticNoindexHeaders } from '../cloudflare-headers.js';
import { STATIC_NOINDEX_HOST_PATTERNS } from '../../seo/config.js';

it('preserves asset cache headers and adds preview host and exact static noindex rules', () => {
	const existing = '/_app/immutable/*\n  Cache-Control: public, immutable\n';
	const result = appendStaticNoindexHeaders(existing, [
		'/about',
		'/login',
		'/login/__data.json',
		'/popup-demo'
	]);
	expect(result).toContain(existing);
	for (const path of [
		...STATIC_NOINDEX_HOST_PATTERNS,
		'/login',
		'/login/__data.json',
		'/popup-demo'
	])
		expect(result).toContain(`${path}\n  X-Robots-Tag: noindex, nofollow`);
	expect(result).not.toContain('\n/about\n');
	expect(result).not.toContain('\n/*\n');
});

it('uses route segment boundaries and supports an application base path', () => {
	const result = appendStaticNoindexHeaders(
		'',
		['/portal/login', '/portal/login-extra', '/portal/about'],
		'/portal'
	);
	expect(result).toContain('\n/portal/login\n');
	expect(result).not.toContain('\n/portal/login-extra\n');
	expect(result).not.toContain('\n/portal/about\n');
});

it('rejects invalid or over-capacity header files instead of silently dropping rules', () => {
	expect(() => appendStaticNoindexHeaders('', ['/login/path\ninjected'])).toThrow('Invalid');
	expect(() =>
		appendStaticNoindexHeaders(
			'',
			Array.from({ length: 100 }, (_, i) => `/login/${i}`)
		)
	).toThrow('budget');
	expect(() => appendStaticNoindexHeaders('/file\n  X-Test: ' + 'a'.repeat(2000), [])).toThrow(
		'2000'
	);
});
