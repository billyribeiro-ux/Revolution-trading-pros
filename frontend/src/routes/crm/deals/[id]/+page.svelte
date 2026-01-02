<!--
	URL: /crm/deals/[id]
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { crmAPI } from '$lib/api/crm';
	import type { Deal } from '$lib/crm/types';
	import { IconArrowLeft, IconCurrencyDollar, IconActivity } from '$lib/icons';

	let deal = $state<Deal | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	let dealId = $derived($page.params.id as string);

	onMount(async () => {
		await loadDeal();
	});

	async function loadDeal() {
		loading = true;
		error = null;
		try {
			deal = await crmAPI.getDeal(dealId);
		} catch (e) {
			console.error('Failed to load deal', e);
			error = 'Failed to load deal. Please try again.';
		} finally {
			loading = false;
		}
	}

	function goBack() {
		goto('/crm/deals');
	}
</script>

<svelte:head>
	<title>{deal ? `${deal.name} | CRM Deal` : 'Deal | CRM'}</title>
</svelte:head>

<div class="min-h-screen bg-slate-950/95 text-slate-50">
	<div class="mx-auto max-w-5xl px-6 py-6">
		<button
			class="mb-4 inline-flex items-center gap-2 text-xs text-slate-400 hover:text-slate-200"
			onclick={goBack}
		>
			<IconArrowLeft size={16} />
			Back to deals
		</button>

		{#if loading && !deal}
			<div class="flex h-64 items-center justify-center text-slate-400">Loading deal…</div>
		{:else if error}
			<div
				class="rounded-xl border border-rose-700/60 bg-rose-950/40 px-4 py-3 text-sm text-rose-200"
			>
				{error}
			</div>
		{:else if deal}
			<div class="space-y-4">
				<div class="rounded-2xl border border-slate-800/80 bg-slate-900/70 p-5">
					<div class="flex items-start justify-between gap-4">
						<div>
							<h1 class="text-xl font-semibold leading-tight">{deal.name}</h1>
							<p class="mt-1 text-xs text-slate-400">
								{deal.pipeline?.name} · {deal.stage?.name}
							</p>
						</div>
						<div class="rounded-xl bg-slate-800/80 px-3 py-2 text-right text-xs">
							<p class="flex items-center justify-end gap-1 text-slate-200">
								<IconCurrencyDollar size={16} />
								{deal.amount.toLocaleString('en-US', {
									style: 'currency',
									currency: deal.currency
								})}
							</p>
							<p class="mt-1 text-[11px] text-slate-400">Probability: {deal.probability}%</p>
						</div>
					</div>

					<div class="mt-4 grid gap-3 text-xs text-slate-300 md:grid-cols-3">
						<div>
							<p class="text-[11px] uppercase tracking-wide text-slate-500">Contact</p>
							<p class="mt-1 text-slate-100">{deal.contact?.full_name ?? 'Unassigned'}</p>
						</div>
						<div>
							<p class="text-[11px] uppercase tracking-wide text-slate-500">Owner</p>
							<p class="mt-1 text-slate-100">{deal.owner?.name ?? 'Unassigned'}</p>
						</div>
						<div>
							<p class="text-[11px] uppercase tracking-wide text-slate-500">Expected Close</p>
							<p class="mt-1 text-slate-100">
								{deal.expected_close_date
									? new Date(deal.expected_close_date).toLocaleDateString()
									: '—'}
							</p>
						</div>
					</div>
				</div>

				<div class="rounded-2xl border border-slate-800/80 bg-slate-900/70 p-5 text-xs">
					<div class="mb-3 flex items-center gap-2 text-sm font-medium text-slate-100">
						<IconActivity size={18} class="text-sky-400" />
						Deal Metrics
					</div>
					<div class="grid gap-3 md:grid-cols-3">
						<div>
							<p class="text-[11px] uppercase tracking-wide text-slate-500">Days in Stage</p>
							<p class="mt-1 text-lg font-semibold text-slate-50">{deal.days_in_stage}</p>
						</div>
						<div>
							<p class="text-[11px] uppercase tracking-wide text-slate-500">Days in Pipeline</p>
							<p class="mt-1 text-lg font-semibold text-slate-50">{deal.days_in_pipeline}</p>
						</div>
						<div>
							<p class="text-[11px] uppercase tracking-wide text-slate-500">Stage Changes</p>
							<p class="mt-1 text-lg font-semibold text-slate-50">{deal.stage_changes_count}</p>
						</div>
					</div>
				</div>
			</div>
		{:else}
			<div class="flex h-64 items-center justify-center text-slate-400">Deal not found.</div>
		{/if}
	</div>
</div>
