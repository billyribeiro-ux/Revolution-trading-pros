<script lang="ts">
	/**
	 * Member Dashboard Page - Simpler Trading EXACT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard
	 * 100% EXACT clone of Simpler Trading dashboard
	 *
	 * @version 7.0.0 (Simpler Trading EXACT / December 2025)
	 */

	import { browser } from '$app/environment';
	import { IconUsers, IconArrowRight, IconAlertTriangle } from '$lib/icons';
	import MembershipCard from '$lib/components/dashboard/MembershipCard.svelte';
	import TradingRoomDropdown from '$lib/components/dashboard/TradingRoomDropdown.svelte';
	import Footer from '$lib/components/sections/Footer.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isLoading = $state(true);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// MOCK DATA - Simpler Trading EXACT (Remove when API is connected)
	// ═══════════════════════════════════════════════════════════════════════════

	const memberships = [
		{
			id: '1',
			name: 'Mastering the Trade',
			slug: 'mastering-the-trade',
			type: 'options',
			roomLabel: 'Trading Room'
		},
		{
			id: '2',
			name: 'Simpler Showcase',
			slug: 'simpler-showcase',
			type: 'foundation',
			roomLabel: 'Breakout Room'
		}
	];

	const tools = [
		{
			id: '3',
			name: 'Weekly Watchlist',
			slug: 'ww',
			type: 'ww'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// WEEKLY WATCHLIST CONFIG - Simpler Trading EXACT
	// ═══════════════════════════════════════════════════════════════════════════

	const weeklyWatchlist = {
		title: 'Weekly Watchlist with Allison Ostrander',
		subtitle: 'Week of December 15, 2025.',
		watchNowLink: '/dashboard/ww',
		image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg'
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const hasMemberships = $derived(memberships.length > 0 || tools.length > 0);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (!browser) return;
		isLoading = false;
	});
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
			{#if memberships.length > 0}
				<section class="dashboard__content-section">
					<h2 class="section-title">Memberships</h2>
					<div class="membership-cards row">
						{#each memberships as item (item.id)}
							<div class="col-sm-6 col-xl-4">
								<MembershipCard
									id={item.id}
									name={item.name}
									type={item.type}
									slug={item.slug}
									roomLabel={item.roomLabel}
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
						{#each tools as item (item.id)}
							<div class="col-sm-6 col-xl-4">
								<MembershipCard
									id={item.id}
									name={item.name}
									type={item.type}
									slug={item.slug}
								/>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			<!-- ═══════════════════════════════════════════════════════════════
			     WEEKLY WATCHLIST FEATURED - Simpler Trading EXACT
			     ═══════════════════════════════════════════════════════════════ -->
			<section class="weekly-watchlist-section">
				<div class="row">
					<div class="col-sm-6 col-lg-5">
						<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
						<!-- Mobile Image -->
						<div class="ww-mobile-image">
							<a href={weeklyWatchlist.watchNowLink}>
								<img src={weeklyWatchlist.image} alt="Weekly Watchlist" class="ww-image u--border-radius" />
							</a>
						</div>
						<h4 class="ww-title">{weeklyWatchlist.title}</h4>
						<p class="ww-subtitle">{weeklyWatchlist.subtitle}</p>
						<a href={weeklyWatchlist.watchNowLink} class="btn btn-tiny btn-default">Watch Now</a>
					</div>
					<div class="col-sm-6 col-lg-7 ww-desktop-image">
						<a href={weeklyWatchlist.watchNowLink}>
							<img src={weeklyWatchlist.image} alt="Weekly Watchlist" class="ww-image u--border-radius" />
						</a>
					</div>
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
		margin-top: 20px;
	}

	/* Section Title Alt - Simpler Trading EXACT */
	.section-title-alt {
		color: #d4a017;
		font-size: 14px;
		font-weight: 700;
		letter-spacing: 0.5px;
		text-transform: uppercase;
		margin: 0 0 8px 0;
		font-family: 'Open Sans', sans-serif;
	}

	.section-title-alt--underline {
		padding-bottom: 8px;
		border-bottom: 2px solid #d4a017;
		display: inline-block;
	}

	.ww-mobile-image {
		display: none;
		margin: 16px 0;
	}

	.ww-title {
		color: #333;
		font-size: 18px;
		font-weight: 700;
		margin: 16px 0 8px 0;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
	}

	.ww-subtitle {
		color: #666;
		font-size: 14px;
		margin: 0 0 16px 0;
	}

	/* Button Styles - Simpler Trading EXACT */
	.btn {
		display: inline-block;
		text-decoration: none;
		transition: all 0.15s ease;
		cursor: pointer;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: #f8f9fa;
		border: 1px solid #ddd;
		border-radius: 4px;
		color: #333;
		font-weight: 600;
	}

	.btn-default:hover {
		background: #e9ecef;
		border-color: #ccc;
	}

	.ww-image {
		width: 100%;
		height: auto;
		display: block;
	}

	.u--border-radius {
		border-radius: 8px;
	}

	/* Bootstrap Grid Columns - Simpler Trading EXACT */
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

	.ww-desktop-image {
		display: none;
	}

	@media (min-width: 992px) {
		.ww-desktop-image {
			display: block;
		}
		.ww-mobile-image {
			display: none;
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

	@media screen and (max-width: 991px) {
		.ww-desktop-image {
			display: none;
		}

		.ww-mobile-image {
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
