<script lang="ts">
	/**
	 * Weekly Watchlist - Latest Redirect
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Redirects to the most recent watchlist item
	 * @version 2.0.0 - December 2025 - Connected to API
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { watchlistApi } from '$lib/api/watchlist';

	let isLoading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		try {
			const response = await watchlistApi.getLatest();
			if (response.success && response.data) {
				await goto(`/watchlist/${response.data.slug}`, { replaceState: true });
				return;
			}
			// Fallback to default slug
			await goto('/watchlist/12222025-tg-watkins', { replaceState: true });
		} catch (err) {
			console.warn('Failed to fetch latest watchlist:', err);
			// Fallback to default slug
			await goto('/watchlist/12222025-tg-watkins', { replaceState: true });
		}
	});
</script>

<svelte:head>
	<title>Weekly Watchlist | Day Trading Room</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Weekly Watchlist</h1>
		<a href="/dashboard/day-trading-room" class="btn btn-xs btn-default">
			← Back to Dashboard
		</a>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			{#if isLoading}
				<div class="loading-state">
					<div class="loading-spinner"></div>
					<p>Loading latest watchlist...</p>
				</div>
			{:else if error}
				<div class="error-state">
					<p>{error}</p>
					<a href="/dashboard/day-trading-room" class="btn btn-default">Return to Dashboard</a>
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	.dashboard__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 30px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		gap: 15px;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		gap: 15px;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.btn {
		padding: 8px 16px;
		border-radius: 3px;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: all 0.2s;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e0e0e0;
	}

	.dashboard__content {
		padding: 30px;
	}

	.dashboard__content-section {
		background: #fff;
		padding: 30px;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0,0,0,0.1);
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #f3f3f3;
		border-top: 3px solid #0984ae;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}

	.loading-state p,
	.error-state p {
		font-size: 16px;
		color: #666;
		margin-bottom: 20px;
	}

	.error-state {
		text-align: center;
		padding: 40px 20px;
	}
</style>
