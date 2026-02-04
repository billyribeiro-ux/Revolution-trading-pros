<!--
	URL: /crm/deals
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { crmAPI } from '$lib/api/crm';
	import { crmStore } from '$lib/stores/crm';
	import type { Deal, Pipeline, Stage } from '$lib/crm/types';
	import { IconActivity, IconArrowRight, IconCurrencyDollar } from '$lib/icons';

	let forecastPeriod = $state('this_month');
	let forecast = $state<{ commit: number; best_case: number; pipeline: number; worst_case: number } | null>(null);

	$effect(() => {
		if (browser) {
			loadPipelinesAndDeals();
			loadForecast();
		}
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
			console.error('Failed to load pipelines/deals', e);
			crmStore.setError('Failed to load deals. Please try again.');
		} finally {
			crmStore.setLoading(false);
		}
	}

	async function loadDeals() {
		if (!crmStore.selectedPipeline) return;
		try {
			crmStore.setLoading(true);
			const response = await crmAPI.getDeals({ pipeline_id: crmStore.selectedPipeline.id, per_page: 500 });
			crmStore.setDeals(response.data);
		} catch (e) {
			console.error('Failed to load deals', e);
			crmStore.setError('Failed to load deals. Please try again.');
		} finally {
			crmStore.setLoading(false);
		}
	}

	async function loadForecast() {
		try {
			forecast = await crmAPI.getDealForecast(forecastPeriod);
		} catch (e) {
			console.error('Failed to load forecast', e);
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
		return crmStore.dealsByStage[stage.id]?.reduce((sum, d) => sum + d.amount, 0) ?? 0;
	}
</script>

<svelte:head>
	<title>CRM Deals | Revolution Trading Pros</title>
</svelte:head>

<div class="bg-slate-950/95 text-slate-50">
	<div class="mx-auto max-w-7xl px-6 py-8">
		<!-- Header -->
		<div class="mb-6 flex flex-wrap items-center justify-between gap-4">
			<div class="flex items-center gap-3">
				<div
					class="flex h-10 w-10 items-center justify-center rounded-xl bg-sky-500/20 text-sky-400"
				>
					<IconActivity size={22} />
				</div>
				<div>
					<h1 class="text-2xl font-semibold tracking-tight">Deals Pipeline</h1>
					<p class="text-sm text-slate-400">
						RevolutionCRM-L8-System · Forecast-ready sales pipeline
					</p>
				</div>
			</div>

			<div class="flex items-center gap-2 text-xs text-slate-400">
				<span>Forecast period:</span>
				<select
					class="rounded-lg border border-slate-800 bg-slate-900/80 px-2 py-1 text-xs"
					bind:value={forecastPeriod}
					onchange={loadForecast}
				>
					<option value="this_month">This month</option>
					<option value="next_month">Next month</option>
					<option value="this_quarter">This quarter</option>
					<option value="next_quarter">Next quarter</option>
				</select>
			</div>
		</div>

		<!-- Forecast Summary -->
		{#if forecast}
			<div class="mb-6 grid gap-4 md:grid-cols-4">
				<div class="rounded-2xl border border-slate-800 bg-slate-900/70 p-4 text-xs">
					<p class="text-[11px] uppercase tracking-wide text-slate-500">Commit</p>
					<p class="mt-2 flex items-center gap-1 text-lg font-semibold text-emerald-300">
						<IconCurrencyDollar size={18} />
						{formatCurrency(forecast.commit)}
					</p>
				</div>
				<div class="rounded-2xl border border-slate-800 bg-slate-900/70 p-4 text-xs">
					<p class="text-[11px] uppercase tracking-wide text-slate-500">Best Case</p>
					<p class="mt-2 flex items-center gap-1 text-lg font-semibold text-sky-300">
						<IconCurrencyDollar size={18} />
						{formatCurrency(forecast.best_case)}
					</p>
				</div>
				<div class="rounded-2xl border border-slate-800 bg-slate-900/70 p-4 text-xs">
					<p class="text-[11px] uppercase tracking-wide text-slate-500">Pipeline</p>
					<p class="mt-2 flex items-center gap-1 text-lg font-semibold text-slate-100">
						<IconCurrencyDollar size={18} />
						{formatCurrency(forecast.pipeline)}
					</p>
				</div>
				<div class="rounded-2xl border border-slate-800 bg-slate-900/70 p-4 text-xs">
					<p class="text-[11px] uppercase tracking-wide text-slate-500">Worst Case</p>
					<p class="mt-2 flex items-center gap-1 text-lg font-semibold text-amber-300">
						<IconCurrencyDollar size={18} />
						{formatCurrency(forecast.worst_case)}
					</p>
				</div>
			</div>
		{/if}

		<!-- Pipelines Selector -->
		<div class="mb-4 flex flex-wrap items-center gap-2 text-xs">
			{#each crmStore.pipelines as pipeline}
				<button
					class={`rounded-full border px-3 py-1 ${
						crmStore.selectedPipeline && crmStore.selectedPipeline.id === pipeline.id
							? 'border-sky-500 bg-sky-500/15 text-sky-200'
							: 'border-slate-800 bg-slate-900/80 text-slate-300 hover:border-slate-700'
					}`}
					onclick={() => selectPipeline(pipeline)}
				>
					{pipeline.name}
				</button>
			{/each}
		</div>

		<!-- Error -->
		{#if crmStore.error}
			<div
				class="mb-4 rounded-xl border border-rose-700/60 bg-rose-950/40 px-4 py-3 text-sm text-rose-200"
			>
				{crmStore.error}
			</div>
		{/if}

		<!-- Kanban Board -->
		<div class="flex gap-3 overflow-x-auto pb-4">
			{#if !crmStore.selectedPipeline}
				<div class="flex-1 py-16 text-center text-sm text-slate-500">No pipelines configured.</div>
			{:else}
				{#each crmStore.selectedPipeline.stages as stage (stage.id)}
					<div
						class="min-w-[260px] flex-1 rounded-2xl border border-slate-800/80 bg-slate-900/70 p-3"
					>
						<div class="mb-3 flex items-center justify-between gap-2 text-xs">
							<div>
								<p class="font-medium text-slate-100">{stage.name}</p>
								<p class="text-[11px] text-slate-500">
									{stage.probability}% · {crmStore.dealsByStage[stage.id]?.length ?? 0} deals
								</p>
							</div>
							<span class="rounded-full bg-slate-800/80 px-2 py-1 text-[11px] text-slate-300">
								{formatCurrency(getStageValue(stage))}
							</span>
						</div>

						<div class="space-y-2 text-xs">
							{#if !crmStore.dealsByStage[stage.id]?.length}
								<p class="py-4 text-center text-[11px] text-slate-500">No deals in this stage.</p>
							{:else}
								{#each crmStore.dealsByStage[stage.id] as deal (deal.id)}
									<button
										type="button"
										class="w-full cursor-pointer rounded-xl border border-slate-800/80 bg-slate-900/90 p-3 text-left hover:border-sky-500/70 focus:outline-none focus:ring-2 focus:ring-sky-500/60"
										onclick={() => openDeal(deal)}
									>
										<p class="text-[13px] font-medium text-slate-100">{deal.name}</p>
										<p class="mt-0.5 text-[11px] text-slate-400">
											{deal.contact?.full_name ?? 'Unassigned contact'}
										</p>
										<div class="mt-2 flex items-center justify-between text-[11px] text-slate-400">
											<span>{formatCurrency(deal.amount)}</span>
											<span class="flex items-center gap-1">
												{deal.probability}%
												<IconArrowRight size={12} class="text-sky-400" />
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
