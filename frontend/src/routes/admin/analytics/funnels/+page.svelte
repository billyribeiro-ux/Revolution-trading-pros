<script lang="ts">
	/**
	 * Funnels Dashboard - Conversion Funnel Analysis
	 * Apple ICT7 Grade Implementation
	 *
	 * Create, manage, and analyze multi-step conversion funnels
	 * with detailed drop-off analysis.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { page } from '$app/state';
	import { analyticsApi } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import FunnelChart from '$lib/components/analytics/FunnelChart.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconList from '@tabler/icons-svelte-runes/icons/list';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import { toastStore } from '$lib/stores/toast.svelte';

	interface Funnel {
		key: string;
		name: string;
		description?: string;
		steps: Array<{
			name: string;
			count: number;
			conversion_rate: number;
			drop_off_rate: number;
		}>;
		overall_conversion: number;
		avg_completion_time?: number;
	}

	// Svelte 5 Runes - State
	let funnels = $state<Funnel[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('30d');
	let selectedFunnel = $state<Funnel | null>(null);
	let showCreateModal = $state(false);

	// New funnel form
	let newFunnel = $state({
		name: '',
		description: '',
		steps: [{ name: '', event_name: '' }]
	});

	async function loadFunnels() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getFunnels(selectedPeriod);
			funnels = response.funnels || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load funnels';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		// FIX-2026-04-26 (audit 08-analytics §P2-6): reset selection on period
		// change. Previously `selectedFunnel` could point at a stale funnel
		// object that wasn't present in the new period, breaking the
		// `selectedFunnel?.key === funnel.key` test.
		selectedFunnel = null;
		loadFunnels();
	}

	function addStep() {
		newFunnel.steps = [...newFunnel.steps, { name: '', event_name: '' }];
	}

	function removeStep(index: number) {
		if (newFunnel.steps.length > 1) {
			newFunnel.steps = newFunnel.steps.filter((_, i) => i !== index);
		}
	}

	async function createFunnel() {
		try {
			await analyticsApi.createFunnel({
				name: newFunnel.name,
				description: newFunnel.description,
				steps: newFunnel.steps.map((s, i) => ({
					step_number: i + 1,
					name: s.name,
					event_name: s.event_name
				}))
			});
			showCreateModal = false;
			newFunnel = { name: '', description: '', steps: [{ name: '', event_name: '' }] };
			loadFunnels();
		} catch (e) {
			toastStore.error(e instanceof Error ? e.message : 'Failed to create funnel');
		}
	}

	function formatTime(seconds?: number): string {
		if (!seconds) return '-';
		if (seconds < 60) return `${seconds.toFixed(0)}s`;
		if (seconds < 3600) return `${(seconds / 60).toFixed(0)}m`;
		return `${(seconds / 3600).toFixed(1)}h`;
	}

	// FIX-2026-04-26 (P1-3): $derived restores reactivity past helper's `untrack`.
	let isAnalyticsConnected = $derived(getIsAnalyticsConnected());

	// FIX-2026-04-26 (P1-1): onMount replaces the $effect cascade pattern.
	onMount(() => {
		if (!browser) return;

		(async () => {
			try {
				await connections.load();
			} catch (e) {
				if (import.meta.env.DEV) {
					console.error('[Funnels] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadFunnels();
			} else {
				loading = false;
			}

			// FIX-2026-04-26 (audit 08-analytics §P3-6): the stub create page at
			// `/admin/analytics/funnels/create` redirects here with `?create=1`
			// so the modal opens itself. Same hook covers any other deep link.
			if (page.url.searchParams.get('create') === '1') {
				showCreateModal = true;
			}
		})();
	});
</script>

<svelte:head>
	<title>Conversion Funnels | Analytics</title>
</svelte:head>

<div class="funnels-page">
	<div class="funnels-page__container">
		<!-- Apple ICT7 Grade Header -->
		<header class="funnels-header">
			<div class="funnels-header__title-group">
				<div class="funnels-header__icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: chevron-down -->
					<IconChevronDown size={24} aria-hidden="true" />
				</div>
				<div>
					<h1>Conversion Funnels</h1>
					<p>Track and optimize user conversion journeys</p>
				</div>
			</div>
			{#if isAnalyticsConnected}
				<div class="funnels-header__actions">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
					<button onclick={() => (showCreateModal = true)} class="primary-action">
						Create Funnel
					</button>
				</div>
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="loading-state">
				<div class="loading-spinner loading-spinner--lg"></div>
			</div>
		{:else if !isAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else if loading}
			<div class="loading-state">
				<div class="loading-spinner"></div>
			</div>
		{:else if error}
			<div class="state-card state-card--error">
				<div class="state-card__icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: alert-circle error -->
					<IconAlertCircle size={32} aria-hidden="true" />
				</div>
				<p>{error}</p>
				<button onclick={loadFunnels} class="state-card__button state-card__button--error">
					Retry
				</button>
			</div>
		{:else if funnels.length === 0}
			<div class="state-card">
				<div class="state-card__icon state-card__icon--violet">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: chevron-down (no funnels empty state) -->
					<IconChevronDown size={32} aria-hidden="true" />
				</div>
				<h3>No Funnels Yet</h3>
				<p>Create your first funnel to track user conversion journeys</p>
				<button
					onclick={() => (showCreateModal = true)}
					class="state-card__button state-card__button--primary"
				>
					Create Your First Funnel
				</button>
			</div>
		{:else}
			<!-- Funnel Cards -->
			<div class="funnel-cards">
				{#each funnels as funnel (funnel.key)}
					<button
						onclick={() => (selectedFunnel = funnel)}
						class={{ 'funnel-card': true, 'is-selected': selectedFunnel?.key === funnel.key }}
					>
						<div class="funnel-card__header">
							<h3>{funnel.name}</h3>
							<span>
								{funnel.overall_conversion.toFixed(1)}%
							</span>
						</div>
						{#if funnel.description}
							<p class="funnel-card__description">{funnel.description}</p>
						{/if}
						<div class="funnel-card__meta">
							<span>
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: list -->
								<IconList size={16} aria-hidden="true" />
								{funnel.steps.length} steps
							</span>
							{#if funnel.avg_completion_time}
								<span>
									<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: clock -->
									<IconClock size={16} aria-hidden="true" />
									Avg: {formatTime(funnel.avg_completion_time)}
								</span>
							{/if}
						</div>
					</button>
				{/each}
			</div>

			<!-- Selected Funnel Detail -->
			{#if selectedFunnel}
				<div class="selected-funnel">
					<FunnelChart
						steps={selectedFunnel.steps}
						title={selectedFunnel.name}
						showDropOff={true}
					/>

					<!-- Step Details Table -->
					<div class="analysis-card">
						<div class="analysis-card__header">
							<h3>Step Analysis</h3>
						</div>
						<div class="analysis-table-scroll">
							<table class="analysis-table">
								<thead>
									<tr>
										<th>Step</th>
										<th class="align-right">Users</th>
										<th class="align-right">Conversion</th>
										<th class="align-right">Drop-off</th>
										<th class="align-right">From Start</th>
									</tr>
								</thead>
								<tbody>
									{#each selectedFunnel.steps as step, i (i)}
										{@const firstStep = selectedFunnel.steps[0]}
										{@const fromStart =
											firstStep && firstStep.count > 0 ? (step.count / firstStep.count) * 100 : 0}
										<tr>
											<td>
												<div class="step-label">
													<span>
														{i + 1}
													</span>
													<strong>{step.name}</strong>
												</div>
											</td>
											<td class="align-right table-value">
												{step.count.toLocaleString()}
											</td>
											<td class="align-right">
												<span class="table-value table-value--positive">
													{step.conversion_rate.toFixed(1)}%
												</span>
											</td>
											<td class="align-right">
												{#if i > 0}
													<span class="table-value--negative">
														-{step.drop_off_rate.toFixed(1)}%
													</span>
												{:else}
													<span class="table-muted">-</span>
												{/if}
											</td>
											<td class="align-right">
												<div class="from-start">
													<div class="from-start__track">
														<div class="from-start__bar" style:width={`${fromStart}%`}></div>
													</div>
													<span>{fromStart.toFixed(1)}%</span>
												</div>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</div>
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Create Funnel Modal -->
{#if showCreateModal}
	<div class="funnel-modal-backdrop">
		<div class="funnel-modal" role="dialog" aria-modal="true" aria-labelledby="funnel-modal-title">
			<div class="funnel-modal__header">
				<div class="funnel-modal__title-row">
					<h2 id="funnel-modal-title">Create Funnel</h2>
					<button
						onclick={() => (showCreateModal = false)}
						class="funnel-modal__close"
						aria-label="Close modal"
					>
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (close) -->
						<IconX size={20} aria-hidden="true" />
					</button>
				</div>
			</div>

			<div class="funnel-modal__body">
				<!-- Basic Info -->
				<div class="form-stack">
					<div class="form-field">
						<label for="funnel-name">Name</label>
						<input
							id="funnel-name"
							name="funnel-name"
							type="text"
							bind:value={newFunnel.name}
							placeholder="e.g., Purchase Funnel"
							class="form-control"
						/>
					</div>
					<div class="form-field">
						<label for="funnel-description">Description</label>
						<textarea
							id="funnel-description"
							bind:value={newFunnel.description}
							placeholder="Describe this funnel..."
							rows={2}
							class="form-control form-control--textarea"
						></textarea>
					</div>
				</div>

				<!-- Steps -->
				<div>
					<div class="steps-header">
						<span>Funnel Steps</span>
						<button onclick={addStep} class="steps-header__add"> + Add Step </button>
					</div>
					<div class="steps-list">
						{#each newFunnel.steps as step, index (index)}
							<div class="step-row">
								<span class="step-row__number">
									{index + 1}
								</span>
								<input
									id={`page-step-name-${index}`}
									name={`page-step-name-${index}`}
									type="text"
									bind:value={step.name}
									placeholder="Step name"
									class="step-input"
								/>
								<input
									id={`page-step-event-name-${index}`}
									name={`page-step-event-name-${index}`}
									type="text"
									bind:value={step.event_name}
									placeholder="Event name"
									class="step-input"
								/>
								{#if newFunnel.steps.length > 1}
									<button
										onclick={() => removeStep(index)}
										class="step-row__remove"
										aria-label="Remove step"
									>
										<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: trash -->
										<IconTrash size={16} aria-hidden="true" />
									</button>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			</div>

			<div class="funnel-modal__footer">
				<button onclick={() => (showCreateModal = false)} class="secondary-action"> Cancel </button>
				<button
					onclick={createFunnel}
					disabled={!newFunnel.name || newFunnel.steps.some((s) => !s.name || !s.event_name)}
					class="primary-action"
				>
					Create Funnel
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.funnels-page {
		min-height: 100%;
		background: linear-gradient(135deg, #020617 0%, #0f172a 50%, #020617 100%);
		color: #ffffff;
	}

	.funnels-page__container {
		max-width: 80rem;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	.funnels-header,
	.funnels-header__title-group,
	.funnels-header__actions,
	.funnel-card__header,
	.funnel-card__meta,
	.funnel-card__meta span,
	.step-label,
	.from-start,
	.funnel-modal__title-row,
	.steps-header,
	.step-row,
	.funnel-modal__footer {
		display: flex;
		align-items: center;
	}

	.funnels-header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.funnels-header__title-group {
		gap: 1rem;
	}

	.funnels-header__actions {
		gap: 1rem;
	}

	.funnels-header__icon,
	.state-card__icon,
	.step-label span,
	.funnel-modal__close,
	.step-row__number {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.funnels-header__icon {
		width: 3rem;
		height: 3rem;
		border-radius: 1rem;
		background: linear-gradient(135deg, #8b5cf6 0%, #9333ea 100%);
		box-shadow: 0 10px 15px -3px rgb(139 92 246 / 0.2);
	}

	.funnels-header h1 {
		margin: 0;
		color: #ffffff;
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: 0;
		line-height: 2rem;
	}

	.funnels-header p,
	.funnel-card__description,
	.funnel-card__meta,
	.table-muted,
	.from-start span {
		margin: 0;
		color: #94a3b8;
	}

	.primary-action,
	.secondary-action,
	.state-card__button,
	.funnel-card,
	.funnel-modal__close,
	.steps-header__add,
	.step-row__remove {
		border: 0;
		cursor: pointer;
		font: inherit;
		transition:
			background 150ms ease,
			border-color 150ms ease,
			box-shadow 150ms ease,
			color 150ms ease,
			opacity 150ms ease;
	}

	.primary-action {
		border-radius: 0.75rem;
		background: linear-gradient(90deg, #8b5cf6 0%, #9333ea 100%);
		box-shadow: 0 10px 15px -3px rgb(139 92 246 / 0.25);
		color: #ffffff;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25rem;
		padding: 0.625rem 1.25rem;
	}

	.primary-action:hover:not(:disabled) {
		background: linear-gradient(90deg, #a78bfa 0%, #a855f7 100%);
		box-shadow: 0 10px 15px -3px rgb(139 92 246 / 0.4);
	}

	.primary-action:disabled,
	.secondary-action:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
	}

	.loading-spinner {
		width: 2.5rem;
		height: 2.5rem;
		border: 4px solid rgb(139 92 246 / 0.2);
		border-top-color: #8b5cf6;
		border-radius: 999px;
		animation: spin 700ms linear infinite;
	}

	.loading-spinner--lg {
		width: 3rem;
		height: 3rem;
	}

	.state-card,
	.funnel-card,
	.analysis-card {
		border: 1px solid rgb(255 255 255 / 0.1);
		background: rgb(255 255 255 / 0.05);
		backdrop-filter: blur(24px);
	}

	.state-card {
		border-radius: 1rem;
		padding: 3rem;
		text-align: center;
	}

	.state-card--error {
		border-color: rgb(239 68 68 / 0.2);
		background: rgb(239 68 68 / 0.1);
	}

	.state-card__icon {
		width: 4rem;
		height: 4rem;
		border-radius: 1rem;
		margin: 0 auto 1rem;
		background: rgb(239 68 68 / 0.1);
		color: #f87171;
	}

	.state-card__icon--violet {
		background: rgb(139 92 246 / 0.1);
		color: #a78bfa;
	}

	.state-card h3 {
		margin: 0 0 0.5rem;
		color: #ffffff;
		font-size: 1.125rem;
		font-weight: 500;
		line-height: 1.75rem;
	}

	.state-card p {
		margin: 0 0 1.5rem;
		color: #94a3b8;
	}

	.state-card--error p {
		color: #f87171;
	}

	.state-card__button {
		display: inline-block;
		border-radius: 0.75rem;
		color: #ffffff;
		font-weight: 600;
		padding: 0.75rem 1.5rem;
	}

	.state-card__button--primary {
		background: linear-gradient(90deg, #8b5cf6 0%, #9333ea 100%);
		box-shadow: 0 10px 15px -3px rgb(139 92 246 / 0.25);
	}

	.state-card__button--primary:hover {
		background: linear-gradient(90deg, #a78bfa 0%, #a855f7 100%);
	}

	.state-card__button--error {
		border: 1px solid rgb(239 68 68 / 0.3);
		background: rgb(239 68 68 / 0.2);
		color: #f87171;
		padding: 0.625rem 1.25rem;
	}

	.state-card__button--error:hover {
		background: rgb(239 68 68 / 0.3);
	}

	.funnel-cards {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.funnel-card {
		border-radius: 1rem;
		color: inherit;
		padding: 1.5rem;
		text-align: left;
		width: 100%;
	}

	.funnel-card:hover {
		border-color: rgb(139 92 246 / 0.3);
		background: rgb(255 255 255 / 0.1);
	}

	.funnel-card.is-selected {
		border-color: rgb(139 92 246 / 0.5);
		box-shadow: 0 0 0 2px #8b5cf6;
	}

	.funnel-card__header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.funnel-card h3 {
		margin: 0;
		color: #ffffff;
		font-weight: 600;
	}

	.funnel-card__header span {
		background: linear-gradient(90deg, #a78bfa 0%, #c084fc 100%);
		background-clip: text;
		color: transparent;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 2rem;
	}

	.funnel-card__description {
		font-size: 0.875rem;
		line-height: 1.25rem;
		margin-bottom: 1rem;
	}

	.funnel-card__meta {
		gap: 1rem;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.funnel-card__meta span {
		gap: 0.375rem;
	}

	.selected-funnel {
		display: grid;
		gap: 1.5rem;
	}

	.analysis-card {
		overflow: hidden;
		border-radius: 1rem;
	}

	.analysis-card__header {
		border-bottom: 1px solid rgb(255 255 255 / 0.1);
		padding: 1.25rem;
	}

	.analysis-card h3 {
		margin: 0;
		color: #ffffff;
		font-weight: 600;
	}

	.analysis-table-scroll {
		overflow-x: auto;
	}

	.analysis-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.analysis-table thead {
		background: rgb(30 41 59 / 0.5);
	}

	.analysis-table th,
	.analysis-table td {
		padding: 1rem 1.25rem;
		text-align: left;
		vertical-align: middle;
	}

	.analysis-table th {
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 500;
		letter-spacing: 0.05em;
		line-height: 1rem;
		text-transform: uppercase;
		white-space: nowrap;
	}

	.analysis-table tbody tr + tr {
		border-top: 1px solid rgb(255 255 255 / 0.05);
	}

	.analysis-table tbody tr {
		transition: background 150ms ease;
	}

	.analysis-table tbody tr:hover {
		background: rgb(255 255 255 / 0.05);
	}

	.align-right {
		text-align: right;
	}

	.step-label {
		gap: 0.75rem;
	}

	.step-label span,
	.step-row__number {
		width: 1.75rem;
		height: 1.75rem;
		border-radius: 0.5rem;
		background: rgb(139 92 246 / 0.2);
		color: #a78bfa;
		font-size: 0.75rem;
		font-weight: 600;
		line-height: 1rem;
		flex: 0 0 auto;
	}

	.step-label strong,
	.table-value {
		color: #ffffff;
		font-weight: 500;
	}

	.table-value--positive {
		color: #34d399;
	}

	.table-value--negative {
		color: #f87171;
	}

	.from-start {
		justify-content: flex-end;
		gap: 0.75rem;
	}

	.from-start__track {
		overflow: hidden;
		width: 5rem;
		height: 0.5rem;
		border-radius: 999px;
		background: #334155;
	}

	.from-start__bar {
		height: 100%;
		border-radius: inherit;
		background: linear-gradient(90deg, #8b5cf6 0%, #a855f7 100%);
	}

	.from-start span {
		width: 3.5rem;
		text-align: right;
	}

	.funnel-modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgb(0 0 0 / 0.6);
		backdrop-filter: blur(8px);
		padding: 1rem;
	}

	.funnel-modal {
		width: 100%;
		max-width: 42rem;
		max-height: 90vh;
		overflow-y: auto;
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 1rem;
		background: #0f172a;
		box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.6);
	}

	.funnel-modal__header,
	.funnel-modal__footer {
		padding: 1.5rem;
	}

	.funnel-modal__header {
		border-bottom: 1px solid rgb(255 255 255 / 0.1);
	}

	.funnel-modal__title-row {
		justify-content: space-between;
		gap: 1rem;
	}

	.funnel-modal h2 {
		margin: 0;
		color: #ffffff;
		font-size: 1.25rem;
		font-weight: 700;
		line-height: 1.75rem;
	}

	.funnel-modal__close {
		width: 2rem;
		height: 2rem;
		border-radius: 0.5rem;
		background: rgb(255 255 255 / 0.05);
		color: #94a3b8;
	}

	.funnel-modal__close:hover {
		background: rgb(255 255 255 / 0.1);
		color: #ffffff;
	}

	.funnel-modal__body {
		display: grid;
		gap: 1.5rem;
		padding: 1.5rem;
	}

	.form-stack {
		display: grid;
		gap: 1rem;
	}

	.form-field {
		display: grid;
		gap: 0.5rem;
	}

	.form-field label,
	.steps-header span {
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
	}

	.form-control,
	.step-input {
		border: 1px solid rgb(255 255 255 / 0.1);
		background: rgb(30 41 59 / 0.5);
		color: #ffffff;
		font: inherit;
		outline: none;
		transition:
			border-color 150ms ease,
			box-shadow 150ms ease;
	}

	.form-control {
		width: 100%;
		border-radius: 0.75rem;
		padding: 0.75rem 1rem;
	}

	.form-control--textarea {
		resize: none;
	}

	.step-input {
		flex: 1 1 0;
		min-width: 10rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		line-height: 1.25rem;
		padding: 0.5rem 0.75rem;
	}

	.form-control::placeholder,
	.step-input::placeholder {
		color: #64748b;
	}

	.form-control:focus,
	.step-input:focus {
		border-color: rgb(139 92 246 / 0.5);
		box-shadow: 0 0 0 2px rgb(139 92 246 / 0.5);
	}

	.steps-header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.steps-header__add {
		background: transparent;
		color: #a78bfa;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
		padding: 0;
	}

	.steps-header__add:hover {
		color: #c4b5fd;
	}

	.steps-list {
		display: grid;
		gap: 0.75rem;
	}

	.step-row {
		gap: 0.75rem;
		border: 1px solid rgb(255 255 255 / 0.05);
		border-radius: 0.75rem;
		background: rgb(30 41 59 / 0.3);
		padding: 1rem;
	}

	.step-row__remove {
		border-radius: 0.5rem;
		background: transparent;
		color: #f87171;
		padding: 0.5rem;
	}

	.step-row__remove:hover {
		background: rgb(239 68 68 / 0.1);
		color: #fca5a5;
	}

	.funnel-modal__footer {
		justify-content: flex-end;
		gap: 0.75rem;
		border-top: 1px solid rgb(255 255 255 / 0.1);
	}

	.secondary-action {
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 0.75rem;
		background: transparent;
		color: #cbd5e1;
		padding: 0.625rem 1.25rem;
	}

	.secondary-action:hover {
		background: rgb(255 255 255 / 0.05);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 640px) {
		.funnels-page__container {
			padding-inline: 1.5rem;
		}
	}

	@media (min-width: 1024px) {
		.funnels-page__container {
			padding-inline: 2rem;
		}

		.funnel-cards {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (max-width: 760px) {
		.funnels-header,
		.funnels-header__actions,
		.funnel-modal__footer {
			align-items: stretch;
			flex-direction: column;
		}

		.funnels-header__actions :global(.period-selector) {
			width: 100%;
		}

		.step-row {
			align-items: stretch;
			flex-direction: column;
		}
	}
</style>
