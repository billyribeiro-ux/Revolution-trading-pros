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
	import type { Snippet } from 'svelte';
	import { user, isInitializing, isAuthenticated } from '$lib/stores/auth';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';

	// Tabler Icons - exact matches to screenshot
	import IconHomeFilled from '@tabler/icons-svelte/icons/home-filled';
	import IconPlayerPlayFilled from '@tabler/icons-svelte/icons/player-play-filled';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconHelpCircle from '@tabler/icons-svelte/icons/help-circle';
	import IconSettings from '@tabler/icons-svelte/icons/settings';

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
</script>

<svelte:head>
	<title>Dashboard | Revolution Trading Pros</title>
</svelte:head>

<!-- NAVBAR - Full Navigation Menu -->
<NavBar />

<!-- BREADCRUMB NAVIGATION - Exact Match from Jesus File -->
<nav id="breadcrumbs" class="breadcrumbs">
	<div class="container-fluid">
		<ul>
			<li class="item-home">
				<a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a>
			</li>
			<li class="separator separator-home"> / </li>
			<li class="item-current item-401190">
				<strong class="breadcrumb-current breadcrumb-401190"> Member Dashboard</strong>
			</li>
		</ul>
	</div>
</nav>

<!-- PAGE WRAPPER - Matches Jesus structure -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">

		<a id="top"></a>

		<!-- DASHBOARD -->
		<div class="dashboard">

		<!-- SIDEBAR -->
		<aside class="dashboard__sidebar">
			<nav class="dashboard__nav-primary">

				<!-- Profile -->
				<a href="/dashboard/account" class="dashboard__profile-nav-item">
					<span class="dashboard__profile-photo"></span>
					<span class="dashboard__profile-name">{$user?.name || 'Member'}</span>
				</a>

				<!-- Main Links -->
				<ul>
					<li></li>
					<ul class="dash_main_links">
						<li class="is-active">
							<a href="/dashboard">
								<span class="dashboard__nav-item-icon st-icon-home"></span>
								<span class="dashboard__nav-item-text">Member Dashboard</span>
							</a>
						</li>
						<li>
							<a href="/dashboard/courses">
								<span class="dashboard__nav-item-icon st-icon-courses"></span>
								<span class="dashboard__nav-item-text" style="font-weight:bold;color: white;">My Classes</span>
							</a>
						</li>
						<li>
							<a href="/dashboard/indicators">
								<span class="dashboard__nav-item-icon st-icon-indicators"></span>
								<span class="dashboard__nav-item-text" style="font-weight:bold;color: white;">My Indicators</span>
							</a>
						</li>
					</ul>
				</ul>

				<!-- MEMBERSHIPS SECTION - Trading Rooms -->
				<ul>
					<li>
						<p class="dashboard__nav-category">memberships</p>
					</li>
					<ul class="dash_main_links">
						{#if membershipsData?.tradingRooms && membershipsData.tradingRooms.length > 0}
							{#each membershipsData.tradingRooms as room (room.id)}
								<li>
									<a href="/dashboard/{room.slug}">
										<span class="dashboard__nav-item-icon st-icon-{room.slug}"></span>
										<span class="dashboard__nav-item-text">{room.name}</span>
									</a>
								</li>
							{/each}
						{/if}
					</ul>
				</ul>

				<!-- MASTERY SECTION - Courses -->
				<ul>
					<li>
						<p class="dashboard__nav-category">mastery</p>
					</li>
					<ul class="dash_main_links">
						{#if membershipsData?.courses && membershipsData.courses.length > 0}
							{#each membershipsData.courses as course (course.id)}
								<li>
									<a href="/dashboard/{course.slug}">
										<span class="dashboard__nav-item-icon st-icon-{course.slug}"></span>
										<span class="dashboard__nav-item-text">{course.name}</span>
									</a>
								</li>
							{/each}
						{/if}
					</ul>
				</ul>

				<!-- PREMIUM REPORTS SECTION -->
				<ul>
					<li>
						<p class="dashboard__nav-category">premium reports</p>
					</li>
					<ul class="dash_main_links">
						{#if membershipsData?.premiumReports && membershipsData.premiumReports.length > 0}
							{#each membershipsData.premiumReports as report (report.id)}
								<li>
									<a href="/dashboard/{report.slug}">
										<span class="dashboard__nav-item-icon">
											<DynamicIcon name={report.icon} size={24} />
										</span>
										<span class="dashboard__nav-item-text">{report.name}</span>
									</a>
								</li>
							{/each}
						{/if}
					</ul>
				</ul>

				<!-- TOOLS SECTION -->
				<ul>
					<li>
						<p class="dashboard__nav-category">tools</p>
					</li>
					<ul class="dash_main_links">
						<li>
							<a href="/dashboard/ww">
								<span class="dashboard__nav-item-icon st-icon-trade-of-the-week"></span>
								<span class="dashboard__nav-item-text">Weekly Watchlist</span>
							</a>
						</li>
						<li>
							<a href="https://intercom.help/simpler-trading/en/" target="_blank">
								<span class="dashboard__nav-item-icon st-icon-support"></span>
								<span class="dashboard__nav-item-text">Support</span>
							</a>
						</li>
					</ul>
				</ul>

				<!-- ACCOUNT SECTION -->
				<ul>
					<li>
						<p class="dashboard__nav-category">account</p>
					</li>
					<ul class="dash_main_links">
						<li>
							<a href="/dashboard/account">
								<span class="dashboard__nav-item-icon st-icon-settings"></span>
								<span class="dashboard__nav-item-text">My Account</span>
							</a>
						</li>
					</ul>
				</ul>

			</nav>
		</aside>

		<!-- MOBILE OVERLAY -->
		<div 
			class="dashboard__overlay" 
			onclick={closeMobileMenu}
			onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && closeMobileMenu()}
			role="button"
			tabindex="-1"
			aria-label="Close menu"
		></div>

		<!-- MOBILE TOGGLE BUTTON -->
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
	   BREADCRUMBS - Exact Jesus File Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.breadcrumbs {
		z-index: 1;
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
	   DASHBOARD CONTAINER - Flexbox Layout
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard {
		display: flex;
		flex-flow: row nowrap;
		flex: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - Exact Simpler Trading Styling
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__sidebar {
		display: flex;
		flex: 0 0 auto;
		flex-flow: row nowrap;
		width: 280px;
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
			width: 280px;
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
		width: 280px;
		padding-bottom: 30px;
		font-size: 16px;
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
		padding: 0 20px 0 80px;
		position: relative;
		color: hsla(0, 0%, 100%, 0.5);
		text-decoration: none;
		font-size: 14px;
		font-weight: 300;
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

	/* Icon Styling - Exact Match */
	.dashboard__nav-item-icon {
		position: absolute;
		top: 50%;
		left: 30px;
		margin-top: -16px;
		width: 32px;
		height: 32px;
		font-size: 32px;
		line-height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #C5CFD5;
		opacity: 0.6;
		transition: all 0.15s ease-in-out;
	}

	.dash_main_links li a:hover .dashboard__nav-item-icon,
	.dash_main_links li.is-active .dashboard__nav-item-icon {
		color: #fff;
		opacity: 1;
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
	   MAIN CONTENT - Exact Simpler Trading Styling
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__main {
		flex: 1 1 auto;
		min-width: 0;
		background-color: #f4f4f4;
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
</style>
