<script lang="ts">
	/**
	 * User Dashboard Page - Svelte 5 / SvelteKit Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Matches WordPress WooCommerce My Account dashboard structure.
	 * Features:
	 * - Svelte 5 runes ($state, $derived, $effect)
	 * - SvelteKit data streaming
	 * - Optimistic UI updates
	 * - Skeleton loading states
	 * - Service-specific icons
	 *
	 * @version 3.0.0 (Svelte 5 / December 2025)
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

<!-- Dashboard Header -->
<div class="wc-content-sction">
	<div class="dashb_headr">
		<div class="dashb_headr-left">
			<h1 class="dashb_pg-titl">My Account</h1>
			{#if stats}
				<div class="dashb_stats">
					<span class="stat-item">
						<strong>{stats.totalActive}</strong> active
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
		<div class="dashb_headr-right">
			<button
				class="refresh-btn"
				onclick={refreshMemberships}
				disabled={isRefreshing}
				aria-label="Refresh memberships"
			>
				<IconRefresh size={18} class={isRefreshing ? 'spin' : ''} />
			</button>
			<a href="/live-trading-rooms" class="btn btn-xs btn-link start-here-btn">
				Start Here
				<IconArrowRight size={16} />
			</a>
		</div>
	</div>
</div>

<!-- Dashboard Content -->
<div class="wc-accontent-inner">
	{#if isLoading}
		<!-- Skeleton Loading State -->
		<div class="cselt-row">
			<h3 class="custom-head-0 skeleton-text">Trading Rooms</h3>
			{#each Array(3) as _, i}
				<div class="col-sm4">
					<MembershipCard skeleton />
				</div>
			{/each}
			<hr class="custom-hr-0" />
			<h3 class="custom-head-1 skeleton-text">Live Alerts</h3>
			{#each Array(2) as _, i}
				<div class="col-sm4">
					<MembershipCard skeleton />
				</div>
			{/each}
		</div>
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
		<!-- Memberships Grid - Matches WordPress cselt-row structure -->
		<div class="cselt-row">
			<!-- Trading Rooms Section -->
			{#if tradingRooms.length > 0}
				<h3 class="custom-head-0">Trading Rooms</h3>
				{#each tradingRooms as room (room.id)}
					<div class="col-sm4">
						<MembershipCard
							id={room.id}
							name={room.name}
							type={room.type}
							slug={room.slug}
							icon={room.icon}
							status={room.status}
							daysUntilExpiry={room.daysUntilExpiry}
							accessUrl={room.accessUrl}
							onclick={() => handleMembershipClick(room)}
						/>
					</div>
				{/each}
				<hr class="custom-hr-0" />
			{/if}

			<!-- Alert Services Section -->
			{#if alertServices.length > 0}
				<h3 class="custom-head-1">Live Alerts</h3>
				{#each alertServices as alert (alert.id)}
					<div class="col-sm4">
						<MembershipCard
							id={alert.id}
							name={alert.name}
							type={alert.type}
							slug={alert.slug}
							icon={alert.icon}
							status={alert.status}
							daysUntilExpiry={alert.daysUntilExpiry}
							accessUrl={alert.accessUrl}
							onclick={() => handleMembershipClick(alert)}
						/>
					</div>
				{/each}
				<hr class="custom-hr-1" />
			{/if}

			<!-- Courses Section -->
			{#if courses.length > 0}
				<h3 class="custom-head-2">Courses</h3>
				{#each courses as course (course.id)}
					<div class="col-sm4">
						<MembershipCard
							id={course.id}
							name={course.name}
							type={course.type}
							slug={course.slug}
							icon={course.icon}
							status={course.status}
							daysUntilExpiry={course.daysUntilExpiry}
							accessUrl={course.accessUrl}
							onclick={() => handleMembershipClick(course)}
						/>
					</div>
				{/each}
				<hr class="custom-hr-2" />
			{/if}

			<!-- Indicators Section -->
			{#if indicators.length > 0}
				<h3 class="custom-head-3">My Indicators</h3>
				<div id="my-indicators-cards">
					{#each indicators as indicator (indicator.id)}
						<div class="col-sm4">
							<MembershipCard
								id={indicator.id}
								name={indicator.name}
								type={indicator.type}
								slug={indicator.slug}
								icon={indicator.icon}
								status={indicator.status}
								daysUntilExpiry={indicator.daysUntilExpiry}
								accessUrl={indicator.accessUrl}
								onclick={() => handleMembershipClick(indicator)}
							/>
						</div>
					{/each}
				</div>
			{/if}
		</div>
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
	   CONTENT SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.wc-content-sction {
		width: 100%;
		margin: auto;
		padding: 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashb_headr {
		background-color: #fff;
		border-bottom: 1px solid var(--border-color);
		max-width: 100%;
		padding: 20px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
	}

	.dashb_headr-left,
	.dashb_headr-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.dashb_headr-right {
		flex-direction: row-reverse;
	}

	.dashb_pg-titl {
		color: var(--text-color);
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
	}

	.dashb_stats {
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
	   START HERE BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.start-here-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		font-size: 14px;
		line-height: 18px;
		padding: 8px 14px;
		font-weight: 600;
		color: var(--primary-color);
		background: var(--bg-light);
		border-color: transparent;
		text-decoration: none;
		border-radius: 5px;
		transition: var(--transition);
	}

	.start-here-btn:hover {
		color: var(--primary-color);
		background: #e7e7e7;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   INNER CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.wc-accontent-inner {
		padding: 4% 2%;
		background: #fff;
		border-radius: 5px;
		box-shadow: var(--card-shadow);
		position: relative;
		margin: 20px;
		min-height: 400px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP GRID
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cselt-row {
		display: flex;
		flex-flow: row wrap;
		align-items: flex-end;
		column-gap: 30px;
		row-gap: 40px;
	}

	.cselt-row .col-sm4 {
		flex: 0 0 auto;
		width: 100%;
		max-width: 320px;
	}

	#my-indicators-cards {
		display: flex;
		flex-flow: row wrap;
		gap: 30px;
		width: 100%;
	}

	#my-indicators-cards .col-sm4 {
		flex: 0 0 auto;
		width: 100%;
		max-width: 320px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION HEADERS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.custom-head-0,
	.custom-head-1,
	.custom-head-2,
	.custom-head-3 {
		color: var(--text-color);
		font-weight: 700;
		font-size: 20px;
		font-family: 'Open Sans', sans-serif;
		width: 100%;
		margin: 0;
	}

	.custom-head-1,
	.custom-head-2,
	.custom-head-3 {
		margin-top: 30px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION DIVIDERS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.custom-hr-0,
	.custom-hr-1,
	.custom-hr-2 {
		border: none;
		border-top: 1px solid var(--border-color);
		width: 100%;
		margin: 20px 0;
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
		.dashb_headr {
			padding: 16px;
		}

		.dashb_pg-titl {
			font-size: 28px;
		}

		.dashb_stats {
			display: none;
		}

		.wc-accontent-inner {
			margin: 12px;
			padding: 16px;
		}
	}

	@media screen and (min-width: 468px) {
		.cselt-row .col-sm4,
		#my-indicators-cards .col-sm4 {
			width: 48%;
		}
	}

	@media screen and (min-width: 768px) {
		.cselt-row .col-sm4,
		#my-indicators-cards .col-sm4 {
			width: 31%;
		}
	}

	@media screen and (min-width: 1280px) {
		.dashb_headr {
			padding: 30px;
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
	}
</style>
