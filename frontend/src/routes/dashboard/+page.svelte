<script lang="ts">
	/**
	 * Member Dashboard Page - Simpler Trading EXACT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard
	 * Displays user's memberships, tools, and weekly watchlist
	 * EXACT clone of Simpler Trading dashboard
	 *
	 * @version 6.0.0 (Simpler Trading Exact / December 2025)
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
	// MOCK DATA - Simpler Trading EXACT (Remove when API is connected)
	// ═══════════════════════════════════════════════════════════════════════════

	const mockMemberships: UserMembership[] = [
		{
			id: '1',
			name: 'Mastering the Trade',
			slug: 'mastering-the-trade',
			type: 'trading-room',
			status: 'active',
			accessUrl: '/dashboard/mastering-the-trade/room',
			icon: 'st-icon-mastering-the-trade'
		},
		{
			id: '2',
			name: 'Simpler Showcase',
			slug: 'simpler-showcase',
			type: 'trading-room',
			status: 'active',
			accessUrl: '/dashboard/simpler-showcase/room',
			icon: 'st-icon-simpler-showcase',
			roomLabel: 'Breakout Room'
		}
	];

	const mockTools: UserMembership[] = [
		{
			id: '3',
			name: 'Weekly Watchlist',
			slug: 'ww',
			type: 'weekly-watchlist',
			status: 'active',
			accessUrl: '/dashboard/ww',
			icon: 'st-icon-trade-of-the-week'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// WEEKLY WATCHLIST CONFIG (Editable)
	// ═══════════════════════════════════════════════════════════════════════════

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

	// Use mock data for now - replace with API data when connected
	const tradingRooms = $derived(mockMemberships);
	const tools = $derived(mockTools);
	const hasMemberships = $derived(tradingRooms.length > 0 || tools.length > 0);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (!browser) return;
		// Set loading to false immediately since we're using mock data
		isLoading = false;
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

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
				<h2 class="section-title">Memberships</h2>
				<div class="membership-cards row">
					{#each Array(2) as _, i}
						<div class="col-sm-6 col-xl-4">
							<MembershipCard skeleton slug="loading-{i}" name="Loading..." />
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
				<button class="btn btn-primary" onclick={() => location.reload()}>
					Try Again
				</button>
			</div>
		{:else if hasMemberships}
			<!-- ═══════════════════════════════════════════════════════════════
			     MEMBERSHIPS SECTION - Simpler Trading EXACT
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
									roomLabel={room.roomLabel}
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
			     TOOLS SECTION - Simpler Trading EXACT
			     ═══════════════════════════════════════════════════════════════ -->
			{#if tools.length > 0}
				<section class="dashboard__content-section">
					<h2 class="section-title">Tools</h2>
					<div class="membership-cards row">
						{#each tools as tool (tool.id)}
							<div class="col-sm-6 col-xl-4">
								<MembershipCard
									id={tool.id}
									name={tool.name}
									type="ww"
									slug={tool.slug}
									icon={tool.icon}
									dashboardUrl="/dashboard/{tool.slug}"
									status={tool.status}
									daysUntilExpiry={tool.daysUntilExpiry}
									onclick={() => handleMembershipClick(tool)}
								/>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			<!-- ═══════════════════════════════════════════════════════════════
			     WEEKLY WATCHLIST FEATURED SECTION - Simpler Trading EXACT
			     ═══════════════════════════════════════════════════════════════ -->
			<section class="weekly-watchlist-section">
				<div class="ww-section__left">
					<h3 class="ww-section__header">Weekly Watchlist</h3>
					<!-- Mobile Image -->
					<div class="ww-section__mobile-image">
						<a href={weeklyWatchlistConfig.watchNowLink}>
							<img
								src={weeklyWatchlistConfig.image}
								alt="Weekly Watchlist"
								class="ww-image"
							/>
						</a>
					</div>
					<h4 class="ww-section__title">{weeklyWatchlistConfig.title}</h4>
					<p class="ww-section__subtitle">{weeklyWatchlistConfig.subtitle}</p>
					<a href={weeklyWatchlistConfig.watchNowLink} class="ww-section__btn">Watch Now</a>
				</div>
				<div class="ww-section__right">
					<a href={weeklyWatchlistConfig.watchNowLink}>
						<img
							src={weeklyWatchlistConfig.image}
							alt="Weekly Watchlist"
							class="ww-image"
						/>
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
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     FOOTER
     ═══════════════════════════════════════════════════════════════════════════ -->

<Footer />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - Simpler Trading EXACT
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

	/* Trading Room Rules - Simpler Trading EXACT */
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
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	.dashboard__content-main {
		max-width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT SECTIONS - Simpler Trading EXACT
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
	   MEMBERSHIP CARDS GRID - Simpler Trading EXACT
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
	   WEEKLY WATCHLIST SECTION - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.weekly-watchlist-section {
		display: flex;
		gap: 30px;
		margin-top: 20px;
	}

	.ww-section__left {
		flex: 0 0 40%;
		max-width: 40%;
	}

	.ww-section__right {
		flex: 0 0 60%;
		max-width: 60%;
	}

	.ww-section__header {
		color: #d4a017;
		font-size: 14px;
		font-weight: 700;
		letter-spacing: 0.5px;
		text-transform: uppercase;
		margin: 0 0 8px 0;
		padding-bottom: 8px;
		border-bottom: 2px solid #d4a017;
		display: inline-block;
	}

	.ww-section__mobile-image {
		display: none;
		margin-bottom: 16px;
	}

	.ww-section__title {
		color: #333;
		font-size: 18px;
		font-weight: 700;
		margin: 16px 0 8px 0;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
	}

	.ww-section__subtitle {
		color: #666;
		font-size: 14px;
		margin: 0 0 16px 0;
	}

	.ww-section__btn {
		display: inline-block;
		padding: 8px 16px;
		background: #f8f9fa;
		border: 1px solid #ddd;
		border-radius: 4px;
		color: #333;
		font-size: 13px;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.ww-section__btn:hover {
		background: #e9ecef;
		border-color: #ccc;
	}

	.ww-image {
		width: 100%;
		height: auto;
		border-radius: 8px;
		display: block;
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
		.weekly-watchlist-section {
			flex-direction: column;
		}

		.ww-section__left,
		.ww-section__right {
			flex: none;
			max-width: 100%;
		}

		.ww-section__right {
			display: none;
		}

		.ww-section__mobile-image {
			display: block;
		}
	}

	@media screen and (max-width: 768px) {
		.trading-room-rules {
			display: none;
		}

		.dashboard__header {
			padding: 16px 20px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
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

		.section-title {
			font-size: 20px;
		}
	}
</style>
