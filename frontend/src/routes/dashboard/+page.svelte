<script lang="ts">
	/**
	 * Member Dashboard Page - Simpler Trading EXACT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard
	 * Displays user's memberships, tools, and weekly watchlist
	 *
	 * @version 5.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { IconUsers, IconArrowRight, IconAlertTriangle } from '$lib/icons';
	import MembershipCard from '$lib/components/dashboard/MembershipCard.svelte';
	import TradingRoomDropdown from '$lib/components/dashboard/TradingRoomDropdown.svelte';
	import Footer from '$lib/components/sections/Footer.svelte';
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

	const data = $derived($page.data as DashboardPageData);

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let membershipsData = $state<UserMembershipsResponse | null>(null);
	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// WEEKLY WATCHLIST CONFIG (Editable)
	// ═══════════════════════════════════════════════════════════════════════════

	// This data would typically come from a CMS or API
	const weeklyWatchlistConfig = {
		title: 'Weekly Watchlist with Allison Ostrander',
		subtitle: 'Week of December 15, 2025.',
		watchNowLink: '/dashboard/ww',
		image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg',
		traderName: 'ALLISON OSTRANDER',
		traderTitle: 'Director of Risk Tolerance',
		badge: 'WEEKLY WATCHLIST'
	};

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

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (!browser) return;

		if (data?.authError) {
			error = 'Session expired. Please log in again.';
			isLoading = false;
			return;
		}

		if (data?.needsRefresh) {
			refreshMemberships();
			return;
		}

		if (data?.memberships) {
			membershipsData = data.memberships;
			isLoading = false;
		} else {
			loadMemberships();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

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

	function handleRetry(): void {
		if (error?.includes('Session expired')) {
			goto('/login?redirect=/dashboard');
		} else {
			refreshMemberships();
		}
	}

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
	<title>Member Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Member Dashboard</h1>
	</div>
	<div class="dashboard__header-right">
		<TradingRoomDropdown />
		<div class="trading-room-rules">
			<a
				href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf"
				target="_blank"
				class="trading-room-rules__link"
			>
				Trading Room Rules
			</a>
			<p class="trading-room-rules__disclaimer">
				By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
			</p>
		</div>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		{#if isLoading}
			<!-- Skeleton Loading State -->
			<section class="dashboard__content-section">
				<h2 class="section-title skeleton-text">Memberships</h2>
				<div class="membership-cards row">
					{#each Array(2) as _, i}
						<div class="col-sm-6 col-xl-4">
							<MembershipCard skeleton slug="loading-{i}" name="Loading..." />
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
			<!-- ═══════════════════════════════════════════════════════════════
			     MEMBERSHIPS SECTION - Trading Rooms
			     ═══════════════════════════════════════════════════════════════ -->
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
									membershipType={room.membershipType}
									daysUntilExpiry={room.daysUntilExpiry}
									useSSO={true}
									onclick={() => handleMembershipClick(room)}
								/>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			<!-- ═══════════════════════════════════════════════════════════════
			     TOOLS SECTION - Weekly Watchlist + Alert Services
			     ═══════════════════════════════════════════════════════════════ -->
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

			<!-- ═══════════════════════════════════════════════════════════════
			     WEEKLY WATCHLIST FEATURED SECTION - Simpler Trading EXACT
			     ═══════════════════════════════════════════════════════════════ -->
			<section class="weekly-watchlist-featured">
				<div class="ww-featured__content">
					<h3 class="ww-featured__header">WEEKLY WATCHLIST</h3>
					<h4 class="ww-featured__title">{weeklyWatchlistConfig.title}</h4>
					<p class="ww-featured__subtitle">{weeklyWatchlistConfig.subtitle}</p>
					<a href={weeklyWatchlistConfig.watchNowLink} class="ww-featured__link">Watch Now</a>
				</div>
				<div class="ww-featured__image-container">
					<a href={weeklyWatchlistConfig.watchNowLink}>
						<div class="ww-featured__image-wrapper">
							<span class="ww-featured__badge">{weeklyWatchlistConfig.badge}</span>
							<img
								src={weeklyWatchlistConfig.image}
								alt="Weekly Watchlist"
								class="ww-featured__image"
							/>
							<div class="ww-featured__overlay">
								<p class="ww-featured__brand">REVOLUTION<span>TRADING</span></p>
								<h5 class="ww-featured__trader-name">{weeklyWatchlistConfig.traderName}</h5>
								<p class="ww-featured__trader-title">{weeklyWatchlistConfig.traderTitle}</p>
							</div>
						</div>
					</a>
				</div>
			</section>
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

	<!-- Sidebar (empty on main dashboard - matches Simpler Trading) -->
	<aside class="dashboard__content-sidebar">
		<!-- Empty sidebar on main dashboard page -->
	</aside>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     FOOTER - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<Footer />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #e9ebed;
		padding: 20px 30px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
	}

	.dashboard__header-right {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* Trading Room Rules */
	.trading-room-rules {
		text-align: right;
		max-width: 220px;
	}

	.trading-room-rules__link {
		display: block;
		font-weight: 700;
		color: #1e73be;
		text-decoration: none;
		font-size: 14px;
		margin-bottom: 4px;
	}

	.trading-room-rules__link:hover {
		text-decoration: underline;
	}

	.trading-room-rules__disclaimer {
		font-size: 11px;
		color: #6b7280;
		line-height: 1.3;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		gap: 30px;
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	.dashboard__content-main {
		flex: 1;
		min-width: 0;
	}

	.dashboard__content-sidebar {
		width: 280px;
		flex-shrink: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT SECTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		margin-bottom: 40px;
	}

	.section-title {
		color: #333;
		font-weight: 700;
		font-size: 24px;
		font-family: 'Open Sans Condensed', sans-serif;
		margin: 0 0 20px 0;
		padding-bottom: 10px;
		border-bottom: 2px solid #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARDS GRID
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

	.row {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

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
	   WEEKLY WATCHLIST FEATURED SECTION - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ -->

	.weekly-watchlist-featured {
		display: flex;
		gap: 30px;
		padding: 0;
		margin-top: 20px;
	}

	.ww-featured__content {
		flex: 0 0 40%;
		max-width: 40%;
	}

	.ww-featured__header {
		color: #d4a017;
		font-size: 14px;
		font-weight: 700;
		letter-spacing: 0.5px;
		margin: 0 0 16px 0;
		padding-bottom: 8px;
		border-bottom: 2px solid #d4a017;
		display: inline-block;
	}

	.ww-featured__title {
		color: #333;
		font-size: 20px;
		font-weight: 700;
		margin: 0 0 8px 0;
		font-family: 'Open Sans', sans-serif;
	}

	.ww-featured__subtitle {
		color: #666;
		font-size: 14px;
		margin: 0 0 16px 0;
	}

	.ww-featured__link {
		color: #dc7309;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
	}

	.ww-featured__link:hover {
		text-decoration: underline;
	}

	.ww-featured__image-container {
		flex: 0 0 60%;
		max-width: 60%;
	}

	.ww-featured__image-wrapper {
		position: relative;
		border-radius: 8px;
		overflow: hidden;
		background: linear-gradient(135deg, #0a2540 0%, #1e4d7b 100%);
	}

	.ww-featured__badge {
		position: absolute;
		top: 12px;
		right: 12px;
		background: #0984ae;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		padding: 4px 10px;
		border-radius: 4px;
		z-index: 2;
	}

	.ww-featured__image {
		width: 100%;
		height: auto;
		display: block;
		object-fit: cover;
	}

	.ww-featured__overlay {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		background: linear-gradient(transparent, rgba(0, 80, 130, 0.95));
		padding: 40px 20px 20px;
		text-align: center;
	}

	.ww-featured__brand {
		color: #fff;
		font-size: 10px;
		font-weight: 400;
		letter-spacing: 2px;
		margin: 0 0 4px 0;
	}

	.ww-featured__brand span {
		font-weight: 700;
	}

	.ww-featured__trader-name {
		color: #fff;
		font-size: 22px;
		font-weight: 700;
		margin: 0 0 4px 0;
		font-family: 'Open Sans Condensed', sans-serif;
	}

	.ww-featured__trader-title {
		color: rgba(255, 255, 255, 0.8);
		font-size: 12px;
		font-weight: 400;
		margin: 0;
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
		0% { background-position: -200% 0; }
		100% { background-position: 200% 0; }
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
		color: #ef4444;
	}

	.error-message {
		color: #333;
		font-size: 16px;
		margin-bottom: 24px;
		max-width: 400px;
	}

	.btn-primary {
		background: #0984ae;
		color: white;
		border: none;
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.btn-primary:hover {
		background: #076787;
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
		color: #6b7280;
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: #f3f4f6;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 24px;
		color: #0984ae;
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: #333;
		margin: 0 0 8px;
	}

	.empty-state p {
		margin: 0 0 24px;
	}

	.btn-orange {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: #f99e31;
		color: #fff;
		font-weight: 700;
		padding: 12px 24px;
		border-radius: 5px;
		text-decoration: none;
		transition: background 0.15s ease;
	}

	.btn-orange:hover {
		background: #dc7309;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 992px) {
		.dashboard__content {
			flex-direction: column;
		}

		.dashboard__content-sidebar {
			display: none;
		}

		.weekly-watchlist-featured {
			flex-direction: column;
		}

		.ww-featured__content,
		.ww-featured__image-container {
			flex: none;
			max-width: 100%;
		}
	}

	@media screen and (max-width: 768px) {
		.trading-room-rules {
			display: none;
		}

		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}
	}

	@media screen and (max-width: 576px) {
		.dashboard__header {
			flex-direction: column;
			align-items: flex-start;
		}

		.dashboard__header-right {
			width: 100%;
			justify-content: flex-end;
		}

		.dashboard__content {
			padding: 16px;
		}

		.section-title {
			font-size: 20px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.skeleton-text {
			animation: none;
		}
	}
</style>
