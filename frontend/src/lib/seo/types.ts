/**
 * SEO Plugin Layer - Type System
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Strict TypeScript interfaces for the unified SEO ownership layer.
 * Nullable semantics: undefined = inherit, null = explicitly remove inherited value.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

// ═══════════════════════════════════════════════════════════════════════════════
// OPEN GRAPH
// ═══════════════════════════════════════════════════════════════════════════════

export interface OpenGraph {
	title?: string | null;
	description?: string | null;
	url?: string | null;
	type?: 'website' | 'article' | 'profile' | 'product' | string;
	siteName?: string | null;
	locale?: string | null;
	image?: string | null;
	imageAlt?: string | null;
	imageWidth?: number | null;
	imageHeight?: number | null;
	imageType?: string | null;
	article?: {
		publishedTime?: string;
		modifiedTime?: string;
		author?: string;
		section?: string;
		tags?: string[];
	} | null;
}

// ═══════════════════════════════════════════════════════════════════════════════
// TWITTER CARD
// ═══════════════════════════════════════════════════════════════════════════════

export interface TwitterCard {
	card?: 'summary' | 'summary_large_image' | 'app' | 'player';
	site?: string | null;
	creator?: string | null;
	title?: string | null;
	description?: string | null;
	image?: string | null;
	imageAlt?: string | null;
}

// ═══════════════════════════════════════════════════════════════════════════════
// ROBOTS DIRECTIVES
// ═══════════════════════════════════════════════════════════════════════════════

export interface RobotsDirectives {
	index?: boolean;
	follow?: boolean;
	noarchive?: boolean;
	nosnippet?: boolean;
	noimageindex?: boolean;
	'max-snippet'?: number;
	'max-image-preview'?: 'none' | 'standard' | 'large';
	'max-video-preview'?: number;
}

// ═══════════════════════════════════════════════════════════════════════════════
// ALTERNATE LINKS (hreflang)
// ═══════════════════════════════════════════════════════════════════════════════

export interface AlternateLink {
	hreflang: string;
	href: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// VERIFICATION TAGS
// ═══════════════════════════════════════════════════════════════════════════════

export interface VerificationTags {
	google?: string | null;
	bing?: string | null;
	yandex?: string | null;
	pinterest?: string | null;
}

// ═══════════════════════════════════════════════════════════════════════════════
// JSON-LD NODE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface JsonLdBase {
	'@context'?: string;
	'@type': string;
	'@id'?: string;
}

export interface JsonLdOrganization extends JsonLdBase {
	'@type': 'Organization';
	name: string;
	url: string;
	logo?: string | { '@type': 'ImageObject'; url: string };
	description?: string;
	email?: string;
	telephone?: string;
	address?: {
		'@type': 'PostalAddress';
		streetAddress?: string;
		addressLocality?: string;
		addressRegion?: string;
		postalCode?: string;
		addressCountry?: string;
	};
	sameAs?: string[];
}

export interface JsonLdWebSite extends JsonLdBase {
	'@type': 'WebSite';
	name: string;
	url: string;
	potentialAction?: {
		'@type': 'SearchAction';
		target: {
			'@type': 'EntryPoint';
			urlTemplate: string;
		};
		'query-input': string;
	};
}

export interface JsonLdBreadcrumbItem {
	'@type': 'ListItem';
	position: number;
	name: string;
	item: string;
}

export interface JsonLdBreadcrumbList extends JsonLdBase {
	'@type': 'BreadcrumbList';
	itemListElement: JsonLdBreadcrumbItem[];
}

export interface JsonLdArticle extends JsonLdBase {
	'@type': 'Article' | 'BlogPosting' | 'NewsArticle';
	headline: string;
	description?: string;
	image?: string | string[];
	datePublished: string;
	dateModified?: string;
	author: {
		'@type': 'Person' | 'Organization';
		name: string;
		url?: string;
	};
	publisher: {
		'@type': 'Organization';
		name: string;
		logo?: {
			'@type': 'ImageObject';
			url: string;
		};
	};
	mainEntityOfPage?: {
		'@type': 'WebPage';
		'@id': string;
	};
	wordCount?: number;
	articleSection?: string;
	keywords?: string[];
}

export interface JsonLdFAQQuestion {
	'@type': 'Question';
	name: string;
	acceptedAnswer: {
		'@type': 'Answer';
		text: string;
	};
}

/**
 * @deprecated As of Google Search update May 7, 2026, FAQ rich results no longer
 * appear in Google Search. June 2026 the Rich Results Test will drop support, and
 * August 2026 the Search Console API support is removed. Google limits the
 * remaining FAQPage eligibility to well-known government/health sites; do not
 * add this markup to marketing pages for Google SEO.
 * See: https://developers.google.com/search/docs/appearance/structured-data/faqpage
 */
export interface JsonLdFAQPage extends JsonLdBase {
	'@type': 'FAQPage';
	mainEntity: JsonLdFAQQuestion[];
}

/**
 * SpeakableSpecification — used by Speakable JSON-LD nodes to mark which parts
 * of a page are suitable for voice/generative-AI summarization. Aligns with the
 * May 15 2026 "optimizing for generative AI features" guidance.
 */
export interface JsonLdSpeakableSpec {
	'@type': 'SpeakableSpecification';
	cssSelector?: string[];
	xpath?: string[];
}

export interface JsonLdSpeakable extends JsonLdBase {
	'@type': 'WebPage';
	speakable: JsonLdSpeakableSpec;
	url?: string;
	name?: string;
}

export type JsonLdNode =
	| JsonLdOrganization
	| JsonLdWebSite
	| JsonLdBreadcrumbList
	| JsonLdArticle
	| JsonLdFAQPage
	| JsonLdSpeakable
	| (JsonLdBase & Record<string, unknown>);

// ═══════════════════════════════════════════════════════════════════════════════
// ROUTE SEO CONTEXT
// ═══════════════════════════════════════════════════════════════════════════════

export type Environment = 'production' | 'staging' | 'development';

export interface RouteSEOContext {
	pathname: string;
	params: Record<string, string>;
	env: Environment;
	isPrivate: boolean;
	isSearchPage: boolean;
	isErrorPage: boolean;
	locale: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CANONICAL CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

export type TrailingSlashPolicy = 'always' | 'never' | 'ignore';

export interface CanonicalConfig {
	siteUrl: string;
	forceHttps: boolean;
	trailingSlash: TrailingSlashPolicy;
	queryParamAllowlist: string[];
	queryParamDenylist: string[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// SEO INPUT (what routes provide)
// ═══════════════════════════════════════════════════════════════════════════════

export interface SEOInput {
	title?: string | null;
	titleTemplate?: string | null;
	description?: string | null;
	canonical?: string | null;
	robots?: RobotsDirectives | null;
	og?: OpenGraph | null;
	twitter?: TwitterCard | null;
	alternates?: AlternateLink[] | null;
	verification?: VerificationTags | null;
	jsonld?: JsonLdNode[] | null;
}

// ═══════════════════════════════════════════════════════════════════════════════
// SEO DEFAULTS (site-wide configuration)
// ═══════════════════════════════════════════════════════════════════════════════

export interface SEODefaults {
	siteUrl: string;
	siteName: string;
	titleTemplate: string;
	defaultTitle: string;
	defaultDescription: string;
	defaultImage: string;
	defaultImageAlt: string;
	defaultLocale: string;
	twitterSite: string;
	twitterCard: TwitterCard['card'];
	canonical: CanonicalConfig;
	robots: RobotsDirectives;
	verification: VerificationTags;
	jsonld: JsonLdNode[];
	privatePathPrefixes: string[];
	searchPathPrefixes: string[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// SEO RESOLVED (final output ready for rendering)
// ═══════════════════════════════════════════════════════════════════════════════

export interface SEOResolved {
	title: string;
	description: string;
	canonical: string;
	robotsContent: string;
	robotsDirectives: RobotsDirectives;
	og: Required<Pick<OpenGraph, 'title' | 'description' | 'url' | 'type' | 'siteName' | 'locale'>> &
		Omit<OpenGraph, 'title' | 'description' | 'url' | 'type' | 'siteName' | 'locale'>;
	twitter: Required<Pick<TwitterCard, 'card' | 'title' | 'description'>> &
		Omit<TwitterCard, 'card' | 'title' | 'description'>;
	alternates: AlternateLink[];
	verification: VerificationTags;
	jsonld: JsonLdNode[];
}
