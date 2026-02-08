/**
 * SEO Plugin Layer - JSON-LD Helpers
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Type-safe builders for structured data nodes.
 * All builders validate required fields and produce ready-to-render nodes.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

import type {
	JsonLdOrganization,
	JsonLdWebSite,
	JsonLdBreadcrumbList,
	JsonLdArticle,
	JsonLdFAQPage,
	JsonLdNode
} from './types';

// ═══════════════════════════════════════════════════════════════════════════════
// BUILDERS
// ═══════════════════════════════════════════════════════════════════════════════

export function organizationSchema(opts: {
	name: string;
	url: string;
	id?: string;
	logo?: string;
	description?: string;
	email?: string;
	telephone?: string;
	sameAs?: string[];
	address?: {
		streetAddress?: string;
		addressLocality?: string;
		addressRegion?: string;
		postalCode?: string;
		addressCountry?: string;
	};
}): JsonLdOrganization {
	const node: JsonLdOrganization = {
		'@context': 'https://schema.org',
		'@type': 'Organization',
		name: opts.name,
		url: opts.url
	};

	if (opts.id) node['@id'] = opts.id;
	if (opts.logo) node.logo = { '@type': 'ImageObject', url: opts.logo };
	if (opts.description) node.description = opts.description;
	if (opts.email) node.email = opts.email;
	if (opts.telephone) node.telephone = opts.telephone;
	if (opts.sameAs && opts.sameAs.length > 0) node.sameAs = opts.sameAs;
	if (opts.address) {
		node.address = {
			'@type': 'PostalAddress',
			...opts.address
		};
	}

	return node;
}

export function websiteSchema(opts: {
	name: string;
	url: string;
	id?: string;
	searchUrlTemplate?: string;
}): JsonLdWebSite {
	const node: JsonLdWebSite = {
		'@context': 'https://schema.org',
		'@type': 'WebSite',
		name: opts.name,
		url: opts.url
	};

	if (opts.id) node['@id'] = opts.id;
	if (opts.searchUrlTemplate) {
		node.potentialAction = {
			'@type': 'SearchAction',
			target: {
				'@type': 'EntryPoint',
				urlTemplate: opts.searchUrlTemplate
			},
			'query-input': 'required name=search_term_string'
		};
	}

	return node;
}

export function breadcrumbSchema(
	items: Array<{ name: string; url: string }>,
	id?: string
): JsonLdBreadcrumbList {
	const node: JsonLdBreadcrumbList = {
		'@context': 'https://schema.org',
		'@type': 'BreadcrumbList',
		itemListElement: items.map((item, index) => ({
			'@type': 'ListItem' as const,
			position: index + 1,
			name: item.name,
			item: item.url
		}))
	};

	if (id) node['@id'] = id;
	return node;
}

export function articleSchema(opts: {
	headline: string;
	url: string;
	datePublished: string;
	authorName: string;
	publisherName: string;
	id?: string;
	type?: 'Article' | 'BlogPosting' | 'NewsArticle';
	description?: string;
	image?: string | string[];
	dateModified?: string;
	authorUrl?: string;
	publisherLogo?: string;
	wordCount?: number;
	articleSection?: string;
	keywords?: string[];
}): JsonLdArticle {
	const node: JsonLdArticle = {
		'@context': 'https://schema.org',
		'@type': opts.type ?? 'Article',
		headline: opts.headline.slice(0, 110),
		datePublished: opts.datePublished,
		author: {
			'@type': 'Person',
			name: opts.authorName
		},
		publisher: {
			'@type': 'Organization',
			name: opts.publisherName
		},
		mainEntityOfPage: {
			'@type': 'WebPage',
			'@id': opts.url
		}
	};

	if (opts.id) node['@id'] = opts.id;
	if (opts.description) node.description = opts.description;
	if (opts.image) node.image = opts.image;
	if (opts.dateModified) node.dateModified = opts.dateModified;
	if (opts.authorUrl) node.author.url = opts.authorUrl;
	if (opts.publisherLogo) {
		node.publisher.logo = { '@type': 'ImageObject', url: opts.publisherLogo };
	}
	if (opts.wordCount) node.wordCount = opts.wordCount;
	if (opts.articleSection) node.articleSection = opts.articleSection;
	if (opts.keywords && opts.keywords.length > 0) node.keywords = opts.keywords;

	return node;
}

export function faqSchema(
	questions: Array<{ question: string; answer: string }>,
	id?: string
): JsonLdFAQPage {
	if (questions.length === 0) {
		throw new Error('[SEO] faqSchema requires at least one question');
	}

	const node: JsonLdFAQPage = {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity: questions.map((q) => ({
			'@type': 'Question' as const,
			name: q.question,
			acceptedAnswer: {
				'@type': 'Answer' as const,
				text: q.answer
			}
		}))
	};

	if (id) node['@id'] = id;
	return node;
}

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Stable hash for a JSON-LD node (for deduplication).
 * Uses @id if present, otherwise sorted JSON serialization.
 */
export function jsonLdHash(node: JsonLdNode): string {
	if (node['@id']) return node['@id'];
	const sortedKeys = Object.keys(node).sort();
	const parts = sortedKeys.map(
		(k) => `${k}:${JSON.stringify((node as Record<string, unknown>)[k])}`
	);
	return parts.join('|');
}

/**
 * Deduplicate JSON-LD nodes by @id or stable hash.
 * Later entries win (last-write-wins).
 */
export function dedupeJsonLd(nodes: JsonLdNode[]): JsonLdNode[] {
	const map = new Map<string, JsonLdNode>();
	for (const node of nodes) {
		map.set(jsonLdHash(node), node);
	}
	return Array.from(map.values());
}

/**
 * Safely serialize a JSON-LD node for embedding in a <script> tag.
 * Escapes </script> and <!-- to prevent XSS via script injection.
 */
export function safeJsonLdSerialize(node: JsonLdNode | JsonLdNode[] | Record<string, unknown>): string {
	const json = JSON.stringify(node);
	return json
		.replace(/<\/script/gi, '<\\/script')
		.replace(/<!--/g, '<\\!--');
}

/**
 * Wrap multiple JSON-LD nodes into a @graph structure.
 * Strips individual @context to avoid redundancy.
 */
export function toGraph(nodes: JsonLdNode[]): Record<string, unknown> {
	return {
		'@context': 'https://schema.org',
		'@graph': nodes.map((n) => {
			const { '@context': _ctx, ...rest } = n as Record<string, unknown>;
			return rest;
		})
	};
}
