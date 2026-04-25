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
	import ClientOnly from '$lib/components/ssr/ClientOnly.svelte';
	import Seo from '$lib/seo/Seo.svelte';
	import { resolveSEO } from '$lib/seo/resolve';
	import type { SEOInput, RouteSEOContext, SEODefaults } from '$lib/seo/types';
	import { NavBar } from '$lib/components/nav';
	import { onMount } from 'svelte';
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { browser, dev } from '$app/environment';
	import { registerServiceWorker } from '$lib/utils/registerServiceWorker';
	import { initPerformanceMonitoring } from '$lib/utils/performance';
	import { isAdminUser } from '$lib/stores/auth.svelte';
	import { initializeAuth } from '$lib/api/auth';
	import type { Snippet } from 'svelte';
	import { initializeConsent } from '$lib/consent';
	import { trackPageView } from '$lib/consent/vendors/ga4';
	// sv-agentation — dev-only inspector that surfaces Svelte component sources
	// for AI agents. Lazy-loaded so it never lands in the production bundle.
	let AgentationComponent = $state<any>(null);
	const AGENTATION_WORKSPACE_ROOT = '/Users/billyribeiro/Desktop/my-websites/Revolution-trading-pros';

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
	const isMarketingPage = $derived(
		!isAdminArea && !isDashboardArea && !isCmsArea && !isEmbedArea
	);

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
				console.debug('[Layout] Page view tracking failed (non-critical):', err);
			}
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE - onMount (client-only by definition, no browser check needed)
	// ═══════════════════════════════════════════════════════════════════════════
	onMount(() => {
		mounted = true;

		// Dev tooling — client-side only (prevents hydration mismatch).
		if (dev) {
			import('sv-agentation')
				.then((mod) => {
					AgentationComponent = mod.Agentation;
				})
				.catch(() => {
					// Package not installed — silently skip.
				});
		}

		// All initializers wrapped in try/catch to prevent render interruption
		initializeAuth().catch((err) => {
			console.debug('[Layout] Auth init failed (non-critical):', err);
		});

		try {
			registerServiceWorker();
		} catch (err) {
			console.debug('[Layout] Service worker registration failed (non-critical):', err);
		}

		try {
			initPerformanceMonitoring();
		} catch (err) {
			console.debug('[Layout] Performance monitoring init failed (non-critical):', err);
		}

		try {
			initializeConsent();
		} catch (err) {
			console.debug('[Layout] Consent init failed (non-critical):', err);
		}
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     SEO - Single ownership layer for all <head> SEO output
     ═══════════════════════════════════════════════════════════════════════════ -->
<Seo seo={resolvedSeo} />

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
     DEVELOPMENT TOOLS - Svelte DevTools (development only)
     ═══════════════════════════════════════════════════════════════════════════ -->
{#if browser && dev && AgentationComponent}
	<AgentationComponent workspaceRoot={AGENTATION_WORKSPACE_ROOT} />
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - Svelte 5 {@render} Pattern (replaces <slot>)
     DRY Principle: Single shared layout for dashboard + marketing pages
     ═══════════════════════════════════════════════════════════════════════════ -->
{#if isAdminArea || isEmbedArea || isCmsArea}
	<!-- Admin/Embed/CMS: Own layouts, no shared chrome, no marketing-page theme.
	     The `.marketing-page-root` class is intentionally absent here so the
	     `app.css` body/universal rules don't apply. See
	     docs/audits/CSS_CASCADE_AUDIT_2026-04-25.md. -->
	{@render props.children()}
{:else}
	<!-- Dashboard + Marketing: Shared layout with NavBar.
	     Only marketing pages get the `marketing-page-root` class; dashboard
	     keeps its own theme via lib/styles/dashboard.css. -->
	<div
		class="min-h-screen flex flex-col min-w-0"
		class:marketing-page-root={isMarketingPage}
		class:has-admin-toolbar={showAdminToolbar}
	>
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
