<script lang="ts">
	/**
	 * My Indicators Page - Member Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Trading indicators and tools library
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconCheck from '@tabler/icons-svelte/icons/check';

	let loading = $state(true);
	let membershipsData = $state<UserMembershipsResponse | null>(null);

	onMount(async () => {
		try {
			membershipsData = await getUserMemberships();
		} catch (err) {
			console.error('Failed to load indicators:', err);
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>My Indicators | Revolution Trading Pros</title>
</svelte:head>

<div class="indicators-page">
	<!-- Page Header -->
	<header class="page-header">
		<div class="header-left">
			<h1 class="page-title">My Indicators</h1>
			<p class="page-subtitle">Download and manage your trading indicators</p>
		</div>
	</header>

	<!-- Indicators Grid -->
	<div class="indicators-content">
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading your indicators...</p>
			</div>
		{:else if membershipsData?.indicators && membershipsData.indicators.length > 0}
			<div class="indicators-grid">
				{#each membershipsData.indicators as indicator (indicator.id)}
					<article class="indicator-card">
						<div class="indicator-header">
							<div class="indicator-icon">
								<DynamicIcon name={indicator.icon || 'chart-line'} size={28} />
							</div>
							<div class="indicator-badge">
								<IconCheck size={14} />
								Owned
							</div>
						</div>
						<div class="indicator-body">
							<h3 class="indicator-title">{indicator.name}</h3>
							<p class="indicator-description">
								Premium trading indicator for advanced market analysis
							</p>
						</div>
						<div class="indicator-actions">
							<button class="btn btn-primary">
								<IconDownload size={16} />
								Download
							</button>
							<a href="/dashboard/{indicator.slug}" class="btn btn-secondary">
								View Details
							</a>
						</div>
					</article>
				{/each}
			</div>
		{:else}
			<div class="empty-state">
				<div class="empty-icon">
					<IconChartCandle size={48} />
				</div>
				<h3>No Indicators Yet</h3>
				<p>You don't have any indicators. Browse our indicator library to enhance your trading.</p>
				<a href="/pricing" class="btn btn-primary">Browse Indicators</a>
			</div>
		{/if}
	</div>
</div>

<style>
	.indicators-page {
		padding: 30px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 32px;
	}

	.page-title {
		font-size: 28px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.page-subtitle {
		font-size: 14px;
		color: #666;
		margin: 0;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		gap: 16px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-state p {
		color: #666;
		font-size: 14px;
	}

	/* Indicators Grid */
	.indicators-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 24px;
	}

	.indicator-card {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		overflow: hidden;
		transition: all 0.15s;
		display: flex;
		flex-direction: column;
	}

	.indicator-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	.indicator-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 20px 0;
	}

	.indicator-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 12px;
		color: #fff;
	}

	.indicator-badge {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 4px 12px;
		background: #d4edda;
		color: #155724;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 600;
	}

	.indicator-body {
		padding: 20px;
		flex: 1;
	}

	.indicator-title {
		font-size: 18px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.indicator-description {
		font-size: 14px;
		color: #666;
		margin: 0;
		line-height: 1.5;
	}

	.indicator-actions {
		display: flex;
		gap: 8px;
		padding: 0 20px 20px;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		text-align: center;
		padding: 40px;
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		margin-bottom: 24px;
	}

	.empty-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.empty-state p {
		font-size: 14px;
		color: #666;
		margin: 0 0 24px;
		max-width: 400px;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 16px;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.15s;
		border: none;
		flex: 1;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover {
		background: #076787;
	}

	.btn-secondary {
		background: #e5e7eb;
		color: #374151;
	}

	.btn-secondary:hover {
		background: #d1d5db;
	}

	@media (max-width: 768px) {
		.indicators-page {
			padding: 20px;
		}

		.indicators-grid {
			grid-template-columns: 1fr;
		}

		.indicator-actions {
			flex-direction: column;
		}
	}
</style>
