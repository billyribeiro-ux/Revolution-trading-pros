<!--
	My Indicators Page
	═══════════════════════════════════════════════════════════════════════════
	Pixel-perfect match to WordPress implementation
	Reference: MyIndicatorsY and MyIndicatorsN HTML files
	
	Features:
	- Shows indicators when user has access
	- Empty state when user has no indicators
	- Breadcrumb: Home / Member Dashboard / My Indicators
	- Page title with dashboard__header wrapper
	- Indicator cards with platform links
	- Matches original WordPress structure exactly
-->
<script lang="ts">
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import { onMount } from 'svelte';
	
	interface Indicator {
		id: number;
		name: string;
		description: string;
		platform: string;
		platformUrl: string;
		icon: string;
		status: 'active' | 'inactive';
	}
	
	// Check if user has indicators - this would come from API/store
	let hasIndicators = false;
	let isLoading = true;
	
	// Mock data for indicators (replace with actual API call)
	let indicators: Indicator[] = [
		{
			id: 1,
			name: 'ThinkorSwim Indicators',
			description: 'Professional trading indicators for ThinkorSwim platform',
			platform: 'ThinkorSwim',
			platformUrl: 'https://www.thinkorswim.com',
			icon: 'fa-line-chart',
			status: 'active'
		},
		{
			id: 2,
			name: 'TradeStation Indicators',
			description: 'Advanced indicators for TradeStation platform',
			platform: 'TradeStation',
			platformUrl: 'https://www.tradestation.com',
			icon: 'fa-bar-chart',
			status: 'active'
		},
		{
			id: 3,
			name: 'NinjaTrader Indicators',
			description: 'Custom indicators for NinjaTrader platform',
			platform: 'NinjaTrader',
			platformUrl: 'https://ninjatrader.com',
			icon: 'fa-area-chart',
			status: 'active'
		}
	];
	
	onMount(async () => {
		// Simulate API call to fetch user's indicators
		// Replace with actual API call: const response = await fetch('/api/user/indicators');
		setTimeout(() => {
			hasIndicators = indicators.length > 0;
			isLoading = false;
		}, 500);
	});
</script>

<svelte:head>
	<title>My Indicators - Simpler Trading</title>
	<meta name="description" content="Access your trading indicators and tools" />
	<meta property="og:title" content="My Indicators" />
	<meta property="og:url" content="https://my.simplertrading.com/dashboard/indicators" />
	<meta property="og:type" content="article" />
</svelte:head>

<!-- Breadcrumbs -->
<DashboardBreadcrumbs />

<!-- Main Content -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<aside class="dashboard__sidebar">
			<!-- Sidebar content if needed -->
		</aside>
		
		<main class="dashboard__main">
			<!-- Page Header with dashboard__header wrapper (matches WordPress) -->
			<header class="dashboard__header">
				<div class="dashboard__header-left">
					<h1 class="dashboard__page-title">My Indicators</h1>
				</div>
			</header>
			
			<!-- Content Area -->
			<div class="dashboard__content">
				{#if isLoading}
					<!-- Loading State -->
					<div class="loading-state">
						<div class="spinner"></div>
						<p>Loading your indicators...</p>
					</div>
				{:else if !hasIndicators}
					<!-- Empty State - No Indicators -->
					<div class="empty-state">
						<div class="empty-state__icon">
							<span class="fa fa-line-chart"></span>
						</div>
						<h2 class="empty-state__title">No Indicators Yet</h2>
						<p class="empty-state__description">
							You don't have any indicators at the moment. Indicators will appear here once you gain access to them through your membership.
						</p>
						<div class="empty-state__actions">
							<a href="/dashboard" class="btn btn-primary">
								<span class="fa fa-arrow-left"></span>
								Back to Dashboard
							</a>
							<a href="https://www.simplertrading.com/products" class="btn btn-secondary" target="_blank" rel="noopener noreferrer">
								<span class="fa fa-shopping-cart"></span>
								Browse Products
							</a>
						</div>
					</div>
				{:else}
					<!-- Indicators Grid -->
					<div class="indicators-grid">
						{#each indicators as indicator (indicator.id)}
							<article class="indicator-card">
								<div class="indicator-card__header">
									<div class="indicator-card__icon">
										<span class="fa {indicator.icon}"></span>
									</div>
									<span class="indicator-card__status indicator-card__status--{indicator.status}">
										{indicator.status}
									</span>
								</div>
								<div class="indicator-card__body">
									<h3 class="indicator-card__title">{indicator.name}</h3>
									<p class="indicator-card__description">{indicator.description}</p>
									<div class="indicator-card__platform">
										<span class="fa fa-desktop"></span>
										<span>{indicator.platform}</span>
									</div>
								</div>
								<div class="indicator-card__footer">
									<a href={indicator.platformUrl} class="btn btn-primary btn-block" target="_blank" rel="noopener noreferrer">
										<span class="fa fa-external-link"></span>
										Access Platform
									</a>
									<a href="/dashboard/indicators/{indicator.id}" class="btn btn-secondary btn-block">
										<span class="fa fa-info-circle"></span>
										View Details
									</a>
								</div>
							</article>
						{/each}
					</div>
				{/if}
			</div>
		</main>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * My Indicators Page Styles
	 * Matches WordPress implementation exactly
	 * ═══════════════════════════════════════════════════════════════════════════ */
	
	.dashboard__header {
		justify-content: space-between;
		padding: 20px 0;
		margin-bottom: 30px;
	}
	
	.dashboard__header-left {
		flex: 1;
	}
	
	.dashboard__page-title {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}
	
	.dashboard__content {
		padding: 20px 0;
	}
	
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Empty State Styles
	 * Identical to "no classes" experience
	 * ═══════════════════════════════════════════════════════════════════════════ */
	
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		padding: 60px 20px;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
		min-height: 400px;
	}
	
	.empty-state__icon {
		width: 120px;
		height: 120px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #143E59 0%, #0984ae 100%);
		border-radius: 50%;
		margin-bottom: 30px;
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.2);
	}
	
	.empty-state__icon .fa {
		font-size: 56px;
		color: #fff;
	}
	
	.empty-state__title {
		font-size: 32px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}
	
	.empty-state__description {
		font-size: 16px;
		line-height: 1.6;
		color: #666;
		max-width: 500px;
		margin: 0 0 30px;
	}
	
	.empty-state__actions {
		display: flex;
		gap: 15px;
		flex-wrap: wrap;
		justify-content: center;
	}
	
	/* Button Styles */
	.btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		font-size: 16px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 6px;
		transition: all 0.2s ease;
		cursor: pointer;
		border: none;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}
	
	.btn-primary {
		background: #143E59;
		color: #fff;
	}
	
	.btn-primary:hover {
		background: #0f2d42;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
	}
	
	.btn-secondary {
		background: #fff;
		color: #143E59;
		border: 2px solid #143E59;
	}
	
	.btn-secondary:hover {
		background: #143E59;
		color: #fff;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.2);
	}
	
	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		min-height: 400px;
	}
	
	.spinner {
		width: 50px;
		height: 50px;
		border: 4px solid #f3f3f3;
		border-top: 4px solid #143E59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 20px;
	}
	
	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}
	
	.loading-state p {
		color: #666;
		font-size: 16px;
	}
	
	/* Indicators Grid */
	.indicators-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 30px;
		padding: 20px 0;
	}
	
	.indicator-card {
		background: #fff;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		overflow: hidden;
		transition: all 0.3s ease;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
		display: flex;
		flex-direction: column;
	}
	
	.indicator-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}
	
	.indicator-card__header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		background: linear-gradient(135deg, #143E59 0%, #0984ae 100%);
	}
	
	.indicator-card__icon {
		width: 50px;
		height: 50px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 8px;
	}
	
	.indicator-card__icon .fa {
		font-size: 24px;
		color: #fff;
	}
	
	.indicator-card__status {
		padding: 4px 12px;
		border-radius: 12px;
		font-size: 12px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}
	
	.indicator-card__status--active {
		background: #4caf50;
		color: #fff;
	}
	
	.indicator-card__status--inactive {
		background: #f44336;
		color: #fff;
	}
	
	.indicator-card__body {
		padding: 20px;
		flex: 1;
	}
	
	.indicator-card__title {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}
	
	.indicator-card__description {
		font-size: 14px;
		line-height: 1.6;
		color: #666;
		margin: 0 0 15px;
	}
	
	.indicator-card__platform {
		display: flex;
		align-items: center;
		gap: 8px;
		color: #143E59;
		font-size: 14px;
		font-weight: 600;
	}
	
	.indicator-card__platform .fa {
		font-size: 16px;
	}
	
	.indicator-card__footer {
		padding: 20px;
		background: #f9f9f9;
		border-top: 1px solid #e5e5e5;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}
	
	.btn-block {
		width: 100%;
		justify-content: center;
	}
	
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Styles
	 * ═══════════════════════════════════════════════════════════════════════════ */
	
	@media (max-width: 768px) {
		.dashboard__page-title {
			font-size: 24px;
		}
		
		.empty-state {
			padding: 40px 20px;
			min-height: 300px;
		}
		
		.empty-state__icon {
			width: 100px;
			height: 100px;
			margin-bottom: 20px;
		}
		
		.empty-state__icon .fa {
			font-size: 48px;
		}
		
		.empty-state__title {
			font-size: 24px;
		}
		
		.empty-state__description {
			font-size: 14px;
		}
		
		.empty-state__actions {
			flex-direction: column;
			width: 100%;
			max-width: 300px;
		}
		
		.btn {
			width: 100%;
			justify-content: center;
		}
		
		.indicators-grid {
			grid-template-columns: 1fr;
			gap: 20px;
		}
	}
	
	@media (min-width: 769px) and (max-width: 1024px) {
		.indicators-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
