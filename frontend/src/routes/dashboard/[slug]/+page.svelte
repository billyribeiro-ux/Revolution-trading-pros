<script lang="ts">
	/**
	 * Membership Dashboard Page - WordPress EXACT Clone
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Matches frontend/dashboardmasteringthetrade EXACTLY:
	 * - Header with title, "New? Start Here", Trading Room Rules, Enter a Trading Room dropdown
	 * - Video player section
	 * - "Latest Updates" with 6 article cards
	 * - "Weekly Watchlist" section
	 * - Right sidebar: Trading Room Schedule + Quick Links
	 *
	 * @version 4.0.0 (WordPress EXACT - December 2025)
	 */

	import { page } from '$app/stores';

	// ═══════════════════════════════════════════════════════════════════════════
	// DROPDOWN STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let dropdownOpen = $state(false);

	function toggleDropdown() {
		dropdownOpen = !dropdownOpen;
	}

	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (!target.closest('.dropdown')) {
			dropdownOpen = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);

	// ═══════════════════════════════════════════════════════════════════════════
	// MEMBERSHIP DATA
	// ═══════════════════════════════════════════════════════════════════════════

	interface MembershipDetail {
		name: string;
		slug: string;
	}

	const membershipData = $derived.by((): MembershipDetail => {
		const memberships: Record<string, MembershipDetail> = {
			'mastering-the-trade': {
				name: 'Mastering the Trade',
				slug: 'mastering-the-trade'
			},
			'simpler-showcase': {
				name: 'Simpler Showcase',
				slug: 'simpler-showcase'
			}
		};

		return memberships[slug] || {
			name: slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' '),
			slug
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// TRADING ROOMS (WordPress EXACT - dropdown items)
	// ═══════════════════════════════════════════════════════════════════════════

	const tradingRooms = [
		{
			name: 'Mastering The Trade Room',
			href: 'https://chat.protradingroom.com',
			icon: 'st-icon-mastering-the-trade'
		},
		{
			name: 'Simpler Showcase Room',
			href: 'https://chat.protradingroom.com/simpler-showcase',
			icon: 'st-icon-simpler-showcase'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// LATEST UPDATES (WordPress EXACT - 6 article cards)
	// ═══════════════════════════════════════════════════════════════════════════

	const latestUpdates = [
		{
			id: 1,
			type: 'Daily Video',
			title: 'Holiday Weekend Market Review',
			date: 'December 19, 2025 with Sam',
			description: 'Indexes continue to churn sideways as we approach next week\'s holiday trade. Bulls usually take over in low volume. Can they do it again?',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			href: '/daily/mastering-the-trade/holiday-weekend-market-review'
		},
		{
			id: 2,
			type: 'Trading Room Archive',
			title: 'December 19, 2025',
			date: 'December 19, 2025',
			description: 'With Henry Gambell',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			href: '/chatroom-archive/mastering-the-trade/12192025'
		},
		{
			id: 3,
			type: 'Daily Video',
			title: 'Ho Ho Whoa!',
			date: 'December 18, 2025 with Bruce Marshall',
			description: 'In this video, Bruce discusses today\'s market action and the outlook for the end of the year. After CPI this morning, we got a nice bounce and relief rally after a long and messy chop phase since Thanksgiving.',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07135027/SimplerCentral_BM.jpg',
			href: '/daily/mastering-the-trade/ho-ho-whoa'
		},
		{
			id: 4,
			type: 'Trading Room Archive',
			title: 'December 18, 2025',
			date: 'December 18, 2025',
			description: 'With Henry Gambell',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			href: '/chatroom-archive/mastering-the-trade/12182025'
		},
		{
			id: 5,
			type: 'Daily Video',
			title: 'A Moment For The VIX',
			date: 'December 17, 2025 with HG',
			description: 'Today\'s action in stocks wasn\'t just out of the blue. We\'d been seeing weakness across the board, but it really came through today after the VIX expiration.',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			href: '/daily/mastering-the-trade/a-moment-for-the-vix'
		},
		{
			id: 6,
			type: 'Trading Room Archive',
			title: 'December 17, 2025',
			date: 'December 17, 2025',
			description: 'With John F. Carter',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			href: '/chatroom-archive/mastering-the-trade/12172025'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// WEEKLY WATCHLIST (WordPress EXACT)
	// ═══════════════════════════════════════════════════════════════════════════

	const weeklyWatchlist = {
		title: 'Weekly Watchlist with Allison Ostrander',
		date: 'Week of December 15, 2025.',
		href: '/watchlist/12152025-allison-ostrander',
		image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg'
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// TRADING ROOM SCHEDULE (WordPress EXACT - Google Calendar API data)
	// ═══════════════════════════════════════════════════════════════════════════

	const schedule = [
		{ traderName: 'Taylor Horton', dateTime: 'Dec 22, 2025, 9:20 AM EST' },
		{ traderName: 'Sam Shames', dateTime: 'Dec 22, 2025, 10:30 AM EST' },
		{ traderName: 'Neil Yeager', dateTime: 'Dec 22, 2025, 11:30 AM EST' },
		{ traderName: 'Bruce Marshall', dateTime: 'Dec 22, 2025, 2:00 PM EST' },
		{ traderName: 'Henry Gambell', dateTime: 'Dec 22, 2025, 3:00 PM EST' },
		{ traderName: 'Henry Gambell', dateTime: 'Dec 23, 2025, 9:15 AM EST' },
		{ traderName: 'Raghee Horner', dateTime: 'Dec 23, 2025, 10:30 AM EST' },
		{ traderName: 'David Starr', dateTime: 'Dec 23, 2025, 11:30 AM EST' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// QUICK LINKS (WordPress EXACT)
	// ═══════════════════════════════════════════════════════════════════════════

	const quickLinks = [
		{ title: 'Support', href: 'https://intercom.help/simpler-trading/en/' },
		{ title: 'Platform Tutorials', href: '/tutorials' },
		{ title: 'Simpler Blog', href: '/blog' }
	];
</script>

<svelte:head>
	<title>{membershipData.name} Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- Close dropdown when clicking outside -->
<svelte:window onclick={handleClickOutside} />

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER - WordPress EXACT: .dashboard__header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">{membershipData.name} Dashboard</h1>
		<a href="/dashboard/{slug}/start-here" class="btn btn-xs btn-default">
			New? Start Here
		</a>
	</div>

	<div class="dashboard__header-right">
		<!-- Trading Room Rules - WordPress EXACT -->
		<ul class="ultradingroom">
			<li class="litradingroom">
				<a href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf" target="_blank" class="btn btn-xs btn-link trading-rules-link">
					Trading Room Rules
				</a>
			</li>
			<li class="litradingroomhint">
				By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
			</li>
		</ul>

		<!-- Enter a Trading Room Dropdown - WordPress EXACT -->
		<div class="dropdown display-inline-block">
			<button
				class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle"
				onclick={toggleDropdown}
				aria-expanded={dropdownOpen}
			>
				<strong>Enter a Trading Room</strong>
			</button>

			<nav class="dropdown-menu dropdown-menu--full-width" class:show={dropdownOpen}>
				<ul class="dropdown-menu__menu">
					{#each tradingRooms as room (room.name)}
						<li>
							<a href={room.href} target="_blank" rel="nofollow">
								<span class="{room.icon} icon icon--md"></span>
								{room.name}
							</a>
						</li>
					{/each}
				</ul>
			</nav>
		</div>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT - WordPress EXACT: .dashboard__content
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Video Player Section - WordPress EXACT -->
		<section class="dashboard__content-section-member">
			<video
				controls
				width="100%"
				poster="https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg"
				style="aspect-ratio: 2 / 1;"
			>
				<source src="https://simpler-options.s3.amazonaws.com/tutorials/MTT_tutorial2025.mp4" type="video/mp4" />
				Your browser does not support the video tag.
			</video>
		</section>

		<!-- Latest Updates Section - WordPress EXACT -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Latest Updates</h2>
			<div class="article-cards row flex-grid">
				{#each latestUpdates as video (video.id)}
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure class="article-card__image" style="background-image: url({video.thumbnail});">
								<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt={video.title} />
							</figure>
							{#if video.type === 'Daily Video'}
								<div class="article-card__type">
									<span class="label label--info">Daily Video</span>
								</div>
							{/if}
							<h4 class="h5 article-card__title">
								<a href={video.href}>{video.title}</a>
							</h4>
							<span class="article-card__meta"><small>{video.date}</small></span>
							<div class="article-card__excerpt">
								<p>{video.description}</p>
							</div>
							<a href={video.href} class="btn btn-tiny btn-default">Watch Now</a>
						</article>
					</div>
				{/each}
			</div>
		</section>

		<!-- Weekly Watchlist Section - WordPress EXACT -->
		<div class="dashboard__content-section u--background-color-white">
			<section>
				<div class="row">
					<div class="col-sm-6 col-lg-5">
						<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
						<div class="watchlist-mobile-image">
							<a href={weeklyWatchlist.href}>
								<img src={weeklyWatchlist.image} alt="Weekly Watchlist image" class="u--border-radius" />
							</a>
						</div>
						<h4 class="h5 u--font-weight-bold">{weeklyWatchlist.title}</h4>
						<div class="watchlist-date">
							<p>{weeklyWatchlist.date}</p>
						</div>
						<a href={weeklyWatchlist.href} class="btn btn-tiny btn-default">Watch Now</a>
					</div>
					<div class="col-sm-6 col-lg-7 watchlist-desktop-image">
						<a href={weeklyWatchlist.href}>
							<img src={weeklyWatchlist.image} alt="Weekly Watchlist image" class="u--border-radius" />
						</a>
					</div>
				</div>
			</section>
		</div>
	</div>

	<!-- Right Sidebar - WordPress EXACT: .dashboard__content-sidebar -->
	<aside class="dashboard__content-sidebar">
		<!-- Trading Room Schedule -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">
				Trading Room Schedule
				<p class="pssubject">Schedule is subject to change.</p>
			</h4>
			<div class="room-sched">
				{#each schedule as item (item.traderName + item.dateTime)}
					<h4>{item.traderName}</h4>
					<span>{item.dateTime}</span>
				{/each}
			</div>
		</section>

		<!-- Quick Links -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Quick Links</h4>
			<ul class="link-list">
				{#each quickLinks as link (link.title)}
					<li>
						<a href={link.href} target="_blank">{link.title}</a>
					</li>
				{/each}
			</ul>
		</section>
	</aside>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress EXACT CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER - WordPress EXACT: .dashboard__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 20px 30px;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
		flex-wrap: wrap;
		gap: 16px;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		gap: 16px;
		flex-wrap: wrap;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.dashboard__header-right {
		display: flex;
		align-items: flex-start;
		gap: 20px;
		flex-wrap: wrap;
	}

	/* Trading Room Rules - WordPress EXACT */
	.ultradingroom {
		text-align: right;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.litradingroom {
		margin-bottom: 4px;
	}

	.trading-rules-link {
		font-weight: 700 !important;
		color: #0984ae;
		text-decoration: none;
	}

	.litradingroomhint {
		font-size: 11px;
		color: #6b7280;
		max-width: 200px;
	}

	/* Buttons - WordPress EXACT */
	.btn {
		display: inline-block;
		padding: 6px 12px;
		font-size: 12px;
		font-weight: 400;
		line-height: 1.5;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn-xs {
		padding: 4px 8px;
		font-size: 11px;
	}

	.btn-default {
		background: #f3f4f6;
		color: #374151;
		border: 1px solid #d1d5db;
	}

	.btn-default:hover {
		background: #e5e7eb;
	}

	.btn-link {
		background: transparent;
		border: none;
		padding: 0;
	}

	.btn-orange {
		background: #f99e31;
		color: #fff;
		border: none;
	}

	.btn-orange:hover {
		background: #e8890f;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 12px;
	}

	/* Dropdown - WordPress EXACT */
	.dropdown {
		position: relative;
	}

	.display-inline-block {
		display: inline-block;
	}

	.dropdown-toggle {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.dropdown-menu {
		display: none;
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 4px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		min-width: 260px;
		z-index: 1000;
	}

	.dropdown-menu.show {
		display: block;
	}

	.dropdown-menu__menu {
		list-style: none;
		margin: 0;
		padding: 8px 0;
	}

	.dropdown-menu__menu li a {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 16px;
		color: #374151;
		text-decoration: none;
		font-size: 14px;
		transition: background 0.15s ease;
	}

	.dropdown-menu__menu li a:hover {
		background: #f3f4f6;
		color: #0984ae;
	}

	/* Icon placeholders */
	.icon--md {
		width: 20px;
		height: 20px;
		display: inline-block;
		background: #0984ae;
		border-radius: 50%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT - WordPress EXACT: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		gap: 30px;
		padding: 30px;
		background: #f4f4f4;
	}

	.dashboard__content-main {
		flex: 1;
		min-width: 0;
	}

	.dashboard__content-section-member {
		margin-bottom: 30px;
	}

	.dashboard__content-section-member video {
		width: 100%;
		border-radius: 8px;
		background: #0a1628;
	}

	.dashboard__content-section {
		margin-bottom: 30px;
	}

	.section-title {
		font-size: 18px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 20px;
	}

	/* Article Cards Grid - WordPress EXACT */
	.article-cards {
		display: flex;
		flex-wrap: wrap;
		margin: -10px;
	}

	.flex-grid-item {
		padding: 10px;
	}

	.col-xs-12 { width: 100%; }

	@media (min-width: 576px) {
		.col-sm-6 { width: 50%; }
	}

	@media (min-width: 768px) {
		.col-md-6 { width: 50%; }
	}

	@media (min-width: 1200px) {
		.col-xl-4 { width: 33.333%; }
	}

	/* Article Card - WordPress EXACT */
	.article-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.article-card__image {
		position: relative;
		aspect-ratio: 16 / 9;
		background-size: cover;
		background-position: center;
		margin: 0;
	}

	.article-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.article-card__type {
		padding: 12px 16px 0;
	}

	.label {
		display: inline-block;
		padding: 2px 8px;
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		border-radius: 4px;
	}

	.label--info {
		background: #0984ae;
		color: #fff;
	}

	.article-card__title {
		padding: 8px 16px 0;
		margin: 0;
		font-size: 16px;
		font-weight: 600;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #1f2937;
		text-decoration: none;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.article-card__meta {
		padding: 4px 16px;
		color: #6b7280;
		font-size: 12px;
	}

	.article-card__excerpt {
		padding: 0 16px;
		flex: 1;
	}

	.article-card__excerpt p {
		font-size: 13px;
		color: #4b5563;
		line-height: 1.5;
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.article-card .btn {
		margin: 12px 16px 16px;
		align-self: flex-start;
	}

	/* Weekly Watchlist Section - WordPress EXACT */
	.u--background-color-white {
		background: #fff;
		border-radius: 8px;
		padding: 24px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.section-title-alt {
		font-size: 18px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 16px;
	}

	.section-title-alt--underline {
		padding-bottom: 12px;
		border-bottom: 2px solid #f99e31;
	}

	.u--font-weight-bold {
		font-weight: 700;
	}

	.h5 {
		font-size: 16px;
		margin: 0 0 8px;
	}

	.watchlist-date {
		margin-bottom: 12px;
	}

	.watchlist-date p {
		margin: 0;
		color: #6b7280;
		font-size: 14px;
	}

	.u--border-radius {
		border-radius: 8px;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: -15px;
	}

	.row > div {
		padding: 15px;
	}

	.watchlist-mobile-image {
		display: block;
		margin-bottom: 16px;
	}

	.watchlist-desktop-image {
		display: none;
	}

	.watchlist-desktop-image img,
	.watchlist-mobile-image img {
		width: 100%;
		height: auto;
	}

	@media (min-width: 992px) {
		.col-lg-5 { width: 41.666%; }
		.col-lg-7 { width: 58.333%; }

		.watchlist-mobile-image {
			display: none;
		}

		.watchlist-desktop-image {
			display: block;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RIGHT SIDEBAR - WordPress EXACT: .dashboard__content-sidebar
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-sidebar {
		width: 280px;
		flex-shrink: 0;
	}

	.content-sidebar__section {
		background: #fff;
		border-radius: 8px;
		padding: 20px;
		margin-bottom: 20px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.content-sidebar__heading {
		font-size: 14px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 16px;
	}

	.pssubject {
		font-size: 10px;
		margin-top: 8px;
		text-transform: initial;
		color: #6b7280;
		font-weight: 400;
	}

	.room-sched {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.room-sched h4 {
		font-size: 14px;
		font-weight: 500;
		color: #0984ae;
		margin: 0;
	}

	.room-sched span {
		font-size: 12px;
		color: #6b7280;
	}

	.link-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.link-list li {
		margin-bottom: 12px;
	}

	.link-list li:last-child {
		margin-bottom: 0;
	}

	.link-list a {
		color: #0984ae;
		font-size: 14px;
		text-decoration: none;
	}

	.link-list a:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1024px) {
		.dashboard__content {
			flex-direction: column;
		}

		.dashboard__content-sidebar {
			width: 100%;
		}
	}

	@media screen and (max-width: 768px) {
		.dashboard__header {
			flex-direction: column;
			align-items: flex-start;
		}

		.dashboard__header-right {
			width: 100%;
			flex-direction: column;
		}

		.ultradingroom {
			text-align: left;
		}

		.litradingroomhint {
			max-width: none;
		}
	}
</style>
