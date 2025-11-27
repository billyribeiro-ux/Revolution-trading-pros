<script lang="ts">
	/**
	 * Dashboard - My Indicators Page
	 * Shows user's purchased indicators and access
	 */
	import SEOHead from '$lib/components/SEOHead.svelte';
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import {
		IconChartLine,
		IconDownload,
		IconExternalLink,
		IconCheck,
		IconClock,
		IconRefresh
	} from '@tabler/icons-svelte';

	// Redirect if not authenticated
	onMount(() => {
		if (!$isAuthenticated) {
			goto('/login?redirect=/dashboard/indicators');
		}
	});

	// Mock purchased indicators data
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

<SEOHead
	title="My Indicators"
	description="Access your purchased trading indicators and tools."
	noindex
/>

{#if $isAuthenticated}
	<main class="dashboard-page">
		<div class="container">
			<!-- Header -->
			<header class="page-header" in:fly={{ y: -20, duration: 400 }}>
				<div class="page-header__content">
					<h1 class="page-header__title">My Indicators</h1>
					<p class="page-header__subtitle">Access and manage your trading indicators</p>
				</div>
				<a href="/indicators" class="browse-btn">
					Browse All Indicators
				</a>
			</header>

			<!-- Indicators Grid -->
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

			{#if purchasedIndicators.length === 0}
				<div class="empty-state" in:fade>
					<IconChartLine size={64} />
					<h2>No Indicators Yet</h2>
					<p>You haven't purchased any indicators yet. Browse our collection to enhance your trading!</p>
					<a href="/indicators" class="empty-state__btn">Browse Indicators</a>
				</div>
			{/if}
		</div>
	</main>
{:else}
	<div class="loading-state">
		<p>Redirecting to login...</p>
	</div>
{/if}

<style>
	.dashboard-page {
		min-height: 100vh;
		background: var(--rtp-bg, #0a0f1a);
		color: var(--rtp-text, #e5e7eb);
		padding: 8rem 1.5rem 4rem;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 3rem;
		gap: 2rem;
		flex-wrap: wrap;
	}

	.page-header__title {
		font-size: 2.5rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 0.5rem;
	}

	.page-header__subtitle {
		color: #94a3b8;
		font-size: 1.1rem;
	}

	.browse-btn {
		padding: 0.75rem 1.5rem;
		background: rgba(139, 92, 246, 0.1);
		border: 1px solid rgba(139, 92, 246, 0.2);
		border-radius: 8px;
		color: #a78bfa;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.browse-btn:hover {
		background: rgba(139, 92, 246, 0.2);
	}

	.indicators-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
		gap: 1.5rem;
	}

	.indicator-card {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		padding: 1.5rem;
		transition: all 0.3s ease;
	}

	.indicator-card:hover {
		border-color: rgba(139, 92, 246, 0.3);
		transform: translateY(-2px);
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
		background: linear-gradient(135deg, rgba(139, 92, 246, 0.2), rgba(59, 130, 246, 0.2));
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #a78bfa;
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
		color: #fff;
		margin-bottom: 0.5rem;
	}

	.indicator-card__description {
		font-size: 0.875rem;
		color: #94a3b8;
		margin-bottom: 1.25rem;
		line-height: 1.5;
	}

	.indicator-card__meta {
		background: rgba(255, 255, 255, 0.02);
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
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.meta-label {
		color: #64748b;
	}

	.meta-value {
		color: #e5e7eb;
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
		background: linear-gradient(135deg, #8b5cf6, #6366f1);
		color: #fff;
	}

	.action-btn--primary:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(139, 92, 246, 0.3);
	}

	.action-btn--secondary {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		color: #e5e7eb;
	}

	.action-btn--secondary:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.action-btn--renew {
		background: rgba(250, 204, 21, 0.1);
		border: 1px solid rgba(250, 204, 21, 0.2);
		color: #facc15;
	}

	.action-btn--renew:hover {
		background: rgba(250, 204, 21, 0.2);
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #64748b;
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: #fff;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
	}

	.empty-state__btn {
		display: inline-flex;
		padding: 0.75rem 2rem;
		background: linear-gradient(135deg, #8b5cf6, #6366f1);
		border-radius: 8px;
		color: #fff;
		font-weight: 600;
		text-decoration: none;
	}

	.loading-state {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--rtp-bg, #0a0f1a);
		color: #94a3b8;
	}

	@media (max-width: 768px) {
		.dashboard-page {
			padding: 7rem 1rem 3rem;
		}

		.page-header {
			flex-direction: column;
		}

		.page-header__title {
			font-size: 2rem;
		}

		.indicators-grid {
			grid-template-columns: 1fr;
		}

		.indicator-card__actions {
			flex-direction: column;
		}
	}
</style>
