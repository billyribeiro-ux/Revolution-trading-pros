<!--
	URL: /dashboard/day-trading-room
	
	Day Trading Room Dashboard Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Member dashboard for Day Trading Room subscribers.
	Displays latest updates, videos, chatroom archives, and resources.
	
	Based on Mastering the Trade dashboard structure from MasterDash reference.
	
	Svelte 5 Features:
	- $state() for component state
	- $derived() for computed values
	- $effect() for reactive side effects
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';
	import CourseSecondaryNav from '$lib/components/dashboard/CourseSecondaryNav.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Dropdown state for trading room access
	let isDropdownOpen = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// SECONDARY NAVIGATION MENU ITEMS
	// ═══════════════════════════════════════════════════════════════════════════

	const secondaryNavItems = [
		{
			href: '/dashboard/day-trading-room',
			icon: 'layout-dashboard',
			text: 'Day Trading Room Dashboard',
			simplerIcon: 'st-icon-dashboard'
		},
		{
			href: '/dashboard/day-trading-room/daily-videos',
			icon: 'video',
			text: 'Premium Daily Videos'
		},
		{
			href: '/dashboard/day-trading-room/learning-center',
			icon: 'school',
			text: 'Learning Center'
		},
		{
			href: '/dashboard/day-trading-room/trading-room-archive',
			icon: 'archive',
			text: 'Trading Room Archives'
		},
		{
			href: '#',
			icon: 'users',
			text: 'Meet the Traders',
			submenu: [
				{ href: '/dashboard/day-trading-room/traders/lead-trader', icon: '', text: 'Lead Trader' },
				{ href: '/dashboard/day-trading-room/traders/senior-analyst', icon: '', text: 'Senior Analyst' },
				{ href: '/dashboard/day-trading-room/traders/head-moderator', icon: '', text: 'Head Moderator' }
			]
		},
		{
			href: '#',
			icon: 'shopping-cart',
			text: 'Trader Store',
			submenu: [
				{ href: '/dashboard/day-trading-room/store/indicators', icon: '', text: 'Indicators' },
				{ href: '/dashboard/day-trading-room/store/courses', icon: '', text: 'Advanced Courses' },
				{ href: '/dashboard/day-trading-room/store/tools', icon: '', text: 'Trading Tools' }
			]
		}
	];

	// Toggle dropdown
	function toggleDropdown() {
		isDropdownOpen = !isDropdownOpen;
	}

	// Close dropdown when clicking outside
	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (!target.closest('.dropdown')) {
			isDropdownOpen = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// MOCK DATA - Replace with API calls
	// ═══════════════════════════════════════════════════════════════════════════

	const latestUpdates = [
		{
			type: 'video',
			title: 'Market Analysis: SPX 0DTE Strategy',
			date: 'January 2, 2026',
			author: 'Lead Trader',
			excerpt: 'Deep dive into today\'s SPX levels and key gamma zones for optimal 0DTE entries.',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			url: '/dashboard/day-trading-room/video/market-analysis-spx-0dte-strategy'
		},
		{
			type: 'video',
			title: 'Trading Psychology: Managing Emotions',
			date: 'January 1, 2026',
			author: 'Trading Coach',
			excerpt: 'Learn how to stay disciplined during volatile market conditions and avoid revenge trading.',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			url: '#'
		},
		{
			type: 'archive',
			title: 'January 2, 2026 - Live Trading Session',
			date: 'January 2, 2026',
			author: 'Head Moderator',
			excerpt: 'Full recording of today\'s live trading session with real-time commentary.',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			url: '#'
		},
		{
			type: 'video',
			title: 'Advanced Order Flow Analysis',
			date: 'December 31, 2025',
			author: 'Senior Analyst',
			excerpt: 'Understanding institutional order flow and how to use it for better trade timing.',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			url: '#'
		},
		{
			type: 'archive',
			title: 'December 30, 2025 - Live Trading Session',
			date: 'December 30, 2025',
			author: 'Head Moderator',
			excerpt: 'Year-end trading session with key takeaways and 2026 outlook.',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			url: '#'
		},
		{
			type: 'video',
			title: 'Risk Management Essentials',
			date: 'December 29, 2025',
			author: 'Risk Manager',
			excerpt: 'Critical risk management techniques every day trader must master.',
			thumbnail: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			url: '#'
		}
	];

	const weeklyWatchlist = {
		title: 'Weekly Watchlist',
		week: 'Week of January 2, 2026',
		author: 'Market Analyst',
		description: 'Top setups and key levels to watch this week for optimal trading opportunities.',
		thumbnail: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
		url: '#'
	};

	const tradingSchedule = [
		{ day: 'Monday - Friday', time: '8:30 AM - 12:00 PM EST', session: 'Live Trading Session' },
		{ day: 'Monday - Friday', time: '12:00 PM - 1:00 PM EST', session: 'Market Recap & Q&A' },
		{ day: 'Daily', time: '3:00 PM EST', session: 'Daily Video Analysis' }
	];
</script>

<svelte:head>
	<title>Day Trading Room Dashboard | Revolution Trading Pros</title>
	<meta name="description" content="Access your Day Trading Room dashboard with live sessions, market analysis, and educational resources." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<svelte:window onclick={handleClickOutside} />

<!-- Secondary Navigation Panel (appears when sidebar is collapsed) -->
<CourseSecondaryNav
	courseName="Day Trading Room"
	menuItems={secondaryNavItems}
/>

<!-- ═══════════════════════════════════════════════════════════════════════════
	 DASHBOARD HEADER
	 ═══════════════════════════════════════════════════════════════════════════ -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Day Trading Room Dashboard</h1>
		<a href="/dashboard/day-trading-room/start-here" class="btn btn-xs btn-default">
			New? Start Here
		</a>
	</div>

	<div class="dashboard__header-right">
		<ul class="ultradingroom" style="text-align: right;list-style: none;">
			<li class="litradingroom">
				<a href="/trading-room-rules.pdf" target="_blank" class="btn btn-xs btn-link" style="font-weight: 700 !important;">Trading Room Rules</a>
			</li>
			<li style="font-size: 11px;" class="btn btn-xs btn-link litradingroomhind">
				By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
			</li>
		</ul>

		<!-- Enter Trading Room Dropdown -->
		<div class="dropdown" class:is-open={isDropdownOpen}>
			<button
				type="button"
				class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle"
				onclick={toggleDropdown}
				aria-expanded={isDropdownOpen}
			>
				<strong>Enter a Trading Room</strong>
				<span class="dropdown-arrow">▼</span>
			</button>

			{#if isDropdownOpen}
				<nav class="dropdown-menu dropdown-menu--full-width">
					<ul class="dropdown-menu__menu">
						<li>
							<a href="/live-trading-rooms/day-trading" target="_blank">
								<RtpIcon name="chart-line" size={20} />
								Day Trading Room
							</a>
						</li>
						<li>
							<a href="/live-trading-rooms/swing-trading" target="_blank">
								<RtpIcon name="chart-bar" size={20} />
								Swing Trading Room
							</a>
						</li>
						<li>
							<a href="/dashboard" target="_blank">
								<RtpIcon name="graduation-cap" size={20} />
								Small Account Mentorship
							</a>
						</li>
					</ul>
				</nav>
			{/if}
		</div>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
	 DASHBOARD CONTENT
	 ═══════════════════════════════════════════════════════════════════════════ -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		
		<!-- Welcome Video Section -->
		<section class="dashboard__content-section-member">
			<div class="welcome-video">
				<video 
					controls 
					width="100%" 
					poster="https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg"
					style="aspect-ratio: 2 / 1; border-radius: 8px;"
				>
					<source src="https://simpler-options.s3.amazonaws.com/tutorials/MTT_tutorial2025.mp4" type="video/mp4">
					Your browser does not support the video tag.
				</video>
			</div>
		</section>

		<!-- Latest Updates Section -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Latest Updates</h2>
			<div class="article-cards row flex-grid">
				{#each latestUpdates as update}
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
						<figure 
							class="article-card__image" 
							style="background-image: url({update.thumbnail});"
						>
							<img src={update.thumbnail} alt={update.title} />
						</figure>
						
						<div class="article-card__type">
							<span class="label label--info">
								{update.type === 'video' ? 'Daily Video' : 'Trading Archive'}
							</span>
						</div>
						
						<h4 class="h5 article-card__title">
							<a href={update.url}>{update.title}</a>
						</h4>
						
						<span class="article-card__meta">
							<small>{update.date} with {update.author}</small>
						</span>
						
						<div class="article-card__excerpt">
							<p>{update.excerpt}</p>
						</div>
						
						<a href={update.url} class="btn btn-tiny btn-default">Watch Now</a>
					</article>
				</div>
				{/each}
			</div>
		</section>

		<!-- Weekly Watchlist Section -->
		<div class="dashboard__content-section u--background-color-white">
			<section>
				<div class="row">
					<div class="col-sm-6 col-lg-5">
						<h2 class="section-title-alt section-title-alt--underline">{weeklyWatchlist.title}</h2>
						<div class="hidden-md d-lg-none pb-2">
							<a href={weeklyWatchlist.url}>
								<img src={weeklyWatchlist.thumbnail} alt="Weekly Watchlist" class="u--border-radius">
							</a>
						</div>
						<h4 class="h5 u--font-weight-bold">{weeklyWatchlist.week} with {weeklyWatchlist.author}</h4>
						<div class="u--hide-read-more">
							<p>{weeklyWatchlist.description}</p>
						</div>
						<a href={weeklyWatchlist.url} class="btn btn-tiny btn-default">Watch Now</a>
					</div>
					<div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
						<a href={weeklyWatchlist.url}>
							<img src={weeklyWatchlist.thumbnail} alt="Weekly Watchlist" class="u--border-radius">
						</a>
					</div>
				</div>
			</section>
		</div>
	</div>

	<!-- Sidebar -->
	<aside class="dashboard__content-sidebar">
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Trading Room Schedule
				<p class="pssubject" style="font-size: 10px;margin-top: 15px;text-transform: initial;">Schedule is subject to change.</p>
			</h4>
			<div class="schedule-list">
				{#each tradingSchedule as schedule}
					<div class="schedule-item">
						<div class="schedule-day">{schedule.day}</div>
						<div class="schedule-time">{schedule.time}</div>
						<div class="schedule-session">{schedule.session}</div>
					</div>
				{/each}
			</div>
		</section>

		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Quick Links</h4>
			<ul class="link-list">
				<li><a href="/support" target="_blank">Support</a></li>
				<li><a href="/tutorials" target="_blank">Platform Tutorials</a></li>
				<li><a href="/blog" target="_blank">Simpler Blog</a></li>
			</ul>
		</section>
	</aside>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DAY TRADING ROOM DASHBOARD STYLES
	 * Based on MasterDash reference implementation
	 * Apple ICT 11+ Standards
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Welcome Video Section */
	.dashboard__content-section-member {
		margin-bottom: 40px;
	}

	.welcome-video {
		background: #fff;
		border-radius: 8px;
		padding: 20px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	/* Article Cards Grid */
	.article-cards {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 24px;
		margin-top: 24px;
	}

	.article-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition: transform 0.2s ease, box-shadow 0.2s ease;
		display: flex;
		flex-direction: column;
	}

	.article-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
	}

	.article-card__image {
		width: 100%;
		height: 180px;
		background-size: cover;
		background-position: center;
		position: relative;
	}

	.article-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.article-card__type {
		position: absolute;
		top: 12px;
		left: 12px;
		z-index: 1;
	}

	.label {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.label--info {
		background-color: #143E59;
		color: #fff;
	}

	.article-card__title {
		padding: 16px 16px 8px;
		margin: 0;
		font-size: 18px;
		font-weight: 800;
		line-height: 1.3;
	}

	.article-card__title a {
		color: #191717;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.article-card__title a:hover {
		color: #143E59;
	}

	.article-card__meta {
		padding: 0 16px;
		color: #666;
		font-size: 12px;
	}

	.article-card__excerpt {
		padding: 12px 16px;
		color: #555;
		font-size: 14px;
		line-height: 1.5;
		flex-grow: 1;
	}

	.article-card__excerpt p {
		margin: 0;
	}

	.article-card .btn {
		margin: 0 16px 16px;
	}

	/* Schedule Section - pssubject class handled by inline styles */

	.schedule-list {
		margin-top: 16px;
	}

	.schedule-item {
		padding: 12px;
		border-bottom: 1px solid #ededed;
		font-size: 13px;
	}

	.schedule-item:last-child {
		border-bottom: none;
	}

	.schedule-day {
		font-weight: 700;
		color: #191717;
		margin-bottom: 4px;
	}

	.schedule-time {
		color: #143E59;
		font-weight: 600;
		margin-bottom: 4px;
	}

	.schedule-session {
		color: #666;
		font-size: 12px;
	}

	/* Sidebar Links - Removed (using global .link-list from app.css) */

	/* Responsive Design */
	@media (max-width: 768px) {
		.article-cards {
			grid-template-columns: 1fr;
		}

		.dashboard__header {
			flex-direction: column;
			align-items: flex-start;
			gap: 16px;
		}

		.dashboard__header-right {
			width: 100%;
			flex-direction: column;
			align-items: stretch;
		}
	}
</style>
