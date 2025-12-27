<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconTrendingUp,
		IconTrendingDown,
		IconMinus,
		IconRefresh,
		IconCalendar
	} from '$lib/icons';
	import { connections, isSeoConnected } from '$lib/stores/connections';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';

	let connectionLoading = $state(true);
	let stats: any = $state(null);
	let topPages: any[] = $state([]);
	let comparison: any = $state(null);
	let loading = $state(false);

	let dateRange = {
		start: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
		end: new Date().toISOString().split('T')[0]
	};

	onMount(async () => {
		// Load connection status first
		await connections.load();
		connectionLoading = false;

		// Only load data if SEO is connected
		if ($isSeoConnected) {
			loadData();
		}
	});

	async function loadData() {
		loading = true;
		try {
			await Promise.all([loadStats(), loadTopPages(), loadComparison()]);
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		const params = new URLSearchParams();
		if (dateRange.start) params.append('start_date', dateRange.start);
		if (dateRange.end) params.append('end_date', dateRange.end);

		try {
			const response = await fetch(`/api/seo/analytics/stats?${params}`);
			const data = await response.json();
			stats = data.stats;
		} catch (error) {
			console.error('Failed to load stats:', error);
		}
	}

	async function loadTopPages() {
		const params = new URLSearchParams();
		if (dateRange.start) params.append('start_date', dateRange.start);
		if (dateRange.end) params.append('end_date', dateRange.end);
		params.append('metric', 'clicks');
		params.append('limit', '10');

		try {
			const response = await fetch(`/api/seo/analytics/top-pages?${params}`);
			const data = await response.json();
			topPages = data.data || [];
		} catch (error) {
			console.error('Failed to load top pages:', error);
		}
	}

	async function loadComparison() {
		try {
			const response = await fetch('/api/seo/analytics/comparison');
			comparison = await response.json();
		} catch (error) {
			console.error('Failed to load comparison:', error);
		}
	}

	function formatNumber(num: number) {
		return new Intl.NumberFormat().format(num);
	}

	function getTrendIcon(direction: string) {
		if (direction === 'up') return IconTrendingUp;
		if (direction === 'down') return IconTrendingDown;
		return IconMinus;
	}

	function getTrendClass(direction: string) {
		if (direction === 'up') return 'positive';
		if (direction === 'down') return 'negative';
		return 'neutral';
	}
</script>

<svelte:head>
	<title>SEO Analytics | Admin</title>
</svelte:head>

<div class="analytics-page">
	<header class="page-header">
		<div>
			<h1>SEO Analytics</h1>
			<p>Track search performance and insights</p>
		</div>
		{#if $isSeoConnected}
			<button class="btn-secondary" onclick={loadData} disabled={loading}>
				<IconRefresh size={18} class={loading ? 'spinning' : ''} />
				Refresh
			</button>
		{/if}
	</header>

	<!-- Connection Check -->
	{#if connectionLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading...</p>
		</div>
	{:else if !$isSeoConnected}
		<ServiceConnectionStatus feature="seo" variant="card" showFeatures={true} />
	{:else}
		<div class="date-range-picker">
			<IconCalendar size={20} />
			<input type="date" bind:value={dateRange.start} onchange={loadData} />
			<span>to</span>
			<input type="date" bind:value={dateRange.end} onchange={loadData} />
		</div>

		{#if stats}
		<div class="metrics-grid">
			<div class="metric-card">
				<div class="metric-label">Total Impressions</div>
				<div class="metric-value">{formatNumber(stats.total_impressions || 0)}</div>
				{#if comparison?.change?.impressions}
					{@const ImpressionsIcon = getTrendIcon(comparison.change.impressions.direction)}
					<div class="metric-change {getTrendClass(comparison.change.impressions.direction)}">
						<ImpressionsIcon size={16} />
						{Math.abs(comparison.change.impressions.percentage)}%
					</div>
				{/if}
			</div>

			<div class="metric-card">
				<div class="metric-label">Total Clicks</div>
				<div class="metric-value">{formatNumber(stats.total_clicks || 0)}</div>
				{#if comparison?.change?.clicks}
					{@const ClicksIcon = getTrendIcon(comparison.change.clicks.direction)}
					<div class="metric-change {getTrendClass(comparison.change.clicks.direction)}">
						<ClicksIcon size={16} />
						{Math.abs(comparison.change.clicks.percentage)}%
					</div>
				{/if}
			</div>

			<div class="metric-card">
				<div class="metric-label">Average CTR</div>
				<div class="metric-value">{(stats.avg_ctr * 100).toFixed(2)}%</div>
				{#if comparison?.change?.ctr}
					{@const CtrIcon = getTrendIcon(comparison.change.ctr.direction)}
					<div class="metric-change {getTrendClass(comparison.change.ctr.direction)}">
						<CtrIcon size={16} />
						{Math.abs(comparison.change.ctr.percentage)}%
					</div>
				{/if}
			</div>

			<div class="metric-card">
				<div class="metric-label">Average Position</div>
				<div class="metric-value">{stats.avg_position ? stats.avg_position.toFixed(1) : '—'}</div>
				{#if comparison?.change?.position}
					{@const PositionIcon = getTrendIcon(comparison.change.position.direction)}
					<div class="metric-change {getTrendClass(comparison.change.position.direction)}">
						<PositionIcon size={16} />
						{Math.abs(comparison.change.position.percentage)}%
					</div>
				{/if}
			</div>
		</div>
	{/if}

	{#if topPages.length > 0}
		<div class="top-pages-section">
			<h2>Top Performing Pages</h2>
			<div class="pages-table">
				<table>
					<thead>
						<tr>
							<th>URL</th>
							<th>Impressions</th>
							<th>Clicks</th>
							<th>CTR</th>
							<th>Avg. Position</th>
						</tr>
					</thead>
					<tbody>
						{#each topPages as page}
							<tr>
								<td>
									<div class="url-cell">{page.url}</div>
								</td>
								<td class="number">{formatNumber(page.total_impressions || 0)}</td>
								<td class="number">{formatNumber(page.total_clicks || 0)}</td>
								<td class="number">{(page.avg_ctr * 100).toFixed(2)}%</td>
								<td class="number">{page.avg_position ? page.avg_position.toFixed(1) : '—'}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{:else if !loading}
		<div class="empty-state">
			<h3>No analytics data available</h3>
			<p>Data will appear once search engines begin indexing your pages</p>
		</div>
	{/if}
	{/if}
</div>

<style>
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		gap: 1rem;
	}

	.loading-state .spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e5e5;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	.loading-state p {
		color: #666;
	}

	.analytics-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.btn-secondary,
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-secondary {
		background: white;
		color: #666;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover:not(:disabled) {
		background: #f8f9fa;
	}

	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.date-range-picker {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		margin-bottom: 2rem;
		width: fit-content;
	}

	.date-range-picker input {
		padding: 0.5rem 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		font-size: 0.95rem;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-bottom: 3rem;
	}

	.metric-card {
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
	}

	.metric-label {
		font-size: 0.9rem;
		color: #666;
		margin-bottom: 0.75rem;
	}

	.metric-value {
		font-size: 2rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.metric-change {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.9rem;
		font-weight: 600;
	}

	.metric-change.positive {
		color: #16a34a;
	}

	.metric-change.negative {
		color: #dc2626;
	}

	.metric-change.neutral {
		color: #999;
	}

	.top-pages-section {
		margin-bottom: 3rem;
	}

	.top-pages-section h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1.5rem;
	}

	.pages-table {
		background: white;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
		overflow: hidden;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	th {
		text-align: left;
		padding: 1rem;
		background: #f8f9fa;
		border-bottom: 1px solid #e5e5e5;
		font-weight: 600;
		color: #1a1a1a;
		font-size: 0.9rem;
	}

	th:not(:first-child) {
		text-align: right;
	}

	td {
		padding: 1rem;
		border-bottom: 1px solid #f0f0f0;
		font-size: 0.95rem;
	}

	td.number {
		text-align: right;
		font-variant-numeric: tabular-nums;
	}

	tbody tr:hover {
		background: #f8f9fa;
	}

	.url-cell {
		color: #3b82f6;
		max-width: 400px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: 0.9rem;
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
	}

	.empty-state h3 {
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.empty-state p {
		color: #666;
		margin-bottom: 1.5rem;
	}
</style>
