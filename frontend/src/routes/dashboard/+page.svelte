<script lang="ts">
	/**
	 * User Dashboard Page - Simpler Trading Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% match to WordPress Simpler Trading structure:
	 * <div class="dashboard__content">
	 *   <section class="dashboard__content-section">
	 *     <h2 class="section-title">Memberships</h2>
	 *     <div class="membership-cards row">
	 *       <div class="col-sm-6 col-xl-4">
	 *         <article class="membership-card membership-card--options">...</article>
	 *       </div>
	 *     </div>
	 *   </section>
	 * </div>
	 *
	 * Features:
	 * - Svelte 5 runes ($state, $derived, $effect)
	 * - Bootstrap grid system (row, col-sm-6, col-xl-4)
	 * - WordPress section structure
	 * - Skeleton loading states
	 *
	 * @version 4.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { IconUsers, IconRefresh, IconAlertTriangle, IconArrowRight } from '@tabler/icons-svelte';
	import MembershipCard from '$lib/components/dashboard/MembershipCard.svelte';
	import {
		getUserMemberships,
		invalidateMembershipCache,
		type UserMembership,
		type UserMembershipsResponse
	} from '$lib/api/user-memberships';
	import type { DashboardPageData } from './+page';

	// ═══════════════════════════════════════════════════════════════════════════
	// PAGE DATA
	// ═══════════════════════════════════════════════════════════════════════════

	// Get data from SvelteKit load function
	const data = $derived($page.data as DashboardPageData);

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let membershipsData = $state<UserMembershipsResponse | null>(null);
	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const tradingRooms = $derived(membershipsData?.tradingRooms ?? []);
	const alertServices = $derived(membershipsData?.alertServices ?? []);
	const courses = $derived(membershipsData?.courses ?? []);
	const indicators = $derived(membershipsData?.indicators ?? []);
	const allMemberships = $derived(membershipsData?.memberships ?? []);
	const stats = $derived(membershipsData?.stats);

	const hasMemberships = $derived(allMemberships.length > 0);
	const hasExpiringMemberships = $derived(stats?.expiringCount ? stats.expiringCount > 0 : false);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Initialize from load function data or fetch fresh
	$effect(() => {
		if (!browser) return;

		// Handle auth errors from load function
		if (data?.authError) {
			error = 'Session expired. Please log in again.';
			isLoading = false;
			return;
		}

		// Handle token refresh needed
		if (data?.needsRefresh) {
			refreshMemberships();
			return;
		}

		// Use data from load function if available
		if (data?.memberships) {
			membershipsData = data.memberships;
			isLoading = false;
		} else {
			// Fallback to client-side fetch
			loadMemberships();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Load memberships with caching
	 */
	async function loadMemberships(): Promise<void> {
		isLoading = true;
		error = null;

		try {
			membershipsData = await getUserMemberships();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load memberships';
			console.error('[Dashboard] Error loading memberships:', e);
		} finally {
			isLoading = false;
		}
	}

	/**
	 * Refresh memberships (bypass cache)
	 */
	async function refreshMemberships(): Promise<void> {
		isRefreshing = true;
		error = null;

		try {
			invalidateMembershipCache();
			membershipsData = await getUserMemberships({ skipCache: true });
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to refresh memberships';
			console.error('[Dashboard] Error refreshing memberships:', e);
		} finally {
			isRefreshing = false;
			isLoading = false;
		}
	}

	/**
	 * Handle retry button click
	 */
	function handleRetry(): void {
		if (error?.includes('Session expired')) {
			goto('/login?redirect=/dashboard');
		} else {
			refreshMemberships();
		}
	}

	/**
	 * Navigate to membership access page
	 */
	function handleMembershipClick(membership: UserMembership): void {
		if (membership.accessUrl) {
			goto(membership.accessUrl);
		}
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>My Account | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE
     ═══════════════════════════════════════════════════════════════════════════ -->

<!-- WordPress: .dashboard__header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Member Dashboard</h1>
		{#if stats}
			<div class="dashboard__stats">
				<span class="stat-item">
					<strong>{stats.totalActive}</strong> active memberships
				</span>
				{#if hasExpiringMemberships}
					<span class="stat-item stat-warning">
						<IconAlertTriangle size={14} />
						<strong>{stats.expiringCount}</strong> expiring soon
					</span>
				{/if}
			</div>
		{/if}
	</div>
	<div class="dashboard__header-right">
		<button
			class="refresh-btn"
			onclick={refreshMemberships}
			disabled={isRefreshing}
			aria-label="Refresh memberships"
		>
			<IconRefresh size={18} class={isRefreshing ? 'spin' : ''} />
		</button>
		<!-- WordPress: Trading Room Dropdown Button -->
		<div class="trading-room-dropdown">
			<a href="/live-trading-rooms" class="btn btn-orange trading-room-btn">
				Trading Room
				<IconArrowRight size={16} />
			</a>
		</div>
	</div>
</header>

<!-- Dashboard Content - WordPress: .dashboard__content -->
<div class="dashboard__content">
	{#if isLoading}
		<!-- Skeleton Loading State - WordPress Exact Structure -->
		<section class="dashboard__content-section">
			<h2 class="section-title skeleton-text">Trading Rooms</h2>
			<div class="membership-cards row">
				{#each Array(3) as _, i}
					<div class="col-sm-6 col-xl-4">
						<MembershipCard skeleton slug="loading-{i}" name="Loading..." />
					</div>
				{/each}
			</div>
		</section>
		<section class="dashboard__content-section">
			<h2 class="section-title skeleton-text">Live Alerts</h2>
			<div class="membership-cards row">
				{#each Array(2) as _, i}
					<div class="col-sm-6 col-xl-4">
						<MembershipCard skeleton slug="loading-alert-{i}" name="Loading..." />
					</div>
				{/each}
			</div>
		</section>
	{:else if error}
		<!-- Error State -->
		<div class="error-state" role="alert">
			<div class="error-icon">
				<IconAlertTriangle size={48} />
			</div>
			<p class="error-message">{error}</p>
			<button class="btn btn-primary" onclick={handleRetry}>
				{error.includes('Session expired') ? 'Log In' : 'Try Again'}
			</button>
		</div>
	{:else if hasMemberships}
		<!-- WordPress Exact: Memberships organized by section -->

		<!-- Trading Rooms Section -->
		{#if tradingRooms.length > 0}
			<section class="dashboard__content-section">
				<h2 class="section-title">Memberships</h2>
				<div class="membership-cards row">
					{#each tradingRooms as room (room.id)}
						<div class="col-sm-6 col-xl-4">
							<MembershipCard
								id={room.id}
								name={room.name}
								type={room.type}
								slug={room.slug}
								icon={room.icon}
								dashboardUrl="/dashboard/{room.slug}"
								roomUrl={room.accessUrl}
								status={room.status}
								daysUntilExpiry={room.daysUntilExpiry}
								onclick={() => handleMembershipClick(room)}
							/>
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<!-- Alert Services Section -->
		{#if alertServices.length > 0}
			<section class="dashboard__content-section">
				<h2 class="section-title">Alert Services</h2>
				<div class="membership-cards row">
					{#each alertServices as alert (alert.id)}
						<div class="col-sm-6 col-xl-4">
							<MembershipCard
								id={alert.id}
								name={alert.name}
								type="alert"
								slug={alert.slug}
								icon={alert.icon}
								dashboardUrl="/dashboard/{alert.slug}"
								roomUrl={alert.accessUrl}
								roomLabel="Alerts"
								status={alert.status}
								daysUntilExpiry={alert.daysUntilExpiry}
								onclick={() => handleMembershipClick(alert)}
							/>
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<!-- Courses Section -->
		{#if courses.length > 0}
			<section class="dashboard__content-section">
				<h2 class="section-title">Courses</h2>
				<div class="membership-cards row">
					{#each courses as course (course.id)}
						<div class="col-sm-6 col-xl-4">
							<MembershipCard
								id={course.id}
								name={course.name}
								type="course"
								slug={course.slug}
								icon={course.icon}
								dashboardUrl="/dashboard/{course.slug}"
								roomUrl={course.accessUrl}
								roomLabel="Course"
								status={course.status}
								daysUntilExpiry={course.daysUntilExpiry}
								onclick={() => handleMembershipClick(course)}
							/>
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<!-- Indicators Section -->
		{#if indicators.length > 0}
			<section class="dashboard__content-section">
				<h2 class="section-title">My Indicators</h2>
				<div class="membership-cards row">
					{#each indicators as indicator (indicator.id)}
						<div class="col-sm-6 col-xl-4">
							<MembershipCard
								id={indicator.id}
								name={indicator.name}
								type="indicator"
								slug={indicator.slug}
								icon={indicator.icon}
								dashboardUrl="/dashboard/{indicator.slug}"
								roomUrl={indicator.accessUrl}
								roomLabel="Indicator"
								status={indicator.status}
								daysUntilExpiry={indicator.daysUntilExpiry}
								onclick={() => handleMembershipClick(indicator)}
							/>
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<!-- Weekly Watchlist Section (WordPress: special type with no Trading Room link) -->
		<!-- This would be rendered separately if user has weekly watchlist -->
	{:else}
		<!-- Empty State -->
		<div class="empty-state">
			<div class="empty-icon">
				<IconUsers size={48} />
			</div>
			<h2>No Active Memberships</h2>
			<p>You don't have any active memberships yet.</p>
			<a href="/live-trading-rooms" class="btn btn-orange">
				Explore Trading Rooms
				<IconArrowRight size={16} />
			</a>
		</div>
	{/if}
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--primary-color: #0984ae;
		--primary-hover: #076787;
		--warning-color: #f59e0b;
		--error-color: #ef4444;
		--success-color: #10b981;
		--text-color: #333;
		--text-muted: #666;
		--border-color: #dbdbdb;
		--bg-light: #f4f4f4;
		--card-shadow: 0 1px 2px rgb(0 0 0 / 15%);
		--transition: all 0.15s ease-in-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER (WordPress: .dashboard__header)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid var(--border-color);
		max-width: 100%;
		padding: 20px 30px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
	}

	.dashboard__header-left,
	.dashboard__header-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.dashboard__page-title {
		color: var(--text-color);
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	.dashboard__stats {
		display: flex;
		gap: 16px;
		margin-left: 16px;
	}

	.stat-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 14px;
		color: var(--text-muted);
	}

	.stat-item strong {
		color: var(--text-color);
	}

	.stat-warning {
		color: var(--warning-color);
	}

	.stat-warning strong {
		color: var(--warning-color);
	}

	/* Trading Room Dropdown (WordPress Reference) */
	.trading-room-dropdown {
		position: relative;
	}

	.trading-room-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: #f99e31;
		color: #fff;
		font-weight: 700;
		font-size: 14px;
		padding: 12px 20px;
		border-radius: 5px;
		text-decoration: none;
		transition: var(--transition);
		border: none;
		cursor: pointer;
	}

	.trading-room-btn:hover {
		background: #f88b09;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REFRESH BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.refresh-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border: 1px solid var(--border-color);
		border-radius: 8px;
		background: #fff;
		color: var(--text-muted);
		cursor: pointer;
		transition: var(--transition);
	}

	.refresh-btn:hover:not(:disabled) {
		border-color: var(--primary-color);
		color: var(--primary-color);
	}

	.refresh-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	:global(.refresh-btn .spin) {
		animation: spin 1s linear infinite;
	}


	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT SECTIONS - WordPress: .dashboard__content-section
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		margin-bottom: 40px;
	}

	.dashboard__content-section:last-child {
		margin-bottom: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION TITLE - WordPress: .section-title
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		color: var(--text-color);
		font-weight: 700;
		font-size: 24px;
		font-family: 'Open Sans Condensed', sans-serif;
		margin: 0 0 20px 0;
		padding-bottom: 10px;
		border-bottom: 2px solid #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARDS GRID - WordPress: .membership-cards.row (Bootstrap)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-cards {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

	.membership-cards > .col-sm-6 {
		position: relative;
		width: 100%;
		padding-right: 15px;
		padding-left: 15px;
		margin-bottom: 30px;
	}

	/* Bootstrap-style column widths */
	@media (min-width: 576px) {
		.membership-cards > .col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.membership-cards > .col-xl-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SKELETON LOADING
	   ═══════════════════════════════════════════════════════════════════════════ */

	.skeleton-text {
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
		color: transparent !important;
	}

	@keyframes shimmer {
		0% {
			background-position: -200% 0;
		}
		100% {
			background-position: 200% 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ERROR STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		text-align: center;
		padding: 24px;
	}

	.error-icon {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: #fef2f2;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 24px;
		color: var(--error-color);
	}

	.error-message {
		color: var(--text-color);
		font-size: 16px;
		margin-bottom: 24px;
		max-width: 400px;
	}

	.btn-primary {
		background: var(--primary-color);
		color: white;
		border: none;
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: var(--transition);
	}

	.btn-primary:hover {
		background: var(--primary-hover);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		text-align: center;
		color: var(--text-muted);
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: var(--bg-light);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 24px;
		color: var(--primary-color);
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: var(--text-color);
		margin: 0 0 8px;
	}

	.empty-state p {
		margin: 0 0 24px;
		color: var(--text-muted);
	}

	.btn-orange {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: #f99e31;
		border-color: #f99e31;
		color: #fff;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		padding: 12px 24px;
		border-radius: 5px;
		text-decoration: none;
		transition: var(--transition);
	}

	.btn-orange:hover {
		background: #f88b09;
		border-color: #f88b09;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ANIMATIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 576px) {
		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__stats {
			display: none;
		}

		.dashboard__content {
			padding: 16px;
		}

		.section-title {
			font-size: 20px;
		}

		.trading-room-btn {
			padding: 10px 16px;
			font-size: 13px;
		}

		.membership-cards {
			margin-right: -8px;
			margin-left: -8px;
		}

		.membership-cards > .col-sm-6 {
			padding-right: 8px;
			padding-left: 8px;
			margin-bottom: 16px;
		}
	}

	@media screen and (min-width: 768px) {
		.dashboard__content {
			padding: 30px 40px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.skeleton-text,
		:global(.refresh-btn .spin) {
			animation: none;
		}

		.dashboard__header,
		.trading-room-btn,
		.refresh-btn {
			transition: none;
		}
	}
</style>
