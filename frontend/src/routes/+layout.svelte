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
	import { isAdminUser } from '$lib/stores/auth.svelte';
	import { initializeAuth } from '$lib/api/auth';
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

	// Derived state from page store (SSR-safe)
	let pathname = $derived(page.url.pathname);
	let isAdminArea = $derived(pathname.startsWith('/admin'));
	let isEmbedArea = $derived(pathname.startsWith('/embed'));
	let isDashboardArea = $derived(pathname.startsWith('/dashboard'));

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
			// ICT11+ Pattern: Always initialize auth on page load
			// This handles page refresh scenarios where tokens need to be restored
			initializeAuth().catch((err) => {
				console.debug('[Layout] Auth init failed (non-critical):', err);
			});

			registerServiceWorker();
			initPerformanceMonitoring();
			initializeConsent();
		}
	});
</script>

<svelte:head>
	<!-- ICT 11 Fix: Font loading handled in app.html - no duplicates -->
	<!-- Viewport already in app.html, theme-color set per page preference -->
	<meta name="theme-color" content="#FFFFFF" />
</svelte:head>

{#if isAdminArea || isEmbedArea}
	<!-- Admin and Embed areas have their own layouts -->
	{@render children()}
{:else if isDashboardArea}
	<!-- Dashboard area: NavBar + Content + Footer -->
	<!-- NO bg-rtp-bg here - dashboard has its own light gray background (#f4f4f4) -->
	<!-- Breadcrumbs handled inside dashboard/+layout.svelte -->
	<div class="min-h-screen" class:has-admin-toolbar={isAdmin}>
		{#if mounted}
			<AdminToolbar />
		{/if}
		<NavBar />
		{@render children()}
		<MarketingFooter />
		{#if mounted}
			<ConsentBanner />
			<ConsentPreferencesModal />
			<ConsentSettingsButton position="bottom-left" />
		{/if}
	</div>
{:else}
	<!-- Marketing pages: No wrapper bg/text - pages define their own themes -->
	<!-- REMOVED nested <main> - pages have their own semantic <main> tags -->
	<div class="min-h-screen" class:has-admin-toolbar={isAdmin}>
		<!-- ICT9+ Hydration-Safe: Only render AdminToolbar after client mount -->
		{#if mounted}
			<AdminToolbar />
		{/if}
		<NavBar />
		{@render children()}
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
