/**
 * JSON-LD Structured Data Utilities - Apple ICT 7+ Principal Engineer Grade
 * ==========================================================================
 *
 * Comprehensive structured data generation for:
 * - Organization
 * - WebSite (with SearchAction)
 * - Article / BlogPosting
 * - Product
 * - Course
 * - FAQPage
 * - BreadcrumbList
 * - Person (Author)
 * - Service
 * - Review / AggregateRating
 *
 * All schemas follow Schema.org specifications and Google's structured data guidelines.
 *
 * @version 2.0.0 - January 2026
 */

// =============================================================================
// Type Definitions
// =============================================================================

export type StructuredDataType =
	| 'Organization'
	| 'WebSite'
	| 'Article'
	| 'BlogPosting'
	| 'Product'
	| 'Course'
	| 'FAQPage'
	| 'BreadcrumbList'
	| 'Person'
	| 'Service'
	| 'Review'
	| 'VideoObject'
	| 'HowTo';

export interface BaseStructuredData {
	'@context': 'https://schema.org';
	'@type': string;
	[key: string]: unknown;
}

export interface OrganizationConfig {
	type: 'Organization';
	name: string;
	url: string;
	logo?: string;
	description?: string;
	email?: string;
	phone?: string;
	address?: {
		street?: string;
		city?: string;
		state?: string;
		postalCode?: string;
		country?: string;
	};
	socialProfiles?: string[];
	sameAs?: string[];
}

export interface WebSiteConfig {
	type: 'WebSite';
	name: string;
	url: string;
	description?: string;
	searchUrl?: string;
	potentialAction?: boolean;
}

export interface ArticleConfig {
	type: 'Article' | 'BlogPosting';
	headline: string;
	url: string;
	image?: string | string[];
	datePublished: string;
	dateModified?: string;
	author: {
		name: string;
		url?: string;
		image?: string;
	};
	publisher?: {
		name: string;
		logo: string;
		url?: string;
	};
	description?: string;
	articleBody?: string;
	keywords?: string[];
	wordCount?: number;
	mainEntityOfPage?: string;
}

export interface ProductConfig {
	type: 'Product';
	name: string;
	url: string;
	image?: string | string[];
	description?: string;
	brand?: string;
	sku?: string;
	price?: number;
	priceCurrency?: string;
	availability?: 'InStock' | 'OutOfStock' | 'PreOrder' | 'Discontinued';
	ratingValue?: number;
	reviewCount?: number;
	reviews?: ReviewConfig[];
}

export interface CourseConfig {
	type: 'Course';
	name: string;
	url: string;
	description?: string;
	provider?: {
		name: string;
		url?: string;
	};
	instructor?: {
		name: string;
		url?: string;
		image?: string;
	};
	courseMode?: 'online' | 'onsite' | 'blended';
	educationalLevel?: string;
	duration?: string;
	price?: number;
	priceCurrency?: string;
	image?: string;
	hasCourseInstance?: {
		startDate?: string;
		endDate?: string;
		courseMode?: string;
	}[];
}

export interface FAQConfig {
	type: 'FAQPage';
	questions: {
		question: string;
		answer: string;
	}[];
}

export interface BreadcrumbConfig {
	type: 'BreadcrumbList';
	items: {
		name: string;
		url: string;
	}[];
}

export interface PersonConfig {
	type: 'Person';
	name: string;
	url?: string;
	image?: string;
	description?: string;
	jobTitle?: string;
	email?: string;
	sameAs?: string[];
}

export interface ServiceConfig {
	type: 'Service';
	name: string;
	url: string;
	description?: string;
	provider?: {
		name: string;
		url?: string;
	};
	serviceType?: string;
	areaServed?: string;
	price?: number;
	priceCurrency?: string;
	image?: string;
	aggregateRating?: {
		ratingValue: number;
		reviewCount: number;
	};
}

export interface ReviewConfig {
	type: 'Review';
	author: string;
	datePublished?: string;
	reviewBody: string;
	ratingValue: number;
	bestRating?: number;
	worstRating?: number;
}

export interface VideoConfig {
	type: 'VideoObject';
	name: string;
	description?: string;
	thumbnailUrl: string;
	uploadDate: string;
	duration?: string;
	contentUrl?: string;
	embedUrl?: string;
	publisher?: {
		name: string;
		logo: string;
	};
}

export interface HowToConfig {
	type: 'HowTo';
	name: string;
	description?: string;
	image?: string;
	totalTime?: string;
	supply?: string[];
	tool?: string[];
	steps: {
		name: string;
		text: string;
		image?: string;
		url?: string;
	}[];
}

export type StructuredDataConfig =
	| OrganizationConfig
	| WebSiteConfig
	| ArticleConfig
	| ProductConfig
	| CourseConfig
	| FAQConfig
	| BreadcrumbConfig
	| PersonConfig
	| ServiceConfig
	| ReviewConfig
	| VideoConfig
	| HowToConfig;

// =============================================================================
// Generator Functions
// =============================================================================

/**
 * Generate Organization schema
 */
function generateOrganization(config: OrganizationConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'Organization',
		name: config.name,
		url: config.url
	};

	if (config.logo) {
		data.logo = {
			'@type': 'ImageObject',
			url: config.logo
		};
	}

	if (config.description) data.description = config.description;
	if (config.email) data.email = config.email;
	if (config.phone) data.telephone = config.phone;

	if (config.address) {
		data.address = {
			'@type': 'PostalAddress',
			...(config.address.street && { streetAddress: config.address.street }),
			...(config.address.city && { addressLocality: config.address.city }),
			...(config.address.state && { addressRegion: config.address.state }),
			...(config.address.postalCode && { postalCode: config.address.postalCode }),
			...(config.address.country && { addressCountry: config.address.country })
		};
	}

	if (config.sameAs?.length) {
		data.sameAs = config.sameAs;
	}

	return data;
}

/**
 * Generate WebSite schema with optional SearchAction
 */
function generateWebSite(config: WebSiteConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'WebSite',
		name: config.name,
		url: config.url
	};

	if (config.description) data.description = config.description;

	if (config.potentialAction && config.searchUrl) {
		data.potentialAction = {
			'@type': 'SearchAction',
			target: {
				'@type': 'EntryPoint',
				urlTemplate: `${config.searchUrl}?q={search_term_string}`
			},
			'query-input': 'required name=search_term_string'
		};
	}

	return data;
}

/**
 * Generate Article or BlogPosting schema
 */
function generateArticle(config: ArticleConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': config.type,
		headline: config.headline,
		url: config.url,
		datePublished: config.datePublished,
		author: {
			'@type': 'Person',
			name: config.author.name,
			...(config.author.url && { url: config.author.url }),
			...(config.author.image && { image: config.author.image })
		}
	};

	if (config.dateModified) data.dateModified = config.dateModified;
	if (config.description) data.description = config.description;
	if (config.articleBody) data.articleBody = config.articleBody;
	if (config.wordCount) data.wordCount = config.wordCount;
	if (config.keywords?.length) data.keywords = config.keywords.join(', ');
	if (config.mainEntityOfPage) {
		data.mainEntityOfPage = {
			'@type': 'WebPage',
			'@id': config.mainEntityOfPage
		};
	}

	if (config.image) {
		data.image = Array.isArray(config.image) ? config.image : [config.image];
	}

	if (config.publisher) {
		data.publisher = {
			'@type': 'Organization',
			name: config.publisher.name,
			logo: {
				'@type': 'ImageObject',
				url: config.publisher.logo
			},
			...(config.publisher.url && { url: config.publisher.url })
		};
	}

	return data;
}

/**
 * Generate Product schema
 */
function generateProduct(config: ProductConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'Product',
		name: config.name,
		url: config.url
	};

	if (config.description) data.description = config.description;
	if (config.brand) {
		data.brand = {
			'@type': 'Brand',
			name: config.brand
		};
	}
	if (config.sku) data.sku = config.sku;
	if (config.image) {
		data.image = Array.isArray(config.image) ? config.image : [config.image];
	}

	if (config.price !== undefined) {
		data.offers = {
			'@type': 'Offer',
			price: config.price,
			priceCurrency: config.priceCurrency || 'USD',
			availability: `https://schema.org/${config.availability || 'InStock'}`,
			url: config.url
		};
	}

	if (config.ratingValue !== undefined) {
		data.aggregateRating = {
			'@type': 'AggregateRating',
			ratingValue: config.ratingValue,
			reviewCount: config.reviewCount || 0,
			bestRating: 5,
			worstRating: 1
		};
	}

	if (config.reviews?.length) {
		data.review = config.reviews.map((review) => ({
			'@type': 'Review',
			author: {
				'@type': 'Person',
				name: review.author
			},
			reviewBody: review.reviewBody,
			reviewRating: {
				'@type': 'Rating',
				ratingValue: review.ratingValue,
				bestRating: review.bestRating || 5,
				worstRating: review.worstRating || 1
			},
			...(review.datePublished && { datePublished: review.datePublished })
		}));
	}

	return data;
}

/**
 * Generate Course schema
 */
function generateCourse(config: CourseConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'Course',
		name: config.name,
		url: config.url
	};

	if (config.description) data.description = config.description;
	if (config.image) data.image = config.image;

	if (config.provider) {
		data.provider = {
			'@type': 'Organization',
			name: config.provider.name,
			...(config.provider.url && { url: config.provider.url })
		};
	}

	if (config.instructor) {
		data.instructor = {
			'@type': 'Person',
			name: config.instructor.name,
			...(config.instructor.url && { url: config.instructor.url }),
			...(config.instructor.image && { image: config.instructor.image })
		};
	}

	if (config.educationalLevel) data.educationalLevel = config.educationalLevel;

	if (config.price !== undefined) {
		data.offers = {
			'@type': 'Offer',
			price: config.price,
			priceCurrency: config.priceCurrency || 'USD',
			availability: 'https://schema.org/InStock',
			url: config.url
		};
	}

	if (config.hasCourseInstance?.length) {
		data.hasCourseInstance = config.hasCourseInstance.map((instance) => ({
			'@type': 'CourseInstance',
			courseMode: instance.courseMode || config.courseMode || 'online',
			...(instance.startDate && { startDate: instance.startDate }),
			...(instance.endDate && { endDate: instance.endDate })
		}));
	}

	return data;
}

/**
 * Generate FAQPage schema
 */
function generateFAQ(config: FAQConfig): BaseStructuredData {
	return {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity: config.questions.map((q) => ({
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
function generateBreadcrumbs(config: BreadcrumbConfig): BaseStructuredData {
	return {
		'@context': 'https://schema.org',
		'@type': 'BreadcrumbList',
		itemListElement: config.items.map((item, index) => ({
			'@type': 'ListItem',
			position: index + 1,
			name: item.name,
			item: item.url
		}))
	};
}

/**
 * Generate Person schema
 */
function generatePerson(config: PersonConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'Person',
		name: config.name
	};

	if (config.url) data.url = config.url;
	if (config.image) data.image = config.image;
	if (config.description) data.description = config.description;
	if (config.jobTitle) data.jobTitle = config.jobTitle;
	if (config.email) data.email = config.email;
	if (config.sameAs?.length) data.sameAs = config.sameAs;

	return data;
}

/**
 * Generate Service schema
 */
function generateService(config: ServiceConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'Service',
		name: config.name,
		url: config.url
	};

	if (config.description) data.description = config.description;
	if (config.serviceType) data.serviceType = config.serviceType;
	if (config.areaServed) data.areaServed = config.areaServed;
	if (config.image) data.image = config.image;

	if (config.provider) {
		data.provider = {
			'@type': 'Organization',
			name: config.provider.name,
			...(config.provider.url && { url: config.provider.url })
		};
	}

	if (config.price !== undefined) {
		data.offers = {
			'@type': 'Offer',
			price: config.price,
			priceCurrency: config.priceCurrency || 'USD',
			availability: 'https://schema.org/InStock',
			url: config.url
		};
	}

	if (config.aggregateRating) {
		data.aggregateRating = {
			'@type': 'AggregateRating',
			ratingValue: config.aggregateRating.ratingValue,
			reviewCount: config.aggregateRating.reviewCount,
			bestRating: 5,
			worstRating: 1
		};
	}

	return data;
}

/**
 * Generate VideoObject schema
 */
function generateVideo(config: VideoConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'VideoObject',
		name: config.name,
		thumbnailUrl: config.thumbnailUrl,
		uploadDate: config.uploadDate
	};

	if (config.description) data.description = config.description;
	if (config.duration) data.duration = config.duration;
	if (config.contentUrl) data.contentUrl = config.contentUrl;
	if (config.embedUrl) data.embedUrl = config.embedUrl;

	if (config.publisher) {
		data.publisher = {
			'@type': 'Organization',
			name: config.publisher.name,
			logo: {
				'@type': 'ImageObject',
				url: config.publisher.logo
			}
		};
	}

	return data;
}

/**
 * Generate HowTo schema
 */
function generateHowTo(config: HowToConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'HowTo',
		name: config.name,
		step: config.steps.map((step, index) => ({
			'@type': 'HowToStep',
			position: index + 1,
			name: step.name,
			text: step.text,
			...(step.image && { image: step.image }),
			...(step.url && { url: step.url })
		}))
	};

	if (config.description) data.description = config.description;
	if (config.image) data.image = config.image;
	if (config.totalTime) data.totalTime = config.totalTime;
	if (config.supply?.length) {
		data.supply = config.supply.map((s) => ({ '@type': 'HowToSupply', name: s }));
	}
	if (config.tool?.length) {
		data.tool = config.tool.map((t) => ({ '@type': 'HowToTool', name: t }));
	}

	return data;
}

// =============================================================================
// Main Generator
// =============================================================================

/**
 * Generate structured data based on config type
 */
export function generateStructuredData(config: StructuredDataConfig): BaseStructuredData {
	switch (config.type) {
		case 'Organization':
			return generateOrganization(config);
		case 'WebSite':
			return generateWebSite(config);
		case 'Article':
		case 'BlogPosting':
			return generateArticle(config);
		case 'Product':
			return generateProduct(config);
		case 'Course':
			return generateCourse(config);
		case 'FAQPage':
			return generateFAQ(config);
		case 'BreadcrumbList':
			return generateBreadcrumbs(config);
		case 'Person':
			return generatePerson(config);
		case 'Service':
			return generateService(config);
		case 'VideoObject':
			return generateVideo(config);
		case 'HowTo':
			return generateHowTo(config);
		default:
			throw new Error(`Unknown structured data type: ${(config as StructuredDataConfig).type}`);
	}
}

/**
 * Generate multiple structured data items
 */
export function generateMultipleStructuredData(
	configs: StructuredDataConfig[]
): BaseStructuredData[] {
	return configs.map(generateStructuredData);
}

// =============================================================================
// Convenience Functions for Common Patterns
// =============================================================================

/**
 * Generate default site-wide structured data
 */
export function generateSiteStructuredData(siteUrl: string): BaseStructuredData[] {
	return [
		generateStructuredData({
			type: 'Organization',
			name: 'Revolution Trading Pros',
			url: siteUrl,
			logo: `${siteUrl}/images/logo.png`,
			description: 'Professional trading education and live trading rooms',
			email: 'support@revolutiontradingpros.com',
			sameAs: [
				'https://twitter.com/RevTradingPros',
				'https://www.youtube.com/@RevolutionTradingPros',
				'https://www.facebook.com/RevolutionTradingPros'
			]
		}),
		generateStructuredData({
			type: 'WebSite',
			name: 'Revolution Trading Pros',
			url: siteUrl,
			description: 'Professional trading education, alerts, and live trading rooms',
			searchUrl: `${siteUrl}/search`,
			potentialAction: true
		})
	];
}

/**
 * Generate blog post structured data
 */
export function generateBlogPostStructuredData(post: {
	title: string;
	url: string;
	image?: string;
	publishedAt: string;
	modifiedAt?: string;
	authorName: string;
	authorUrl?: string;
	excerpt?: string;
	keywords?: string[];
}): BaseStructuredData {
	return generateStructuredData({
		type: 'BlogPosting',
		headline: post.title,
		url: post.url,
		image: post.image,
		datePublished: post.publishedAt,
		dateModified: post.modifiedAt,
		author: {
			name: post.authorName,
			url: post.authorUrl
		},
		publisher: {
			name: 'Revolution Trading Pros',
			logo: 'https://revolutiontradingpros.com/images/logo.png'
		},
		description: post.excerpt,
		keywords: post.keywords,
		mainEntityOfPage: post.url
	});
}

/**
 * Generate trading service structured data
 */
export function generateTradingServiceStructuredData(service: {
	name: string;
	url: string;
	description?: string;
	price?: number;
	ratingValue?: number;
	reviewCount?: number;
}): BaseStructuredData {
	return generateStructuredData({
		type: 'Service',
		name: service.name,
		url: service.url,
		description: service.description,
		provider: {
			name: 'Revolution Trading Pros',
			url: 'https://revolutiontradingpros.com'
		},
		serviceType: 'Trading Education',
		price: service.price,
		priceCurrency: 'USD',
		aggregateRating: service.ratingValue
			? {
					ratingValue: service.ratingValue,
					reviewCount: service.reviewCount || 0
				}
			: undefined
	});
}
