<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	let selectedCategory = $state('all');

	const videos = [
		{
			id: 1,
			title: 'Weekly Breakdown: Top Swing Setups',
			date: 'January 13, 2026',
			duration: '24:35',
			category: 'weekly',
			thumbnail: 'https://placehold.co/640x360/143E59/FFFFFF/png?text=Weekly+Breakdown',
			excerpt: 'Complete breakdown of this week\'s top swing trade opportunities with technical analysis.',
			href: '/dashboard/explosive-swings/video/weekly-011326'
		},
		{
			id: 2,
			title: 'NVDA Entry Alert - Opening Swing Position',
			date: 'January 13, 2026',
			duration: '8:45',
			category: 'entry',
			thumbnail: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=NVDA+ENTRY',
			excerpt: 'Entering NVDA at $142.50. Full breakdown of entry reasoning and risk management.',
			href: '/dashboard/explosive-swings/updates/nvda-entry-011326'
		},
		{
			id: 3,
			title: 'MSFT Exit - Closing for +8.2% Profit',
			date: 'January 12, 2026',
			duration: '6:20',
			category: 'exit',
			thumbnail: 'https://placehold.co/640x360/3b82f6/FFFFFF/png?text=MSFT+EXIT',
			excerpt: 'Taking profits on MSFT. Exit strategy breakdown and lessons learned.',
			href: '/dashboard/explosive-swings/updates/msft-exit-011226'
		},
		{
			id: 4,
			title: 'META Entry - Momentum Play Setup',
			date: 'January 11, 2026',
			duration: '7:15',
			category: 'entry',
			thumbnail: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=META+ENTRY',
			excerpt: 'META triggered our entry. Full breakdown of the momentum setup.',
			href: '/dashboard/explosive-swings/updates/meta-entry-011126'
		},
		{
			id: 5,
			title: 'Market Analysis: Tech Sector Rotation',
			date: 'January 10, 2026',
			duration: '15:30',
			category: 'analysis',
			thumbnail: 'https://placehold.co/640x360/f59e0b/FFFFFF/png?text=Market+Analysis',
			excerpt: 'Deep dive into tech sector rotation and what it means for swing traders.',
			href: '/dashboard/explosive-swings/video/market-analysis-011026'
		},
		{
			id: 6,
			title: 'AMD Short Setup - Bearish Breakdown',
			date: 'January 10, 2026',
			duration: '9:10',
			category: 'entry',
			thumbnail: 'https://placehold.co/640x360/ef4444/FFFFFF/png?text=AMD+SHORT',
			excerpt: 'AMD short position triggered. Breakdown of the bearish setup.',
			href: '/dashboard/explosive-swings/updates/amd-short-011026'
		},
		{
			id: 7,
			title: 'Weekly Breakdown: January 6th Week',
			date: 'January 6, 2026',
			duration: '26:15',
			category: 'weekly',
			thumbnail: 'https://placehold.co/640x360/143E59/FFFFFF/png?text=Weekly+Breakdown',
			excerpt: 'Last week\'s complete market breakdown and swing trade opportunities.',
			href: '/dashboard/explosive-swings/video/weekly-010626'
		},
		{
			id: 8,
			title: 'TSLA Exit - Target Hit for +12.5%',
			date: 'January 5, 2026',
			duration: '5:45',
			category: 'exit',
			thumbnail: 'https://placehold.co/640x360/3b82f6/FFFFFF/png?text=TSLA+EXIT',
			excerpt: 'Closing TSLA position after hitting our third target. Trade review.',
			href: '/dashboard/explosive-swings/updates/tsla-exit-010526'
		},
		{
			id: 9,
			title: 'Options Strategy: Weekly Swing Plays',
			date: 'January 3, 2026',
			duration: '18:20',
			category: 'education',
			thumbnail: 'https://placehold.co/640x360/7c3aed/FFFFFF/png?text=Options+Strategy',
			excerpt: 'How to use options for swing trading. Strike selection and timing.',
			href: '/dashboard/explosive-swings/video/options-strategy-010326'
		},
		{
			id: 10,
			title: 'Risk Management Masterclass',
			date: 'December 30, 2025',
			duration: '22:40',
			category: 'education',
			thumbnail: 'https://placehold.co/640x360/7c3aed/FFFFFF/png?text=Risk+Management',
			excerpt: 'Complete guide to position sizing and risk management for swing trades.',
			href: '/dashboard/explosive-swings/video/risk-management-123025'
		},
		{
			id: 11,
			title: 'AMZN Entry - Breakout Confirmation',
			date: 'December 28, 2025',
			duration: '7:45',
			category: 'entry',
			thumbnail: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=AMZN+ENTRY',
			excerpt: 'AMZN breaking above key resistance. Entry at $185 with technical breakdown.',
			href: '/dashboard/explosive-swings/updates/amzn-entry-122825'
		},
		{
			id: 12,
			title: 'Year-End Market Outlook 2026',
			date: 'December 27, 2025',
			duration: '32:15',
			category: 'analysis',
			thumbnail: 'https://placehold.co/640x360/f59e0b/FFFFFF/png?text=2026+Outlook',
			excerpt: 'Looking ahead to 2026. Top sectors and setups to watch.',
			href: '/dashboard/explosive-swings/video/2026-outlook-122725'
		}
	];

	const filteredVideos = $derived(
		selectedCategory === 'all'
			? videos
			: videos.filter(v => v.category === selectedCategory)
	);
</script>

<svelte:head>
	<title>Video Library | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader 
	roomName="Explosive Swings" 
	startHereUrl="/dashboard/explosive-swings/start-here" 
/>

<div class="video-library">
	<div class="page-header">
		<h1>Video Library</h1>
		<p>All trade breakdowns, market analysis, and educational content</p>
	</div>

	<div class="filter-section">
		<button class="filter-btn" class:active={selectedCategory === 'all'} onclick={() => selectedCategory = 'all'}>
			All Videos
		</button>
		<button class="filter-btn" class:active={selectedCategory === 'weekly'} onclick={() => selectedCategory = 'weekly'}>
			Weekly Breakdowns
		</button>
		<button class="filter-btn" class:active={selectedCategory === 'entry'} onclick={() => selectedCategory = 'entry'}>
			Entry Alerts
		</button>
		<button class="filter-btn" class:active={selectedCategory === 'exit'} onclick={() => selectedCategory = 'exit'}>
			Exit Alerts
		</button>
		<button class="filter-btn" class:active={selectedCategory === 'analysis'} onclick={() => selectedCategory = 'analysis'}>
			Market Analysis
		</button>
		<button class="filter-btn" class:active={selectedCategory === 'education'} onclick={() => selectedCategory = 'education'}>
			Education
		</button>
	</div>

	<div class="videos-grid">
		{#each filteredVideos as video}
			<a href={video.href} class="video-card">
				<div class="video-thumbnail" style="background-image: url('{video.thumbnail}')">
					<div class="play-overlay">
						<svg viewBox="0 0 24 24" fill="currentColor" width="56" height="56">
							<path d="M8 5v14l11-7z"/>
						</svg>
					</div>
					<div class="video-duration">{video.duration}</div>
				</div>
				<div class="video-content">
					<div class="video-category">{video.category.toUpperCase()}</div>
					<h3>{video.title}</h3>
					<p class="video-date">{video.date}</p>
					<p class="video-excerpt">{video.excerpt}</p>
				</div>
			</a>
		{/each}
	</div>
</div>

<style>
	.video-library {
		background: #f5f7fa;
		min-height: 100vh;
		padding: 40px 30px;
	}

	.page-header {
		text-align: center;
		max-width: 800px;
		margin: 0 auto 40px;
	}

	.page-header h1 {
		font-size: 36px;
		font-weight: 700;
		margin: 0 0 12px 0;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.page-header p {
		font-size: 16px;
		color: #666;
		margin: 0;
	}

	.filter-section {
		display: flex;
		gap: 10px;
		justify-content: center;
		flex-wrap: wrap;
		margin-bottom: 40px;
	}

	.filter-btn {
		background: #fff;
		border: 2px solid #e5e7eb;
		padding: 10px 20px;
		border-radius: 25px;
		font-size: 14px;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-btn:hover {
		border-color: #143E59;
		color: #143E59;
	}

	.filter-btn.active {
		background: #143E59;
		border-color: #143E59;
		color: #fff;
	}

	.videos-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 30px;
		max-width: 1400px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		.videos-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.videos-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.video-card {
		background: #fff;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 4px 20px rgba(0,0,0,0.08);
		transition: all 0.3s ease;
		text-decoration: none;
		color: inherit;
		display: block;
	}

	.video-card:hover {
		transform: translateY(-8px);
		box-shadow: 0 12px 40px rgba(0,0,0,0.15);
	}

	.video-thumbnail {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background-size: cover;
		background-position: center;
	}

	.play-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0,0,0,0.4);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: background 0.3s;
	}

	.video-card:hover .play-overlay {
		background: rgba(0,0,0,0.6);
	}

	.play-overlay svg {
		color: #fff;
		opacity: 0.9;
		transition: all 0.3s;
	}

	.video-card:hover .play-overlay svg {
		opacity: 1;
		transform: scale(1.1);
	}

	.video-duration {
		position: absolute;
		bottom: 12px;
		right: 12px;
		background: rgba(0,0,0,0.85);
		color: #fff;
		padding: 5px 10px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
	}

	.video-content {
		padding: 20px;
		text-align: center;
	}

	.video-category {
		display: inline-block;
		background: #F69532;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		padding: 4px 10px;
		border-radius: 4px;
		margin-bottom: 10px;
	}

	.video-content h3 {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 8px 0;
		color: #333;
		line-height: 1.4;
	}

	.video-date {
		font-size: 12px;
		color: #888;
		margin: 0 0 12px 0;
	}

	.video-excerpt {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0;
	}
</style>
