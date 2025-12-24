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

	// ICT 11+ Auth Guard: Redirect to login if not authenticated after init completes
	$effect(() => {
		if (browser && !$isInitializing && !$isAuthenticated) {
			goto('/login?redirect=/dashboard');
		}
	});

	// Fetch memberships when auth is ready
	$effect(() => {
		if (browser && !$isInitializing) {
			getUserMemberships().then(data => {
				membershipsData = data;
			});
		}
	});

	// Derived: Show content only when auth is ready
	let authReady = $derived(!$isInitializing);
</script>

<svelte:head>
	<title>Dashboard | Revolution Trading Pros</title>
</svelte:head>

<!-- PAGE WRAPPER -->
<div class="dashboard-page">

	<!-- NAVBAR -->
	<NavBar />

	<!-- BREADCRUMB -->
	<nav class="breadcrumbs">
		<div class="breadcrumbs__container">
			<span><a href="/">Home</a></span>
			<span class="separator">/</span>
			<span><strong>Member Dashboard</strong></span>
		</div>
	</nav>

	<!-- DASHBOARD -->
	<div class="dashboard">

		<!-- SIDEBAR -->
		<aside class="dashboard__sidebar">
			<nav class="dashboard__nav-primary">

				<!-- Profile -->
				<a href="/dashboard/account" class="dashboard__profile">
					<span class="dashboard__profile-photo"></span>
					<span class="dashboard__profile-name">{$user?.name || 'Member'}</span>
				</a>

				<!-- Main Links -->
				<ul class="dashboard__nav-list">
					<li class="is-active">
						<a href="/dashboard">
							<span class="dashboard__nav-icon"><IconHomeFilled size={24} /></span>
							<span>Member Dashboard</span>
						</a>
					</li>
					<li>
						<a href="/dashboard/courses">
							<span class="dashboard__nav-icon"><IconPlayerPlayFilled size={24} /></span>
							<span class="nav-text-bold">My Courses</span>
						</a>
					</li>
					<li>
						<a href="/dashboard/indicators">
							<span class="dashboard__nav-icon"><IconChartCandle size={24} /></span>
							<span class="nav-text-bold">My Indicators</span>
						</a>
					</li>
				</ul>

				<!-- DAY TRADING ROOM ONLY - Other rooms commented out until Day Trading is 100% -->
				{#if membershipsData?.tradingRooms && membershipsData.tradingRooms.length > 0}
					<p class="dashboard__nav-category">trading rooms</p>
					<ul class="dashboard__nav-list">
						{#each membershipsData.tradingRooms.filter(r => r.slug === 'day-trading-room') as room (room.id)}
							<li>
								<a href="/dashboard/{room.slug}">
									<span class="dashboard__nav-icon">
										<DynamicIcon name={room.icon} size={24} />
									</span>
									<span>{room.name}</span>
								</a>
							</li>
						{/each}
					</ul>
				{/if}

				<!-- COMMENTED OUT: Other Trading Rooms - uncomment when Day Trading Room is 100% complete
				{#each membershipsData.tradingRooms.filter(r => r.slug !== 'day-trading-room') as room (room.id)}
					<li>
						<a href="/dashboard/{room.slug}">
							<span class="dashboard__nav-icon">
								<DynamicIcon name={room.icon} size={24} />
							</span>
							<span>{room.name}</span>
						</a>
					</li>
				{/each}
				-->

				<!-- COMMENTED OUT: Alert Services - uncomment when Day Trading Room is 100% complete
				{#if membershipsData?.alertServices && membershipsData.alertServices.length > 0}
					<p class="dashboard__nav-category">alert services</p>
					<ul class="dashboard__nav-list">
						{#each membershipsData.alertServices as alert (alert.id)}
							<li>
								<a href="/dashboard/{alert.slug}/alerts">
									<span class="dashboard__nav-icon">
										<DynamicIcon name={alert.icon} size={24} />
									</span>
									<span>{alert.name}</span>
								</a>
							</li>
						{/each}
					</ul>
				{/if}
				-->

				<!-- Premium Reports (future feature) -->
				<p class="dashboard__nav-category">premium reports</p>
				<ul class="dashboard__nav-list">
					<!-- Future: Premium report items will go here -->
				</ul>

				<!-- Tools -->
				<p class="dashboard__nav-category">tools</p>
				<ul class="dashboard__nav-list">
					<li>
						<a href="/dashboard/support">
							<span class="dashboard__nav-icon"><IconHelpCircle size={24} /></span>
							<span>Support</span>
						</a>
					</li>
				</ul>

				<!-- Account -->
				<p class="dashboard__nav-category">account</p>
				<ul class="dashboard__nav-list">
					<li>
						<a href="/dashboard/account">
							<span class="dashboard__nav-icon"><IconSettings size={24} /></span>
							<span>My Account</span>
						</a>
					</li>
				</ul>

			</nav>
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

	<!-- FOOTER -->
	<Footer />
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE WRAPPER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard-page {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BREADCRUMBS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.breadcrumbs {
		background: #f8f9fa;
		border-bottom: 1px solid #e5e7eb;
		padding: 12px 30px 12px 310px;
		font-size: 13px;
		font-family: 'Open Sans', sans-serif;
	}

	.breadcrumbs__container {
		max-width: 1700px;
	}

	.breadcrumbs a {
		color: #6b7280;
		text-decoration: none;
	}

	.breadcrumbs a:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.breadcrumbs .separator {
		margin: 0 8px;
		color: #9ca3af;
	}

	.breadcrumbs strong {
		color: #333;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTAINER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard {
		display: flex;
		flex: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__sidebar {
		width: 280px;
		flex-shrink: 0;
		background: #0f2d41;
	}

	.dashboard__nav-primary {
		padding-bottom: 30px;
	}

	/* Profile Section */
	.dashboard__profile {
		display: block;
		padding: 32px 20px 28px 80px;
		position: relative;
		text-decoration: none;
		transition: background 0.15s ease-in-out;
	}

	.dashboard__profile:hover {
		background: rgba(255,255,255,0.05);
	}

	.dashboard__profile-photo {
		position: absolute;
		left: 30px;
		top: 50%;
		margin-top: -17px;
		width: 34px;
		height: 34px;
		border: 2px solid #fff;
		border-radius: 50%;
		background: #1a3a4f;
		transition: all 0.15s ease-in-out;
	}

	.dashboard__profile-name {
		display: block;
		color: #fff;
		font-size: 16px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
	}

	/* Navigation List */
	.dashboard__nav-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-list li {
		position: relative;
	}

	.dashboard__nav-list li a {
		display: flex;
		align-items: center;
		min-height: 40px;
		padding: 0 20px 0 80px;
		position: relative;
		color: hsla(0,0%,100%,0.5);
		text-decoration: none;
		font-size: 14px;
		font-weight: 300;
		font-family: 'Open Sans', sans-serif;
		margin-bottom: 10px;
		transition: all 0.15s ease-in-out;
	}

	.dashboard__nav-list li a:hover {
		color: #fff;
	}

	.dashboard__nav-list li a:hover .dashboard__nav-icon {
		color: #fff;
		opacity: 1;
	}

	.dashboard__nav-list li.is-active a {
		color: #fff;
	}

	.dashboard__nav-list li.is-active a::after {
		content: '';
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 5px;
		background: #0984ae;
	}

	/* Bold text for My Classes and My Indicators */
	.nav-text-bold {
		font-weight: bold;
		color: #fff;
	}

	/* Icon Styling */
	.dashboard__nav-icon {
		position: absolute;
		left: 30px;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 24px;
		color: #C5CFD5;
		opacity: 0.6;
		line-height: 24px;
		transition: all 0.15s ease-in-out;
	}

	.dashboard__nav-list li.is-active .dashboard__nav-icon {
		color: #fff;
		opacity: 1;
	}

	/* Category Headers */
	.dashboard__nav-category {
		padding: 30px 30px 10px;
		color: #fff;
		text-transform: uppercase;
		font-size: 10px;
		font-weight: 700;
		letter-spacing: 0.5px;
		margin: 0 0 5px 0;
		font-family: 'Open Sans', sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__main {
		flex: 1;
		background: #f4f4f4;
		min-height: 500px;
		padding: 30px;
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
</style>
