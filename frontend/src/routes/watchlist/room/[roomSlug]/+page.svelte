<script lang="ts">
	/**
	 * Room-Specific Watchlist Archive
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Displays all watchlist items for a specific trading room or service.
	 * @version 1.0.0 - December 2025
	 */

	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import { untrack } from 'svelte';
	import { watchlistApi, type WatchlistItem } from '$lib/api/watchlist';
	import { getRoomById, type Room } from '$lib/config/rooms';
	import {
		DashboardHeader,
		ArticleCard,
		SectionTitle
	} from '$lib/components/dashboard';

	// Get room from URL
	const roomSlug = $derived($page.params.roomSlug);
	const room = $derived<Room | undefined>(getRoomById(roomSlug));

	// State
	let items = $state<WatchlistItem[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	// Fetch watchlist items for this room
	$effect(() => {
		const currentSlug = roomSlug;
		if (!browser || !currentSlug) return;

		untrack(() => {
			isLoading = true;
			error = null;
		});

		watchlistApi.getByRoom(currentSlug, { per_page: 20 })
			.then((response) => {
				items = response.data || [];
				isLoading = false;
			})
			.catch((err) => {
				console.error('Failed to fetch watchlist items:', err);
				error = err.message || 'Failed to load watchlist items';
				isLoading = false;
			});
	});
</script>

<svelte:head>
	<title>Weekly Watchlist {room ? `- ${room.name}` : ''} | Revolution Trading Pros</title>
	<meta name="description" content="Weekly Watchlist archives for {room?.name || 'your trading room'}. Video rundowns and downloadable watchlists." />
</svelte:head>

<!-- DASHBOARD HEADER -->
<DashboardHeader
	title="Weekly Watchlist {room ? `- ${room.shortName}` : ''}"
	showRules={false}
>
	{#snippet rightContent()}
		{#if room}
			<a href="/dashboard/{room.slug}" class="btn-back">
				← Back to {room.shortName} Dashboard
			</a>
		{/if}
	{/snippet}
</DashboardHeader>

<!-- ROOM INFO BANNER -->
{#if room}
	<div class="room-banner" style="background: linear-gradient(135deg, {room.color}20, {room.color}05);">
		<div class="room-banner__content">
			<span class="room-banner__icon">{room.icon}</span>
			<div class="room-banner__info">
				<h2 class="room-banner__name">{room.name}</h2>
				<p class="room-banner__type">
					{room.type === 'live-trading' ? 'Live Trading Room' : 'Alerts Only Service'}
				</p>
			</div>
		</div>
	</div>
{:else if roomSlug && !isLoading}
	<div class="room-not-found">
		<p>Room "{roomSlug}" not found.</p>
		<a href="/dashboard" class="btn-default">Return to Dashboard</a>
	</div>
{/if}

<!-- WATCHLIST CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<SectionTitle title="Watchlist Archive" />

			{#if isLoading}
				<div class="loading-state">
					<div class="loading-spinner"></div>
					<p>Loading watchlist items...</p>
				</div>
			{:else if error}
				<div class="error-state">
					<p>{error}</p>
					<button type="button" onclick={() => location.reload()} class="btn-retry">
						Try Again
					</button>
				</div>
			{:else if items.length === 0}
				<div class="empty-state">
					<p>No watchlist items available for this room.</p>
					{#if room}
						<a href="/dashboard/{room.slug}" class="btn-default">Return to Dashboard</a>
					{/if}
				</div>
			{:else}
				<div class="article-cards">
					{#each items as item (item.id)}
						<div class="article-cards__item">
							<ArticleCard
								title={item.title}
								href="/watchlist/{item.slug}"
								image={item.video.poster}
								meta={item.datePosted}
								excerpt={item.subtitle}
								buttonText="Watch Now"
								isVideo={true}
							/>
						</div>
					{/each}
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	/* Back Button */
	.btn-back {
		display: inline-block;
		padding: 6px 12px;
		font-size: 12px;
		font-weight: 600;
		background: #f5f5f5;
		color: #333;
		border: 1px solid #dbdbdb;
		border-radius: 3px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-back:hover {
		background: #e0e0e0;
	}

	/* Room Banner */
	.room-banner {
		padding: 20px 30px;
		border-bottom: 1px solid #dbdbdb;
	}

	.room-banner__content {
		display: flex;
		align-items: center;
		gap: 16px;
		max-width: 1200px;
	}

	.room-banner__icon {
		font-size: 32px;
	}

	.room-banner__info {
		flex: 1;
	}

	.room-banner__name {
		margin: 0;
		font-size: 18px;
		font-weight: 700;
		color: #333;
		font-family: 'Open Sans', sans-serif;
	}

	.room-banner__type {
		margin: 4px 0 0;
		font-size: 13px;
		color: #666;
	}

	/* Room Not Found */
	.room-not-found {
		text-align: center;
		padding: 60px 20px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
	}

	.room-not-found p {
		font-size: 16px;
		color: #666;
		margin-bottom: 20px;
	}

	/* Dashboard Content */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		flex: 1 1 auto;
		min-width: 0;
	}

	.dashboard__content-section {
		padding: 30px;
		background-color: #fff;
	}

	@media (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	/* Article Cards Grid */
	.article-cards {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
	}

	.article-cards__item {
		display: flex;
		min-width: 0;
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

	/* Error State */
	.error-state {
		text-align: center;
		padding: 40px 20px;
		color: #666;
	}

	.btn-retry {
		margin-top: 16px;
		padding: 10px 20px;
		background: #0984ae;
		color: #fff;
		border: none;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	.btn-retry:hover {
		background: #076787;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	.empty-state p {
		margin-bottom: 20px;
	}

	.btn-default {
		display: inline-block;
		padding: 10px 20px;
		background: #f5f5f5;
		color: #333;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: background-color 0.2s;
	}

	.btn-default:hover {
		background: #e0e0e0;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.room-banner {
			padding: 16px 20px;
		}

		.room-banner__icon {
			font-size: 24px;
		}

		.room-banner__name {
			font-size: 16px;
		}

		.dashboard__content-section {
			padding: 20px;
		}
	}
</style>
