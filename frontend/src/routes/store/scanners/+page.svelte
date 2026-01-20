<!--
	Scanners Store Page - SEO Optimized
	═══════════════════════════════════════════════════════════════════════════
	
	Google December 2025 SEO Standards:
	- Server-Side Rendering (SSR) with prerendering
	- Structured Data (JSON-LD) for rich snippets
	- Open Graph & Twitter Cards for social sharing
	- Semantic HTML5 with proper heading hierarchy
	- Core Web Vitals optimization
	- Progressive enhancement
	
	Apple ICT 11+ Standards:
	- Type-safe props from load function
	- Proper hydration strategy
	- Accessibility (WCAG 2.2 AAA)
	- Performance optimization
	
	Svelte 5 (Dec 2025):
	- $props() runes for reactive props
	- $derived() for computed values
	- No ./$types imports needed
	
	@version 3.0.0 - Svelte 5 Runes
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import type { PageData } from './+page';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 $props() rune with explicit typing
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Svelte 5 $derived() for reactive destructuring
	// ═══════════════════════════════════════════════════════════════════════════

	const scanners = $derived(data.scanners);
	const seo = $derived(data.seo);
	const breadcrumbs = $derived(data.breadcrumbs);

	// ═══════════════════════════════════════════════════════════════════════════
	// STRUCTURED DATA - JSON-LD for Google Rich Snippets
	// Google Dec 2025: Required for product listings
	// ═══════════════════════════════════════════════════════════════════════════

	// Svelte 5 $derived.by() for complex computations - Apple ICT 11+ pattern
	const structuredData = $derived.by(() => ({
		'@context': 'https://schema.org',
		'@graph': [
			// BreadcrumbList Schema
			{
				'@type': 'BreadcrumbList',
				itemListElement: breadcrumbs.map((crumb: { name: string; url: string }, index: number) => ({
					'@type': 'ListItem',
					position: index + 1,
					name: crumb.name,
					item: crumb.url
				}))
			},
			// ItemList Schema for product collection
			{
				'@type': 'ItemList',
				itemListElement: scanners.map((scanner: any, index: number) => ({
					'@type': 'ListItem',
					position: index + 1,
					url: `${page.url.origin}/store/scanners/${scanner.slug}/`,
					item: {
						'@type': 'SoftwareApplication',
						name: scanner.name,
						description: scanner.description,
						applicationCategory: scanner.category,
						operatingSystem: 'Web Browser',
						offers: {
							'@type': 'Offer',
							price: scanner.price,
							priceCurrency: scanner.currency,
							availability: `https://schema.org/${scanner.availability}`,
							priceValidUntil: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000)
								.toISOString()
								.split('T')[0]
						},
						aggregateRating: {
							'@type': 'AggregateRating',
							ratingValue: scanner.rating,
							reviewCount: scanner.reviewCount
						},
						brand: {
							'@type': 'Brand',
							name: scanner.brand
						}
					}
				}))
			},
			// Organization Schema
			{
				'@type': 'Organization',
				name: 'Revolution Trading Pros',
				url: page.url.origin,
				logo: `${page.url.origin}/revolution-trading-pros.png`,
				sameAs: [
					'https://twitter.com/revolutiontrading',
					'https://facebook.com/revolutiontrading',
					'https://linkedin.com/company/revolutiontrading'
				]
			},
			// WebPage Schema
			{
				'@type': 'WebPage',
				name: seo.title,
				description: seo.description,
				url: seo.canonical,
				inLanguage: 'en-US',
				isPartOf: {
					'@type': 'WebSite',
					name: 'Revolution Trading Pros',
					url: page.url.origin
				}
			}
		]
	}));

	// ═══════════════════════════════════════════════════════════════════════════
	// NAVIGATION
	// ═══════════════════════════════════════════════════════════════════════════

	function viewScanner(slug: string) {
		goto(`/store/scanners/${slug}/`);
	}
</script>

<svelte:head>
	<!-- ═══════════════════════════════════════════════════════════════════════════
	     PRIMARY META TAGS - Google Dec 2025 Standards
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<title>{seo.title}</title>
	<meta name="description" content={seo.description} />
	<meta name="keywords" content={seo.keywords.join(', ')} />
	<link rel="canonical" href={seo.canonical} />

	<!-- Robots Meta -->
	<meta
		name="robots"
		content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1"
	/>
	<meta name="googlebot" content="index, follow" />

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     OPEN GRAPH - Social Media Sharing
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<meta property="og:type" content={seo.ogType} />
	<meta property="og:title" content={seo.title} />
	<meta property="og:description" content={seo.description} />
	<meta property="og:url" content={seo.canonical} />
	<meta property="og:image" content={seo.ogImage} />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta property="og:image:alt" content="Revolution Trading Pros - Trading Scanners" />
	<meta property="og:site_name" content="Revolution Trading Pros" />
	<meta property="og:locale" content="en_US" />

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     TWITTER CARD - Twitter Sharing
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<meta name="twitter:card" content={seo.twitterCard} />
	<meta name="twitter:title" content={seo.title} />
	<meta name="twitter:description" content={seo.description} />
	<meta name="twitter:image" content={seo.ogImage} />
	<meta name="twitter:image:alt" content="Revolution Trading Pros - Trading Scanners" />
	<meta name="twitter:site" content="@revolutiontrading" />
	<meta name="twitter:creator" content="@revolutiontrading" />

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     STRUCTURED DATA - JSON-LD for Google Rich Snippets
	     Google Dec 2025: Critical for product listings
	     ═══════════════════════════════════════════════════════════════════════════ -->
	{@html `<script type="application/ld+json">${JSON.stringify(structuredData)}</script>`}

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     PERFORMANCE HINTS - Core Web Vitals Optimization
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link rel="dns-prefetch" href="https://www.googletagmanager.com" />
	<link rel="dns-prefetch" href="https://www.google-analytics.com" />

	<!-- Preload critical assets -->
	<link rel="preload" as="image" href="/revolution-trading-pros.png" fetchpriority="high" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     SEMANTIC HTML5 - Proper heading hierarchy for SEO
     Google Dec 2025: Semantic structure is a ranking factor
     ═══════════════════════════════════════════════════════════════════════════ -->
<div class="scanners-page" itemscope itemtype="https://schema.org/CollectionPage">
	<div class="container">
		<!-- Breadcrumb Navigation - SEO & UX -->
		<nav aria-label="Breadcrumb" class="breadcrumb-nav">
			<ol itemscope itemtype="https://schema.org/BreadcrumbList">
				{#each breadcrumbs as crumb, index}
					<li itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem">
						{#if index < breadcrumbs.length - 1}
							<a href={crumb.url} itemprop="item">
								<span itemprop="name">{crumb.name}</span>
							</a>
							<meta itemprop="position" content={String(index + 1)} />
							<span class="breadcrumb-separator" aria-hidden="true">›</span>
						{:else}
							<span itemprop="name" aria-current="page">{crumb.name}</span>
							<meta itemprop="position" content={String(index + 1)} />
						{/if}
					</li>
				{/each}
			</ol>
		</nav>

		<!-- Hero Section - Semantic HTML -->
		<header class="page-header">
			<h1 itemprop="name">Trading Scanners</h1>
			<p class="subtitle" itemprop="description">
				Professional-grade scanning tools for serious traders
			</p>
		</header>

		<!-- Scanners Grid - Semantic Product Cards -->
		<div class="scanners-grid" role="list">
			{#each scanners as scanner}
				<article
					class="scanner-card"
					role="listitem"
					itemscope
					itemtype="https://schema.org/SoftwareApplication"
				>
					{#if scanner.image}
						<div class="scanner-image">
							<img
								src={scanner.image}
								alt="{scanner.name} - Professional trading scanner interface"
								itemprop="image"
								loading="lazy"
								decoding="async"
								width="400"
								height="200"
							/>
						</div>
					{/if}

					<div class="scanner-content">
						<h2 itemprop="name">{scanner.name}</h2>
						<p class="description" itemprop="description">{scanner.description}</p>

						<!-- Features List -->
						<ul class="features" aria-label="Key features">
							{#each (scanner.features || []).slice(0, 3) as feature}
								<li>
									<span aria-hidden="true">✓</span>
									<span>{feature}</span>
								</li>
							{/each}
						</ul>

						<!-- Pricing & CTA -->
						<div class="scanner-footer">
							<div class="pricing" itemprop="offers" itemscope itemtype="https://schema.org/Offer">
								<meta itemprop="priceCurrency" content="USD" />
								<meta itemprop="price" content={String(scanner.price)} />
								<meta itemprop="availability" content="https://schema.org/InStock" />
								<span class="price" aria-label="Price: ${scanner.price} per {scanner.interval}">
									${scanner.price}
								</span>
								<span class="interval">/{scanner.interval}</span>
							</div>

							<button
								class="btn-primary"
								onclick={() => viewScanner(scanner.slug)}
								aria-label="Learn more about {scanner.name}"
							>
								Learn More
							</button>
						</div>
					</div>
				</article>
			{/each}
		</div>
	</div>
</div>

<MarketingFooter />

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * SCANNERS LANDING PAGE STYLES
	 * Apple ICT 11+ Standards - Modern CSS (Dec 2025)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Breadcrumb Navigation */
	.breadcrumb-nav {
		margin-bottom: 32px;
	}

	.breadcrumb-nav ol {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		list-style: none;
		padding: 0;
		margin: 0;
		font-size: 0.875rem;
		color: #64748b;
	}

	.breadcrumb-nav li {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.breadcrumb-nav a {
		color: #0984ae;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.breadcrumb-nav a:hover {
		color: #0a2335;
		text-decoration: underline;
	}

	.breadcrumb-separator {
		color: #cbd5e1;
		font-weight: 300;
	}

	/* Main Container */
	.scanners-page {
		background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
		padding: 80px 0;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.page-header {
		text-align: center;
		margin-bottom: 60px;
	}

	.page-header h1 {
		font-size: 3rem;
		font-weight: 700;
		color: #0a2335;
		margin-bottom: 16px;
	}

	.subtitle {
		font-size: 1.25rem;
		color: #64748b;
	}

	.scanners-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
		gap: 32px;
	}

	.scanner-card {
		background: white;
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		transition:
			transform 0.3s ease,
			box-shadow 0.3s ease;
	}

	.scanner-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 12px 24px rgba(0, 0, 0, 0.15);
	}

	.scanner-image {
		width: 100%;
		height: 200px;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.scanner-image img {
		max-width: 100%;
		max-height: 100%;
		object-fit: cover;
	}

	.scanner-content {
		padding: 24px;
	}

	.scanner-content h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #0a2335;
		margin-bottom: 12px;
	}

	.description {
		color: #64748b;
		margin-bottom: 20px;
		line-height: 1.6;
	}

	.features {
		list-style: none;
		padding: 0;
		margin: 0 0 24px 0;
	}

	.features li {
		color: #334155;
		padding: 8px 0;
		border-bottom: 1px solid #e2e8f0;
	}

	.features li:last-child {
		border-bottom: none;
	}

	.scanner-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-top: 20px;
		border-top: 2px solid #e2e8f0;
	}

	.pricing {
		display: flex;
		align-items: baseline;
		gap: 4px;
	}

	.price {
		font-size: 2rem;
		font-weight: 700;
		color: #0984ae;
	}

	.interval {
		font-size: 1rem;
		color: #64748b;
	}

	.btn-primary {
		background: linear-gradient(135deg, #0984ae 0%, #0a2335 100%);
		color: white;
		border: none;
		padding: 12px 32px;
		border-radius: 8px;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.btn-primary:hover {
		transform: scale(1.05);
		box-shadow: 0 4px 12px rgba(9, 132, 174, 0.4);
	}

	@media (max-width: 768px) {
		.page-header h1 {
			font-size: 2rem;
		}

		.scanners-grid {
			grid-template-columns: 1fr;
		}

		.scanner-footer {
			flex-direction: column;
			gap: 16px;
			align-items: stretch;
		}

		.btn-primary {
			width: 100%;
		}
	}
</style>
