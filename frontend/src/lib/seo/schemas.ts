/**
 * JSON-LD Schema Builders
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Typed builders that return `JsonLdNode` instances. Pages call them in their
 * load() and put the result(s) into `seo.jsonld`. The unified SEO layer
 * (resolve.ts → Seo.svelte) handles dedupe and serialization.
 *
 * Conventions:
 * - Every builder returns a single JsonLdNode (or array where natural).
 * - All URL fields expect absolute URLs (helpers do not synthesize an origin).
 * - Undefined fields are omitted by serializeJsonLd downstream; callers may
 *   safely pass undefined for optional inputs.
 *
 * @since May 2026 (post Google May core update floor-raise)
 */

import type {
	JsonLdNode,
	JsonLdBreadcrumbList,
	JsonLdBreadcrumbItem,
	JsonLdArticle,
	JsonLdFAQPage,
	JsonLdFAQQuestion
} from './types';

// ─────────────────────────────────────────────────────────────────────────────
// Breadcrumb
// ─────────────────────────────────────────────────────────────────────────────

export interface BreadcrumbItem {
	name: string;
	url: string;
}

export function buildBreadcrumb(items: BreadcrumbItem[]): JsonLdBreadcrumbList {
	const itemListElement: JsonLdBreadcrumbItem[] = items.map((entry, index) => ({
		'@type': 'ListItem',
		position: index + 1,
		name: entry.name,
		item: entry.url
	}));
	return {
		'@context': 'https://schema.org',
		'@type': 'BreadcrumbList',
		itemListElement
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// Article / BlogPosting
// ─────────────────────────────────────────────────────────────────────────────

export interface ArticleInput {
	url: string;
	headline: string;
	description?: string;
	image?: string | string[];
	datePublished: string;
	dateModified?: string;
	author: { name: string; url?: string };
	publisher?: { name: string; logoUrl?: string };
	section?: string;
	keywords?: string[];
	wordCount?: number;
	type?: 'Article' | 'BlogPosting' | 'NewsArticle';
}

const DEFAULT_PUBLISHER = {
	name: 'Revolution Trading Pros',
	logoUrl: 'https://revolution-trading-pros.pages.dev/icon-512.png'
};

export function buildArticle(input: ArticleInput): JsonLdArticle {
	const publisher = input.publisher ?? DEFAULT_PUBLISHER;
	return {
		'@context': 'https://schema.org',
		'@type': input.type ?? 'BlogPosting',
		headline: input.headline,
		description: input.description,
		image: input.image,
		datePublished: input.datePublished,
		dateModified: input.dateModified ?? input.datePublished,
		author: {
			'@type': 'Person',
			name: input.author.name,
			url: input.author.url
		},
		publisher: {
			'@type': 'Organization',
			name: publisher.name,
			logo: publisher.logoUrl ? { '@type': 'ImageObject', url: publisher.logoUrl } : undefined
		},
		mainEntityOfPage: {
			'@type': 'WebPage',
			'@id': input.url
		},
		articleSection: input.section,
		keywords: input.keywords,
		wordCount: input.wordCount
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// Course (schema.org/Course)
// ─────────────────────────────────────────────────────────────────────────────

export interface CourseInput {
	url: string;
	name: string;
	description: string;
	provider: { name: string; url: string };
	image?: string;
	offers?: {
		price: number;
		priceCurrency: string;
		availability?: 'InStock' | 'PreOrder' | 'SoldOut';
		url?: string;
	};
	hasCourseInstance?: {
		courseMode: 'Online' | 'Onsite' | 'Blended';
		courseWorkload?: string;
		instructor?: { name: string; url?: string };
	};
	educationalLevel?: 'Beginner' | 'Intermediate' | 'Advanced';
}

export function buildCourse(input: CourseInput): JsonLdNode {
	return {
		'@context': 'https://schema.org',
		'@type': 'Course',
		name: input.name,
		description: input.description,
		url: input.url,
		image: input.image,
		provider: {
			'@type': 'Organization',
			name: input.provider.name,
			url: input.provider.url,
			sameAs: input.provider.url
		},
		educationalLevel: input.educationalLevel,
		offers: input.offers
			? {
					'@type': 'Offer',
					price: input.offers.price,
					priceCurrency: input.offers.priceCurrency,
					availability: input.offers.availability
						? `https://schema.org/${input.offers.availability}`
						: undefined,
					url: input.offers.url ?? input.url
				}
			: undefined,
		hasCourseInstance: input.hasCourseInstance
			? {
					'@type': 'CourseInstance',
					courseMode: input.hasCourseInstance.courseMode,
					courseWorkload: input.hasCourseInstance.courseWorkload,
					instructor: input.hasCourseInstance.instructor
						? {
								'@type': 'Person',
								name: input.hasCourseInstance.instructor.name,
								url: input.hasCourseInstance.instructor.url
							}
						: undefined
				}
			: undefined
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// HowTo
// ─────────────────────────────────────────────────────────────────────────────

export interface HowToStep {
	name: string;
	text: string;
	image?: string;
	url?: string;
}

export interface HowToInput {
	name: string;
	description: string;
	totalTime?: string;
	image?: string;
	steps: HowToStep[];
}

export function buildHowTo(input: HowToInput): JsonLdNode {
	return {
		'@context': 'https://schema.org',
		'@type': 'HowTo',
		name: input.name,
		description: input.description,
		totalTime: input.totalTime,
		image: input.image,
		step: input.steps.map((s, i) => ({
			'@type': 'HowToStep',
			position: i + 1,
			name: s.name,
			text: s.text,
			image: s.image,
			url: s.url
		}))
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// Event / BroadcastEvent (live trading rooms)
// ─────────────────────────────────────────────────────────────────────────────

export interface EventInput {
	name: string;
	description: string;
	url: string;
	startDate: string;
	endDate?: string;
	eventStatus?: 'EventScheduled' | 'EventRescheduled' | 'EventCancelled';
	eventAttendanceMode?:
		| 'OnlineEventAttendanceMode'
		| 'OfflineEventAttendanceMode'
		| 'MixedEventAttendanceMode';
	location?: { name: string; url?: string };
	organizer: { name: string; url: string };
	isLiveBroadcast?: boolean;
	performer?: { name: string; url?: string };
	offers?: { price: number; priceCurrency: string; url: string };
}

export function buildEvent(input: EventInput): JsonLdNode {
	const type = input.isLiveBroadcast ? 'BroadcastEvent' : 'Event';
	return {
		'@context': 'https://schema.org',
		'@type': type,
		name: input.name,
		description: input.description,
		url: input.url,
		startDate: input.startDate,
		endDate: input.endDate,
		eventStatus: input.eventStatus
			? `https://schema.org/${input.eventStatus}`
			: 'https://schema.org/EventScheduled',
		eventAttendanceMode: input.eventAttendanceMode
			? `https://schema.org/${input.eventAttendanceMode}`
			: 'https://schema.org/OnlineEventAttendanceMode',
		location: input.location
			? {
					'@type': 'VirtualLocation',
					name: input.location.name,
					url: input.location.url ?? input.url
				}
			: { '@type': 'VirtualLocation', url: input.url },
		organizer: {
			'@type': 'Organization',
			name: input.organizer.name,
			url: input.organizer.url
		},
		isLiveBroadcast: input.isLiveBroadcast,
		performer: input.performer
			? { '@type': 'Person', name: input.performer.name, url: input.performer.url }
			: undefined,
		offers: input.offers
			? {
					'@type': 'Offer',
					price: input.offers.price,
					priceCurrency: input.offers.priceCurrency,
					url: input.offers.url
				}
			: undefined
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// SoftwareApplication (indicators-as-tools)
// ─────────────────────────────────────────────────────────────────────────────

export interface SoftwareApplicationInput {
	name: string;
	description: string;
	url: string;
	applicationCategory: string;
	operatingSystem?: string;
	image?: string;
	offers?: { price: number; priceCurrency: string; url?: string };
	aggregateRating?: { ratingValue: number; ratingCount: number };
}

export function buildSoftwareApplication(input: SoftwareApplicationInput): JsonLdNode {
	return {
		'@context': 'https://schema.org',
		'@type': 'SoftwareApplication',
		name: input.name,
		description: input.description,
		url: input.url,
		applicationCategory: input.applicationCategory,
		operatingSystem: input.operatingSystem ?? 'Web',
		image: input.image,
		offers: input.offers
			? {
					'@type': 'Offer',
					price: input.offers.price,
					priceCurrency: input.offers.priceCurrency,
					url: input.offers.url ?? input.url
				}
			: undefined,
		aggregateRating: input.aggregateRating
			? {
					'@type': 'AggregateRating',
					ratingValue: input.aggregateRating.ratingValue,
					ratingCount: input.aggregateRating.ratingCount
				}
			: undefined
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// FAQPage
//
// Google deprecated FAQ rich results in May 2026 and limits the remaining
// eligibility to well-known government/health sites. Keep this builder only for
// non-Google consumers or explicitly eligible pages; do not add it to marketing
// pages for Google SEO.
// ─────────────────────────────────────────────────────────────────────────────

export interface FAQItem {
	q: string;
	a: string;
}

export function buildFAQPage(items: FAQItem[]): JsonLdFAQPage {
	const mainEntity: JsonLdFAQQuestion[] = items.map((item) => ({
		'@type': 'Question',
		name: item.q,
		acceptedAnswer: {
			'@type': 'Answer',
			text: item.a
		}
	}));
	return {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// Person (author / instructor profile pages)
// ─────────────────────────────────────────────────────────────────────────────

export interface PersonInput {
	name: string;
	url: string;
	jobTitle?: string;
	description?: string;
	image?: string;
	sameAs?: string[];
	knowsAbout?: string[];
	worksFor?: { name: string; url: string };
}

export function buildPerson(input: PersonInput): JsonLdNode {
	return {
		'@context': 'https://schema.org',
		'@type': 'Person',
		name: input.name,
		url: input.url,
		jobTitle: input.jobTitle,
		description: input.description,
		image: input.image,
		sameAs: input.sameAs,
		knowsAbout: input.knowsAbout,
		worksFor: input.worksFor
			? { '@type': 'Organization', name: input.worksFor.name, url: input.worksFor.url }
			: undefined
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// ItemList (collection landing pages: /courses, /indicators, /classes, etc.)
// ─────────────────────────────────────────────────────────────────────────────

export interface ItemListEntry {
	name: string;
	url: string;
	description?: string;
	image?: string;
}

export function buildItemList(name: string, entries: ItemListEntry[]): JsonLdNode {
	return {
		'@context': 'https://schema.org',
		'@type': 'ItemList',
		name,
		itemListElement: entries.map((e, i) => ({
			'@type': 'ListItem',
			position: i + 1,
			url: e.url,
			name: e.name
		}))
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// Service (alerts subscriptions, mentorship offerings)
// ─────────────────────────────────────────────────────────────────────────────

export interface ServiceInput {
	name: string;
	description: string;
	url: string;
	provider: { name: string; url: string };
	serviceType?: string;
	areaServed?: string;
	offers?: { price: number; priceCurrency: string; url?: string };
}

export function buildService(input: ServiceInput): JsonLdNode {
	return {
		'@context': 'https://schema.org',
		'@type': 'Service',
		name: input.name,
		description: input.description,
		url: input.url,
		serviceType: input.serviceType,
		areaServed: input.areaServed,
		provider: {
			'@type': 'Organization',
			name: input.provider.name,
			url: input.provider.url
		},
		offers: input.offers
			? {
					'@type': 'Offer',
					price: input.offers.price,
					priceCurrency: input.offers.priceCurrency,
					url: input.offers.url ?? input.url
				}
			: undefined
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// WebPage with dateModified (trust pages: privacy, terms, cookie-policy)
// ─────────────────────────────────────────────────────────────────────────────

export interface WebPageInput {
	url: string;
	name: string;
	description?: string;
	datePublished?: string;
	dateModified: string;
	pageType?: 'WebPage' | 'AboutPage' | 'ContactPage' | 'ProfilePage';
}

export function buildWebPage(input: WebPageInput): JsonLdNode {
	return {
		'@context': 'https://schema.org',
		'@type': input.pageType ?? 'WebPage',
		url: input.url,
		name: input.name,
		description: input.description,
		datePublished: input.datePublished,
		dateModified: input.dateModified
	};
}
