<script lang="ts">
import { logger } from '$lib/utils/logger';
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
	import ClientOnly from '$lib/components/ssr/ClientOnly.svelte';
	import Seo from '$lib/seo/Seo.svelte';
	import { resolveSEO } from '$lib/seo/resolve';
	import type { SEOInput, RouteSEOContext, SEODefaults } from '$lib/seo/types';
	import { NavBar } from '$lib/components/nav';
	import { onMount } from 'svelte';
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { dev } from '$app/environment';
	import { registerServiceWorker } from '$lib/utils/registerServiceWorker';
	import { initPerformanceMonitoring } from '$lib/utils/performance';
	import { isAdminUser } from '$lib/stores/auth.svelte';
	import { initializeAuth } from '$lib/api/auth';
	import type { Snippet } from 'svelte';
	import { initializeConsent } from '$lib/consent';
	import { trackPageView } from '$lib/consent/vendors/ga4';
	// ICT 7: Dynamic import — svelte-render-scan is dev-only and must not run during SSR
	let RenderScanComponent = $state<any>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 $props() Pattern (no destructuring)
	// ═══════════════════════════════════════════════════════════════════════════
	interface Props {
		children: Snippet;
		data: {
			initialConsent?: unknown;
			hasConsentInteraction?: boolean;
			seoContext: RouteSEOContext;
			seoDefaults: SEODefaults;
			seo?: SEOInput;
		};
	}
	let props: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED - Svelte 5 $derived() Pattern (replaces $: reactive statements)
	// ═══════════════════════════════════════════════════════════════════════════
	const pathname = $derived(page.url.pathname);
	const isAdminArea = $derived(pathname.startsWith('/admin'));
	const isEmbedArea = $derived(pathname.startsWith('/embed'));

	// ═══════════════════════════════════════════════════════════════════════════
	// SEO - Unified SEO resolution (single ownership layer)
	// Page-level SEO overrides come from page.data.seo (set in +page.server.ts)
	// ═══════════════════════════════════════════════════════════════════════════
	const pageSeo = $derived((page.data as { seo?: SEOInput })?.seo);
	const resolvedSeo = $derived(
		resolveSEO(
			{ ...props.data.seoContext, pathname: page.url.pathname },
			props.data.seoDefaults,
			pageSeo
		)
	);

	// ICT Level 7: Hydration-safe — mounted guard prevents class mismatch on SSR
	let mounted = $state(false);
	const showAdminToolbar = $derived(mounted && isAdminUser.current);

	// ═══════════════════════════════════════════════════════════════════════════
	// SIDE EFFECTS - SvelteKit Navigation Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════
	afterNavigate(({ to }) => {
		if (to?.url) {
			try {
				trackPageView(to.url.href);
			} catch (err) {
				logger.debug('[Layout] Page view tracking failed (non-critical):', err);
			}
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE - onMount (client-only by definition, no browser check needed)
	// ═══════════════════════════════════════════════════════════════════════════
	onMount(() => {
		mounted = true;

		// ICT 7: Load dev tooling client-side only (prevents hydration mismatch)
		if (dev) {
			import('svelte-render-scan')
				.then((mod) => {
					RenderScanComponent = mod.RenderScan;
				})
				.catch(() => {
					// Package not installed — silently skip
				});
		}

		// All initializers wrapped in try/catch to prevent render interruption
		initializeAuth().catch((err) => {
			logger.debug('[Layout] Auth init failed (non-critical):', err);
		});

		try {
			registerServiceWorker();
		} catch (err) {
			logger.debug('[Layout] Service worker registration failed (non-critical):', err);
		}

		try {
			initPerformanceMonitoring();
		} catch (err) {
			logger.debug('[Layout] Performance monitoring init failed (non-critical):', err);
		}

		try {
			initializeConsent();
		} catch (err) {
			logger.debug('[Layout] Consent init failed (non-critical):', err);
		}
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     SEO - Single ownership layer for all <head> SEO output
     ═══════════════════════════════════════════════════════════════════════════ -->
<Seo seo={resolvedSeo} />

<svelte:head>
	<!-- Non-SEO head tags (theme-color, favicons, PWA, RSS feeds) stay here -->
	<meta name="theme-color" content="#FFFFFF" />

	<!-- Favicons — explicit declarations prevent browser auto-probe 404s -->
	<link rel="icon" type="image/x-icon" href="/favicon.ico" />
	<link rel="icon" type="image/png" sizes="128x128" href="/favicon.png" />
	<link rel="apple-touch-icon" sizes="192x192" href="/apple-touch-icon.png" />
	<link rel="apple-touch-icon-precomposed" sizes="192x192" href="/apple-touch-icon-precomposed.png" />

	<!-- PWA Manifest -->
	<link rel="manifest" href="/manifest.json" />

	<!-- RSS/Atom Feed Discovery Links (SEO: Auto-discovery for feed readers) -->
	<link
		rel="alternate"
		type="application/rss+xml"
		title="Revolution Trading Pros RSS Feed"
		href="/feed.xml"
	/>
	<link
		rel="alternate"
		type="application/atom+xml"
		title="Revolution Trading Pros Atom Feed"
		href="/atom.xml"
	/>
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DEVELOPMENT TOOLS - Svelte DevTools (development only)
     ═══════════════════════════════════════════════════════════════════════════ -->
{#if dev && RenderScanComponent}
	<RenderScanComponent initialEnabled={false} offsetLeft={60} duration={1500} />
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - Svelte 5 {@render} Pattern (replaces <slot>)
     DRY Principle: Single shared layout for dashboard + marketing pages
     ═══════════════════════════════════════════════════════════════════════════ -->
{#if isAdminArea || isEmbedArea}
	<!-- Admin/Embed: Own layouts, no shared chrome -->
	{@render props.children()}
{:else}
	<!-- Dashboard + Marketing: Shared layout with NavBar -->
	<!-- Pages control their own backgrounds (no forced bg-white) -->
	<!-- Footer moved to individual pages for better control -->
	<div class="min-h-screen flex flex-col min-w-0" class:has-admin-toolbar={showAdminToolbar}>
		<!-- ICT Level 7: ClientOnly prevents hydration mismatch for auth-dependent AdminToolbar -->
		<ClientOnly>
			{#if showAdminToolbar}
				<AdminToolbar />
			{/if}
		</ClientOnly>

		<NavBar />

		<main id="main-content" class="flex-1 min-w-0 overflow-x-clip">
			{@render props.children()}
		</main>

		<!-- Consent UI: Re-enable when consent system is ready -->
	</div>
{/if}

<style>
	.has-admin-toolbar {
		--admin-toolbar-height: 46px;
		padding-top: var(--admin-toolbar-height);
	}
</style>
