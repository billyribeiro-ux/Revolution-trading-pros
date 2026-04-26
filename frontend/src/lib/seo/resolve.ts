/**
 * SEO Plugin Layer - Resolution Pipeline
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Orchestrates the full SEO resolution:
 * 1. Merge layers (defaults → layout → page)
 * 2. Normalize canonical
 * 3. Build robots directives
 * 4. Normalize/dedupe JSON-LD
 * 5. Dedupe head tags
 * 6. Return final SEOResolved ready to render
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

import type { SEOInput, SEODefaults, SEOResolved, RouteSEOContext } from './types';
import { mergeSEO, defaultsToInput } from './merge';
import { normalizeCanonical } from './canonical';
import { buildRobots } from './robots';
import { dedupeJsonLd } from './jsonld';

/**
 * Apply title template to a raw title.
 * Template uses %s as placeholder for the page title.
 */
function applyTitleTemplate(title: string, template: string | null | undefined): string {
	if (!template) return title;
	if (template === '%s') return title;
	return template.replace('%s', title);
}

/**
 * Resolve SEO for a given route context.
 *
 * @param context - Route context (pathname, env, flags)
 * @param defaults - Site-wide SEO defaults
 * @param overrides - Additional SEO input layers (layout, page, etc.)
 * @returns Fully resolved, deduplicated SEO payload ready for rendering
 */
export function resolveSEO(
	context: RouteSEOContext,
	defaults: SEODefaults,
	...overrides: (SEOInput | undefined | null)[]
): SEOResolved {
	// Step 1: Merge all layers
	const baseInput = defaultsToInput(defaults);
	const merged = mergeSEO(baseInput, ...overrides);

	// Step 2: Resolve title with template
	const rawTitle = merged.title ?? defaults.defaultTitle;
	const titleTemplate = merged.titleTemplate ?? defaults.titleTemplate;
	// If the raw title IS the default title (homepage), don't template it
	const title =
		rawTitle === defaults.defaultTitle ? rawTitle : applyTitleTemplate(rawTitle, titleTemplate);

	// Step 3: Resolve description
	const description = merged.description ?? defaults.defaultDescription;

	// Step 4: Normalize canonical
	const rawCanonical = merged.canonical ?? context.pathname;
	const canonical = normalizeCanonical(rawCanonical, defaults.canonical);

	// Step 5: Build robots
	const { directives: robotsDirectives, content: robotsContent } = buildRobots(
		context,
		merged.robots,
		defaults
	);

	// Step 6: Normalize/dedupe JSON-LD
	const jsonldNodes = merged.jsonld ?? [];
	const jsonld = dedupeJsonLd(jsonldNodes);

	// Step 7: Resolve OG
	// If OG title/description still match the defaults, use the resolved page title/description
	// so that page-level title overrides cascade into OG automatically.
	const ogTitleIsDefault = merged.og?.title === defaults.defaultTitle;
	const ogDescIsDefault = merged.og?.description === defaults.defaultDescription;
	const og = {
		title: ogTitleIsDefault ? title : (merged.og?.title ?? title),
		description: ogDescIsDefault ? description : (merged.og?.description ?? description),
		url: merged.og?.url ?? canonical,
		type: merged.og?.type ?? 'website',
		siteName: merged.og?.siteName ?? defaults.siteName,
		locale: merged.og?.locale ?? defaults.defaultLocale,
		image: merged.og?.image ?? defaults.defaultImage,
		imageAlt: merged.og?.imageAlt ?? defaults.defaultImageAlt,
		imageWidth: merged.og?.imageWidth ?? undefined,
		imageHeight: merged.og?.imageHeight ?? undefined,
		imageType: merged.og?.imageType ?? undefined,
		article: merged.og?.article ?? undefined
	};

	// Step 8: Resolve Twitter
	// Same cascade logic: if twitter title/description match defaults, use resolved values.
	const twTitleIsDefault = merged.twitter?.title === defaults.defaultTitle;
	const twDescIsDefault = merged.twitter?.description === defaults.defaultDescription;
	const twitter = {
		card: merged.twitter?.card ?? defaults.twitterCard ?? 'summary_large_image',
		title: twTitleIsDefault ? title : (merged.twitter?.title ?? title),
		description: twDescIsDefault ? description : (merged.twitter?.description ?? description),
		site: merged.twitter?.site ?? defaults.twitterSite,
		creator: merged.twitter?.creator ?? undefined,
		image: merged.twitter?.image ?? defaults.defaultImage,
		imageAlt: merged.twitter?.imageAlt ?? defaults.defaultImageAlt
	};

	// Step 9: Resolve alternates (already deduped by merge)
	const alternates = merged.alternates ?? [];

	// Step 10: Resolve verification
	const verification = merged.verification ?? defaults.verification;

	return {
		title,
		description,
		canonical,
		robotsContent,
		robotsDirectives,
		og,
		twitter,
		alternates,
		verification,
		jsonld
	};
}

/**
 * Create a RouteSEOContext from common SvelteKit page data.
 * Convenience helper for +layout.server.ts / +page.server.ts.
 */
export function createSEOContext(opts: {
	pathname: string;
	params?: Record<string, string>;
	env?: 'production' | 'staging' | 'development';
	isPrivate?: boolean;
	isSearchPage?: boolean;
	isErrorPage?: boolean;
	locale?: string;
}): RouteSEOContext {
	return {
		pathname: opts.pathname,
		params: opts.params ?? {},
		env: opts.env ?? 'production',
		isPrivate: opts.isPrivate ?? false,
		isSearchPage: opts.isSearchPage ?? false,
		isErrorPage: opts.isErrorPage ?? false,
		locale: opts.locale ?? 'en'
	};
}
