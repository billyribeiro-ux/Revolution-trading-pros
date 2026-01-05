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

	@version 4.0.0 - ICT 11+ Server-Side Auth
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { browser } from '$app/environment';
	import { onMount, type Snippet } from 'svelte';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';
	import { getUserMemberships, type UserMembershipsResponse, type MembershipType } from '$lib/api/user-memberships';
	import DashboardSidebar from '$lib/components/dashboard/DashboardSidebar.svelte';
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';

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

	// Sidebar state
	let sidebarCollapsed = $state(false);

	// Memberships data
	let membershipsData = $state<UserMembershipsResponse | null>(null);

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
		].filter((m: { status: string }) => m.status === 'active')
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
	// DATA LOADING
	// Server-side auth is complete, just load memberships client-side
	// ICT 11+ FIX: Must sync auth state before loading memberships
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(async () => {
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
	});

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

	// ═══════════════════════════════════════════════════════════════════════════
	// WATCH AUTH CHANGES - Client-side logout detection
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		// If user logs out while on dashboard (client-side action), redirect to login
		// Server auth is already validated, this is for client-side logout
		if (!$isAuthenticated && !data.user && browser) {
			const currentPath = page?.url?.pathname ?? '/dashboard';
			goto(`/login?redirect=${encodeURIComponent(currentPath)}`, { replaceState: true });
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// AUTO-COLLAPSE SIDEBAR ON MEMBERSHIP DASHBOARD PAGES
	// When user navigates to a membership dashboard, collapse main sidebar
	// so the secondary sidebar can extend
	// ═══════════════════════════════════════════════════════════════════════════

	// Routes that should auto-collapse the main sidebar and show secondary nav
	const membershipRoutes: Record<string, { title: string; items: Array<{ href: string; icon: string; text: string; submenu?: Array<{ href: string; icon: string; text: string }> }> }> = {
		'/dashboard/day-trading-room': {
			title: 'Day Trading Room',
			items: [
				{ href: '/dashboard/day-trading-room', icon: 'layout-dashboard', text: 'Day Trading Room Dashboard' },
				{ href: '/dashboard/day-trading-room/daily-videos', icon: 'video', text: 'Premium Daily Videos' },
				{ href: '/dashboard/day-trading-room/learning-center', icon: 'school', text: 'Learning Center' },
				{ href: '/dashboard/day-trading-room/trading-room-archive', icon: 'archive', text: 'Trading Room Archives' },
				{
					href: '#',
					icon: 'users',
					text: 'Meet the Traders',
					submenu: [
						{ href: '/dashboard/day-trading-room/meet-the-traders/billy-ribeiro', icon: '', text: 'Billy Ribeiro' },
						{ href: '/dashboard/day-trading-room/meet-the-traders/freddie-ferber', icon: '', text: 'Freddie Ferber' },
						{ href: '/dashboard/day-trading-room/meet-the-traders/shao-wan', icon: '', text: 'Shao Wan' }
					]
				},
				{
					href: '#',
					icon: 'shopping-cart',
					text: 'Trader Store',
					submenu: [
						{ href: '/dashboard/day-trading-room/meet-the-traders/billy-ribeiro/trader-store', icon: '', text: 'Billy Ribeiro' },
						{ href: '/dashboard/day-trading-room/meet-the-traders/freddie-ferber/trader-store', icon: '', text: 'Freddie Ferber' },
						{ href: '/dashboard/day-trading-room/meet-the-traders/shao-wan/trader-store', icon: '', text: 'Shao Wan' }
					]
				}
			]
		},
		'/dashboard/swing-trading-room': {
			title: 'Swing Trading Room',
			items: [
				{ href: '/dashboard/swing-trading-room', icon: 'layout-dashboard', text: 'Swing Trading Dashboard' },
				{ href: '/dashboard/swing-trading-room/daily-videos', icon: 'video', text: 'Premium Daily Videos' },
				{ href: '/dashboard/swing-trading-room/learning-center', icon: 'school', text: 'Learning Center' }
			]
		},
		'/dashboard/small-accounts-room': {
			title: 'Small Accounts Room',
			items: [
				{ href: '/dashboard/small-accounts-room', icon: 'layout-dashboard', text: 'Small Accounts Dashboard' },
				{ href: '/dashboard/small-accounts-room/daily-videos', icon: 'video', text: 'Premium Daily Videos' },
				{ href: '/dashboard/small-accounts-room/learning-center', icon: 'school', text: 'Learning Center' }
			]
		},
		'/dashboard/spx-profit-pulse': {
			title: 'SPX Profit Pulse',
			items: [
				{ href: '/dashboard/spx-profit-pulse', icon: 'layout-dashboard', text: 'SPX Profit Pulse Dashboard' },
				{ href: '/dashboard/spx-profit-pulse/alerts', icon: 'bolt', text: 'Alerts' }
			]
		},
		'/dashboard/explosive-swings': {
			title: 'Explosive Swings',
			items: [
				{ href: '/dashboard/explosive-swings', icon: 'layout-dashboard', text: 'Explosive Swings Dashboard' },
				{ href: '/dashboard/explosive-swings/alerts', icon: 'bolt', text: 'Alerts' }
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

	// Derived: Get current membership route data (if on a membership page)
	let currentMembershipData = $derived.by(() => {
		const currentPath = page?.url?.pathname ?? '';
		for (const [route, data] of Object.entries(membershipRoutes)) {
			if (currentPath.startsWith(route)) {
				return data;
			}
		}
		return null;
	});

	// Derived: Check if on membership route (secondary sidebar visible)
	let isOnMembershipRoute = $derived(currentMembershipData !== null);

	$effect(() => {
		if (isOnMembershipRoute && !sidebarCollapsed) {
			sidebarCollapsed = true;
		} else if (!isOnMembershipRoute && sidebarCollapsed) {
			sidebarCollapsed = false;
		}
	});
</script>

<svelte:head>
	<meta name="robots" content="noindex, nofollow" />
	<title>Member Dashboard | Revolution Trading Pros</title>
</svelte:head>

<!-- Breadcrumbs Navigation - Full Width -->
<DashboardBreadcrumbs />

<!-- Dashboard Content - Flex layout matching WordPress exactly -->
<div class="dashboard">
	<!-- Sidebar Navigation (LEFT) -->
	<DashboardSidebar
		user={userData}
		bind:collapsed={sidebarCollapsed}
		secondaryNavItems={currentMembershipData?.items ?? []}
		secondarySidebarTitle={currentMembershipData?.title ?? ''}
	/>

	<!-- Main Content Area - flex: 1 1 auto fills remaining space -->
	<main class="dashboard__main" class:has-secondary-sidebar={isOnMembershipRoute}>
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
	</main>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * APPLE ICT 11+ RESPONSIVE DESIGN SYSTEM
	 * Modern CSS Best Practices (Nov/Dec 2025)
	 * ═══════════════════════════════════════════════════════════════════════════
	 * 
	 * BREAKPOINT STRATEGY:
	 * - CSS Custom Properties for maintainability
	 * - Range syntax (width >= 1280px) for modern browsers
	 * - Container queries where appropriate
	 * - Logical properties for i18n support
	 * - Fluid typography and spacing
	 * 
	 * DEVICE TARGETS:
	 * - Mobile: 320px - 767px (iPhone SE to iPhone 15 Pro Max)
	 * - Tablet: 768px - 1279px (iPad Mini to iPad Pro 11")
	 * - Desktop: 1280px+ (MacBook Air 13" to Studio Display)
	 * - Large: 1920px+ (iMac 27" to Pro Display XDR)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		/* Breakpoint tokens - Apple Design System aligned */
		--bp-mobile-max: 767px;
		--bp-tablet-min: 768px;
		--bp-tablet-max: 1279px;
		--bp-desktop-min: 1280px;
		--bp-desktop-large: 1440px;
		--bp-desktop-xl: 1920px;

		/* Spacing scale - 8pt grid system */
		--space-xs: 0.25rem;  /* 4px */
		--space-sm: 0.5rem;   /* 8px */
		--space-md: 1rem;     /* 16px */
		--space-lg: 1.5rem;   /* 24px */
		--space-xl: 2rem;     /* 32px */
		--space-2xl: 3rem;    /* 48px */
		--space-3xl: 4rem;    /* 64px */

		/* Layout dimensions */
		--sidebar-width-expanded: 280px;
		--sidebar-width-collapsed: 80px;
		--mobile-footer-height: 50px;

		/* Z-index scale */
		--z-base: 1;
		--z-dropdown: 10;
		--z-sticky: 100;
		--z-overlay: 1000;
		--z-modal: 10000;

		/* Transitions - Apple-style easing */
		--ease-standard: cubic-bezier(0.4, 0, 0.2, 1);
		--ease-decelerate: cubic-bezier(0, 0, 0.2, 1);
		--ease-accelerate: cubic-bezier(0.4, 0, 1, 1);
		--duration-fast: 150ms;
		--duration-normal: 250ms;
		--duration-slow: 350ms;
	}

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
		min-block-size: 100vh;
		min-block-size: 100dvh; /* Dynamic viewport height for mobile */
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
		background-color: #f4f4f4;
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

	/* Tablet: 768px - 1279px (iPad, Surface, small laptops) */
	@media (768px <= width < 1280px) {
		.dashboard__main {
			inline-size: 100%;
			padding-block-end: var(--mobile-footer-height);
		}

		/* No secondary sidebar margin on tablet */
		.dashboard__main.has-secondary-sidebar {
			margin-inline-start: 0;
		}
	}

	/* Desktop: >= 1280px (MacBook Air 13"+, iMac, Studio Display) */
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
