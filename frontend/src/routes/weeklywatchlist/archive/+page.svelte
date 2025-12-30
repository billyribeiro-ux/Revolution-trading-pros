<script lang="ts">
	/**
	 * Weekly Watchlist Archive - Matching Simpler Trading
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Archive of past weekly watchlists (spreadsheet tab)
	 * Now fetches from API with room filtering support.
	 * @version 3.0.0 - December 2025 - API integration with room filter
	 */

	import { untrack } from 'svelte';
	import { browser } from '$app/environment';
	import { watchlistApi, type WatchlistItem } from '$lib/api/watchlist';
	import { ROOMS, getRoomById } from '$lib/config/rooms';

	// State
	let items = $state<WatchlistItem[]>([]);
	let isLoading = $state(true);
	let selectedRoom = $state('');
	let currentPage = $state(1);
	let totalPages = $state(1);
	const perPage = 12;

	// Fetch watchlist items
	$effect(() => {
		const room = selectedRoom;
		const page = currentPage;
		if (!browser) return;

		untrack(() => {
			isLoading = true;
		});

		const params: any = { per_page: perPage, page, status: 'published' };
		if (room) params.room = room;

		watchlistApi.getAll(params)
			.then((response) => {
				items = response.data || [];
				totalPages = response.pagination?.last_page || 1;
				isLoading = false;
			})
			.catch((err) => {
				console.error('Failed to fetch watchlist:', err);
				isLoading = false;
			});
	});

	// Handle room filter change
	function handleRoomChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		selectedRoom = target.value;
		currentPage = 1;
	}
</script>

<svelte:head>
	<title>Weekly Watchlist Archive - Revolution Trading Pros</title>
	<meta name="description" content="Archive of past weekly trading watchlists" />
</svelte:head>

<!-- Dashboard Layout -->
<div class="dashboard">
	<!-- Sidebar Navigation -->
	<aside class="dashboard__sidebar">
		<nav class="dashboard__nav-secondary">
			<ul>
				<li>
					<a href="/dashboard/ww/">
						<span class="dashboard__nav-item-icon st-icon-dashboard"></span>
						<span class="dashboard__nav-item-text">Weekly Watchlist</span>
					</a>
				</li>
				<li>
					<a href="/dashboard/ww/watchlist-rundown-archive/">
						<span class="dashboard__nav-item-icon st-icon-chatroom-archive"></span>
						<span class="dashboard__nav-item-text">Watchlist Rundown Archive</span>
					</a>
				</li>
				<li class="is-active">
					<a href="/dashboard/ww/weekly-watchlist-archive/">
						<span class="dashboard__nav-item-icon st-icon-chatroom-archive"></span>
						<span class="dashboard__nav-item-text">Weekly Watchlist Archive</span>
					</a>
				</li>
			</ul>
		</nav>
	</aside>

	<!-- Main Content -->
	<main class="dashboard__main">
		<header class="dashboard__header">
			<div class="dashboard__header-left">
				<h1 class="dashboard__page-title">Weekly Watchlist Dashboard</h1>
			</div>
			<div class="dashboard__header-right">
				<select class="room-filter" onchange={handleRoomChange} value={selectedRoom}>
					<option value="">All Rooms</option>
					{#each ROOMS as room}
						<option value={room.id}>{room.name}</option>
					{/each}
				</select>
			</div>
		</header>

		<div class="dashboard__content">
			<div class="dashboard__content-main">
				<section class="dashboard__content-section">
					{#if isLoading}
						<div class="loading-state">
							<div class="loading-spinner"></div>
							<p>Loading watchlist items...</p>
						</div>
					{:else if items.length === 0}
						<div class="empty-state">
							<p>No watchlist items found{selectedRoom ? ` for ${getRoomById(selectedRoom)?.name || 'this room'}` : ''}.</p>
						</div>
					{:else}
						<div class="fl-post-grid-post">
							{#each items as item (item.id)}
								<div class="card fl-post-text">
									<div class="">
										<section class="card-body u--squash">
											<h4 class="h5 card-title">Weekly Watchlist for {item.datePosted}</h4>
											<div class="excerpt"><i>With {item.trader}</i></div>
											<div class="fl-post-more-link">
												<a class="btn btn-tiny btn-default" href="/watchlist/{item.slug}?tab=2">Read Now</a>
											</div>
										</section>
									</div>
								</div>
							{/each}
						</div>

						<!-- Pagination -->
						{#if totalPages > 1}
							<div class="text-center pagination-container">
								<div id="loopage_pg" class="pagination-wrap">
									{#if currentPage > 1}
										<button class="page-numbers" onclick={() => currentPage--}>&laquo;</button>
									{/if}

									<span aria-current="page" class="page-numbers current">{currentPage}</span>

									{#if currentPage < totalPages}
										<button class="page-numbers" onclick={() => currentPage++}>{currentPage + 1}</button>
									{/if}

									{#if currentPage + 1 < totalPages}
										<button class="page-numbers" onclick={() => currentPage += 2}>{currentPage + 2}</button>
									{/if}

									{#if currentPage + 2 < totalPages}
										<span class="page-numbers dots">&hellip;</span>
										<button class="page-numbers" onclick={() => currentPage = totalPages}>{totalPages}</button>
									{/if}

									{#if currentPage < totalPages}
										<button class="next page-numbers" onclick={() => currentPage++}>&raquo;</button>
									{/if}
								</div>
							</div>
						{/if}
					{/if}
				</section>
			</div>
		</div>

		<!-- Current Watchlist Section -->
		{#if items.length > 0}
			{@const latest = items[0]}
			<div class="dashboard__content-section u--background-color-white">
				<section>
					<div class="row">
						<div class="col-sm-6 col-lg-5">
							<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
							<div class="hidden-md d-lg-none pb-2">
								<a href="/watchlist/{latest.slug}">
									<img src={latest.video.poster} alt="Weekly Watchlist" class="u--border-radius" />
								</a>
							</div>
							<h4 class="h5 u--font-weight-bold">{latest.title}</h4>
							<div class="u--hide-read-more">
								<p>{latest.subtitle}</p>
							</div>
							<a href="/watchlist/{latest.slug}" class="btn btn-tiny btn-default">Watch Now</a>
						</div>
						<div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
							<a href="/watchlist/{latest.slug}">
								<img src={latest.video.poster} alt="Weekly Watchlist" class="u--border-radius" />
							</a>
						</div>
					</div>
				</section>
			</div>
		{/if}
	</main>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   Weekly Watchlist Archive - Matching Simpler Trading Exactly
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Layout */
	.dashboard {
		display: flex;
		min-height: 100vh;
		background: #f4f4f4;
	}

	/* Sidebar - WordPress dashboard.8f78208b.css: 280px width, #0f2d41 background */
	.dashboard__sidebar {
		width: 280px;
		background: #0f2d41;
		flex-shrink: 0;
	}

	.dashboard__nav-secondary ul {
		list-style: none;
		margin: 0;
		padding: 20px 0;
	}

	.dashboard__nav-secondary li {
		margin: 0;
	}

	.dashboard__nav-secondary a {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 24px;
		color: #999;
		text-decoration: none;
		font-size: 14px;
		transition: all 0.2s;
	}

	.dashboard__nav-secondary a:hover {
		color: #fff;
		background: rgba(255, 255, 255, 0.05);
	}

	.dashboard__nav-secondary li.is-active a {
		color: #fff;
		background: rgba(9, 132, 174, 0.2);
		border-left: 3px solid #0984ae;
	}

	.dashboard__nav-item-icon {
		font-size: 18px;
	}

	/* Main Content */
	.dashboard__main {
		flex: 1;
		min-width: 0;
	}

	.dashboard__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 30px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		gap: 20px;
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__header-right {
		flex-shrink: 0;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__content {
		padding: 30px;
	}

	.dashboard__content-section {
		background: #fff;
		border-radius: 4px;
		padding: 20px;
		margin-bottom: 20px;
	}

	/* Room Filter Dropdown */
	.room-filter {
		padding: 8px 12px;
		font-size: 14px;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		background: #fff;
		color: #333;
		cursor: pointer;
		min-width: 160px;
	}

	.room-filter:hover {
		border-color: #999;
	}

	.room-filter:focus {
		outline: none;
		border-color: #0984ae;
		box-shadow: 0 0 0 2px rgba(9, 132, 174, 0.1);
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		color: #666;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e0e0e0;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	.empty-state p {
		font-size: 16px;
		margin: 0;
	}

	/* Archive Grid - Matching Reference */
	.fl-post-grid-post {
		font-size: 19px;
		background: transparent !important;
		border: 0;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		gap: 1%;
	}

	.card {
		max-width: 32%;
		margin: 0.5%;
		background: #fff;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		flex: 1 1 calc(33.333% - 1%);
		min-width: 280px;
	}

	.card-body.u--squash {
		padding: 20px;
	}

	.card-title {
		font-size: 24px;
		line-height: 1.3;
		margin: 0 0 10px;
		color: #333;
		font-weight: 600;
	}

	.excerpt {
		margin: 20px 0;
		font-size: 16px;
		color: #666;
	}

	.fl-post-more-link {
		margin-top: 15px;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 10px 20px;
		text-decoration: none;
		border-radius: 3px;
		font-weight: 600;
		transition: all 0.2s;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 13px;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e8e8e8;
		border-color: #999;
	}

	/* Pagination - Matching Reference */
	.pagination-container {
		margin-top: 20px;
		text-align: center;
	}

	.pagination-wrap {
		display: inline-flex;
		gap: 0;
	}

	.page-numbers {
		display: inline-block;
		padding: 8px 14px;
		font-size: 17px;
		font-weight: 600;
		color: #666;
		background: #fff !important;
		text-decoration: none;
		border: 1px solid #e6e6e6;
		margin-left: -1px;
	}

	.page-numbers:first-child {
		border-radius: 5px 0 0 5px;
		margin-left: 0;
	}

	.page-numbers:last-child {
		border-radius: 0 5px 5px 0;
	}

	.page-numbers:hover {
		background: #f5f5f5 !important;
	}

	.page-numbers.current {
		color: #f4f4f4;
		background: #0984ae !important;
		border-color: #0984ae;
	}

	.page-numbers.dots {
		border: none;
		background: transparent !important;
	}

	/* Current Watchlist Section */
	.u--background-color-white {
		background: #fff;
		margin: 0 30px 30px;
		padding: 30px;
		border-radius: 4px;
	}

	.section-title-alt {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
	}

	.section-title-alt--underline {
		padding-bottom: 15px;
		border-bottom: 2px solid #0984ae;
	}

	.u--font-weight-bold {
		font-weight: 700;
	}

	.u--border-radius {
		border-radius: 4px;
		width: 100%;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
		padding: 0 15px;
	}

	.col-lg-5 {
		flex: 0 0 41.666667%;
		max-width: 41.666667%;
	}

	.col-lg-7 {
		flex: 0 0 58.333333%;
		max-width: 58.333333%;
	}

	.pb-2 {
		padding-bottom: 10px;
	}

	/* Responsive */
	@media (max-width: 992px) {
		.dashboard__sidebar {
			display: none;
		}

		.col-lg-5,
		.col-lg-7 {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.d-lg-block {
			display: none !important;
		}

		.d-lg-none {
			display: block !important;
		}
	}

	@media (max-width: 768px) {
		.card {
			flex: 1 1 100%;
			max-width: 100%;
		}

		.col-sm-6 {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.dashboard__content {
			padding: 15px;
		}

		.u--background-color-white {
			margin: 0 15px 15px;
			padding: 20px;
		}
	}

	/* Hide classes */
	.hidden-xs,
	.hidden-sm {
		display: block;
	}

	.hidden-md {
		display: none;
	}

	@media (max-width: 992px) {
		.hidden-md {
			display: block;
		}

		.d-none {
			display: none !important;
		}
	}
</style>
