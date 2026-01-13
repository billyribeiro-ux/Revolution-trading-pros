<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	const favorites = [
		{
			id: 1,
			type: 'alert',
			title: 'NVDA Entry Alert - Opening Swing Position',
			date: 'January 13, 2026',
			excerpt: 'Entering NVDA at $142.50. Watch this video breakdown for entry reasoning.',
			href: '/dashboard/explosive-swings/updates/nvda-entry-011326'
		},
		{
			id: 2,
			type: 'video',
			title: 'Risk Management Masterclass',
			date: 'December 30, 2025',
			excerpt: 'Complete guide to position sizing and risk management for swing trades.',
			href: '/dashboard/explosive-swings/video/risk-management-123025'
		},
		{
			id: 3,
			type: 'alert',
			title: 'MSFT Exit - Closing for +8.2% Profit',
			date: 'January 12, 2026',
			excerpt: 'Taking profits on MSFT. Exit strategy breakdown and lessons learned.',
			href: '/dashboard/explosive-swings/updates/msft-exit-011226'
		}
	];
</script>

<svelte:head>
	<title>Favorites | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader 
	roomName="Explosive Swings" 
	startHereUrl="/dashboard/explosive-swings/start-here" 
/>

<div class="favorites-page">
	<div class="page-header">
		<h1>⭐ Your Favorites</h1>
		<p>Bookmarked alerts and videos for quick access</p>
	</div>

	{#if favorites.length === 0}
		<div class="empty-state">
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="64" height="64">
				<path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
			</svg>
			<h2>No favorites yet</h2>
			<p>Click the star icon on any alert or video to save it here</p>
		</div>
	{:else}
		<div class="favorites-list">
			{#each favorites as item}
				<a href={item.href} class="favorite-card">
					<div class="favorite-type">{item.type.toUpperCase()}</div>
					<h3>{item.title}</h3>
					<p class="favorite-date">{item.date}</p>
					<p class="favorite-excerpt">{item.excerpt}</p>
					<button class="remove-btn" onclick={(e) => { e.preventDefault(); alert('Remove from favorites'); }}>
						Remove
					</button>
				</a>
			{/each}
		</div>
	{/if}
</div>

<style>
	.favorites-page {
		background: #f5f7fa;
		min-height: 100vh;
		padding: 40px 30px;
	}

	.page-header {
		text-align: center;
		max-width: 800px;
		margin: 0 auto 50px;
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

	.empty-state {
		text-align: center;
		padding: 80px 20px;
		max-width: 500px;
		margin: 0 auto;
	}

	.empty-state svg {
		color: #d1d5db;
		margin-bottom: 20px;
	}

	.empty-state h2 {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px 0;
	}

	.empty-state p {
		font-size: 16px;
		color: #666;
		margin: 0;
	}

	.favorites-list {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 20px;
		max-width: 900px;
		margin: 0 auto;
	}

	.favorite-card {
		background: #fff;
		border-radius: 16px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0,0,0,0.08);
		text-decoration: none;
		color: inherit;
		display: block;
		position: relative;
		transition: all 0.3s;
	}

	.favorite-card:hover {
		transform: translateY(-3px);
		box-shadow: 0 8px 30px rgba(0,0,0,0.12);
	}

	.favorite-type {
		display: inline-block;
		background: #F69532;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		padding: 4px 10px;
		border-radius: 4px;
		margin-bottom: 12px;
	}

	.favorite-card h3 {
		font-size: 18px;
		font-weight: 700;
		margin: 0 0 8px 0;
		color: #333;
	}

	.favorite-date {
		font-size: 13px;
		color: #888;
		margin: 0 0 12px 0;
	}

	.favorite-excerpt {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0 0 15px 0;
	}

	.remove-btn {
		background: #fee2e2;
		color: #991b1b;
		border: none;
		padding: 8px 16px;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.remove-btn:hover {
		background: #fecaca;
	}
</style>
