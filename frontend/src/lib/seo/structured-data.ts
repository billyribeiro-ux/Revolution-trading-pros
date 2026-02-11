/**
 * Structured Data Generator
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Generates JSON-LD structured data for SEO optimization.
 * Supports multiple schema types for rich search results.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface OrganizationSchema {
	name: string;
	url: string;
	logo?: string;
	description?: string;
	email?: string;
	telephone?: string;
	address?: {
		streetAddress?: string;
		addressLocality?: string;
		addressRegion?: string;
		postalCode?: string;
		addressCountry?: string;
	};
	sameAs?: string[];
}

export interface WebsiteSchema {
	name: string;
	url: string;
	searchUrl?: string;
}

export interface ArticleSchema {
	headline: string;
	description?: string;
	image?: string | string[];
	datePublished: string;
	dateModified?: string;
	author: {
		name: string;
		url?: string;
	};
	publisher: {
		name: string;
		logo?: string;
	};
	url: string;
}

export interface CourseSchema {
	name: string;
	description: string;
	provider: {
		name: string;
		url?: string;
	};
	url?: string;
	image?: string;
	instructor?: {
		name: string;
		description?: string;
	};
	duration?: string;
	offers?: {
		price: number | string;
		priceCurrency: string;
		availability?: string;
	};
}

export interface ProductSchema {
	name: string;
	description?: string;
	image?: string | string[];
	brand?: string;
	sku?: string;
	offers?: {
		price: number | string;
		priceCurrency: string;
		availability?: string;
		url?: string;
	};
	aggregateRating?: {
		ratingValue: number;
		reviewCount: number;
	};
}

export interface FAQSchema {
	questions: Array<{
		question: string;
		answer: string;
	}>;
}

export interface BreadcrumbSchema {
	items: Array<{
		name: string;
		url: string;
	}>;
}

export interface VideoSchema {
	name: string;
	description: string;
	thumbnailUrl: string;
	uploadDate: string;
	duration?: string;
	contentUrl?: string;
	embedUrl?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// GENERATORS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Generate Organization schema
 */
export function generateOrganizationSchema(data: OrganizationSchema): object {
	const schema: Record<string, unknown> = {
		'@context': 'https://schema.org',
		'@type': 'Organization',
		name: data.name,
		url: data.url
	};

	if (data.logo) {
		schema.logo = data.logo;
	}

	if (data.description) {
		schema.description = data.description;
	}

	if (data.email) {
		schema.email = data.email;
	}

	if (data.telephone) {
		schema.telephone = data.telephone;
	}

	if (data.address) {
		schema.address = {
			'@type': 'PostalAddress',
			...data.address
		};
	}

	if (data.sameAs && data.sameAs.length > 0) {
		schema.sameAs = data.sameAs;
	}

	return schema;
}

/**
 * Generate WebSite schema with search action
 */
export function generateWebsiteSchema(data: WebsiteSchema): object {
	const schema: Record<string, unknown> = {
		'@context': 'https://schema.org',
		'@type': 'WebSite',
		name: data.name,
		url: data.url
	};

	if (data.searchUrl) {
		schema.potentialAction = {
			'@type': 'SearchAction',
			target: {
				'@type': 'EntryPoint',
				urlTemplate: data.searchUrl
			},
			'query-input': 'required name=search_term_string'
		};
	}

	return schema;
}

/**
 * Generate Article schema
 */
export function generateArticleSchema(data: ArticleSchema): object {
	const schema: Record<string, unknown> = {
		'@context': 'https://schema.org',
		'@type': 'Article',
		headline: data.headline,
		datePublished: data.datePublished,
		author: {
			'@type': 'Person',
			name: data.author.name,
			...(data.author.url && { url: data.author.url })
		},
		publisher: {
			'@type': 'Organization',
			name: data.publisher.name,
			...(data.publisher.logo && {
				logo: {
					'@type': 'ImageObject',
					url: data.publisher.logo
				}
			})
		},
		mainEntityOfPage: {
			'@type': 'WebPage',
			'@id': data.url
		}
	};

	if (data.description) {
		schema.description = data.description;
	}

	if (data.image) {
		schema.image = data.image;
	}

	if (data.dateModified) {
		schema.dateModified = data.dateModified;
	}

	return schema;
}

/**
 * Generate Course schema
 */
export function generateCourseSchema(data: CourseSchema): object {
	const schema: Record<string, unknown> = {
		'@context': 'https://schema.org',
		'@type': 'Course',
		name: data.name,
		description: data.description,
		provider: {
			'@type': 'Organization',
			name: data.provider.name,
			...(data.provider.url && { url: data.provider.url })
		}
	};

	if (data.url) {
		schema.url = data.url;
	}

	if (data.image) {
		schema.image = data.image;
	}

	if (data.instructor) {
		schema.instructor = {
			'@type': 'Person',
			name: data.instructor.name,
			...(data.instructor.description && { description: data.instructor.description })
		};
	}

	if (data.duration) {
		schema.timeRequired = data.duration;
	}

	if (data.offers) {
		schema.offers = {
			'@type': 'Offer',
			price: data.offers.price,
			priceCurrency: data.offers.priceCurrency,
			availability: data.offers.availability || 'https://schema.org/InStock'
		};
	}

	return schema;
}

/**
 * Generate Product schema
 */
export function generateProductSchema(data: ProductSchema): object {
	const schema: Record<string, unknown> = {
		'@context': 'https://schema.org',
		'@type': 'Product',
		name: data.name
	};

	if (data.description) {
		schema.description = data.description;
	}

	if (data.image) {
		schema.image = data.image;
	}

	if (data.brand) {
		schema.brand = {
			'@type': 'Brand',
			name: data.brand
		};
	}

	if (data.sku) {
		schema.sku = data.sku;
	}

	if (data.offers) {
		schema.offers = {
			'@type': 'Offer',
			price: data.offers.price,
			priceCurrency: data.offers.priceCurrency,
			availability: data.offers.availability || 'https://schema.org/InStock',
			...(data.offers.url && { url: data.offers.url })
		};
	}

	if (data.aggregateRating) {
		schema.aggregateRating = {
			'@type': 'AggregateRating',
			ratingValue: data.aggregateRating.ratingValue,
			reviewCount: data.aggregateRating.reviewCount
		};
	}

	return schema;
}

/**
 * Generate FAQ schema
 */
export function generateFAQSchema(data: FAQSchema): object {
	return {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity: data.questions.map((q) => ({
			'@type': 'Question',
			name: q.question,
			acceptedAnswer: {
				'@type': 'Answer',
				text: q.answer
			}
		}))
	};
}

/**
 * Generate BreadcrumbList schema
 */
export function generateBreadcrumbSchema(data: BreadcrumbSchema): object {
	return {
		'@context': 'https://schema.org',
		'@type': 'BreadcrumbList',
		itemListElement: data.items.map((item, index) => ({
			'@type': 'ListItem',
			position: index + 1,
			name: item.name,
			item: item.url
		}))
	};
}

/**
 * Generate VideoObject schema
 */
export function generateVideoSchema(data: VideoSchema): object {
	const schema: Record<string, unknown> = {
		'@context': 'https://schema.org',
		'@type': 'VideoObject',
		name: data.name,
		description: data.description,
		thumbnailUrl: data.thumbnailUrl,
		uploadDate: data.uploadDate
	};

	if (data.duration) {
		schema.duration = data.duration;
	}

	if (data.contentUrl) {
		schema.contentUrl = data.contentUrl;
	}

	if (data.embedUrl) {
		schema.embedUrl = data.embedUrl;
	}

	return schema;
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Convert schema object to JSON-LD script tag content
 */
export function toJsonLd(schema: object | object[]): string {
	return JSON.stringify(schema, null, 0);
}

/**
 * Wrap schema in script tag
 */
export function toScriptTag(schema: object | object[]): string {
	return `<script type="application/ld+json">${toJsonLd(schema)}</script>`;
}

/**
 * Combine multiple schemas into a graph
 */
export function combineSchemas(schemas: object[]): object {
	return {
		'@context': 'https://schema.org',
		'@graph': schemas.map((s) => {
			const record = s as Record<string, unknown>;
			const { '@context': context, ...rest } = record;
			void context;
			return rest;
		})
	};
}

/**
 * Format ISO date for schema
 */
export function formatSchemaDate(date: Date | string): string {
	const d = typeof date === 'string' ? new Date(date) : date;
	return d.toISOString();
}

/**
 * Format duration in ISO 8601 format
 */
export function formatSchemaDuration(minutes: number): string {
	const hours = Math.floor(minutes / 60);
	const mins = minutes % 60;

	if (hours > 0) {
		return `PT${hours}H${mins > 0 ? `${mins}M` : ''}`;
	}
	return `PT${mins}M`;
}

// ═══════════════════════════════════════════════════════════════════════════════
// DEFAULT SCHEMAS FOR REVOLUTION TRADING PROS
// ═══════════════════════════════════════════════════════════════════════════════

export const DEFAULT_ORGANIZATION: OrganizationSchema = {
	name: 'Revolution Trading Pros',
	url: 'https://revolutiontradingpros.com',
	logo: 'https://revolutiontradingpros.com/logo.png',
	description:
		'Professional trading education, custom indicators, and live trading rooms for 18,000+ active traders.',
	sameAs: [
		'https://twitter.com/RevTradingPros',
		'https://facebook.com/RevTradingPros',
		'https://youtube.com/RevTradingPros'
	]
};

export const DEFAULT_WEBSITE: WebsiteSchema = {
	name: 'Revolution Trading Pros',
	url: 'https://revolutiontradingpros.com',
	searchUrl: 'https://revolutiontradingpros.com/search?q={search_term_string}'
};

/**
 * Generate default schemas for the site
 */
export function generateDefaultSchemas(): object {
	return combineSchemas([
		generateOrganizationSchema(DEFAULT_ORGANIZATION),
		generateWebsiteSchema(DEFAULT_WEBSITE)
	]);
}
