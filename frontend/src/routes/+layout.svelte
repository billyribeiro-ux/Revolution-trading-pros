<script lang="ts">
	/**
	 * Root Layout - Restored with essential components
	 * 
	 * ICT9+ Hydration-Safe Pattern:
	 * - Auth-dependent UI uses client-only state to prevent SSR/client mismatch
	 * - Stores that differ between SSR and client are only read after mount
	 * 
	 * ICT8-11+ GA4 Integration:
	 * - Uses SvelteKit's afterNavigate for page view tracking
	 * - Prevents GA4 from using history.pushState (conflicts with SvelteKit router)
	 */
	import '../app.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import { NavBar } from '$lib/components/nav';
	import { MarketingFooter } from '$lib/components/layout';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { registerServiceWorker } from '$lib/utils/registerServiceWorker';
	import { initPerformanceMonitoring } from '$lib/utils/performance';
	import { isAdminUser } from '$lib/stores/auth';
	import type { Snippet } from 'svelte';

	// Consent System
	import { 
		initializeConsent,
		ConsentBanner,
		ConsentPreferencesModal,
		ConsentSettingsButton
	} from '$lib/consent';

	// GA4 Page View Tracking (ICT8-11+ SvelteKit-native approach)
	import { trackPageView } from '$lib/consent/vendors/ga4';

	let { children }: { children: Snippet } = $props();

	// Derived state from page (safe - page state is consistent SSR/client)
	let pathname = $derived(page.url.pathname);
	let isAdminArea = $derived(pathname.startsWith('/admin'));
	let isEmbedArea = $derived(pathname.startsWith('/embed'));
	
	// ICT9+ Hydration-Safe Pattern: 
	// Auth state is ONLY read client-side to prevent SSR/client mismatch
	// SSR always renders without admin toolbar, client adds it after hydration
	let mounted = $state(false);
	let isAdmin = $derived(mounted && $isAdminUser);

	// ICT8-11+ Fix: Track page views using SvelteKit's navigation lifecycle
	// This replaces GA4's history.pushState interception which conflicts with SvelteKit
	afterNavigate(({ to }) => {
		if (browser && to?.url) {
			trackPageView(to.url.href);
		}
	});

	onMount(() => {
		// Mark as mounted FIRST to enable client-only derived values
		mounted = true;
		
		if (browser) {
			// ICT8-11+ Performance: Activate font stylesheet immediately
			const fontLink = document.getElementById('critical-fonts') as HTMLLinkElement;
			if (fontLink) fontLink.media = 'all';
			
			// ICT8-11+ Performance: Register service worker immediately for caching
			registerServiceWorker();
			
			// ICT8-11+ Performance: Defer non-critical initialization to after LCP
			// Use requestIdleCallback for consent/tracking (not render-blocking)
			if ('requestIdleCallback' in window) {
				(window as any).requestIdleCallback(() => {
					initPerformanceMonitoring();
					initializeConsent();
				}, { timeout: 2000 });
			} else {
				// Fallback: defer by 100ms to not block main thread
				setTimeout(() => {
					initPerformanceMonitoring();
					initializeConsent();
				}, 100);
			}
		}
	});
</script>

<svelte:head>
	<!-- ICT8-11+ Performance: Preconnect to critical origins FIRST -->
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	
	<!-- ICT8-11+ Performance: Load only critical font weights with display=swap -->
	<link 
		rel="stylesheet" 
		href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600&display=swap"
		media="print"
		id="critical-fonts"
	/>
	
	<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
	<meta name="theme-color" content="#0a101c" />
</svelte:head>

{#if isAdminArea || isEmbedArea}
	{@render children()}
{:else}
	<div class="min-h-screen bg-rtp-bg text-rtp-text" class:has-admin-toolbar={isAdmin}>
		<!-- ICT9+ Hydration-Safe: Only render AdminToolbar after client mount -->
		{#if mounted}
			<AdminToolbar />
		{/if}
		<NavBar />
		<main>
			{@render children()}
		</main>
		<MarketingFooter />
		<!-- ICT9+ Hydration-Safe: Only render consent components after client mount -->
		{#if mounted}
			<ConsentBanner />
			<ConsentPreferencesModal />
			<ConsentSettingsButton position="bottom-left" />
		{/if}
	</div>
{/if}

<style>
	.has-admin-toolbar {
		--admin-toolbar-height: 46px;
		padding-top: var(--admin-toolbar-height);
	}
</style>
