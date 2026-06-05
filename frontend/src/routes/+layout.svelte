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
	// Global typography/reset only. Page and component visuals own their scoped CSS.
	import '../app.css';

	// Marketing CSS — imported at the root so the layout tree topology stays
	// stable across route transitions (no component mount/unmount on
	// /login → /admin etc., which previously triggered the auth-store fan-out
	// cascade — see commit 01c4eecfe). The `.marketing-shell` class is only
	// applied on marketing pages, so admin/dashboard/cms get zero visual
	// effect from this import (rules don't match their DOM).
	import '../marketing.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import ClientOnly from '$lib/components/ssr/ClientOnly.svelte';
	import Seo from '$lib/seo/Seo.svelte';
	import { resolveSEO } from '$lib/seo/resolve';
	import { createPageSeoContext } from '$lib/seo/page-seo-context.svelte';
	import type { SEOInput, RouteSEOContext, SEODefaults } from '$lib/seo/types';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';
	import { NavBar } from '$lib/components/nav';
	import { onMount } from 'svelte';
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { registerServiceWorker } from '$lib/utils/registerServiceWorker';
	import { initPerformanceMonitoring } from '$lib/utils/performance';
	import { isAdminUser } from '$lib/stores/auth.svelte';
	import { initializeAuth } from '$lib/api/auth';
	import type { Snippet } from 'svelte';
	import { initializeConsent } from '$lib/consent';
	import { trackPageView } from '$lib/consent/vendors/ga4';
	import { logger } from '$lib/utils/logger';

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
	const isCmsArea = $derived(pathname.startsWith('/cms'));
	const isDashboardArea = $derived(pathname.startsWith('/dashboard'));
	// `.marketing-page-root` opts a route INTO the global app.css light theme
	// (body bg + universal border color). Backend surfaces (admin, dashboard,
	// cms, embed) are intentionally NOT marketing pages — they paint their
	// own surfaces and must remain isolated from the global theme. See
	// docs/audits/CSS_CASCADE_AUDIT_2026-04-25.md.
	const isMarketingPage = $derived(!isAdminArea && !isDashboardArea && !isCmsArea && !isEmbedArea);

	// ═══════════════════════════════════════════════════════════════════════════
	// SEO - Unified SEO resolution (single ownership layer)
	// Page-level SEO overrides come from page.data.seo (set in +page.server.ts)
	// ═══════════════════════════════════════════════════════════════════════════
	const pageSeo = $derived((page.data as { seo?: SEOInput })?.seo);

	// Page-level SEO context — allows legacy `<SEOHead>` components to
	// contribute their props to the unified Seo renderer (synchronous
	// mutation during child <script> evaluation; see page-seo-context.svelte.ts).
	const pageSeoCtx = createPageSeoContext();

	const resolvedSeo = $derived(
		resolveSEO(
			{ ...props.data.seoContext, pathname: page.url.pathname },
			props.data.seoDefaults,
			pageSeo,
			pageSeoCtx.value ? pageSeoCtx.value() : null
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
				logger.error('[Layout] Page view tracking failed (non-critical):', err);
			}
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE - onMount (client-only by definition, no browser check needed)
	// ═══════════════════════════════════════════════════════════════════════════
	onMount(() => {
		mounted = true;

		// All initializers wrapped in try/catch to prevent render interruption
		initializeAuth().catch((err) => {
			logger.error('[Layout] Auth init failed (non-critical):', err);
		});

		try {
			registerServiceWorker();
		} catch (err) {
			logger.error('[Layout] Service worker registration failed (non-critical):', err);
		}

		try {
			initPerformanceMonitoring();
		} catch (err) {
			logger.error('[Layout] Performance monitoring init failed (non-critical):', err);
		}

		try {
			initializeConsent();
		} catch (err) {
			logger.error('[Layout] Consent init failed (non-critical):', err);
		}
	});
</script>

<svelte:head>
	<!-- Non-SEO head tags (theme-color, RSS feeds) stay here -->
	<meta name="theme-color" content="#FFFFFF" />

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
     TEMPLATE - Svelte 5 {@render} Pattern (replaces <slot>)
     DRY Principle: Single shared layout for dashboard + marketing pages
     ═══════════════════════════════════════════════════════════════════════════ -->
{#if isAdminArea || isEmbedArea || isCmsArea}
	<!-- Admin/Embed/CMS: Own layouts, no shared chrome, no marketing theme.
	     Marketing CSS is loaded globally (above) but inert here because the
	     `.marketing-shell` class is not applied. See
	     docs/audits/CSS_CASCADE_AUDIT_2026-04-25.md. -->
	{@render props.children()}
{:else}
	<!-- Marketing + Dashboard: Shared wrapper. Stable topology across route
	     transitions (commit 01c4eecfe deliberately kept the wrapper mounted
	     to avoid effect_update_depth_exceeded on the auth fan-out). The
	     marketing-shell class is what flips marketing routes into their dark
	     canvas; dashboard routes keep their own surfaces from dashboard.css. -->
	<div
		class="app-shell"
		class:marketing-shell={isMarketingPage}
		class:dark={isMarketingPage}
		class:has-admin-toolbar={showAdminToolbar}
	>
		<!-- ICT Level 7: ClientOnly prevents hydration mismatch for auth-dependent AdminToolbar -->
		<ClientOnly>
			{#if showAdminToolbar}
				<AdminToolbar />
			{/if}
		</ClientOnly>

		<NavBar />

		<main id="main-content" class="app-main">
			{@render props.children()}
		</main>

		{#if isMarketingPage}
			<MarketingFooter />
		{:else if isDashboardArea}
			<!-- Dashboard needs dark+marketing-shell token scope for the footer -->
			<div class="dark marketing-shell dashboard-footer-ctx">
				<MarketingFooter />
			</div>
		{/if}

		<!-- Consent UI: Re-enable when consent system is ready -->
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     SEO - Single ownership layer for all <head> SEO output
     MUST render AFTER children so page-level SEO context (written by
     <SEOHead> components during child <script> init) is available.
     ═══════════════════════════════════════════════════════════════════════════ -->
<Seo seo={resolvedSeo} />

<style>
	.has-admin-toolbar {
		--admin-toolbar-height: 46px;
		padding-top: var(--admin-toolbar-height);
	}

	:global(.dashboard-footer-ctx) {
		display: block;
		margin: 0;
		padding: 0;
	}

	.app-shell {
		display: flex;
		flex-direction: column;
		min-width: 0;
		min-height: 100vh;
		min-height: 100dvh;
	}

	.app-main {
		display: flex;
		flex: 1;
		flex-direction: column;
		min-width: 0;
		overflow-x: clip;
	}
</style>
