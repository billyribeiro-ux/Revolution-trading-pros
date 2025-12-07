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
	const weeklyWatchlist = $derived(membershipsData?.weeklyWatchlist ?? []);
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

<!-- Dashboard Content - WordPress: .dashboard__content (EXACT SimplerMain Match) -->
<div class="dashboard__content">
	<!-- WordPress: .dashboard__content-main (left column) -->
	<div class="dashboard__content-main">
		{#if isLoading}
			<!-- Skeleton Loading State - WordPress Exact Structure -->
			<section class="dashboard__content-section">
				<h2 class="section-title skeleton-text">Memberships</h2>
				<div class="membership-cards row">
					{#each Array(3) as _, i}
						<div class="col-sm-6 col-xl-4">
							<MembershipCard skeleton slug="loading-{i}" name="Loading..." />
						</div>
					{/each}
				</div>
			</section>
			<section class="dashboard__content-section">
				<h2 class="section-title skeleton-text">Mastery</h2>
				<div class="membership-cards row">
					{#each Array(2) as _, i}
						<div class="col-sm-6 col-xl-4">
							<MembershipCard skeleton slug="loading-mastery-{i}" name="Loading..." />
						</div>
					{/each}
				</div>
			</section>
			<section class="dashboard__content-section">
				<h2 class="section-title skeleton-text">Tools</h2>
				<div class="membership-cards row">
					<div class="col-sm-6 col-xl-4">
						<MembershipCard skeleton slug="loading-tools" name="Loading..." />
					</div>
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
			<!-- WordPress EXACT: Memberships organized by SimplerMain sections -->

			<!-- SECTION: Memberships (Trading Rooms - options type) -->
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

			<!-- SECTION: Mastery (Moxie Mastery, Courses, Indicators) -->
			{#if courses.length > 0 || indicators.length > 0}
				<section class="dashboard__content-section">
					<h2 class="section-title">Mastery</h2>
					<div class="membership-cards row">
						{#each courses as course (course.id)}
							<div class="col-sm-6 col-xl-4">
								<MembershipCard
									id={course.id}
									name={course.name}
									type="moxie"
									slug={course.slug}
									icon={course.icon}
									dashboardUrl="/dashboard/{course.slug}"
									roomUrl={course.accessUrl}
									roomLabel="Trading Room"
									status={course.status}
									daysUntilExpiry={course.daysUntilExpiry}
									onclick={() => handleMembershipClick(course)}
								/>
							</div>
						{/each}
						{#each indicators as indicator (indicator.id)}
							<div class="col-sm-6 col-xl-4">
								<MembershipCard
									id={indicator.id}
									name={indicator.name}
									type="moxie"
									slug={indicator.slug}
									icon={indicator.icon}
									dashboardUrl="/dashboard/{indicator.slug}"
									roomUrl={indicator.accessUrl}
									roomLabel="Trading Room"
									status={indicator.status}
									daysUntilExpiry={indicator.daysUntilExpiry}
									onclick={() => handleMembershipClick(indicator)}
								/>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			<!-- SECTION: Tools (Weekly Watchlist, Alert Services) -->
			{#if weeklyWatchlist.length > 0 || alertServices.length > 0}
				<section class="dashboard__content-section">
					<h2 class="section-title">Tools</h2>
					<div class="membership-cards row">
						{#each weeklyWatchlist as ww (ww.id)}
							<div class="col-sm-6 col-xl-4">
								<MembershipCard
									id={ww.id}
									name={ww.name}
									type="ww"
									slug={ww.slug}
									icon={ww.icon}
									dashboardUrl="/dashboard/{ww.slug}"
									status={ww.status}
									daysUntilExpiry={ww.daysUntilExpiry}
									onclick={() => handleMembershipClick(ww)}
								/>
							</div>
						{/each}
						{#each alertServices as alert (alert.id)}
							<div class="col-sm-6 col-xl-4">
								<MembershipCard
									id={alert.id}
									name={alert.name}
									type="ww"
									slug={alert.slug}
									icon={alert.icon}
									dashboardUrl="/dashboard/{alert.slug}"
									status={alert.status}
									daysUntilExpiry={alert.daysUntilExpiry}
									onclick={() => handleMembershipClick(alert)}
								/>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			<!-- WordPress EXACT: Weekly Watchlist Featured Section -->
			{#if weeklyWatchlist.length > 0}
				<div class="dashboard__content-section u--background-color-white">
					<section>
						<div class="row">
							<div class="col-sm-6 col-lg-5">
								<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
								<!-- Mobile Image -->
								<div class="hidden-lg">
									<a href="/watchlist">
										<img src="/images/weekly-watchlist.jpg" alt="Weekly Watchlist image" class="u--border-radius" />
									</a>
								</div>
								<h4 class="h5 u--font-weight-bold">Weekly Watchlist with Our Experts</h4>
								<div class="u--hide-read-more">
									<p>Get the latest weekly stock picks and market analysis.</p>
								</div>
								<a href="/watchlist" class="btn btn-tiny btn-default">Watch Now</a>
							</div>
							<div class="col-sm-6 col-lg-7 hidden-mobile">
								<a href="/watchlist">
									<img src="/images/weekly-watchlist.jpg" alt="Weekly Watchlist image" class="u--border-radius" />
								</a>
							</div>
						</div>
					</section>
				</div>
			{/if}
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

	<!-- WordPress: .dashboard__content-sidebar (right column) -->
	<aside class="dashboard__content-sidebar">
		<section class="content-sidebar__section">
			<!-- Sidebar content placeholder - matches SimplerMain empty sidebar -->
		</section>
	</aside>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD STYLES - Uses global --st-* variables from app.css
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER (WordPress: .dashboard__header)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid var(--st-border-color);
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
		color: var(--st-text-color);
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
		color: var(--st-text-muted);
	}

	.stat-item strong {
		color: var(--st-text-color);
	}

	.stat-warning {
		color: var(--st-warning);
	}

	.stat-warning strong {
		color: var(--st-warning);
	}

	/* Trading Room Dropdown (WordPress Reference) */
	.trading-room-dropdown {
		position: relative;
	}

	.trading-room-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: var(--st-orange);
		color: #fff;
		font-weight: 700;
		font-size: 14px;
		padding: 12px 20px;
		border-radius: 5px;
		text-decoration: none;
		transition: var(--st-transition);
		border: none;
		cursor: pointer;
	}

	.trading-room-btn:hover {
		background: var(--st-orange-hover);
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
		border: 1px solid var(--st-border-color);
		border-radius: 8px;
		background: #fff;
		color: var(--st-text-muted);
		cursor: pointer;
		transition: var(--st-transition);
	}

	.refresh-btn:hover:not(:disabled) {
		border-color: var(--st-primary);
		color: var(--st-primary);
	}

	.refresh-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	:global(.refresh-btn .spin) {
		animation: spin 1s linear infinite;
	}


	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content (SimplerMain EXACT)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		gap: 30px;
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	/* WordPress: .dashboard__content-main (left column) */
	.dashboard__content-main {
		flex: 1;
		min-width: 0;
	}

	/* WordPress: .dashboard__content-sidebar (right column) */
	.dashboard__content-sidebar {
		width: 280px;
		flex-shrink: 0;
	}

	.content-sidebar__section {
		/* Empty placeholder - matches SimplerMain */
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
		color: var(--st-text-color);
		font-weight: 700;
		font-size: 24px;
		font-family: 'Open Sans Condensed', sans-serif;
		margin: 0 0 20px 0;
		padding-bottom: 10px;
		border-bottom: 2px solid var(--st-primary);
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
		color: var(--st-error);
	}

	.error-message {
		color: var(--st-text-color);
		font-size: 16px;
		margin-bottom: 24px;
		max-width: 400px;
	}

	.btn-primary {
		background: var(--st-primary);
		color: white;
		border: none;
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: var(--st-transition);
	}

	.btn-primary:hover {
		background: var(--st-primary-hover);
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
		color: var(--st-text-muted);
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: var(--st-bg-light);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 24px;
		color: var(--st-primary);
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: var(--st-text-color);
		margin: 0 0 8px;
	}

	.empty-state p {
		margin: 0 0 24px;
		color: var(--st-text-muted);
	}

	.btn-orange {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: var(--st-orange);
		border-color: var(--st-orange);
		color: #fff;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		padding: 12px 24px;
		border-radius: 5px;
		text-decoration: none;
		transition: var(--st-transition);
	}

	.btn-orange:hover {
		background: var(--st-orange-hover);
		border-color: var(--st-orange-hover);
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
	   WEEKLY WATCHLIST FEATURED SECTION - WordPress SimplerMain EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.u--background-color-white {
		background-color: #fff;
		padding: 24px;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.section-title-alt {
		color: var(--st-text-color);
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 24px;
		font-weight: 700;
		margin: 0 0 16px 0;
	}

	.section-title-alt--underline {
		padding-bottom: 10px;
		border-bottom: 2px solid var(--st-primary);
	}

	.h5 {
		font-size: 18px;
		line-height: 1.4;
		margin: 0 0 12px;
	}

	.u--font-weight-bold {
		font-weight: 700;
		color: var(--st-text-color);
	}

	.u--hide-read-more {
		margin-bottom: 16px;
	}

	.u--hide-read-more p {
		font-size: 14px;
		color: var(--st-text-muted);
		line-height: 1.6;
		margin: 0;
	}

	.u--border-radius {
		border-radius: 8px;
		width: 100%;
		height: auto;
		display: block;
	}

	.btn-tiny {
		display: inline-block;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 600;
		border-radius: 4px;
		text-decoration: none;
		transition: var(--st-transition);
	}

	.btn-default {
		background: var(--st-primary);
		color: #fff;
		border: 1px solid var(--st-primary);
	}

	.btn-default:hover {
		background: var(--st-primary-hover);
		border-color: var(--st-primary-hover);
	}

	/* Bootstrap-style row for Weekly Watchlist */
	.row {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

	.col-sm-6,
	.col-lg-5,
	.col-lg-7 {
		position: relative;
		width: 100%;
		padding-right: 15px;
		padding-left: 15px;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-5 {
			flex: 0 0 41.666667%;
			max-width: 41.666667%;
		}

		.col-lg-7 {
			flex: 0 0 58.333333%;
			max-width: 58.333333%;
		}
	}

	/* Responsive visibility classes */
	.hidden-lg {
		display: block;
	}

	.hidden-mobile {
		display: none;
	}

	@media (min-width: 992px) {
		.hidden-lg {
			display: none;
		}

		.hidden-mobile {
			display: block;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Hide sidebar on medium screens and below */
	@media screen and (max-width: 992px) {
		.dashboard__content {
			flex-direction: column;
		}

		.dashboard__content-sidebar {
			width: 100%;
			display: none; /* Hide sidebar on mobile - matches SimplerMain behavior */
		}
	}

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

		/* Weekly Watchlist Featured Section responsive */
		.u--background-color-white {
			padding: 16px;
		}

		.section-title-alt {
			font-size: 20px;
		}

		.h5 {
			font-size: 16px;
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
