/**
 * structured-data — Unit Tests (R27-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/utils/structured-data.ts` is a 1942-LOC SEO module that emits
 * JSON-LD into every page's <head>. Bugs are SILENT — invalid schema.org
 * still renders but tanks Rich Results eligibility; bad date formats get
 * dropped by Google's parser without a visible error.
 *
 * These tests pin the contracts that matter most:
 *   1. `@context` is always the schema.org URL.
 *   2. `@type` matches what callers asked for (Organization, BlogPosting,
 *      Product, FAQPage, BreadcrumbList, Person, Service, VideoObject,
 *      HowTo, CollectionPage, WebPage, Course).
 *   3. `@id` follows the URL-fragment convention (`<url>/#organization`,
 *      `<url>/#article`, etc.) — the @graph cross-references depend on it.
 *   4. Required schema.org fields are populated.
 *   5. Article/BlogPosting `image` is ALWAYS coerced to an array (Google
 *      Rich Results requirement, easy to break).
 *   6. BreadcrumbList positions start at 1 and increment.
 *   7. FAQPage mainEntity is the array form (Q/A pairs) — flipping the
 *      type to a string breaks rich snippets.
 *   8. @graph helper strips the inner `@context` (single context at root
 *      is the official pattern).
 *   9. `validateStructuredData` flags missing required fields.
 *  10. `toJsonLdScript` returns JSON-parsable text (it ships in a
 *      <script> tag — broken JSON crashes Google's parser silently).
 *
 * Negative cases:
 *   - Unknown `type` throws (no silent fallback).
 *   - Empty FAQ questions array → still valid FAQPage, empty mainEntity.
 *   - validate returns errors when @context/@type are missing.
 */

import { describe, expect, it } from 'vitest';
import {
	generateStructuredData,
	generateMultipleStructuredData,
	generateGraphStructuredData,
	generateBlogPostWithFAQ,
	generateBlogPostWithHowTo,
	generateSiteStructuredData,
	generateBlogPostStructuredData,
	generateBlogIndexStructuredData,
	generateTradingServiceStructuredData,
	generateTradingTutorialStructuredData,
	generateAuthorStructuredData,
	toJsonLdScript,
	createJsonLdScriptTag,
	validateStructuredData,
	type BaseStructuredData,
	type GraphStructuredData
} from '../structured-data';

const SITE_URL = 'https://revolutiontradingpros.com';
const POST_URL = `${SITE_URL}/blog/sample-post`;
const ISO_DATE_RE = /^\d{4}-\d{2}-\d{2}T?\d{0,2}/;

// ═══════════════════════════════════════════════════════════════════════════
// generateStructuredData — Organization
// ═══════════════════════════════════════════════════════════════════════════

describe('generateStructuredData — Organization', () => {
	it('emits @context + @type + @id with the documented #organization fragment', () => {
		const data = generateStructuredData({
			type: 'Organization',
			name: 'Acme Trading',
			url: SITE_URL
		});

		expect(data['@context']).toBe('https://schema.org');
		expect(data['@type']).toBe('Organization');
		// The @id convention is load-bearing: WebSite.publisher refers to it.
		expect(data['@id']).toBe(`${SITE_URL}/#organization`);
		expect(data.name).toBe('Acme Trading');
		expect(data.url).toBe(SITE_URL);
	});

	it('wraps logo string into an ImageObject keyed by #logo', () => {
		const data = generateStructuredData({
			type: 'Organization',
			name: 'Acme',
			url: SITE_URL,
			logo: `${SITE_URL}/logo.png`
		});

		expect(data.logo).toEqual({
			'@type': 'ImageObject',
			'@id': `${SITE_URL}/#logo`,
			url: `${SITE_URL}/logo.png`,
			contentUrl: `${SITE_URL}/logo.png`,
			caption: 'Acme'
		});
	});

	it('handles numberOfEmployees as both scalar and range', () => {
		const scalar = generateStructuredData({
			type: 'Organization',
			name: 'A',
			url: SITE_URL,
			numberOfEmployees: 42
		});
		expect(scalar.numberOfEmployees).toEqual({
			'@type': 'QuantitativeValue',
			value: 42
		});

		const range = generateStructuredData({
			type: 'Organization',
			name: 'A',
			url: SITE_URL,
			numberOfEmployees: { minValue: 10, maxValue: 50 }
		});
		expect(range.numberOfEmployees).toEqual({
			'@type': 'QuantitativeValue',
			minValue: 10,
			maxValue: 50
		});
	});

	it('maps areaServed strings into Country objects (not raw strings)', () => {
		const data = generateStructuredData({
			type: 'Organization',
			name: 'A',
			url: SITE_URL,
			areaServed: ['United States', 'Canada']
		});

		expect(data.areaServed).toEqual([
			{ '@type': 'Country', name: 'United States' },
			{ '@type': 'Country', name: 'Canada' }
		]);
	});

	it('omits optional E-E-A-T fields when not provided', () => {
		const data = generateStructuredData({
			type: 'Organization',
			name: 'A',
			url: SITE_URL
		});
		// NEGATIVE: these must be absent — Google penalises empty/null values.
		expect(data.foundingDate).toBeUndefined();
		expect(data.address).toBeUndefined();
		expect(data.sameAs).toBeUndefined();
		expect(data.knowsAbout).toBeUndefined();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateStructuredData — WebSite + SearchAction
// ═══════════════════════════════════════════════════════════════════════════

describe('generateStructuredData — WebSite', () => {
	it('emits @type=WebSite with #website @id and publisher reference', () => {
		const data = generateStructuredData({
			type: 'WebSite',
			name: 'Acme',
			url: SITE_URL
		});

		expect(data['@type']).toBe('WebSite');
		expect(data['@id']).toBe(`${SITE_URL}/#website`);
		// The publisher must reference the Organization @id so @graph cross-links.
		expect(data.publisher).toEqual({ '@id': `${SITE_URL}/#organization` });
	});

	it('emits SearchAction only when both potentialAction AND searchUrl are set', () => {
		const withSearch = generateStructuredData({
			type: 'WebSite',
			name: 'A',
			url: SITE_URL,
			potentialAction: true,
			searchUrl: `${SITE_URL}/search`
		});
		expect(withSearch.potentialAction).toEqual({
			'@type': 'SearchAction',
			target: {
				'@type': 'EntryPoint',
				urlTemplate: `${SITE_URL}/search?q={search_term_string}`
			},
			'query-input': 'required name=search_term_string'
		});

		// NEGATIVE: potentialAction=true alone without searchUrl → no action emitted.
		const noUrl = generateStructuredData({
			type: 'WebSite',
			name: 'A',
			url: SITE_URL,
			potentialAction: true
		});
		expect(noUrl.potentialAction).toBeUndefined();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateStructuredData — Article / BlogPosting
// ═══════════════════════════════════════════════════════════════════════════

describe('generateStructuredData — Article/BlogPosting', () => {
	it('preserves caller-chosen @type (Article vs BlogPosting)', () => {
		const article = generateStructuredData({
			type: 'Article',
			headline: 'Headline',
			url: POST_URL,
			datePublished: '2026-01-15T08:00:00Z',
			author: { name: 'Author' }
		});
		expect(article['@type']).toBe('Article');

		const blogPost = generateStructuredData({
			type: 'BlogPosting',
			headline: 'Headline',
			url: POST_URL,
			datePublished: '2026-01-15T08:00:00Z',
			author: { name: 'Author' }
		});
		expect(blogPost['@type']).toBe('BlogPosting');
	});

	it('always coerces image to an array (Google Rich Results requirement)', () => {
		const single = generateStructuredData({
			type: 'BlogPosting',
			headline: 'H',
			url: POST_URL,
			datePublished: '2026-01-15T08:00:00Z',
			author: { name: 'A' },
			image: 'https://example.com/img.jpg'
		});
		expect(Array.isArray(single.image)).toBe(true);
		expect(single.image).toEqual(['https://example.com/img.jpg']);

		const multi = generateStructuredData({
			type: 'BlogPosting',
			headline: 'H',
			url: POST_URL,
			datePublished: '2026-01-15T08:00:00Z',
			author: { name: 'A' },
			image: ['https://example.com/a.jpg', 'https://example.com/b.jpg']
		});
		expect(multi.image).toEqual(['https://example.com/a.jpg', 'https://example.com/b.jpg']);
	});

	it('passes datePublished through unchanged in ISO 8601 format', () => {
		const iso = '2026-01-15T08:00:00Z';
		const data = generateStructuredData({
			type: 'BlogPosting',
			headline: 'H',
			url: POST_URL,
			datePublished: iso,
			author: { name: 'A' }
		});
		expect(data.datePublished).toBe(iso);
		expect(typeof data.datePublished).toBe('string');
		expect(data.datePublished).toMatch(ISO_DATE_RE);
	});

	it('joins keywords array into a comma-separated string (schema.org convention)', () => {
		const data = generateStructuredData({
			type: 'BlogPosting',
			headline: 'H',
			url: POST_URL,
			datePublished: '2026-01-15T08:00:00Z',
			author: { name: 'A' },
			keywords: ['trading', 'options', 'futures']
		});
		// CONTRACT: schema.org keywords is a string, not array.
		expect(data.keywords).toBe('trading, options, futures');
	});

	it('builds author with E-E-A-T credentials when expertise is provided', () => {
		const data = generateStructuredData({
			type: 'BlogPosting',
			headline: 'H',
			url: POST_URL,
			datePublished: '2026-01-15T08:00:00Z',
			author: {
				name: 'Expert Author',
				url: 'https://example.com/author',
				expertise: {
					knowsAbout: ['Day Trading', 'Options'],
					hasCredential: [
						{
							'@type': 'EducationalOccupationalCredential',
							name: 'CMT',
							credentialCategory: 'Professional Certification'
						}
					]
				}
			}
		});

		const author = data.author as Record<string, unknown>;
		expect(author['@type']).toBe('Person');
		expect(author.knowsAbout).toEqual(['Day Trading', 'Options']);
		expect(Array.isArray(author.hasCredential)).toBe(true);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateStructuredData — Product
// ═══════════════════════════════════════════════════════════════════════════

describe('generateStructuredData — Product', () => {
	it('emits Offer with default USD currency + InStock availability', () => {
		const data = generateStructuredData({
			type: 'Product',
			name: 'Trading Course',
			url: `${SITE_URL}/courses/trading`,
			price: 199
		});

		expect(data['@type']).toBe('Product');
		expect(data.offers).toEqual({
			'@type': 'Offer',
			price: 199,
			priceCurrency: 'USD',
			availability: 'https://schema.org/InStock',
			url: `${SITE_URL}/courses/trading`
		});
	});

	it('honours explicit priceCurrency and availability overrides', () => {
		const data = generateStructuredData({
			type: 'Product',
			name: 'Course',
			url: `${SITE_URL}/c`,
			price: 99,
			priceCurrency: 'EUR',
			availability: 'OutOfStock'
		});
		const offers = data.offers as Record<string, unknown>;
		expect(offers.priceCurrency).toBe('EUR');
		expect(offers.availability).toBe('https://schema.org/OutOfStock');
	});

	it('omits offers entirely when price is undefined (no $0 ghost offer)', () => {
		const data = generateStructuredData({
			type: 'Product',
			name: 'Course',
			url: `${SITE_URL}/c`
		});
		// NEGATIVE: silent $0 offer would mislead Google Shopping.
		expect(data.offers).toBeUndefined();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateStructuredData — FAQPage
// ═══════════════════════════════════════════════════════════════════════════

describe('generateStructuredData — FAQPage', () => {
	it('wraps Q/A pairs in Question + acceptedAnswer (Answer) shape', () => {
		const data = generateStructuredData({
			type: 'FAQPage',
			questions: [
				{ question: 'What is trading?', answer: 'Buying and selling securities.' },
				{ question: 'How do I start?', answer: 'Open a brokerage account.' }
			]
		});

		expect(data['@type']).toBe('FAQPage');
		expect(data.mainEntity).toEqual([
			{
				'@type': 'Question',
				name: 'What is trading?',
				acceptedAnswer: { '@type': 'Answer', text: 'Buying and selling securities.' }
			},
			{
				'@type': 'Question',
				name: 'How do I start?',
				acceptedAnswer: { '@type': 'Answer', text: 'Open a brokerage account.' }
			}
		]);
	});

	it('handles empty questions array without crashing (still valid FAQPage)', () => {
		const data = generateStructuredData({
			type: 'FAQPage',
			questions: []
		});
		expect(data['@type']).toBe('FAQPage');
		expect(data.mainEntity).toEqual([]);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateStructuredData — BreadcrumbList
// ═══════════════════════════════════════════════════════════════════════════

describe('generateStructuredData — BreadcrumbList', () => {
	it('emits ListItems with 1-based positions and item URLs', () => {
		const data = generateStructuredData({
			type: 'BreadcrumbList',
			items: [
				{ name: 'Home', url: SITE_URL },
				{ name: 'Blog', url: `${SITE_URL}/blog` },
				{ name: 'Post', url: POST_URL }
			]
		});

		expect(data['@type']).toBe('BreadcrumbList');
		expect(data.itemListElement).toEqual([
			{ '@type': 'ListItem', position: 1, name: 'Home', item: SITE_URL },
			{ '@type': 'ListItem', position: 2, name: 'Blog', item: `${SITE_URL}/blog` },
			{ '@type': 'ListItem', position: 3, name: 'Post', item: POST_URL }
		]);
	});

	it('handles single-item breadcrumb (edge case: position=1 only)', () => {
		const data = generateStructuredData({
			type: 'BreadcrumbList',
			items: [{ name: 'Home', url: SITE_URL }]
		});
		const items = data.itemListElement as Array<Record<string, unknown>>;
		expect(items).toHaveLength(1);
		expect(items[0].position).toBe(1);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateStructuredData — HowTo
// ═══════════════════════════════════════════════════════════════════════════

describe('generateStructuredData — HowTo', () => {
	it('numbers steps starting at position=1 with HowToStep @type', () => {
		const data = generateStructuredData({
			type: 'HowTo',
			name: 'Trade options',
			steps: [
				{ name: 'Open account', text: 'Sign up' },
				{ name: 'Fund it', text: 'Wire money' },
				{ name: 'Place trade', text: 'Buy a call' }
			]
		});

		const steps = data.step as Array<Record<string, unknown>>;
		expect(steps).toHaveLength(3);
		expect(steps[0]).toMatchObject({ '@type': 'HowToStep', position: 1, name: 'Open account' });
		expect(steps[1].position).toBe(2);
		expect(steps[2].position).toBe(3);
	});

	it('wraps single image string into array (parity with Article)', () => {
		const data = generateStructuredData({
			type: 'HowTo',
			name: 'Test',
			steps: [{ name: 's', text: 't' }],
			image: 'https://example.com/img.jpg'
		});
		expect(data.image).toEqual(['https://example.com/img.jpg']);
	});

	it('emits MonetaryAmount for estimatedCost', () => {
		const data = generateStructuredData({
			type: 'HowTo',
			name: 'T',
			steps: [{ name: 's', text: 't' }],
			estimatedCost: { currency: 'USD', value: 100 }
		});
		expect(data.estimatedCost).toEqual({
			'@type': 'MonetaryAmount',
			currency: 'USD',
			value: 100
		});
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateStructuredData — VideoObject
// ═══════════════════════════════════════════════════════════════════════════

describe('generateStructuredData — VideoObject', () => {
	it('emits required schema.org video fields (name, thumbnailUrl, uploadDate)', () => {
		const data = generateStructuredData({
			type: 'VideoObject',
			name: 'Sample',
			thumbnailUrl: 'https://example.com/thumb.jpg',
			uploadDate: '2026-01-15T08:00:00Z'
		});
		expect(data['@type']).toBe('VideoObject');
		expect(data.name).toBe('Sample');
		expect(data.thumbnailUrl).toBe('https://example.com/thumb.jpg');
		expect(data.uploadDate).toBe('2026-01-15T08:00:00Z');
	});

	it('maps hasPart clips with start/end offsets and Clip @type', () => {
		const data = generateStructuredData({
			type: 'VideoObject',
			name: 'V',
			thumbnailUrl: 'https://example.com/t.jpg',
			uploadDate: '2026-01-15T08:00:00Z',
			hasPart: [
				{ '@type': 'Clip', name: 'Intro', startOffset: 0, endOffset: 30 },
				{ '@type': 'Clip', name: 'Demo', startOffset: 30, endOffset: 120 }
			]
		});
		const clips = data.hasPart as Array<Record<string, unknown>>;
		expect(clips[0]).toMatchObject({ '@type': 'Clip', name: 'Intro', startOffset: 0, endOffset: 30 });
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateStructuredData — unknown type
// ═══════════════════════════════════════════════════════════════════════════

describe('generateStructuredData — unknown type', () => {
	it('throws when given an unknown @type (no silent fallback)', () => {
		// NEGATIVE: a silent fallback would ship invalid JSON-LD into <head>.
		expect(() =>
			generateStructuredData({
				// @ts-expect-error — intentionally invalid type
				type: 'NotAType',
				name: 'x',
				url: 'https://x'
			})
		).toThrow(/Unknown structured data type/);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateMultipleStructuredData / generateGraphStructuredData
// ═══════════════════════════════════════════════════════════════════════════

describe('generateGraphStructuredData', () => {
	it('strips inner @context from every graph member (single root @context)', () => {
		const graph = generateGraphStructuredData([
			{ type: 'Organization', name: 'O', url: SITE_URL },
			{ type: 'WebSite', name: 'W', url: SITE_URL }
		]);

		expect(graph['@context']).toBe('https://schema.org');
		expect(Array.isArray(graph['@graph'])).toBe(true);
		// CONTRACT: inner @context must be removed (schema.org @graph spec).
		for (const member of graph['@graph']) {
			expect(member['@context']).toBeUndefined();
		}
		// But @type / @id must survive intact.
		expect(graph['@graph'][0]['@type']).toBe('Organization');
		expect(graph['@graph'][1]['@type']).toBe('WebSite');
	});

	it('generateMultipleStructuredData keeps each member fully self-contained (with @context)', () => {
		const arr = generateMultipleStructuredData([
			{ type: 'Organization', name: 'O', url: SITE_URL },
			{ type: 'WebSite', name: 'W', url: SITE_URL }
		]);
		expect(arr).toHaveLength(2);
		// NOT a graph — each member retains its own @context.
		for (const member of arr) {
			expect(member['@context']).toBe('https://schema.org');
		}
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateBlogPostWithFAQ / generateBlogPostWithHowTo
// ═══════════════════════════════════════════════════════════════════════════

describe('generateBlogPostWithFAQ', () => {
	it('combines BlogPosting + FAQPage in a single @graph', () => {
		const graph = generateBlogPostWithFAQ(
			{
				type: 'BlogPosting',
				headline: 'H',
				url: POST_URL,
				datePublished: '2026-01-15T08:00:00Z',
				author: { name: 'A' }
			},
			[{ question: 'Q1', answer: 'A1' }]
		);

		expect(graph['@context']).toBe('https://schema.org');
		expect(graph['@graph']).toHaveLength(2);
		const types = graph['@graph'].map((m) => m['@type']);
		expect(types).toEqual(['BlogPosting', 'FAQPage']);
	});
});

describe('generateBlogPostWithHowTo', () => {
	it('combines BlogPosting + HowTo in a single @graph', () => {
		const graph = generateBlogPostWithHowTo(
			{
				type: 'BlogPosting',
				headline: 'H',
				url: POST_URL,
				datePublished: '2026-01-15T08:00:00Z',
				author: { name: 'A' }
			},
			{
				name: 'How to',
				steps: [{ name: 's', text: 't' }]
			}
		);
		expect(graph['@graph']).toHaveLength(2);
		expect(graph['@graph'][1]['@type']).toBe('HowTo');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// Convenience builders
// ═══════════════════════════════════════════════════════════════════════════

describe('generateSiteStructuredData', () => {
	it('produces a 2-node @graph: Organization + WebSite for the site root', () => {
		const graph = generateSiteStructuredData(SITE_URL);
		expect(graph['@graph']).toHaveLength(2);
		expect(graph['@graph'][0]['@type']).toBe('Organization');
		expect(graph['@graph'][1]['@type']).toBe('WebSite');
		// Cross-reference: WebSite.publisher must point at Organization.@id
		const website = graph['@graph'][1];
		expect((website.publisher as Record<string, unknown>)['@id']).toBe(`${SITE_URL}/#organization`);
	});
});

describe('generateBlogPostStructuredData', () => {
	it('returns plain BlogPosting when no faqContent is provided', () => {
		const data = generateBlogPostStructuredData({
			title: 'My Post',
			url: POST_URL,
			publishedAt: '2026-01-15T08:00:00Z',
			authorName: 'Author'
		}) as BaseStructuredData;
		expect(data['@type']).toBe('BlogPosting');
		expect(data['@graph']).toBeUndefined();
	});

	it('returns @graph when faqContent IS provided (BlogPosting + FAQPage)', () => {
		const data = generateBlogPostStructuredData({
			title: 'My Post',
			url: POST_URL,
			publishedAt: '2026-01-15T08:00:00Z',
			authorName: 'Author',
			faqContent: [{ question: 'Q', answer: 'A' }]
		}) as GraphStructuredData;
		expect(data['@graph']).toHaveLength(2);
	});

	it('defaults speakable to article-headline + article-summary selectors when not provided', () => {
		const data = generateBlogPostStructuredData({
			title: 'My Post',
			url: POST_URL,
			publishedAt: '2026-01-15T08:00:00Z',
			authorName: 'Author'
		}) as BaseStructuredData;
		const speakable = data.speakable as Record<string, unknown>;
		expect(speakable['@type']).toBe('SpeakableSpecification');
		expect(speakable.cssSelector).toEqual(['.article-headline', '.article-summary']);
	});
});

describe('generateTradingServiceStructuredData', () => {
	it('emits Service with USD priceCurrency by default', () => {
		const data = generateTradingServiceStructuredData({
			name: 'Live Trading Room',
			url: `${SITE_URL}/services/live-room`,
			price: 99
		});
		expect(data['@type']).toBe('Service');
		const offers = data.offers as Record<string, unknown>;
		expect(offers.price).toBe(99);
		expect(offers.priceCurrency).toBe('USD');
	});

	it('omits aggregateRating when no ratingValue provided', () => {
		const data = generateTradingServiceStructuredData({
			name: 'S',
			url: `${SITE_URL}/s`
		});
		// NEGATIVE: a default aggregateRating with 0 reviews would lie to Google.
		expect(data.aggregateRating).toBeUndefined();
	});
});

describe('generateAuthorStructuredData', () => {
	it('emits Person with worksFor=Revolution Trading Pros and knowsAbout from expertise', () => {
		const data = generateAuthorStructuredData({
			name: 'Jane Doe',
			expertise: ['Day Trading', 'Options']
		});
		expect(data['@type']).toBe('Person');
		expect(data.name).toBe('Jane Doe');
		expect(data.knowsAbout).toEqual(['Day Trading', 'Options']);
		expect(data.worksFor).toMatchObject({
			'@type': 'Organization',
			name: 'Revolution Trading Pros'
		});
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// JSON-LD serialization
// ═══════════════════════════════════════════════════════════════════════════

describe('toJsonLdScript / createJsonLdScriptTag', () => {
	it('toJsonLdScript returns JSON.parse-able text (no trailing commas, no JS sigils)', () => {
		const data = generateStructuredData({
			type: 'Organization',
			name: 'A',
			url: SITE_URL
		});
		const json = toJsonLdScript(data);
		expect(typeof json).toBe('string');
		// Round-trip parse — broken JSON would crash Google's parser silently.
		const parsed = JSON.parse(json);
		expect(parsed['@type']).toBe('Organization');
	});

	it('createJsonLdScriptTag wraps the JSON in a properly-typed <script> tag', () => {
		const data = generateStructuredData({
			type: 'Organization',
			name: 'A',
			url: SITE_URL
		});
		const tag = createJsonLdScriptTag(data);
		expect(tag.startsWith('<script type="application/ld+json">')).toBe(true);
		expect(tag.endsWith('</script>')).toBe(true);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// validateStructuredData
// ═══════════════════════════════════════════════════════════════════════════

describe('validateStructuredData', () => {
	it('returns valid=true when @context and @type are both present', () => {
		const result = validateStructuredData({
			'@context': 'https://schema.org',
			'@type': 'Organization'
		});
		expect(result).toEqual({ valid: true, errors: [] });
	});

	it('flags missing @context', () => {
		// Intentionally invalid shape — cast through unknown to bypass the
		// type guard so we can exercise validateStructuredData's runtime check.
		const result = validateStructuredData({
			'@context': undefined,
			'@type': 'Organization'
		} as unknown as BaseStructuredData);
		expect(result.valid).toBe(false);
		expect(result.errors).toContain('Missing @context');
	});

	it('flags missing @type', () => {
		const result = validateStructuredData({
			'@context': 'https://schema.org',
			'@type': undefined
		} as unknown as BaseStructuredData);
		expect(result.valid).toBe(false);
		expect(result.errors).toContain('Missing @type');
	});

	it('accumulates all errors when multiple required fields are missing', () => {
		const result = validateStructuredData({} as BaseStructuredData);
		expect(result.valid).toBe(false);
		expect(result.errors).toHaveLength(2);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateBlogIndexStructuredData / generateTradingTutorialStructuredData
// ═══════════════════════════════════════════════════════════════════════════

describe('generateBlogIndexStructuredData', () => {
	it('emits CollectionPage with ItemList of posts at 1-based positions', () => {
		const data = generateBlogIndexStructuredData({
			url: `${SITE_URL}/blog`,
			posts: [
				{ url: `${SITE_URL}/blog/a`, name: 'A' },
				{ url: `${SITE_URL}/blog/b`, name: 'B' }
			]
		});
		expect(data['@type']).toBe('CollectionPage');
		const itemList = data.mainEntity as Record<string, unknown>;
		expect(itemList['@type']).toBe('ItemList');
		const items = itemList.itemListElement as Array<Record<string, unknown>>;
		expect(items[0].position).toBe(1);
		expect(items[1].position).toBe(2);
	});
});

describe('generateTradingTutorialStructuredData', () => {
	it('emits HowTo with trading keywords injected and the steps preserved', () => {
		const data = generateTradingTutorialStructuredData({
			name: 'How to read a chart',
			steps: [
				{ name: 'Identify trend', text: 'Look at the slope' },
				{ name: 'Find support', text: 'Mark the floor' }
			]
		});
		expect(data['@type']).toBe('HowTo');
		// Keywords are auto-injected for SEO consistency.
		expect(data.keywords).toBe('trading, tutorial, how to trade, trading education');
		const steps = data.step as Array<Record<string, unknown>>;
		expect(steps).toHaveLength(2);
	});
});
