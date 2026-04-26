/**
 * SEO Merge Engine Tests
 * ═══════════════════════════════════════════════════════════════════════════════
 */

import { describe, it, expect } from 'vitest';
import { mergeSEO, defaultsToInput } from '../merge';
import { seoDefaults } from '../defaults';
import type { SEOInput } from '../types';

describe('mergeSEO', () => {
	it('returns empty object when no layers provided', () => {
		const result = mergeSEO();
		expect(result).toEqual({});
	});

	it('returns first layer when only one provided', () => {
		const input: SEOInput = { title: 'Hello' };
		const result = mergeSEO(input);
		expect(result.title).toBe('Hello');
	});

	it('later layers override earlier scalar values', () => {
		const a: SEOInput = { title: 'First', description: 'Desc A' };
		const b: SEOInput = { title: 'Second' };
		const result = mergeSEO(a, b);
		expect(result.title).toBe('Second');
		expect(result.description).toBe('Desc A');
	});

	it('null explicitly removes inherited value', () => {
		const a: SEOInput = { title: 'First', description: 'Desc A' };
		const b: SEOInput = { title: null };
		const result = mergeSEO(a, b);
		expect(result.title).toBeNull();
		expect(result.description).toBe('Desc A');
	});

	it('undefined preserves inherited value', () => {
		const a: SEOInput = { title: 'First' };
		const b: SEOInput = { description: 'New desc' };
		const result = mergeSEO(a, b);
		expect(result.title).toBe('First');
		expect(result.description).toBe('New desc');
	});

	it('deep merges OG object', () => {
		const a: SEOInput = {
			og: { title: 'OG Title', description: 'OG Desc', type: 'website' }
		};
		const b: SEOInput = {
			og: { title: 'New OG Title' }
		};
		const result = mergeSEO(a, b);
		expect(result.og?.title).toBe('New OG Title');
		expect(result.og?.description).toBe('OG Desc');
		expect(result.og?.type).toBe('website');
	});

	it('null in OG field removes that field', () => {
		const a: SEOInput = {
			og: { title: 'OG Title', description: 'OG Desc' }
		};
		const b: SEOInput = {
			og: { description: null }
		};
		const result = mergeSEO(a, b);
		expect(result.og?.title).toBe('OG Title');
		expect(result.og?.description).toBeUndefined();
	});

	it('null OG removes entire OG object', () => {
		const a: SEOInput = {
			og: { title: 'OG Title' }
		};
		const b: SEOInput = { og: null };
		const result = mergeSEO(a, b);
		expect(result.og).toBeUndefined();
	});

	it('deduplicates alternates by hreflang+href', () => {
		const a: SEOInput = {
			alternates: [
				{ hreflang: 'en', href: 'https://example.com/en' },
				{ hreflang: 'fr', href: 'https://example.com/fr' }
			]
		};
		const b: SEOInput = {
			alternates: [{ hreflang: 'en', href: 'https://example.com/en-updated' }]
		};
		const result = mergeSEO(a, b);
		expect(result.alternates).toHaveLength(3);
	});

	it('deduplicates JSON-LD by @id', () => {
		const a: SEOInput = {
			jsonld: [{ '@type': 'Organization', '@id': '#org', name: 'Old', url: 'https://old.com' }]
		};
		const b: SEOInput = {
			jsonld: [{ '@type': 'Organization', '@id': '#org', name: 'New', url: 'https://new.com' }]
		};
		const result = mergeSEO(a, b);
		expect(result.jsonld).toHaveLength(1);
		expect((result.jsonld![0] as Record<string, unknown>).name).toBe('New');
	});

	it('null jsonld removes all JSON-LD', () => {
		const a: SEOInput = {
			jsonld: [{ '@type': 'Organization', name: 'Test', url: 'https://test.com' }]
		};
		const b: SEOInput = { jsonld: null };
		const result = mergeSEO(a, b);
		expect(result.jsonld).toBeNull();
	});

	it('skips null/undefined layers', () => {
		const a: SEOInput = { title: 'First' };
		const result = mergeSEO(a, null, undefined, { description: 'Desc' });
		expect(result.title).toBe('First');
		expect(result.description).toBe('Desc');
	});

	it('three-layer merge: defaults → layout → page', () => {
		const defaults: SEOInput = {
			title: 'Default',
			description: 'Default desc',
			robots: { index: true, follow: true }
		};
		const layout: SEOInput = {
			titleTemplate: '%s | Blog'
		};
		const page: SEOInput = {
			title: 'My Post',
			description: 'Post desc'
		};
		const result = mergeSEO(defaults, layout, page);
		expect(result.title).toBe('My Post');
		expect(result.description).toBe('Post desc');
		expect(result.titleTemplate).toBe('%s | Blog');
		expect(result.robots?.index).toBe(true);
	});
});

describe('defaultsToInput', () => {
	it('converts SEODefaults to SEOInput', () => {
		const input = defaultsToInput(seoDefaults);
		expect(input.title).toBe(seoDefaults.defaultTitle);
		expect(input.description).toBe(seoDefaults.defaultDescription);
		expect(input.og?.title).toBe(seoDefaults.defaultTitle);
		expect(input.twitter?.card).toBe(seoDefaults.twitterCard);
		expect(input.jsonld).toEqual(seoDefaults.jsonld);
	});
});
