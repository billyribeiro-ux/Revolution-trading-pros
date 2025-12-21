<script lang="ts">
	/**
	 * Membership Dashboard Page - Simpler Trading EXACT Clone
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Exact match of Simpler Trading's "Mastering the Trade Dashboard" page:
	 * - Breadcrumb navigation
	 * - Header with title, "New? Start Here" badge, and "Enter Trading Room" button
	 * - Trading Room Rules link
	 * - Featured video player
	 * - Trading Room Schedule sidebar
	 * - Quick Links sidebar
	 * - Latest Updates video cards
	 * - Weekly Watchlist section
	 *
	 * @version 2.0.0 (December 2025)
	 */

	import { page } from '$app/stores';

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
		tradingRoomUrl: string;
	}

	const membershipData = $derived.by((): MembershipDetail => {
		const memberships: Record<string, MembershipDetail> = {
			'mastering-the-trade': {
				name: 'Mastering the Trade',
				slug: 'mastering-the-trade',
				tradingRoomUrl: '/trading-room/mastering-the-trade'
			},
			'simpler-showcase': {
				name: 'Simpler Showcase',
				slug: 'simpler-showcase',
				tradingRoomUrl: '/trading-room/simpler-showcase'
			},
			'mm': {
				name: 'Moxie Indicator™ Mastery',
				slug: 'mm',
				tradingRoomUrl: '/trading-room/moxie-mastery'
			}
		};

		return memberships[slug] || {
			name: slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' '),
			slug,
			tradingRoomUrl: `/trading-room/${slug}`
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// TRADING ROOM SCHEDULE
	// ═══════════════════════════════════════════════════════════════════════════

	interface ScheduleItem {
		traderName: string;
		dateTime: string;
	}

	const schedule: ScheduleItem[] = [
		{ traderName: 'Taylor Horton', dateTime: 'Dec 22, 2025, 9:20 AM EST' },
		{ traderName: 'Sam Shames', dateTime: 'Dec 22, 2025, 10:30 AM EST' },
		{ traderName: 'Neil Yeager', dateTime: 'Dec 22, 2025, 11:30 AM EST' },
		{ traderName: 'Bruce Marshall', dateTime: 'Dec 22, 2025, 2:00 PM EST' },
		{ traderName: 'Henry Gambell', dateTime: 'Dec 22, 2025, 3:00 PM EST' },
		{ traderName: 'Henry Gambell', dateTime: 'Dec 23, 2025, 9:15 AM EST' },
		{ traderName: 'Raghee Horner', dateTime: 'Dec 23, 2025, 10:30 AM EST' },
		{ traderName: 'David Starr', dateTime: 'Dec 23, 2025, 11:30 AM EST' },
		{ traderName: 'Heather Vanek', dateTime: 'Dec 23, 2025, 2:00 PM EST' },
		{ traderName: 'Danielle Shay', dateTime: 'Dec 23, 2025, 3:00 PM EST' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// QUICK LINKS
	// ═══════════════════════════════════════════════════════════════════════════

	const quickLinks = [
		{ title: 'Support', href: 'https://intercom.help/simpler-trading/en/' },
		{ title: 'Platform Tutorials', href: '/tutorials' },
		{ title: 'Simpler Blog', href: '/blog' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// LATEST UPDATES (Videos)
	// ═══════════════════════════════════════════════════════════════════════════

	interface VideoUpdate {
		id: number;
		type: 'daily-video' | 'weekly-watchlist';
		typeLabel: string;
		title: string;
		description: string;
		date: string;
		traderName: string;
		thumbnail: string;
	}

	const latestUpdates: VideoUpdate[] = [
		{
			id: 1,
			type: 'daily-video',
			typeLabel: 'DAILY VIDEO',
			title: 'Holiday Weekend Market Review',
			description: 'Indexes continue to churn sideways as we approach next week\'s holiday trade. Bulls usually take over in low volume. Can they do it again?',
			date: 'December 19, 2025 with Sam',
			traderName: 'Sam Shames',
			thumbnail: 'https://cdn.simplertrading.com/images/traders/sam-shames.jpg'
		},
		{
			id: 2,
			type: 'daily-video',
			typeLabel: 'DAILY VIDEO',
			title: 'December 19, 2025',
			description: 'With Henry Gambell',
			date: 'December 19, 2025',
			traderName: 'Henry Gambell',
			thumbnail: 'https://cdn.simplertrading.com/images/traders/henry-gambell.jpg'
		},
		{
			id: 3,
			type: 'daily-video',
			typeLabel: 'DAILY VIDEO',
			title: 'Ho Ho Whoa!',
			description: 'In this video, Bruce discusses today\'s market action and the outlook for the end of the year. After CPI this morning, we got a nice bounce and relief rally after a long and messy chop phase since Thanksgiving.',
			date: 'December 18, 2025 with Bruce Marshall',
			traderName: 'Bruce Marshall',
			thumbnail: 'https://cdn.simplertrading.com/images/traders/bruce-marshall.jpg'
		},
		{
			id: 4,
			type: 'daily-video',
			typeLabel: 'DAILY VIDEO',
			title: 'December 18, 2025',
			description: 'With Henry Gambell',
			date: 'December 18, 2025',
			traderName: 'Henry Gambell',
			thumbnail: 'https://cdn.simplertrading.com/images/traders/henry-gambell.jpg'
		},
		{
			id: 5,
			type: 'daily-video',
			typeLabel: 'DAILY VIDEO',
			title: 'A Moment For The VIX',
			description: 'Today\'s action in stocks wasn\'t just out of the blue. We\'d been seeing weakness across the board, but it really came through today after the VIX expiration.',
			date: 'December 17, 2025 with HG',
			traderName: 'Henry Gambell',
			thumbnail: 'https://cdn.simplertrading.com/images/traders/henry-gambell.jpg'
		},
		{
			id: 6,
			type: 'daily-video',
			typeLabel: 'DAILY VIDEO',
			title: 'December 17, 2025',
			description: 'With John F. Carter',
			date: 'December 17, 2025',
			traderName: 'John Carter',
			thumbnail: 'https://cdn.simplertrading.com/images/traders/john-carter.jpg'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// WEEKLY WATCHLIST
	// ═══════════════════════════════════════════════════════════════════════════

	const weeklyWatchlist = {
		title: 'Weekly Watchlist with Allison Ostrander',
		date: 'Week of December 15, 2025',
		traderName: 'Allison Ostrander',
		traderTitle: 'Director of Risk Tolerance',
		thumbnail: 'https://cdn.simplertrading.com/images/traders/allison-ostrander.jpg'
	};
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>{membershipData.name} Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     BREADCRUMB - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<nav class="breadcrumb" aria-label="Breadcrumb">
	<ol>
		<li><a href="/">Home</a></li>
		<li><a href="/dashboard">Member Dashboard</a></li>
		<li><span>{membershipData.name} Dashboard</span></li>
	</ol>
</nav>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard-header">
	<div class="dashboard-header__left">
		<h1 class="dashboard-header__title">{membershipData.name} Dashboard</h1>
		<a href="/dashboard/{slug}/getting-started" class="new-badge">New? Start Here</a>
	</div>
	<div class="dashboard-header__right">
		<a href={membershipData.tradingRoomUrl} class="btn-enter-room" target="_blank" rel="nofollow">
			Enter a Trading Room ›
		</a>
		<div class="trading-rules">
			<a href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf" target="_blank" class="trading-rules__link">
				Trading Room Rules
			</a>
			<p class="trading-rules__text">
				By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
			</p>
		</div>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT WITH SIDEBAR - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard-content">
	<div class="dashboard-content__main">
		<!-- Featured Video Player -->
		<section class="featured-video">
			<div class="video-wrapper">
				<div class="video-placeholder">
					<div class="video-branding">
						<span class="brand-icon">◎</span>
						<span class="brand-text">SIMPLER</span><span class="brand-text-bold">TRADING</span><span class="brand-tm">®</span>
					</div>
					<h2 class="video-title">Welcome to<br/>Simpler Trading!</h2>
				</div>
				<div class="video-controls">
					<button class="play-btn" aria-label="Play video">▶</button>
					<span class="video-time">0:00 / 5:32</span>
					<div class="video-controls-right">
						<button aria-label="Previous">⏮</button>
						<button aria-label="Next">⏭</button>
						<button aria-label="Fullscreen">⛶</button>
						<button aria-label="More">⋮</button>
					</div>
				</div>
			</div>
		</section>

		<!-- Latest Updates Section -->
		<section class="latest-updates">
			<h2 class="section-title">Latest Updates</h2>
			<div class="updates-grid">
				{#each latestUpdates as video (video.id)}
					<article class="video-card">
						<div class="video-card__thumbnail">
							<div class="video-card__badge">{membershipData.name.toUpperCase()}</div>
							<div class="video-card__overlay">
								<div class="video-card__branding">
									<span class="brand-icon-sm">◎</span>
									<span>SIMPLER</span><span class="bold">TRADING</span><span class="tm">®</span>
								</div>
								{#if video.traderName}
									<div class="video-card__trader">
										<span class="trader-label">SIMPLER<span class="highlight">TRADING</span></span>
										<span class="trader-name">{video.traderName.toUpperCase()}</span>
										{#if video.id === 1}
											<span class="trader-title">Vice President of Options</span>
										{:else if video.id === 3}
											<span class="trader-title">Senior Director of Options & Income Trading</span>
										{/if}
									</div>
								{/if}
							</div>
						</div>
						<div class="video-card__content">
							<span class="video-card__type">{video.typeLabel}</span>
							<h3 class="video-card__title">{video.title}</h3>
							<p class="video-card__date">{video.date}</p>
							{#if video.description && video.id !== 2 && video.id !== 4 && video.id !== 6}
								<p class="video-card__desc">{video.description}</p>
							{/if}
							<a href="/dashboard/{slug}/videos/{video.id}" class="video-card__link">Watch Now</a>
						</div>
					</article>
				{/each}
			</div>
		</section>

		<!-- Weekly Watchlist Section -->
		<section class="weekly-watchlist">
			<div class="watchlist-content">
				<span class="watchlist-label">WEEKLY WATCHLIST</span>
				<h3 class="watchlist-title">{weeklyWatchlist.title}</h3>
				<p class="watchlist-date">{weeklyWatchlist.date}</p>
				<a href="/dashboard/ww" class="watchlist-link">Watch Now</a>
			</div>
			<div class="watchlist-image">
				<div class="watchlist-badge">WEEKLY WATCHLIST</div>
				<div class="watchlist-trader">
					<div class="watchlist-branding">
						<span class="brand-icon-sm">◎</span>
						<span>SIMPLER</span><span class="bold">TRADING</span><span class="tm">®</span>
					</div>
					<span class="watchlist-trader-name">{weeklyWatchlist.traderName.toUpperCase()}</span>
					<span class="watchlist-trader-title">{weeklyWatchlist.traderTitle}</span>
				</div>
			</div>
		</section>
	</div>

	<!-- Right Sidebar -->
	<aside class="dashboard-sidebar">
		<!-- Trading Room Schedule -->
		<section class="sidebar-section schedule-section">
			<h4 class="sidebar-title">TRADING ROOM SCHEDULE</h4>
			<p class="schedule-note">Schedule is subject to change.</p>
			<ul class="schedule-list">
				{#each schedule as item}
					<li class="schedule-item">
						<a href="/traders/{item.traderName.toLowerCase().replace(' ', '-')}" class="schedule-name">{item.traderName}</a>
						<span class="schedule-time">{item.dateTime}</span>
					</li>
				{/each}
			</ul>
		</section>

		<!-- Quick Links -->
		<section class="sidebar-section">
			<h4 class="sidebar-title">QUICK LINKS</h4>
			<ul class="quick-links">
				{#each quickLinks as link}
					<li>
						<span class="link-arrow">›</span>
						<a href={link.href} target="_blank">{link.title}</a>
					</li>
				{/each}
			</ul>
		</section>
	</aside>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   BREADCRUMB
	   ═══════════════════════════════════════════════════════════════════════════ */

	.breadcrumb {
		background: #fff;
		padding: 12px 30px;
		border-bottom: 1px solid #e5e7eb;
		font-size: 12px;
	}

	.breadcrumb ol {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.breadcrumb li {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.breadcrumb li:not(:last-child)::after {
		content: '/';
		color: #9ca3af;
	}

	.breadcrumb a {
		color: #6b7280;
		text-decoration: none;
	}

	.breadcrumb a:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.breadcrumb span {
		color: #374151;
		font-weight: 500;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-header {
		background: #fff;
		padding: 20px 30px;
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		flex-wrap: wrap;
		gap: 16px;
		border-bottom: 1px solid #e5e7eb;
	}

	.dashboard-header__left {
		display: flex;
		align-items: center;
		gap: 16px;
		flex-wrap: wrap;
	}

	.dashboard-header__title {
		font-family: 'Open Sans Condensed', 'Arial Narrow', sans-serif;
		font-size: 28px;
		font-weight: 700;
		color: #1f2937;
		margin: 0;
	}

	.new-badge {
		display: inline-block;
		padding: 6px 12px;
		background: #e5e7eb;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
		color: #374151;
		text-decoration: none;
	}

	.new-badge:hover {
		background: #d1d5db;
	}

	.dashboard-header__right {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 12px;
	}

	.btn-enter-room {
		display: inline-block;
		padding: 12px 24px;
		background: #f99e31;
		color: #fff;
		font-size: 14px;
		font-weight: 700;
		text-decoration: none;
		border-radius: 4px;
		transition: background 0.15s ease;
	}

	.btn-enter-room:hover {
		background: #e8890f;
	}

	.trading-rules {
		text-align: right;
		max-width: 200px;
	}

	.trading-rules__link {
		display: block;
		color: #0984ae;
		font-size: 14px;
		font-weight: 700;
		text-decoration: none;
		margin-bottom: 4px;
	}

	.trading-rules__link:hover {
		text-decoration: underline;
	}

	.trading-rules__text {
		font-size: 10px;
		color: #6b7280;
		line-height: 1.4;
		margin: 0;
		font-style: italic;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-content {
		display: flex;
		gap: 30px;
		padding: 30px;
		background: #f4f4f4;
	}

	.dashboard-content__main {
		flex: 1;
		min-width: 0;
	}

	.dashboard-sidebar {
		width: 280px;
		flex-shrink: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FEATURED VIDEO
	   ═══════════════════════════════════════════════════════════════════════════ */

	.featured-video {
		margin-bottom: 30px;
	}

	.video-wrapper {
		background: #0a1628;
		border-radius: 8px;
		overflow: hidden;
		aspect-ratio: 16 / 9;
		position: relative;
	}

	.video-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: calc(100% - 40px);
		color: #fff;
		text-align: center;
		padding: 40px;
		background: linear-gradient(135deg, #0a1628 0%, #1a365d 100%);
	}

	.video-branding {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 14px;
		margin-bottom: 20px;
		color: rgba(255, 255, 255, 0.9);
	}

	.brand-icon {
		font-size: 18px;
		color: #0984ae;
	}

	.brand-text {
		font-weight: 300;
	}

	.brand-text-bold {
		font-weight: 700;
	}

	.brand-tm {
		font-size: 10px;
		vertical-align: super;
	}

	.video-title {
		font-family: 'Open Sans Condensed', 'Arial Narrow', sans-serif;
		font-size: 42px;
		font-weight: 300;
		line-height: 1.2;
		margin: 0;
	}

	.video-controls {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 8px 16px;
		background: #1a1a1a;
		color: #fff;
		font-size: 12px;
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
	}

	.play-btn {
		background: none;
		border: none;
		color: #fff;
		font-size: 16px;
		cursor: pointer;
		padding: 4px 8px;
	}

	.video-time {
		color: #9ca3af;
	}

	.video-controls-right {
		margin-left: auto;
		display: flex;
		gap: 8px;
	}

	.video-controls-right button {
		background: none;
		border: none;
		color: #9ca3af;
		font-size: 14px;
		cursor: pointer;
		padding: 4px;
	}

	.video-controls-right button:hover {
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LATEST UPDATES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.latest-updates {
		margin-bottom: 30px;
	}

	.section-title {
		font-family: 'Open Sans Condensed', 'Arial Narrow', sans-serif;
		font-size: 24px;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 20px;
	}

	.updates-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 20px;
	}

	.video-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.video-card__thumbnail {
		position: relative;
		aspect-ratio: 16 / 10;
		background: linear-gradient(135deg, #0a1628 0%, #1a365d 100%);
		overflow: hidden;
	}

	.video-card__badge {
		position: absolute;
		top: 10px;
		left: 10px;
		padding: 4px 8px;
		background: #0984ae;
		color: #fff;
		font-size: 9px;
		font-weight: 700;
		border-radius: 3px;
		letter-spacing: 0.5px;
		z-index: 2;
	}

	.video-card__overlay {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 40px 15px 15px;
	}

	.video-card__branding {
		display: flex;
		align-items: center;
		gap: 3px;
		font-size: 11px;
		color: rgba(255, 255, 255, 0.8);
	}

	.brand-icon-sm {
		font-size: 14px;
		color: #0984ae;
	}

	.video-card__branding .bold {
		font-weight: 700;
	}

	.video-card__branding .tm {
		font-size: 8px;
		vertical-align: super;
	}

	.video-card__trader {
		display: flex;
		flex-direction: column;
		color: #fff;
	}

	.trader-label {
		font-size: 10px;
		color: rgba(255, 255, 255, 0.7);
		letter-spacing: 1px;
	}

	.trader-label .highlight {
		color: #0984ae;
	}

	.trader-name {
		font-family: 'Open Sans Condensed', 'Arial Narrow', sans-serif;
		font-size: 22px;
		font-weight: 700;
		line-height: 1.1;
		margin-top: 4px;
	}

	.trader-title {
		font-size: 9px;
		color: rgba(255, 255, 255, 0.7);
		margin-top: 2px;
	}

	.video-card__content {
		padding: 16px;
	}

	.video-card__type {
		display: inline-block;
		font-size: 10px;
		font-weight: 700;
		color: #dc2626;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 8px;
	}

	.video-card__title {
		font-size: 16px;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 6px;
		line-height: 1.3;
	}

	.video-card__date {
		font-size: 13px;
		color: #6b7280;
		margin: 0 0 8px;
	}

	.video-card__desc {
		font-size: 13px;
		color: #6b7280;
		line-height: 1.5;
		margin: 0 0 12px;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.video-card__link {
		display: inline-block;
		color: #0984ae;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
	}

	.video-card__link:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   WEEKLY WATCHLIST
	   ═══════════════════════════════════════════════════════════════════════════ */

	.weekly-watchlist {
		display: flex;
		gap: 30px;
		background: #fff;
		border-radius: 8px;
		padding: 30px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.watchlist-content {
		flex: 1;
	}

	.watchlist-label {
		display: inline-block;
		font-size: 11px;
		font-weight: 700;
		color: #dc2626;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 8px;
	}

	.watchlist-title {
		font-size: 20px;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 6px;
	}

	.watchlist-date {
		font-size: 14px;
		color: #6b7280;
		margin: 0 0 16px;
	}

	.watchlist-link {
		display: inline-block;
		color: #0984ae;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
	}

	.watchlist-link:hover {
		text-decoration: underline;
	}

	.watchlist-image {
		width: 300px;
		aspect-ratio: 16 / 10;
		background: linear-gradient(135deg, #0a1628 0%, #1a365d 100%);
		border-radius: 8px;
		position: relative;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		justify-content: flex-end;
		padding: 20px;
	}

	.watchlist-badge {
		position: absolute;
		top: 15px;
		right: 15px;
		padding: 6px 12px;
		background: #10b981;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		border-radius: 4px;
		letter-spacing: 0.5px;
	}

	.watchlist-trader {
		display: flex;
		flex-direction: column;
		color: #fff;
	}

	.watchlist-branding {
		display: flex;
		align-items: center;
		gap: 3px;
		font-size: 10px;
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 8px;
	}

	.watchlist-branding .bold {
		font-weight: 700;
	}

	.watchlist-branding .tm {
		font-size: 7px;
		vertical-align: super;
	}

	.watchlist-trader-name {
		font-family: 'Open Sans Condensed', 'Arial Narrow', sans-serif;
		font-size: 24px;
		font-weight: 700;
		line-height: 1.1;
	}

	.watchlist-trader-title {
		font-size: 11px;
		color: rgba(255, 255, 255, 0.7);
		margin-top: 4px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.sidebar-section {
		background: #fff;
		border-radius: 8px;
		padding: 20px;
		margin-bottom: 20px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.sidebar-title {
		font-size: 14px;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 16px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.schedule-note {
		font-size: 11px;
		color: #9ca3af;
		font-style: italic;
		margin: 0 0 16px;
	}

	.schedule-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.schedule-item {
		display: flex;
		flex-direction: column;
		padding: 10px 0;
		border-bottom: 1px solid #f3f4f6;
	}

	.schedule-item:last-child {
		border-bottom: none;
	}

	.schedule-name {
		font-size: 14px;
		font-weight: 600;
		color: #0984ae;
		text-decoration: none;
	}

	.schedule-name:hover {
		text-decoration: underline;
	}

	.schedule-time {
		font-size: 12px;
		color: #6b7280;
		margin-top: 2px;
	}

	.quick-links {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.quick-links li {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 0;
		border-bottom: 1px solid #f3f4f6;
	}

	.quick-links li:last-child {
		border-bottom: none;
	}

	.link-arrow {
		color: #0984ae;
		font-weight: 700;
		font-size: 14px;
	}

	.quick-links a {
		color: #0984ae;
		font-size: 14px;
		text-decoration: none;
	}

	.quick-links a:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1200px) {
		.updates-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 992px) {
		.dashboard-content {
			flex-direction: column;
		}

		.dashboard-sidebar {
			width: 100%;
		}

		.weekly-watchlist {
			flex-direction: column;
		}

		.watchlist-image {
			width: 100%;
		}
	}

	@media (max-width: 768px) {
		.breadcrumb,
		.dashboard-header,
		.dashboard-content {
			padding-left: 16px;
			padding-right: 16px;
		}

		.dashboard-header__title {
			font-size: 22px;
		}

		.updates-grid {
			grid-template-columns: 1fr;
		}

		.video-title {
			font-size: 28px;
		}
	}
</style>
