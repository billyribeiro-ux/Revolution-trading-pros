<script lang="ts">
	/**
	 * @deprecated As of May 2026 SEO audit, all pages have been migrated to the
	 * unified SEO layer at `$lib/seo/` (Seo.svelte + resolveSEO + +page.{ts,server.ts}
	 * load returning `{ seo: SEOInput }`). This legacy component is retained only
	 * for archaeological reference and has zero importers (verified by grep).
	 *
	 * Do not import this component. If you find yourself reaching for it, write
	 * an SEO payload in your route's `load()` instead — see frontend/src/routes/
	 * about/+page.ts or blog/[slug]/+page.ts for examples.
	 *
	 * SEO Head Component - Apple ICT 7+ Principal Engineer Grade
	 * ===========================================================
	 *
	 * Comprehensive SEO meta tag management for:
	 * - Standard meta tags (title, description, keywords)
	 * - Open Graph (Facebook, LinkedIn)
	 * - Twitter Cards
	 * - JSON-LD structured data
	 * - Canonical URLs
	 * - Robots directives
	 *
	 * @version 2.0.0 - January 2026
	 */

	import { generateStructuredData, type StructuredDataConfig } from '$lib/utils/structured-data';
	import { serializeJsonLd } from '$lib/seo/serializeJsonLd';
	import { getPageSeoContext } from '$lib/seo/page-seo-context.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		// Basic SEO
		title: string;
		description?: string;
		keywords?: string[];
		canonicalUrl?: string;

		// Robots
		noindex?: boolean;
		nofollow?: boolean;
		noarchive?: boolean;

		// Open Graph
		ogTitle?: string;
		ogDescription?: string;
		ogImage?: string;
		ogImageAlt?: string;
		ogType?: 'website' | 'article' | 'product' | 'profile';
		ogUrl?: string;
		ogSiteName?: string;
		ogLocale?: string;

		// Article-specific OG
		articleAuthor?: string;
		articlePublishedTime?: string;
		articleModifiedTime?: string;
		articleSection?: string;
		articleTags?: string[];

		// Twitter
		twitterCard?: 'summary' | 'summary_large_image' | 'app' | 'player';
		twitterSite?: string;
		twitterCreator?: string;
		twitterTitle?: string;
		twitterDescription?: string;
		twitterImage?: string;
		twitterImageAlt?: string;

		// Structured Data
		structuredData?: StructuredDataConfig | StructuredDataConfig[];

		// Site defaults
		titleSuffix?: string;
		defaultImage?: string;
	}

	let {
		title,
		description = '',
		keywords = [],
		canonicalUrl,
		noindex = false,
		nofollow = false,
		noarchive = false,
		ogTitle,
		ogDescription,
		ogImage,
		ogImageAlt,
		ogType = 'website',
		ogUrl,
		ogSiteName = 'Revolution Trading Pros',
		ogLocale = 'en_US',
		articleAuthor,
		articlePublishedTime,
		articleModifiedTime,
		articleSection,
		articleTags = [],
		twitterCard = 'summary_large_image',
		twitterSite = '@RevTradingPros',
		twitterCreator,
		twitterTitle,
		twitterDescription,
		twitterImage,
		twitterImageAlt,
		structuredData,
		titleSuffix = ' | Revolution Trading Pros',
		defaultImage = 'https://revolutiontradingpros.com/images/og-default.jpg'
	}: Props = $props();

	// ==========================================================================
	// Computed Values
	// ==========================================================================

	const fullTitle = $derived(title ? `${title}${titleSuffix}` : titleSuffix.slice(3));
	const metaDescription = $derived(description?.slice(0, 160) || '');
	const metaKeywords = $derived(keywords.join(', '));

	// Robots meta
	const robotsContent = $derived.by(() => {
		const directives: string[] = [];
		if (noindex) directives.push('noindex');
		if (nofollow) directives.push('nofollow');
		if (noarchive) directives.push('noarchive');
		return directives.length > 0 ? directives.join(', ') : 'index, follow';
	});

	// Open Graph
	const finalOgTitle = $derived(ogTitle || title);
	const finalOgDescription = $derived(ogDescription || description);
	const finalOgImage = $derived(ogImage || defaultImage);
	const finalOgUrl = $derived(ogUrl || canonicalUrl);

	// Twitter
	const finalTwitterTitle = $derived(twitterTitle || finalOgTitle);
	const finalTwitterDescription = $derived(twitterDescription || finalOgDescription);
	const finalTwitterImage = $derived(twitterImage || finalOgImage);

	// Structured Data JSON
	// ═══════════════════════════════════════════════════════════════════════════
	// Unified SEO Context Bridge (SSR-safe — runs during synchronous init)
	// ═══════════════════════════════════════════════════════════════════════════
	// If the layout has created a page-SEO context, we write our computed meta
	// data into it. The layout's <Seo> component then emits the single source
	// of truth, eliminating duplicate meta/canonical/og/twitter tags.
	// If no context (standalone / test), we fall back to emitting ourselves.
	// ═══════════════════════════════════════════════════════════════════════════
	const ctx = getPageSeoContext();
	const hasLayoutSeo = !!ctx;

	// Build SEOInput for the unified layer (only when context is available).
	// We assign a CLOSURE so prop reads happen lazily inside the layout's
	// $derived (reactive context). This eliminates state_referenced_locally.
	// IMPORTANT: pass RAW title (no suffix) so resolveSEO applies titleTemplate once.
	if (ctx) {
		ctx.value = () => ({
			title,
			titleTemplate: titleSuffix,
			description: (description || '').slice(0, 160) || null,
			canonical: canonicalUrl || null,
			robots: {
				index: !noindex,
				follow: !nofollow,
				noarchive: noarchive || undefined
			},
			og: {
				title: ogTitle || title || null,
				description: ogDescription || (description || '').slice(0, 160) || null,
				url: ogUrl || canonicalUrl || null,
				type: ogType,
				siteName: ogSiteName || null,
				locale: ogLocale || null,
				image: ogImage || defaultImage || null,
				imageAlt: ogImageAlt || null,
				imageWidth: 1200,
				imageHeight: 630,
				article:
					ogType === 'article'
						? {
							publishedTime: articlePublishedTime,
							modifiedTime: articleModifiedTime,
							author: articleAuthor,
							section: articleSection,
							tags: articleTags
						}
						: null
			},
			twitter: {
				card: twitterCard,
				site: twitterSite || null,
				creator: twitterCreator || null,
				title: twitterTitle || ogTitle || title || null,
				description: twitterDescription || ogDescription || (description || '').slice(0, 160) || null,
				image: twitterImage || ogImage || defaultImage || null,
				imageAlt: (twitterImageAlt || ogImageAlt) || null
			}
		});
	}

	// ==========================================================================
	// Computed Values
	// ==========================================================================

	const structuredDataJson = $derived.by(() => {
		if (!structuredData) return null;
		const configs = Array.isArray(structuredData) ? structuredData : [structuredData];
		return configs.map((config) => generateStructuredData(config));
	});
</script>

<svelte:head>
	{#if !hasLayoutSeo}
		<!-- ═══ FALLBACK: only emit meta tags when no layout context is present ═══ -->
		<title>{fullTitle}</title>
		{#if metaDescription}
			<meta name="description" content={metaDescription} />
		{/if}
		{#if metaKeywords}
			<meta name="keywords" content={metaKeywords} />
		{/if}

		<!-- Robots -->
		<meta name="robots" content={robotsContent} />

		<!-- Canonical URL -->
		{#if canonicalUrl}
			<link rel="canonical" href={canonicalUrl} />
		{/if}

		<!-- Open Graph / Facebook -->
		<meta property="og:type" content={ogType} />
		<meta property="og:site_name" content={ogSiteName} />
		<meta property="og:locale" content={ogLocale} />
		<meta property="og:title" content={finalOgTitle} />
		{#if finalOgDescription}
			<meta property="og:description" content={finalOgDescription} />
		{/if}
		{#if finalOgUrl}
			<meta property="og:url" content={finalOgUrl} />
		{/if}
		{#if finalOgImage}
			<meta property="og:image" content={finalOgImage} />
			{#if ogImageAlt}
				<meta property="og:image:alt" content={ogImageAlt} />
			{/if}
			<meta property="og:image:width" content="1200" />
			<meta property="og:image:height" content="630" />
		{/if}

		<!-- Article-specific Open Graph -->
		{#if ogType === 'article'}
			{#if articleAuthor}
				<meta property="article:author" content={articleAuthor} />
			{/if}
			{#if articlePublishedTime}
				<meta property="article:published_time" content={articlePublishedTime} />
			{/if}
			{#if articleModifiedTime}
				<meta property="article:modified_time" content={articleModifiedTime} />
			{/if}
			{#if articleSection}
				<meta property="article:section" content={articleSection} />
			{/if}
			{#each articleTags as tag (tag)}
				<meta property="article:tag" content={tag} />
			{/each}
		{/if}

		<!-- Twitter Card -->
		<meta name="twitter:card" content={twitterCard} />
		{#if twitterSite}
			<meta name="twitter:site" content={twitterSite} />
		{/if}
		{#if twitterCreator}
			<meta name="twitter:creator" content={twitterCreator} />
		{/if}
		<meta name="twitter:title" content={finalTwitterTitle} />
		{#if finalTwitterDescription}
			<meta name="twitter:description" content={finalTwitterDescription} />
		{/if}
		{#if finalTwitterImage}
			<meta name="twitter:image" content={finalTwitterImage} />
			{#if twitterImageAlt || ogImageAlt}
				<meta name="twitter:image:alt" content={twitterImageAlt || ogImageAlt} />
			{/if}
		{/if}
	{/if}

	<!-- ═══ JSON-LD: always emitted by SEOHead (multiple scripts are valid) ═══ -->
	{#if structuredDataJson}
		{#each structuredDataJson as jsonLd, i (i)}
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			{@html '<scr' +
				'ipt type="application/ld+json">' +
				serializeJsonLd(jsonLd) +
				'</scr' +
				'ipt>'}
		{/each}
	{/if}
</svelte:head>
