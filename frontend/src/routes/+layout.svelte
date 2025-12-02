<script lang="ts">
	/**
	 * Root Layout - Global shell for all routes
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Enterprise-grade root layout with:
	 * - Multi-provider analytics (GA4, Backend, Console)
	 * - Consent-aware tracking
	 * - Performance monitoring
	 * - Service worker registration
	 *
	 * Updated to Svelte 5 runes syntax (December 2025)
	 *
	 * @version 4.0.0
	 * @author Revolution Trading Pros
	 */
	import '../app.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import { NavBar } from '$lib/components/nav';
	import { MarketingFooter } from '$lib/components/layout';
	import PopupModal from '$lib/components/PopupModal.svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { page } from '$app/stores';
	import { registerServiceWorker } from '$lib/utils/registerServiceWorker';
	import { initPerformanceMonitoring } from '$lib/utils/performance';
	import { authStore, user as userStore, isAdminUser } from '$lib/stores/auth';
	import type { Snippet } from 'svelte';

	// Svelte 5: Props with children snippet
	let { children }: { children: Snippet } = $props();

	// Consent System
	import {
		ConsentBanner,
		ConsentPreferencesModal,
		ConsentSettingsButton,
		initializeConsent,
		trackGA4PageView,
		consentStore,
		onConsentChange,
	} from '$lib/consent';

	// Observability System (New Multi-Provider Analytics)
	import {
		initializeAnalytics,
		updateAnalyticsConsent,
		trackPageView,
		identify,
		flushAnalytics,
	} from '$lib/observability';

	// ═══════════════════════════════════════════════════════════════════════════
	// Svelte 5 Runes - Route-based layout detection
	// ═══════════════════════════════════════════════════════════════════════════

	// Derived state from page store
	let pathname = $derived($page.url.pathname);
	let isAdminArea = $derived(pathname.startsWith('/admin'));
	let isTradingRoom = $derived(pathname.startsWith('/live-trading-rooms/') && pathname.split('/').length > 2);
	let isEmbedArea = $derived(pathname.startsWith('/embed'));
	let isAuthPage = $derived(pathname.startsWith('/login') || pathname.startsWith('/signup') || pathname.startsWith('/register'));

	// Show marketing chrome only on public pages
	let showMarketingChrome = $derived(!isAdminArea && !isTradingRoom && !isEmbedArea);

	// Use centralized admin check from auth store
	let isAdmin = $derived($isAdminUser);

	// Track if initial page view has been sent (prevent double tracking)
	let initialPageViewSent = false;

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle & Effects
	// ═══════════════════════════════════════════════════════════════════════════

	// Initialize performance optimizations, consent, and analytics systems
	onMount(() => {
		if (browser) {
			// Register service worker for offline support and caching
			registerServiceWorker();

			// Initialize Core Web Vitals monitoring
			initPerformanceMonitoring();

			// Initialize consent management system
			// This loads stored preferences, applies Google Consent Mode,
			// and loads vendors (GA4, Meta Pixel) based on consent
			const unsubscribeConsent = initializeConsent();

			// Get initial consent state
			const initialConsent = consentStore.getState();

			// Initialize multi-provider analytics system
			// This sets up GA4, Backend API, and Console adapters
			initializeAnalytics({
				consent: {
					analytics: initialConsent.analytics,
					marketing: initialConsent.marketing,
				},
			});

			// Subscribe to consent changes and sync with analytics orchestrator
			const unsubscribeConsentChanges = onConsentChange((event: CustomEvent) => {
				const consent = event.detail;
				if (consent && typeof consent === 'object') {
					updateAnalyticsConsent({
						analytics: Boolean(consent.analytics),
						marketing: Boolean(consent.marketing),
					});
				}
			});

			// Identify user if logged in
			const user = $userStore;
			if (user?.id) {
				identify(String(user.id), {
					email: user.email,
					name: user.name || user.first_name || '',
				});
			}

			// Cleanup on unmount
			return () => {
				// Flush any pending analytics before cleanup
				flushAnalytics();
				unsubscribeConsent();
				unsubscribeConsentChanges();
			};
		}
	});

	// Track page views on navigation (SPA) - Svelte 5 effect
	$effect(() => {
		if (browser && pathname) {
			// Small delay to ensure page title has updated
			setTimeout(() => {
				// Track via consent system (backward compatibility)
				trackGA4PageView(window.location.href);

				// Track via multi-provider analytics (new system)
				// Skip if this is the initial page view (already tracked by GA4 config)
				if (initialPageViewSent) {
					trackPageView({
						path: pathname,
						title: document.title,
						referrer: document.referrer,
					});
				} else {
					initialPageViewSent = true;
					// Track initial page view
					trackPageView({
						path: pathname,
						title: document.title,
						referrer: document.referrer,
					});
				}
			}, 100);
		}
	});

	// Sync user identity when auth state changes
	$effect(() => {
		if (browser && $userStore) {
			const user = $userStore;
			if (user?.id) {
				identify(String(user.id), {
					email: user.email,
					name: user.name || user.first_name || '',
				});
			}
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

	<!-- Analytics: Preconnect to Google Analytics (non-blocking) -->
	<link rel="dns-prefetch" href="https://www.googletagmanager.com" />
	<link rel="dns-prefetch" href="https://www.google-analytics.com" />
	<link rel="preconnect" href="https://www.googletagmanager.com" crossorigin="anonymous" />

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
	{@render children()}
{:else}
	<div class="min-h-screen bg-rtp-bg text-rtp-text" class:has-admin-toolbar={isAdmin}>
		<!-- Admin Toolbar (only shows for logged-in admins) -->
		<AdminToolbar />

		<!-- Navigation -->
		<NavBar />

		<main>
			{@render children()}
		</main>

		<!-- Marketing Footer -->
		<MarketingFooter />

		<!-- Global Popup Modal - Enterprise Grade: Only shows when explicitly triggered -->
		<PopupModal />

		<!-- Consent Management UI -->
		<ConsentBanner />
		<ConsentPreferencesModal />
		<ConsentSettingsButton position="bottom-left" />
	</div>
{/if}

<style>
	/* When admin toolbar is visible, add top padding to account for it */
	.has-admin-toolbar {
		--admin-toolbar-height: 46px;
		padding-top: var(--admin-toolbar-height);
	}
</style>
