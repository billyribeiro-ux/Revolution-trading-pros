/**
 * SEO Robots Engine Tests
 * ═══════════════════════════════════════════════════════════════════════════════
 */

import { describe, it, expect } from 'vitest';
import { buildRobots, directivesToString, parseRobotsString } from '../robots';
import { seoDefaults } from '../defaults';
import type { RouteSEOContext } from '../types';

function makeContext(overrides: Partial<RouteSEOContext> = {}): RouteSEOContext {
	return {
		pathname: '/',
		params: {},
		env: 'production',
		isPrivate: false,
		isSearchPage: false,
		isErrorPage: false,
		locale: 'en',
		...overrides
	};
}

describe('buildRobots', () => {
	it('production public page: index,follow by default', () => {
		const ctx = makeContext({ pathname: '/about', env: 'production' });
		const { directives, content } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(true);
		expect(directives.follow).toBe(true);
		expect(content).toContain('index');
		expect(content).toContain('follow');
	});

	it('production public page includes advanced directives from defaults', () => {
		const ctx = makeContext({ pathname: '/blog', env: 'production' });
		const { content } = buildRobots(ctx, undefined, seoDefaults);
		expect(content).toContain('max-snippet:-1');
		expect(content).toContain('max-image-preview:large');
		expect(content).toContain('max-video-preview:-1');
	});

	it('development env: noindex,nofollow by default', () => {
		const ctx = makeContext({ pathname: '/about', env: 'development' });
		const { directives } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.follow).toBe(false);
	});

	it('staging env: noindex,nofollow by default', () => {
		const ctx = makeContext({ pathname: '/about', env: 'staging' });
		const { directives } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.follow).toBe(false);
	});

	it('private path prefix: noindex,nofollow', () => {
		const ctx = makeContext({ pathname: '/account/settings', env: 'production' });
		const { directives } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.follow).toBe(false);
	});

	it('admin path: noindex,nofollow', () => {
		const ctx = makeContext({ pathname: '/admin/users', env: 'production' });
		const { directives } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.follow).toBe(false);
	});

	it('checkout path: noindex,nofollow', () => {
		const ctx = makeContext({ pathname: '/checkout/payment', env: 'production' });
		const { directives } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.follow).toBe(false);
	});

	it('dashboard path: noindex,nofollow', () => {
		const ctx = makeContext({ pathname: '/dashboard/overview', env: 'production' });
		const { directives } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.follow).toBe(false);
	});

	it('isPrivate flag: noindex,nofollow', () => {
		const ctx = makeContext({ pathname: '/custom-private', isPrivate: true, env: 'production' });
		const { directives } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.follow).toBe(false);
	});

	it('isSearchPage flag: noindex,nofollow', () => {
		const ctx = makeContext({ pathname: '/search', isSearchPage: true, env: 'production' });
		const { directives } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.follow).toBe(false);
	});

	it('isErrorPage flag: noindex,nofollow', () => {
		const ctx = makeContext({ pathname: '/404', isErrorPage: true, env: 'production' });
		const { directives } = buildRobots(ctx, undefined, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.follow).toBe(false);
	});

	it('explicit override on production public page', () => {
		const ctx = makeContext({ pathname: '/about', env: 'production' });
		const { directives } = buildRobots(ctx, { noarchive: true }, seoDefaults);
		expect(directives.index).toBe(true);
		expect(directives.noarchive).toBe(true);
	});

	it('ignores index:true override on non-production (safety guard)', () => {
		const ctx = makeContext({ pathname: '/about', env: 'development' });
		const { directives } = buildRobots(ctx, { index: true }, seoDefaults);
		expect(directives.index).toBe(false);
	});

	it('ignores index:true override on private pages', () => {
		const ctx = makeContext({ pathname: '/account', env: 'production' });
		const { directives } = buildRobots(ctx, { index: true }, seoDefaults);
		expect(directives.index).toBe(false);
	});

	it('applies non-index overrides on private pages', () => {
		const ctx = makeContext({ pathname: '/account', env: 'production' });
		const { directives } = buildRobots(ctx, { noarchive: true }, seoDefaults);
		expect(directives.index).toBe(false);
		expect(directives.noarchive).toBe(true);
	});
});

describe('directivesToString', () => {
	it('renders index,follow', () => {
		expect(directivesToString({ index: true, follow: true })).toBe('index, follow');
	});

	it('renders noindex,nofollow', () => {
		expect(directivesToString({ index: false, follow: false })).toBe('noindex, nofollow');
	});

	it('renders advanced directives', () => {
		const result = directivesToString({
			index: true,
			follow: true,
			noarchive: true,
			'max-snippet': -1,
			'max-image-preview': 'large',
			'max-video-preview': -1
		});
		expect(result).toContain('index');
		expect(result).toContain('follow');
		expect(result).toContain('noarchive');
		expect(result).toContain('max-snippet:-1');
		expect(result).toContain('max-image-preview:large');
		expect(result).toContain('max-video-preview:-1');
	});

	it('omits unset directives', () => {
		const result = directivesToString({ index: true });
		expect(result).toBe('index');
		expect(result).not.toContain('follow');
	});
});

describe('parseRobotsString', () => {
	it('parses index,follow', () => {
		const result = parseRobotsString('index, follow');
		expect(result.index).toBe(true);
		expect(result.follow).toBe(true);
	});

	it('parses noindex,nofollow', () => {
		const result = parseRobotsString('noindex, nofollow');
		expect(result.index).toBe(false);
		expect(result.follow).toBe(false);
	});

	it('parses advanced directives', () => {
		const result = parseRobotsString('index, follow, max-snippet:-1, max-image-preview:large');
		expect(result.index).toBe(true);
		expect(result['max-snippet']).toBe(-1);
		expect(result['max-image-preview']).toBe('large');
	});

	it('roundtrips through directivesToString', () => {
		const original = { index: true, follow: true, noarchive: true, 'max-snippet': -1 as const };
		const str = directivesToString(original);
		const parsed = parseRobotsString(str);
		expect(parsed.index).toBe(original.index);
		expect(parsed.follow).toBe(original.follow);
		expect(parsed.noarchive).toBe(original.noarchive);
		expect(parsed['max-snippet']).toBe(original['max-snippet']);
	});
});
