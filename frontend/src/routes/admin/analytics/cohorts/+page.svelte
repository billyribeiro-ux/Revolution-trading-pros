<script lang="ts">
	/**
	 * Cohort Analysis - User Retention Analysis
	 * Apple ICT7 Grade Implementation
	 *
	 * Analyze user behavior patterns over time with
	 * cohort-based retention matrices.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { page } from '$app/state';
	import { analyticsApi } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import CohortMatrix from '$lib/components/analytics/CohortMatrix.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import { toastStore } from '$lib/stores/toast.svelte';

	// FIX-2026-04-26 (audit 08-analytics §P2-4): widened `type` and `granularity`
	// to `string`. The backend returns `string` (not the strict literal union),
	// and the previous `as any` cast at the assign site papered over the
	// mismatch — making any backend rename silently break the UI.
	interface Cohort {
		key: string;
		name: string;
		description?: string;
		type: string;
		granularity: string;
		retention_matrix: Array<{
			cohort: string;
			size: number;
			periods: number[];
		}>;
		summary?: {
			avg_retention: number;
			best_cohort: string;
			worst_cohort: string;
		};
	}

	// Svelte 5 Runes - State
	let cohorts = $state<Cohort[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('90d');
	let selectedGranularity = $state('weekly');
	let selectedCohort = $state<Cohort | null>(null);
	let showCreateModal = $state(false);

	// New cohort form
	let newCohort = $state({
		name: '',
		description: '',
		type: 'signup' as 'signup' | 'first_purchase' | 'custom',
		granularity: 'weekly' as 'daily' | 'weekly' | 'monthly',
		start_event: '',
		return_event: ''
	});

	async function loadCohorts() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getCohorts({
				period: selectedPeriod,
				granularity: selectedGranularity
			});
			// FIX-2026-04-26 (audit 08-analytics §P2-4): drop `as any`. The
			// API surface returned from `analyticsApi.getCohorts()` is a
			// structurally compatible shape — TypeScript can verify it.
			cohorts = response.cohorts || [];
			if (cohorts.length > 0 && !selectedCohort) {
				selectedCohort = cohorts[0];
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load cohorts';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		// FIX-2026-04-26 (audit 08-analytics §P2-6): reset selection on period
		// change. Stale references mismatch when the new period yields a
		// different cohort set.
		selectedCohort = null;
		loadCohorts();
	}

	function handleGranularityChange() {
		loadCohorts();
	}

	async function createCohort() {
		try {
			await analyticsApi.createCohort({
				name: newCohort.name,
				description: newCohort.description,
				type: newCohort.type,
				granularity: newCohort.granularity,
				start_event: newCohort.start_event || undefined,
				return_event: newCohort.return_event || undefined
			});
			showCreateModal = false;
			newCohort = {
				name: '',
				description: '',
				type: 'signup',
				granularity: 'weekly',
				start_event: '',
				return_event: ''
			};
			loadCohorts();
		} catch (e) {
			toastStore.error(e instanceof Error ? e.message : 'Failed to create cohort');
		}
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
					console.error('[Cohorts] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadCohorts();
			} else {
				loading = false;
			}

			// FIX-2026-04-26 (audit 08-analytics §P3-6): the stub create page at
			// `/admin/analytics/cohorts/create` redirects here with `?create=1`.
			if (page.url.searchParams.get('create') === '1') {
				showCreateModal = true;
			}
		})();
	});

	// Calculate overall retention metrics
	const overallMetrics = $derived(
		selectedCohort
			? {
					avgWeek1: calculateAvgRetention(selectedCohort.retention_matrix, 0),
					avgWeek4: calculateAvgRetention(selectedCohort.retention_matrix, 3),
					avgWeek8: calculateAvgRetention(selectedCohort.retention_matrix, 7)
				}
			: null
	);

	const selectedCohortTotalUsers = $derived(
		selectedCohort ? selectedCohort.retention_matrix.reduce((sum, row) => sum + row.size, 0) : 0
	);

	function formatRetentionLabel(periodNumber: number): string {
		const unit =
			selectedGranularity === 'weekly'
				? 'Week'
				: selectedGranularity === 'monthly'
					? 'Month'
					: 'Day';
		return `${unit} ${periodNumber}`;
	}

	function calculateAvgRetention(
		matrix: Array<{ cohort: string; size: number; periods: number[] }>,
		periodIndex: number
	): number {
		const values = matrix
			.filter((r) => r.periods[periodIndex] !== undefined)
			.map((r) => r.periods[periodIndex]);
		if (values.length === 0) return 0;
		return values.reduce((sum, v) => sum + v, 0) / values.length;
	}
</script>

<svelte:head>
	<title>Cohort Analysis | Analytics</title>
</svelte:head>

<div class="cohorts-page">
	<div class="cohorts-page__container">
		<!-- Apple ICT7 Grade Header -->
		<header class="cohorts-header">
			<div class="cohorts-header__title-group">
				<div class="cohorts-header__icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: users -->
					<IconUsers size={24} aria-hidden="true" />
				</div>
				<div>
					<h1>Cohort Analysis</h1>
					<p>Analyze user retention patterns over time</p>
				</div>
			</div>
			{#if isAnalyticsConnected}
				<div class="cohorts-header__actions">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
					<label class="granularity-field" for="cohort-granularity-filter">
						<span>Granularity</span>
						<select
							id="cohort-granularity-filter"
							bind:value={selectedGranularity}
							onchange={handleGranularityChange}
						>
							<option value="daily">Daily</option>
							<option value="weekly">Weekly</option>
							<option value="monthly">Monthly</option>
						</select>
					</label>
					<button type="button" onclick={() => (showCreateModal = true)} class="primary-action">
						Create Cohort
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
				<button
					type="button"
					onclick={loadCohorts}
					class="state-card__button state-card__button--error"
				>
					Retry
				</button>
			</div>
		{:else if cohorts.length === 0}
			<div class="state-card">
				<div class="state-card__icon state-card__icon--purple">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: users (no cohorts empty state) -->
					<IconUsers size={32} aria-hidden="true" />
				</div>
				<h3>No Cohorts Yet</h3>
				<p>Create your first cohort to analyze user retention</p>
				<button
					type="button"
					onclick={() => (showCreateModal = true)}
					class="state-card__button state-card__button--primary"
				>
					Create Your First Cohort
				</button>
			</div>
		{:else}
			<!-- Cohort Selector -->
			<div class="cohort-selector" aria-label="Cohorts">
				{#each cohorts as cohort (cohort.key)}
					<button
						type="button"
						onclick={() => (selectedCohort = cohort)}
						class={{ 'cohort-tab': true, 'is-active': selectedCohort?.key === cohort.key }}
					>
						{cohort.name}
					</button>
				{/each}
			</div>

			{#if selectedCohort}
				<!-- Retention Summary -->
				{#if overallMetrics}
					<div class="retention-grid">
						<div class="retention-card">
							<div class="retention-card__label">{formatRetentionLabel(1)} Retention</div>
							<div class="retention-card__value">{overallMetrics.avgWeek1.toFixed(1)}%</div>
						</div>
						<div class="retention-card">
							<div class="retention-card__label">{formatRetentionLabel(4)} Retention</div>
							<div class="retention-card__value" data-tone="purple">
								{overallMetrics.avgWeek4.toFixed(1)}%
							</div>
						</div>
						<div class="retention-card">
							<div class="retention-card__label">{formatRetentionLabel(8)} Retention</div>
							<div class="retention-card__value" data-tone="pink">
								{overallMetrics.avgWeek8.toFixed(1)}%
							</div>
						</div>
					</div>
				{/if}

				<!-- Cohort Matrix -->
				<div class="matrix-panel">
					<CohortMatrix data={selectedCohort.retention_matrix} title={selectedCohort.name} />
				</div>

				<!-- Cohort Info -->
				<div class="details-panel">
					<h2>Cohort Details</h2>
					<div class="details-grid">
						<div class="detail-item">
							<div class="detail-item__label">Type</div>
							<div class="detail-item__value">{selectedCohort.type.replace('_', ' ')}</div>
						</div>
						<div class="detail-item">
							<div class="detail-item__label">Granularity</div>
							<div class="detail-item__value">{selectedCohort.granularity}</div>
						</div>
						<div class="detail-item">
							<div class="detail-item__label">Total Cohorts</div>
							<div class="detail-item__value">{selectedCohort.retention_matrix.length}</div>
						</div>
						<div class="detail-item">
							<div class="detail-item__label">Total Users</div>
							<div class="detail-item__value">{selectedCohortTotalUsers.toLocaleString()}</div>
						</div>
					</div>
					{#if selectedCohort.description}
						<div class="detail-description">
							<div class="detail-item__label">Description</div>
							<p>{selectedCohort.description}</p>
						</div>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Create Cohort Modal -->
{#if showCreateModal}
	<div class="cohort-modal-backdrop">
		<div class="cohort-modal" role="dialog" aria-modal="true" aria-labelledby="cohort-modal-title">
			<div class="cohort-modal__header">
				<div class="cohort-modal__title-row">
					<h2 id="cohort-modal-title">Create Cohort</h2>
					<button
						type="button"
						onclick={() => (showCreateModal = false)}
						class="cohort-modal__close"
						aria-label="Close modal"
					>
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (close) -->
						<IconX size={20} aria-hidden="true" />
					</button>
				</div>
			</div>

			<div class="cohort-modal__body">
				<div class="form-field">
					<label for="cohort-name">Name</label>
					<input
						id="cohort-name"
						name="cohort-name"
						type="text"
						bind:value={newCohort.name}
						placeholder="e.g., Weekly Signup Retention"
						class="form-control"
					/>
				</div>

				<div class="form-field">
					<label for="cohort-description">Description</label>
					<textarea
						id="cohort-description"
						bind:value={newCohort.description}
						placeholder="Describe this cohort..."
						rows={2}
						class="form-control form-control--textarea"
					></textarea>
				</div>

				<div class="form-grid">
					<div class="form-field">
						<label for="cohort-type">Type</label>
						<select id="cohort-type" bind:value={newCohort.type} class="form-control">
							<option value="signup">Signup Date</option>
							<option value="first_purchase">First Purchase</option>
							<option value="custom">Custom Event</option>
						</select>
					</div>
					<div class="form-field">
						<label for="cohort-granularity">Granularity</label>
						<select id="cohort-granularity" bind:value={newCohort.granularity} class="form-control">
							<option value="daily">Daily</option>
							<option value="weekly">Weekly</option>
							<option value="monthly">Monthly</option>
						</select>
					</div>
				</div>

				{#if newCohort.type === 'custom'}
					<div class="form-grid">
						<div class="form-field">
							<label for="cohort-start-event">Start Event</label>
							<input
								id="cohort-start-event"
								name="cohort-start-event"
								type="text"
								bind:value={newCohort.start_event}
								placeholder="e.g., signup"
								class="form-control"
							/>
						</div>
						<div class="form-field">
							<label for="cohort-return-event">Return Event</label>
							<input
								id="cohort-return-event"
								name="cohort-return-event"
								type="text"
								bind:value={newCohort.return_event}
								placeholder="e.g., login"
								class="form-control"
							/>
						</div>
					</div>
				{/if}
			</div>

			<div class="cohort-modal__footer">
				<button type="button" onclick={() => (showCreateModal = false)} class="secondary-action">
					Cancel
				</button>
				<button
					type="button"
					onclick={createCohort}
					disabled={!newCohort.name}
					class="primary-action"
				>
					Create Cohort
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.cohorts-page {
		min-height: 100vh;
		background:
			radial-gradient(circle at top left, rgb(168 85 247 / 0.16), transparent 28rem),
			linear-gradient(135deg, #020617 0%, #0f172a 48%, #020617 100%);
		color: white;
	}

	.cohorts-page__container {
		width: min(100%, 80rem);
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	.cohorts-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.cohorts-header__title-group,
	.cohorts-header__actions,
	.cohort-modal__title-row,
	.cohort-modal__footer {
		display: flex;
		align-items: center;
	}

	.cohorts-header__title-group {
		gap: 1rem;
		min-width: 0;
	}

	.cohorts-header__icon {
		display: inline-grid;
		width: 3rem;
		height: 3rem;
		flex: 0 0 auto;
		place-items: center;
		border-radius: 1rem;
		background: linear-gradient(135deg, #a855f7, #db2777);
		box-shadow: 0 18px 44px rgb(168 85 247 / 0.24);
	}

	.cohorts-header h1 {
		margin: 0;
		font-size: clamp(1.875rem, 4vw, 2.5rem);
		font-weight: 700;
		line-height: 1.1;
	}

	.cohorts-header p {
		margin: 0.35rem 0 0;
		color: #94a3b8;
	}

	.cohorts-header__actions {
		gap: 1rem;
		flex-wrap: wrap;
		justify-content: flex-end;
	}

	.primary-action,
	.secondary-action,
	.state-card__button,
	.cohort-tab,
	.cohort-modal__close {
		border: 0;
		font: inherit;
	}

	.primary-action,
	.secondary-action,
	.state-card__button,
	.cohort-tab {
		display: inline-flex;
		min-height: 2.75rem;
		align-items: center;
		justify-content: center;
		border-radius: 0.75rem;
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
		font-weight: 600;
		transition:
			transform 180ms ease,
			border-color 180ms ease,
			background 180ms ease,
			box-shadow 180ms ease;
	}

	.primary-action {
		background: linear-gradient(135deg, #a855f7, #db2777);
		color: white;
		box-shadow: 0 16px 36px rgb(168 85 247 / 0.28);
	}

	.primary-action:hover:not(:disabled),
	.state-card__button:hover,
	.cohort-tab:hover {
		transform: translateY(-1px);
	}

	.primary-action:disabled {
		cursor: not-allowed;
		opacity: 0.5;
		box-shadow: none;
	}

	.secondary-action {
		border: 1px solid rgb(255 255 255 / 0.1);
		background: rgb(255 255 255 / 0.04);
		color: #cbd5e1;
	}

	.secondary-action:hover {
		border-color: rgb(255 255 255 / 0.16);
		background: rgb(255 255 255 / 0.08);
		color: white;
	}

	.granularity-field,
	.form-field {
		display: grid;
		gap: 0.5rem;
	}

	.granularity-field span,
	.form-field label {
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.granularity-field select,
	.form-control {
		width: 100%;
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 0.75rem;
		background: rgb(15 23 42 / 0.82);
		color: white;
		font: inherit;
		padding: 0.75rem 1rem;
		transition:
			border-color 180ms ease,
			box-shadow 180ms ease;
	}

	.granularity-field select:focus,
	.form-control:focus {
		border-color: #a855f7;
		box-shadow: 0 0 0 3px rgb(168 85 247 / 0.16);
		outline: none;
	}

	.form-control::placeholder {
		color: #64748b;
	}

	.form-control--textarea {
		min-height: 5rem;
		resize: vertical;
	}

	.loading-state {
		display: grid;
		min-height: 18rem;
		place-items: center;
	}

	.loading-spinner {
		width: 2rem;
		height: 2rem;
		border: 2px solid rgb(168 85 247 / 0.22);
		border-top-color: #a855f7;
		border-radius: 999px;
		animation: cohorts-spin 780ms linear infinite;
	}

	.loading-spinner--lg {
		width: 3rem;
		height: 3rem;
	}

	.state-card,
	.retention-card,
	.matrix-panel,
	.details-panel,
	.cohort-modal {
		border: 1px solid rgb(255 255 255 / 0.1);
		background: rgb(255 255 255 / 0.05);
		box-shadow: 0 24px 80px rgb(2 6 23 / 0.24);
		backdrop-filter: blur(24px);
	}

	.state-card {
		display: grid;
		justify-items: center;
		gap: 0.75rem;
		border-radius: 1rem;
		padding: 4rem 1.5rem;
		text-align: center;
	}

	.state-card--error {
		border-color: rgb(239 68 68 / 0.25);
		background: rgb(127 29 29 / 0.18);
	}

	.state-card__icon {
		display: inline-grid;
		width: 4rem;
		height: 4rem;
		place-items: center;
		border-radius: 999px;
		color: #f87171;
		background: rgb(239 68 68 / 0.12);
	}

	.state-card__icon--purple {
		color: #c084fc;
		background: rgb(168 85 247 / 0.12);
	}

	.state-card h3 {
		margin: 0;
		font-size: 1.25rem;
	}

	.state-card p {
		margin: 0;
		max-width: 34rem;
		color: #94a3b8;
	}

	.state-card__button {
		margin-top: 0.5rem;
		color: white;
	}

	.state-card__button--primary {
		background: linear-gradient(135deg, #a855f7, #db2777);
		box-shadow: 0 16px 36px rgb(168 85 247 / 0.24);
	}

	.state-card__button--error {
		background: linear-gradient(135deg, #dc2626, #f97316);
		box-shadow: 0 16px 36px rgb(220 38 38 / 0.24);
	}

	.cohort-selector {
		display: flex;
		gap: 0.5rem;
		overflow-x: auto;
		margin-bottom: 2rem;
		padding-bottom: 0.5rem;
	}

	.cohort-tab {
		flex: 0 0 auto;
		border: 1px solid rgb(255 255 255 / 0.1);
		background: rgb(255 255 255 / 0.05);
		color: #94a3b8;
		white-space: nowrap;
	}

	.cohort-tab.is-active {
		border-color: transparent;
		background: linear-gradient(135deg, #a855f7, #db2777);
		color: white;
		box-shadow: 0 16px 36px rgb(168 85 247 / 0.24);
	}

	.retention-grid {
		display: grid;
		grid-template-columns: repeat(1, minmax(0, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.retention-card {
		border-radius: 1rem;
		padding: 1.5rem;
	}

	.retention-card__label {
		margin-bottom: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.retention-card__value {
		color: white;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1;
	}

	.retention-card__value[data-tone='purple'] {
		color: #c084fc;
	}

	.retention-card__value[data-tone='pink'] {
		color: #f472b6;
	}

	.matrix-panel,
	.details-panel {
		overflow: hidden;
		border-radius: 1rem;
		margin-bottom: 2rem;
	}

	.details-panel {
		padding: 1.5rem;
	}

	.details-panel h2 {
		margin: 0 0 1rem;
		font-size: 1.125rem;
		font-weight: 700;
	}

	.details-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1.5rem;
	}

	.detail-item__label {
		margin-bottom: 0.25rem;
		color: #64748b;
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
	}

	.detail-item__value {
		color: white;
		font-size: 0.875rem;
		text-transform: capitalize;
	}

	.detail-description {
		margin-top: 1rem;
		border-top: 1px solid rgb(255 255 255 / 0.1);
		padding-top: 1rem;
	}

	.detail-description p {
		margin: 0;
		color: #cbd5e1;
		font-size: 0.875rem;
	}

	.cohort-modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: grid;
		place-items: center;
		padding: 1rem;
		background: rgb(2 6 23 / 0.72);
		backdrop-filter: blur(8px);
	}

	.cohort-modal {
		width: min(100%, 32rem);
		max-height: 90vh;
		overflow: hidden;
		border-radius: 1rem;
	}

	.cohort-modal__header,
	.cohort-modal__footer {
		padding: 1.5rem;
	}

	.cohort-modal__header {
		border-bottom: 1px solid rgb(255 255 255 / 0.1);
	}

	.cohort-modal__title-row {
		justify-content: space-between;
		gap: 1rem;
	}

	.cohort-modal h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
	}

	.cohort-modal__close {
		display: inline-grid;
		width: 2.25rem;
		height: 2.25rem;
		place-items: center;
		border-radius: 0.625rem;
		background: rgb(255 255 255 / 0.05);
		color: #94a3b8;
		transition:
			background 180ms ease,
			color 180ms ease;
	}

	.cohort-modal__close:hover {
		background: rgb(255 255 255 / 0.1);
		color: white;
	}

	.cohort-modal__body {
		display: grid;
		max-height: calc(90vh - 9rem);
		gap: 1rem;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
	}

	.cohort-modal__footer {
		justify-content: flex-end;
		gap: 0.75rem;
		border-top: 1px solid rgb(255 255 255 / 0.1);
	}

	@media (min-width: 768px) {
		.cohorts-page__container {
			padding: 2rem;
		}

		.retention-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}

		.details-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (max-width: 760px) {
		.cohorts-header,
		.cohorts-header__actions,
		.cohort-modal__footer {
			align-items: stretch;
			flex-direction: column;
		}

		.cohorts-header {
			align-items: flex-start;
		}

		.cohorts-header__actions,
		.cohorts-header__actions :global(.period-selector),
		.granularity-field,
		.primary-action,
		.secondary-action,
		.state-card__button {
			width: 100%;
		}

		.form-grid,
		.details-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.loading-spinner {
			animation: none;
		}

		.primary-action,
		.secondary-action,
		.state-card__button,
		.cohort-tab,
		.cohort-modal__close {
			transition: none;
		}

		.primary-action:hover:not(:disabled),
		.state-card__button:hover,
		.cohort-tab:hover {
			transform: none;
		}
	}

	@keyframes cohorts-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
