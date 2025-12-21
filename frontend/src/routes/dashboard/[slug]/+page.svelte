<script lang="ts">
	/**
	 * Membership Dashboard Page - WordPress EXACT Clone
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Matches frontend/dashboardmasteringthetrade EXACTLY:
	 * - Breadcrumb: Home / Member Dashboard / Page Title
	 * - Header with title, "New? Start Here", Trading Room Rules, Enter dropdown
	 * - Video player section
	 * - "Latest Updates" with 6 article cards
	 * - "Weekly Watchlist" section
	 * - Right sidebar: Trading Room Schedule + Quick Links
	 *
	 * @version 1.0.0 (WordPress EXACT - December 2025)
	 */

	import { page } from '$app/stores';
	import Breadcrumb from '$lib/components/dashboard/Breadcrumb.svelte';
	import TradingRoomDropdown from '$lib/components/dashboard/TradingRoomDropdown.svelte';
	import Footer from '$lib/components/sections/Footer.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);

	// ═══════════════════════════════════════════════════════════════════════════
	// MEMBERSHIP DATA
	// ═══════════════════════════════════════════════════════════════════════════

	interface MembershipDetail {
		name: string;
		dashboardTitle: string;
		slug: string;
		videoPoster: string;
		videoSrc: string;
	}

	const membershipData = $derived.by((): MembershipDetail => {
		const memberships: Record<string, MembershipDetail> = {
			'mastering-the-trade': {
				name: 'Mastering the Trade',
				dashboardTitle: 'Mastering the Trade Dashboard',
				slug: 'mastering-the-trade',
				videoPoster: 'https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg',
				videoSrc: 'https://simpler-options.s3.amazonaws.com/tutorials/MTT_tutorial2025.mp4'
			},
			'simpler-showcase': {
				name: 'Simpler Showcase',
				dashboardTitle: 'Simpler Showcase Dashboard',
				slug: 'simpler-showcase',
				videoPoster: 'https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg',
				videoSrc: 'https://simpler-options.s3.amazonaws.com/tutorials/MTT_tutorial2025.mp4'
			}
		};

		const defaultName = slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ');
		return memberships[slug] || {
			name: defaultName,
			dashboardTitle: `${defaultName} Dashboard`,
			slug,
			videoPoster: 'https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg',
			videoSrc: 'https://simpler-options.s3.amazonaws.com/tutorials/MTT_tutorial2025.mp4'
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// BREADCRUMB
	// ═══════════════════════════════════════════════════════════════════════════

	const breadcrumbItems = $derived([
		{ label: 'Home', href: '/' },
		{ label: 'Member Dashboard', href: '/dashboard' },
		{ label: membershipData.dashboardTitle }
	]);

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
			type: 'Archive',
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
			type: 'Archive',
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
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			href: '/daily/mastering-the-trade/a-moment-for-the-vix'
		},
		{
			id: 6,
			type: 'Archive',
			title: 'December 17, 2025',
			date: 'December 17, 2025',
			description: 'With John F. Carter',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			href: '/chatroom-archive/mastering-the-trade/12172025'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// WEEKLY WATCHLIST
	// ═══════════════════════════════════════════════════════════════════════════

	const weeklyWatchlist = {
		title: 'Weekly Watchlist with Allison Ostrander',
		subtitle: 'Week of December 15, 2025.',
		watchNowLink: '/watchlist/12152025-allison-ostrander',
		image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg'
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// TRADING ROOM SCHEDULE (WordPress EXACT)
	// ═══════════════════════════════════════════════════════════════════════════

	const tradingRoomSchedule = [
		{ trader: 'Taylor Horton', date: 'Dec 22, 2025, 9:20 AM EST' },
		{ trader: 'Sam Shames', date: 'Dec 22, 2025, 10:30 AM EST' },
		{ trader: 'Neil Yeager', date: 'Dec 22, 2025, 11:30 AM EST' },
		{ trader: 'Bruce Marshall', date: 'Dec 22, 2025, 2:00 PM EST' },
		{ trader: 'Henry Gambell', date: 'Dec 22, 2025, 3:00 PM EST' },
		{ trader: 'Henry Gambell', date: 'Dec 23, 2025, 9:15 AM EST' },
		{ trader: 'Raghee Horner', date: 'Dec 23, 2025, 10:30 AM EST' },
		{ trader: 'David Starr', date: 'Dec 23, 2025, 11:30 AM EST' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// QUICK LINKS (WordPress EXACT)
	// ═══════════════════════════════════════════════════════════════════════════

	const quickLinks = [
		{ label: 'Support', href: 'https://intercom.help/simpler-trading/en/', external: true },
		{ label: 'Platform Tutorials', href: '/platform-tutorials', external: false },
		{ label: 'Simpler Blog', href: '/blog', external: false }
	];
</script>

<svelte:head>
	<title>{membershipData.dashboardTitle} | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     BREADCRUMB - WordPress EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<Breadcrumb items={breadcrumbItems} />

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER - WordPress EXACT: .dashboard__header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">{membershipData.dashboardTitle}</h1>
		<a href="/dashboard/{slug}/start-here" class="btn btn-xs btn-default">
			New? Start Here
		</a>
	</div>

	<div class="dashboard__header-right">
		<!-- Trading Room Rules - WordPress EXACT -->
		<ul class="ultradingroom">
			<li class="litradingroom">
				<a
					href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf"
					target="_blank"
					rel="noopener noreferrer"
					class="trading-rules-link"
				>
					Trading Room Rules
				</a>
			</li>
			<li class="litradingroomhint">
				By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
			</li>
		</ul>

		<!-- Enter a Trading Room Dropdown - WordPress EXACT -->
		<TradingRoomDropdown />
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
				poster={membershipData.videoPoster}
				style="aspect-ratio: 2 / 1;"
			>
				<source src={membershipData.videoSrc} type="video/mp4" />
				<track kind="captions" />
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
							<div class="article-card__excerpt u--hide-read-more">
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
						<!-- Mobile Image -->
						<div class="hidden-md d-lg-none pb-2">
							<a href={weeklyWatchlist.watchNowLink}>
								<img src={weeklyWatchlist.image} alt="Weekly Watchlist image" class="u--border-radius" />
							</a>
						</div>
						<h4 class="h5 u--font-weight-bold">{weeklyWatchlist.title}</h4>
						<div class="u--hide-read-more">
							<p>{weeklyWatchlist.subtitle}</p>
						</div>
						<a href={weeklyWatchlist.watchNowLink} class="btn btn-tiny btn-default">Watch Now</a>
					</div>
					<!-- Desktop Image -->
					<div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
						<a href={weeklyWatchlist.watchNowLink}>
							<img src={weeklyWatchlist.image} alt="Weekly Watchlist image" class="u--border-radius" />
						</a>
					</div>
				</div>
			</section>
		</div>
	</div>

	<!-- Right Sidebar - WordPress EXACT -->
	<aside class="dashboard__content-sidebar">
		<!-- Trading Room Schedule -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">
				Trading Room Schedule
				<p class="pssubject">Schedule is subject to change.</p>
			</h4>
			<div class="room-sched">
				{#each tradingRoomSchedule as event}
					<div class="schedule-item">
						<h4 class="trader-name">{event.trader}</h4>
						<span class="trader-time">{event.date}</span>
					</div>
				{/each}
			</div>
		</section>

		<!-- Quick Links -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Quick Links</h4>
			<ul class="link-list">
				{#each quickLinks as link}
					<li>
						<a
							href={link.href}
							target={link.external ? '_blank' : undefined}
							rel={link.external ? 'noopener noreferrer' : undefined}
						>
							{link.label}
						</a>
					</li>
				{/each}
			</ul>
		</section>
	</aside>
</div>

<!-- Footer -->
<Footer />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		max-width: 1700px;
		padding: 20px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: flex-start;
		gap: 16px;
	}

	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		gap: 16px;
		flex-wrap: wrap;
	}

	.dashboard__header-right {
		display: flex;
		align-items: center;
		gap: 16px;
		flex-wrap: wrap;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* New? Start Here button */
	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
		border-radius: 4px;
	}

	.btn-default {
		background: #f8f9fa;
		border: 1px solid #ddd;
		color: #333;
		font-weight: 600;
		text-decoration: none;
		display: inline-block;
		transition: all 0.15s ease;
	}

	.btn-default:hover {
		background: #e9ecef;
		border-color: #ccc;
	}

	/* Trading Room Rules */
	.ultradingroom {
		text-align: right;
		list-style: none;
		margin: 0;
		padding: 0;
		max-width: 300px;
	}

	.ultradingroom .litradingroom {
		margin-bottom: 2px;
	}

	.trading-rules-link {
		font-weight: 700;
		color: #1e73be;
		text-decoration: none;
		font-size: 14px;
	}

	.trading-rules-link:hover {
		text-decoration: underline;
	}

	.ultradingroom .litradingroomhint {
		font-size: 11px;
		color: #6b7280;
		line-height: 1.3;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
		padding: 30px;
		background: #f4f4f4;
		gap: 30px;
	}

	.dashboard__content-main {
		flex: 1 1 auto;
		min-width: 0;
	}

	.dashboard__content-sidebar {
		flex: 0 0 280px;
		max-width: 280px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO SECTION - WordPress EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section-member {
		margin-bottom: 30px;
	}

	.dashboard__content-section-member video {
		width: 100%;
		border-radius: 8px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT SECTIONS - WordPress EXACT
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
	   ARTICLE CARDS GRID - WordPress EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.article-cards {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.flex-grid-item {
		padding: 0 15px;
		margin-bottom: 30px;
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
		border-radius: 5px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		overflow: hidden;
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.article-card__image {
		position: relative;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
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

	.article-card__type {
		padding: 12px 16px 0;
	}

	.label--info {
		background: #0984ae;
		color: #fff;
		padding: 4px 8px;
		border-radius: 3px;
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
	}

	.article-card__title {
		padding: 12px 16px 0;
		margin: 0;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.article-card__meta {
		padding: 8px 16px;
		color: #6b7280;
		font-size: 12px;
	}

	.article-card__excerpt {
		padding: 0 16px;
		flex: 1;
	}

	.article-card__excerpt p {
		color: #666;
		font-size: 13px;
		line-height: 1.5;
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.article-card .btn-tiny {
		margin: 16px;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   WEEKLY WATCHLIST SECTION - WordPress EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.u--background-color-white {
		background-color: #fff;
		padding: 30px;
		border-radius: 8px;
	}

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

	.h5 {
		font-size: 18px;
		font-weight: 700;
		margin: 16px 0 8px 0;
		color: #333;
	}

	.u--font-weight-bold {
		font-weight: 700 !important;
	}

	.u--hide-read-more p {
		color: #666;
		font-size: 14px;
		margin: 0 0 16px 0;
	}

	.u--border-radius {
		border-radius: 8px;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.row > div {
		padding: 0 15px;
	}

	@media (min-width: 992px) {
		.col-lg-5 { width: 41.666667%; }
		.col-lg-7 { width: 58.333333%; }
	}

	/* Visibility classes */
	.hidden-md.d-lg-none { display: block; }
	.d-none { display: none !important; }
	.d-lg-block { display: none; }
	.pb-2 { padding-bottom: 0.5rem; }

	@media (min-width: 992px) {
		.hidden-md.d-lg-none { display: none !important; }
		.d-lg-block { display: block !important; }
	}

	.u--background-color-white img {
		width: 100%;
		height: auto;
		display: block;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RIGHT SIDEBAR - WordPress EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.content-sidebar__section {
		background: #fff;
		border-radius: 8px;
		padding: 20px;
		margin-bottom: 20px;
	}

	.content-sidebar__heading {
		color: #333;
		font-size: 14px;
		font-weight: 700;
		text-transform: uppercase;
		margin: 0 0 16px 0;
		padding-bottom: 12px;
		border-bottom: 2px solid #0984ae;
	}

	.pssubject {
		font-size: 10px;
		color: #6b7280;
		text-transform: initial;
		font-weight: 400;
		margin-top: 8px;
	}

	/* Trading Room Schedule */
	.room-sched {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.schedule-item {
		padding-bottom: 12px;
		border-bottom: 1px solid #eee;
	}

	.schedule-item:last-child {
		border-bottom: none;
		padding-bottom: 0;
	}

	.trader-name {
		color: #0984ae;
		font-size: 14px;
		font-weight: 600;
		margin: 0 0 4px 0;
	}

	.trader-time {
		color: #6b7280;
		font-size: 12px;
	}

	/* Quick Links */
	.link-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.link-list li {
		margin-bottom: 8px;
	}

	.link-list a {
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.link-list a::before {
		content: '>';
		color: #0984ae;
	}

	.link-list a:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1279px) {
		.dashboard__content {
			flex-direction: column;
		}

		.dashboard__content-sidebar {
			flex: none;
			max-width: 100%;
			display: grid;
			grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
			gap: 20px;
		}

		.content-sidebar__section {
			margin-bottom: 0;
		}
	}

	@media (max-width: 767px) {
		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 26px;
		}

		.ultradingroom {
			display: none;
		}

		.dashboard__content {
			padding: 20px;
		}
	}

	@media (max-width: 575px) {
		.dashboard__header {
			flex-direction: column;
			align-items: flex-start;
		}

		.dashboard__header-left {
			flex-direction: column;
			align-items: flex-start;
			gap: 12px;
		}

		.dashboard__page-title {
			font-size: 22px;
		}

		.article-cards {
			margin: 0 -10px;
		}

		.flex-grid-item {
			padding: 0 10px;
		}
	}
</style>
