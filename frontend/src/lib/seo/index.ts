/**
 * SEO Utilities Barrel Export
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Central export for all SEO-related utilities.
 * The unified SEO plugin layer is the primary API surface.
 * Legacy modules (structured-data, image-seo, etc.) are preserved for backward compatibility.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

// ═══════════════════════════════════════════════════════════════════════════════
// UNIFIED SEO PLUGIN LAYER (primary API)
// ═══════════════════════════════════════════════════════════════════════════════
export type {
	SEOInput,
	SEOResolved,
	SEODefaults,
	OpenGraph,
	TwitterCard,
	RobotsDirectives,
	AlternateLink,
	VerificationTags,
	JsonLdNode,
	JsonLdOrganization,
	JsonLdWebSite,
	JsonLdBreadcrumbList,
	JsonLdArticle,
	JsonLdFAQPage,
	RouteSEOContext,
	CanonicalConfig,
	TrailingSlashPolicy,
	Environment
} from './types';

export { seoDefaults } from './defaults';
export { mergeSEO, defaultsToInput } from './merge';
export { normalizeCanonical, buildCanonical } from './canonical';
export { buildRobots, directivesToString, parseRobotsString } from './robots';
export {
	organizationSchema,
	websiteSchema,
	breadcrumbSchema,
	articleSchema,
	faqSchema,
	safeJsonLdSerialize,
	toGraph,
	dedupeJsonLd,
	jsonLdHash
} from './jsonld';
export {
	dedupeHeadTags,
	dedupeAlternates,
	dedupeJsonLdNodes,
	findConflicts,
	titleKey,
	metaNameKey,
	metaPropertyKey,
	canonicalKey,
	alternateKey,
	jsonLdKey
} from './dedupe';
export { resolveSEO, createSEOContext } from './resolve';

// ═══════════════════════════════════════════════════════════════════════════════
// LEGACY MODULES (backward compatibility)
// ═══════════════════════════════════════════════════════════════════════════════
export * from './structured-data';
export * from './image-seo';
export * from './email-reports';
export * from './store-locator';
