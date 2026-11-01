/**
 * SEO Plugin Layer - Resolution Pipeline
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Orchestrates the full SEO resolution:
 * 1. Merge layers (defaults → layout → page)
 * 2. Normalize title (15-60 chars target, no duplicate brand suffix)
 * 3. Normalize description (50-160 chars target)
 * 4. Normalize canonical & absolute OG/Twitter image URLs
 * 5. Build robots directives
 * 6. Normalize/dedupe JSON-LD (Organization + WebSite on / only; auto BreadcrumbList on non-home)
 * 7. Return final SEOResolved ready to render
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 * @since November 2026
 */

import type { SEOInput, SEODefaults, SEOResolved, RouteSEOContext, JsonLdNode } from './types';
import { mergeSEO, defaultsToInput } from './merge';
import { normalizeCanonical } from './canonical';
import { buildRobots } from './robots';
import { dedupeJsonLd, breadcrumbSchema } from './jsonld';
import { buildBreadcrumbsForPath, isNoindexPath, PRODUCTION_ORIGIN } from './config';

/**
 * Apply title template to a raw title while preventing duplicate brand suffixes
 * and keeping the final `<title>` within 15–60 characters (`META-01`).
 */
function applyTitleTemplate(rawTitle: string, template: string | null | undefined): string {
	// Strip any existing trailing brand suffix from the raw title
	const stripped = rawTitle
		.replace(/\s*[|—–-]\s*Revolution Trading Pros$/i, '')
		.replace(/\s*[|—–-]\s*RTP$/i, '')
		.trim();

	if (!template || template === '%s') {
		return rawTitle;
	}

	const fullTemplated = template.replace('%s', stripped);
	if (fullTemplated.length <= 60) {
		return fullTemplated;
	}

	// If full "| Revolution Trading Pros" pushes title over 60 chars, try "| RTP"
	const shortTemplated = `${stripped} | RTP`;
	if (shortTemplated.length <= 60) {
		return shortTemplated;
	}

	// If stripped itself is <= 60 chars, return stripped directly
	if (stripped.length <= 60) {
		return stripped;
	}

	// Otherwise truncate cleanly at word boundary <= 57 chars + "..."
	const slice = stripped.slice(0, 57);
	const lastSpace = slice.lastIndexOf(' ');
	return `${(lastSpace > 35 ? slice.slice(0, lastSpace) : slice).replace(/[—–\-|,:;\s]+$/, '')}...`;
}

/**
 * Ensure meta description stays within 50–160 characters (`META-02`).
 */
function normalizeDescription(rawDesc: string, fallbackDesc: string): string {
	let desc = rawDesc.trim().replace(/\s+/g, ' ');
	if (desc.length < 50) {
		desc = `${desc} — Institutional trading education & tools by Revolution Trading Pros.`;
	}
	if (desc.length > 160) {
		const slice = desc.slice(0, 157);
		const lastSpace = slice.lastIndexOf(' ');
		desc = `${(lastSpace > 100 ? slice.slice(0, lastSpace) : slice).replace(/[.,;:\-—–\s]+$/, '')}...`;
	}
	return desc || fallbackDesc;
}

/**
 * Ensure an image URL is absolute using `siteUrl`.
 */
function toAbsoluteImageUrl(img: string | undefined | null, siteUrl: string, fallback: string): string {
	if (!img) return fallback;
	if (img.startsWith('http://') || img.startsWith('https://')) {
		// Replace any staging .pages.dev origin with production siteUrl
		return img.replace(/^https?:\/\/revolution-trading-pros\.pages\.dev/i, siteUrl);
	}
	return `${siteUrl.replace(/\/+$/, '')}/${img.replace(/^\/+/, '')}`;
}

/**
 * Resolve SEO for a given route context.
 */
export function resolveSEO(
	context: RouteSEOContext,
	defaults: SEODefaults,
	...overrides: (SEOInput | undefined | null)[]
): SEOResolved {
	const isRoot = context.pathname === '/' || context.pathname === '';

	// Step 1: Merge all layers
	const baseInput = defaultsToInput(defaults);
	// Per SD-04: Organization and WebSite JSON-LD belong on the home page `/` ONLY
	if (!isRoot) {
		baseInput.jsonld = [];
	}
	const merged = mergeSEO(baseInput, ...overrides);

	// Step 2: Resolve title with template
	const crumbs = buildBreadcrumbsForPath(context.pathname);
	const derivedLeafTitle =
		!isRoot && crumbs.length > 0 ? crumbs[crumbs.length - 1]?.name : undefined;
	const rawTitle = merged.title ?? derivedLeafTitle ?? defaults.defaultTitle;
	const titleTemplate = merged.titleTemplate !== undefined ? merged.titleTemplate : defaults.titleTemplate;
	const title =
		rawTitle === defaults.defaultTitle ? rawTitle : applyTitleTemplate(rawTitle, titleTemplate);

	// Step 3: Resolve description
	const rawDescription = merged.description ?? defaults.defaultDescription;
	const description = normalizeDescription(rawDescription, defaults.defaultDescription);

	// Step 4: Normalize canonical (preserving ?page=N for N > 1 per URL-05)
	let rawCanonical = merged.canonical ?? context.pathname;
	const pageParam = context.searchParams?.get('page');
	if (pageParam && Number(pageParam) > 1 && !rawCanonical.includes('?')) {
		rawCanonical = `${rawCanonical}?page=${encodeURIComponent(pageParam)}`;
	}
	const canonical = normalizeCanonical(rawCanonical, defaults.canonical);

	// Step 5: Build robots
	const { directives: robotsDirectives, content: robotsContent } = buildRobots(
		context,
		merged.robots,
		defaults
	);

	// Step 6: Normalize/dedupe JSON-LD
	const jsonldNodes: JsonLdNode[] = [...(merged.jsonld ?? [])];

	// Per SD-05: Every non-home indexable page automatically receives a BreadcrumbList
	// matching the visible <Breadcrumbs /> component in +layout.svelte if not already present.
	if (
		!isRoot &&
		!context.isErrorPage &&
		!context.isPrivate &&
		!isNoindexPath(context.pathname) &&
		robotsDirectives.index !== false
	) {
		const hasBreadcrumb = jsonldNodes.some((n) => n['@type'] === 'BreadcrumbList');
		if (!hasBreadcrumb) {
			const autoCrumbs = buildBreadcrumbsForPath(context.pathname, title);
			if (autoCrumbs.length >= 2) {
				jsonldNodes.push(breadcrumbSchema(autoCrumbs, `${PRODUCTION_ORIGIN}${context.pathname}#breadcrumb`));
			}
		} else {
			// Normalize existing BreadcrumbList item URLs to match buildBreadcrumbsForPath
			const autoCrumbs = buildBreadcrumbsForPath(context.pathname, title);
			if (autoCrumbs.length >= 2) {
				for (let i = 0; i < jsonldNodes.length; i++) {
					if (jsonldNodes[i]['@type'] === 'BreadcrumbList') {
						jsonldNodes[i] = breadcrumbSchema(
							autoCrumbs,
							`${PRODUCTION_ORIGIN}${context.pathname}#breadcrumb`
						);
					}
				}
			}
		}
	}

	const jsonld = dedupeJsonLd(jsonldNodes);

	// Step 7: Resolve OG
	const ogTitleIsDefault = merged.og?.title === defaults.defaultTitle;
	const ogDescIsDefault = merged.og?.description === defaults.defaultDescription;
	const resolvedOgImage = toAbsoluteImageUrl(
		merged.og?.image,
		defaults.siteUrl,
		defaults.defaultImage
	);
	const og = {
		title: ogTitleIsDefault ? title : (merged.og?.title ?? title),
		description: ogDescIsDefault ? description : (merged.og?.description ?? description),
		url: canonical,
		type: merged.og?.type ?? 'website',
		siteName: merged.og?.siteName ?? defaults.siteName,
		locale: merged.og?.locale ?? defaults.defaultLocale,
		image: resolvedOgImage,
		imageAlt: merged.og?.imageAlt ?? defaults.defaultImageAlt,
		imageWidth: merged.og?.imageWidth ?? undefined,
		imageHeight: merged.og?.imageHeight ?? undefined,
		imageType: merged.og?.imageType ?? undefined,
		article: merged.og?.article ?? undefined
	};

	// Step 8: Resolve Twitter
	const twTitleIsDefault = merged.twitter?.title === defaults.defaultTitle;
	const twDescIsDefault = merged.twitter?.description === defaults.defaultDescription;
	const resolvedTwImage = toAbsoluteImageUrl(
		merged.twitter?.image ?? merged.og?.image,
		defaults.siteUrl,
		defaults.defaultImage
	);
	const twitter = {
		card: merged.twitter?.card ?? defaults.twitterCard ?? 'summary_large_image',
		title: twTitleIsDefault ? title : (merged.twitter?.title ?? title),
		description: twDescIsDefault ? description : (merged.twitter?.description ?? description),
		site: merged.twitter?.site ?? defaults.twitterSite,
		creator: merged.twitter?.creator ?? undefined,
		image: resolvedTwImage,
		imageAlt: merged.twitter?.imageAlt ?? defaults.defaultImageAlt
	};

	// Step 9: Resolve alternates
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
