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

	// Breadcrumb mapping for dashboard sub-pages
	const breadcrumbTitles: Record<string, string> = {
		'mastering-the-trade': 'Mastering the Trade',
		'simpler-showcase': 'Simpler Showcase',
		'day-trading-room': 'Day Trading Room',
		'ww': 'Weekly Watchlist',
		'weekly-watchlist': 'Weekly Watchlist',
		'courses': 'Courses',
		'indicators': 'Indicators',
		'alerts': 'Alerts',
		'settings': 'Settings',
		'account': 'Account',
		'profile': 'Profile'
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
		<aside class="dashboard__sidebar">
			<nav class="dashboard__nav-primary">

				<!-- Profile -->
				<a href="/dashboard/account/" class="dashboard__profile-nav-item">
					<span
						class="dashboard__profile-photo"
						style={$user?.avatar ? `background-image: url(${$user.avatar});` : ''}
					></span>
					<span class="dashboard__profile-name">{$user?.name || 'Member'}</span>
				</a>

				<!-- Main Links -->
				<ul class="dash_main_links">
					<li class={$page.url.pathname === '/dashboard' || $page.url.pathname === '/dashboard/' ? 'is-active' : ''}>
						<a href="/dashboard/">
							<span class="dashboard__nav-item-icon">
								<IconHome size={24} />
							</span>
							<span class="dashboard__nav-item-text">Member Dashboard</span>
						</a>
					</li>
					<li class={$page.url.pathname.startsWith('/dashboard/classes') ? 'is-active' : ''}>
						<a href="/dashboard/classes/">
							<span class="dashboard__nav-item-icon">
								<IconSchool size={24} />
							</span>
							<span class="dashboard__nav-item-text">My Classes</span>
						</a>
					</li>
					<li class={$page.url.pathname.startsWith('/dashboard/indicators') ? 'is-active' : ''}>
						<a href="/dashboard/indicators/">
							<span class="dashboard__nav-item-icon">
								<IconChartCandle size={24} />
							</span>
							<span class="dashboard__nav-item-text">My Indicators</span>
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
										<DynamicIcon name={room.icon || 'chart-line'} size={24} />
									</span>
									<span class="dashboard__nav-item-text">{room.name}</span>
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
										<DynamicIcon name={course.icon || 'book'} size={24} />
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
										<DynamicIcon name={report.icon || 'report-analytics'} size={24} />
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
								<IconCalendarWeek size={24} />
							</span>
							<span class="dashboard__nav-item-text">Weekly Watchlist</span>
						</a>
					</li>
					<li>
						<a href="https://intercom.help/simpler-trading/en/" target="_blank" rel="noopener noreferrer">
							<span class="dashboard__nav-item-icon">
								<IconHeadset size={24} />
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
								<IconSettings size={24} />
							</span>
							<span class="dashboard__nav-item-text">My Account</span>
						</a>
					</li>
				</ul>

			</nav>

			<!-- SECONDARY NAVIGATION - Membership-specific navigation (appears on membership pages only) -->
			{#if $page.params.slug}
				<nav class="dashboard__nav-secondary">
					<ul class="dash_main_links">
						<li>
							<a href="/dashboard/{$page.params.slug}">
								<span class="dashboard__nav-item-icon">
									<IconLayoutDashboard size={24} />
								</span>
								<span class="dashboard__nav-item-text">Dashboard</span>
							</a>
						</li>
						<li>
							<a href="/dashboard/{$page.params.slug}/daily-videos">
								<span class="dashboard__nav-item-icon">
									<IconVideo size={24} />
								</span>
								<span class="dashboard__nav-item-text">Premium Daily Videos</span>
							</a>
						</li>
						<li>
							<a href="/dashboard/{$page.params.slug}/learning-center">
								<span class="dashboard__nav-item-icon">
									<IconBooks size={24} />
								</span>
								<span class="dashboard__nav-item-text">Learning Center</span>
							</a>
						</li>
						<li>
							<a href="/dashboard/{$page.params.slug}/trading-room-archive">
								<span class="dashboard__nav-item-icon">
									<IconArchive size={24} />
								</span>
								<span class="dashboard__nav-item-text">Trading Room Archives</span>
							</a>
						</li>
						<li>
							<a href="/dashboard/{$page.params.slug}/traders">
								<span class="dashboard__nav-item-icon">
									<IconUsers size={24} />
								</span>
								<span class="dashboard__nav-item-text">Meet the Traders</span>
							</a>
						</li>
						<li>
							<a href="/dashboard/{$page.params.slug}/trader-store">
								<span class="dashboard__nav-item-icon">
									<IconShoppingBag size={24} />
								</span>
								<span class="dashboard__nav-item-text">Trader Store</span>
							</a>
						</li>
					</ul>
				</nav>
			{/if}

			<!-- MOBILE TOGGLE BUTTON - INSIDE sidebar per WordPress structure -->
			<footer class="dashboard__toggle">
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

			<!-- MOBILE OVERLAY - INSIDE sidebar per WordPress structure -->
			<div
				class="dashboard__overlay"
				onclick={closeMobileMenu}
				onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && closeMobileMenu()}
				role="button"
				tabindex="-1"
				aria-label="Close menu"
				data-toggle-dashboard-menu
			></div>
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

	:global(.h1, h1) {
		font-size: 36px;
	}

	:global(.h2, h2) {
		font-size: 30px;
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
		transition: color 0.15s ease-in-out;
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
	}

	#content {
		background-color: #efefef !important;
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
		background-color: #0f2d41;
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

	.dashboard__nav-primary > ul + ul {
		margin-top: 20px;
	}

	/* Profile Section - Exact Match */
	.dashboard__profile-nav-item {
		display: block;
		height: auto;
		line-height: 1.4;
		padding: 32px 20px 28px 80px;
		position: relative;
		text-decoration: none;
		color: hsla(0, 0%, 100%, 0.5);
		transition: all 0.15s ease-in-out;
	}

	.dashboard__profile-nav-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.dashboard__profile-nav-item:hover .dashboard__profile-photo {
		border-color: #0984ae;
	}

	.dashboard__profile-nav-item::after {
		position: absolute;
		display: block;
		content: "";
		top: 0;
		right: 0;
		bottom: 0;
		width: 5px;
		background: transparent;
		transform: scale(1);
		transition: all 0.15s ease-in-out;
		transform-origin: 100% 50%;
	}

	.dashboard__profile-photo {
		position: absolute;
		top: 50%;
		left: 30px;
		margin-top: -17px;
		width: 34px;
		height: 34px;
		border: 2px solid #fff;
		border-radius: 50%;
		background: #1a3a4f no-repeat center;
		background-size: 32px;
		transition: all 0.15s ease-in-out;
	}

	.dashboard__profile-name {
		display: block;
		color: #fff;
		font-size: 16px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
	}

	/* Navigation Links - Exact Match */
	.dash_main_links {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dash_main_links li {
		position: relative;
	}

	.dash_main_links li a {
		display: flex;
		align-items: center;
		height: 50px;
		padding: 0 20px 0 60px;
		position: relative;
		color: hsla(0, 0%, 100%, 0.7);
		text-decoration: none;
		font-size: 14px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.15s ease-in-out;
	}

	.dash_main_links li a::after {
		position: absolute;
		display: block;
		content: "";
		top: 0;
		right: 0;
		bottom: 0;
		width: 5px;
		background: transparent;
		transform: scale(1);
		transition: all 0.15s ease-in-out;
		transform-origin: 100% 50%;
	}

	.dash_main_links li a:hover {
		color: #fff;
	}

	.dash_main_links li.is-active a {
		color: #fff;
	}

	.dash_main_links li.is-active a::after {
		background-color: #0984ae;
	}

	/* Icon Styling - Exact Match with Tabler SVG icons */
	.dashboard__nav-item-icon {
		position: absolute;
		top: 50%;
		left: 20px;
		transform: translateY(-50%);
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #0984ae;
		transition: all 0.15s ease-in-out;
	}

	/* Ensure SVG icons inherit color */
	.dashboard__nav-item-icon :global(svg) {
		width: 24px;
		height: 24px;
		color: inherit;
		stroke: currentColor;
	}

	.dash_main_links li a:hover .dashboard__nav-item-icon,
	.dash_main_links li.is-active .dashboard__nav-item-icon {
		color: #0984ae;
	}

	/* Text Styling */
	.dashboard__nav-item-text {
		color: inherit;
	}

	/* Category Headers - Exact Match */
	.dashboard__nav-category {
		font-weight: 700;
		padding: 30px 30px 0;
		color: #fff;
		text-transform: uppercase;
		font-size: 10px;
		letter-spacing: 0.5px;
		margin: 0 0 5px 0;
		font-family: 'Open Sans', sans-serif;
		line-height: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY NAVIGATION - Membership-specific navigation in LEFT sidebar
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__nav-secondary {
		background-color: #0a2335;
		padding: 0;
		margin: 0;
	}

	.dashboard__nav-secondary ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-secondary li {
		position: relative;
	}

	.dashboard__nav-secondary a {
		display: flex;
		align-items: center;
		padding: 15px 30px;
		color: #c5cfd5;
		text-decoration: none;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.15s ease-in-out;
		position: relative;
	}

	.dashboard__nav-secondary a:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.dashboard__nav-secondary .dashboard__nav-item-icon {
		position: static;
		margin-right: 12px;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #0984ae;
		transition: all 0.15s ease-in-out;
	}

	.dashboard__nav-secondary a:hover .dashboard__nav-item-icon {
		color: #0984ae;
	}

	.dashboard__nav-secondary .dashboard__nav-item-text {
		color: inherit;
	}

	.dashboard__nav-secondary .dashboard__nav-submenu {
		background-color: #081a27;
		list-style: none;
		margin: 0;
		padding: 0;
		z-index: 110;
	}

	.dashboard__nav-secondary .dashboard__nav-submenu li {
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.dashboard__nav-secondary .dashboard__nav-submenu a {
		padding: 12px 30px 12px 60px;
		font-size: 13px;
	}

	.dashboard__nav-secondary .submenu-toggle {
		display: flex;
		align-items: center;
		padding: 15px 30px;
		color: #c5cfd5;
		background: none;
		border: none;
		width: 100%;
		text-align: left;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.15s ease-in-out;
		cursor: pointer;
	}

	.dashboard__nav-secondary .submenu-toggle:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: #fff;
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
		transition: all 0.15s ease-in-out;
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

	@media (max-width: 1279px) {
		:global(body) {
			padding-bottom: 50px;
		}
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
