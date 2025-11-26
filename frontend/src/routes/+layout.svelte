<script>
	import '../app.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import NavBar from '$lib/components/NavBar.svelte';
	import Footer from '$lib/components/sections/Footer.svelte';
	import PopupModal from '$lib/components/PopupModal.svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { registerServiceWorker } from '$lib/utils/registerServiceWorker';
	import { initPerformanceMonitoring } from '$lib/utils/performance';

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

<svelte:head>
	<title>Revolution Trading Pros | Live Trading Rooms, Alerts & Pro Tools</title>
	<meta
		name="description"
		content="Professional trading education and tools. Live trading rooms, SPX alerts, courses, and indicators built for disciplined traders."
	/>
	
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

<div class="min-h-screen bg-rtp-bg text-rtp-text">
	<!-- Admin Toolbar (only shows for logged-in admins) -->
	<AdminToolbar />

	<NavBar />

	<main>
		<slot />
	</main>

	<Footer />

	<!-- Global Popup Modal - Enterprise Grade: Only shows when explicitly triggered -->
	<PopupModal />
</div>
