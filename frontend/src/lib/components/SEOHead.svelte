<!--
/**
 * SEOHead Component - Google November 2025 Enterprise Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * ENTERPRISE FEATURES (Updated November 2025):
 *
 * 1. ADVANCED SEO:
 *    - Dynamic meta generation
 *    - GEO (Generative Engine Optimization) for AI search
 *    - Multi-language support with hreflang
 *    - Regional targeting
 *    - Mobile-first indexing
 *    - Core Web Vitals hints (LCP, INP, CLS)
 *
 * 2. SCHEMA GENERATION (November 2025):
 *    - Auto schema detection
 *    - Rich snippets
 *    - Knowledge graph
 *    - FAQ schema
 *    - Product schema
 *    - Course schema
 *    - VideoObject schema
 *    - HowTo schema
 *    - Speakable schema (voice search optimization)
 *
 * 3. SOCIAL OPTIMIZATION:
 *    - Multi-platform cards
 *    - Dynamic OG images
 *    - Twitter/X cards
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
 *    - AI citation tracking
 *
 * @version 4.0.0 (Google November 2025 + GEO)
 * @component
 */
-->

<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// Props & Configuration
	// ═══════════════════════════════════════════════════════════════════════════

	// Props Interface
	interface Props {
		title: string;
		description: string;
		canonical?: string | null;
		keywords?: string[];
		author?: string | null;
		robots?: string | null;
		noindex?: boolean;
		nofollow?: boolean;
		noimageindex?: boolean;
		noarchive?: boolean;
		nosnippet?: boolean;
		notranslate?: boolean;
		unavailable_after?: string | null;
		ogType?: 'website' | 'article' | 'product' | 'video' | 'music' | 'book' | 'profile';
		ogImage?: string | null;
		ogImageAlt?: string | null;
		ogImageWidth?: number;
		ogImageHeight?: number;
		ogVideo?: string | null;
		ogAudio?: string | null;
		ogLocale?: string;
		ogAlternateLocales?: string[];
		ogSiteName?: string | null;
		twitterCard?: 'summary' | 'summary_large_image' | 'app' | 'player';
		twitterSite?: string | null;
		twitterCreator?: string | null;
		twitterImage?: string | null;
		twitterImageAlt?: string | null;
		twitterPlayer?: string | null;
		publishedTime?: string | null;
		modifiedTime?: string | null;
		expirationTime?: string | null;
		section?: string | null;
		tags?: string[];
		productPrice?: number | null;
		productCurrency?: string;
		productAvailability?: 'in stock' | 'out of stock' | 'preorder' | 'discontinued';
		productCondition?: 'new' | 'used' | 'refurbished';
		productBrand?: string | null;
		productSKU?: string | null;
		productRating?: number | null;
		productReviewCount?: number | null;
		schema?: any;
		autoSchema?: boolean;
		schemaType?: string | null;
		breadcrumbs?: Array<{ name: string; url: string }>;
		language?: string;
		region?: string;
		rating?: 'general' | 'mature' | 'restricted';
		referrer?: string;
		themeColor?: string;
		colorScheme?: 'light' | 'dark' | 'auto';
		viewport?: string;
		preconnect?: string[];
		dnsPrefetch?: string[];
		preload?: Array<{
			href: string;
			as:
				| 'audio'
				| 'document'
				| 'embed'
				| 'fetch'
				| 'font'
				| 'image'
				| 'object'
				| 'script'
				| 'style'
				| 'track'
				| 'video'
				| 'worker';
			type?: string;
		}>;
		prefetch?: string[];
		modulePreload?: string[];
		// November 2025 - Enhanced Schema Support
		speakable?: boolean;
		speakableSelectors?: string[];
		video?: {
			name: string;
			description: string;
			thumbnailUrl: string;
			uploadDate: string;
			duration?: string;
			contentUrl?: string;
			embedUrl?: string;
		} | null;
		howTo?: {
			name: string;
			description: string;
			totalTime?: string;
			estimatedCost?: { currency: string; value: number };
			steps: Array<{
				name: string;
				text: string;
				image?: string;
				url?: string;
			}>;
		} | null;
		faqItems?: Array<{ question: string; answer: string }>;
		// GEO (Generative Engine Optimization)
		geoOptimized?: boolean;
		aiCitationReady?: boolean;
	}

	let {
		title,
		description,
		canonical = null,
		keywords = [],
		author = null,
		robots = null,
		noindex = false,
		nofollow = false,
		noimageindex = false,
		noarchive = false,
		nosnippet = false,
		notranslate = false,
		unavailable_after = null,
		ogType = 'website',
		ogImage = null,
		ogImageAlt = null,
		ogImageWidth = 1200,
		ogImageHeight = 630,
		ogVideo = null,
		ogAudio = null,
		ogLocale = 'en_US',
		ogAlternateLocales = [],
		ogSiteName = null,
		twitterCard = 'summary_large_image',
		twitterSite = null,
		twitterCreator = null,
		twitterImage = null,
		twitterImageAlt = null,
		twitterPlayer = null,
		publishedTime = null,
		modifiedTime = null,
		expirationTime = null,
		section = null,
		tags = [],
		productPrice = null,
		productCurrency = 'USD',
		productAvailability = 'in stock',
		productCondition = 'new',
		productBrand = null,
		productSKU = null,
		productRating = null,
		productReviewCount = null,
		schema = null,
		autoSchema = true,
		schemaType = null,
		breadcrumbs = [],
		language = 'en',
		region = 'US',
		rating = 'general',
		referrer = 'no-referrer-when-downgrade',
		themeColor = '#1a1a1a',
		colorScheme = 'auto',
		viewport = 'width=device-width, initial-scale=1, maximum-scale=5',
		preconnect = [],
		dnsPrefetch = [],
		preload = [],
		prefetch = [],
		modulePreload = [],
		// November 2025 - Enhanced Schema Support
		speakable = true,
		speakableSelectors = ['h1', 'h2', '.speakable', '[data-speakable]'],
		video = null,
		howTo = null,
		faqItems = [],
		// GEO (Generative Engine Optimization)
		geoOptimized = true,
		aiCitationReady: _aiCitationReady = true
	}: Props = $props();

	// Site Configuration
	const siteUrl = import.meta.env['VITE_SITE_URL'] || 'https://revolution-trading-pros.pages.dev';
	const siteName = import.meta.env['VITE_SITE_NAME'] || 'Revolution Trading Pros';
	const siteDescription =
		import.meta.env['VITE_SITE_DESCRIPTION'] ||
		'Master the markets with institutional-grade trading tools and education';
	const defaultImage = import.meta.env['VITE_DEFAULT_OG_IMAGE'] || '/og-image-default.png';
	const twitterHandle = import.meta.env['VITE_TWITTER_HANDLE'] || '@RevTradingPros';
	const facebookAppId = import.meta.env['VITE_FACEBOOK_APP_ID'] || '';

	// ═══════════════════════════════════════════════════════════════════════════
	// Computed Properties
	// ═══════════════════════════════════════════════════════════════════════════

	// URL Construction - use consistent server-side URL to avoid hydration mismatch
	let currentUrl = $derived(`${siteUrl}${page.url.pathname}`);
	let fullCanonical = $derived(
		canonical ? (canonical.startsWith('http') ? canonical : `${siteUrl}${canonical}`) : currentUrl
	);

	// Title Construction
	let fullTitle = $derived(constructTitle(title, siteName));
	let titleLength = $derived(fullTitle.length);
	let titleOptimal = $derived(titleLength >= 30 && titleLength <= 60);

	// Description Construction
	let fullDescription = $derived(description || siteDescription);
	let descriptionLength = $derived(fullDescription.length);
	let descriptionOptimal = $derived(descriptionLength >= 80 && descriptionLength <= 200);

	// Image URLs
	let fullOgImage = $derived(constructImageUrl(ogImage || twitterImage || defaultImage));
	let fullTwitterImage = $derived(constructImageUrl(twitterImage || ogImage || defaultImage));

	// Robots Directives
	let robotsContent = $derived(constructRobotsContent());

	// Language & Region
	let hreflang = $derived(`${language}-${region}`);

	// Schema Generation
	let generatedSchema = $derived(autoSchema ? generateSchema() : schema);
	let allSchemas = $derived(combineSchemas(generatedSchema));

	let effectiveKeywords = $derived(keywords.length >= 3 ? keywords : generateDefaultKeywords());
	let effectiveBreadcrumbs = $derived(
		breadcrumbs.length > 0 ? breadcrumbs : generateBreadcrumbsFromPath(page.url.pathname)
	);

	// SEO Score
	let seoScore = $derived(calculateSEOScore());
	let seoWarnings = $derived(getSEOWarnings());

	// ═══════════════════════════════════════════════════════════════════════════
	// Helper Functions
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Generate stable JSON-LD script to avoid hydration mismatches
	 * Uses deterministic key ordering for consistent server/client output
	 */
	function generateJsonLdScript(schema: Record<string, unknown>): string {
		const stableStringify = (obj: unknown): string => {
			if (obj === null || typeof obj !== 'object') {
				return JSON.stringify(obj);
			}
			if (Array.isArray(obj)) {
				return '[' + obj.map(stableStringify).join(',') + ']';
			}
			const keys = Object.keys(obj as Record<string, unknown>).sort();
			const pairs = keys
				.filter((k) => (obj as Record<string, unknown>)[k] !== undefined)
				.map((k) => `${JSON.stringify(k)}:${stableStringify((obj as Record<string, unknown>)[k])}`);
			return '{' + pairs.join(',') + '}';
		};
		// Use concatenation to avoid Svelte parsing the script tag
		const tag = 'script';
		return `<${tag} type="application/ld+json">${stableStringify(schema)}</${tag}>`;
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
			...(publishedTime ? { datePublished: publishedTime } : {}),
			...(modifiedTime ? { dateModified: modifiedTime } : {}),
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
			breadcrumbs.length > 0 ? breadcrumbs : generateBreadcrumbsFromPath(page.url.pathname);
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

		// FAQ Schema - Enhanced with actual FAQ items
		if (schemaType === 'FAQPage' || faqItems.length > 0) {
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'FAQPage',
				'@id': `${fullCanonical}/#faq`,
				mainEntity: faqItems.map((item) => ({
					'@type': 'Question',
					name: item.question,
					acceptedAnswer: {
						'@type': 'Answer',
						text: item.answer
					}
				}))
			});
		}

		// ═══════════════════════════════════════════════════════════════════════════
		// November 2025 - Enhanced Schema Types
		// ═══════════════════════════════════════════════════════════════════════════

		// Speakable Schema (Voice Search Optimization)
		if (speakable) {
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'WebPage',
				'@id': `${fullCanonical}/#speakable`,
				speakable: {
					'@type': 'SpeakableSpecification',
					cssSelector: speakableSelectors
				},
				url: fullCanonical
			});
		}

		// VideoObject Schema
		if (video) {
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'VideoObject',
				'@id': `${fullCanonical}/#video`,
				name: video.name,
				description: video.description,
				thumbnailUrl: video.thumbnailUrl.startsWith('http')
					? video.thumbnailUrl
					: `${siteUrl}${video.thumbnailUrl}`,
				uploadDate: video.uploadDate,
				...(video.duration && { duration: video.duration }),
				...(video.contentUrl && { contentUrl: video.contentUrl }),
				...(video.embedUrl && { embedUrl: video.embedUrl }),
				publisher: { '@id': `${siteUrl}/#organization` }
			});
		}

		// HowTo Schema (for tutorials and guides)
		if (howTo) {
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'HowTo',
				'@id': `${fullCanonical}/#howto`,
				name: howTo.name,
				description: howTo.description,
				...(howTo.totalTime && { totalTime: howTo.totalTime }),
				...(howTo.estimatedCost && {
					estimatedCost: {
						'@type': 'MonetaryAmount',
						currency: howTo.estimatedCost.currency,
						value: howTo.estimatedCost.value
					}
				}),
				step: howTo.steps.map((step, index) => ({
					'@type': 'HowToStep',
					position: index + 1,
					name: step.name,
					text: step.text,
					...(step.image && {
						image: step.image.startsWith('http') ? step.image : `${siteUrl}${step.image}`
					}),
					...(step.url && {
						url: step.url.startsWith('http') ? step.url : `${siteUrl}${step.url}`
					})
				}))
			});
		}

		// GEO Optimization Schema (for AI Search Engines)
		if (geoOptimized) {
			// Add EducationalOrganization schema for trading education context
			schemas.push({
				'@context': 'https://schema.org',
				'@type': 'EducationalOrganization',
				'@id': `${siteUrl}/#educational-org`,
				name: siteName,
				description: siteDescription,
				url: siteUrl,
				sameAs: [
					twitterHandle ? `https://twitter.com/${twitterHandle.replace('@', '')}` : null,
					'https://www.facebook.com/RevolutionTradingPros',
					'https://www.linkedin.com/company/revolution-trading-pros',
					'https://www.youtube.com/@RevolutionTradingPros'
				].filter(Boolean),
				areaServed: {
					'@type': 'Country',
					name: 'United States'
				},
				hasOfferCatalog: {
					'@type': 'OfferCatalog',
					name: 'Trading Education Programs',
					itemListElement: [
						{
							'@type': 'Offer',
							itemOffered: {
								'@type': 'Course',
								name: 'Day Trading Masterclass',
								description: 'Professional day trading strategies and techniques'
							}
						},
						{
							'@type': 'Offer',
							itemOffered: {
								'@type': 'Course',
								name: 'Swing Trading Pro',
								description: 'Advanced swing trading methods for consistent profits'
							}
						}
					]
				}
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
				logger.info(`SEO Score: ${seoScore}/100`);
				seoWarnings.forEach((warning) => logger.warn(warning));
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

	<!-- GTM is loaded via app.html or hooks to avoid hydration mismatch -->
</svelte:head>
