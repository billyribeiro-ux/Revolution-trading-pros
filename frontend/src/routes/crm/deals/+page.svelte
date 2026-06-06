<!--
	URL: /crm/deals
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '$lib/api/crm';
	import { crmStore } from '$lib/stores/crm.svelte';
	import type { Deal, Pipeline, Stage } from '$lib/crm/types';
	import { IconActivity, IconArrowRight, IconCurrencyDollar } from '$lib/icons';
	import { logger } from '$lib/utils/logger';

	let forecastPeriod = $state('this_month');
	let forecast = $state<{
		commit: number;
		best_case: number;
		pipeline: number;
		worst_case: number;
	} | null>(null);

	onMount(async () => {
		await loadPipelinesAndDeals();
		await loadForecast();
	});

	async function loadPipelinesAndDeals() {
		crmStore.setLoading(true);
		crmStore.clearError();
		try {
			const pls = await crmAPI.getPipelines();
			crmStore.setPipelines(pls);
			if (pls.length && !crmStore.selectedPipeline) {
				crmStore.selectPipeline(pls[0]);
			}
			await loadDeals();
		} catch (e) {
			logger.error('Failed to load pipelines/deals', e);
			crmStore.setError('Failed to load deals. Please try again.');
		} finally {
			crmStore.setLoading(false);
		}
	}

	async function loadDeals() {
		if (!crmStore.selectedPipeline) return;
		try {
			crmStore.setLoading(true);
			const response = await crmAPI.getDeals({
				pipeline_id: crmStore.selectedPipeline.id,
				per_page: 500
			});
			crmStore.setDeals(response.data);
		} catch (e) {
			logger.error('Failed to load deals', e);
			crmStore.setError('Failed to load deals. Please try again.');
		} finally {
			crmStore.setLoading(false);
		}
	}

	async function loadForecast() {
		try {
			forecast = await crmAPI.getDealForecast(forecastPeriod);
		} catch (e) {
			logger.error('Failed to load forecast', e);
		}
	}

	function selectPipeline(p: Pipeline) {
		crmStore.selectPipeline(p);
		loadDeals();
		loadForecast();
	}

	function openDeal(deal: Deal) {
		goto(`/crm/deals/${deal.id}`);
	}

	function formatCurrency(value: number): string {
		if (!value) return '$0';
		if (value >= 1_000_000) return `$${(value / 1_000_000).toFixed(1)}M`;
		if (value >= 1_000) return `$${(value / 1_000).toFixed(1)}k`;
		return `$${value.toFixed(0)}`;
	}

	// ICT 7: Type-safe stage value calculation
	function getStageValue(stage: Stage): number {
		return crmStore.dealsByStage[stage.id]?.reduce((sum: number, d) => sum + d.amount, 0) ?? 0;
	}
</script>

<svelte:head>
	<title>CRM Deals | Revolution Trading Pros</title>
</svelte:head>

<div class="deals-page">
	<div class="deals-shell">
		<!-- Header -->
		<div class="page-header">
			<div class="title-group">
				<div class="title-icon">
					<IconActivity size={22} />
				</div>
				<div>
					<h1 class="page-title">Deals Pipeline</h1>
					<p class="page-subtitle">RevolutionCRM-L8-System · Forecast-ready sales pipeline</p>
				</div>
			</div>

			<div class="forecast-control">
				<span>Forecast period:</span>
				<select class="forecast-select" bind:value={forecastPeriod} onchange={loadForecast}>
					<option value="this_month">This month</option>
					<option value="next_month">Next month</option>
					<option value="this_quarter">This quarter</option>
					<option value="next_quarter">Next quarter</option>
				</select>
			</div>
		</div>

		<!-- Forecast Summary -->
		{#if forecast}
			<div class="forecast-grid">
				<div class="forecast-card">
					<p class="metric-label">Commit</p>
					<p class="metric-value metric-value--emerald">
						<IconCurrencyDollar size={18} />
						{formatCurrency(forecast.commit)}
					</p>
				</div>
				<div class="forecast-card">
					<p class="metric-label">Best Case</p>
					<p class="metric-value metric-value--sky">
						<IconCurrencyDollar size={18} />
						{formatCurrency(forecast.best_case)}
					</p>
				</div>
				<div class="forecast-card">
					<p class="metric-label">Pipeline</p>
					<p class="metric-value">
						<IconCurrencyDollar size={18} />
						{formatCurrency(forecast.pipeline)}
					</p>
				</div>
				<div class="forecast-card">
					<p class="metric-label">Worst Case</p>
					<p class="metric-value metric-value--amber">
						<IconCurrencyDollar size={18} />
						{formatCurrency(forecast.worst_case)}
					</p>
				</div>
			</div>
		{/if}

		<!-- Pipelines Selector -->
		<div class="pipeline-tabs">
			{#each crmStore.pipelines as pipeline (pipeline.id)}
				<button
					class={{
						'pipeline-tab': true,
						'pipeline-tab--active':
							crmStore.selectedPipeline && crmStore.selectedPipeline.id === pipeline.id
					}}
					onclick={() => selectPipeline(pipeline)}
				>
					{pipeline.name}
				</button>
			{/each}
		</div>

		<!-- Error -->
		{#if crmStore.error}
			<div class="error-panel">
				{crmStore.error}
			</div>
		{/if}

		<!-- Kanban Board -->
		<div class="kanban-board">
			{#if !crmStore.selectedPipeline}
				<div class="empty-board">No pipelines configured.</div>
			{:else}
				{#each crmStore.selectedPipeline.stages as stage (stage.id)}
					<div class="stage-column">
						<div class="stage-header">
							<div>
								<p class="stage-title">{stage.name}</p>
								<p class="stage-meta">
									{stage.probability}% · {crmStore.dealsByStage[stage.id]?.length ?? 0} deals
								</p>
							</div>
							<span class="stage-value">
								{formatCurrency(getStageValue(stage))}
							</span>
						</div>

						<div class="deal-list">
							{#if !crmStore.dealsByStage[stage.id]?.length}
								<p class="empty-stage">No deals in this stage.</p>
							{:else}
								{#each crmStore.dealsByStage[stage.id] as deal (deal.id)}
									<button type="button" class="deal-card" onclick={() => openDeal(deal)}>
										<p class="deal-title">{deal.name}</p>
										<p class="deal-contact">
											{deal.contact?.full_name ?? 'Unassigned contact'}
										</p>
										<div class="deal-meta">
											<span>{formatCurrency(deal.amount)}</span>
											<span class="deal-probability">
												{deal.probability}%
												<span class="deal-arrow">
													<IconArrowRight size={12} />
												</span>
											</span>
										</div>
									</button>
								{/each}
							{/if}
						</div>
					</div>
				{/each}
			{/if}
		</div>
	</div>
</div>

<style>
	.deals-page {
		min-height: 100%;
		background: rgba(2, 6, 23, 0.95);
		color: #f8fafc;
	}

	.deals-shell {
		width: min(100%, 80rem);
		margin: 0 auto;
		padding: 2rem 1.5rem;
	}

	.page-header,
	.title-group,
	.title-icon,
	.forecast-control,
	.metric-value,
	.pipeline-tabs,
	.kanban-board,
	.stage-header,
	.deal-meta,
	.deal-probability,
	.deal-arrow {
		display: flex;
	}

	.page-header {
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.title-group {
		align-items: center;
		gap: 0.75rem;
	}

	.title-icon {
		width: 2.5rem;
		height: 2.5rem;
		align-items: center;
		justify-content: center;
		border-radius: 0.75rem;
		background: rgba(14, 165, 233, 0.2);
		color: #38bdf8;
	}

	.page-title {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 600;
		letter-spacing: 0;
		line-height: 1.2;
	}

	.page-subtitle {
		margin: 0.25rem 0 0;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.forecast-control {
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		font-size: 0.75rem;
	}

	.forecast-select {
		border: 1px solid #1e293b;
		border-radius: 0.5rem;
		background: rgba(15, 23, 42, 0.8);
		color: #e2e8f0;
		padding: 0.25rem 0.5rem;
		font: inherit;
		font-size: 0.75rem;
	}

	.forecast-grid {
		display: grid;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.forecast-card,
	.stage-column {
		border: 1px solid #1e293b;
		border-radius: 1rem;
		background: rgba(15, 23, 42, 0.7);
	}

	.forecast-card {
		padding: 1rem;
		font-size: 0.75rem;
	}

	.metric-label {
		margin: 0;
		color: #64748b;
		font-size: 0.6875rem;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}

	.metric-value {
		align-items: center;
		gap: 0.25rem;
		margin: 0.5rem 0 0;
		color: #f1f5f9;
		font-size: 1.125rem;
		font-weight: 600;
	}

	.metric-value--emerald {
		color: #6ee7b7;
	}

	.metric-value--sky {
		color: #7dd3fc;
	}

	.metric-value--amber {
		color: #fcd34d;
	}

	.pipeline-tabs {
		flex-wrap: wrap;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 1rem;
		font-size: 0.75rem;
	}

	.pipeline-tab {
		border: 1px solid #1e293b;
		border-radius: 999px;
		background: rgba(15, 23, 42, 0.8);
		color: #cbd5e1;
		padding: 0.25rem 0.75rem;
		transition:
			background 0.2s ease,
			border-color 0.2s ease,
			color 0.2s ease;
	}

	.pipeline-tab:hover,
	.pipeline-tab:focus-visible {
		border-color: #334155;
	}

	.pipeline-tab--active {
		border-color: #0ea5e9;
		background: rgba(14, 165, 233, 0.15);
		color: #bae6fd;
	}

	.error-panel {
		margin-bottom: 1rem;
		border: 1px solid rgba(190, 18, 60, 0.6);
		border-radius: 0.75rem;
		background: rgba(76, 5, 25, 0.4);
		color: #fecdd3;
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
	}

	.kanban-board {
		gap: 0.75rem;
		overflow-x: auto;
		padding-bottom: 1rem;
	}

	.empty-board {
		flex: 1;
		padding: 4rem 0;
		color: #64748b;
		font-size: 0.875rem;
		text-align: center;
	}

	.stage-column {
		min-width: 16.25rem;
		flex: 1;
		padding: 0.75rem;
	}

	.stage-header {
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		font-size: 0.75rem;
	}

	.stage-title {
		margin: 0;
		color: #f1f5f9;
		font-weight: 500;
	}

	.stage-meta,
	.empty-stage,
	.deal-contact,
	.deal-meta {
		color: #64748b;
	}

	.stage-meta {
		margin: 0.125rem 0 0;
		font-size: 0.6875rem;
	}

	.stage-value {
		border-radius: 999px;
		background: rgba(30, 41, 59, 0.8);
		color: #cbd5e1;
		padding: 0.25rem 0.5rem;
		font-size: 0.6875rem;
		white-space: nowrap;
	}

	.deal-list {
		display: grid;
		gap: 0.5rem;
		font-size: 0.75rem;
	}

	.empty-stage {
		margin: 0;
		padding: 1rem 0;
		font-size: 0.6875rem;
		text-align: center;
	}

	.deal-card {
		width: 100%;
		cursor: pointer;
		border: 1px solid rgba(30, 41, 59, 0.8);
		border-radius: 0.75rem;
		background: rgba(15, 23, 42, 0.9);
		color: inherit;
		padding: 0.75rem;
		text-align: left;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.deal-card:hover,
	.deal-card:focus-visible {
		border-color: rgba(14, 165, 233, 0.7);
		outline: none;
		box-shadow: 0 0 0 2px rgba(14, 165, 233, 0.35);
	}

	.deal-title {
		margin: 0;
		color: #f1f5f9;
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.deal-contact {
		margin: 0.125rem 0 0;
		font-size: 0.6875rem;
	}

	.deal-meta {
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		margin-top: 0.5rem;
		font-size: 0.6875rem;
	}

	.deal-probability {
		align-items: center;
		gap: 0.25rem;
	}

	.deal-arrow {
		color: #38bdf8;
	}

	@media (min-width: 768px) {
		.forecast-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (max-width: 767px) {
		.deals-shell {
			padding-right: 1rem;
			padding-left: 1rem;
		}

		.forecast-control,
		.forecast-select {
			width: 100%;
		}
	}
</style>
