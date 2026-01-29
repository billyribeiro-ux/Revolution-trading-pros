/**
 * JSON-LD Structured Data Utilities - Advanced SEO for #1 Rankings
 * =================================================================
 *
 * Enhanced for January 2026 Google Updates including:
 * - E-E-A-T Signals (Expertise, Experience, Authoritativeness, Trust)
 * - Speakable property for Google Assistant voice search
 * - Entity linking with about/mentions properties
 * - @graph support for combining multiple schemas
 * - CollectionPage for blog index pages
 * - Enhanced author credentials and fact-checking claims
 *
 * Comprehensive structured data generation for:
 * - Organization (with authority signals)
 * - WebSite (with SearchAction)
 * - Article / BlogPosting (with E-E-A-T)
 * - Product
 * - Course
 * - FAQPage
 * - BreadcrumbList
 * - Person (Author with credentials)
 * - Service
 * - Review / AggregateRating
 * - HowTo (with tools, supplies, cost)
 * - CollectionPage (for blog listings)
 * - WebPage (with speakable)
 *
 * All schemas follow Schema.org specifications and Google's structured data guidelines.
 *
 * @version 3.0.0 - January 2026 (Advanced SEO Enhancement)
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
	| 'HowTo'
	| 'CollectionPage'
	| 'WebPage';

export interface BaseStructuredData {
	'@context': 'https://schema.org';
	'@type': string;
	'@id'?: string;
	[key: string]: unknown;
}

export interface GraphStructuredData {
	'@context': 'https://schema.org';
	'@graph': BaseStructuredData[];
}

// =============================================================================
// E-E-A-T Enhanced Types
// =============================================================================

export interface AuthorCredential {
	'@type': 'EducationalOccupationalCredential';
	name: string;
	credentialCategory?: string;
	recognizedBy?: {
		'@type': 'Organization';
		name: string;
		url?: string;
	};
}

export interface AuthorExpertise {
	knowsAbout: string[];
	hasCredential?: AuthorCredential[];
	hasOccupation?: {
		'@type': 'Occupation';
		name: string;
		occupationalCategory?: string;
	};
	alumniOf?: {
		'@type': 'Organization' | 'EducationalOrganization';
		name: string;
		url?: string;
	}[];
	memberOf?: {
		'@type': 'Organization';
		name: string;
		url?: string;
	}[];
	award?: string[];
	yearsOfExperience?: number;
}

export interface FactCheckClaim {
	'@type': 'ClaimReview';
	claimReviewed: string;
	reviewRating: {
		'@type': 'Rating';
		ratingValue: number;
		bestRating: number;
		worstRating: number;
		alternateName?: string;
	};
	author: {
		'@type': 'Organization' | 'Person';
		name: string;
	};
}

export interface SpeakableProperty {
	'@type': 'SpeakableSpecification';
	cssSelector?: string[];
	xpath?: string[];
}

export interface EntityReference {
	'@type': string;
	'@id'?: string;
	name: string;
	url?: string;
	sameAs?: string[];
}

// =============================================================================
// Configuration Interfaces
// =============================================================================

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
	// E-E-A-T Authority Signals
	foundingDate?: string;
	founder?: {
		name: string;
		url?: string;
	};
	numberOfEmployees?: number | { minValue: number; maxValue: number };
	areaServed?: string[];
	award?: string[];
	accreditation?: {
		name: string;
		url?: string;
	}[];
	knowsAbout?: string[];
	publishingPrinciples?: string;
	correctionsPolicy?: string;
	ethicsPolicy?: string;
	diversityPolicy?: string;
	masthead?: string;
	ownershipFundingInfo?: string;
}

export interface WebSiteConfig {
	type: 'WebSite';
	name: string;
	url: string;
	description?: string;
	searchUrl?: string;
	potentialAction?: boolean;
	inLanguage?: string;
	copyrightYear?: number;
	copyrightHolder?: {
		name: string;
		url?: string;
	};
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
		// E-E-A-T Author Expertise
		expertise?: AuthorExpertise;
		jobTitle?: string;
		description?: string;
		sameAs?: string[];
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
	// Advanced SEO Properties
	speakable?: SpeakableProperty;
	about?: EntityReference[];
	mentions?: EntityReference[];
	citation?: {
		'@type': 'CreativeWork' | 'Article' | 'WebPage';
		name: string;
		url: string;
	}[];
	isAccessibleForFree?: boolean;
	isPartOf?: {
		'@type': 'Blog' | 'WebSite';
		name: string;
		url: string;
	};
	inLanguage?: string;
	copyrightYear?: number;
	copyrightHolder?: {
		name: string;
		url?: string;
	};
	// Fact-checking claims
	claimReviewed?: FactCheckClaim[];
	// FAQ content for @graph combination
	faqContent?: {
		question: string;
		answer: string;
	}[];
	// Review/rating for the article topic
	reviewRating?: {
		ratingValue: number;
		bestRating?: number;
		worstRating?: number;
	};
	// Article section and position
	articleSection?: string;
	position?: number;
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
		expertise?: AuthorExpertise;
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
	syllabusSections?: string[];
	teaches?: string[];
	assesses?: string[];
	competencyRequired?: string[];
}

export interface FAQConfig {
	type: 'FAQPage';
	questions: {
		question: string;
		answer: string;
	}[];
	mainEntity?: string;
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
	// E-E-A-T Enhanced Properties
	expertise?: AuthorExpertise;
	worksFor?: {
		name: string;
		url?: string;
	};
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
	// Enhanced for 2026
	hasPart?: {
		'@type': 'Clip';
		name: string;
		startOffset: number;
		endOffset: number;
		url?: string;
	}[];
	transcript?: string;
}

export interface HowToConfig {
	type: 'HowTo';
	name: string;
	description?: string;
	image?: string | string[];
	totalTime?: string;
	prepTime?: string;
	performTime?: string;
	supply?: {
		name: string;
		url?: string;
		image?: string;
	}[];
	tool?: {
		name: string;
		url?: string;
		image?: string;
	}[];
	steps: {
		name: string;
		text: string;
		image?: string;
		url?: string;
		// Enhanced step properties
		video?: {
			name: string;
			contentUrl: string;
			thumbnailUrl?: string;
		};
		itemListElement?: {
			name: string;
			text: string;
		}[];
	}[];
	// Enhanced HowTo properties
	estimatedCost?: {
		currency: string;
		value: number | string;
	};
	yield?: string;
	keywords?: string[];
	video?: {
		name: string;
		thumbnailUrl: string;
		contentUrl?: string;
		embedUrl?: string;
		uploadDate: string;
		duration?: string;
	};
}

export interface CollectionPageConfig {
	type: 'CollectionPage';
	name: string;
	url: string;
	description?: string;
	image?: string;
	mainEntity?: {
		'@type': 'ItemList';
		itemListElement: {
			position: number;
			url: string;
			name?: string;
		}[];
	};
	breadcrumb?: {
		name: string;
		url: string;
	}[];
	speakable?: SpeakableProperty;
	inLanguage?: string;
	isPartOf?: {
		'@type': 'WebSite';
		name: string;
		url: string;
	};
	about?: EntityReference[];
	primaryImageOfPage?: {
		'@type': 'ImageObject';
		url: string;
		width?: number;
		height?: number;
	};
	lastReviewed?: string;
}

export interface WebPageConfig {
	type: 'WebPage';
	name: string;
	url: string;
	description?: string;
	speakable?: SpeakableProperty;
	breadcrumb?: {
		name: string;
		url: string;
	}[];
	primaryImageOfPage?: string;
	datePublished?: string;
	dateModified?: string;
	inLanguage?: string;
	isPartOf?: {
		'@type': 'WebSite';
		name: string;
		url: string;
	};
	about?: EntityReference[];
	mentions?: EntityReference[];
	mainContentOfPage?: {
		'@type': 'WebPageElement';
		cssSelector?: string;
	};
	significantLink?: string[];
	relatedLink?: string[];
	lastReviewed?: string;
	reviewedBy?: {
		'@type': 'Person' | 'Organization';
		name: string;
		url?: string;
	};
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
	| HowToConfig
	| CollectionPageConfig
	| WebPageConfig;

// =============================================================================
// Generator Functions
// =============================================================================

/**
 * Generate Organization schema with E-E-A-T authority signals
 */
function generateOrganization(config: OrganizationConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'Organization',
		'@id': `${config.url}/#organization`,
		name: config.name,
		url: config.url
	};

	if (config.logo) {
		data.logo = {
			'@type': 'ImageObject',
			'@id': `${config.url}/#logo`,
			url: config.logo,
			contentUrl: config.logo,
			caption: config.name
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

	// E-E-A-T Authority Signals
	if (config.foundingDate) data.foundingDate = config.foundingDate;
	if (config.founder) {
		data.founder = {
			'@type': 'Person',
			name: config.founder.name,
			...(config.founder.url && { url: config.founder.url })
		};
	}
	if (config.numberOfEmployees) {
		data.numberOfEmployees =
			typeof config.numberOfEmployees === 'number'
				? { '@type': 'QuantitativeValue', value: config.numberOfEmployees }
				: {
						'@type': 'QuantitativeValue',
						minValue: config.numberOfEmployees.minValue,
						maxValue: config.numberOfEmployees.maxValue
					};
	}
	if (config.areaServed?.length) {
		data.areaServed = config.areaServed.map((area) => ({
			'@type': 'Country',
			name: area
		}));
	}
	if (config.award?.length) data.award = config.award;
	if (config.accreditation?.length) {
		data.hasCredential = config.accreditation.map((acc) => ({
			'@type': 'EducationalOccupationalCredential',
			name: acc.name,
			...(acc.url && { url: acc.url })
		}));
	}
	if (config.knowsAbout?.length) data.knowsAbout = config.knowsAbout;

	// Editorial/Trust Policies
	if (config.publishingPrinciples) data.publishingPrinciples = config.publishingPrinciples;
	if (config.correctionsPolicy) data.correctionsPolicy = config.correctionsPolicy;
	if (config.ethicsPolicy) data.ethicsPolicy = config.ethicsPolicy;
	if (config.diversityPolicy) data.diversityPolicy = config.diversityPolicy;
	if (config.masthead) data.masthead = config.masthead;
	if (config.ownershipFundingInfo) data.ownershipFundingInfo = config.ownershipFundingInfo;

	return data;
}

/**
 * Generate WebSite schema with optional SearchAction
 */
function generateWebSite(config: WebSiteConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'WebSite',
		'@id': `${config.url}/#website`,
		name: config.name,
		url: config.url
	};

	if (config.description) data.description = config.description;
	if (config.inLanguage) data.inLanguage = config.inLanguage;

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

	if (config.copyrightYear) data.copyrightYear = config.copyrightYear;
	if (config.copyrightHolder) {
		data.copyrightHolder = {
			'@type': 'Organization',
			name: config.copyrightHolder.name,
			...(config.copyrightHolder.url && { url: config.copyrightHolder.url })
		};
	}

	// Link to Organization
	data.publisher = {
		'@id': `${config.url}/#organization`
	};

	return data;
}

/**
 * Generate Article or BlogPosting schema with E-E-A-T signals
 */
function generateArticle(config: ArticleConfig): BaseStructuredData {
	const articleId = `${config.url}/#article`;

	// Build enhanced author object with E-E-A-T signals
	const authorData: Record<string, unknown> = {
		'@type': 'Person',
		'@id': config.author.url ? `${config.author.url}/#person` : undefined,
		name: config.author.name,
		...(config.author.url && { url: config.author.url }),
		...(config.author.image && { image: config.author.image }),
		...(config.author.jobTitle && { jobTitle: config.author.jobTitle }),
		...(config.author.description && { description: config.author.description }),
		...(config.author.sameAs?.length && { sameAs: config.author.sameAs })
	};

	// Add E-E-A-T expertise signals
	if (config.author.expertise) {
		const expertise = config.author.expertise;

		if (expertise.knowsAbout?.length) {
			authorData.knowsAbout = expertise.knowsAbout;
		}

		if (expertise.hasCredential?.length) {
			authorData.hasCredential = expertise.hasCredential.map((cred) => ({
				'@type': 'EducationalOccupationalCredential',
				name: cred.name,
				...(cred.credentialCategory && { credentialCategory: cred.credentialCategory }),
				...(cred.recognizedBy && {
					recognizedBy: {
						'@type': 'Organization',
						name: cred.recognizedBy.name,
						...(cred.recognizedBy.url && { url: cred.recognizedBy.url })
					}
				})
			}));
		}

		if (expertise.hasOccupation) {
			authorData.hasOccupation = {
				'@type': 'Occupation',
				name: expertise.hasOccupation.name,
				...(expertise.hasOccupation.occupationalCategory && {
					occupationalCategory: expertise.hasOccupation.occupationalCategory
				})
			};
		}

		if (expertise.alumniOf?.length) {
			authorData.alumniOf = expertise.alumniOf.map((org) => ({
				'@type': org['@type'] || 'EducationalOrganization',
				name: org.name,
				...(org.url && { url: org.url })
			}));
		}

		if (expertise.memberOf?.length) {
			authorData.memberOf = expertise.memberOf.map((org) => ({
				'@type': 'Organization',
				name: org.name,
				...(org.url && { url: org.url })
			}));
		}

		if (expertise.award?.length) {
			authorData.award = expertise.award;
		}
	}

	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': config.type,
		'@id': articleId,
		headline: config.headline,
		url: config.url,
		datePublished: config.datePublished,
		author: authorData
	};

	if (config.dateModified) data.dateModified = config.dateModified;
	if (config.description) data.description = config.description;
	if (config.articleBody) data.articleBody = config.articleBody;
	if (config.wordCount) data.wordCount = config.wordCount;
	if (config.keywords?.length) data.keywords = config.keywords.join(', ');
	if (config.articleSection) data.articleSection = config.articleSection;
	if (config.inLanguage) data.inLanguage = config.inLanguage;
	if (config.isAccessibleForFree !== undefined)
		data.isAccessibleForFree = config.isAccessibleForFree;
	if (config.copyrightYear) data.copyrightYear = config.copyrightYear;
	if (config.position) data.position = config.position;

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
			'@id': config.publisher.url ? `${config.publisher.url}/#organization` : undefined,
			name: config.publisher.name,
			logo: {
				'@type': 'ImageObject',
				url: config.publisher.logo
			},
			...(config.publisher.url && { url: config.publisher.url })
		};
	}

	if (config.copyrightHolder) {
		data.copyrightHolder = {
			'@type': 'Organization',
			name: config.copyrightHolder.name,
			...(config.copyrightHolder.url && { url: config.copyrightHolder.url })
		};
	}

	// Speakable property for Google Assistant
	if (config.speakable) {
		data.speakable = {
			'@type': 'SpeakableSpecification',
			...(config.speakable.cssSelector?.length && { cssSelector: config.speakable.cssSelector }),
			...(config.speakable.xpath?.length && { xpath: config.speakable.xpath })
		};
	}

	// About property for topic linking
	if (config.about?.length) {
		data.about = config.about.map((entity) => ({
			'@type': entity['@type'] || 'Thing',
			name: entity.name,
			...(entity['@id'] && { '@id': entity['@id'] }),
			...(entity.url && { url: entity.url }),
			...(entity.sameAs?.length && { sameAs: entity.sameAs })
		}));
	}

	// Mentions property for entity linking
	if (config.mentions?.length) {
		data.mentions = config.mentions.map((entity) => ({
			'@type': entity['@type'] || 'Thing',
			name: entity.name,
			...(entity['@id'] && { '@id': entity['@id'] }),
			...(entity.url && { url: entity.url }),
			...(entity.sameAs?.length && { sameAs: entity.sameAs })
		}));
	}

	// Citations for credibility
	if (config.citation?.length) {
		data.citation = config.citation.map((cite) => ({
			'@type': cite['@type'],
			name: cite.name,
			url: cite.url
		}));
	}

	// Is part of (Blog or WebSite)
	if (config.isPartOf) {
		data.isPartOf = {
			'@type': config.isPartOf['@type'],
			'@id': `${config.isPartOf.url}/#${config.isPartOf['@type'].toLowerCase()}`,
			name: config.isPartOf.name,
			url: config.isPartOf.url
		};
	}

	// Review rating for the article topic
	if (config.reviewRating) {
		data.reviewRating = {
			'@type': 'Rating',
			ratingValue: config.reviewRating.ratingValue,
			bestRating: config.reviewRating.bestRating || 5,
			worstRating: config.reviewRating.worstRating || 1
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
		'@id': `${config.url}/#product`,
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
 * Generate Course schema with enhanced instructor expertise
 */
function generateCourse(config: CourseConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'Course',
		'@id': `${config.url}/#course`,
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
		const instructorData: Record<string, unknown> = {
			'@type': 'Person',
			name: config.instructor.name,
			...(config.instructor.url && { url: config.instructor.url }),
			...(config.instructor.image && { image: config.instructor.image })
		};

		// Add instructor expertise
		if (config.instructor.expertise) {
			if (config.instructor.expertise.knowsAbout?.length) {
				instructorData.knowsAbout = config.instructor.expertise.knowsAbout;
			}
			if (config.instructor.expertise.hasCredential?.length) {
				instructorData.hasCredential = config.instructor.expertise.hasCredential;
			}
		}

		data.instructor = instructorData;
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

	// Enhanced course properties
	if (config.teaches?.length) data.teaches = config.teaches;
	if (config.assesses?.length) data.assesses = config.assesses;
	if (config.competencyRequired?.length) data.competencyRequired = config.competencyRequired;
	if (config.syllabusSections?.length) {
		data.syllabusSections = config.syllabusSections;
	}

	return data;
}

/**
 * Generate FAQPage schema
 */
function generateFAQ(config: FAQConfig): BaseStructuredData {
	const data: BaseStructuredData = {
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

	if (config.mainEntity) {
		data['@id'] = config.mainEntity;
	}

	return data;
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
 * Generate Person schema with E-E-A-T expertise
 */
function generatePerson(config: PersonConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'Person',
		'@id': config.url ? `${config.url}/#person` : undefined,
		name: config.name
	};

	if (config.url) data.url = config.url;
	if (config.image) data.image = config.image;
	if (config.description) data.description = config.description;
	if (config.jobTitle) data.jobTitle = config.jobTitle;
	if (config.email) data.email = config.email;
	if (config.sameAs?.length) data.sameAs = config.sameAs;

	if (config.worksFor) {
		data.worksFor = {
			'@type': 'Organization',
			name: config.worksFor.name,
			...(config.worksFor.url && { url: config.worksFor.url })
		};
	}

	// E-E-A-T Expertise
	if (config.expertise) {
		if (config.expertise.knowsAbout?.length) {
			data.knowsAbout = config.expertise.knowsAbout;
		}

		if (config.expertise.hasCredential?.length) {
			data.hasCredential = config.expertise.hasCredential.map((cred) => ({
				'@type': 'EducationalOccupationalCredential',
				name: cred.name,
				...(cred.credentialCategory && { credentialCategory: cred.credentialCategory }),
				...(cred.recognizedBy && {
					recognizedBy: {
						'@type': 'Organization',
						name: cred.recognizedBy.name,
						...(cred.recognizedBy.url && { url: cred.recognizedBy.url })
					}
				})
			}));
		}

		if (config.expertise.hasOccupation) {
			data.hasOccupation = {
				'@type': 'Occupation',
				name: config.expertise.hasOccupation.name,
				...(config.expertise.hasOccupation.occupationalCategory && {
					occupationalCategory: config.expertise.hasOccupation.occupationalCategory
				})
			};
		}

		if (config.expertise.alumniOf?.length) {
			data.alumniOf = config.expertise.alumniOf.map((org) => ({
				'@type': org['@type'] || 'EducationalOrganization',
				name: org.name,
				...(org.url && { url: org.url })
			}));
		}

		if (config.expertise.memberOf?.length) {
			data.memberOf = config.expertise.memberOf.map((org) => ({
				'@type': 'Organization',
				name: org.name,
				...(org.url && { url: org.url })
			}));
		}

		if (config.expertise.award?.length) {
			data.award = config.expertise.award;
		}
	}

	return data;
}

/**
 * Generate Service schema
 */
function generateService(config: ServiceConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'Service',
		'@id': `${config.url}/#service`,
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
 * Generate VideoObject schema with enhanced features
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
	if (config.transcript) data.transcript = config.transcript;

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

	// Video clips for key moments
	if (config.hasPart?.length) {
		data.hasPart = config.hasPart.map((clip) => ({
			'@type': 'Clip',
			name: clip.name,
			startOffset: clip.startOffset,
			endOffset: clip.endOffset,
			...(clip.url && { url: clip.url })
		}));
	}

	return data;
}

/**
 * Generate HowTo schema with enhanced properties
 */
function generateHowTo(config: HowToConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'HowTo',
		name: config.name,
		step: config.steps.map((step, index) => {
			const stepData: Record<string, unknown> = {
				'@type': 'HowToStep',
				position: index + 1,
				name: step.name,
				text: step.text,
				...(step.image && { image: step.image }),
				...(step.url && { url: step.url })
			};

			// Video for the step
			if (step.video) {
				stepData.video = {
					'@type': 'VideoObject',
					name: step.video.name,
					contentUrl: step.video.contentUrl,
					...(step.video.thumbnailUrl && { thumbnailUrl: step.video.thumbnailUrl })
				};
			}

			// Sub-steps within a step
			if (step.itemListElement?.length) {
				stepData.itemListElement = step.itemListElement.map((subStep, subIndex) => ({
					'@type': 'HowToDirection',
					position: subIndex + 1,
					text: subStep.text
				}));
			}

			return stepData;
		})
	};

	if (config.description) data.description = config.description;

	// Support both string and array for image
	if (config.image) {
		data.image = Array.isArray(config.image) ? config.image : [config.image];
	}

	if (config.totalTime) data.totalTime = config.totalTime;
	if (config.prepTime) data.prepTime = config.prepTime;
	if (config.performTime) data.performTime = config.performTime;
	if (config.keywords?.length) data.keywords = config.keywords.join(', ');
	if (config.yield) data.yield = config.yield;

	// Enhanced supply with URL and image
	if (config.supply?.length) {
		data.supply = config.supply.map((s) => ({
			'@type': 'HowToSupply',
			name: s.name,
			...(s.url && { url: s.url }),
			...(s.image && { image: s.image })
		}));
	}

	// Enhanced tool with URL and image
	if (config.tool?.length) {
		data.tool = config.tool.map((t) => ({
			'@type': 'HowToTool',
			name: t.name,
			...(t.url && { url: t.url }),
			...(t.image && { image: t.image })
		}));
	}

	// Estimated cost
	if (config.estimatedCost) {
		data.estimatedCost = {
			'@type': 'MonetaryAmount',
			currency: config.estimatedCost.currency,
			value: config.estimatedCost.value
		};
	}

	// Video for the entire HowTo
	if (config.video) {
		data.video = {
			'@type': 'VideoObject',
			name: config.video.name,
			thumbnailUrl: config.video.thumbnailUrl,
			uploadDate: config.video.uploadDate,
			...(config.video.contentUrl && { contentUrl: config.video.contentUrl }),
			...(config.video.embedUrl && { embedUrl: config.video.embedUrl }),
			...(config.video.duration && { duration: config.video.duration })
		};
	}

	return data;
}

/**
 * Generate CollectionPage schema for blog index/listing pages
 */
function generateCollectionPage(config: CollectionPageConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'CollectionPage',
		'@id': `${config.url}/#collectionpage`,
		name: config.name,
		url: config.url
	};

	if (config.description) data.description = config.description;
	if (config.image) data.image = config.image;
	if (config.inLanguage) data.inLanguage = config.inLanguage;
	if (config.lastReviewed) data.lastReviewed = config.lastReviewed;

	// Main entity (list of blog posts)
	if (config.mainEntity) {
		data.mainEntity = {
			'@type': 'ItemList',
			itemListElement: config.mainEntity.itemListElement.map((item) => ({
				'@type': 'ListItem',
				position: item.position,
				url: item.url,
				...(item.name && { name: item.name })
			}))
		};
	}

	// Breadcrumb
	if (config.breadcrumb?.length) {
		data.breadcrumb = {
			'@type': 'BreadcrumbList',
			itemListElement: config.breadcrumb.map((item, index) => ({
				'@type': 'ListItem',
				position: index + 1,
				name: item.name,
				item: item.url
			}))
		};
	}

	// Speakable for voice search
	if (config.speakable) {
		data.speakable = {
			'@type': 'SpeakableSpecification',
			...(config.speakable.cssSelector?.length && { cssSelector: config.speakable.cssSelector }),
			...(config.speakable.xpath?.length && { xpath: config.speakable.xpath })
		};
	}

	// Is part of website
	if (config.isPartOf) {
		data.isPartOf = {
			'@type': 'WebSite',
			'@id': `${config.isPartOf.url}/#website`,
			name: config.isPartOf.name,
			url: config.isPartOf.url
		};
	}

	// About entities
	if (config.about?.length) {
		data.about = config.about.map((entity) => ({
			'@type': entity['@type'] || 'Thing',
			name: entity.name,
			...(entity.url && { url: entity.url }),
			...(entity.sameAs?.length && { sameAs: entity.sameAs })
		}));
	}

	// Primary image
	if (config.primaryImageOfPage) {
		data.primaryImageOfPage = {
			'@type': 'ImageObject',
			url: config.primaryImageOfPage.url,
			...(config.primaryImageOfPage.width && { width: config.primaryImageOfPage.width }),
			...(config.primaryImageOfPage.height && { height: config.primaryImageOfPage.height })
		};
	}

	return data;
}

/**
 * Generate WebPage schema with speakable
 */
function generateWebPage(config: WebPageConfig): BaseStructuredData {
	const data: BaseStructuredData = {
		'@context': 'https://schema.org',
		'@type': 'WebPage',
		'@id': `${config.url}/#webpage`,
		name: config.name,
		url: config.url
	};

	if (config.description) data.description = config.description;
	if (config.inLanguage) data.inLanguage = config.inLanguage;
	if (config.datePublished) data.datePublished = config.datePublished;
	if (config.dateModified) data.dateModified = config.dateModified;
	if (config.primaryImageOfPage) data.primaryImageOfPage = config.primaryImageOfPage;
	if (config.lastReviewed) data.lastReviewed = config.lastReviewed;

	// Speakable for voice search
	if (config.speakable) {
		data.speakable = {
			'@type': 'SpeakableSpecification',
			...(config.speakable.cssSelector?.length && { cssSelector: config.speakable.cssSelector }),
			...(config.speakable.xpath?.length && { xpath: config.speakable.xpath })
		};
	}

	// Breadcrumb
	if (config.breadcrumb?.length) {
		data.breadcrumb = {
			'@type': 'BreadcrumbList',
			itemListElement: config.breadcrumb.map((item, index) => ({
				'@type': 'ListItem',
				position: index + 1,
				name: item.name,
				item: item.url
			}))
		};
	}

	// Is part of website
	if (config.isPartOf) {
		data.isPartOf = {
			'@type': 'WebSite',
			'@id': `${config.isPartOf.url}/#website`,
			name: config.isPartOf.name,
			url: config.isPartOf.url
		};
	}

	// About entities
	if (config.about?.length) {
		data.about = config.about.map((entity) => ({
			'@type': entity['@type'] || 'Thing',
			name: entity.name,
			...(entity.url && { url: entity.url }),
			...(entity.sameAs?.length && { sameAs: entity.sameAs })
		}));
	}

	// Mentions entities
	if (config.mentions?.length) {
		data.mentions = config.mentions.map((entity) => ({
			'@type': entity['@type'] || 'Thing',
			name: entity.name,
			...(entity.url && { url: entity.url }),
			...(entity.sameAs?.length && { sameAs: entity.sameAs })
		}));
	}

	// Main content selector
	if (config.mainContentOfPage) {
		data.mainContentOfPage = {
			'@type': 'WebPageElement',
			...(config.mainContentOfPage.cssSelector && {
				cssSelector: config.mainContentOfPage.cssSelector
			})
		};
	}

	// Related links
	if (config.significantLink?.length) data.significantLink = config.significantLink;
	if (config.relatedLink?.length) data.relatedLink = config.relatedLink;

	// Reviewed by (for trust signals)
	if (config.reviewedBy) {
		data.reviewedBy = {
			'@type': config.reviewedBy['@type'],
			name: config.reviewedBy.name,
			...(config.reviewedBy.url && { url: config.reviewedBy.url })
		};
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
		case 'CollectionPage':
			return generateCollectionPage(config);
		case 'WebPage':
			return generateWebPage(config);
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

/**
 * Generate @graph structured data combining multiple schemas
 * Use this for combining BlogPosting with FAQPage, etc.
 */
export function generateGraphStructuredData(configs: StructuredDataConfig[]): GraphStructuredData {
	const schemas = configs.map((config) => {
		const schema = generateStructuredData(config);
		// Remove @context from individual items in graph
		const { '@context': _, ...rest } = schema;
		return rest as BaseStructuredData;
	});

	return {
		'@context': 'https://schema.org',
		'@graph': schemas
	};
}

/**
 * Generate BlogPosting with embedded FAQ using @graph
 * Combines article schema with FAQ schema for posts containing Q&A sections
 */
export function generateBlogPostWithFAQ(
	articleConfig: ArticleConfig,
	faqQuestions: { question: string; answer: string }[]
): GraphStructuredData {
	const configs: StructuredDataConfig[] = [
		articleConfig,
		{
			type: 'FAQPage',
			questions: faqQuestions,
			mainEntity: `${articleConfig.url}/#faqpage`
		}
	];

	return generateGraphStructuredData(configs);
}

/**
 * Generate BlogPosting with HowTo using @graph
 * For tutorial posts that include step-by-step instructions
 */
export function generateBlogPostWithHowTo(
	articleConfig: ArticleConfig,
	howToConfig: Omit<HowToConfig, 'type'>
): GraphStructuredData {
	const configs: StructuredDataConfig[] = [articleConfig, { ...howToConfig, type: 'HowTo' }];

	return generateGraphStructuredData(configs);
}

// =============================================================================
// Convenience Functions for Common Patterns
// =============================================================================

/**
 * Generate default site-wide structured data with E-E-A-T signals
 */
export function generateSiteStructuredData(siteUrl: string): GraphStructuredData {
	return generateGraphStructuredData([
		{
			type: 'Organization',
			name: 'Revolution Trading Pros',
			url: siteUrl,
			logo: `${siteUrl}/images/logo.png`,
			description: 'Professional trading education and live trading rooms',
			email: 'support@revolutiontradingpros.com',
			foundingDate: '2020-01-01',
			numberOfEmployees: { minValue: 10, maxValue: 50 },
			areaServed: ['United States', 'Canada', 'United Kingdom', 'Australia'],
			knowsAbout: [
				'Day Trading',
				'Stock Trading',
				'Options Trading',
				'Futures Trading',
				'Technical Analysis',
				'Trading Psychology',
				'Risk Management'
			],
			sameAs: [
				'https://twitter.com/RevTradingPros',
				'https://www.youtube.com/@RevolutionTradingPros',
				'https://www.facebook.com/RevolutionTradingPros',
				'https://www.linkedin.com/company/revolution-trading-pros'
			],
			publishingPrinciples: `${siteUrl}/about/editorial-policy`,
			correctionsPolicy: `${siteUrl}/about/corrections-policy`,
			ethicsPolicy: `${siteUrl}/about/ethics-policy`
		},
		{
			type: 'WebSite',
			name: 'Revolution Trading Pros',
			url: siteUrl,
			description: 'Professional trading education, alerts, and live trading rooms',
			searchUrl: `${siteUrl}/search`,
			potentialAction: true,
			inLanguage: 'en-US',
			copyrightYear: new Date().getFullYear(),
			copyrightHolder: {
				name: 'Revolution Trading Pros',
				url: siteUrl
			}
		}
	]);
}

/**
 * Generate enhanced blog post structured data with E-E-A-T signals
 */
export function generateBlogPostStructuredData(post: {
	title: string;
	url: string;
	image?: string;
	publishedAt: string;
	modifiedAt?: string;
	authorName: string;
	authorUrl?: string;
	authorImage?: string;
	authorJobTitle?: string;
	authorExpertise?: AuthorExpertise;
	excerpt?: string;
	keywords?: string[];
	articleSection?: string;
	wordCount?: number;
	// For voice search
	speakableSelectors?: string[];
	// For entity linking
	aboutTopics?: EntityReference[];
	mentions?: EntityReference[];
	// For FAQ combination
	faqContent?: { question: string; answer: string }[];
}): BaseStructuredData | GraphStructuredData {
	const siteUrl = 'https://revolutiontradingpros.com';

	const articleConfig: ArticleConfig = {
		type: 'BlogPosting',
		headline: post.title,
		url: post.url,
		image: post.image,
		datePublished: post.publishedAt,
		dateModified: post.modifiedAt,
		author: {
			name: post.authorName,
			url: post.authorUrl,
			image: post.authorImage,
			jobTitle: post.authorJobTitle,
			expertise: post.authorExpertise,
			sameAs: post.authorUrl ? [post.authorUrl] : undefined
		},
		publisher: {
			name: 'Revolution Trading Pros',
			logo: `${siteUrl}/images/logo.png`,
			url: siteUrl
		},
		description: post.excerpt,
		keywords: post.keywords,
		mainEntityOfPage: post.url,
		articleSection: post.articleSection,
		wordCount: post.wordCount,
		inLanguage: 'en-US',
		isAccessibleForFree: true,
		isPartOf: {
			'@type': 'Blog',
			name: 'Revolution Trading Pros Blog',
			url: `${siteUrl}/blog`
		},
		// Speakable for Google Assistant
		speakable: post.speakableSelectors?.length
			? { '@type': 'SpeakableSpecification', cssSelector: post.speakableSelectors }
			: {
					'@type': 'SpeakableSpecification',
					cssSelector: ['.article-headline', '.article-summary']
				},
		// Entity linking
		about: post.aboutTopics,
		mentions: post.mentions
	};

	// If FAQ content is provided, combine with @graph
	if (post.faqContent?.length) {
		return generateBlogPostWithFAQ(articleConfig, post.faqContent);
	}

	return generateStructuredData(articleConfig);
}

/**
 * Generate blog index/listing page structured data
 */
export function generateBlogIndexStructuredData(config: {
	url: string;
	name?: string;
	description?: string;
	posts: { url: string; name: string }[];
	currentPage?: number;
	totalPages?: number;
}): BaseStructuredData {
	const siteUrl = 'https://revolutiontradingpros.com';

	return generateStructuredData({
		type: 'CollectionPage',
		name: config.name || 'Trading Blog - Revolution Trading Pros',
		url: config.url,
		description:
			config.description ||
			'Expert trading insights, strategies, and market analysis from professional traders.',
		inLanguage: 'en-US',
		mainEntity: {
			'@type': 'ItemList',
			itemListElement: config.posts.map((post, index) => ({
				position: index + 1,
				url: post.url,
				name: post.name
			}))
		},
		breadcrumb: [
			{ name: 'Home', url: siteUrl },
			{ name: 'Blog', url: `${siteUrl}/blog` }
		],
		isPartOf: {
			'@type': 'WebSite',
			name: 'Revolution Trading Pros',
			url: siteUrl
		},
		about: [
			{
				'@type': 'Thing',
				name: 'Stock Trading',
				sameAs: ['https://en.wikipedia.org/wiki/Stock_trader']
			},
			{
				'@type': 'Thing',
				name: 'Technical Analysis',
				sameAs: ['https://en.wikipedia.org/wiki/Technical_analysis']
			}
		],
		speakable: {
			'@type': 'SpeakableSpecification',
			cssSelector: ['.page-title', '.page-description']
		}
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

/**
 * Generate HowTo structured data for trading tutorials
 */
export function generateTradingTutorialStructuredData(tutorial: {
	name: string;
	description?: string;
	image?: string | string[];
	totalTime?: string;
	steps: {
		name: string;
		text: string;
		image?: string;
	}[];
	tools?: { name: string; url?: string }[];
	supplies?: { name: string; url?: string }[];
	estimatedCost?: { currency: string; value: number | string };
}): BaseStructuredData {
	return generateStructuredData({
		type: 'HowTo',
		name: tutorial.name,
		description: tutorial.description,
		image: tutorial.image,
		totalTime: tutorial.totalTime,
		steps: tutorial.steps,
		tool: tutorial.tools,
		supply: tutorial.supplies,
		estimatedCost: tutorial.estimatedCost,
		keywords: ['trading', 'tutorial', 'how to trade', 'trading education']
	});
}

/**
 * Generate author/expert structured data with full E-E-A-T signals
 */
export function generateAuthorStructuredData(author: {
	name: string;
	url?: string;
	image?: string;
	jobTitle?: string;
	description?: string;
	expertise: string[];
	credentials?: {
		name: string;
		category?: string;
		issuer?: { name: string; url?: string };
	}[];
	education?: { name: string; url?: string }[];
	memberships?: { name: string; url?: string }[];
	awards?: string[];
	socialProfiles?: string[];
}): BaseStructuredData {
	return generateStructuredData({
		type: 'Person',
		name: author.name,
		url: author.url,
		image: author.image,
		jobTitle: author.jobTitle,
		description: author.description,
		sameAs: author.socialProfiles,
		worksFor: {
			name: 'Revolution Trading Pros',
			url: 'https://revolutiontradingpros.com'
		},
		expertise: {
			knowsAbout: author.expertise,
			hasCredential: author.credentials?.map((cred) => ({
				'@type': 'EducationalOccupationalCredential',
				name: cred.name,
				credentialCategory: cred.category,
				recognizedBy: cred.issuer
					? {
							'@type': 'Organization',
							name: cred.issuer.name,
							url: cred.issuer.url
						}
					: undefined
			})),
			alumniOf: author.education?.map((edu) => ({
				'@type': 'EducationalOrganization' as const,
				name: edu.name,
				url: edu.url
			})),
			memberOf: author.memberships?.map((mem) => ({
				'@type': 'Organization',
				name: mem.name,
				url: mem.url
			})),
			award: author.awards
		}
	});
}

// =============================================================================
// Utility Functions
// =============================================================================

/**
 * Convert structured data to JSON-LD script tag content
 */
export function toJsonLdScript(data: BaseStructuredData | GraphStructuredData): string {
	return JSON.stringify(data, null, 0);
}

/**
 * Create a complete script tag for embedding in HTML
 */
export function createJsonLdScriptTag(data: BaseStructuredData | GraphStructuredData): string {
	return `<script type="application/ld+json">${toJsonLdScript(data)}</script>`;
}

/**
 * Validate that required fields are present (basic validation)
 */
export function validateStructuredData(data: BaseStructuredData): {
	valid: boolean;
	errors: string[];
} {
	const errors: string[] = [];

	if (!data['@context']) {
		errors.push('Missing @context');
	}
	if (!data['@type']) {
		errors.push('Missing @type');
	}

	return {
		valid: errors.length === 0,
		errors
	};
}
