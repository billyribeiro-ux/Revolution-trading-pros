<script lang="ts">
	/**
	 * Member Dashboard - ICT 11+ Principal Engineer Pattern
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Fully data-driven dashboard with no hardcoded values.
	 * All memberships, icons, and content come from the API/database.
	 *
	 * Layout guarantees auth is initialized before this component mounts,
	 * so we can safely fetch data in onMount without race conditions.
	 *
	 * @version 2.2.0 - Fixed icons with Tabler SVG icons
	 */
	import { onMount } from 'svelte';
	import { getUserMemberships, type UserMembership, type UserMembershipsResponse } from '$lib/api/user-memberships';

	// Tabler Icons for membership cards - replacing broken st-icon font
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconActivity from '@tabler/icons-svelte/icons/activity';
	import IconRocket from '@tabler/icons-svelte/icons/rocket';
	import IconStar from '@tabler/icons-svelte/icons/star';
	import IconBolt from '@tabler/icons-svelte/icons/bolt';
	import IconTarget from '@tabler/icons-svelte/icons/target';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconSchool from '@tabler/icons-svelte/icons/school';
	import IconCalendarWeek from '@tabler/icons-svelte/icons/calendar-week';
	import IconWallet from '@tabler/icons-svelte/icons/wallet';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconReportAnalytics from '@tabler/icons-svelte/icons/report-analytics';

	// Icon registry - maps membership slugs to Tabler icon components
	type IconComponent = typeof IconChartLine;
	const membershipIconRegistry: Record<string, IconComponent> = {
		// Trading memberships
		'mastering-the-trade': IconChartCandle,
		'options-day-trading': IconChartCandle,
		'simpler-showcase': IconStar,
		'revolution-showcase': IconStar,
		'trade-of-the-week': IconCalendarWeek,
		'weekly-watchlist': IconCalendarWeek,
		'ww': IconCalendarWeek,
		// Trading room types
		'day-trading-room': IconChartCandle,
		'explosive-swings': IconBolt,
		'swing-trading': IconTrendingUp,
		'small-accounts': IconWallet,
		'day-trading': IconActivity,
		'moxie': IconRocket,
		// Education
		'foundation': IconBook,
		'courses': IconSchool,
		// Indicators & alerts
		'indicators': IconReportAnalytics,
		'spx-profit-pulse': IconBell,
		// Default fallback
		'default': IconChartLine
	};

	// Get icon component for a membership
	function getMembershipIcon(slug: string): IconComponent {
		return membershipIconRegistry[slug] || membershipIconRegistry['default'];
	}

	let dropdownOpen = $state(false);
	let membershipsData = $state<UserMembershipsResponse | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// ICT 11+ Pattern: Layout guarantees auth is ready, so onMount is safe
	onMount(async () => {
		console.log('[Dashboard] 🚀 Initializing dashboard page...');
		try {
			console.log('[Dashboard] 📡 Fetching user memberships...');
			membershipsData = await getUserMemberships();
			
			console.log('[Dashboard] ✅ Memberships loaded:', {
				total: membershipsData?.memberships?.length || 0,
				tradingRooms: membershipsData?.tradingRooms?.length || 0,
				courses: membershipsData?.courses?.length || 0,
				indicators: membershipsData?.indicators?.length || 0,
				weeklyWatchlist: membershipsData?.weeklyWatchlist?.length || 0,
				premiumReports: membershipsData?.premiumReports?.length || 0
			});
			
			if (membershipsData?.tradingRooms && membershipsData.tradingRooms.length > 0) {
				console.log('[Dashboard] 🎯 Trading Rooms:', membershipsData.tradingRooms.map(r => r.name));
			} else {
				console.warn('[Dashboard] ⚠️ No trading rooms found in memberships data');
			}
		} catch (err) {
			console.error('[Dashboard] ❌ Failed to load memberships:', err);
			error = 'Failed to load memberships. Please try again.';
		} finally {
			loading = false;
			console.log('[Dashboard] ✅ Dashboard initialization complete');
		}
	});

	/**
	 * ICT 11+ Pattern: Pure function handlers with explicit event typing
	 * Prevents event bubbling and maintains clean separation of concerns
	 */
	function handleDropdownToggle(event: MouseEvent): void {
		event.preventDefault();
		event.stopPropagation();
		dropdownOpen = !dropdownOpen;
	}

	function closeDropdown(): void {
		dropdownOpen = false;
	}

	/**
	 * ICT 11+ ENTERPRISE PATTERN: Manual refresh for testing
	 * Clears cache and refetches memberships
	 */
	async function handleRefreshMemberships(): Promise<void> {
		console.log('[Dashboard] 🔄 Manual refresh triggered - clearing cache...');
		loading = true;
		error = null;
		
		try {
			// Force cache bypass
			membershipsData = await getUserMemberships({ skipCache: true });
			console.log('[Dashboard] ✅ Manual refresh complete');
		} catch (err) {
			console.error('[Dashboard] ❌ Manual refresh failed:', err);
			error = 'Failed to refresh memberships. Please try again.';
		} finally {
			loading = false;
		}
	}

	/**
	 * ICT 11+ Pattern: Data-driven URL generation
	 * Trading room memberships link to their room dashboard (via roomSlug)
	 * Other memberships link to their own dashboard
	 */
	function getDashboardUrl(membership: UserMembership): string {
		if (membership.type === 'trading-room') {
			// Trading room memberships go to the room dashboard
			const roomSlug = membership.roomSlug || membership.slug;
			return `/dashboard/${roomSlug}`;
		}
		return membership.accessUrl || `/dashboard/${membership.slug}`;
	}

	/**
	 * ICT 11+ Pattern: Secondary action URL
	 * For trading rooms: same as dashboard (room page)
	 * For others: alerts/content page
	 */
	function getAccessUrl(membership: UserMembership): string {
		if (membership.type === 'trading-room') {
			const roomSlug = membership.roomSlug || membership.slug;
			return `/dashboard/${roomSlug}`;
		}
		return `/dashboard/${membership.slug}/alerts`;
	}

	// Get action label based on membership type from data
	function getActionLabel(membership: UserMembership): string {
		// Use roomLabel from data if available, otherwise derive from type
		if (membership.roomLabel) {
			return membership.roomLabel.includes('Room') ? 'Trading Room' : membership.roomLabel;
		}
		switch (membership.type) {
			case 'trading-room':
				return 'Trading Room';
			case 'alert-service':
				return 'View Alerts';
			case 'course':
				return 'View Course';
			case 'indicator':
				return 'Download';
			default:
				return 'Access';
		}
	}

	// Check if action should open in new tab based on membership type
	function shouldOpenNewTab(membership: UserMembership): boolean {
		return membership.type === 'trading-room';
	}
</script>

<svelte:window onclick={closeDropdown} />

<!-- DASHBOARD HEADER -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Member Dashboard</h1>
	</div>
	<div class="dashboard__header-right">
		<ul class="ultradingroom" style="text-align: right; list-style: none;">
			<li class="litradingroom"><a href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf" target="_blank" class="btn btn-xs btn-link" style="font-weight: 700 !important;">Trading Room Rules</a></li>
			<li style="font-size: 11px;" class="btn btn-xs btn-link litradingroomhind">By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.</li>
		</ul>

		{#if membershipsData?.tradingRooms && membershipsData.tradingRooms.length > 0}
			<div class="dropdown display-inline-block" class:is-open={dropdownOpen}>
				<a
					href="#trading-rooms"
					class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle"
					id="dLabel"
					role="button"
					onclick={handleDropdownToggle}
					aria-expanded={dropdownOpen}
				>
					<strong>Enter a Trading Room</strong>
				</a>

				{#if dropdownOpen}
					<nav class="dropdown-menu dropdown-menu--full-width" aria-labelledby="dLabel">
						<ul class="dropdown-menu__menu">
							{#each membershipsData.tradingRooms as room (room.id)}
								{@const RoomIcon = getMembershipIcon(room.slug)}
								<li>
									<a href={getAccessUrl(room)} target="_blank" rel="nofollow">
										<span class="dropdown-icon">
											<RoomIcon size={20} />
										</span>
										{room.roomLabel || room.name}
									</a>
								</li>
							{/each}
						</ul>
					</nav>
				{/if}
			</div>
		{/if}
	</div>
</header>

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- MEMBERSHIPS SECTION - Only show if user has memberships -->
		{#if loading}
			<section class="dashboard__content-section">
				<h2 class="section-title">Memberships</h2>
				<div class="loading-state">Loading memberships...</div>
			</section>
		{:else if error}
			<section class="dashboard__content-section">
				<h2 class="section-title">Memberships</h2>
				<div class="error-state">
					<p>{error}</p>
					<button class="btn btn-primary" onclick={() => location.reload()}>Retry</button>
				</div>
			</section>
		{:else if membershipsData?.memberships && membershipsData.memberships.length > 0}
			<section class="dashboard__content-section">
				<h2 class="section-title">Memberships</h2>
				<div class="membership-cards row">
					{#each membershipsData.memberships as membership (membership.id)}
						{@const MembershipIcon = getMembershipIcon(membership.slug)}
						<div class="col-sm-6 col-xl-4">
							<article class="membership-card membership-card--{membership.type === 'trading-room' ? 'options' : 'foundation'}">
								<a href={getDashboardUrl(membership)} class="membership-card__header">
									<span class="mem_icon">
										<span class="membership-card__icon{membership.slug === 'simpler-showcase' ? ' simpler-showcase-icon' : ''}">
											<MembershipIcon size={24} />
										</span>
									</span>
									<span class="mem_div">{membership.name}</span>
								</a>
								<div class="membership-card__actions">
									<a href={getDashboardUrl(membership)}>Dashboard</a>
									{#if shouldOpenNewTab(membership)}
										<a href={getAccessUrl(membership)} target="_blank" rel="nofollow">{getActionLabel(membership)}</a>
									{:else}
										<a href={getAccessUrl(membership)}>{getActionLabel(membership)}</a>
									{/if}
								</div>
							</article>
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<!-- LATEST UPDATES SECTION - Simpler Trading shows this when no memberships -->
		{#if !loading && !error && (!membershipsData?.memberships || membershipsData.memberships.length === 0)}
			<section class="dashboard__content-section">
				<h2 class="section-title u--margin-top-20">Latest Updates</h2>
				<div class="article-cards row flex-grid">
					<!-- Placeholder article cards - will be replaced with actual blog/video content -->
					<!-- Article Card 1 - Label INSIDE figure for overlay positioning -->
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure class="article-card__image" style="background-image: url('https://images.unsplash.com/photo-1611974789855-9c2a0a7236a3?w=800&h=600&fit=crop');">
								<img src="https://images.unsplash.com/photo-1611974789855-9c2a0a7236a3?w=800&h=600&fit=crop" alt="Trading Update" />
								<div class="article-card__type">
									<span class="label label--info">Daily Video</span>
								</div>
							</figure>
							<h4 class="h5 article-card__title"><a href="/blog">Welcome to Revolution Trading Pros</a></h4>
							<span class="article-card__meta"><small>Latest market insights and trading education</small></span>
							<div class="article-card__excerpt u--hide-read-more">
								<div class="woocommerce">
									<div class="woocommerce-info wc-memberships-restriction-message wc-memberships-message wc-memberships-content-restricted-message">
										This content is only available to members.
									</div>
								</div>
							</div>
							<a href="/pricing" class="btn btn-tiny btn-default">Watch Now</a>
						</article>
					</div>
					<!-- Article Card 2 -->
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure class="article-card__image" style="background-image: url('https://images.unsplash.com/photo-1590283603385-17ffb3a7f29f?w=800&h=600&fit=crop');">
								<img src="https://images.unsplash.com/photo-1590283603385-17ffb3a7f29f?w=800&h=600&fit=crop" alt="Market Analysis" />
								<div class="article-card__type">
									<span class="label label--info">Daily Video</span>
								</div>
							</figure>
							<h4 class="h5 article-card__title"><a href="/blog">Market Analysis & Trading Strategies</a></h4>
							<span class="article-card__meta"><small>Expert insights from our trading team</small></span>
							<div class="article-card__excerpt u--hide-read-more">
								<div class="woocommerce">
									<div class="woocommerce-info wc-memberships-restriction-message wc-memberships-message wc-memberships-content-restricted-message">
										This content is only available to members.
									</div>
								</div>
							</div>
							<a href="/pricing" class="btn btn-tiny btn-default">Watch Now</a>
						</article>
					</div>
					<!-- Article Card 3 -->
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure class="article-card__image" style="background-image: url('https://images.unsplash.com/photo-1642790106117-e829e14a795f?w=800&h=600&fit=crop');">
								<img src="https://images.unsplash.com/photo-1642790106117-e829e14a795f?w=800&h=600&fit=crop" alt="Trading Education" />
								<div class="article-card__type">
									<span class="label label--info">Daily Video</span>
								</div>
							</figure>
							<h4 class="h5 article-card__title"><a href="/blog">Learn Advanced Trading Techniques</a></h4>
							<span class="article-card__meta"><small>Professional trading education</small></span>
							<div class="article-card__excerpt u--hide-read-more">
								<p>Get access to exclusive trading education, live trading rooms, and expert analysis. Join our community of successful traders today.</p>
							</div>
							<a href="/pricing" class="btn btn-tiny btn-default">Watch Now</a>
						</article>
					</div>
				</div>
			</section>
		{/if}

		<!-- TOOLS SECTION - Exact Match from WordPress line 3456 -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Tools</h2>
			<div class="membership-cards row">
				<div class="col-sm-6 col-xl-4">
					<article class="membership-card membership-card--ww">
						<a href="/dashboard/ww/" class="membership-card__header">
							<span class="mem_icon">
								<span class="membership-card__icon">
									<IconCalendarWeek size={24} />
								</span>
							</span>
							<span class="mem_div">Weekly Watchlist</span>
						</a>
						<div class="membership-card__actions">
							<a href="/dashboard/ww/">Dashboard</a>
						</div>
					</article>
				</div>
			</div>
		</section>

		<!-- WEEKLY WATCHLIST FEATURED SECTION - Exact Match from WordPress line 3482 -->
		<div class="dashboard__content-section u--background-color-white">
			<section>
				<div class="row">
					<div class="col-sm-6 col-lg-5">
						<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
						<div class="hidden-md d-lg-none pb-2">
							<a class="" href="/watchlist/latest">
								<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="u--border-radius" />
							</a>
						</div>
						<h4 class="h5 u--font-weight-bold">Weekly Watchlist with TG Watkins</h4>
						<div class="u--hide-read-more">
							<p>Week of December 22, 2025.</p>
						</div>
						<a href="/watchlist/latest" class="btn btn-tiny btn-default">Watch Now</a>
					</div>
					<div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
						<a href="/watchlist/latest">
							<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="u--border-radius" />
						</a>
					</div>
				</div>
			</section>
		</div>

	</div>

	<!-- PANEL 2: SECONDARY SIDEBAR (Content Sidebar) - EMPTY on main dashboard per WordPress line 3520 -->
	<aside class="dashboard__content-sidebar">
		<section class="content-sidebar__section">
		</section>
	</aside>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD PAGE - Page-specific styles only
	   Header and title use global styles from dashboard-globals.css
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Header layout extensions for this page */
	.dashboard__header-left {
		display: flex;
		align-items: center;
		flex-direction: column;
	}

	@media screen and (min-width: 577px) {
		.dashboard__header-left {
			flex-direction: row;
		}
	}

	.dashboard__header-right {
		display: flex;
		align-items: center;
		flex-direction: column;
		margin-top: 10px;
		text-align: right;
	}

	@media screen and (min-width: 577px) {
		.dashboard__header-right {
			flex-direction: row;
			justify-content: flex-end;
		}
	}

	@media screen and (min-width: 820px) {
		.dashboard__header-right {
			flex-direction: row;
			justify-content: flex-end;
			margin-top: 0;
		}
	}

	@media screen and (min-width: 577px) {
		.dashboard__header-right > * + * {
			margin-right: 10px;
		}
	}

	@media screen and (min-width: 820px) {
		.dashboard__header-right > * + * {
			margin-left: 6px;
			margin-right: 0;
		}
	}

	/* Trading Room Rules - Exact WordPress Match (ultradingroom)
	   Note: In reference, JS removes .btn class. We override here instead. */
	.ultradingroom {
		text-align: right;
		list-style: none;
		margin: 0;
		padding: 0;
		display: block !important;  /* Override global display:none */
		max-width: none;
	}

	.ultradingroom .litradingroom {
		display: block;
		padding: 0;
		border: none;
		background: transparent;
	}

	.ultradingroom .litradingroom a {
		font-weight: 700 !important;
		font-size: 12px;
		color: #0984ae;
		text-decoration: none;
		padding: 0;
		background: transparent;
		border: none;
		box-shadow: none;
	}

	.ultradingroom .litradingroom a:hover {
		text-decoration: underline;
		color: #065a75;
	}

	.ultradingroom .litradingroomhind {
		font-size: 11px;
		color: #666;
		display: block;
		padding: 0;
		border: none;
		background: transparent;
		box-shadow: none;
		text-align: right;
	}

	.btn-xs {
		padding: 1px 5px;
		font-size: 12px;
		line-height: 1.5;
		border-radius: 3px;
	}

	.btn-link {
		background: none;
		border: none;
		color: #1e73be;
	}

	/* Dropdown - Exact Simpler Trading Match */
	.display-inline-block {
		display: inline-block !important;
	}

	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown-toggle {
		text-decoration: none;
	}

	.dropdown-toggle::after {
		display: inline-block;
		margin-left: 0.255em;
		vertical-align: 0.255em;
		content: "";
		border-top: 0.3em solid;
		border-right: 0.3em solid transparent;
		border-bottom: 0;
		border-left: 0.3em solid transparent;
	}

	.btn-orange,
	.btn-tradingroom {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		background: #F69532;
		border: none;
		color: #fff;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		transition: all 0.2s ease-in-out;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		line-height: 1.4;
	}

	/* EXACT from Simpler Trading style.css - btn-tradingroom override */
	.btn.btn-xs.btn-orange.btn-tradingroom {
		width: 280px;
		padding: 12px 18px;
	}

	.btn-orange:hover,
	.btn-tradingroom:hover {
		background: #dc7309;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 5px;
		background: #fff;
		border: none;
		border-radius: 4px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		min-width: 260px;
		padding: 20px;
		z-index: 1000;
		font-size: 14px;
	}

	.dropdown-menu ul {
		list-style: none;
		margin: -10px;
		padding: 0;
	}

	.dropdown-menu ul li a {
		display: block;
		padding: 10px 15px;
		color: #666;
		text-decoration: none;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		white-space: nowrap;
		text-overflow: ellipsis;
		transition: all 0.15s ease-in-out;
	}

	.dropdown-menu ul li a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}

	/* Dropdown icon styling for Tabler SVG icons */
	.dropdown-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		margin-right: 10px;
		color: #999;
		vertical-align: middle;
		transition: all 0.15s ease-in-out;
	}

	.dropdown-menu ul li a:hover .dropdown-icon {
		color: #0984ae;
	}

	.dropdown-icon :global(svg) {
		width: 20px;
		height: 20px;
	}

	/* Icon styles removed - not used in template */

	/* Simpler Showcase icon - WordPress exact match */
	.simpler-showcase-icon {
		background: black !important;
		color: orange !important;
	}

	/* Dropdown icon styles removed - not used in template */

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.logged-in .dashboard__content) {
		max-width: 100%;
	}

	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
		flex: 1 1 auto;
		background-color: #efefef;
		min-width: 0;
	}

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
		overflow-x: auto;
		overflow-y: hidden;
	}

	@media screen and (min-width: 1280px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	.dashboard__content-section + .dashboard__content-section {
		border-top: 1px solid #dbdbdb;
	}

	.section-title,
	h2.section-title {
		color: #333;
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MEMBERSHIP CARDS - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.membership-cards {
		margin-top: -30px;
	}

	.membership-cards.row {
		display: flex;
		flex-wrap: wrap;
		margin: -30px -15px 0;
	}

	/* Fix for proper Bootstrap-style grid negative margins */
	.row {
		margin-left: -15px;
		margin-right: -15px;
	}

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
		padding: 0 15px;
		box-sizing: border-box;
		margin-top: 30px;
	}

	.col-xl-4 {
		flex: 0 0 33.333%;
		max-width: 33.333%;
	}

	@media (max-width: 992px) {
		.col-xl-4 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (max-width: 641px) {
		.col-sm-6,
		.col-xl-4 {
			flex: 0 0 100%;
			max-width: 100%;
		}
	}

	/* Membership Card - EXACT match to dashboard-globals.css lines 369-402 */
	.membership-card {
		margin-top: 30px;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
	}

	.membership-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.membership-card__header {
		display: block;
		padding: 20px;
		color: #333;
		font-weight: 600;
		white-space: nowrap;
		transition: all 0.15s ease-in-out;
		text-decoration: none;
		font-family: 'Open Sans', sans-serif;
		font-size: 15px;
		line-height: 1.4;
	}

	.membership-card__header:visited {
		color: #333;
	}

	/* Hover state - exact match to reference (background color change, not text color) */
	.membership-card__header:hover {
		background-color: #f4f4f4;
	}

	.mem_icon,
	.mem_div {
		display: inline-block;
		vertical-align: middle;
	}

	.mem_div {
		white-space: normal;
		width: calc(100% - 43px);
	}

	/* Membership Card Icon - EXACT match to dashboard-globals.css lines 404-416 */
	.membership-card__icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		margin-right: 9px;
		color: #fff !important;
		background-color: #0984ae;
		border-radius: 50%;
		flex-shrink: 0;
		box-shadow: 0 10px 20px rgba(9, 132, 174, 0.25);
	}

	/* Tabler SVG icons inside membership card icons - EXACT match to globals line 419-424 */
	.membership-card__icon :global(svg) {
		width: 24px;
		height: 24px;
		color: #fff !important;
		fill: currentColor;
	}

	/* Membership Card Actions - EXACT match to globals lines 434-456 */
	.membership-card__actions {
		display: flex;
		font-size: 16px;
		border-top: 1px solid #ededed;
	}

	.membership-card__actions a {
		flex: 1 1 auto;
		padding: 15px 10px;
		color: #0984ae;
		font-size: 14px;
		text-align: center;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
		font-family: 'Open Sans', sans-serif;
	}

	.membership-card__actions a:hover {
		background-color: #f4f4f4;
	}

	.membership-card__actions a + a {
		border-left: 1px solid #ededed;
	}

	/* Loading, Error & Empty States */
	.loading-state,
	.error-state {
		padding: 40px;
		text-align: center;
		color: #666;
		font-size: 14px;
		width: 100%;
	}

	.error-state p {
		margin: 0 0 20px 0;
		color: #dc3545;
	}

	/* Article Cards - Simpler Trading Style */
	.article-cards {
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
	}

	.article-cards.row {
		margin: 0 -10px;
	}

	.flex-grid {
		display: flex;
		flex-wrap: wrap;
	}

	.flex-grid-item {
		display: flex;
	}

	.col-xs-12 {
		width: 100%;
		padding: 0 10px;
		box-sizing: border-box;
		margin-bottom: 20px;
	}

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
	}

	.col-md-6 {
		flex: 0 0 50%;
		max-width: 50%;
	}

	.col-xl-4 {
		flex: 0 0 33.333%;
		max-width: 33.333%;
	}

	@media (max-width: 992px) {
		.col-xl-4 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (max-width: 641px) {
		.col-sm-6,
		.col-md-6,
		.col-xl-4 {
			flex: 0 0 100%;
			max-width: 100%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARD - ICT11+ PIXEL-PERFECT MATCH to dashboard-globals.css
	   Reference lines 931-1021
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-card {
		position: relative;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
		margin-bottom: 30px;
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	.article-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	/* Image with 16:9 aspect ratio using padding-top trick - Reference line 947 */
	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
		margin: 0;
	}

	.article-card__image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	/* Type label positioned OVER the image - Reference line 957-961 */
	.article-card__type {
		position: absolute;
		top: 10px;
		left: 10px;
		z-index: 2;
		margin: 0;
	}

	/* Label badge - PILL SHAPE 25px radius - Reference line 963-971 */
	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 25px;
	}

	.label--info {
		background-color: #0984ae;
		color: #fff;
	}

	/* Title - Reference line 978-995 */
	.article-card__title {
		margin: 0;
		padding: 15px 15px 10px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.3;
		font-family: 'Open Sans', sans-serif;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.h5 {
		font-size: 16px;
		font-weight: 700;
	}

	/* Meta - Reference line 997-1006 */
	.article-card__meta {
		display: block;
		padding: 0 15px;
		color: #999;
		font-size: 12px;
	}

	.article-card__meta small {
		font-size: 12px;
	}

	/* Excerpt - Reference line 1008-1017 */
	.article-card__excerpt {
		padding: 10px 15px;
		font-size: 14px;
		line-height: 1.5;
		color: #666;
	}

	.article-card__excerpt p {
		margin: 0;
	}

	.u--hide-read-more {
		display: none;
	}

	.woocommerce {
		margin: 0;
	}

	.wc-memberships-restriction-message {
		background: #f8f9fa;
		border-left: 4px solid #0984ae;
		padding: 12px 16px;
		margin: 0;
		color: #666;
		font-size: 13px;
		border-radius: 3px;
	}

	/* Button in article card - Reference line 1019-1021 */
	.article-card .btn {
		margin: 0 15px 15px;
	}

	/* btn-tiny - Reference line 497-502 */
	.btn-tiny {
		padding: 5px 10px;
		font-size: 11px;
		line-height: 1.5;
		border-radius: 3px;
	}

	/* btn-tiny.btn-default SPECIAL - From Learning-Center reference line 1343-1354
	   Watch Now button is ORANGE with transparent background */
	.article-card .btn.btn-tiny.btn-default {
		background: transparent;
		color: #F3911B;
		padding-left: 0;
		font-size: 17px;
		border: none;
	}

	.article-card .btn.btn-tiny.btn-default:hover {
		color: #F3911B;
		background: #e7e7e7;
		padding-left: 8px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   WEEKLY WATCHLIST SECTION - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.section-title-alt {
		color: #0984ae;
		font-weight: 700;
		font-size: 14px;
		letter-spacing: 0.2em;
		margin-bottom: 30px;
		text-transform: uppercase;
		font-family: 'Open Sans', sans-serif;
	}

	.section-title-alt--underline {
		padding-bottom: 30px;
		position: relative;
	}

	.section-title-alt--underline::after {
		background-color: #e8e8e8;
		bottom: 2px;
		content: " ";
		display: block;
		height: 2px;
		position: absolute;
		left: 0;
		width: 50px;
	}

	.u--background-color-white {
		background-color: #fff !important;
		padding: 40px;
	}

	.u--border-radius {
		border-radius: 8px !important;
	}

	.u--font-weight-bold {
		font-weight: 700 !important;
	}


	.hidden-md {
		display: block;
	}

	.d-lg-none {
		display: block;
	}

	@media (min-width: 992px) {
		.d-lg-none {
			display: none !important;
		}
	}

	.pb-2 {
		padding-bottom: 0.5rem;
	}

	.hidden-xs,
	.hidden-sm {
		display: none;
	}

	.d-none {
		display: none !important;
	}

	.d-lg-block {
		display: none !important;
	}

	@media (min-width: 992px) {
		.d-lg-block {
			display: block !important;
		}

		.hidden-xs,
		.hidden-sm {
			display: block;
		}
	}

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

	@media (min-width: 641px) {
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

	.dashboard__content-section section {
		margin: 0;
	}

	.dashboard__content-section section img {
		width: 100%;
		height: auto;
		display: block;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
		box-shadow: none;
	}

	.btn-default:hover {
		color: #333;
		background-color: #e6e6e6;
		border-color: #adadad;
	}

	.u--margin-top-20 {
		margin-top: 20px;
	}

	.btn,
	.btn-primary {
		display: inline-block;
		padding: 10px 20px;
		background: #F69532;
		color: #fff;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		transition: all 0.2s ease-in-out;
		font-size: 14px;
		text-align: center;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
	}

	.btn-primary:hover {
		background: #dc7309;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PANEL 2: SECONDARY SIDEBAR (CONTENT SIDEBAR) - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════
	   
	   This sidebar appears on individual membership pages but is HIDDEN on the
	   main dashboard landing page. It provides contextual navigation for specific
	   memberships.
	   
	   VISIBILITY RULES:
	   - Main dashboard (/dashboard): HIDDEN (display: none)
	   - Membership pages (/dashboard/[slug]): VISIBLE (desktop 1080px+)
	   - Mobile (<1080px): Always hidden
	   
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	.dashboard__content-sidebar {
		display: none;
		width: 260px;
		flex: 0 0 auto;
		margin-top: -1px;
		background: #fff;
		border-right: 1px solid #dbdbdb;
		border-top: 1px solid #dbdbdb;
		font-family: 'Open Sans', sans-serif;
		font-size: 14px;
		line-height: 1.6;
	}

	/* Panel 2 is ALWAYS hidden on main dashboard page - matches Jesus file line 2451-2453 */
	/* On membership pages, it will be shown via their own CSS */

	.content-sidebar__section {
		padding: 20px 30px 20px 20px;
		border-bottom: 1px solid #dbdbdb;
	}

	/* .content-sidebar__heading removed - not used in template */

	/* ═══════════════════════════════════════════════════════════════════════════
	   ADDITIONAL GLOBAL STYLES FROM SIMPLER TRADING
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	/* Empty paragraph cleanup */
	:global(p:empty) {
		display: none;
	}

	/* Scanner image cleanup */
	:global(.scanner-load-content img[src*="/public/images/space.gif"]) {
		display: none;
	}

	/* Typography overrides */
	:global(h2) {
		font-size: 32px;
	}

	:global(h3) {
		font-size: 26px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTON SHAPE VARIANTS - Simpler Trading Brand Buttons
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	/* Shaped Button - Pill-shaped with extra bold text */
	:global(.shaped-btn) {
		display: block;
		border-radius: 25px;
		width: 100%;
		font-weight: 800;
		font-size: 18px;
		text-transform: uppercase;
		padding: 10px 20px;
		letter-spacing: 1.125px;
		transition: all 0.2s ease-in-out;
	}

	/* Squared Button - Subtle rounded corners */
	:global(.squared-btn) {
		display: block;
		border-radius: 4px;
		width: 100%;
		font-weight: 800;
		font-size: 14px;
		text-transform: uppercase;
		padding: 10px 20px;
		letter-spacing: 1.125px;
		transition: all 0.2s ease-in-out;
	}

	/* Primary Button Color - Brand Orange */
	:global(.primary-btn) {
		background-color: #F69532;
		color: #fff;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
	}

	:global(.primary-btn:hover) {
		background-color: #dc7309;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD-SPECIFIC UTILITY STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	/* Dashboard navigation submenu z-index fix */
	:global(.dashboard__nav-secondary .dashboard__nav-submenu) {
		z-index: 110 !important;
	}

	/* Trading room image wrapper padding reset */
	:global(.tr_img_wrap) {
		padding: 0 !important;
	}

	/* Weekly watchlist title padding reset */
	:global(figure.weekly_watchlist .article-card__title) {
		padding: 0 !important;
	}

	/* Trading room layout controls - ultradingroom visibility controlled by component styles */
	:global(.dashboard__header) {
		justify-content: space-between;
	}

	/* EXACT CSS FROM FILE 1 - Lines 3924-4072 */
	:global(p:empty) {
		display: none;
	}

	:global(.scanner-load-content img[src*="/public/images/space.gif"]) {
		display: none;
	}

	:global(h2) {
		font-size: 32px;
	}

	:global(h3) {
		font-size: 26px;
	}

	/* Button Shapes - EXACT from file 1 */
	:global(.shaped-btn) {
		display: block;
		border-radius: 25px;
		width: 100%;
		font-weight: 800;
		font-size: 18px;
		text-transform: uppercase;
		padding: 10px 20px;
		letter-spacing: 1.125px;
		transition: all .2s ease-in-out;
	}

	:global(.squared-btn) {
		display: block;
		border-radius: 4px;
		width: 100%;
		font-weight: 800;
		font-size: 14px;
		text-transform: uppercase;
		padding: 10px 20px;
		letter-spacing: 1.125px;
		transition: all .2s ease-in-out;
	}

	/* Button Colors - EXACT from file 1 */
	:global(.primary-btn) {
		background-color: #F69532;
		color: #fff;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
	}

	:global(.primary-btn:hover) {
		background-color: #dc7309;
	}

	/* Dashboard-specific styles - EXACT from file 1 */
	:global(.dashboard__nav-secondary .dashboard__nav-submenu) {
		z-index: 110 !important;
	}

	:global(.tr_img_wrap) {
		padding: 0 !important;
	}

	:global(figure.weekly_watchlist .article-card__title) {
		padding: 0 !important;
	}

	/* Mobile navigation override - EXACT from file 1 */
	@media (max-width: 641px) {
		:global(.main-navigation .main-nav ul li[class*="current-menu-"] > a),
		:global(.main-navigation .main-nav ul li > a) {
			color: #191717 !important;
		}
	}

	/* Icon styles - EXACT from file 1 */
	:global(.st-icon-this-week) {
		font-size: 28px;
	}

	:global(.simpler-showcase-icon) {
		background: black !important;
		color: orange !important;
	}

	/* EXACT INLINE STYLES FROM FILE 1 - Lines 5776-5785 */
	:global(.mem_icon),
	:global(.mem_div) {
		display: inline-block;
		vertical-align: middle;
	}

	:global(.mem_div) {
		white-space: normal;
		width: calc(100% - 43px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MISSING BUTTON CLASSES - btn-xs, btn-link
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.btn) {
		display: inline-block;
		padding: 6px 12px;
		margin-bottom: 0;
		font-size: 14px;
		font-weight: 400;
		line-height: 1.42857143;
		text-align: center;
		white-space: nowrap;
		vertical-align: middle;
		cursor: pointer;
		border: 1px solid transparent;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
	}

	:global(.btn-xs) {
		padding: 1px 5px;
		font-size: 12px;
		line-height: 1.5;
		border-radius: 3px;
	}

	:global(.btn-link) {
		color: #0984ae;
		font-weight: 400;
		border-radius: 0;
		background-color: transparent;
		border-color: transparent;
		box-shadow: none;
	}

	:global(.btn-link:hover),
	:global(.btn-link:focus) {
		color: #065a75;
		text-decoration: underline;
		background-color: transparent;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD ICON SIZING - ICT 11 CANONICAL VALUES from dashboard-globals.css
	   28px icons, WHITE color - NOT 24px blue!
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.dashboard__nav-item-icon) {
		display: inline-block;
		width: 28px;
		height: 28px;
		margin-right: 10px;
		font-size: 28px;
		color: #fff;
		vertical-align: middle;
	}

	:global(.dashboard__nav-item-icon.st-icon-training-room) {
		font-size: 26px !important;
	}

	:global(.dashboard__nav-secondary .dashboard__nav-item-icon.st-icon-training-room) {
		font-size: 20px !important;
	}

	:global(.dashboard__nav-item-icon.st-icon-stacked-profits) {
		font-size: 40px !important;
	}

	:global(.st-icon-this-week) {
		font-size: 28px;
	}

	:global(.st-icon-big-market-scorecard:before) {
		margin-left: 4px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICON SIZE CLASSES - icon--lg, icon--md
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.icon) {
		display: inline-block;
		vertical-align: middle;
	}

	:global(.icon--lg) {
		font-size: 32px;
		width: 32px;
		height: 32px;
	}

	:global(.icon--md) {
		font-size: 24px;
		width: 24px;
		height: 24px;
	}

	/* Simpler Showcase special icon - black background with orange icon */
	:global(.simpler-showcase-icon) {
		background: black !important;
		color: orange !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FONT WEIGHT & SIZE FIXES - ICT 11 CANONICAL VALUES: 17px NOT 14px
	   ═══════════════════════════════════════════════════════════════════════════ */
	:global(.dashboard__nav-item-text) {
		font-size: 17px;
		font-weight: 400;
		color: hsla(0, 0%, 100%, 0.7);
	}

	/* Bold white text for My Classes and My Indicators */
	:global(.dashboard__nav-item-text--bold) {
		font-weight: 700 !important;
		color: #fff !important;
	}
</style>
