<script lang="ts">
	/**
	 * SEO Plugin Layer - Renderer Component
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Single rendering component for all SEO head tags.
	 * Receives a pre-resolved SEOResolved payload and emits <svelte:head> tags.
	 *
	 * USAGE: Render ONCE at the root layout with merged route data.
	 * Do NOT render in multiple nested layouts — this component is the single
	 * ownership layer for all <head> SEO output.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @since February 2026
	 */

	import type { SEOResolved } from './types';
	import { safeJsonLdSerialize, toGraph } from './jsonld';

	interface Props {
		seo: SEOResolved;
	}

	const { seo }: Props = $props();

	const jsonLdScript = $derived(
		seo.jsonld.length > 0 ? safeJsonLdSerialize(toGraph(seo.jsonld)) : null
	);
</script>

<svelte:head>
	<!-- Title -->
	<title>{seo.title}</title>

	<!-- Meta Description -->
	<meta name="description" content={seo.description} />

	<!-- Canonical -->
	<link rel="canonical" href={seo.canonical} />

	<!-- Robots -->
	<meta name="robots" content={seo.robotsContent} />

	<!-- Open Graph -->
	<meta property="og:title" content={seo.og.title} />
	<meta property="og:description" content={seo.og.description} />
	<meta property="og:url" content={seo.og.url} />
	<meta property="og:type" content={seo.og.type} />
	<meta property="og:site_name" content={seo.og.siteName} />
	<meta property="og:locale" content={seo.og.locale} />

	{#if seo.og.image}
		<meta property="og:image" content={seo.og.image} />
	{/if}
	{#if seo.og.imageAlt}
		<meta property="og:image:alt" content={seo.og.imageAlt} />
	{/if}
	{#if seo.og.imageWidth}
		<meta property="og:image:width" content={String(seo.og.imageWidth)} />
	{/if}
	{#if seo.og.imageHeight}
		<meta property="og:image:height" content={String(seo.og.imageHeight)} />
	{/if}
	{#if seo.og.imageType}
		<meta property="og:image:type" content={seo.og.imageType} />
	{/if}

	<!-- OG Article (if applicable) -->
	{#if seo.og.article}
		{#if seo.og.article.publishedTime}
			<meta property="article:published_time" content={seo.og.article.publishedTime} />
		{/if}
		{#if seo.og.article.modifiedTime}
			<meta property="article:modified_time" content={seo.og.article.modifiedTime} />
		{/if}
		{#if seo.og.article.author}
			<meta property="article:author" content={seo.og.article.author} />
		{/if}
		{#if seo.og.article.section}
			<meta property="article:section" content={seo.og.article.section} />
		{/if}
		{#if seo.og.article.tags}
			{#each seo.og.article.tags as tag}
				<meta property="article:tag" content={tag} />
			{/each}
		{/if}
	{/if}

	<!-- Twitter Card -->
	<meta name="twitter:card" content={seo.twitter.card} />
	<meta name="twitter:title" content={seo.twitter.title} />
	<meta name="twitter:description" content={seo.twitter.description} />

	{#if seo.twitter.site}
		<meta name="twitter:site" content={seo.twitter.site} />
	{/if}
	{#if seo.twitter.creator}
		<meta name="twitter:creator" content={seo.twitter.creator} />
	{/if}
	{#if seo.twitter.image}
		<meta name="twitter:image" content={seo.twitter.image} />
	{/if}
	{#if seo.twitter.imageAlt}
		<meta name="twitter:image:alt" content={seo.twitter.imageAlt} />
	{/if}

	<!-- Alternate Links (hreflang) -->
	{#each seo.alternates as alt}
		<link rel="alternate" hreflang={alt.hreflang} href={alt.href} />
	{/each}

	<!-- Verification Tags -->
	{#if seo.verification.google}
		<meta name="google-site-verification" content={seo.verification.google} />
	{/if}
	{#if seo.verification.bing}
		<meta name="msvalidate.01" content={seo.verification.bing} />
	{/if}
	{#if seo.verification.yandex}
		<meta name="yandex-verification" content={seo.verification.yandex} />
	{/if}
	{#if seo.verification.pinterest}
		<meta name="p:domain_verify" content={seo.verification.pinterest} />
	{/if}

	<!-- JSON-LD Structured Data -->
	{#if jsonLdScript}
		{@html `<script type="application/ld+json">${jsonLdScript}</script>`}
	{/if}
</svelte:head>
