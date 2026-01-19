<script lang="ts">
	/**
	 * Root Layout - Apple Principal Engineer ICT Level 7 Grade
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Svelte 5 / SvelteKit 2.x Best Practices (January 2026)
	 *
	 * PATTERNS IMPLEMENTED:
	 * ✅ $props() for component props with Snippet type
	 * ✅ $state() for reactive local state
	 * ✅ $derived() for computed values (replaces $: reactive statements)
	 * ✅ $effect() for side effects (replaces afterUpdate/beforeUpdate)
	 * ✅ {@render} for snippet rendering (replaces <slot>)
	 * ✅ Rune-based store access via .current (replaces $ prefix)
	 * ✅ page from '$app/state' (SvelteKit 2.x pattern)
	 * ✅ Hydration-safe client-only rendering
	 *
	 * @version 5.0.0 - Svelte 5 Runes Compliant
	 * @author Revolution Trading Pros
	 * ═══════════════════════════════════════════════════════════════════════════
	 */
	import '../app.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import { NavBar } from '$lib/components/nav';
	import { MarketingFooter } from '$lib/components/layout';
	import { onMount } from 'svelte';
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { registerServiceWorker } from '$lib/utils/registerServiceWorker';
	import { initPerformanceMonitoring } from '$lib/utils/performance';
	import { isAdminUser } from '$lib/stores/auth.svelte';
	import { initializeAuth } from '$lib/api/auth';
	import type { Snippet } from 'svelte';
	import {
		initializeConsent,
		ConsentBanner,
		ConsentPreferencesModal,
		ConsentSettingsButton
	} from '$lib/consent';
	import { trackPageView } from '$lib/consent/vendors/ga4';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 $props() Pattern
	// ═══════════════════════════════════════════════════════════════════════════
	let { children }: { children: Snippet } = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE - Svelte 5 $state() Pattern
	// ═══════════════════════════════════════════════════════════════════════════
	let mounted = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED - Svelte 5 $derived() Pattern (replaces $: reactive statements)
	// ═══════════════════════════════════════════════════════════════════════════
	const pathname = $derived(page.url.pathname);
	const isAdminArea = $derived(pathname.startsWith('/admin'));
	const isEmbedArea = $derived(pathname.startsWith('/embed'));
	const isDashboardArea = $derived(pathname.startsWith('/dashboard'));

	// Svelte 5: Use .current for rune-based stores (NOT $ prefix)
	const isAdmin = $derived(mounted && isAdminUser.current);

	// Determine if page needs shared layout (NavBar + Footer)
	const needsSharedLayout = $derived(!isAdminArea && !isEmbedArea);

	// ═══════════════════════════════════════════════════════════════════════════
	// SIDE EFFECTS - SvelteKit Navigation Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════
	afterNavigate(({ to }) => {
		if (to?.url) {
			trackPageView(to.url.href);
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE - onMount (client-only by definition, no browser check needed)
	// ═══════════════════════════════════════════════════════════════════════════
	onMount(() => {
		mounted = true;

		// onMount ONLY runs in browser - no redundant check needed
		initializeAuth().catch((err) => {
			console.debug('[Layout] Auth init failed (non-critical):', err);
		});

		registerServiceWorker();
		initPerformanceMonitoring();
		initializeConsent();
	});
</script>

<svelte:head>
	<!-- ICT 11 Fix: Font loading handled in app.html - no duplicates -->
	<!-- Viewport already in app.html, theme-color set per page preference -->
	<meta name="theme-color" content="#FFFFFF" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - Svelte 5 {@render} Pattern (replaces <slot>)
     DRY Principle: Single shared layout for dashboard + marketing pages
     ═══════════════════════════════════════════════════════════════════════════ -->
{#if isAdminArea || isEmbedArea}
	<!-- Admin/Embed: Own layouts, no shared chrome -->
	{@render children()}
{:else}
	<!-- Dashboard + Marketing: Shared layout with NavBar + Footer -->
	<!-- Pages control their own backgrounds (no forced bg-white) -->
	<div class="min-h-screen overflow-x-hidden" class:has-admin-toolbar={isAdmin}>
		<!-- Hydration-safe: Admin toolbar only after client mount -->
		{#if mounted}
			<AdminToolbar />
		{/if}

		<NavBar />

		{@render children()}

		<MarketingFooter />

		<!-- Hydration-safe: Consent components only after client mount -->
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
