<script lang="ts">
	/**
	 * Dashboard - My Indicators Page
	 * Shows user's purchased indicators and access
	 */
	import { fly, fade } from 'svelte/transition';
	import {
		IconChartLine,
		IconDownload,
		IconExternalLink,
		IconCheck,
		IconClock,
		IconRefresh
	} from '@tabler/icons-svelte';

	// Mock purchased indicators data - will be fetched from API
	const purchasedIndicators = [
		{
			id: 1,
			name: 'Revolution Pro Scanner',
			description: 'Advanced market scanner with real-time alerts',
			platform: 'TradingView',
			status: 'active',
			expiresAt: '2025-12-31',
			downloadUrl: '#',
			version: '2.4.1'
		},
		{
			id: 2,
			name: 'Momentum Tracker',
			description: 'Identify momentum shifts before they happen',
			platform: 'ThinkorSwim',
			status: 'active',
			expiresAt: '2025-06-15',
			downloadUrl: '#',
			version: '1.8.0'
		},
		{
			id: 3,
			name: 'Support & Resistance Zones',
			description: 'Automatic S/R level detection',
			platform: 'TradingView',
			status: 'expired',
			expiresAt: '2024-11-01',
			downloadUrl: '#',
			version: '3.1.2'
		}
	];
</script>

<!-- Dashboard Header -->
<div class="wc-content-sction">
	<div class="dashb_headr">
		<div class="dashb_headr-left">
			<h1 class="dashb_pg-titl">My Indicators</h1>
		</div>
		<div class="dashb_headr-right">
			<a href="/indicators" class="btn btn-xs btn-link start-here-btn">
				Browse All Indicators
			</a>
		</div>
	</div>
</div>

<!-- Indicators Content -->
<div class="wc-accontent-inner">
	{#if purchasedIndicators.length > 0}
		<div class="indicators-grid">
			{#each purchasedIndicators as indicator, i (indicator.id)}
				<div
					class="indicator-card"
					class:indicator-card--expired={indicator.status === 'expired'}
					in:fly={{ y: 20, delay: 100 * i, duration: 400 }}
				>
					<div class="indicator-card__header">
						<div class="indicator-card__icon">
							<IconChartLine size={28} />
						</div>
						<div class="indicator-card__status" class:active={indicator.status === 'active'}>
							{#if indicator.status === 'active'}
								<IconCheck size={14} />
								Active
							{:else}
								<IconClock size={14} />
								Expired
							{/if}
						</div>
					</div>

					<h3 class="indicator-card__title">{indicator.name}</h3>
					<p class="indicator-card__description">{indicator.description}</p>

					<div class="indicator-card__meta">
						<div class="meta-row">
							<span class="meta-label">Platform:</span>
							<span class="meta-value">{indicator.platform}</span>
						</div>
						<div class="meta-row">
							<span class="meta-label">Version:</span>
							<span class="meta-value">v{indicator.version}</span>
						</div>
						<div class="meta-row">
							<span class="meta-label">{indicator.status === 'active' ? 'Expires:' : 'Expired:'}</span>
							<span class="meta-value">{indicator.expiresAt}</span>
						</div>
					</div>

					<div class="indicator-card__actions">
						{#if indicator.status === 'active'}
							<a href={indicator.downloadUrl} class="action-btn action-btn--primary">
								<IconDownload size={18} />
								Download
							</a>
							<a href="/docs/indicators" class="action-btn action-btn--secondary">
								<IconExternalLink size={18} />
								Documentation
							</a>
						{:else}
							<a href="/indicators/{indicator.id}" class="action-btn action-btn--renew">
								<IconRefresh size={18} />
								Renew License
							</a>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<div class="empty-state" in:fade>
			<IconChartLine size={64} />
			<h2>No Indicators Yet</h2>
			<p>You haven't purchased any indicators yet. Browse our collection to enhance your trading!</p>
			<a href="/indicators" class="btn btn-orange">Browse Indicators</a>
		</div>
	{/if}
</div>

<style>
	/* Content Section */
	.wc-content-sction {
		width: 100%;
		margin: auto;
		padding: 20px;
	}

	/* Dashboard Header */
	.dashb_headr {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		max-width: 100%;
		padding: 20px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
	}

	.dashb_headr-left,
	.dashb_headr-right {
		align-items: center;
		display: flex;
		flex-direction: row;
	}

	.dashb_pg-titl {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
	}

	.start-here-btn {
		font-size: 14px;
		line-height: 18px;
		padding: 8px 14px;
		font-weight: 600;
		color: #0984ae;
		background: #f4f4f4;
		border-color: transparent;
		text-decoration: none;
		border-radius: 5px;
		transition: all 0.15s ease-in-out;
	}

	.start-here-btn:hover {
		color: #0984ae;
		background: #e7e7e7;
	}

	/* Inner Content */
	.wc-accontent-inner {
		padding: 4% 2%;
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 1px 2px rgb(0 0 0 / 15%);
		position: relative;
		margin: 20px;
	}

	/* Indicators Grid */
	.indicators-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	/* Indicator Card */
	.indicator-card {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		padding: 1.5rem;
		transition: all 0.3s ease;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.indicator-card:hover {
		border-color: #0984ae;
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	.indicator-card--expired {
		opacity: 0.7;
	}

	.indicator-card__header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.indicator-card__icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, #e0f2fe, #dbeafe);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #0984ae;
	}

	.indicator-card__status {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
	}

	.indicator-card__status.active {
		background: rgba(16, 185, 129, 0.1);
		color: #10b981;
	}

	.indicator-card__title {
		font-size: 1.25rem;
		font-weight: 600;
		color: #333;
		margin-bottom: 0.5rem;
	}

	.indicator-card__description {
		font-size: 0.875rem;
		color: #64748b;
		margin-bottom: 1.25rem;
		line-height: 1.5;
	}

	.indicator-card__meta {
		background: #f8fafc;
		border-radius: 8px;
		padding: 1rem;
		margin-bottom: 1.25rem;
	}

	.meta-row {
		display: flex;
		justify-content: space-between;
		padding: 0.375rem 0;
		font-size: 0.8rem;
	}

	.meta-row:not(:last-child) {
		border-bottom: 1px solid #e5e7eb;
	}

	.meta-label {
		color: #64748b;
	}

	.meta-value {
		color: #333;
		font-weight: 500;
	}

	.indicator-card__actions {
		display: flex;
		gap: 0.75rem;
	}

	.action-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.action-btn--primary {
		background: #0984ae;
		color: #fff;
	}

	.action-btn--primary:hover {
		background: #076787;
	}

	.action-btn--secondary {
		background: #f4f4f4;
		border: 1px solid #e5e7eb;
		color: #333;
	}

	.action-btn--secondary:hover {
		background: #e5e7eb;
	}

	.action-btn--renew {
		background: #fef3c7;
		border: 1px solid #fcd34d;
		color: #92400e;
	}

	.action-btn--renew:hover {
		background: #fde68a;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #64748b;
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: #333;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
	}

	.btn-orange {
		display: inline-flex;
		padding: 0.75rem 2rem;
		background: #f99e31;
		border-radius: 8px;
		color: #fff;
		font-weight: 600;
		text-decoration: none;
	}

	.btn-orange:hover {
		background: #f88b09;
	}

	@media (max-width: 768px) {
		.indicators-grid {
			grid-template-columns: 1fr;
		}

		.indicator-card__actions {
			flex-direction: column;
		}
	}
</style>
