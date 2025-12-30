<script lang="ts">
	/**
	 * Dashboard Layout - Svelte 5 Component-Based Architecture
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Auth-guarded layout with dynamic sidebar navigation.
	 * CSS relies on dashboard-globals.css for styling.
	 *
	 * @version 3.1.0 - Svelte 5 with proper runes patterns (Dec 2025)
	 */
	import { onMount, untrack } from 'svelte';
	import { NavBar } from '$lib/components/nav';
	import Footer from '$lib/components/sections/Footer.svelte';

	// Global dashboard styles
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

	// Import Svelte 5 Dashboard Components
	import { DashboardBreadcrumbs, MobileToggle } from '$lib/components/dashboard';

	// Tabler Icons
	import IconHome from '@tabler/icons-svelte/icons/home';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconCalendarWeek from '@tabler/icons-svelte/icons/calendar-week';
	import IconHeadset from '@tabler/icons-svelte/icons/headset';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconTrophy from '@tabler/icons-svelte/icons/trophy';
	import IconSchool from '@tabler/icons-svelte/icons/school';
	import IconLayoutDashboard from '@tabler/icons-svelte/icons/layout-dashboard';
	import IconArchive from '@tabler/icons-svelte/icons/archive';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconAward from '@tabler/icons-svelte/icons/award';
	import IconBuildingStore from '@tabler/icons-svelte/icons/building-store';

	// Props - Svelte 5 $props() pattern
	let { children }: { children: Snippet } = $props();

	// State - Svelte 5 $state() pattern
	let membershipsData = $state<UserMembershipsResponse | null>(null);
	let mobileMenuOpen = $state(false);
	let hasFetchedMemberships = $state(false);
	let openSubmenu = $state<string | null>(null);

	// Submenu toggle handler - for mobile click support
	function toggleSubmenu(menuId: string): void {
		openSubmenu = openSubmenu === menuId ? null : menuId;
	}

	// Mobile menu handler - callback prop pattern for child components
	function toggleMobileMenu(): void {
		mobileMenuOpen = !mobileMenuOpen;
		if (browser) {
			document.documentElement.classList.toggle('html--dashboard-menu-open', mobileMenuOpen);
		}
	}

	// Auth guard - $effect for reactive side effects that should re-run on auth changes
	$effect(() => {
		// Read reactive values synchronously (these will be tracked)
		const initializing = $isInitializing;
		const authenticated = $isAuthenticated;

		if (browser && !initializing && !authenticated) {
			goto('/login?redirect=/dashboard');
		}
	});

	// Fetch memberships - runs once when auth is ready
	// Using $effect with a fetch guard to prevent multiple API calls
	$effect(() => {
		// Read reactive values synchronously for tracking
		const initializing = $isInitializing;
		const authenticated = $isAuthenticated;
		const alreadyFetched = untrack(() => hasFetchedMemberships);

		if (browser && !initializing && authenticated && !alreadyFetched) {
			// Mark as fetched immediately to prevent duplicate calls
			hasFetchedMemberships = true;

			getUserMemberships()
				.then((data) => {
					membershipsData = data;
				})
				.catch((err) => {
					console.error('[Layout] Failed to load sidebar memberships:', err);
					// Reset flag on error so user can retry
					hasFetchedMemberships = false;
				});
		}
	});

	// Cleanup mobile menu state on unmount
	$effect(() => {
		return () => {
			if (browser) {
				document.documentElement.classList.remove('html--dashboard-menu-open');
			}
		};
	});

	// Derived states
	let authReady = $derived(!$isInitializing);
	let userAvatarUrl = $derived(getUserAvatarUrl($user, { size: 32, default: 'mp' }));

	// Breadcrumb titles
	const breadcrumbTitles: Record<string, string> = {
		'day-trading-room': 'Day Trading Room',
		'swing-trading-room': 'Swing Trading Room',
		'small-account-mentorship': 'Small Account Mentorship',
		'options-day-trading': 'Options Day Trading',
		'simpler-showcase': 'Simpler Showcase',
		'spx-profit-pulse': 'SPX Profit Pulse',
		'explosive-swing': 'Explosive Swing',
		'ww': 'Weekly Watchlist',
		'weekly-watchlist': 'Weekly Watchlist',
		'courses': 'Courses',
		'classes': 'My Classes',
		'indicators': 'My Indicators',
		'alerts': 'Alerts',
		'settings': 'Settings',
		'account': 'Account',
		'profile': 'Profile',
		'learning-center': 'Learning Center',
		'start-here': 'Start Here',
		'resources': 'Resources',
		'daily-videos': 'Premium Daily Videos',
		'trading-room-archive': 'Trading Room Archives',
		'traders': 'Meet the Traders',
		'trader-store': 'Trader Store'
	};

	// Membership slugs with secondary nav
	const membershipSlugs = [
		'day-trading-room',
		'swing-trading-room',
		'small-account-mentorship',
		'spx-profit-pulse',
		'explosive-swing',
		'options-day-trading',
		'simpler-showcase'
	];

	// Current membership slug from URL
	let currentMembershipSlug = $derived.by(() => {
		if ($page.params.slug) return $page.params.slug;
		const segments = $page.url.pathname.replace('/dashboard/', '').split('/');
		const firstSegment = segments[0];
		if (membershipSlugs.includes(firstSegment)) return firstSegment;
		return null;
	});

	let isOnMembershipPage = $derived(!!currentMembershipSlug);
	let isOnAccountPage = $derived($page.url.pathname.startsWith('/dashboard/account'));
	let hasSecondaryNav = $derived(isOnMembershipPage || isOnAccountPage);

	// Account navigation
	const accountNavLinks = [
		{ slug: 'orders', label: 'My Orders' },
		{ slug: 'subscriptions', label: 'My Subscriptions' },
		{ slug: 'coupons', label: 'Coupons' },
		{ slug: 'billing-address', label: 'Billing Address' },
		{ slug: 'payment-methods', label: 'Payment Methods' },
		{ slug: 'edit-account', label: 'Account Details' },
		{ slug: 'customer-logout', label: 'Log out' }
	];
</script>

<svelte:head>
	<title>Dashboard | Revolution Trading Pros</title>
</svelte:head>

<NavBar />

<!-- BREADCRUMBS -->
<DashboardBreadcrumbs titleMap={breadcrumbTitles} />

<!-- PAGE WRAPPER -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<a id="top" aria-label="Skip to top"></a>

		<div class="dashboard">
			<!-- SIDEBAR -->
			<aside class="dashboard__sidebar" class:has-secondary={hasSecondaryNav}>
				<nav class="dashboard__nav-primary" class:is-collapsed={hasSecondaryNav}>

					<!-- Profile -->
					<a href="/dashboard/account/" class="dashboard__profile-nav-item">
						<span class="dashboard__profile-photo" style="background-image: url({userAvatarUrl});"></span>
						<span class="dashboard__profile-name">{$user?.name || 'Member'}</span>
					</a>

					<!-- Main Links -->
					<ul class="dash_main_links">
						<li class={$page.url.pathname === '/dashboard' || $page.url.pathname === '/dashboard/' ? 'is-active' : ''}>
							<a href="/dashboard/">
								<span class="dashboard__nav-item-icon"><IconHome size={28} /></span>
								<span class="dashboard__nav-item-text">Member Dashboard</span>
							</a>
						</li>
						<li class={$page.url.pathname.startsWith('/dashboard/classes') ? 'is-active' : ''}>
							<a href="/dashboard/classes/">
								<span class="dashboard__nav-item-icon"><IconSchool size={28} /></span>
								<span class="dashboard__nav-item-text dashboard__nav-item-text--bold">My Classes</span>
							</a>
						</li>
						<li class={$page.url.pathname.startsWith('/dashboard/indicators') ? 'is-active' : ''}>
							<a href="/dashboard/indicators/">
								<span class="dashboard__nav-item-icon"><IconChartCandle size={28} /></span>
								<span class="dashboard__nav-item-text dashboard__nav-item-text--bold">My Indicators</span>
							</a>
						</li>
					</ul>

					<!-- Trading Rooms -->
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

					<!-- Alert Services -->
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

					<!-- Courses -->
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

					<!-- Premium Reports -->
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

					<!-- Tools -->
					<p class="dashboard__nav-category">tools</p>
					<ul class="dash_main_links">
						<li class={$page.url.pathname.startsWith('/dashboard/ww') ? 'is-active' : ''}>
							<a href="/dashboard/ww/">
								<span class="dashboard__nav-item-icon"><IconCalendarWeek size={28} /></span>
								<span class="dashboard__nav-item-text">Weekly Watchlist</span>
							</a>
						</li>
						<li>
							<a href="https://intercom.help/simpler-trading/en/" target="_blank" rel="noopener noreferrer">
								<span class="dashboard__nav-item-icon"><IconHeadset size={28} /></span>
								<span class="dashboard__nav-item-text">Support</span>
							</a>
						</li>
					</ul>

					<!-- Account -->
					<p class="dashboard__nav-category">account</p>
					<ul class="dash_main_links">
						<li class={$page.url.pathname.startsWith('/dashboard/account') ? 'is-active' : ''}>
							<a href="/dashboard/account/">
								<span class="dashboard__nav-item-icon"><IconSettings size={28} /></span>
								<span class="dashboard__nav-item-text">My Account</span>
							</a>
						</li>
					</ul>
				</nav>

				<!-- Mobile Toggle -->
				<MobileToggle isOpen={mobileMenuOpen} onToggle={toggleMobileMenu} />

				<!-- Secondary Nav - Membership Pages -->
				{#if currentMembershipSlug}
					<nav class="dashboard__nav-secondary">
						<ul class="dash_main_links">
							<li class={$page.url.pathname === `/dashboard/${currentMembershipSlug}` || $page.url.pathname === `/dashboard/${currentMembershipSlug}/` ? 'is-active' : ''}>
								<a href="/dashboard/{currentMembershipSlug}">
									<span class="dashboard__nav-item-icon"><IconLayoutDashboard size={28} /></span>
									<span class="dashboard__nav-item-text">{breadcrumbTitles[currentMembershipSlug] || 'Dashboard'} Dashboard</span>
								</a>
							</li>
							<li class={$page.url.pathname.includes('/daily-videos') ? 'is-active' : ''}>
								<a href="/dashboard/{currentMembershipSlug}/daily-videos">
									<span class="dashboard__nav-item-icon"><IconPlayerPlay size={28} /></span>
									<span class="dashboard__nav-item-text">Premium Daily Videos</span>
								</a>
							</li>
							<li class={$page.url.pathname.includes('/learning-center') ? 'is-active' : ''}>
								<a href="/dashboard/{currentMembershipSlug}/learning-center">
									<span class="dashboard__nav-item-icon"><IconAward size={28} /></span>
									<span class="dashboard__nav-item-text">Learning Center</span>
								</a>
							</li>
							<li class={$page.url.pathname.includes('/trading-room-archive') ? 'is-active' : ''}>
								<a href="/dashboard/{currentMembershipSlug}/trading-room-archive">
									<span class="dashboard__nav-item-icon"><IconArchive size={28} /></span>
									<span class="dashboard__nav-item-text">Trading Room Archives</span>
								</a>
							</li>
							<li class="has-submenu" class:is-active={$page.url.pathname.includes('/traders')} class:is-open={openSubmenu === 'traders'}>
								<span
									class="submenu-trigger"
									role="button"
									tabindex="0"
									onclick={() => toggleSubmenu('traders')}
									onkeydown={(e) => e.key === 'Enter' && toggleSubmenu('traders')}
								>
									<span class="dashboard__nav-item-icon"><IconUsers size={28} /></span>
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
							<li class="has-submenu" class:is-active={$page.url.pathname.includes('/trader-store')} class:is-open={openSubmenu === 'trader-store'}>
								<span
									class="submenu-trigger"
									role="button"
									tabindex="0"
									onclick={() => toggleSubmenu('trader-store')}
									onkeydown={(e) => e.key === 'Enter' && toggleSubmenu('trader-store')}
								>
									<span class="dashboard__nav-item-icon"><IconBuildingStore size={28} /></span>
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
							<li class={$page.url.pathname.includes('/simpler-showcase') ? 'is-active' : ''}>
								<a href="/dashboard/simpler-showcase">
									<span class="dashboard__nav-item-icon"><IconTrophy size={28} /></span>
									<span class="dashboard__nav-item-text">Simpler Showcase</span>
								</a>
							</li>
						</ul>
					</nav>
				{/if}

				<!-- Secondary Nav - Account Pages -->
				{#if isOnAccountPage}
					<nav class="dashboard__nav-secondary">
						<ul>
							{#each accountNavLinks as link (link.slug)}
								<li class="woocommerce-MyAccount-navigation-link" class:is-active={$page.url.pathname.includes(`/account/${link.slug}`)}>
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
	</div>
</div>

<Footer />

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   LAYOUT-SPECIFIC STYLES
	   All sidebar styles come from dashboard-globals.css
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Page wrapper */
	#page {
		background-color: #efefef;
		margin-bottom: 0;
		padding-bottom: 0;
	}

	#content {
		background-color: #efefef;
		margin-bottom: 0;
		padding-bottom: 0;
	}

	/* Dashboard container */
	.dashboard {
		display: flex;
		flex-flow: row nowrap;
		flex: 1;
		max-width: 100%;
		margin: 0;
		width: 100%;
	}

	/* Main content */
	.dashboard__main {
		flex: 1 1 auto;
		min-width: 0;
		background-color: #fff;
		display: flex;
		flex-direction: column;
	}

	/* Auth loading state */
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

	/* Responsive */
	@media (max-width: 1279px) {
		.dashboard {
			flex-direction: column;
		}

		:global(body) {
			padding-bottom: 50px;
		}
	}
</style>
