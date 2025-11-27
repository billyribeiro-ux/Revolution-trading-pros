<script lang="ts">
	/**
	 * Root Layout - Global shell for all routes
	 * Handles performance monitoring, service worker, and route-based chrome
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import '../app.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import { MarketingNav, MarketingFooter } from '$lib/components/layout';
	import PopupModal from '$lib/components/PopupModal.svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { page } from '$app/stores';
	import { registerServiceWorker } from '$lib/utils/registerServiceWorker';
	import { initPerformanceMonitoring } from '$lib/utils/performance';

	// Route-based layout detection
	$: pathname = $page.url.pathname;
	$: isAdminArea = pathname.startsWith('/admin');
	$: isTradingRoom = pathname.startsWith('/live-trading-rooms/') && pathname.split('/').length > 2;
	$: isEmbedArea = pathname.startsWith('/embed');
	$: isAuthPage = pathname.startsWith('/login') || pathname.startsWith('/signup') || pathname.startsWith('/register');
	
	// Show marketing chrome only on public pages
	$: showMarketingChrome = !isAdminArea && !isTradingRoom && !isEmbedArea;

	// Initialize performance optimizations
	onMount(() => {
		if (browser) {
			// Register service worker for offline support and caching
			registerServiceWorker();
			
			// Initialize Core Web Vitals monitoring
			initPerformanceMonitoring();
		}
	});
</script>

<!--
	SEO Note: Page-specific SEO is handled by individual +page.svelte files using SEOHead component.
	Global meta tags are set in app.html for fallback only.
	Each page must define its own title, description, canonical, and structured data.
-->

<svelte:head>
	<!-- Performance: Critical Resource Hints (L11+ Optimization) -->
	<link rel="dns-prefetch" href="https://fonts.googleapis.com" />
	<link rel="dns-prefetch" href="https://fonts.gstatic.com" />
	<link rel="preconnect" href="https://fonts.googleapis.com" crossorigin="anonymous" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	
	<!-- Preload Critical Fonts with font-display: swap -->
	<link rel="preload" as="font" type="font/woff2" href="https://fonts.gstatic.com/s/inter/v13/UcCO3FwrK3iLTeHuS_fvQtMwCp50KnMw2boKoduKmMEVuLyfAZ9hiA.woff2" crossorigin="anonymous" />
	
	<!-- Optimize font loading with display=swap to prevent FOIT -->
	<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap" media="all" />
	
	<!-- Viewport optimization -->
	<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
	
	<!-- Performance hints -->
	<meta name="theme-color" content="#0a101c" />
</svelte:head>

{#if isAdminArea || isTradingRoom || isEmbedArea}
	<!-- Admin/Trading/Embed: No marketing chrome, just the content -->
	<slot />
{:else}
	<div class="min-h-screen bg-rtp-bg text-rtp-text">
		<!-- Admin Toolbar (only shows for logged-in admins) -->
		<AdminToolbar />

		<!-- Marketing Navigation -->
		<MarketingNav />

		<main>
			<slot />
		</main>

		<!-- Marketing Footer -->
		<MarketingFooter />

		<!-- Global Popup Modal - Enterprise Grade: Only shows when explicitly triggered -->
		<PopupModal />
	</div>
{/if}
