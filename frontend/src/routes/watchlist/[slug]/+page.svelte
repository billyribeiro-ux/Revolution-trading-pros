<!--
	URL: /watchlist/[slug]
	
	Weekly Watchlist Single Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Features:
	- Video player with poster image
	- Rundown/Watchlist tabs
	- Pagination (Previous/Next) based on position in list
	- Responsive design matching Simpler Trading WordPress
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// Mock data for weekly watchlist entries - in production this would come from API
	const watchlistEntries = [
		{
			slug: '01032026-melissa-beegle',
			title: 'Weekly Watchlist with Melissa Beegle',
			trader: 'Melissa Beegle',
			weekOf: 'January 3, 2026',
			poster: 'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
			videoUrl: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-MB-01032026.mp4',
			spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml',
			isLatest: true,
			watchlistDates: [
				{ date: '1/3/2026', spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml' },
				{ date: '5/28/2025', spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml' },
				{ date: '3/9/2025', spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml' }
			]
		},
		{
			slug: '12292025-david-starr',
			title: 'Weekly Watchlist with David Starr',
			trader: 'David Starr',
			weekOf: 'December 29, 2025',
			poster: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
			videoUrl: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-DS-12292025.mp4',
			spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml',
			isLatest: false
		},
		{
			slug: '12222025-tg-watkins',
			title: 'Weekly Watchlist with TG Watkins',
			trader: 'TG Watkins',
			weekOf: 'December 22, 2025',
			poster: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			videoUrl: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-TG-12222025.mp4',
			spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml',
			isLatest: false
		}
	];

	// State
	let activeTab = $state<'rundown' | 'watchlist'>('rundown');
	let currentEntry = $state<typeof watchlistEntries[0] | null>(null);
	let previousEntry = $state<typeof watchlistEntries[0] | null>(null);
	let nextEntry = $state<typeof watchlistEntries[0] | null>(null);
	let selectedDateIndex = $state(0);
	let currentSpreadsheetUrl = $state('');

	// Get current slug from URL
	const slug = $derived(page.params.slug);

	// Find current entry and adjacent entries for pagination
	$effect(() => {
		const currentIndex = watchlistEntries.findIndex(e => e.slug === slug);
		
		if (currentIndex !== -1) {
			currentEntry = watchlistEntries[currentIndex];
			
			// Previous = older entry (higher index in array since newest is first)
			previousEntry = currentIndex < watchlistEntries.length - 1 
				? watchlistEntries[currentIndex + 1] 
				: null;
			
			// Next = newer entry (lower index in array since newest is first)
			nextEntry = currentIndex > 0 
				? watchlistEntries[currentIndex - 1] 
				: null;
		}
	});

	// Handle "latest" slug - redirect to most recent
	$effect(() => {
		if (slug === 'latest' || slug === 'current') {
			currentEntry = watchlistEntries[0];
			previousEntry = watchlistEntries.length > 1 ? watchlistEntries[1] : null;
			nextEntry = null; // Latest has no next
		}
	});

	function setTab(tab: 'rundown' | 'watchlist') {
		activeTab = tab;
	}

	function selectDate(index: number) {
		selectedDateIndex = index;
		if (currentEntry?.watchlistDates?.[index]) {
			currentSpreadsheetUrl = currentEntry.watchlistDates[index].spreadsheetUrl;
		}
	}

	function nextDate() {
		if (currentEntry?.watchlistDates && selectedDateIndex < currentEntry.watchlistDates.length - 1) {
			selectDate(selectedDateIndex + 1);
		}
	}

	function previousDate() {
		if (selectedDateIndex > 0) {
			selectDate(selectedDateIndex - 1);
		}
	}

	// Initialize spreadsheet URL when entry changes
	$effect(() => {
		if (currentEntry?.watchlistDates?.[selectedDateIndex]) {
			currentSpreadsheetUrl = currentEntry.watchlistDates[selectedDateIndex].spreadsheetUrl;
		} else if (currentEntry?.spreadsheetUrl) {
			currentSpreadsheetUrl = currentEntry.spreadsheetUrl;
		}
	});
</script>

<svelte:head>
	<title>{currentEntry?.title || 'Weekly Watchlist'} - Revolution Trading Pros</title>
</svelte:head>

<div class="watchlist-page">
	<!-- Breadcrumb - WordPress Match -->
	<nav class="breadcrumbs">
		<div class="container-fluid">
			<ul>
				<li class="item-home">
					<a class="breadcrumb-link" href="/">Home</a>
				</li>
				<li class="separator"> / </li>
				<li class="item-cat">
					<a class="breadcrumb-link" href="/dashboard/weekly-watchlist">Weekly Watchlist</a>
				</li>
				<li class="separator"> / </li>
				<li class="item-current">
					<strong class="breadcrumb-current">{currentEntry?.title || 'Loading...'}</strong>
				</li>
			</ul>
		</div>
	</nav>

	<!-- Header Section with Pagination - WordPress Match -->
	<section id="ww-title" class="cpost-section cpost-title-section">
		<div class="section-inner">
			<!-- Previous Button -->
			<div id="w-previous" class="cpost-previous">
				{#if previousEntry}
					<a href="/watchlist/{previousEntry.slug}" title="{previousEntry.title}">
						<i class="fa fa-chevron-circle-left"></i><span> Previous</span>
					</a>
				{/if}
			</div>

			<!-- Title -->
			<h1 class="cpost-title">{currentEntry?.title || 'Weekly Watchlist'}</h1>

			<!-- Next Button -->
			<div id="ww-next" class="cpost-next">
				{#if nextEntry}
					<a href="/watchlist/{nextEntry.slug}" title="{nextEntry.title}">
						<span>Next </span><i class="fa fa-chevron-circle-right"></i>
					</a>
				{/if}
			</div>

			<!-- Subtitle -->
			<h2 class="cpost-subtitle sub-lg-up">Week of {currentEntry?.weekOf || ''}</h2>
		</div>
	</section>

	<!-- Content Section -->
	<section class="cpost-section ww-content-section">
		<div class="section-inner">
			<!-- Tabs - WordPress Match -->
			<div class="tr_tabs ww-single">
				<div class="tab">
					<button 
						id="tab-link-1"
						data-tab="tab-1"
						class="tablinks" 
						class:active={activeTab === 'rundown'}
						onclick={() => setTab('rundown')}
					>
						Rundown
					</button>
					<button 
						id="tab-link-2"
						data-tab="tab-2"
						class="tablinks" 
						class:active={activeTab === 'watchlist'}
						onclick={() => setTab('watchlist')}
					>
						Watchlist
					</button>
				</div>

				<!-- Rundown Tab Content -->
				<div id="tab-1" class="tabcontent" class:active={activeTab === 'rundown'}>
					{#if currentEntry}
						<div class="ww-content-block">
							<div class="current-vid">
								<h3 class="current-title">{currentEntry.title}</h3>
								<div class="video-container">
									<video 
										controls 
										width="100%" 
										poster={currentEntry.poster}
										style="aspect-ratio: 16/9;"
										title={currentEntry.title}
									>
										<source src={currentEntry.videoUrl} type="video/mp4">
										Your browser does not support the video tag.
									</video>
								</div>
							</div>
						</div>
						<div class="ww-description">Week starting on {currentEntry.weekOf}.</div>
					{/if}
				</div>

				<!-- Watchlist Tab Content -->
				<div id="tab-2" class="tabcontent" class:active={activeTab === 'watchlist'}>
					{#if currentSpreadsheetUrl}
						<div id="ww-spreadsheet" class="ww-spreadsheet">
							<iframe 
								src={currentSpreadsheetUrl} 
								width="100%" 
								height="700"
								title="Weekly Watchlist Spreadsheet"
							></iframe>
						</div>

						<!-- Date Switcher -->
						{#if currentEntry?.watchlistDates && currentEntry.watchlistDates.length > 1}
							<div class="switcherContainer">
								<table class="switcherTable">
									<tbody>
										<tr>
											{#each currentEntry.watchlistDates as dateOption, index}
												<td 
													class:switcherItemActive={selectedDateIndex === index}
													class:switcherItem={selectedDateIndex !== index}
													onclick={() => selectDate(index)}
												>
													{dateOption.date}
												</td>
											{/each}
										</tr>
									</tbody>
								</table>
								
								<div class="switcherArrows">
									<button 
										class="switcherArrow" 
										disabled={selectedDateIndex >= (currentEntry.watchlistDates.length - 1)}
										onclick={nextDate}
										aria-label="Next date"
									>
										<b>&gt;</b>
									</button>
									<button 
										class="switcherArrow" 
										disabled={selectedDateIndex <= 0}
										onclick={previousDate}
										aria-label="Previous date"
									>
										<b>&lt;</b>
									</button>
								</div>
							</div>
						{/if}
					{/if}
				</div>
			</div>
		</div>
	</section>
</div>

<style>
	.watchlist-page {
		background-color: #f5f5f5;
		min-height: 100vh;
		padding-bottom: 40px;
	}

	/* Breadcrumbs - WordPress Match */
	.breadcrumbs {
		padding: 10px 20px;
		font-size: 13px;
		color: #666;
		background: #f9f9f9;
		border-bottom: 1px solid #e5e5e5;
	}

	.breadcrumbs .container-fluid {
		max-width: 1100px;
		margin: 0 auto;
		padding: 0;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		flex-wrap: wrap;
	}

	.breadcrumbs li {
		margin: 0;
		padding: 0;
	}

	.breadcrumbs a {
		color: #0984ae;
		text-decoration: none;
		font-size: 13px;
	}

	.breadcrumbs a:hover {
		text-decoration: underline;
	}

	.breadcrumbs .separator {
		margin: 0 8px;
		color: #ccc;
	}

	.breadcrumb-current {
		color: #333;
		font-weight: 600;
		font-size: 13px;
	}

	/* Header Section - WordPress Match */
	.cpost-section.cpost-title-section {
		background: #143E59;
		padding: 40px 20px;
		text-align: center;
	}

	.section-inner {
		max-width: 1100px;
		margin: 0 auto;
		position: relative;
	}

	.cpost-title {
		color: #fff;
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 10px;
		font-family: 'Open Sans', sans-serif;
	}

	.cpost-subtitle {
		color: rgba(255, 255, 255, 0.8);
		font-size: 16px;
		font-weight: 400;
		margin: 0;
	}

	/* Pagination Buttons */
	.cpost-previous,
	.cpost-next {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
	}

	.cpost-previous {
		left: 0;
	}

	.cpost-next {
		right: 0;
	}

	.cpost-previous a,
	.cpost-next a {
		display: flex;
		align-items: center;
		gap: 6px;
		color: #fff;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		padding: 10px 16px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 5px;
		transition: background 0.2s ease;
	}

	.cpost-previous a:hover,
	.cpost-next a:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	/* Content Section */
	.cpost-section {
		padding: 30px 20px;
	}

	/* Tabs - WordPress Match */
	.tr_tabs.ww-single {
		background: #fff;
		border-radius: 0;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.tab {
		display: flex;
		border-bottom: 2px solid #e5e5e5;
		background: #f5f5f5;
	}

	.tablinks {
		flex: 1;
		padding: 18px 24px;
		font-size: 16px;
		font-weight: 600;
		color: #666;
		background: transparent;
		border: none;
		border-bottom: 3px solid transparent;
		cursor: pointer;
		transition: all 0.3s ease;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.tablinks:hover {
		background: rgba(255, 255, 255, 0.5);
		color: #333;
	}

	.tablinks.active {
		background: #fff;
		color: #143E59;
		border-bottom-color: #143E59;
	}

	.tabcontent {
		display: none;
		padding: 40px 30px;
		background: #fff;
	}

	.tabcontent.active {
		display: block;
	}

	/* Video Content */
	.ww-content-block {
		max-width: 900px;
		margin: 0 auto;
	}

	.current-title {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	.video-container {
		background: #000;
		border-radius: 8px;
		overflow: hidden;
	}

	.video-container video {
		display: block;
		width: 100%;
		height: auto;
	}

	.ww-description {
		text-align: center;
		color: #666;
		font-size: 14px;
		margin-top: 20px;
		padding-top: 20px;
		border-top: 1px solid #e5e5e5;
	}

	/* Spreadsheet */
	.ww-spreadsheet {
		border-radius: 8px;
		overflow: hidden;
		border: 1px solid #e5e5e5;
	}

	.ww-spreadsheet iframe {
		display: block;
		border: none;
	}

	/* Date Switcher */
	.switcherContainer {
		margin-top: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 20px;
	}

	.switcherTable {
		border-collapse: collapse;
	}

	.switcherTable tbody tr {
		display: flex;
		justify-content: center;
		gap: 10px;
	}

	.switcherTable td {
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		border-radius: 5px;
		transition: all 0.2s ease;
		font-family: 'Open Sans', sans-serif;
	}

	.switcherItem {
		background: #f5f5f5;
		color: #666;
		border: 1px solid #e5e5e5;
	}

	.switcherItem:hover {
		background: #e8e8e8;
		color: #333;
	}

	.switcherItemActive {
		background: #143E59;
		color: #fff;
		border: 1px solid #143E59;
	}

	.switcherArrows {
		display: flex;
		gap: 8px;
	}

	.switcherArrow {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #143E59;
		color: #fff;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-size: 18px;
		padding: 0;
	}

	.switcherArrow:hover:not(:disabled) {
		background: #0f2d41;
	}

	.switcherArrow:disabled {
		background: #e5e5e5;
		color: #999;
		cursor: not-allowed;
		opacity: 0.5;
	}

	.switcherArrow b {
		font-weight: 700;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.cpost-section.cpost-title-section {
			padding: 25px 15px;
		}

		.cpost-title {
			font-size: 20px;
			padding: 0 80px;
		}

		.cpost-subtitle {
			font-size: 14px;
		}

		.cpost-previous a span,
		.cpost-next a span {
			display: none;
		}

		.cpost-previous a,
		.cpost-next a {
			padding: 10px;
		}

		.tabcontent {
			padding: 25px 15px;
		}

		.tablinks {
			padding: 14px 16px;
			font-size: 14px;
		}

		.current-title {
			font-size: 18px;
		}

		.ww-spreadsheet iframe {
			height: 500px;
		}
	}

	@media (max-width: 480px) {
		.cpost-title {
			font-size: 18px;
			padding: 0 50px;
		}

		.cpost-previous,
		.cpost-next {
			position: static;
			transform: none;
		}

		.section-inner {
			display: flex;
			flex-direction: column;
			align-items: center;
			gap: 15px;
		}

		.cpost-section.cpost-title-section .section-inner {
			flex-direction: row;
			flex-wrap: wrap;
			justify-content: space-between;
		}

		.cpost-title {
			order: -1;
			width: 100%;
			padding: 0;
			margin-bottom: 5px;
		}

		.cpost-subtitle {
			order: 0;
			width: 100%;
			margin-bottom: 15px;
		}
	}
</style>
