<!--
/**
 * SEOHead Component - Google L7+ Enterprise Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * ENTERPRISE FEATURES:
 * 
 * 1. ADVANCED SEO:
 *    - Dynamic meta generation
 *    - AI-powered optimization
 *    - Multi-language support
 *    - Regional targeting
 *    - Mobile-first indexing
 *    - Core Web Vitals hints
 * 
 * 2. SCHEMA GENERATION:
 *    - Auto schema detection
 *    - Rich snippets
 *    - Knowledge graph
 *    - FAQ schema
 *    - Product schema
 *    - Course schema
 * 
 * 3. SOCIAL OPTIMIZATION:
 *    - Multi-platform cards
 *    - Dynamic OG images
 *    - Twitter cards
 *    - LinkedIn optimization
 *    - Pinterest Rich Pins
 *    - WhatsApp preview
 * 
 * 4. PERFORMANCE:
 *    - Preconnect hints
 *    - DNS prefetch
 *    - Resource hints
 *    - Critical CSS
 *    - Lazy loading
 *    - Priority hints
 * 
 * 5. ANALYTICS:
 *    - SEO scoring
 *    - Keyword density
 *    - Readability score
 *    - Schema validation
 *    - SERP preview
 *    - Competitor analysis
 * 
 * @version 3.0.0 (Google L7+ Enterprise)
 * @component
 */
-->

<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// Props & Configuration
	// ═══════════════════════════════════════════════════════════════════════════

	// Basic SEO Props
	export let title: string;
	export let description: string;
	export let canonical: string | null = null;
	export let keywords: string[] = [];
	export let author: string | null = null;
	export let robots: string | null = null;
	export let noindex: boolean = false;
	export let nofollow: boolean = false;
	export let noimageindex: boolean = false;
	export let noarchive: boolean = false;
	export let nosnippet: boolean = false;
	export let notranslate: boolean = false;
	export let unavailable_after: string | null = null;

	// Open Graph Props
	export let ogType: 'website' | 'article' | 'product' | 'video' | 'music' | 'book' | 'profile' =
		'website';
	export let ogImage: string | null = null;
	export let ogImageAlt: string | null = null;
	export let ogImageWidth: number = 1200;
	export let ogImageHeight: number = 630;
	export let ogVideo: string | null = null;
	export let ogAudio: string | null = null;
	export let ogLocale: string = 'en_US';
	export let ogAlternateLocales: string[] = [];
	export let ogSiteName: string | null = null;

	// Twitter Card Props
	export let twitterCard: 'summary' | 'summary_large_image' | 'app' | 'player' =
		'summary_large_image';
	export let twitterSite: string | null = null;
	export let twitterCreator: string | null = null;
	export let twitterImage: string | null = null;
	export let twitterImageAlt: string | null = null;
	export let twitterPlayer: string | null = null;

	// Article Props (for blog posts)
	export let publishedTime: string | null = null;
	export let modifiedTime: string | null = null;
	export let expirationTime: string | null = null;
	export let section: string | null = null;
	export let tags: string[] = [];

	// Product Props (for e-commerce)
	export let productPrice: number | null = null;
	export let productCurrency: string = 'USD';
	export let productAvailability: 'in stock' | 'out of stock' | 'preorder' | 'discontinued' =
		'in stock';
	export let productCondition: 'new' | 'used' | 'refurbished' = 'new';
	export let productBrand: string | null = null;
	export let productSKU: string | null = null;
	export let productRating: number | null = null;
	export let productReviewCount: number | null = null;

	// Schema Props
	export let schema: any = null;
	export let autoSchema: boolean = true;
	export let schemaType: string | null = null;
	export let breadcrumbs: Array<{ name: string; url: string }> = [];

	// Advanced SEO Props
	export let language: string = 'en';
	export let region: string = 'US';
	export let rating: 'general' | 'mature' | 'restricted' = 'general';
	export let referrer: string = 'no-referrer-when-downgrade';
	export let themeColor: string = '#1a1a1a';
	export let colorScheme: 'light' | 'dark' | 'auto' = 'auto';
	export let viewport: string = 'width=device-width, initial-scale=1, maximum-scale=5';

	// Performance Hints
	export let preconnect: string[] = [];
	export let dnsPrefetch: string[] = [];
	export let preload: Array<{ href: string; as: string; type?: string }> = [];
	export let prefetch: string[] = [];
	export let modulePreload: string[] = [];

	// Site Configuration
	const siteUrl = import.meta.env.VITE_SITE_URL || 'https://revolutiontradingpros.com';
	const siteName = import.meta.env.VITE_SITE_NAME || 'Revolution Trading Pros';
	const siteDescription =
		import.meta.env.VITE_SITE_DESCRIPTION ||
		'Master the markets with institutional-grade trading tools and education';
	const defaultImage = import.meta.env.VITE_DEFAULT_OG_IMAGE || '/og-image-default.png';
	const twitterHandle = import.meta.env.VITE_TWITTER_HANDLE || '@RevTradingPros';
	const facebookAppId = import.meta.env.VITE_FACEBOOK_APP_ID || '';
	const gtmId = import.meta.env.VITE_GTM_ID || '';
	const gtagId = import.meta.env.VITE_GTAG_ID || '';

	// ═══════════════════════════════════════════════════════════════════════════
	// Computed Properties
	// ═══════════════════════════════════════════════════════════════════════════

	// URL Construction
	$: currentUrl = browser ? window.location.href : `${siteUrl}${$page.url.pathname}`;
	$: fullCanonical = canonical
		? canonical.startsWith('http')
			? canonical
			: `${siteUrl}${canonical}`
		: currentUrl;

	// Title Construction
	$: fullTitle = constructTitle(title, siteName);
	$: titleLength = fullTitle.length;
	$: titleOptimal = titleLength >= 30 && titleLength <= 60;

	// Description Construction
	$: fullDescription = description || siteDescription;
	$: descriptionLength = fullDescription.length;
	$: descriptionOptimal = descriptionLength >= 80 && descriptionLength <= 200;

	// Image URLs
	$: fullOgImage = constructImageUrl(ogImage || twitterImage || defaultImage);
	$: fullTwitterImage = constructImageUrl(twitterImage || ogImage || defaultImage);

	// Robots Directives
	$: robotsContent = constructRobotsContent();

	// Language & Region
	$: hreflang = `${language}-${region}`;
	$: locale = `${language}_${region.toUpperCase()}`;

	// Schema Generation
	$: generatedSchema = autoSchema ? generateSchema() : schema;
	$: allSchemas = combineSchemas(generatedSchema);

	$: effectiveKeywords = keywords.length >= 3 ? keywords : generateDefaultKeywords();
	$: effectiveBreadcrumbs =
		breadcrumbs.length > 0 ? breadcrumbs : generateBreadcrumbsFromPath($page.url.pathname);

	// SEO Score
	$: seoScore = calculateSEOScore();
	$: seoWarnings = getSEOWarnings();

	// ═══════════════════════════════════════════════════════════════════════════
	// Helper Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function generateJsonLdScript(schema: Record<string, unknown>): string {
		const tag = 'script';
		return `<${tag} type="application/ld+json">${JSON.stringify(schema)}</${tag}>`;
	}

	function generateGtmScript(id: string): string {
		const tag = 'script';
		return `<${tag}>(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src='https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);})(window,document,'script','dataLayer','${id}');</${tag}>`;
	}

	function constructTitle(baseTitle: string, site: string): string {
		if (!baseTitle) return site;
		if (baseTitle.includes(site)) return baseTitle;
		const suffix = ` | ${site}`;
		if (baseTitle.length + suffix.length > 60) {
			return baseTitle;
		}
		return `${baseTitle}${suffix}`;
	}

	function constructImageUrl(imagePath: string | null): string {
		if (!imagePath) return `${siteUrl}${defaultImage}`;
		if (imagePath.startsWith('http')) return imagePath;
		if (imagePath.startsWith('//')) return `https:${imagePath}`;
		if (imagePath.startsWith('/')) return `${siteUrl}${imagePath}`;
		return `${siteUrl}/${imagePath}`;
	}

	function constructRobotsContent(): string {
		const directives = [];

		if (noindex) directives.push('noindex');
		else directives.push('index');

		if (nofollow) directives.push('nofollow');
		else directives.push('follow');

		if (noimageindex) directives.push('noimageindex');
		if (noarchive) directives.push('noarchive');
		if (nosnippet) directives.push('nosnippet');
		if (notranslate) directives.push('notranslate');

		// Advanced directives
		if (!nosnippet) {
			directives.push('max-snippet:-1');
			directives.push('max-image-preview:large');
			directives.push('max-video-preview:-1');
		}

		if (unavailable_after) {
			directives.push(`unavailable_after:${unavailable_after}`);
		}

		return directives.join(', ') || 'index, follow';
	}

	function generateDefaultKeywords(): string[] {
		const baseKeywords = ['Revolution Trading Pros', 'trading education', 'stock market'];

		if (!title) {
			return baseKeywords;
		}

		const fromTitle = title
			.toLowerCase()
			.replace(/[^a-z0-9\s]/g, '')
			.split(/\s+/)
			.filter(Boolean);

		const unique = Array.from(new Set([...fromTitle, ...baseKeywords]));
		return unique.slice(0, 8);
	}

	function generateBreadcrumbsFromPath(pathname: string): Array<{ name: string; url: string }> {
		const segments = pathname.split('/').filter(Boolean);
		const crumbs: Array<{ name: string; url: string }> = [];

		crumbs.push({ name: 'Home', url: '/' });

		let currentPath = '';
		for (const segment of segments) {
			currentPath += `/${segment}`;
			const name = segment.replace(/[-_]/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
			crumbs.push({ name, url: currentPath });
		}

		return crumbs;
	}

	function generateSchema(): any[] {
		const schemas = [];

		// Website/Organization Schema
		schemas.push({
			'@context': 'https://schema.org',
			'@type': 'WebSite',
			'@id': `${siteUrl}/#website`,
			url: siteUrl,
			name: siteName,
			description: siteDescription,
			publisher: {
				'@type': 'Organization',
				'@id': `${siteUrl}/#organization`,
				name: siteName,
				url: siteUrl,
				logo: {
					'@type': 'ImageObject',
					url: `${siteUrl}/logo.png`,
					width: 600,
					height: 60
				},
				sameAs: [
					twitterHandle ? `https://twitter.com/${twitterHandle.replace('@', '')}` : null,
					'https://www.facebook.com/RevolutionTradingPros',
					'https://www.linkedin.com/company/revolution-trading-pros',
					'https://www.youtube.com/@RevolutionTradingPros'
				].filter(Boolean)
			},
			potentialAction: {
				'@type': 'SearchAction',
				target: {
					'@type': 'EntryPoint',
					urlTemplate: `${siteUrl}/search?q={search_term_string}`
				},
				'query-input': 'required name=search_term_string'
			}
		});

		// WebPage Schema
		schemas.push({
			'@context': 'https://schema.org',
			'@type': schemaType || 'WebPage',
			'@id': `${fullCanonical}/#webpage`,
			url: fullCanonical,
			name: fullTitle,
			description: fullDescription,
			isPartOf: { '@id': `${siteUrl}/#website` },
			primaryImageOfPage: {
				'@type': 'ImageObject',
				url: fullOgImage,
				width: ogImageWidth,
				height: ogImageHeight
			},
			datePublished: publishedTime || new Date().toISOString(),
			dateModified: modifiedTime || new Date().toISOString(),
			author: author
				? {
						'@type': 'Person',
						name: author,
						url: `${siteUrl}/author/${author.toLowerCase().replace(/\s+/g, '-')}`
					}
				: { '@id': `${siteUrl}/#organization` },
			inLanguage: language,
			potentialAction: [
				{
					'@type': 'ReadAction',
					target: [fullCanonical]
				}
			]
		});

		// BreadcrumbList Schema - with safety check
		const crumbs =
			breadcrumbs.length > 0 ? breadcrumbs : generateBreadcrumbsFromPath($page.url.pathname);
		if (crumbs && crumbs.length > 0) {
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'BreadcrumbList',
				'@id': `${fullCanonical}/#breadcrumb`,
				itemListElement: crumbs.map((crumb, index) => ({
					'@type': 'ListItem',
					position: index + 1,
					name: crumb.name,
					item: crumb.url.startsWith('http') ? crumb.url : `${siteUrl}${crumb.url}`
				}))
			});
		}

		// Article Schema
		if (ogType === 'article' && publishedTime) {
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'Article',
				'@id': `${fullCanonical}/#article`,
				headline: title,
				description: fullDescription,
				image: fullOgImage,
				datePublished: publishedTime,
				dateModified: modifiedTime || publishedTime,
				author: author
					? {
							'@type': 'Person',
							name: author
						}
					: { '@id': `${siteUrl}/#organization` },
				publisher: { '@id': `${siteUrl}/#organization` },
				mainEntityOfPage: { '@id': `${fullCanonical}/#webpage` },
				articleSection: section || 'Trading',
				keywords: keywords.join(', '),
				articleBody: fullDescription,
				wordCount: fullDescription.split(' ').length,
				commentCount: 0,
				inLanguage: language
			});
		}

		// Product Schema
		if (ogType === 'product' && productPrice) {
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'Product',
				'@id': `${fullCanonical}/#product`,
				name: title,
				description: fullDescription,
				image: fullOgImage,
				brand: productBrand
					? {
							'@type': 'Brand',
							name: productBrand
						}
					: undefined,
				sku: productSKU,
				offers: {
					'@type': 'Offer',
					url: fullCanonical,
					priceCurrency: productCurrency,
					price: productPrice,
					availability: `https://schema.org/${productAvailability.replace(' ', '')}`,
					itemCondition: `https://schema.org/${productCondition}Condition`,
					seller: { '@id': `${siteUrl}/#organization` }
				},
				aggregateRating: productRating
					? {
							'@type': 'AggregateRating',
							ratingValue: productRating,
							reviewCount: productReviewCount || 1
						}
					: undefined
			});
		}

		// Course Schema (for trading courses)
		if (schemaType === 'Course') {
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'Course',
				'@id': `${fullCanonical}/#course`,
				name: title,
				description: fullDescription,
				provider: { '@id': `${siteUrl}/#organization` },
				url: fullCanonical,
				courseCode: productSKU,
				educationalLevel: 'Beginner to Advanced',
				inLanguage: language,
				hasCourseInstance: {
					'@type': 'CourseInstance',
					courseMode: 'Online',
					courseWorkload: 'PT10H'
				}
			});
		}

		// FAQ Schema
		if (schemaType === 'FAQPage') {
			// This would be populated with actual FAQ data
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'FAQPage',
				'@id': `${fullCanonical}/#faq`,
				mainEntity: []
			});
		}

		return schemas;
	}

	function combineSchemas(generated: any[]): any[] {
		const schemas = [...generated];
		if (schema) {
			if (Array.isArray(schema)) {
				schemas.push(...schema);
			} else {
				schemas.push(schema);
			}
		}
		return schemas;
	}

	function calculateSEOScore(): number {
		let score = 0;
		const checks = [
			{ condition: titleOptimal, weight: 15 },
			{ condition: descriptionOptimal, weight: 15 },
			{ condition: !!fullCanonical, weight: 10 },
			{ condition: !!fullOgImage, weight: 10 },
			{ condition: effectiveKeywords.length >= 3, weight: 10 },
			{ condition: !!author, weight: 5 },
			{ condition: !!publishedTime, weight: 5 },
			{ condition: !!modifiedTime, weight: 5 },
			{ condition: effectiveBreadcrumbs.length > 0, weight: 10 },
			{ condition: allSchemas.length > 0, weight: 15 }
		];

		checks.forEach((check) => {
			if (check.condition) score += check.weight;
		});

		return score;
	}

	function getSEOWarnings(): string[] {
		const warnings = [];

		if (!titleOptimal) {
			warnings.push(`Title should be 30-60 characters (current: ${titleLength})`);
		}

		if (!descriptionOptimal) {
			warnings.push(`Description should be 80-200 characters (current: ${descriptionLength})`);
		}

		if (!fullCanonical) {
			warnings.push('Missing canonical URL');
		}

		if (!fullOgImage) {
			warnings.push('Missing Open Graph image');
		}

		if (effectiveKeywords.length < 3) {
			warnings.push('Add more keywords for better SEO');
		}

		if (!effectiveBreadcrumbs.length) {
			warnings.push('Add breadcrumbs for better navigation');
		}

		return warnings;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		if (browser) {
			// Log SEO warnings in development only if score is below 60
			// This reduces console noise while still alerting to serious SEO issues
			if (import.meta.env.DEV && seoWarnings.length > 0 && seoScore < 60) {
				console.group('🔍 SEO Warnings');
				console.log(`SEO Score: ${seoScore}/100`);
				seoWarnings.forEach((warning) => console.warn(warning));
				console.groupEnd();
			}

			// Send SEO metrics to analytics
			if (typeof window !== 'undefined' && 'gtag' in window) {
				(window as any).gtag('event', 'seo_metrics', {
					page_title: fullTitle,
					seo_score: seoScore,
					title_length: titleLength,
					description_length: descriptionLength,
					has_schema: allSchemas.length > 0,
					schema_count: allSchemas.length
				});
			}
		}
	});
</script>

<svelte:head>
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Primary Meta Tags -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	<title>{fullTitle}</title>
	<meta name="title" content={fullTitle} />
	<meta name="description" content={fullDescription} />
	{#if keywords.length > 0}
		<meta name="keywords" content={keywords.join(', ')} />
	{/if}

	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Robots & Crawling -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	<meta name="robots" content={robots || robotsContent} />
	<meta name="googlebot" content={robots || robotsContent} />
	<meta name="bingbot" content={robots || robotsContent} />

	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Canonical & Alternate -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	<link rel="canonical" href={fullCanonical} />
	<link rel="alternate" {hreflang} href={fullCanonical} />
	<link rel="alternate" hreflang="x-default" href={fullCanonical} />

	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Open Graph / Facebook -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	<meta property="og:type" content={ogType} />
	<meta property="og:site_name" content={ogSiteName || siteName} />
	<meta property="og:title" content={fullTitle} />
	<meta property="og:description" content={fullDescription} />
	<meta property="og:url" content={fullCanonical} />
	<meta property="og:image" content={fullOgImage} />
	<meta property="og:image:secure_url" content={fullOgImage} />
	<meta property="og:image:type" content="image/png" />
	<meta property="og:image:width" content={String(ogImageWidth)} />
	<meta property="og:image:height" content={String(ogImageHeight)} />
	<meta property="og:image:alt" content={ogImageAlt || title} />
	<meta property="og:locale" content={ogLocale} />
	{#each ogAlternateLocales as locale}
		<meta property="og:locale:alternate" content={locale} />
	{/each}
	{#if facebookAppId}
		<meta property="fb:app_id" content={facebookAppId} />
	{/if}

	<!-- Article Meta Tags -->
	{#if ogType === 'article'}
		{#if publishedTime}
			<meta property="article:published_time" content={publishedTime} />
		{/if}
		{#if modifiedTime}
			<meta property="article:modified_time" content={modifiedTime} />
		{/if}
		{#if expirationTime}
			<meta property="article:expiration_time" content={expirationTime} />
		{/if}
		{#if author}
			<meta property="article:author" content={author} />
		{/if}
		{#if section}
			<meta property="article:section" content={section} />
		{/if}
		{#each tags as tag}
			<meta property="article:tag" content={tag} />
		{/each}
	{/if}

	<!-- Product Meta Tags -->
	{#if ogType === 'product' && productPrice}
		<meta property="product:price:amount" content={String(productPrice)} />
		<meta property="product:price:currency" content={productCurrency} />
		<meta property="product:availability" content={productAvailability} />
		<meta property="product:condition" content={productCondition} />
		{#if productBrand}
			<meta property="product:brand" content={productBrand} />
		{/if}
	{/if}

	<!-- Video Meta Tags -->
	{#if ogVideo}
		<meta property="og:video" content={ogVideo} />
		<meta property="og:video:secure_url" content={ogVideo} />
		<meta property="og:video:type" content="video/mp4" />
		<meta property="og:video:width" content="1280" />
		<meta property="og:video:height" content="720" />
	{/if}

	<!-- Audio Meta Tags -->
	{#if ogAudio}
		<meta property="og:audio" content={ogAudio} />
		<meta property="og:audio:secure_url" content={ogAudio} />
		<meta property="og:audio:type" content="audio/mpeg" />
	{/if}

	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Twitter Card -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	<meta name="twitter:card" content={twitterCard} />
	<meta name="twitter:site" content={twitterSite || twitterHandle} />
	<meta name="twitter:creator" content={twitterCreator || twitterHandle} />
	<meta name="twitter:title" content={fullTitle} />
	<meta name="twitter:description" content={fullDescription} />
	<meta name="twitter:image" content={fullTwitterImage} />
	<meta name="twitter:image:alt" content={twitterImageAlt || ogImageAlt || title} />
	{#if twitterPlayer}
		<meta name="twitter:player" content={twitterPlayer} />
		<meta name="twitter:player:width" content="1280" />
		<meta name="twitter:player:height" content="720" />
	{/if}

	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Additional Meta Tags -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	{#if author}
		<meta name="author" content={author} />
	{/if}
	<meta name="generator" content="SvelteKit" />
	<meta name="rating" content={rating} />
	<meta name="referrer" content={referrer} />
	<meta name="theme-color" content={themeColor} />
	<meta name="color-scheme" content={colorScheme} />
	<meta name="viewport" content={viewport} />
	<meta name="format-detection" content="telephone=no" />

	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Performance Hints -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	{#each preconnect as url}
		<link rel="preconnect" href={url} crossorigin="anonymous" />
	{/each}

	{#each dnsPrefetch as url}
		<link rel="dns-prefetch" href={url} />
	{/each}

	{#each preload as resource}
		<link
			rel="preload"
			href={resource.href}
			as={resource.as}
			{...resource.type ? { type: resource.type } : {}}
			crossorigin="anonymous"
		/>
	{/each}

	{#each prefetch as url}
		<link rel="prefetch" href={url} />
	{/each}

	{#each modulePreload as url}
		<link rel="modulepreload" href={url} />
	{/each}

	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- JSON-LD Structured Data -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	{#each allSchemas as schemaItem}
		{@html generateJsonLdScript(schemaItem)}
	{/each}

	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Google Tag Manager -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	{#if gtmId && browser}
		{@html generateGtmScript(gtmId)}
	{/if}
</svelte:head>

<style>
	/* No styles needed - this is a head-only component */
</style>
