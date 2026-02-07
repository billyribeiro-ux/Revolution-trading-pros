<script lang="ts">
	/**
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
	const structuredDataJson = $derived.by(() => {
		if (!structuredData) return null;
		const configs = Array.isArray(structuredData) ? structuredData : [structuredData];
		return configs.map((config) => generateStructuredData(config));
	});
</script>

<svelte:head>
	<!-- Primary Meta Tags -->
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
		{#each articleTags as tag}
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

	<!-- Structured Data / JSON-LD -->
	{#if structuredDataJson}
		{#each structuredDataJson as jsonLd}
			<!-- eslint-disable-next-line -->
			{@html `<script type="application/ld+json">${JSON.stringify(jsonLd)}</script>`}
		{/each}
	{/if}
</svelte:head>
