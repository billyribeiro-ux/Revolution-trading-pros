/**
 * SEO Canonical Engine Tests
 * ═══════════════════════════════════════════════════════════════════════════════
 */

import { describe, it, expect } from 'vitest';
import { normalizeCanonical } from '../canonical';
import type { CanonicalConfig } from '../types';

const baseConfig: CanonicalConfig = {
	siteUrl: 'https://revolution-trading-pros.pages.dev',
	forceHttps: true,
	trailingSlash: 'never',
	queryParamAllowlist: ['page'],
	queryParamDenylist: ['utm_source', 'utm_medium', 'utm_campaign', 'gclid', 'fbclid']
};

describe('normalizeCanonical', () => {
	it('converts relative path to absolute URL', () => {
		const result = normalizeCanonical('/blog/my-post', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/blog/my-post');
	});

	it('forces HTTPS', () => {
		const result = normalizeCanonical('http://revolution-trading-pros.pages.dev/about', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/about');
	});

	it('lowercases hostname', () => {
		const result = normalizeCanonical('https://Revolution-Trading-Pros.Pages.Dev/about', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/about');
	});

	it('strips hash fragments', () => {
		const result = normalizeCanonical('/blog/post#comments', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/blog/post');
	});

	it('removes tracking params (utm_source)', () => {
		const result = normalizeCanonical('/blog?utm_source=twitter&utm_medium=social', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/blog');
	});

	it('removes gclid param', () => {
		const result = normalizeCanonical('/pricing?gclid=abc123', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/pricing');
	});

	it('removes fbclid param', () => {
		const result = normalizeCanonical('/pricing?fbclid=xyz789', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/pricing');
	});

	it('preserves allowlisted query params (page)', () => {
		const result = normalizeCanonical('/blog?page=2', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/blog?page=2');
	});

	it('preserves allowlisted and removes denylisted params together', () => {
		const result = normalizeCanonical('/blog?page=3&utm_source=google&gclid=abc', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/blog?page=3');
	});

	it('removes default denylisted params (_ga, _gl, etc.)', () => {
		const result = normalizeCanonical('/about?_ga=123&_gl=456', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/about');
	});

	it('normalizes duplicate slashes', () => {
		const result = normalizeCanonical('/blog//my-post///here', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/blog/my-post/here');
	});

	it('trailing slash policy: never — strips trailing slash', () => {
		const result = normalizeCanonical('/blog/', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/blog');
	});

	it('trailing slash policy: never — preserves root slash', () => {
		const result = normalizeCanonical('/', baseConfig);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/');
	});

	it('trailing slash policy: always — adds trailing slash', () => {
		const config: CanonicalConfig = { ...baseConfig, trailingSlash: 'always' };
		const result = normalizeCanonical('/blog', config);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/blog/');
	});

	it('trailing slash policy: ignore — leaves as-is', () => {
		const config: CanonicalConfig = { ...baseConfig, trailingSlash: 'ignore' };
		const withSlash = normalizeCanonical('/blog/', config);
		const withoutSlash = normalizeCanonical('/blog', config);
		expect(withSlash).toBe('https://revolution-trading-pros.pages.dev/blog/');
		expect(withoutSlash).toBe('https://revolution-trading-pros.pages.dev/blog');
	});

	it('handles already-absolute URLs', () => {
		const result = normalizeCanonical(
			'https://revolution-trading-pros.pages.dev/about',
			baseConfig
		);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/about');
	});

	it('handles malformed input gracefully', () => {
		const result = normalizeCanonical('blog/post', baseConfig);
		expect(result).toContain('https://revolution-trading-pros.pages.dev');
		expect(result).toContain('blog/post');
	});

	it('sorts query params deterministically', () => {
		const config: CanonicalConfig = {
			...baseConfig,
			queryParamAllowlist: ['page', 'sort']
		};
		const result = normalizeCanonical('/blog?sort=date&page=2', config);
		expect(result).toBe('https://revolution-trading-pros.pages.dev/blog?page=2&sort=date');
	});
});
