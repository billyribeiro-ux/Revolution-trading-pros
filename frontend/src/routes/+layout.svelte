<script lang="ts">
	/**
	 * Root Layout - Restored with essential components
	 */
	import '../app.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import { NavBar } from '$lib/components/nav';
	import { MarketingFooter } from '$lib/components/layout';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
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

	let { children }: { children: Snippet } = $props();

	// Derived state from page
	let pathname = $derived(page.url.pathname);
	let isAdminArea = $derived(pathname.startsWith('/admin'));
	let isTradingRoom = $derived(pathname.startsWith('/live-trading-rooms/') && pathname.split('/').length > 2);
	let isEmbedArea = $derived(pathname.startsWith('/embed'));
	let isAdmin = $derived($isAdminUser);

	onMount(() => {
		if (browser) {
			registerServiceWorker();
			initPerformanceMonitoring();
			initializeConsent();
		}
	});
</script>

<svelte:head>
	<link rel="dns-prefetch" href="https://fonts.googleapis.com" />
	<link rel="dns-prefetch" href="https://fonts.gstatic.com" />
	<link rel="preconnect" href="https://fonts.googleapis.com" crossorigin="anonymous" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap" media="all" />
	<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
	<meta name="theme-color" content="#0a101c" />
</svelte:head>

{#if isAdminArea || isTradingRoom || isEmbedArea}
	{@render children()}
{:else}
	<div class="min-h-screen bg-rtp-bg text-rtp-text" class:has-admin-toolbar={isAdmin}>
		<AdminToolbar />
		<NavBar />
		<main>
			{@render children()}
		</main>
		<MarketingFooter />
		<ConsentBanner />
		<ConsentPreferencesModal />
		<ConsentSettingsButton position="bottom-left" />
	</div>
{/if}

<style>
	.has-admin-toolbar {
		--admin-toolbar-height: 46px;
		padding-top: var(--admin-toolbar-height);
	}
</style>
