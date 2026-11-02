<script lang="ts">
	/**
	 * Attribution Report - Multi-Touch Attribution Analysis
	 *
	 * Compare attribution models and analyze marketing
	 * channel performance across the customer journey.
	 */
	import { onMount } from 'svelte';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconMapPin from '@tabler/icons-svelte-runes/icons/map-pin';
	import { browser } from '$app/env';
	import { analyticsApi, type AttributionReport } from '#lib/api/analytics.js';
	import AttributionChart from '#lib/components/analytics/AttributionChart.svelte';
	import PeriodSelector from '#lib/components/analytics/PeriodSelector.svelte';

	let report = $state<AttributionReport | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('30d');
	let selectedModel = $state('linear');

	const models = [
		{
			value: 'first_touch',
			label: 'First Touch',
			description: 'Credits the first interaction'
		},

		{
			value: 'last_touch',
			label: 'Last Touch',
			description: 'Credits the last interaction'
		},

		{
			value: 'linear',
			label: 'Linear',
			description: 'Equal credit to all touchpoints'
		},

		{
			value: 'time_decay',
			label: 'Time Decay',
			description: 'More credit to recent touchpoints'
		},

		{
			value: 'position_based',
			label: 'Position Based',
			description: '40% first, 40% last, 20% middle'
		}
	];

	async function loadAttribution() {
		loading = true;
		error = null;
		try {
			report = await analyticsApi.getChannelAttribution(selectedModel, selectedPeriod);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load attribution data';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		loadAttribution();
	}

	function handleModelChange() {
		loadAttribution();
	}

	function formatCurrency(num: number): string {
		if (num >= 1000000) return '$' + (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return '$' + (num / 1000).toFixed(1) + 'K';
		return '$' + num.toFixed(0);
	}

	// FIX-2026-04-26 (audit 08-analytics §P1-1): use `onMount` instead of
	// `$effect` for one-shot init. The previous `$effect` would re-run if any
	// `$state` read added inside `loadAttribution` participated in tracking,
	// producing the cascade that 12 other pages were explicitly migrated away
	// from in commit 34a0bd070.
	onMount(() => {
		if (browser) loadAttribution();
	});

	// Calculate totals
	const totalRevenue = $derived(
		report?.channels?.reduce((sum, c) => sum + c.attributed_revenue, 0) || 0
	);

	const totalConversions = $derived(
		report?.channels?.reduce((sum, c) => sum + c.attributed_conversions, 0) || 0
	);
	const totalTouchpoints = $derived(
		report?.channels?.reduce((sum, c) => sum + c.touchpoints, 0) || 0
	);

	function getModelClass(modelValue: string): Record<string, boolean> {
		return {
			'model-card': true,
			'model-card--selected': selectedModel === modelValue
		};
	}

	function getRevenueShareWidth(share: number): string {
		return `${Math.min(Math.max(share, 0), 100).toFixed(1)}%`;
	}
</script>

<svelte:head>
	<title>Attribution Report | Analytics</title>
</svelte:head>

<div class="attribution-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Attribution Analysis</h1>
			<p>Multi-touch attribution across marketing channels</p>
		</div>
		<div class="header-actions">
			<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
			<button type="button" class="secondary-button"> Export Report </button>
		</div>
	</div>

	<!-- Model Selection -->
	<div class="panel model-panel">
		<h3 class="section-title">Attribution Model</h3>
		<div class="model-grid">
			{#each models as model (model.value)}
				<button
					type="button"
					onclick={() => {
						selectedModel = model.value;
						handleModelChange();
					}}
					class={getModelClass(model.value)}
				>
					<div class="model-name">{model.label}</div>
					<div class="model-description">{model.description}</div>
				</button>
			{/each}
		</div>
	</div>

	{#if loading}
		<div class="loading-state">
			<div class="loader"></div>
		</div>
	{:else if error}
		<div class="error-card">
			<p>{error}</p>
			<button type="button" onclick={loadAttribution}> Retry </button>
		</div>
	{:else if report}
		<!-- Summary Stats -->
		<div class="summary-grid">
			<div class="summary-card">
				<div class="summary-label">Attributed Revenue</div>
				<div class="summary-value">{formatCurrency(totalRevenue)}</div>
			</div>
			<div class="summary-card">
				<div class="summary-label">Total Conversions</div>
				<div class="summary-value">{totalConversions.toLocaleString()}</div>
			</div>
			<div class="summary-card">
				<div class="summary-label">Total Touchpoints</div>
				<div class="summary-value">{totalTouchpoints.toLocaleString()}</div>
			</div>
			<div class="summary-card">
				<div class="summary-label">Avg Touchpoints/Conv</div>
				<div class="summary-value">
					{totalConversions > 0 ? (totalTouchpoints / totalConversions).toFixed(1) : '0'}
				</div>
			</div>
		</div>

		<!-- Attribution Chart -->
		{#if report.channels && report.channels.length > 0}
			<div class="chart-section">
				<AttributionChart channels={report.channels} model={selectedModel} />
			</div>

			<!-- Detailed Table -->
			<div class="table-panel">
				<div class="table-header">
					<h3>Channel Performance</h3>
				</div>
				<div class="table-scroll">
					<table>
						<thead>
							<tr>
								<th>Channel</th>
								<th class="numeric-cell">Touchpoints</th>
								<th class="numeric-cell">Conversions</th>
								<th class="numeric-cell">Revenue</th>
								<th class="numeric-cell">Revenue Share</th>
								<th class="numeric-cell">Avg Order Value</th>
								<th class="numeric-cell">Assisted Conv.</th>
							</tr>
						</thead>
						<tbody>
							{#each report.channels as channel (channel.channel)}
								<tr>
									<td>
										<span class="channel-name">{channel.channel}</span>
									</td>
									<td class="numeric-cell muted-cell">
										{channel.touchpoints.toLocaleString()}
									</td>
									<td class="numeric-cell muted-cell">
										{channel.attributed_conversions.toFixed(1)}
									</td>
									<td class="numeric-cell strong-cell">
										{formatCurrency(channel.attributed_revenue)}
									</td>
									<td class="numeric-cell">
										<div class="share-cell">
											<div class="share-track">
												<div
													class="share-bar"
													style:width={getRevenueShareWidth(channel.revenue_share)}
												></div>
											</div>
											<span>{channel.revenue_share.toFixed(1)}%</span>
										</div>
									</td>
									<td class="numeric-cell muted-cell">
										{channel.attributed_conversions > 0
											? formatCurrency(channel.attributed_revenue / channel.attributed_conversions)
											: '-'}
									</td>
									<td class="numeric-cell muted-cell">
										{channel.assisted_conversions.toLocaleString()}
									</td>
								</tr>
							{/each}
						</tbody>
						<tfoot>
							<tr>
								<td>Total</td>
								<td class="numeric-cell">{totalTouchpoints.toLocaleString()}</td>
								<td class="numeric-cell">{totalConversions.toFixed(1)}</td>
								<td class="numeric-cell">{formatCurrency(totalRevenue)}</td>
								<td class="numeric-cell">100%</td>
								<td class="numeric-cell">
									{totalConversions > 0 ? formatCurrency(totalRevenue / totalConversions) : '-'}
								</td>
								<td class="numeric-cell">
									{report.channels
										.reduce((sum, c) => sum + c.assisted_conversions, 0)
										.toLocaleString()}
								</td>
							</tr>
						</tfoot>
					</table>
				</div>
			</div>

			<!-- Conversion Paths -->
			{#if report.conversion_paths && report.conversion_paths.length > 0}
				<div class="panel paths-panel">
					<h3 class="section-title">Top Conversion Paths</h3>
					<div class="path-stack">
						{#each report.conversion_paths.slice(0, 10) as path, i (i)}
							<div class="path-row">
								<span class="path-rank">{i + 1}.</span>
								<div class="path-channels">
									{#each path.channels as channel, j (j)}
										<span class="path-chip">
											{channel}
										</span>
										{#if j < path.channels.length - 1}
											<span class="path-arrow"><IconArrowRight size={14} aria-hidden="true" /></span>
										{/if}
									{/each}
								</div>
								<div class="path-result">
									<div>{path.conversions} conv</div>
									<div>{formatCurrency(path.revenue)}</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		{:else}
			<div class="empty-card">
				<div class="empty-icon"><IconMapPin size={40} aria-hidden="true" /></div>
				<h3>No Attribution Data</h3>
				<p>Attribution data will appear once conversions are tracked</p>
			</div>
		{/if}
	{/if}
</div>

<style>
	.attribution-page {
		max-width: 80rem;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	.page-header,
	.header-actions,
	.share-cell,
	.path-row,
	.path-channels {
		display: flex;
		align-items: center;
	}

	.page-header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		margin: 0;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.25;
	}

	.page-header p {
		margin: 0.25rem 0 0;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.header-actions {
		gap: 1rem;
	}

	.secondary-button,
	.error-card button,
	.model-card {
		border-radius: 8px;
		font: inherit;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			border-color 0.2s ease,
			color 0.2s ease;
	}

	.secondary-button {
		border: 1px solid var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
		color: var(--text-primary, #f0f6fc);
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.secondary-button:hover {
		background: var(--bg-elevated, #161b22);
	}

	.panel,
	.summary-card,
	.table-panel,
	.empty-card,
	.error-card {
		border: 1px solid var(--border-default, #30363d);
		border-radius: 12px;
		background: var(--bg-elevated, #161b22);
	}

	.model-panel,
	.summary-grid,
	.chart-section,
	.table-panel {
		margin-bottom: 2rem;
	}

	.panel {
		padding: 1.5rem;
	}

	.section-title {
		margin: 0 0 1rem;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.125rem;
		font-weight: 700;
		line-height: 1.4;
	}

	.model-grid,
	.summary-grid {
		display: grid;
		grid-template-columns: minmax(0, 1fr);
		gap: 1rem;
	}

	.model-card {
		border: 2px solid var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
		padding: 1rem;
		text-align: left;
	}

	.model-card:hover {
		border-color: var(--border-default, #30363d);
	}

	.model-card--selected,
	.model-card--selected:hover {
		border-color: #2563eb;
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
	}

	.model-name {
		margin-bottom: 0.25rem;
		color: var(--text-primary, #f0f6fc);
		font-weight: 600;
		line-height: 1.4;
	}

	.model-description {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
		line-height: 1.5;
	}

	.loading-state {
		display: grid;
		padding-block: 5rem;
		place-items: center;
	}

	.loader {
		width: 2rem;
		height: 2rem;
		border: 4px solid #2563eb;
		border-top-color: transparent;
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.error-card,
	.empty-card {
		padding: 1.5rem;
		text-align: center;
	}

	.error-card p {
		margin: 0;
		color: var(--error-emphasis, #f85149);
	}

	.error-card button {
		margin-top: 1rem;
		border: 0;
		background: #dc2626;
		color: #ffffff;
		padding: 0.5rem 1rem;
	}

	.error-card button:hover {
		background: #b91c1c;
	}

	.summary-card {
		padding: 1.5rem;
	}

	.summary-label {
		margin-bottom: 0.25rem;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.summary-value {
		color: var(--text-primary, #f0f6fc);
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.table-panel {
		overflow: hidden;
	}

	.table-header {
		border-bottom: 1px solid var(--border-default, #30363d);
		padding: 1rem;
	}

	.table-header h3 {
		margin: 0;
		color: var(--text-primary, #f0f6fc);
		font-weight: 700;
		line-height: 1.4;
	}

	.table-scroll {
		overflow-x: auto;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	thead,
	tfoot {
		background: var(--bg-elevated, #161b22);
	}

	tbody tr {
		border-top: 1px solid var(--border-default, #30363d);
	}

	tbody tr:hover {
		background: var(--bg-elevated, #161b22);
	}

	th,
	td {
		padding: 0.75rem 1rem;
		text-align: left;
		white-space: nowrap;
	}

	th {
		color: var(--text-tertiary, #6e7681);
		font-weight: 600;
	}

	tfoot td {
		color: var(--text-primary, #f0f6fc);
		font-weight: 600;
	}

	.numeric-cell {
		text-align: right;
	}

	.channel-name {
		color: var(--text-primary, #f0f6fc);
		font-weight: 600;
		text-transform: capitalize;
	}

	.muted-cell,
	.share-cell {
		color: var(--text-tertiary, #6e7681);
	}

	.strong-cell {
		color: var(--text-primary, #f0f6fc);
		font-weight: 600;
	}

	.share-cell {
		justify-content: flex-end;
		gap: 0.5rem;
	}

	.share-track {
		width: 4rem;
		height: 0.5rem;
		overflow: hidden;
		border-radius: 999px;
		background: var(--bg-surface, #1c2128);
	}

	.share-bar {
		height: 100%;
		border-radius: inherit;
		background: #3b82f6;
	}

	.paths-panel {
		margin-bottom: 0;
	}

	.path-stack {
		display: grid;
		gap: 0.75rem;
	}

	.path-row {
		gap: 1rem;
		border-radius: 8px;
		background: var(--bg-elevated, #161b22);
		padding: 0.75rem;
	}

	.path-rank {
		width: 1.5rem;
		flex: 0 0 auto;
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
	}

	.path-channels {
		flex: 1 1 auto;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.path-chip {
		border: 1px solid var(--border-default, #30363d);
		border-radius: 4px;
		background: var(--bg-elevated, #161b22);
		padding: 0.25rem 0.5rem;
		font-size: 0.875rem;
		text-transform: capitalize;
	}

	.path-arrow {
		display: inline-flex;
		color: var(--text-secondary, #8b949e);
	}

	.path-result {
		text-align: right;
	}

	.path-result div:first-child {
		color: var(--text-primary, #f0f6fc);
		font-weight: 600;
		line-height: 1.4;
	}

	.path-result div:last-child {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.empty-card {
		padding: 3rem;
	}

	.empty-icon {
		display: flex;
		justify-content: center;
		margin-bottom: 1rem;
		color: var(--text-tertiary);
		line-height: 1;
	}

	.empty-card h3 {
		margin: 0 0 0.5rem;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.empty-card p {
		margin: 0;
		color: var(--text-tertiary, #6e7681);
		line-height: 1.5;
	}

	@media (min-width: 640px) {
		.attribution-page {
			padding-inline: 1.5rem;
		}
	}

	@media (min-width: 768px) {
		.summary-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.attribution-page {
			padding-inline: 2rem;
		}

		.model-grid {
			grid-template-columns: repeat(5, minmax(0, 1fr));
		}
	}

	@media (max-width: 767.98px) {
		.page-header {
			align-items: flex-start;
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			align-items: stretch;
			flex-direction: column;
		}
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
