/**
 * Schema Builder Tests
 * ───────────────────────────────────────────────────────────────────────────
 * Shape-stability tests for the JSON-LD builders in schemas.ts. These guard
 * against accidental drift in @type, @context, required-field shapes — the
 * exact things Google Rich Results validators reject.
 */

import { describe, it, expect } from 'vitest';
import {
	buildBreadcrumb,
	buildArticle,
	buildCourse,
	buildHowTo,
	buildEvent,
	buildSoftwareApplication,
	buildFAQPage,
	buildPerson,
	buildItemList,
	buildService,
	buildWebPage
} from '../schemas';

describe('buildBreadcrumb', () => {
	it('emits 1-indexed ListItems in input order', () => {
		const node = buildBreadcrumb([
			{ name: 'Home', url: 'https://example.com/' },
			{ name: 'Blog', url: 'https://example.com/blog' },
			{ name: 'Post', url: 'https://example.com/blog/post' }
		]);
		expect(node['@type']).toBe('BreadcrumbList');
		expect(node['@context']).toBe('https://schema.org');
		expect(node.itemListElement).toHaveLength(3);
		expect(node.itemListElement[0]).toEqual({
			'@type': 'ListItem',
			position: 1,
			name: 'Home',
			item: 'https://example.com/'
		});
		expect(node.itemListElement[2].position).toBe(3);
	});

	it('handles empty input without throwing', () => {
		expect(buildBreadcrumb([]).itemListElement).toEqual([]);
	});
});

describe('buildArticle', () => {
	it('defaults type to BlogPosting and dateModified to datePublished', () => {
		const node = buildArticle({
			url: 'https://example.com/blog/x',
			headline: 'Test',
			datePublished: '2026-05-25T00:00:00Z',
			author: { name: 'Author', url: 'https://example.com/authors/a' }
		});
		expect(node['@type']).toBe('BlogPosting');
		expect(node.dateModified).toBe('2026-05-25T00:00:00Z');
		expect(node.author).toEqual({
			'@type': 'Person',
			name: 'Author',
			url: 'https://example.com/authors/a'
		});
		expect(node.mainEntityOfPage).toEqual({
			'@type': 'WebPage',
			'@id': 'https://example.com/blog/x'
		});
	});

	it('uses Revolution Trading Pros publisher by default', () => {
		const node = buildArticle({
			url: 'https://example.com/blog/x',
			headline: 'Test',
			datePublished: '2026-05-25',
			author: { name: 'A' }
		});
		expect(node.publisher.name).toBe('Revolution Trading Pros');
	});

	it('respects explicit type override', () => {
		const node = buildArticle({
			url: 'https://example.com/news/x',
			headline: 'Test',
			datePublished: '2026-05-25',
			author: { name: 'A' },
			type: 'NewsArticle'
		});
		expect(node['@type']).toBe('NewsArticle');
	});
});

describe('buildCourse', () => {
	it('emits Course with Offer when offers provided', () => {
		const node = buildCourse({
			url: 'https://example.com/courses/x',
			name: 'X',
			description: 'D',
			provider: { name: 'P', url: 'https://example.com' },
			offers: { price: 99, priceCurrency: 'USD', availability: 'InStock' }
		});
		expect(node['@type']).toBe('Course');
		expect((node as Record<string, unknown>).offers).toMatchObject({
			'@type': 'Offer',
			price: 99,
			priceCurrency: 'USD',
			availability: 'https://schema.org/InStock'
		});
	});

	it('omits offers / hasCourseInstance when not provided', () => {
		const node = buildCourse({
			url: 'https://example.com/courses/x',
			name: 'X',
			description: 'D',
			provider: { name: 'P', url: 'https://example.com' }
		}) as Record<string, unknown>;
		expect(node.offers).toBeUndefined();
		expect(node.hasCourseInstance).toBeUndefined();
	});
});

describe('buildHowTo', () => {
	it('numbers steps starting at 1', () => {
		const node = buildHowTo({
			name: 'How',
			description: 'D',
			steps: [
				{ name: 'A', text: 'a' },
				{ name: 'B', text: 'b' }
			]
		}) as Record<string, unknown>;
		expect(node['@type']).toBe('HowTo');
		const steps = node.step as Array<Record<string, unknown>>;
		expect(steps[0].position).toBe(1);
		expect(steps[1].position).toBe(2);
	});
});

describe('buildEvent', () => {
	it('emits BroadcastEvent with default OnlineEventAttendanceMode when live', () => {
		const node = buildEvent({
			name: 'Live Room',
			description: 'D',
			url: 'https://example.com/r',
			startDate: '2026-06-01T09:30:00-04:00',
			organizer: { name: 'O', url: 'https://example.com' },
			isLiveBroadcast: true
		}) as Record<string, unknown>;
		expect(node['@type']).toBe('BroadcastEvent');
		expect(node.eventAttendanceMode).toBe('https://schema.org/OnlineEventAttendanceMode');
		expect(node.eventStatus).toBe('https://schema.org/EventScheduled');
	});

	it('emits plain Event when not a live broadcast', () => {
		const node = buildEvent({
			name: 'X',
			description: 'D',
			url: 'https://example.com/r',
			startDate: '2026-06-01',
			organizer: { name: 'O', url: 'https://example.com' }
		}) as Record<string, unknown>;
		expect(node['@type']).toBe('Event');
	});
});

describe('buildSoftwareApplication', () => {
	it('defaults operatingSystem to Web', () => {
		const node = buildSoftwareApplication({
			name: 'MACD',
			description: 'D',
			url: 'https://example.com/indicators/macd',
			applicationCategory: 'FinanceApplication'
		}) as Record<string, unknown>;
		expect(node['@type']).toBe('SoftwareApplication');
		expect(node.operatingSystem).toBe('Web');
	});
});

describe('buildFAQPage', () => {
	it('wraps each Q/A pair in Question + Answer types', () => {
		const node = buildFAQPage([{ q: 'Q1?', a: 'A1' }]);
		expect(node['@type']).toBe('FAQPage');
		expect(node.mainEntity[0]).toEqual({
			'@type': 'Question',
			name: 'Q1?',
			acceptedAnswer: { '@type': 'Answer', text: 'A1' }
		});
	});
});

describe('buildPerson', () => {
	it('emits a Person node with worksFor expansion', () => {
		const node = buildPerson({
			name: 'Billy',
			url: 'https://example.com/authors/billy',
			jobTitle: 'Lead Trader',
			worksFor: { name: 'RTP', url: 'https://example.com' }
		}) as Record<string, unknown>;
		expect(node['@type']).toBe('Person');
		expect(node.worksFor).toEqual({
			'@type': 'Organization',
			name: 'RTP',
			url: 'https://example.com'
		});
	});
});

describe('buildItemList', () => {
	it('numbers entries starting at position 1', () => {
		const node = buildItemList('Courses', [
			{ name: 'A', url: 'https://example.com/a' },
			{ name: 'B', url: 'https://example.com/b' }
		]) as Record<string, unknown>;
		expect(node['@type']).toBe('ItemList');
		const items = node.itemListElement as Array<Record<string, unknown>>;
		expect(items[0].position).toBe(1);
		expect(items[1].url).toBe('https://example.com/b');
	});
});

describe('buildService', () => {
	it('emits Service with provider + optional Offer', () => {
		const node = buildService({
			name: 'Alerts',
			description: 'D',
			url: 'https://example.com/alerts',
			provider: { name: 'RTP', url: 'https://example.com' },
			offers: { price: 49, priceCurrency: 'USD' }
		}) as Record<string, unknown>;
		expect(node['@type']).toBe('Service');
		expect((node.offers as Record<string, unknown>).price).toBe(49);
	});
});

describe('buildWebPage', () => {
	it('defaults pageType to WebPage and preserves dateModified', () => {
		const node = buildWebPage({
			url: 'https://example.com/privacy',
			name: 'Privacy',
			dateModified: '2026-05-25'
		}) as Record<string, unknown>;
		expect(node['@type']).toBe('WebPage');
		expect(node.dateModified).toBe('2026-05-25');
	});

	it('honors pageType override', () => {
		const node = buildWebPage({
			url: 'https://example.com/contact',
			name: 'Contact',
			dateModified: '2026-05-25',
			pageType: 'ContactPage'
		});
		expect(node['@type']).toBe('ContactPage');
	});
});
