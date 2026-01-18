<script lang="ts">
	/**
	 * Favorites Page - Small Account Mentorship
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Displays user's bookmarked alerts and videos with real persistence
	 *
	 * @version 2.0.0 - ICT 11 Principal Engineer Grade
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	const ROOM_SLUG = 'small-account-mentorship';

	interface Favorite {
		id: number;
		item_type: string;
		item_id: number;
		title: string | null;
		excerpt: string | null;
		href: string | null;
		thumbnail_url: string | null;
		created_at: string;
	}

	let favorites = $state<Favorite[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	async function fetchFavorites() {
		isLoading = true;
		error = null;
		try {
			const res = await fetch(`/api/favorites?room_slug=${ROOM_SLUG}`);
			if (res.ok) {
				const data = await res.json();
				favorites = data.data || [];
			} else {
				error = 'Failed to load favorites';
			}
		} catch (err) {
			console.error('Failed to fetch favorites:', err);
			error = 'Failed to load favorites';
		} finally {
			isLoading = false;
		}
	}

	async function removeFavorite(id: number) {
		try {
			const res = await fetch(`/api/favorites/${id}`, { method: 'DELETE' });
			if (res.ok) {
				favorites = favorites.filter(f => f.id !== id);
			}
		} catch (err) {
			console.error('Failed to remove favorite:', err);
		}
	}

	function formatDate(dateStr: string): string {
		try {
			return new Date(dateStr).toLocaleDateString('en-US', {
				month: 'long',
				day: 'numeric',
				year: 'numeric'
			});
		} catch {
			return dateStr;
		}
	}

	onMount(() => {
		fetchFavorites();
	});
</script>

<svelte:head>
	<title>Favorites | Small Account Mentorship</title>
</svelte:head>

<TradingRoomHeader
	roomName="Small Account Mentorship"
	startHereUrl="/dashboard/small-account-mentorship/start-here"
/>

<div class="favorites-page">
	<div class="page-header">
		<h1>⭐ Your Favorites</h1>
		<p>Bookmarked videos and resources for quick access</p>
	</div>

	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading favorites...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => fetchFavorites()}>Retry</button>
		</div>
	{:else if favorites.length === 0}
		<div class="empty-state">
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				width="64"
				height="64"
			>
				<path
					d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"
				/>
			</svg>
			<h2>No favorites yet</h2>
			<p>Click the star icon on any video or resource to save it here</p>
			<a href="/dashboard/small-account-mentorship" class="back-btn">Browse Content</a>
		</div>
	{:else}
		<div class="favorites-list">
			{#each favorites as item (item.id)}
				<a href={item.href || '#'} class="favorite-card">
					{#if item.thumbnail_url}
						<div class="favorite-thumbnail" style="background-image: url('{item.thumbnail_url}')"></div>
					{/if}
					<div class="favorite-content">
						<div class="favorite-type">{item.item_type.toUpperCase().replace('_', ' ')}</div>
						<h3>{item.title || 'Untitled'}</h3>
						<p class="favorite-date">{formatDate(item.created_at)}</p>
						{#if item.excerpt}
							<p class="favorite-excerpt">{item.excerpt}</p>
						{/if}
					</div>
					<button
						class="remove-btn"
						onclick={(e) => {
							e.preventDefault();
							e.stopPropagation();
							removeFavorite(item.id);
						}}
					>
						Remove
					</button>
				</a>
			{/each}
		</div>
	{/if}

	<div class="back-link">
		<a href="/dashboard/small-account-mentorship">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="m15 18-6-6 6-6"/>
			</svg>
			Back to Dashboard
		</a>
	</div>
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

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px 20px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 4px solid #e5e7eb;
		border-top-color: #f69532;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.loading-state p {
		margin-top: 16px;
		color: #666;
	}

	.error-state {
		text-align: center;
		padding: 60px 20px;
		background: #fef2f2;
		border-radius: 12px;
		max-width: 500px;
		margin: 0 auto;
	}

	.error-state p {
		color: #991b1b;
		margin: 0 0 16px 0;
	}

	.error-state button {
		padding: 10px 24px;
		background: #ef4444;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
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
		margin: 0 0 20px 0;
	}

	.back-btn {
		display: inline-block;
		padding: 12px 24px;
		background: #143e59;
		color: white;
		text-decoration: none;
		border-radius: 8px;
		font-weight: 600;
		transition: background 0.2s;
	}

	.back-btn:hover {
		background: #0f2d42;
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
		overflow: hidden;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
		text-decoration: none;
		color: inherit;
		display: flex;
		position: relative;
		transition: all 0.3s;
	}

	.favorite-card:hover {
		transform: translateY(-3px);
		box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
	}

	.favorite-thumbnail {
		width: 160px;
		min-height: 120px;
		background-size: cover;
		background-position: center;
		flex-shrink: 0;
	}

	.favorite-content {
		padding: 20px;
		flex: 1;
	}

	.favorite-type {
		display: inline-block;
		background: #f69532;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		padding: 4px 10px;
		border-radius: 4px;
		margin-bottom: 8px;
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
		margin: 0 0 8px 0;
	}

	.favorite-excerpt {
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		margin: 0;
	}

	.remove-btn {
		position: absolute;
		top: 16px;
		right: 16px;
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

	.back-link {
		max-width: 900px;
		margin: 40px auto 0;
	}

	.back-link a {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #666;
		text-decoration: none;
		font-size: 14px;
		transition: color 0.2s;
	}

	.back-link a:hover {
		color: #143e59;
	}

	@media (max-width: 640px) {
		.favorite-card {
			flex-direction: column;
		}

		.favorite-thumbnail {
			width: 100%;
			height: 160px;
		}
	}
</style>
