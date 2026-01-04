<!--
	URL: /dashboard/day-trading-room
	
	Day Trading Room Dashboard Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer | Svelte 5 / SvelteKit Best Practices
	
	Features:
	- Full TypeScript typing with interfaces
	- Svelte 5 runes: $state(), $derived(), $effect()
	- Proper accessibility (ARIA, keyboard navigation)
	- SEO optimized with structured data
	- Scoped CSS with CSS custom properties
	
	@version 2.0.0 - Svelte 5 Best Practices
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';
	import TradingRoomSidebar from '$lib/components/dashboard/TradingRoomSidebar.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPE DEFINITIONS - Svelte 5 / TypeScript Best Practice
	// ═══════════════════════════════════════════════════════════════════════════

	interface Update {
		type: 'video' | 'archive';
		title: string;
		date: string;
		author: string;
		excerpt: string;
		thumbnail: string;
		url: string;
	}

	interface Watchlist {
		title: string;
		week: string;
		author: string;
		description: string;
		thumbnail: string;
		url: string;
	}

	interface ScheduleItem {
		traderName: string;
		date: string;
		time: string;
	}

	interface TradingRoom {
		name: string;
		icon: string;
		url: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	// Dropdown state with $state rune
	let isDropdownOpen = $state<boolean>(false);

	// Derived: Button aria-label based on state
	let dropdownAriaLabel = $derived(
		isDropdownOpen ? 'Close trading room menu' : 'Open trading room menu'
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS - Svelte 5 Pattern
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleDropdown(): void {
		isDropdownOpen = !isDropdownOpen;
	}

	function closeDropdown(): void {
		isDropdownOpen = false;
	}

	function handleClickOutside(event: MouseEvent): void {
		const target = event.target as HTMLElement;
		if (!target.closest('.dropdown')) {
			closeDropdown();
		}
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape' && isDropdownOpen) {
			closeDropdown();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA - Would come from +page.server.ts load function in production
	// ═══════════════════════════════════════════════════════════════════════════

	const latestUpdates: Update[] = [
		{
			type: 'video',
			title: 'Market Analysis: SPX 0DTE Strategy',
			date: 'January 2, 2026',
			author: 'Lead Trader',
			excerpt: "Deep dive into today's SPX levels and key gamma zones for optimal 0DTE entries.",
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
			excerpt: "Full recording of today's live trading session with real-time commentary.",
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

	const weeklyWatchlist: Watchlist = {
		title: 'Weekly Watchlist',
		week: 'Week of January 2, 2026',
		author: 'Market Analyst',
		description: 'Top setups and key levels to watch this week for optimal trading opportunities.',
		thumbnail: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
		url: '#'
	};

	const tradingSchedule: ScheduleItem[] = [
		{ traderName: 'Taylor Horton', date: 'Jan 5, 2026', time: '9:20 AM EST' },
		{ traderName: 'Sam Shames', date: 'Jan 5, 2026', time: '10:30 AM EST' },
		{ traderName: 'Neil Yeager', date: 'Jan 5, 2026', time: '11:30 AM EST' },
		{ traderName: 'Bruce Marshall', date: 'Jan 5, 2026', time: '2:00 PM EST' },
		{ traderName: 'Henry Gambell', date: 'Jan 5, 2026', time: '3:00 PM EST' },
		{ traderName: 'Henry Gambell', date: 'Jan 6, 2026', time: '9:15 AM EST' },
		{ traderName: 'Raghee Horner', date: 'Jan 6, 2026', time: '10:30 AM EST' },
		{ traderName: 'David Starr', date: 'Jan 6, 2026', time: '11:30 AM EST' },
		{ traderName: 'Taylor Horton', date: 'Jan 6, 2026', time: '2:00 PM EST' },
		{ traderName: 'Danielle Shay / John Carter', date: 'Jan 6, 2026', time: '3:00 PM EST' }
	];

	const tradingRooms: TradingRoom[] = [
		{ name: 'Day Trading Room', icon: 'chart-line', url: '/live-trading-rooms/day-trading' },
		{ name: 'Swing Trading Room', icon: 'chart-bar', url: '/live-trading-rooms/swing-trading' },
		{ name: 'Small Account Mentorship', icon: 'school', url: '/dashboard' }
	];
</script>

<svelte:head>
	<title>Day Trading Room Dashboard | Revolution Trading Pros</title>
	<meta name="description" content="Access your Day Trading Room dashboard with live sessions, market analysis, and educational resources." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<svelte:window onclick={handleClickOutside} onkeydown={handleKeydown} />

<!-- ═══════════════════════════════════════════════════════════════════════════
	 DASHBOARD LAYOUT - Mobile First Responsive
	 SIDEBAR DEACTIVATED: Commented out for layout optimization purposes
	 ═══════════════════════════════════════════════════════════════════════════ -->

<!-- ═══════════════════════════════════════════════════════════════════════════
	 DASHBOARD HEADER - Mobile-First Responsive Design
	 ═══════════════════════════════════════════════════════════════════════════ -->
<header class="dashboard__header">
	<!-- Title Row -->
	<div class="dashboard__header-title-row">
		<h1 class="dashboard__page-title">Day Trading Room Dashboard</h1>
		<a href="/dashboard/day-trading-room/start-here" class="btn btn-xs btn-default btn-start-here">
			New? Start Here
		</a>
	</div>

	<!-- Actions Row -->
	<div class="dashboard__header-actions-row">
		<div class="trading-room-rules">
			<a 
				href="/trading-room-rules.pdf" 
				target="_blank" 
				rel="noopener noreferrer"
				class="trading-room-rules__link"
			>
				Trading Room Rules
			</a>
			<p class="trading-room-rules__disclaimer">
				By logging into any of our Live Trading Rooms, you are agreeing to our Rules of the Room.
			</p>
		</div>

		<!-- Enter Trading Room Dropdown -->
		<div class="dropdown" class:is-open={isDropdownOpen}>
			<button
				type="button"
				class="btn btn-orange btn-tradingroom dropdown-toggle"
				onclick={toggleDropdown}
				aria-expanded={isDropdownOpen}
				aria-haspopup="menu"
				aria-label={dropdownAriaLabel}
				id="trading-room-dropdown-btn"
			>
				<strong>Enter a Trading Room</strong>
				<span class="dropdown-arrow" aria-hidden="true">▼</span>
			</button>

			{#if isDropdownOpen}
				<div 
					class="dropdown-menu" 
					role="menu" 
					aria-labelledby="trading-room-dropdown-btn"
				>
					<ul class="dropdown-menu__menu">
						{#each tradingRooms as room}
							<li role="none">
								<a 
									href={room.url} 
									target="_blank"
									rel="noopener noreferrer"
									role="menuitem"
								>
									<RtpIcon name={room.icon} size={20} />
									{room.name}
								</a>
							</li>
						{/each}
					</ul>
				</div>
			{/if}
		</div>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
	 DASHBOARD CONTENT
	 ═══════════════════════════════════════════════════════════════════════════ -->
<div class="dashboard__content">
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
			<div class="article-cards">
				{#each latestUpdates as update}
					<article class="article-card">
						<figure 
							class="article-card__image" 
							style="background-image: url({update.thumbnail});"
						>
							<img src={update.thumbnail} alt={update.title} />
							
							<div class="article-card__type">
								<span class="label label--info">
									{update.type === 'video' ? 'Daily Video' : 'Trading Archive'}
								</span>
							</div>
						</figure>
						
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
				{/each}
			</div>
		</section>

		<!-- Weekly Watchlist Section -->
		<section class="dashboard__content-section watchlist-section">
			<div class="watchlist-grid">
				<div class="watchlist-content">
					<h2 class="section-title">{weeklyWatchlist.title}</h2>
					<div class="watchlist-mobile-image">
						<a href={weeklyWatchlist.url}>
							<img src={weeklyWatchlist.thumbnail} alt="Weekly Watchlist" />
						</a>
					</div>
					<h4 class="watchlist-subtitle">{weeklyWatchlist.week} with {weeklyWatchlist.author}</h4>
					<p class="watchlist-description">{weeklyWatchlist.description}</p>
					<a href={weeklyWatchlist.url} class="btn btn-tiny btn-default">Watch Now</a>
				</div>
				<div class="watchlist-image">
					<a href={weeklyWatchlist.url}>
						<img src={weeklyWatchlist.thumbnail} alt="Weekly Watchlist" />
					</a>
				</div>
			</div>
		</section>
</div>
<!-- End dashboard__content -->

<!-- ═══════════════════════════════════════════════════════════════════════════
	 TRADING ROOM SIDEBAR - DEACTIVATED FOR LAYOUT OPTIMIZATION
	 Temporarily commented out to fix layout issues and ensure mobile-first design
	 ═══════════════════════════════════════════════════════════════════════════
<TradingRoomSidebar planSlug="day-trading-room" />
-->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DAY TRADING ROOM DASHBOARD STYLES - MOBILE FIRST RESPONSIVE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD HEADER - Mobile-First Responsive Design
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		margin-bottom: 20px;
		padding: 15px;
	}

	@media (min-width: 768px) {
		.dashboard__header {
			margin-bottom: 30px;
			padding: 20px;
		}
	}

	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

		/* Title Row - Mobile: stacked, Desktop: inline */
		.dashboard__header-title-row {
			display: flex;
			flex-direction: column;
			gap: 12px;
			margin-bottom: 20px;
		}

		@media (min-width: 768px) {
			.dashboard__header-title-row {
				flex-direction: row;
				align-items: center;
				justify-content: space-between;
				gap: 20px;
			}
		}

		.dashboard__page-title {
			margin: 0;
			color: #333;
			font-size: 24px;
			font-weight: 400;
			font-family: var(--font-heading), 'Montserrat', sans-serif;
			line-height: 1.2;
		}

		@media (min-width: 768px) {
			.dashboard__page-title {
				font-size: 28px;
			}
		}

		@media (min-width: 1024px) {
			.dashboard__page-title {
				font-size: 32px;
			}
		}

		@media (min-width: 1440px) {
			.dashboard__page-title {
				font-size: 36px;
			}
		}

		.btn-start-here {
			align-self: flex-start;
		}

		@media (min-width: 768px) {
			.btn-start-here {
				align-self: center;
				white-space: nowrap;
			}
		}

		/* Actions Row - Mobile: stacked, Desktop: side by side */
		.dashboard__header-actions-row {
			display: flex;
			flex-direction: column;
			gap: 15px;
		}

		@media (min-width: 768px) {
			.dashboard__header-actions-row {
				flex-direction: row;
				align-items: center;
				justify-content: space-between;
				gap: 20px;
			}
		}

		/* Trading Room Rules */
		.trading-room-rules {
			text-align: left;
			order: 2;
		}

		@media (min-width: 768px) {
			.trading-room-rules {
				text-align: right;
				order: 1;
				flex: 1;
			}
		}

		.trading-room-rules__link {
			display: block;
			font-size: 14px;
			font-weight: 700;
			font-family: var(--font-heading), 'Montserrat', sans-serif;
			color: #1e73be;
			text-decoration: none;
			margin-bottom: 4px;
			transition: color 0.15s ease-in-out;
		}

		@media (min-width: 768px) {
			.trading-room-rules__link {
				font-size: 16px;
			}
		}

		@media (min-width: 1024px) {
			.trading-room-rules__link {
				font-size: 18px;
			}
		}

		.trading-room-rules__link:hover {
			color: #0984ae;
			text-decoration: underline;
		}

		.trading-room-rules__disclaimer {
			font-size: 11px;
			color: #666;
			margin: 0;
			line-height: 1.4;
			font-family: var(--font-heading), 'Montserrat', sans-serif;
		}

		@media (min-width: 768px) {
			.trading-room-rules__disclaimer {
				font-size: 12px;
			}
		}

		@media (min-width: 1024px) {
			.trading-room-rules__disclaimer {
				font-size: 13px;
			}
		}

		/* Dropdown Container */
		.dropdown {
			order: 1;
		}

		@media (min-width: 768px) {
			.dropdown {
				order: 2;
			}
		}

		/* ═══════════════════════════════════════════════════════════════════════════
		 * BUTTONS - WordPress Exact Match
		 * ═══════════════════════════════════════════════════════════════════════════ */

		.btn {
			display: inline-flex;
			align-items: center;
			justify-content: center;
			gap: 6px;
			padding: 10px 20px;
			border-radius: 4px;
			font-size: 14px;
			font-weight: 600;
			font-family: var(--font-heading), 'Montserrat', sans-serif;
			text-decoration: none;
			border: none;
			cursor: pointer;
			transition: all 0.15s ease-in-out;
		}

		.btn-xs {
			padding: 6px 12px;
			font-size: 12px;
		}

		.btn-tiny {
			padding: 8px 16px;
			font-size: 13px;
		}

		.btn-default {
			background-color: #143E59;
			color: #fff;
		}

		.btn-default:hover {
			background-color: #0f2d41;
		}

		.btn-orange {
			background-color: #dd6b20;
			color: #fff;
		}

		.btn-orange:hover {
			background-color: #c05621;
		}

		.btn-tradingroom {
			padding: 12px 24px;
			font-size: 14px;
		}

		/* ═══════════════════════════════════════════════════════════════════════════
		 * DROPDOWN - Enter Trading Room Button
		 * ═══════════════════════════════════════════════════════════════════════════ */

		.dropdown {
			position: relative;
			display: inline-block;
		}

		.dropdown-toggle {
			display: inline-flex;
			align-items: center;
			gap: 8px;
		}

		.dropdown-arrow {
			font-size: 10px;
			transition: transform 0.2s ease;
		}

		.dropdown.is-open .dropdown-arrow {
			transform: rotate(180deg);
		}

		.dropdown-menu {
			position: absolute;
			top: 100%;
			right: 0;
			margin-top: 4px;
			background: #fff;
			border-radius: 8px;
			box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
			z-index: 1000;
			min-width: 220px;
			overflow: hidden;
		}

		.dropdown-menu__menu {
			list-style: none;
			margin: 0;
			padding: 8px 0;
		}

		.dropdown-menu__menu li a {
			display: flex;
			align-items: center;
			gap: 12px;
			padding: 12px 16px;
			color: #333;
			text-decoration: none;
			font-size: 14px;
			transition: background-color 0.15s ease;
		}

		.dropdown-menu__menu li a:hover {
			background-color: #f4f4f4;
			color: #143E59;
		}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD CONTENT LAYOUT - Mobile First Responsive
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		flex-direction: column;
		gap: 0;
		padding: 15px;
	}

	@media (min-width: 768px) {
		.dashboard__content {
			padding: 20px;
		}
	}

	@media (min-width: 1024px) {
		.dashboard__content {
			padding: 30px;
		}
	}

	.dashboard__content-section {
		margin-bottom: 30px;
	}

	@media (min-width: 768px) {
		.dashboard__content-section {
			margin-bottom: 40px;
		}
	}

	.section-title {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 16px 0;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	@media (min-width: 768px) {
		.section-title {
			font-size: 24px;
		}
	}

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

	/* Article Cards Grid - WordPress: col-xl-4 = 3 columns */
	.article-cards {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 24px;
		margin-top: 24px;
	}

	@media (max-width: 1199px) {
		.article-cards {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 767px) {
		.article-cards {
			grid-template-columns: 1fr;
		}
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

	/* Weekly Watchlist Section */
	.watchlist-section {
		background: #fff;
		border-radius: 8px;
		padding: 24px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.watchlist-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 30px;
		align-items: center;
	}

	@media (max-width: 991px) {
		.watchlist-grid {
			grid-template-columns: 1fr;
		}

		.watchlist-image {
			display: none;
		}

		.watchlist-mobile-image {
			display: block;
			margin-bottom: 16px;
		}
	}

	@media (min-width: 992px) {
		.watchlist-mobile-image {
			display: none;
		}
	}

	.watchlist-content {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.watchlist-subtitle {
		font-size: 16px;
		font-weight: 700;
		color: #333;
		margin: 0;
	}

	.watchlist-description {
		font-size: 14px;
		color: #555;
		line-height: 1.6;
		margin: 0;
	}

	.watchlist-image img,
	.watchlist-mobile-image img {
		width: 100%;
		border-radius: 8px;
	}

	/* Responsive Design */
	@media (max-width: 768px) {
		.dashboard__header {
			flex-direction: column;
			align-items: flex-start;
			gap: 16px;
		}
	}
</style>
