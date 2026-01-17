<script lang="ts">
	/**
	 * Watchlist - Explosive Swings
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Displays weekly trade plan entries fetched from backend API
	 *
	 * @version 2.0.0 - ICT 11 Principal Engineer Grade
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import type { TradePlanEntry } from '$lib/types/trading';

	const ROOM_SLUG = 'explosive-swings';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE - Svelte 5 $state runes
	// ═══════════════════════════════════════════════════════════════════════════

	let tradePlanEntries = $state<TradePlanEntry[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let weekOf = $state<string | null>(null);
	let dataSource = $state<'backend' | 'mock' | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// API FETCH
	// ═══════════════════════════════════════════════════════════════════════════

	async function fetchTradePlan() {
		isLoading = true;
		error = null;
		try {
			const response = await fetch(`/api/trade-plans/${ROOM_SLUG}`);
			const data = await response.json();
			if (data.success) {
				tradePlanEntries = data.data || [];
				weekOf = data.week_of || null;
				dataSource = data._source || 'backend';
			} else {
				error = 'Failed to load trade plan';
			}
		} catch (err) {
			console.error('Failed to fetch trade plan:', err);
			error = 'Failed to load trade plan';
		} finally {
			isLoading = false;
		}
	}

	onMount(() => {
		fetchTradePlan();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDate(dateStr: string | null | undefined): string {
		if (!dateStr) return '';
		try {
			const date = new Date(dateStr);
			return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
		} catch {
			return dateStr;
		}
	}

	function formatWeekOf(dateStr: string | null): string {
		if (!dateStr) return '';
		try {
			const date = new Date(dateStr);
			return date.toLocaleDateString('en-US', {
				weekday: 'long',
				month: 'long',
				day: 'numeric',
				year: 'numeric'
			});
		} catch {
			return dateStr;
		}
	}
</script>

<svelte:head>
	<title>This Week's Watchlist | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
/>

<div class="watchlist-page">
	<div class="page-header">
		<h1>This Week's Watchlist</h1>
		{#if weekOf}
			<p>Week of {formatWeekOf(weekOf)}</p>
		{:else}
			<p>Complete trade plan with entries, targets, stops, and options plays</p>
		{/if}
		{#if dataSource === 'mock'}
			<span class="data-source-badge">Demo Data</span>
		{/if}
	</div>

	<!-- Loading State -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading watchlist...</p>
		</div>
	{:else if error}
		<!-- Error State -->
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => fetchTradePlan()}>Retry</button>
		</div>
	{:else if tradePlanEntries.length === 0}
		<!-- Empty State -->
		<div class="empty-state">
			<p>No trade plan entries for this week yet.</p>
			<p class="subtext">Check back soon for new setups.</p>
		</div>
	{:else}
		<!-- Watchlist Grid -->
		<div class="watchlist-grid">
			{#each tradePlanEntries as stock (stock.id)}
				<div class="stock-card">
					<div class="stock-header">
						<div class="stock-title">
							<h2>{stock.ticker}</h2>
						</div>
					</div>

					<div class="bias-badge bias--{stock.bias.toLowerCase()}">
						{stock.bias}
					</div>

					<div class="trade-plan">
						<div class="plan-row">
							<span class="label">Entry:</span>
							<span class="value entry">{stock.entry || '-'}</span>
						</div>
						<div class="plan-row">
							<span class="label">Target 1:</span>
							<span class="value target">{stock.target1 || '-'}</span>
						</div>
						<div class="plan-row">
							<span class="label">Target 2:</span>
							<span class="value target">{stock.target2 || '-'}</span>
						</div>
						<div class="plan-row">
							<span class="label">Target 3:</span>
							<span class="value target">{stock.target3 || '-'}</span>
						</div>
						{#if stock.runner}
							<div class="plan-row">
								<span class="label">Runner:</span>
								<span class="value runner">{stock.runner}</span>
							</div>
						{/if}
						<div class="plan-row">
							<span class="label">Stop:</span>
							<span class="value stop">{stock.stop || '-'}</span>
						</div>
					</div>

					{#if stock.options_strike || stock.options_exp}
						<div class="options-section">
							<h4>Options Play</h4>
							<div class="options-info">
								<span>{stock.options_strike || '-'}</span>
								<span>Exp: {formatDate(stock.options_exp)}</span>
							</div>
						</div>
					{/if}

					<div class="status-badge status--{stock.is_active ? 'active' : 'inactive'}">
						{stock.is_active ? 'ACTIVE' : 'INACTIVE'}
					</div>

					{#if stock.notes}
						<div class="notes-section">
							<h4>Trade Notes</h4>
							<p>{stock.notes}</p>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}

	<!-- Back Link -->
	<div class="back-link">
		<a href="/dashboard/explosive-swings">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="m15 18-6-6 6-6" />
			</svg>
			Back to Dashboard
		</a>
	</div>
</div>

<style>
	.watchlist-page {
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

	.data-source-badge {
		display: inline-block;
		margin-top: 12px;
		padding: 4px 12px;
		background: #fef3c7;
		color: #92400e;
		border-radius: 9999px;
		font-size: 12px;
		font-weight: 600;
	}

	/* Loading State */
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
		to {
			transform: rotate(360deg);
		}
	}

	.loading-state p {
		margin-top: 16px;
		color: #666;
	}

	/* Error State */
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
		transition: background 0.2s;
	}

	.error-state button:hover {
		background: #dc2626;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 80px 20px;
		background: #f8fafc;
		border-radius: 12px;
		max-width: 500px;
		margin: 0 auto;
	}

	.empty-state p {
		color: #666;
		margin: 0;
	}

	.empty-state .subtext {
		color: #999;
		font-size: 14px;
		margin-top: 8px;
	}

	.watchlist-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 30px;
		max-width: 1400px;
		margin: 0 auto;
	}

	@media (min-width: 768px) {
		.watchlist-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1200px) {
		.watchlist-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.stock-card {
		background: #fff;
		border-radius: 16px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
		transition: all 0.3s ease;
		position: relative;
	}

	.stock-card:hover {
		transform: translateY(-5px);
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
	}

	.stock-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 15px;
	}

	.stock-title h2 {
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 5px 0;
		color: #143e59;
		font-family: 'Montserrat', sans-serif;
	}

	.bias-badge {
		display: inline-block;
		padding: 6px 14px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 700;
		margin-bottom: 20px;
	}

	.bias--bullish {
		background: #dcfce7;
		color: #166534;
	}

	.bias--bearish {
		background: #fee2e2;
		color: #991b1b;
	}

	.bias--neutral {
		background: #fef3c7;
		color: #92400e;
	}

	.trade-plan {
		background: #f8fafc;
		border-radius: 12px;
		padding: 15px;
		margin-bottom: 20px;
	}

	.plan-row {
		display: flex;
		justify-content: space-between;
		padding: 8px 0;
		border-bottom: 1px solid #e5e7eb;
	}

	.plan-row:last-child {
		border-bottom: none;
	}

	.plan-row .label {
		font-size: 13px;
		color: #666;
		font-weight: 600;
	}

	.plan-row .value {
		font-size: 14px;
		font-weight: 700;
	}

	.plan-row .entry {
		color: #143e59;
	}

	.plan-row .target {
		color: #22c55e;
	}

	.plan-row .runner {
		color: #0ea5e9;
	}

	.plan-row .stop {
		color: #ef4444;
	}

	.options-section {
		background: #f3f4f6;
		border-radius: 8px;
		padding: 12px;
		margin-bottom: 15px;
	}

	.options-section h4 {
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		color: #666;
		margin: 0 0 8px 0;
	}

	.options-info {
		display: flex;
		justify-content: space-between;
		font-size: 14px;
		font-weight: 600;
		color: #7c3aed;
	}

	.status-badge {
		position: absolute;
		top: 20px;
		right: 20px;
		padding: 5px 12px;
		border-radius: 20px;
		font-size: 10px;
		font-weight: 700;
	}

	.status--active {
		background: #dcfce7;
		color: #166534;
	}

	.status--inactive {
		background: #f3f4f6;
		color: #6b7280;
	}

	.notes-section {
		margin-bottom: 15px;
	}

	.notes-section h4 {
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		color: #666;
		margin: 0 0 8px 0;
	}

	.notes-section p {
		font-size: 13px;
		color: #555;
		line-height: 1.6;
		margin: 0;
	}

	.back-link {
		max-width: 1400px;
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
</style>
