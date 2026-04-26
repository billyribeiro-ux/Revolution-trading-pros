/**
 * SEO Resolve Pipeline Tests
 * ═══════════════════════════════════════════════════════════════════════════════
 */

import { describe, it, expect } from 'vitest';
import { resolveSEO, createSEOContext } from '../resolve';
import { seoDefaults } from '../defaults';
import type { SEOInput, RouteSEOContext } from '../types';

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

describe('resolveSEO', () => {
	it('returns defaults when no overrides provided', () => {
		const ctx = makeContext();
		const result = resolveSEO(ctx, seoDefaults);
		expect(result.title).toBe(seoDefaults.defaultTitle);
		expect(result.description).toBe(seoDefaults.defaultDescription);
		expect(result.canonical).toContain(seoDefaults.siteUrl);
		expect(result.og.title).toBe(seoDefaults.defaultTitle);
		expect(result.twitter.card).toBe(seoDefaults.twitterCard);
	});

	it('applies title template for non-default titles', () => {
		const ctx = makeContext({ pathname: '/about' });
		const seo: SEOInput = { title: 'About Us' };
		const result = resolveSEO(ctx, seoDefaults, seo);
		expect(result.title).toBe('About Us | Revolution Trading Pros');
	});

	it('does NOT template the default title (homepage)', () => {
		const ctx = makeContext();
		const result = resolveSEO(ctx, seoDefaults);
		expect(result.title).toBe(seoDefaults.defaultTitle);
		expect(result.title).not.toContain('| Revolution Trading Pros | Revolution Trading Pros');
	});

	it('page overrides win over defaults', () => {
		const ctx = makeContext({ pathname: '/blog/my-post' });
		const seo: SEOInput = {
			title: 'My Blog Post',
			description: 'A great post about trading'
		};
		const result = resolveSEO(ctx, seoDefaults, seo);
		expect(result.title).toContain('My Blog Post');
		expect(result.description).toBe('A great post about trading');
	});

	it('canonical is normalized from pathname', () => {
		const ctx = makeContext({ pathname: '/blog/my-post' });
		const result = resolveSEO(ctx, seoDefaults);
		expect(result.canonical).toBe('https://revolution-trading-pros.pages.dev/blog/my-post');
	});

	it('explicit canonical override is normalized', () => {
		const ctx = makeContext({ pathname: '/blog/my-post' });
		const seo: SEOInput = {
			canonical: 'https://revolution-trading-pros.pages.dev/blog/my-post?utm_source=twitter'
		};
		const result = resolveSEO(ctx, seoDefaults, seo);
		expect(result.canonical).not.toContain('utm_source');
	});

	it('robots: production public page is indexable', () => {
		const ctx = makeContext({ pathname: '/about', env: 'production' });
		const result = resolveSEO(ctx, seoDefaults);
		expect(result.robotsContent).toContain('index');
		expect(result.robotsContent).toContain('follow');
		expect(result.robotsDirectives.index).toBe(true);
	});

	it('robots: private page is noindex', () => {
		const ctx = makeContext({ pathname: '/account/settings', env: 'production' });
		const result = resolveSEO(ctx, seoDefaults);
		expect(result.robotsContent).toContain('noindex');
		expect(result.robotsDirectives.index).toBe(false);
	});

	it('robots: dev env is noindex', () => {
		const ctx = makeContext({ pathname: '/about', env: 'development' });
		const result = resolveSEO(ctx, seoDefaults);
		expect(result.robotsContent).toContain('noindex');
	});

	it('OG inherits from resolved title/description', () => {
		const ctx = makeContext({ pathname: '/about' });
		const seo: SEOInput = { title: 'About', description: 'About page' };
		const result = resolveSEO(ctx, seoDefaults, seo);
		expect(result.og.title).toBe('About | Revolution Trading Pros');
		expect(result.og.description).toBe('About page');
		expect(result.og.url).toBe(result.canonical);
	});

	it('OG can be explicitly overridden', () => {
		const ctx = makeContext({ pathname: '/about' });
		const seo: SEOInput = {
			title: 'About',
			og: { title: 'Custom OG Title' }
		};
		const result = resolveSEO(ctx, seoDefaults, seo);
		expect(result.og.title).toBe('Custom OG Title');
	});

	it('Twitter inherits from resolved title/description', () => {
		const ctx = makeContext({ pathname: '/about' });
		const seo: SEOInput = { title: 'About', description: 'About page' };
		const result = resolveSEO(ctx, seoDefaults, seo);
		expect(result.twitter.title).toBe('About | Revolution Trading Pros');
		expect(result.twitter.description).toBe('About page');
	});

	it('JSON-LD includes defaults when no override', () => {
		const ctx = makeContext();
		const result = resolveSEO(ctx, seoDefaults);
		expect(result.jsonld.length).toBeGreaterThan(0);
		const types = result.jsonld.map((n) => n['@type']);
		expect(types).toContain('Organization');
		expect(types).toContain('WebSite');
	});

	it('JSON-LD merges page-level nodes with defaults', () => {
		const ctx = makeContext({ pathname: '/blog/post' });
		const seo: SEOInput = {
			jsonld: [
				{
					'@type': 'Article',
					'@id': '#article',
					headline: 'Test',
					datePublished: '2026-02-08',
					author: { '@type': 'Person', name: 'Author' },
					publisher: { '@type': 'Organization', name: 'RTP' }
				}
			]
		};
		const result = resolveSEO(ctx, seoDefaults, seo);
		const types = result.jsonld.map((n) => n['@type']);
		expect(types).toContain('Organization');
		expect(types).toContain('WebSite');
		expect(types).toContain('Article');
	});

	it('alternates are preserved', () => {
		const ctx = makeContext();
		const seo: SEOInput = {
			alternates: [
				{ hreflang: 'en', href: 'https://example.com/en' },
				{ hreflang: 'es', href: 'https://example.com/es' }
			]
		};
		const result = resolveSEO(ctx, seoDefaults, seo);
		expect(result.alternates).toHaveLength(2);
	});

	it('verification tags pass through', () => {
		const ctx = makeContext();
		const seo: SEOInput = {
			verification: { google: 'abc123' }
		};
		const result = resolveSEO(ctx, seoDefaults, seo);
		expect(result.verification.google).toBe('abc123');
	});

	it('multi-layer merge: defaults → layout → page', () => {
		const ctx = makeContext({ pathname: '/blog/post' });
		const layout: SEOInput = { titleTemplate: '%s — Blog' };
		const page: SEOInput = { title: 'My Post', description: 'Post desc' };
		const result = resolveSEO(ctx, seoDefaults, layout, page);
		expect(result.title).toBe('My Post — Blog');
		expect(result.description).toBe('Post desc');
	});
});

describe('createSEOContext', () => {
	it('creates context with defaults', () => {
		const ctx = createSEOContext({ pathname: '/about' });
		expect(ctx.pathname).toBe('/about');
		expect(ctx.env).toBe('production');
		expect(ctx.isPrivate).toBe(false);
		expect(ctx.locale).toBe('en');
	});

	it('accepts all overrides', () => {
		const ctx = createSEOContext({
			pathname: '/account',
			params: { id: '123' },
			env: 'staging',
			isPrivate: true,
			isSearchPage: false,
			isErrorPage: false,
			locale: 'es'
		});
		expect(ctx.env).toBe('staging');
		expect(ctx.isPrivate).toBe(true);
		expect(ctx.params.id).toBe('123');
		expect(ctx.locale).toBe('es');
	});
});
