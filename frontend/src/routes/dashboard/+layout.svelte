<!--
	Dashboard Layout - Member Dashboard with Sidebar
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	Svelte 5 (December 2025) Best Practices

	SERVER-SIDE AUTH PATTERN (Svelte 5 Recommended):
	- Auth is validated server-side in hooks.server.ts BEFORE this component loads
	- User data is passed from +layout.server.ts via load function
	- No client-side auth polling needed - server already handled it
	- Memberships fetched client-side for dynamic updates

	Svelte 5 Features:
	- $props() for component props including data from load
	- $state() for reactive state management
	- $derived() for computed values
	- $effect() for side effects
	- Snippet for children rendering

	@version 4.1.0 - ICT 11+ Server-Side Auth (Aligned to Admin Layout)
	@author Revolution Trading Pros
-->
<script lang="ts">
	// Dashboard Design System - Only loaded in dashboard area, not globally
	import '$lib/styles/main.css';
	import '$lib/styles/dashboard.css'; // Dashboard-specific styles - ISOLATED from front pages

	import { goto } from '$app/navigation';
	import { page } from '$app/stores'; // FIXED: Use $app/stores for consistency with Admin Layout
	import { browser } from '$app/environment';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth.svelte';
	import {
		getUserMemberships,
		type UserMembershipsResponse,
		type MembershipType
	} from '$lib/api/user-memberships';
	import DashboardSidebar from '$lib/components/dashboard/DashboardSidebar.svelte';
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';

	import type { Snippet } from 'svelte'; // FIXED: Separate type import for clarity

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 Pattern
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		children: Snippet;
		data: {
			user: {
				id: string;
				email: string;
				name?: string;
				role?: string;
			} | null;
		};
	}

	let { children, data }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Loading state for memberships data
	let isLoadingData = $state(false);

	// Sidebar state - user can manually toggle
	let userToggledSidebar = $state<boolean | null>(null);

	// Memberships data
	let membershipsData = $state<UserMembershipsResponse | null>(null);

	// Track if initial data load is complete
	let isInitialized = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Svelte 5 Pattern
	// ═══════════════════════════════════════════════════════════════════════════

	// User data for sidebar - prefer server data, fall back to client store
	const userData = $derived({
		id: data.user?.id ?? $user?.id?.toString() ?? '',
		email: data.user?.email ?? $user?.email ?? '',
		name: data.user?.name ?? $user?.name ?? $user?.email?.split('@')[0] ?? 'Member',
		avatar: $user?.avatar_url ?? null,
		// Pass ALL membership types for sidebar to render dynamically
		memberships: [
			...(membershipsData?.tradingRooms ?? []),
			...(membershipsData?.alertServices ?? []),
			...(membershipsData?.courses ?? []),
			...(membershipsData?.indicators ?? []),
			...(membershipsData?.scanners ?? []),
			...(membershipsData?.weeklyWatchlist ?? []),
			...(membershipsData?.premiumReports ?? [])
		]
			.filter((m: { status: string }) => m.status === 'active')
			.map((m: { name: string; slug: string; icon?: string; type: MembershipType }) => ({
				name: m.name,
				slug: m.slug,
				icon: m.icon,
				type: m.type
			}))
	});

	// Auth is handled server-side, but we still check client store for logout detection
	const isUserAuthenticated = $derived(!!data.user || $isAuthenticated);

	// ═══════════════════════════════════════════════════════════════════════════
	// MEMBERSHIP ROUTES CONFIGURATION
	// Routes that should auto-collapse the main sidebar and show secondary nav
	// ═══════════════════════════════════════════════════════════════════════════

	const membershipRoutes: Record<
		string,
		{
			title: string;
			items: Array<{
				href: string;
				icon: string;
				text: string;
				submenu?: Array<{ href: string; icon: string; text: string }>;
			}>;
		}
	> = {
		'/dashboard/day-trading-room': {
			title: 'Day Trading Room',
			items: [
				{
					href: '/dashboard/day-trading-room',
					icon: 'layout-dashboard',
					text: 'Day Trading Room Dashboard'
				},
				{
					href: '/dashboard/day-trading-room/daily-videos',
					icon: 'video',
					text: 'Premium Daily Videos'
				},
				{
					href: '/dashboard/day-trading-room/learning-center',
					icon: 'school',
					text: 'Learning Center'
				},
				{
					href: '/dashboard/day-trading-room/trading-room-archive',
					icon: 'archive',
					text: 'Trading Room Archives'
				},
				{
					href: '#',
					icon: 'users',
					text: 'Meet the Traders',
					submenu: [
						{
							href: '/dashboard/day-trading-room/meet-the-traders/billy-ribeiro',
							icon: '',
							text: 'Billy Ribeiro'
						},
						{
							href: '/dashboard/day-trading-room/meet-the-traders/freddie-ferber',
							icon: '',
							text: 'Freddie Ferber'
						},
						{
							href: '/dashboard/day-trading-room/meet-the-traders/shao-wan',
							icon: '',
							text: 'Shao Wan'
						}
					]
				},
				{
					href: '#',
					icon: 'shopping-cart',
					text: 'Trader Store',
					submenu: [
						{
							href: '/dashboard/day-trading-room/meet-the-traders/billy-ribeiro/trader-store',
							icon: '',
							text: 'Billy Ribeiro'
						},
						{
							href: '/dashboard/day-trading-room/meet-the-traders/freddie-ferber/trader-store',
							icon: '',
							text: 'Freddie Ferber'
						},
						{
							href: '/dashboard/day-trading-room/meet-the-traders/shao-wan/trader-store',
							icon: '',
							text: 'Shao Wan'
						}
					]
				}
			]
		},
		'/dashboard/small-account-mentorship': {
			title: 'Small Account Mentorship',
			items: [
				{
					href: '/dashboard/small-account-mentorship',
					icon: 'layout-dashboard',
					text: 'Small Account Mentorship Dashboard'
				},
				{
					href: '/dashboard/small-account-mentorship/daily-videos',
					icon: 'video',
					text: 'Premium Daily Videos'
				},
				{
					href: '/dashboard/small-account-mentorship/learning-center',
					icon: 'school',
					text: 'Learning Center'
				},
				{
					href: '/dashboard/small-account-mentorship/trading-room-archive',
					icon: 'archive',
					text: 'Trading Room Archives'
				},
				{
					href: '#',
					icon: 'users',
					text: 'Meet the Traders',
					submenu: [
						{
							href: '/dashboard/small-account-mentorship/meet-the-traders/billy-ribeiro',
							icon: '',
							text: 'Billy Ribeiro'
						},
						{
							href: '/dashboard/small-account-mentorship/meet-the-traders/freddie-ferber',
							icon: '',
							text: 'Freddie Ferber'
						},
						{
							href: '/dashboard/small-account-mentorship/meet-the-traders/shao-wan',
							icon: '',
							text: 'Shao Wan'
						}
					]
				},
				{
					href: '#',
					icon: 'shopping-cart',
					text: 'Trader Store',
					submenu: [
						{
							href: '/dashboard/small-account-mentorship/meet-the-traders/billy-ribeiro/trader-store',
							icon: '',
							text: 'Billy Ribeiro'
						},
						{
							href: '/dashboard/small-account-mentorship/meet-the-traders/freddie-ferber/trader-store',
							icon: '',
							text: 'Freddie Ferber'
						},
						{
							href: '/dashboard/small-account-mentorship/meet-the-traders/shao-wan/trader-store',
							icon: '',
							text: 'Shao Wan'
						}
					]
				}
			]
		},
		'/dashboard/swing-trading-room': {
			title: 'Swing Trading Room',
			items: [
				{
					href: '/dashboard/swing-trading-room',
					icon: 'layout-dashboard',
					text: 'Swing Trading Dashboard'
				},
				{
					href: '/dashboard/swing-trading-room/daily-videos',
					icon: 'video',
					text: 'Premium Daily Videos'
				},
				{
					href: '/dashboard/swing-trading-room/learning-center',
					icon: 'school',
					text: 'Learning Center'
				}
			]
		},
		'/dashboard/small-accounts-room': {
			title: 'Small Accounts Room',
			items: [
				{
					href: '/dashboard/small-accounts-room',
					icon: 'layout-dashboard',
					text: 'Small Accounts Dashboard'
				},
				{
					href: '/dashboard/small-accounts-room/daily-videos',
					icon: 'video',
					text: 'Premium Daily Videos'
				},
				{
					href: '/dashboard/small-accounts-room/learning-center',
					icon: 'school',
					text: 'Learning Center'
				}
			]
		},
		'/dashboard/spx-profit-pulse': {
			title: 'SPX Profit Pulse',
			items: [
				{
					href: '/dashboard/spx-profit-pulse',
					icon: 'layout-dashboard',
					text: 'Tr3ndy SPX Alerts Service Dashboard'
				},
				{
					href: '/dashboard/spx-profit-pulse/premium-videos',
					icon: 'video',
					text: 'Premium Videos'
				},
				{
					href: '/dashboard/spx-profit-pulse/learning-center',
					icon: 'school',
					text: 'Learning Center'
				},
				{
					href: '#',
					icon: 'users',
					text: 'Meet Jonathan',
					submenu: [
						{ href: '/dashboard/spx-profit-pulse/jonathan-mckeever', icon: '', text: 'Overview' },
						{
							href: '/dashboard/spx-profit-pulse/jonathan-mckeever/trading-strategies',
							icon: '',
							text: 'Trading Strategies'
						},
						{
							href: '/dashboard/spx-profit-pulse/jonathan-mckeever/trader-store',
							icon: '',
							text: 'Trader Store'
						}
					]
				},
				{
					href: '/dashboard/spx-profit-pulse/jonathan-mckeever/trader-store',
					icon: 'shopping-cart',
					text: 'Trader Store'
				}
			]
		},
		'/dashboard/explosive-swings': {
			title: 'Explosive Swings',
			items: [
				{
					href: '/dashboard/explosive-swings',
					icon: 'layout-dashboard',
					text: 'Explosive Swings Dashboard'
				},
				{ href: '/dashboard/explosive-swings/start-here', icon: 'info', text: 'Start Here' },
				{ href: '/dashboard/explosive-swings/alerts', icon: 'bolt', text: 'Alerts' },
				{
					href: '/dashboard/explosive-swings/trade-tracker',
					icon: 'chart-line',
					text: 'Trade Tracker'
				},
				{ href: '/dashboard/explosive-swings/watchlist', icon: 'list', text: 'Watchlist' },
				{ href: '/dashboard/explosive-swings/video-library', icon: 'video', text: 'Video Library' },
				{ href: '/dashboard/explosive-swings/favorites', icon: 'star', text: 'Favorites' }
			]
		},
		'/dashboard/weekly-watchlist': {
			title: 'Weekly Watchlist',
			items: [
				{ href: '/dashboard/weekly-watchlist', icon: 'layout-dashboard', text: 'Weekly Watchlist' },
				{
					href: '/dashboard/weekly-watchlist/watchlist-rundown-archive',
					icon: 'video',
					text: 'Watchlist Rundown Archive'
				},
				{
					href: '/dashboard/weekly-watchlist/archive',
					icon: 'file',
					text: 'Weekly Watchlist Archive'
				}
			]
		},
		'/dashboard/account': {
			title: 'My Account',
			items: [
				{ href: '/dashboard/account/orders', icon: '', text: 'My Orders' },
				{ href: '/dashboard/account/subscriptions', icon: '', text: 'My Subscriptions' },
				{ href: '/dashboard/account/coupons', icon: '', text: 'Coupons' },
				{ href: '/dashboard/account/edit-address', icon: '', text: 'Billing Address' },
				{ href: '/dashboard/account/payment-methods', icon: '', text: 'Payment Methods' },
				{ href: '/dashboard/account/edit-account', icon: '', text: 'Account Details' },
				{ href: '/logout', icon: '', text: 'Log out' }
			]
		}
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED: Route-based computations
	// ═══════════════════════════════════════════════════════════════════════════

	// Get current membership route data (if on a membership page)
	let currentMembershipData = $derived.by(() => {
		const currentPath = $page.url.pathname; // FIXED: Use $page from stores
		for (const [route, routeData] of Object.entries(membershipRoutes)) {
			if (currentPath.startsWith(route)) {
				return routeData;
			}
		}
		return null;
	});

	// Check if on membership route (secondary sidebar visible)
	let isOnMembershipRoute = $derived(currentMembershipData !== null);

	// Dashboard home should never start in collapsed state
	let isDashboardHome = $derived.by(() => {
		const path = $page.url.pathname; // FIXED: Use $page from stores
		return path === '/dashboard' || path === '/dashboard/';
	});

	// SVELTE 5 BEST PRACTICE: Use $derived for computed state, not $effect to mutate state
	// This prevents layout shift by computing correct value BEFORE first render
	let sidebarCollapsed = $derived(
		isDashboardHome ? false : userToggledSidebar !== null ? userToggledSidebar : isOnMembershipRoute
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS - Svelte 5 Pattern (Aligned with Admin Layout)
	// ═══════════════════════════════════════════════════════════════════════════

	// Auth sync and data loading - runs once on mount
	$effect(() => {
		if (!browser) return;
		if (isInitialized) return;

		isInitialized = true;

		// Async IIFE for data loading
		(async () => {
			// ICT 11+ FIX: On page refresh, server has validated auth but client store is empty
			// We need to restore auth state before making API calls that require token
			if (data.user && !$isAuthenticated) {
				console.debug('[Dashboard] Server auth valid, syncing to client store...');

				// ALWAYS set user first from server data - this is the source of truth
				const serverUser = {
					id: parseInt(data.user.id) || 0,
					name: data.user.name || data.user.email?.split('@')[0] || 'Member',
					email: data.user.email || '',
					role: data.user.role,
					created_at: new Date().toISOString()
				};
				authStore.setUser(serverUser);
				console.debug('[Dashboard] User synced to client store:', serverUser.email);

				// Try to refresh the token to get a valid access token in memory
				// The refresh token is persisted in localStorage
				try {
					const refreshed = await authStore.refreshToken();
					if (refreshed) {
						console.debug('[Dashboard] Token refreshed successfully');
					} else {
						console.debug('[Dashboard] Token refresh failed, will use cookies for API calls');
					}
				} catch (error) {
					console.warn('[Dashboard] Token refresh error (will use cookies):', error);
				}
			}

			// Load memberships data
			await loadMembershipsData();
		})();
	});

	// Auth guard - redirect on logout (client-side logout detection)
	$effect(() => {
		if (!browser) return;

		// If user logs out while on dashboard (client-side action), redirect to login
		// Server auth is already validated, this is for client-side logout
		if (!$isAuthenticated && !data.user) {
			const currentPath = $page.url.pathname;
			goto(`/login?redirect=${encodeURIComponent(currentPath)}`, { replaceState: true });
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadMembershipsData(): Promise<void> {
		if (!isUserAuthenticated) return;

		isLoadingData = true;

		try {
			membershipsData = await getUserMemberships();
		} catch (error) {
			console.error('[Dashboard] Failed to load memberships:', error);
			membershipsData = null;
		} finally {
			isLoadingData = false;
		}
	}

	// Handler for user manually toggling sidebar
	function handleSidebarToggle(collapsed: boolean) {
		userToggledSidebar = collapsed;
	}
</script>

<svelte:head>
	<meta name="robots" content="noindex, nofollow" />
	<title>Member Dashboard | Revolution Trading Pros</title>
</svelte:head>

<!-- Breadcrumbs Navigation - Full Width -->
<DashboardBreadcrumbs />

<!-- Dashboard Content - Flex layout matching WordPress exactly -->
<div class="dashboard dashboard-layout">
	<!-- Sidebar Navigation (LEFT) -->
	<DashboardSidebar
		user={userData}
		collapsed={sidebarCollapsed}
		onToggle={handleSidebarToggle}
		secondaryNavItems={currentMembershipData?.items ?? []}
		secondarySidebarTitle={currentMembershipData?.title ?? ''}
	/>

	<!-- Main Content Area - flex: 1 1 auto fills remaining space -->
	<!-- ICT11+ Fix: Changed from <main> to <div> - root +layout.svelte already provides <main id="main-content"> -->
	<div class="dashboard__main" class:has-secondary-sidebar={isOnMembershipRoute}>
		{#if isLoadingData}
			<div class="dashboard__loading-overlay">
				<div class="dashboard__loading-spinner"></div>
			</div>
		{/if}
		<!-- Content Wrapper - WordPress exact structure -->
		<div class="dashboard__content">
			<!-- Content Main - Contains page content -->
			<div class="dashboard__content-main">
				{@render children()}
			</div>
		</div>
	</div>
</div>

<!-- Dashboard Footer - Consistent across all dashboard pages -->
<MarketingFooter />

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * APPLE ICT 11+ RESPONSIVE DESIGN SYSTEM
	 * Modern CSS Best Practices (Nov/Dec 2025)
	 * ═══════════════════════════════════════════════════════════════════════════
	 * 
	 * BREAKPOINT STRATEGY (Industry Standard - Jan 2026):
	 * - CSS Custom Properties for maintainability
	 * - Range syntax for modern browsers
	 * - Container queries where appropriate
	 * - Logical properties for i18n support
	 * - Fluid typography and spacing
	 * 
	 * DEVICE TARGETS (Tailwind/Apple ICT 11+ Aligned):
	 * - Mobile: < 768px (iPhone SE to iPhone 15 Pro Max) - Sidebar hidden
	 * - Tablet+: >= 768px (iPad Mini+) - Sidebar visible
	 * - Desktop: >= 1024px (MacBook Air 13"+) - Full layout
	 * - Large: >= 1440px (MacBook Pro 16", iMac) - Extra spacing
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Breakpoints defined in app.css (SSOT) - use var(--bp-*) or hardcoded values in media queries */
	/* Layout uses global CSS custom properties from app.css */

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Collapsed Sidebar Hover Effect - WordPress Evidence-Based Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 * 
	 * SOURCE EVIDENCE: dashboard.8f78208b.css
	 * 
	 * WordPress behavior: Collapsed primary nav does NOT change width on hover.
	 * It only shows absolutely-positioned tooltip labels that appear OUTSIDE
	 * the sidebar element, preventing layout shift.
	 * 
	 * The nav item text/profile name are positioned with:
	 *   position: absolute;
	 *   left: 100%;  (appears outside sidebar)
	 *   transform: translate(0);  (on hover)
	 * 
	 * NO width change = NO layout shift
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Show tooltip labels on hover - these are absolutely positioned OUTSIDE sidebar */
	:global(.dashboard__nav-primary.is-collapsed a:hover .dashboard__nav-item-text),
	:global(.dashboard__nav-primary.is-collapsed a:hover .dashboard__profile-name) {
		opacity: 1;
		visibility: visible;
		transform: translate(0);
	}

	/* Hover state visual feedback on the icon background */
	:global(.dashboard__nav-primary.is-collapsed a:hover:before) {
		transform: scale(1);
		background-color: rgba(0, 0, 0, 0.2);
	}

	:global(.dashboard__nav-primary.is-collapsed a:hover:after) {
		transform: scaleX(0);
	}

	:global(.dashboard__nav-primary.is-collapsed a:hover .dashboard__nav-item-icon),
	:global(.dashboard__nav-primary.is-collapsed a:hover .dashboard__profile-photo) {
		transform: scale(0.9);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Dashboard Layout Container
	 * Modern Flexbox with logical properties
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard {
		display: flex;
		flex-direction: row;
		/* ICT11+ Fix: Removed min-block-size: 100vh which was causing footer to be pushed below viewport
		   when dashboard layout is nested inside root layout's shared chrome */
		flex: 1; /* Allow dashboard to grow within parent flex container */
		position: relative;
		isolation: isolate; /* Create stacking context */
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Main Content Area
	 * Container query support for component-level responsiveness
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__main {
		flex: 1 1 auto;
		min-inline-size: 0; /* Prevent flex overflow */
		background-color: #efefef;
		position: relative;
		container-type: inline-size;
		container-name: dashboard-main;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Content Wrapper - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md:427-432
	 * WordPress CSS: display: flex; flex-flow: row nowrap;
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Content Main - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md:434-440
	 * WordPress CSS: flex: 1 1 auto; min-width: 0; border-right: 1px solid #dbdbdb;
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-main {
		flex: 1 1 auto;
		min-width: 0;
		border-right: 1px solid #dbdbdb;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Content Sidebar (RIGHT) - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md:460-474
	 * Hidden by default, shows at 1080px+ breakpoint
	 * Width: 260px fixed
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Loading overlay for data fetching */
	.dashboard__loading-overlay {
		position: absolute;
		inset: 0; /* Shorthand for top/right/bottom/left: 0 */
		background: rgb(239 239 239 / 0.8);
		backdrop-filter: blur(4px);
		display: grid;
		place-items: center;
		z-index: var(--z-overlay);
	}

	.dashboard__loading-spinner {
		inline-size: 32px;
		block-size: 32px;
		border: 3px solid #e0e0e0;
		border-block-start-color: #0a2335;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		will-change: transform; /* Performance hint */
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Modern Range Syntax
	 * Apple ICT 11+ Standard: Mobile-first with range queries
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile: < 768px (iPhone, Android phones) */
	@media (width < 768px) {
		.dashboard {
			flex-direction: column;
		}

		.dashboard__main {
			inline-size: 100%;
			padding-block-end: var(--mobile-footer-height);
		}

		/* No secondary sidebar margin on mobile */
		.dashboard__main.has-secondary-sidebar {
			margin-inline-start: 0;
		}
	}

	/* Desktop: >= 768px - Sidebar visible (Industry Standard) */
	/* No special rules needed - sidebar stays visible via DashboardSidebar component */

	/* Desktop: >= 1024px (MacBook Air 13"+, iMac, Studio Display) */
	/* Sidebar is static in flex layout, main content fills remaining space */

	/* Large Desktop: >= 1440px (MacBook Pro 16", iMac 27") */
	@media (width >= 1440px) {
		.dashboard__main {
			/* Additional spacing for large screens */
			max-inline-size: 1920px;
			margin-inline: auto;
		}
	}

	/* Ultra-wide: >= 1920px (Pro Display XDR, Studio Display) */
	@media (width >= 1920px) {
		.dashboard__main {
			max-inline-size: 2560px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * ACCESSIBILITY & USER PREFERENCES
	 * Apple ICT 11+ Standard: Respect user preferences
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Reduced Motion - Accessibility */
	@media (prefers-reduced-motion: reduce) {
		.dashboard__loading-spinner {
			animation: none;
		}

		* {
			animation-duration: 0.01ms !important;
			animation-iteration-count: 1 !important;
			transition-duration: 0.01ms !important;
		}
	}

	/* High Contrast Mode */
	@media (prefers-contrast: high) {
		.dashboard__loading-overlay {
			background: rgb(239 239 239 / 0.95);
			backdrop-filter: none;
		}

		.dashboard__loading-spinner {
			border-width: 4px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CONTAINER QUERIES - Component-level responsiveness
	 * Modern alternative to media queries for component sizing
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@container dashboard-main (inline-size < 600px) {
		/* Adjust child components when main area is narrow */
		/* This allows sidebar to be open while main content adapts */
	}

	@container dashboard-main (inline-size >= 900px) {
		/* Optimize layout when main area has more space */
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * PRINT STYLES - Professional print output
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media print {
		.dashboard {
			display: block;
		}

		.dashboard__loading-overlay {
			display: none;
		}

		.dashboard__main {
			background-color: white;
		}
	}
</style>