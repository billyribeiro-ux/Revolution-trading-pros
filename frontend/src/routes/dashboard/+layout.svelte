<script lang="ts">
	/**
	 * Dashboard Layout - ICT 11+ Auth-Guarded Layout
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Principal Engineer Pattern:
	 * - Layout-level auth guard ensures auth is initialized before rendering children
	 * - Child pages can safely assume auth token is available
	 * - Single responsibility: auth check happens once, not in every page
	 * - Sidebar shows user's actual memberships (data-driven)
	 *
	 * @version 2.1.0
	 */
	import { onMount } from 'svelte';
	import { NavBar } from '$lib/components/nav';
	import Footer from '$lib/components/sections/Footer.svelte';
	
	// Import pixel-perfect global styles from reference file 1
	import '$lib/styles/dashboard-globals.css';
	import '$lib/styles/st-icons.css';
	import type { Snippet } from 'svelte';
	import { user, isInitializing, isAuthenticated } from '$lib/stores/auth';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { page } from '$app/stores';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';
	import { getUserAvatarUrl } from '$lib/utils/gravatar';

	// Tabler Icons - Sidebar Navigation Icons
	import IconHome from '@tabler/icons-svelte/icons/home';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconCalendarWeek from '@tabler/icons-svelte/icons/calendar-week';
	import IconHeadset from '@tabler/icons-svelte/icons/headset';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconStar from '@tabler/icons-svelte/icons/star';
	import IconTrophy from '@tabler/icons-svelte/icons/trophy';
	import IconSchool from '@tabler/icons-svelte/icons/school';
	import IconReportAnalytics from '@tabler/icons-svelte/icons/report-analytics';
	import IconLayoutDashboard from '@tabler/icons-svelte/icons/layout-dashboard';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconBooks from '@tabler/icons-svelte/icons/books';
	import IconArchive from '@tabler/icons-svelte/icons/archive';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconShoppingBag from '@tabler/icons-svelte/icons/shopping-bag';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconAward from '@tabler/icons-svelte/icons/award';
	import IconBuildingStore from '@tabler/icons-svelte/icons/building-store';

	let { children }: { children: Snippet } = $props();

	// Memberships data for sidebar
	let membershipsData = $state<UserMembershipsResponse | null>(null);
	
	// Mobile menu state
	let mobileMenuOpen = $state(false);

	// Toggle mobile menu
	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
		if (browser) {
			document.documentElement.classList.toggle('html--dashboard-menu-open', mobileMenuOpen);
		}
	}

	// Close mobile menu
	function closeMobileMenu() {
		mobileMenuOpen = false;
		if (browser) {
			document.documentElement.classList.remove('html--dashboard-menu-open');
		}
	}

	// ICT 11+ Auth Guard: Redirect to login if not authenticated after init completes
	$effect(() => {
		if (browser && !$isInitializing && !$isAuthenticated) {
			goto('/login?redirect=/dashboard');
		}
	});

	// Fetch memberships when auth is ready
	$effect(() => {
		if (browser && !$isInitializing) {
			console.log('[Layout] 🔄 Auth ready, fetching memberships for sidebar...');
			getUserMemberships().then(data => {
				membershipsData = data;
				console.log('[Layout] ✅ Sidebar memberships loaded:', {
					total: data?.memberships?.length || 0,
					tradingRooms: data?.tradingRooms?.length || 0,
					courses: data?.courses?.length || 0,
					premiumReports: data?.premiumReports?.length || 0,
					weeklyWatchlist: data?.weeklyWatchlist?.length || 0
				});
				
				if (data?.tradingRooms && data.tradingRooms.length > 0) {
					console.log('[Layout] 🎯 Sidebar Trading Rooms:', data.tradingRooms.map(r => r.name));
				} else {
					console.warn('[Layout] ⚠️ No trading rooms in sidebar data');
				}
			}).catch(err => {
				console.error('[Layout] ❌ Failed to load sidebar memberships:', err);
			});
		}
	});

	// Derived: Show content only when auth is ready
	let authReady = $derived(!$isInitializing);

	// Derived: User's avatar URL (Gravatar with fallback)
	let userAvatarUrl = $derived(getUserAvatarUrl($user, { size: 32, default: 'mp' }));

	// Breadcrumb mapping for dashboard sub-pages
	const breadcrumbTitles: Record<string, string> = {
		// Trading Rooms
		'day-trading-room': 'Day Trading Room',
		'swing-trading-room': 'Swing Trading Room',
		'small-account-mentorship': 'Small Account Mentorship',
		'options-day-trading': 'Options Day Trading',
		'simpler-showcase': 'Simpler Showcase',
		// Alert Services
		'spx-profit-pulse': 'SPX Profit Pulse',
		'explosive-swing': 'Explosive Swing',
		// Tools
		'ww': 'Weekly Watchlist',
		'weekly-watchlist': 'Weekly Watchlist',
		// Member Dashboard
		'courses': 'Courses',
		'classes': 'My Classes',
		'indicators': 'My Indicators',
		'alerts': 'Alerts',
		'settings': 'Settings',
		'account': 'Account',
		'profile': 'Profile',
		// Sub-pages
		'learning-center': 'Learning Center',
		'start-here': 'Start Here',
		'resources': 'Resources',
		'daily-videos': 'Premium Daily Videos',
		'trading-room-archive': 'Trading Room Archives',
		'traders': 'Meet the Traders',
		'trader-store': 'Trader Store'
	};

	// Derived: Generate breadcrumbs from current URL path
	let breadcrumbs = $derived.by(() => {
		const pathname = $page.url.pathname;
		const segments = pathname.split('/').filter(Boolean);

		const crumbs: Array<{ label: string; href: string; isCurrent: boolean }> = [
			{ label: 'Home', href: '/', isCurrent: false }
		];

		// Build path progressively
		let currentPath = '';
		segments.forEach((segment, index) => {
			currentPath += `/${segment}`;
			const isLast = index === segments.length - 1;

			let label = segment;
			if (segment === 'dashboard') {
				label = 'Member Dashboard';
			} else if (breadcrumbTitles[segment]) {
				label = breadcrumbTitles[segment];
			} else {
				// Convert slug to title case
				label = segment.split('-').map(word =>
					word.charAt(0).toUpperCase() + word.slice(1)
				).join(' ');
			}

			crumbs.push({
				label,
				href: currentPath,
				isCurrent: isLast
			});
		});

		return crumbs;
	});

	// Known trading room/membership slugs that have secondary navigation
	// Trading Rooms: Day Trading Room, Swing Trading Room, Small Account Mentorship
	// Alert Services: SPX Profit Pulse, Explosive Swings
	const membershipSlugs = [
		// Trading Rooms
		'day-trading-room',
		'swing-trading-room',
		'small-account-mentorship',
		// Alert Services
		'spx-profit-pulse',
		'explosive-swing',
		// Legacy
		'options-day-trading',
		'simpler-showcase'
	];

	// Derived: Get current membership slug from URL path
	let currentMembershipSlug = $derived.by(() => {
		const pathname = $page.url.pathname;
		// Check if we're on a dynamic [slug] route
		if ($page.params.slug) {
			return $page.params.slug;
		}
		// Check if we're on a static trading room route
		const segments = pathname.replace('/dashboard/', '').split('/');
		const firstSegment = segments[0];
		if (membershipSlugs.includes(firstSegment)) {
			return firstSegment;
		}
		return null;
	});

	// Derived: Check if we're on a membership sub-page (triggers collapsed sidebar)
	let isOnMembershipPage = $derived(!!currentMembershipSlug);

	// Derived: Check if we're on an account page (triggers account secondary nav)
	let isOnAccountPage = $derived($page.url.pathname.startsWith('/dashboard/account'));

	// Derived: Check if sidebar should show secondary nav (membership OR account pages)
	let hasSecondaryNav = $derived(isOnMembershipPage || isOnAccountPage);

	// Account navigation links - WooCommerce style (exact match to reference)
	// Slugs match our actual SvelteKit routes in /dashboard/account/
	const accountNavLinks = [
		{ slug: 'orders', label: 'My Orders', class: 'woocommerce-MyAccount-navigation-link--orders' },
		{ slug: 'subscriptions', label: 'My Subscriptions', class: 'woocommerce-MyAccount-navigation-link--subscriptions' },
		{ slug: 'coupons', label: 'Coupons', class: 'woocommerce-MyAccount-navigation-link--wc-smart-coupons' },
		{ slug: 'billing-address', label: 'Billing Address', class: 'woocommerce-MyAccount-navigation-link--edit-address' },
		{ slug: 'payment-methods', label: 'Payment Methods', class: 'woocommerce-MyAccount-navigation-link--payment-methods' },
		{ slug: 'edit-account', label: 'Account Details', class: 'woocommerce-MyAccount-navigation-link--edit-account' },
		{ slug: 'customer-logout', label: 'Log out', class: 'woocommerce-MyAccount-navigation-link--customer-logout' }
	];
</script>

<svelte:head>
	<title>Dashboard | Revolution Trading Pros</title>
</svelte:head>

<!-- NAVBAR - Full Navigation Menu -->
<NavBar />

<!-- BREADCRUMB NAVIGATION - Dynamic based on current page -->
<nav id="breadcrumbs" class="breadcrumbs">
	<div class="container-fluid">
		<ul>
			{#each breadcrumbs as crumb, index (crumb.href)}
				{#if index > 0}
					<li class="separator"> / </li>
				{/if}
				<li class="item-{crumb.label.toLowerCase().replace(/\s+/g, '-')}">
					{#if crumb.isCurrent}
						<strong class="breadcrumb-current"> {crumb.label}</strong>
					{:else}
						<a class="breadcrumb-link" href={crumb.href} title={crumb.label}>{crumb.label}</a>
					{/if}
				</li>
			{/each}
		</ul>
	</div>
</nav>

<!-- PAGE WRAPPER - Matches Jesus structure -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">

		<a id="top" aria-label="Skip to top"></a>

		<!-- DASHBOARD -->
		<div class="dashboard">

		<!-- SIDEBAR -->
		<aside class="dashboard__sidebar" class:has-secondary={hasSecondaryNav}>
			<nav class="dashboard__nav-primary" class:is-collapsed={hasSecondaryNav}>

				<!-- Profile - Avatar loads from Gravatar using user's email -->
				<a href="/dashboard/account/" class="dashboard__profile-nav-item">
					<span
						class="dashboard__profile-photo"
						style="background-image: url({userAvatarUrl});"
					></span>
					<span class="dashboard__profile-name">{$user?.name || 'Member'}</span>
				</a>

				<!-- Main Links -->
				<ul class="dash_main_links">
					<li class={$page.url.pathname === '/dashboard' || $page.url.pathname === '/dashboard/' ? 'is-active' : ''}>
						<a href="/dashboard/">
							<span class="dashboard__nav-item-icon">
								<IconHome size={28} />
							</span>
							<span class="dashboard__nav-item-text">Member Dashboard</span>
						</a>
					</li>
					<li class={$page.url.pathname.startsWith('/dashboard/classes') ? 'is-active' : ''}>
						<a href="/dashboard/classes/">
							<span class="dashboard__nav-item-icon">
								<IconSchool size={28} />
							</span>
							<span class="dashboard__nav-item-text dashboard__nav-item-text--bold">My Classes</span>
						</a>
					</li>
					<li class={$page.url.pathname.startsWith('/dashboard/indicators') ? 'is-active' : ''}>
						<a href="/dashboard/indicators/">
							<span class="dashboard__nav-item-icon">
								<IconChartCandle size={28} />
							</span>
							<span class="dashboard__nav-item-text dashboard__nav-item-text--bold">My Indicators</span>
						</a>
					</li>
				</ul>

				<!-- MEMBERSHIPS SECTION - Trading Rooms -->
				{#if membershipsData?.tradingRooms && membershipsData.tradingRooms.length > 0}
					<p class="dashboard__nav-category">memberships</p>
					<ul class="dash_main_links">
						{#each membershipsData.tradingRooms as room (room.id)}
							<li class={$page.url.pathname.startsWith(`/dashboard/${room.slug}`) ? 'is-active' : ''}>
								<a href="/dashboard/{room.slug}">
									<span class="dashboard__nav-item-icon">
										<DynamicIcon name={room.icon || 'chart-line'} size={28} />
									</span>
									<span class="dashboard__nav-item-text">{room.name}</span>
								</a>
							</li>
						{/each}
					</ul>
				{/if}

				<!-- ALERT SERVICES SECTION -->
				{#if membershipsData?.alertServices && membershipsData.alertServices.length > 0}
					<p class="dashboard__nav-category">alert services</p>
					<ul class="dash_main_links">
						{#each membershipsData.alertServices as alert (alert.id)}
							<li class={$page.url.pathname.startsWith(`/dashboard/${alert.slug}`) ? 'is-active' : ''}>
								<a href="/dashboard/{alert.slug}">
									<span class="dashboard__nav-item-icon">
										<DynamicIcon name={alert.icon || 'bell'} size={28} />
									</span>
									<span class="dashboard__nav-item-text">{alert.name}</span>
								</a>
							</li>
						{/each}
					</ul>
				{/if}

				<!-- MASTERY SECTION - Courses -->
				{#if membershipsData?.courses && membershipsData.courses.length > 0}
					<p class="dashboard__nav-category">mastery</p>
					<ul class="dash_main_links">
						{#each membershipsData.courses as course (course.id)}
							<li class={$page.url.pathname.startsWith(`/dashboard/${course.slug}`) ? 'is-active' : ''}>
								<a href="/dashboard/{course.slug}">
									<span class="dashboard__nav-item-icon">
										<DynamicIcon name={course.icon || 'book'} size={28} />
									</span>
									<span class="dashboard__nav-item-text">{course.name}</span>
								</a>
							</li>
						{/each}
					</ul>
				{/if}

				<!-- PREMIUM REPORTS SECTION -->
				{#if membershipsData?.premiumReports && membershipsData.premiumReports.length > 0}
					<p class="dashboard__nav-category">premium reports</p>
					<ul class="dash_main_links">
						{#each membershipsData.premiumReports as report (report.id)}
							<li class={$page.url.pathname.startsWith(`/dashboard/${report.slug}`) ? 'is-active' : ''}>
								<a href="/dashboard/{report.slug}">
									<span class="dashboard__nav-item-icon">
										<DynamicIcon name={report.icon || 'report-analytics'} size={28} />
									</span>
									<span class="dashboard__nav-item-text">{report.name}</span>
								</a>
							</li>
						{/each}
					</ul>
				{/if}

				<!-- TOOLS SECTION -->
				<p class="dashboard__nav-category">tools</p>
				<ul class="dash_main_links">
					<li class={$page.url.pathname.startsWith('/dashboard/ww') ? 'is-active' : ''}>
						<a href="/dashboard/ww/">
							<span class="dashboard__nav-item-icon">
								<IconCalendarWeek size={28} />
							</span>
							<span class="dashboard__nav-item-text">Weekly Watchlist</span>
						</a>
					</li>
					<li>
						<a href="https://intercom.help/simpler-trading/en/" target="_blank" rel="noopener noreferrer">
							<span class="dashboard__nav-item-icon">
								<IconHeadset size={28} />
							</span>
							<span class="dashboard__nav-item-text">Support</span>
						</a>
					</li>
				</ul>

				<!-- ACCOUNT SECTION -->
				<p class="dashboard__nav-category">account</p>
				<ul class="dash_main_links">
					<li class={$page.url.pathname.startsWith('/dashboard/account') ? 'is-active' : ''}>
						<a href="/dashboard/account/">
							<span class="dashboard__nav-item-icon">
								<IconSettings size={28} />
							</span>
							<span class="dashboard__nav-item-text">My Account</span>
						</a>
					</li>
				</ul>

			</nav>

<!-- MOBILE TOGGLE - Inside aside, after primary nav (exact match to reference) -->
<footer class="dashboard__toggle is-collapsed" class:is-open={mobileMenuOpen}>
	<button
		type="button"
		class="dashboard__toggle-button"
		onclick={toggleMobileMenu}
		data-toggle-dashboard-menu
	>
		<div class="dashboard__toggle-button-icon">
			<span></span>
			<span></span>
			<span></span>
		</div>
		<span class="framework__toggle-button-label">Dashboard Menu</span>
	</button>
</footer>
<div
	class="dashboard__overlay"
	onclick={closeMobileMenu}
	onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && closeMobileMenu()}
	role="button"
	tabindex="-1"
	aria-label="Close menu"
	data-toggle-dashboard-menu
></div>

			<!-- SECONDARY NAVIGATION - Membership-specific navigation (appears on membership pages only) -->
			{#if currentMembershipSlug}
				<nav class="dashboard__nav-secondary">
					<ul class="dash_main_links">
						<!-- Dashboard with membership name -->
						<li class={$page.url.pathname === `/dashboard/${currentMembershipSlug}` || $page.url.pathname === `/dashboard/${currentMembershipSlug}/` ? 'is-active' : ''}>
							<a href="/dashboard/{currentMembershipSlug}">
								<span class="dashboard__nav-item-icon">
									<IconLayoutDashboard size={28} />
								</span>
								<span class="dashboard__nav-item-text">{breadcrumbTitles[currentMembershipSlug] || 'Dashboard'} Dashboard</span>
							</a>
						</li>
						<!-- Premium Daily Videos -->
						<li class={$page.url.pathname.includes('/daily-videos') ? 'is-active' : ''}>
							<a href="/dashboard/{currentMembershipSlug}/daily-videos">
								<span class="dashboard__nav-item-icon">
									<IconPlayerPlay size={28} />
								</span>
								<span class="dashboard__nav-item-text">Premium Daily Videos</span>
							</a>
						</li>
						<!-- Learning Center -->
						<li class={$page.url.pathname.includes('/learning-center') ? 'is-active' : ''}>
							<a href="/dashboard/{currentMembershipSlug}/learning-center">
								<span class="dashboard__nav-item-icon">
									<IconAward size={28} />
								</span>
								<span class="dashboard__nav-item-text">Learning Center</span>
							</a>
						</li>
						<!-- Trading Room Archives -->
						<li class={$page.url.pathname.includes('/trading-room-archive') ? 'is-active' : ''}>
							<a href="/dashboard/{currentMembershipSlug}/trading-room-archive">
								<span class="dashboard__nav-item-icon">
									<IconArchive size={28} />
								</span>
								<span class="dashboard__nav-item-text">Trading Room Archives</span>
							</a>
						</li>
						<!-- Meet the Traders (with submenu) - Exact WordPress match -->
						<li class="has-submenu" class:is-active={$page.url.pathname.includes('/traders')}>
							<span class="submenu-trigger" role="button" tabindex="0">
								<span class="dashboard__nav-item-icon">
									<IconUsers size={28} />
								</span>
								<span class="dashboard__nav-item-text">Meet the Traders</span>
							</span>
							<ul class="dashboard__nav-submenu">
								<li><a href="/dashboard/{currentMembershipSlug}/john-carter">John Carter</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/henry-gambell">Henry Gambell</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/taylor-horton">Taylor Horton</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/bruce-marshall">Bruce Marshall</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/danielle-shay">Danielle Shay</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/allison-ostrander">Allison Ostrander</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/sam-shames">Sam Shames</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/kody-ashmore">Kody Ashmore</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/raghee-horner">Raghee Horner</a></li>
							</ul>
						</li>
						<!-- Trader Store (with submenu) - Exact WordPress match -->
						<li class="has-submenu" class:is-active={$page.url.pathname.includes('/trader-store')}>
							<span class="submenu-trigger" role="button" tabindex="0">
								<span class="dashboard__nav-item-icon">
									<IconBuildingStore size={28} />
								</span>
								<span class="dashboard__nav-item-text">Trader Store</span>
							</span>
							<ul class="dashboard__nav-submenu">
								<li><a href="/dashboard/{currentMembershipSlug}/john-carter/john-carter-trader-store">John Carter</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/henry-gambell/trader-store">Henry Gambell</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/taylor-horton/taylor-horton-trader-store">Taylor Horton</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/bruce-marshall/bruce-marshall-trader-store">Bruce Marshall</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/danielle-shay/danielle-shay-trader-store">Danielle Shay</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/allison-ostrander/allison-ostrander-trader-store">Allison Ostrander</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/sam-shames/sam-shames-trader-store">Sam Shames</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/kody-ashmore/kody-ashmore-trader-store">Kody Ashmore</a></li>
								<li><a href="/dashboard/{currentMembershipSlug}/raghee-horner/raghee-horner-trader-store">Raghee Horner</a></li>
							</ul>
						</li>
						<!-- Simpler Showcase -->
						<li class={$page.url.pathname.includes('/simpler-showcase') ? 'is-active' : ''}>
							<a href="/dashboard/simpler-showcase">
								<span class="dashboard__nav-item-icon">
									<IconTrophy size={28} />
								</span>
								<span class="dashboard__nav-item-text">Simpler Showcase</span>
							</a>
						</li>
					</ul>
				</nav>
			{/if}

			<!-- ACCOUNT SECONDARY NAVIGATION - WooCommerce style (exact match to reference) -->
			{#if isOnAccountPage}
				<nav class="dashboard__nav-secondary">
					<ul>
						{#each accountNavLinks as link (link.slug)}
							<li class="woocommerce-MyAccount-navigation-link {link.class}" class:is-active={$page.url.pathname.includes(`/account/${link.slug}`)}>
								<a class="no-icon" href="/dashboard/account/{link.slug}">{link.label}</a>
							</li>
						{/each}
					</ul>
				</nav>
			{/if}

		</aside>

		<!-- MAIN CONTENT -->
		<main class="dashboard__main">
			{#if authReady}
				{@render children()}
			{:else}
				<div class="auth-loading">
					<div class="loading-spinner"></div>
					<p>Loading your dashboard...</p>
				</div>
			{/if}
		</main>

		</div>

	</div><!-- #content -->
</div><!-- #page -->

<!-- FOOTER - Full Width Outside Dashboard -->
<Footer />

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   GLOBAL RESETS & BASE STYLES - Exact Match to Simpler Trading
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(body #page.grid-container) {
		max-width: none;
	}

	/* ICT11+ Fix: Reference line 1793 - EXACT font styles from Simpler Trading */
	:global(.h1, h1) {
		font-family: "Open Sans Condensed", sans-serif;
		font-weight: bold;
		font-size: 44px;
		line-height: 1.1em;
	}

	:global(.h2, h2) {
		font-size: 32px;
	}

	@media (max-width: 768px) {
		:global(.h1, h1) {
			font-size: 30px;
		}
		:global(.h2, h2) {
			font-size: 25px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE WRAPPER - Matches Jesus HTML Structure
	   ═══════════════════════════════════════════════════════════════════════════ */
	#page {
		display: block;
	}

	#content {
		display: block;
	}

	.site-content {
		display: block;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTAINER FLUID - Bootstrap-style Container (MISSING FROM YOUR DASHBOARD)
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.container-fluid) {
		max-width: 100%;
		padding-right: 15px;
		padding-left: 15px;
		margin-right: 0;
		margin-left: 0;
		width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BREADCRUMBS - Exact Reference File Match with Container
	   ═══════════════════════════════════════════════════════════════════════════ */
	.breadcrumbs {
		z-index: 1;
		background-color: #f5f5f5;
		border-bottom: 1px solid #e5e5e5;
	}

	.breadcrumbs .container-fluid {
		padding-top: 10px;
		padding-bottom: 10px;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		font-size: 13px;
		line-height: 1.4;
	}

	.breadcrumbs li {
		display: inline;
		margin: 0;
	}

	.breadcrumbs a {
		color: #666;
		text-decoration: none;
		transition: color 0.2s ease-in-out;
	}

	.breadcrumbs a:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.breadcrumbs .separator {
		margin: 0 8px;
		color: #999;
	}

	.breadcrumbs strong {
		color: #333;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE & SITE CONTENT WRAPPERS - FORCE LIGHT BACKGROUNDS
	   Matches reference file 1 exactly - no dark containers
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(html),
	:global(body) {
		background-color: #efefef !important;
	}

	:global(#page) {
		max-width: 100%;
		background-color: #efefef !important;
	}

	:global(.site-content) {
		max-width: 100%;
		margin: 0;
		background-color: #efefef !important;
	}

	#page {
		background-color: #efefef !important;
		margin-bottom: 0 !important;
		padding-bottom: 0 !important;
	}

	#content {
		background-color: #efefef !important;
		margin-bottom: 0 !important;
		padding-bottom: 0 !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTAINER - Flexbox Layout
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard {
		display: flex;
		flex-flow: row nowrap;
		flex: 1;
		max-width: 100%;
		margin: 0;
		margin-bottom: 0 !important;
		width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - Exact Simpler Trading Styling
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__sidebar {
		display: flex;
		flex: 0 0 auto;
		flex-flow: row nowrap;
		width: 220px;
		background-color: #0e2433;
		min-height: 100%;
		bottom: 50px;
		left: 0;
		opacity: 1;
		overflow-x: hidden;
		overflow-y: auto;
		position: static;
		top: 0;
		transition: all 0.3s ease-in-out;
		visibility: visible;
		z-index: auto;
	}

	@media (max-width: 1279px) {
		.dashboard__sidebar {
			position: fixed;
			opacity: 0;
			visibility: hidden;
			z-index: 100010;
			width: 220px;
		}

		/* Sidebar with secondary nav needs full width (80px + 220px = 300px) on mobile */
		.dashboard__sidebar.has-secondary {
			width: 300px;
		}
	}

	@media (min-width: 1280px) {
		.dashboard__sidebar {
			display: block;
			bottom: auto;
			left: auto;
			opacity: 1;
			overflow: visible;
			position: static;
			top: auto;
			visibility: visible;
			z-index: auto;
		}
	}

	.dashboard__nav-primary {
		width: 220px;
		padding-bottom: 30px;
		font-size: 14px;
		line-height: 1;
		transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1), padding 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		will-change: width, padding;
		contain: layout style;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COLLAPSED STATE - When on membership sub-pages
	   Primary nav collapses to 80px, shows only icons
	   Pixel-perfect match to Simpler Trading reference
	   Uses GPU-accelerated transitions for smooth layout shift prevention
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__nav-primary.is-collapsed {
		width: 80px;
		padding: 30px 0 30px 0;
		/* Prevent layout shift with stable flex-basis */
		flex: 0 0 80px;
		min-width: 80px;
		max-width: 80px;
	}

	/* Collapsed profile - centered photo, 40px, tooltip for name */
	.dashboard__nav-primary.is-collapsed .dashboard__profile-nav-item {
		padding: 20px 0;
		text-align: center;
		height: auto;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		position: relative;
		display: block;
	}

	.dashboard__nav-primary.is-collapsed .dashboard__profile-photo {
		position: relative;
		left: auto;
		top: auto;
		transform: none;
		margin: 0 auto;
		width: 40px;
		height: 40px;
		border: 2px solid hsla(0, 0%, 100%, 0.2);
	}

	/* Note: .dashboard__profile-name tooltip styles applied in combined selector below */

	.dashboard__nav-primary.is-collapsed .dashboard__nav-category {
		display: none;
	}

	.dashboard__nav-primary.is-collapsed .dash_main_links li a {
		padding: 14px 0;
		justify-content: center;
		position: relative;
	}

	.dashboard__nav-primary.is-collapsed .dash_main_links li a::after {
		display: none;
	}

	.dashboard__nav-primary.is-collapsed .dashboard__nav-item-icon {
		margin-right: 0;
		transform: scale(1);
		transition: all 0.2s ease-in-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COLLAPSED TOOLTIP - Shows label bubble on hover
	   Pixel-perfect match to Simpler Trading reference
	   Reference: .dashboard__nav-primary.is-collapsed .dash_main_links .dashboard__nav-item-text { color: #0984ae !important }
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__nav-primary.is-collapsed .dashboard__nav-item-text,
	.dashboard__nav-primary.is-collapsed .dashboard__profile-name {
		display: block !important;
		z-index: 100;
		position: absolute;
		left: 100%;
		top: 50%;
		transform: translate(-10px, -50%);
		margin-left: 15px;
		padding: 8px 12px;
		opacity: 0;
		visibility: hidden;
		background: #fff;
		color: #0984ae !important; /* Reference: blue text in tooltip */
		border-radius: 4px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		white-space: nowrap;
		font-size: 13px;
		font-weight: 600;
		pointer-events: none;
		transition: opacity 0.2s ease-in-out, visibility 0.2s ease-in-out, transform 0.2s ease-in-out;
		will-change: opacity, visibility, transform;
	}

	/* Tooltip arrow */
	.dashboard__nav-primary.is-collapsed .dashboard__nav-item-text::before,
	.dashboard__nav-primary.is-collapsed .dashboard__profile-name::before {
		content: '';
		position: absolute;
		left: -6px;
		top: 50%;
		transform: translateY(-50%);
		border-width: 6px;
		border-style: solid;
		border-color: transparent #fff transparent transparent;
	}

	/* Show tooltip on hover */
	.dashboard__nav-primary.is-collapsed .dash_main_links li a:hover .dashboard__nav-item-text,
	.dashboard__nav-primary.is-collapsed .dashboard__profile-nav-item:hover .dashboard__profile-name {
		opacity: 1;
		visibility: visible;
		transform: translate(0, -50%);
	}

	/* Background circle effect on link hover - Pixel-perfect match to WordPress reference */
	.dashboard__nav-primary.is-collapsed a::before {
		position: absolute;
		display: block;
		content: '';
		top: 50%;
		left: 50%;
		width: 50px;
		height: 50px;
		margin-top: -25px;
		margin-left: -25px;
		border-radius: 50%;
		transform: scale(.9);
		background: transparent;
		transition: all .15s ease-in-out;
	}

	.dashboard__nav-primary.is-collapsed a:hover::before {
		transform: scale(1);
		background-color: rgba(0, 0, 0, .2);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR WITH SECONDARY NAV - Stable flex container
	   Prevents layout shift during navigation transitions
	   Total width: 80px (collapsed primary) + 220px (secondary) = 300px
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__sidebar.has-secondary {
		display: flex;
		flex-flow: row nowrap;
		width: 300px; /* Fixed width prevents layout shift */
		min-width: 300px;
		max-width: 300px;
		contain: layout style;
	}

	.dashboard__sidebar.has-secondary .dashboard__nav-primary {
		flex: 0 0 80px;
		width: 80px;
		min-width: 80px;
		max-width: 80px;
		border-right: 1px solid rgba(255, 255, 255, 0.08);
	}

	.dashboard__sidebar.has-secondary .dashboard__nav-secondary {
		flex: 0 0 220px;
		width: 220px;
		min-width: 220px;
		max-width: 220px;
	}

	.dashboard__nav-primary ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-primary li {
		position: relative;
	}

	.dashboard__nav-primary a {
		position: relative;
		display: block;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PROFILE SECTION - ICT 11 CANONICAL from dashboard-globals.css:883-923
	   Photo: 44px absolute positioned left, Name: 16px/700
	   Padding: 32px top, 20px right, 28px bottom, 80px left (for photo space)
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__profile-nav-item {
		display: block;
		height: auto;
		line-height: 1.4;
		padding: 32px 20px 28px 80px;
		position: relative;
		text-decoration: none;
		color: hsla(0, 0%, 100%, 0.5);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		transition: all 0.15s ease-in-out;
	}

	.dashboard__profile-nav-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.dashboard__profile-photo {
		position: absolute;
		left: 20px;
		top: 50%;
		transform: translateY(-50%);
		width: 44px;
		height: 44px;
		border-radius: 50%;
		border: 2px solid hsla(0, 0%, 100%, 0.2);
		background-color: #0f2d41;
		background-size: cover;
		background-position: center;
		transition: border-color 0.15s ease-in-out;
	}

	.dashboard__profile-nav-item:hover .dashboard__profile-photo {
		border-color: #0984ae;
	}

	.dashboard__profile-name {
		display: block;
		font-size: 16px;
		font-weight: 700;
		color: #fff;
	}

	/* Navigation Links - Exact Match to dashboard-globals.css reference */
	.dash_main_links {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dash_main_links li {
		position: relative;
	}

	.dashboard__nav-primary .dash_main_links li a {
		display: flex;
		align-items: center;
		padding: 12px 20px;
		position: relative;
		color: hsla(0, 0%, 100%, 0.7);
		text-decoration: none;
		font-size: 17px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		transition: all 0.2s ease-in-out;
	}

	/* Active indicator on RIGHT side for primary nav */
	.dashboard__nav-primary .dash_main_links li a::after {
		position: absolute;
		display: block;
		content: "";
		top: 0;
		right: 0;
		bottom: 0;
		width: 5px;
		background: transparent;
		transition: all 0.2s ease-in-out;
	}

	.dashboard__nav-primary .dash_main_links li a:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.dashboard__nav-primary .dash_main_links li.is-active a {
		background-color: rgba(255, 255, 255, 0.1);
		color: #fff;
	}

	.dashboard__nav-primary .dash_main_links li.is-active a::after {
		background-color: #0984ae;
	}

	/* Icon Styling - WHITE icons per core reference (not blue) */
	.dashboard__nav-item-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		margin-right: 10px;
		color: #fff;
		vertical-align: middle;
		flex-shrink: 0;
	}

	/* Ensure SVG icons inherit color */
	.dashboard__nav-item-icon :global(svg) {
		width: 28px;
		height: 28px;
		color: inherit;
		stroke: currentColor;
	}

	/* Text Styling - Exact Match to core reference (17px not 14px) */
	.dashboard__nav-item-text {
		font-size: 17px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		color: hsla(0, 0%, 100%, 0.7);
		line-height: 1.4;
	}

	/* Bold white text for My Classes and My Indicators */
	.dashboard__nav-item-text--bold {
		font-size: 17px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #fff;
	}

	/* ICT11+ Fix: Reference lines 2744, 2750 - Bold white text in collapsed tooltip
	   Override the blue #0984ae color for My Classes and My Indicators items */
	.dashboard__nav-primary.is-collapsed .dashboard__nav-item-text--bold {
		color: #fff !important;
		font-weight: 700 !important;
	}

	/* Category Headers - Exact Match to dashboard-globals.css reference */
	.dashboard__nav-category {
		font-size: 11px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: hsla(0, 0%, 100%, 0.3);
		letter-spacing: 0.5px;
		line-height: 1;
		margin: 0;
		padding: 20px 20px 12px;
		text-transform: uppercase;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY NAVIGATION - Membership-specific navigation in LEFT sidebar
	   Pixel-perfect match to Simpler Trading reference
	   Primary nav (collapsed): #0F2D41 | Secondary nav: #143E59
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__nav-secondary {
		background-color: #143E59;
		padding: 20px 0 0 0;
		margin: 0;
		min-height: 100%;
	}

	.dashboard__nav-secondary ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-secondary li {
		position: relative;
	}

	.dashboard__nav-secondary li a {
		display: flex;
		align-items: center;
		padding: 14px 20px 14px 20px;
		color: hsla(0, 0%, 100%, 0.7);
		text-decoration: none;
		font-size: 17px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		transition: all 0.2s ease-in-out;
		position: relative;
	}

	/* Active indicator on LEFT side for secondary nav */
	.dashboard__nav-secondary li a::before {
		position: absolute;
		display: block;
		content: "";
		top: 0;
		left: 0;
		bottom: 0;
		width: 5px;
		background: transparent;
		transition: all 0.2s ease-in-out;
	}

	.dashboard__nav-secondary li a:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.dashboard__nav-secondary li.is-active a {
		background-color: rgba(255, 255, 255, 0.1);
		color: #fff;
	}

	.dashboard__nav-secondary li.is-active a::before {
		background-color: #0984ae;
	}

	.dashboard__nav-secondary .dashboard__nav-item-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		margin-right: 10px;
		color: #fff;
		flex-shrink: 0;
	}

	.dashboard__nav-secondary .dashboard__nav-item-icon :global(svg) {
		width: 28px;
		height: 28px;
		color: inherit;
		stroke: currentColor;
	}

	.dashboard__nav-secondary .dashboard__nav-item-text {
		color: inherit;
		flex: 1;
		font-size: 17px;
		font-weight: 400;
	}

	/* Arrow indicator for items with submenus */
	.dashboard__nav-secondary .nav-arrow {
		margin-left: auto;
		color: hsla(0, 0%, 100%, 0.4);
		display: inline-flex;
		align-items: center;
	}

	.dashboard__nav-secondary .nav-arrow :global(svg) {
		width: 16px;
		height: 16px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBMENU - Hover-based dropdown for Traders and Trader Store
	   Pixel-perfect match to WordPress reference (core 1:2980-3068)
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__nav-secondary li.has-submenu {
		position: relative;
	}

	/* Submenu trigger - styled like nav links but non-navigating (Svelte 5 a11y compliant) */
	.dashboard__nav-secondary .submenu-trigger {
		display: flex;
		align-items: center;
		padding: 14px 20px 14px 20px;
		color: hsla(0, 0%, 100%, 0.7);
		text-decoration: none;
		font-size: 17px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		transition: all 0.2s ease-in-out;
		cursor: default;
		position: relative;
	}

	.dashboard__nav-secondary .submenu-trigger:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.dashboard__nav-secondary li.has-submenu.is-active .submenu-trigger {
		background-color: rgba(255, 255, 255, 0.1);
		color: #fff;
	}

	/* Submenu container - hidden by default, appears on hover */
	.dashboard__nav-secondary .dashboard__nav-submenu {
		position: absolute;
		left: 100%;
		top: 0;
		min-width: 220px;
		background-color: #143E59;
		list-style: none;
		margin: 0;
		padding: 0;
		opacity: 0;
		visibility: hidden;
		transform: translateX(-10px);
		transition: all 0.2s ease-in-out;
		z-index: 110;
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
		border-radius: 0 4px 4px 0;
	}

	/* Show submenu on hover */
	.dashboard__nav-secondary li.has-submenu:hover > .dashboard__nav-submenu {
		opacity: 1;
		visibility: visible;
		transform: translateX(0);
	}

	/* Submenu items */
	.dashboard__nav-secondary .dashboard__nav-submenu li {
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-secondary .dashboard__nav-submenu li a {
		display: block;
		padding: 12px 20px;
		color: hsla(0, 0%, 100%, 0.7);
		text-decoration: none;
		font-size: 13px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		transition: all 0.2s ease-in-out;
		white-space: nowrap;
		border-left: 3px solid transparent;
	}

	.dashboard__nav-secondary .dashboard__nav-submenu li a:hover {
		background-color: rgba(255, 255, 255, 0.1);
		color: #fff;
		border-left-color: #0984ae;
	}

	/* Active state for submenu items */
	.dashboard__nav-secondary .dashboard__nav-submenu li.is-active a {
		background-color: rgba(255, 255, 255, 0.1);
		color: #fff;
		border-left-color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   WOOCOMMERCE ACCOUNT NAVIGATION - Exact match to reference
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__nav-secondary .woocommerce-MyAccount-navigation-link {
		list-style: none;
	}

	.dashboard__nav-secondary .woocommerce-MyAccount-navigation-link a {
		display: block;
		padding: 14px 20px;
		color: hsla(0, 0%, 100%, 0.7);
		text-decoration: none;
		font-size: 17px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		transition: all 0.2s ease-in-out;
		position: relative;
	}

	.dashboard__nav-secondary .woocommerce-MyAccount-navigation-link a.no-icon {
		padding-left: 20px;
	}

	.dashboard__nav-secondary .woocommerce-MyAccount-navigation-link a::before {
		position: absolute;
		display: block;
		content: "";
		top: 0;
		left: 0;
		bottom: 0;
		width: 5px;
		background: transparent;
		transition: all 0.2s ease-in-out;
	}

	.dashboard__nav-secondary .woocommerce-MyAccount-navigation-link a:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.dashboard__nav-secondary .woocommerce-MyAccount-navigation-link.is-active a {
		background-color: rgba(255, 255, 255, 0.1);
		color: #fff;
	}

	.dashboard__nav-secondary .woocommerce-MyAccount-navigation-link.is-active a::before {
		background-color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN CONTENT - Exact Simpler Trading Styling
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__main {
		flex: 1 1 auto;
		min-width: 0;
		background-color: #fff;
		display: flex;
		flex-direction: column;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   AUTH LOADING STATE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.auth-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		color: #666;
	}

	.auth-loading p {
		margin-top: 16px;
		font-size: 14px;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1279px) {
		.dashboard {
			flex-direction: column;
		}
		
		.dashboard__sidebar {
			width: 100%;
		}

		:global(body) {
			padding-bottom: 50px;
		}
	}

	@media (max-width: 768px) {
		.dashboard__sidebar {
			width: 100%;
			min-height: auto;
		}
	}

	@media (max-width: 641px) {
		.dashboard {
			min-height: auto;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE TOGGLE BUTTON & OVERLAY - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__toggle {
		background-color: #0d2532;
		bottom: 0;
		height: 50px;
		left: 0;
		line-height: 50px;
		padding: 0;
		position: fixed;
		right: 0;
		z-index: 100010;
	}

	@media (min-width: 1280px) {
		.dashboard__toggle {
			display: none;
		}
	}

	.dashboard__toggle-button {
		appearance: none;
		background: none;
		border: none;
		color: #fff;
		height: 50px;
		overflow: hidden;
		padding: 0 20px 0 60px;
		position: relative;
		width: 100%;
		text-align: left;
		cursor: pointer;
	}

	.dashboard__toggle-button:active,
	.dashboard__toggle-button:focus,
	.dashboard__toggle-button:hover {
		background: none;
		outline: none;
	}

	.dashboard__toggle-button-icon {
		height: 50px;
		left: 20px;
		position: absolute;
		top: 50%;
		margin-top: -7px;
		width: 50px;
	}

	.dashboard__toggle-button-icon span {
		background-color: #fff;
		border-radius: 0;
		display: block;
		height: 2px;
		left: 0;
		opacity: 1;
		position: absolute;
		transform: rotate(0);
		transform-origin: left center;
		transition: all 0.2s ease-in-out;
		width: 20px;
	}

	.dashboard__toggle-button-icon span:first-child {
		top: 0;
	}

	.dashboard__toggle-button-icon span:nth-child(2) {
		top: 6px;
	}

	.dashboard__toggle-button-icon span:nth-child(3) {
		top: 12px;
	}

	:global(.html--dashboard-menu-open) .dashboard__toggle-button-icon span:first-child {
		left: 3px;
		top: -1px;
		transform: rotate(45deg);
	}

	:global(.html--dashboard-menu-open) .dashboard__toggle-button-icon span:nth-child(2) {
		opacity: 0;
		width: 0;
	}

	:global(.html--dashboard-menu-open) .dashboard__toggle-button-icon span:nth-child(3) {
		left: 3px;
		top: 13px;
		transform: rotate(-45deg);
	}

	.framework__toggle-button-label {
		font-size: 12px;
		position: relative;
		text-transform: uppercase;
		top: -2px;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__overlay {
		background-color: rgba(0, 0, 0, 0.65);
		bottom: 0;
		left: 0;
		opacity: 0;
		position: fixed;
		right: 0;
		top: 0;
		transition: all 0.3s ease-in-out;
		visibility: hidden;
		z-index: 100009;
	}

	:global(.html--dashboard-menu-open) .dashboard__overlay {
		opacity: 1;
		visibility: visible;
	}

	:global(.html--dashboard-menu-open) .dashboard__sidebar {
		opacity: 1;
		visibility: visible;
	}

	:global(.html--dashboard-menu-open) {
		overflow: hidden;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   GLOBAL ICON UTILITY CLASSES
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.icon) {
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	:global(.icon--lg) {
		width: 32px;
		height: 32px;
	}

	:global(.icon--md) {
		width: 24px;
		height: 24px;
	}
</style>
