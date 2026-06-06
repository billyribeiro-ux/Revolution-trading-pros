<!--
	URL: /crm/deals/[id]
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { crmAPI } from '$lib/api/crm';
	import type { Deal } from '$lib/crm/types';
	import { IconArrowLeft, IconCurrencyDollar, IconActivity } from '$lib/icons';

	let deal = $state<Deal | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	let dealId = $derived(page.params.id as string);

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

<div class="deal-detail-page">
	<div class="deal-detail-shell">
		<button class="back-button" onclick={goBack}>
			<IconArrowLeft size={16} />
			Back to deals
		</button>

		{#if loading && !deal}
			<div class="center-state">Loading deal…</div>
		{:else if error}
			<div class="error-panel">
				{error}
			</div>
		{:else if deal}
			<div class="deal-stack">
				<div class="panel">
					<div class="deal-header">
						<div>
							<h1 class="deal-title">{deal.name}</h1>
							<p class="deal-subtitle">
								{deal.pipeline?.name} · {deal.stage?.name}
							</p>
						</div>
						<div class="amount-card">
							<p class="amount-value">
								<IconCurrencyDollar size={16} />
								{deal.amount.toLocaleString('en-US', {
									style: 'currency',
									currency: deal.currency
								})}
							</p>
							<p class="amount-meta">Probability: {deal.probability}%</p>
						</div>
					</div>

					<div class="detail-grid">
						<div class="detail-block">
							<p class="metric-label">Contact</p>
							<p class="detail-value">{deal.contact?.full_name ?? 'Unassigned'}</p>
						</div>
						<div class="detail-block">
							<p class="metric-label">Owner</p>
							<p class="detail-value">{deal.owner?.name ?? 'Unassigned'}</p>
						</div>
						<div class="detail-block">
							<p class="metric-label">Expected Close</p>
							<p class="detail-value">
								{deal.expected_close_date
									? new Date(deal.expected_close_date).toLocaleDateString()
									: '—'}
							</p>
						</div>
					</div>
				</div>

				<div class="panel">
					<div class="section-heading">
						<span class="section-icon">
							<IconActivity size={18} />
						</span>
						Deal Metrics
					</div>
					<div class="detail-grid">
						<div class="detail-block">
							<p class="metric-label">Days in Stage</p>
							<p class="metric-value">{deal.days_in_stage}</p>
						</div>
						<div class="detail-block">
							<p class="metric-label">Days in Pipeline</p>
							<p class="metric-value">{deal.days_in_pipeline}</p>
						</div>
						<div class="detail-block">
							<p class="metric-label">Stage Changes</p>
							<p class="metric-value">{deal.stage_changes_count}</p>
						</div>
					</div>
				</div>
			</div>
		{:else}
			<div class="center-state">Deal not found.</div>
		{/if}
	</div>
</div>

<style>
	.deal-detail-page {
		min-height: 100%;
		background: rgba(2, 6, 23, 0.95);
		color: #f8fafc;
	}

	.deal-detail-shell {
		width: min(100%, 64rem);
		margin: 0 auto;
		padding: 1.5rem;
	}

	.back-button,
	.center-state,
	.deal-header,
	.amount-value,
	.section-heading,
	.section-icon {
		display: flex;
	}

	.back-button {
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 1rem;
		border: 0;
		background: transparent;
		color: #94a3b8;
		padding: 0;
		font-size: 0.75rem;
		transition: color 0.2s ease;
	}

	.back-button:hover,
	.back-button:focus-visible {
		color: #e2e8f0;
	}

	.center-state {
		height: 16rem;
		align-items: center;
		justify-content: center;
		color: #94a3b8;
	}

	.error-panel {
		border: 1px solid rgba(190, 18, 60, 0.6);
		border-radius: 0.75rem;
		background: rgba(76, 5, 25, 0.4);
		color: #fecdd3;
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
	}

	.deal-stack {
		display: grid;
		gap: 1rem;
	}

	.panel {
		border: 1px solid rgba(30, 41, 59, 0.8);
		border-radius: 1rem;
		background: rgba(15, 23, 42, 0.7);
		padding: 1.25rem;
	}

	.deal-header {
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
	}

	.deal-title {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		letter-spacing: 0;
		line-height: 1.2;
	}

	.deal-subtitle {
		margin: 0.25rem 0 0;
		color: #94a3b8;
		font-size: 0.75rem;
	}

	.amount-card {
		border-radius: 0.75rem;
		background: rgba(30, 41, 59, 0.8);
		padding: 0.5rem 0.75rem;
		font-size: 0.75rem;
		text-align: right;
	}

	.amount-value {
		align-items: center;
		justify-content: flex-end;
		gap: 0.25rem;
		margin: 0;
		color: #e2e8f0;
	}

	.amount-meta {
		margin: 0.25rem 0 0;
		color: #94a3b8;
		font-size: 0.6875rem;
	}

	.detail-grid {
		display: grid;
		gap: 0.75rem;
		margin-top: 1rem;
		color: #cbd5e1;
		font-size: 0.75rem;
	}

	.metric-label {
		margin: 0;
		color: #64748b;
		font-size: 0.6875rem;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}

	.detail-value,
	.metric-value {
		margin: 0.25rem 0 0;
		color: #f1f5f9;
	}

	.metric-value {
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.2;
	}

	.section-heading {
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		color: #f1f5f9;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.section-icon {
		color: #38bdf8;
	}

	@media (min-width: 768px) {
		.detail-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (max-width: 640px) {
		.deal-detail-shell {
			padding-right: 1rem;
			padding-left: 1rem;
		}

		.deal-header {
			flex-direction: column;
		}

		.amount-card {
			width: 100%;
			text-align: left;
		}

		.amount-value {
			justify-content: flex-start;
		}
	}
</style>
