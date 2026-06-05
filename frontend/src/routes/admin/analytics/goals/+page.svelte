<script lang="ts">
	/**
	 * Conversion Goals - Goal Tracking Management
	 * Apple ICT7 Grade Implementation
	 *
	 * Create, manage, and track conversion goals
	 * with real-time progress monitoring.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconDots from '@tabler/icons-svelte-runes/icons/dots-vertical';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import { toastStore } from '$lib/stores/toast.svelte';

	interface Goal {
		id: string;
		name: string;
		description?: string;
		type: 'event' | 'pageview' | 'revenue' | 'duration';
		target_event?: string;
		target_url?: string;
		target_value?: number;
		current_value: number;
		completions: number;
		conversion_rate: number;
		status: 'active' | 'paused' | 'completed';
		created_at: string;
	}

	// Svelte 5 Runes - State
	let goals = $state<Goal[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('30d');
	let showCreateModal = $state(false);
	let activeFilter = $state<'all' | 'active' | 'paused' | 'completed'>('all');

	// New goal form
	let newGoal = $state({
		name: '',
		description: '',
		type: 'event' as 'event' | 'pageview' | 'revenue' | 'duration',
		target_event: '',
		target_url: '',
		target_value: 0
	});

	// Goal type options
	const goalTypes = [
		{
			value: 'event',
			label: 'Event Goal',
			icon: 'M13 10V3L4 14h7v7l9-11h-7z',
			description: 'Track specific events'
		},
		{
			value: 'pageview',
			label: 'Pageview Goal',
			icon: 'M15 12a3 3 0 11-6 0 3 3 0 016 0zM2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z',
			description: 'Track page visits'
		},
		{
			value: 'revenue',
			label: 'Revenue Goal',
			icon: 'M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
			description: 'Track revenue targets'
		},
		{
			value: 'duration',
			label: 'Duration Goal',
			icon: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z',
			description: 'Track time on site'
		}
	];

	async function loadGoals() {
		loading = true;
		error = null;
		try {
			const params = new URLSearchParams({ period: selectedPeriod });
			const response = await fetch(`/api/admin/analytics/goals?${params.toString()}`);
			if (!response.ok) {
				// FIX-2026-04-26 (audit 08-analytics §P1-4): surface real upstream
				// status instead of silently zeroing the list.
				throw new Error(`Failed to load goals (HTTP ${response.status})`);
			}
			const data = await response.json();
			goals = data.goals || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load goals';
			goals = [];
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		loadGoals();
	}

	async function createGoal() {
		try {
			// Prepared for API integration
			const response = await fetch('/api/admin/analytics/goals', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(newGoal)
			});
			if (response.ok) {
				showCreateModal = false;
				newGoal = {
					name: '',
					description: '',
					type: 'event',
					target_event: '',
					target_url: '',
					target_value: 0
				};
				loadGoals();
			}
		} catch (e) {
			toastStore.error(e instanceof Error ? e.message : 'Failed to create goal');
		}
	}

	function getProgressTone(rate: number): 'complete' | 'strong' | 'warning' | 'danger' {
		if (rate >= 100) return 'complete';
		if (rate >= 75) return 'strong';
		if (rate >= 50) return 'warning';
		return 'danger';
	}

	// FIX-2026-04-26 (P1-3): see analytics/+page.svelte — $derived restores
	// reactivity that the helper's `untrack` would otherwise discard.
	let isAnalyticsConnected = $derived(getIsAnalyticsConnected());

	// FIX-2026-04-26 (P1-1): onMount replaces the $effect cascade pattern.
	onMount(() => {
		if (!browser) return;

		(async () => {
			try {
				await connections.load();
			} catch (e) {
				if (import.meta.env.DEV) {
					console.error('[Goals] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadGoals();
			} else {
				loading = false;
			}
		})();
	});

	// Derived filtered goals
	const filteredGoals = $derived(
		goals.filter((goal) => {
			if (activeFilter === 'all') return true;
			return goal.status === activeFilter;
		})
	);

	// Derived stats
	const stats = $derived({
		total: goals.length,
		active: goals.filter((g) => g.status === 'active').length,
		completed: goals.filter((g) => g.status === 'completed').length,
		totalCompletions: goals.reduce((sum, g) => sum + g.completions, 0)
	});
</script>

<svelte:head>
	<title>Conversion Goals | Analytics</title>
</svelte:head>

<div class="goals-page">
	<div class="goals-page__container">
		<!-- Apple ICT7 Grade Header -->
		<header class="goals-header">
			<div class="goals-header__title-group">
				<div class="goals-header__icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check (goal) -->
					<IconCircleCheck size={24} aria-hidden="true" />
				</div>
				<div>
					<h1>Conversion Goals</h1>
					<p>Track and measure your conversion objectives</p>
				</div>
			</div>
			{#if isAnalyticsConnected}
				<div class="goals-header__actions">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
					<button onclick={() => (showCreateModal = true)} class="primary-action">
						Create Goal
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
		{:else}
			<!-- Stats Grid -->
			<div class="goals-stats">
				<div class="goals-stat">
					<div class="goals-stat__value">{stats.total}</div>
					<div class="goals-stat__label">Total Goals</div>
				</div>
				<div class="goals-stat">
					<div class="goals-stat__value" data-tone="emerald">{stats.active}</div>
					<div class="goals-stat__label">Active Goals</div>
				</div>
				<div class="goals-stat">
					<div class="goals-stat__value" data-tone="blue">{stats.completed}</div>
					<div class="goals-stat__label">Completed</div>
				</div>
				<div class="goals-stat">
					<div class="goals-stat__value" data-tone="amber">
						{stats.totalCompletions.toLocaleString()}
					</div>
					<div class="goals-stat__label">Total Completions</div>
				</div>
			</div>

			<!-- Filters -->
			<div class="goal-filters">
				{#each [{ value: 'all', label: 'All Goals' }, { value: 'active', label: 'Active' }, { value: 'paused', label: 'Paused' }, { value: 'completed', label: 'Completed' }] as filter (filter.value)}
					<button
						onclick={() => (activeFilter = filter.value as typeof activeFilter)}
						class={{ 'goal-filter': true, 'is-active': activeFilter === filter.value }}
					>
						{filter.label}
					</button>
				{/each}
			</div>

			{#if loading}
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
					<button onclick={loadGoals} class="state-card__button state-card__button--error">
						Retry
					</button>
				</div>
			{:else if filteredGoals.length === 0}
				<div class="state-card">
					<div class="state-card__icon state-card__icon--emerald">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check (no goals empty state) -->
						<IconCircleCheck size={32} aria-hidden="true" />
					</div>
					<h3>No Goals Configured</h3>
					<p>Set up conversion goals to track important user actions</p>
					<button
						onclick={() => (showCreateModal = true)}
						class="state-card__button state-card__button--primary"
					>
						Create Your First Goal
					</button>
				</div>
			{:else}
				<!-- Goals Grid -->
				<div class="goals-grid">
					{#each filteredGoals as goal (goal.id)}
						<div class="goal-card">
							<div class="goal-card__header">
								<div class="goal-card__title-group">
									<div class="goal-card__icon">
										<svg
											class="goal-card__svg"
											fill="none"
											stroke="currentColor"
											viewBox="0 0 24 24"
										>
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d={goalTypes.find((t) => t.value === goal.type)?.icon ||
													'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z'}
											/>
										</svg>
									</div>
									<div>
										<h3>{goal.name}</h3>
										<span class="goal-status" data-status={goal.status}>
											{goal.status}
										</span>
									</div>
								</div>
								<button class="goal-card__menu" aria-label="More options">
									<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: dots-vertical (menu) -->
									<IconDots size={20} aria-hidden="true" />
								</button>
							</div>

							{#if goal.description}
								<p class="goal-card__description">{goal.description}</p>
							{/if}

							<!-- Progress Bar -->
							<div class="goal-progress">
								<div class="goal-progress__header">
									<span>Progress</span>
									<strong>{goal.conversion_rate.toFixed(1)}%</strong>
								</div>
								<div class="goal-progress__track">
									<div
										class="goal-progress__bar"
										data-tone={getProgressTone(goal.conversion_rate)}
										style:width={`${Math.min(100, goal.conversion_rate)}%`}
									></div>
								</div>
							</div>

							<div class="goal-card__meta">
								<span>{goal.completions.toLocaleString()} completions</span>
								<span>{goal.type}</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Create Goal Modal -->
{#if showCreateModal}
	<div class="goal-modal-backdrop">
		<div class="goal-modal" role="dialog" aria-modal="true" aria-labelledby="goal-modal-title">
			<div class="goal-modal__header">
				<div class="goal-modal__title-row">
					<h2 id="goal-modal-title">Create Goal</h2>
					<button
						onclick={() => (showCreateModal = false)}
						class="goal-modal__close"
						aria-label="Close modal"
					>
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (close) -->
						<IconX size={20} aria-hidden="true" />
					</button>
				</div>
			</div>

			<div class="goal-modal__body">
				<!-- Basic Info -->
				<div class="form-stack">
					<div class="form-field">
						<label for="goal-name">Name</label>
						<input
							id="goal-name"
							name="goal-name"
							type="text"
							bind:value={newGoal.name}
							placeholder="e.g., Newsletter Signup"
							class="form-control"
						/>
					</div>
					<div class="form-field">
						<label for="goal-description">Description</label>
						<textarea
							id="goal-description"
							bind:value={newGoal.description}
							placeholder="Describe this goal..."
							rows={2}
							class="form-control form-control--textarea"
						></textarea>
					</div>
				</div>

				<!-- Goal Type Selection -->
				<div>
					<span class="type-picker-label">Goal Type</span>
					<div class="type-picker">
						{#each goalTypes as type (type.value)}
							<button
								onclick={() => (newGoal.type = type.value as typeof newGoal.type)}
								class={{ 'type-option': true, 'is-selected': newGoal.type === type.value }}
							>
								<svg
									class="type-option__icon"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d={type.icon}
									/>
								</svg>
								<div class="type-option__label">{type.label}</div>
								<div class="type-option__description">{type.description}</div>
							</button>
						{/each}
					</div>
				</div>

				<!-- Conditional Fields -->
				{#if newGoal.type === 'event'}
					<div class="form-field">
						<label for="target-event">Target Event</label>
						<input
							id="target-event"
							name="target-event"
							type="text"
							bind:value={newGoal.target_event}
							placeholder="e.g., signup_completed"
							class="form-control"
						/>
					</div>
				{:else if newGoal.type === 'pageview'}
					<div class="form-field">
						<label for="target-url">Target URL</label>
						<input
							id="target-url"
							name="target-url"
							type="text"
							bind:value={newGoal.target_url}
							placeholder="e.g., /thank-you"
							class="form-control"
						/>
					</div>
				{:else if newGoal.type === 'revenue' || newGoal.type === 'duration'}
					<div class="form-field">
						<label for="target-value">
							{newGoal.type === 'revenue' ? 'Target Revenue ($)' : 'Target Duration (seconds)'}
						</label>
						<input
							id="target-value"
							name="target-value"
							type="number"
							bind:value={newGoal.target_value}
							placeholder={newGoal.type === 'revenue' ? 'e.g., 10000' : 'e.g., 300'}
							class="form-control"
						/>
					</div>
				{/if}
			</div>

			<div class="goal-modal__footer">
				<button onclick={() => (showCreateModal = false)} class="secondary-action"> Cancel </button>
				<button onclick={createGoal} disabled={!newGoal.name} class="primary-action">
					Create Goal
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.goals-page {
		min-height: 100vh;
		background:
			radial-gradient(circle at top left, rgb(16 185 129 / 0.16), transparent 28rem),
			linear-gradient(135deg, #020617 0%, #0f172a 48%, #020617 100%);
		color: white;
	}

	.goals-page__container {
		width: min(100%, 80rem);
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	.goals-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.goals-header__title-group {
		display: flex;
		align-items: center;
		gap: 1rem;
		min-width: 0;
	}

	.goals-header__icon {
		display: inline-grid;
		width: 3rem;
		height: 3rem;
		flex: 0 0 auto;
		place-items: center;
		border-radius: 1rem;
		background: linear-gradient(135deg, #10b981, #14b8a6);
		box-shadow: 0 18px 44px rgb(16 185 129 / 0.24);
	}

	.goals-header h1 {
		margin: 0;
		font-size: clamp(1.875rem, 4vw, 2.5rem);
		font-weight: 700;
		line-height: 1.1;
	}

	.goals-header p {
		margin: 0.35rem 0 0;
		color: #94a3b8;
	}

	.goals-header__actions {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
		justify-content: flex-end;
	}

	.primary-action,
	.secondary-action,
	.goal-filter,
	.state-card__button,
	.goal-card__menu,
	.goal-modal__close,
	.type-option {
		border: 0;
		font: inherit;
	}

	.primary-action,
	.secondary-action,
	.state-card__button,
	.goal-filter {
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
		background: linear-gradient(135deg, #059669, #0d9488);
		color: white;
		box-shadow: 0 16px 36px rgb(13 148 136 / 0.28);
	}

	.primary-action:hover:not(:disabled),
	.state-card__button:hover,
	.goal-filter:hover {
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

	.loading-state {
		display: grid;
		min-height: 18rem;
		place-items: center;
	}

	.loading-spinner {
		width: 2rem;
		height: 2rem;
		border: 2px solid rgb(16 185 129 / 0.22);
		border-top-color: #10b981;
		border-radius: 999px;
		animation: goals-spin 780ms linear infinite;
	}

	.loading-spinner--lg {
		width: 3rem;
		height: 3rem;
	}

	.goals-stats {
		display: grid;
		grid-template-columns: repeat(1, minmax(0, 1fr));
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.goals-stat,
	.goal-card,
	.state-card,
	.goal-modal {
		border: 1px solid rgb(255 255 255 / 0.1);
		background: rgb(255 255 255 / 0.05);
		box-shadow: 0 24px 80px rgb(2 6 23 / 0.24);
		backdrop-filter: blur(24px);
	}

	.goals-stat {
		border-radius: 1rem;
		padding: 1.5rem;
	}

	.goals-stat__value {
		color: white;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1;
	}

	.goals-stat__value[data-tone='emerald'] {
		color: #34d399;
	}

	.goals-stat__value[data-tone='blue'] {
		color: #60a5fa;
	}

	.goals-stat__value[data-tone='amber'] {
		color: #fbbf24;
	}

	.goals-stat__label {
		margin-top: 0.45rem;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.goal-filters {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin-bottom: 2rem;
	}

	.goal-filter {
		border: 1px solid rgb(255 255 255 / 0.1);
		background: rgb(255 255 255 / 0.05);
		color: #cbd5e1;
	}

	.goal-filter.is-active {
		border-color: rgb(255 255 255 / 0.28);
		background: white;
		color: #020617;
		box-shadow: 0 12px 34px rgb(255 255 255 / 0.1);
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

	.state-card__icon--emerald {
		color: #34d399;
		background: rgb(16 185 129 / 0.12);
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
		background: linear-gradient(135deg, #059669, #0d9488);
		box-shadow: 0 16px 36px rgb(13 148 136 / 0.24);
	}

	.state-card__button--error {
		background: linear-gradient(135deg, #dc2626, #f97316);
		box-shadow: 0 16px 36px rgb(220 38 38 / 0.24);
	}

	.goals-grid {
		display: grid;
		grid-template-columns: repeat(1, minmax(0, 1fr));
		gap: 1.5rem;
	}

	.goal-card {
		border-radius: 1rem;
		padding: 1.5rem;
		transition:
			transform 180ms ease,
			border-color 180ms ease,
			background 180ms ease;
	}

	.goal-card:hover {
		border-color: rgb(255 255 255 / 0.18);
		background: rgb(255 255 255 / 0.08);
		transform: translateY(-2px);
	}

	.goal-card__header,
	.goal-card__title-group,
	.goal-progress__header,
	.goal-card__meta,
	.goal-modal__title-row,
	.goal-modal__footer {
		display: flex;
		align-items: center;
	}

	.goal-card__header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.goal-card__title-group {
		gap: 0.75rem;
		min-width: 0;
	}

	.goal-card__icon {
		display: inline-grid;
		width: 2.5rem;
		height: 2.5rem;
		flex: 0 0 auto;
		place-items: center;
		border-radius: 0.75rem;
		color: #34d399;
		background: rgb(16 185 129 / 0.12);
	}

	.goal-card__svg,
	.type-option__icon {
		width: 1.5rem;
		height: 1.5rem;
	}

	.goal-card h3 {
		margin: 0;
		overflow: hidden;
		color: white;
		font-size: 1rem;
		font-weight: 600;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.goal-status {
		display: inline-flex;
		margin-top: 0.25rem;
		border-radius: 999px;
		padding: 0.125rem 0.5rem;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.goal-status[data-status='active'] {
		background: rgb(16 185 129 / 0.14);
		color: #6ee7b7;
	}

	.goal-status[data-status='paused'] {
		background: rgb(245 158 11 / 0.14);
		color: #fcd34d;
	}

	.goal-status[data-status='completed'] {
		background: rgb(59 130 246 / 0.14);
		color: #93c5fd;
	}

	.goal-card__menu,
	.goal-modal__close {
		display: inline-grid;
		width: 2.25rem;
		height: 2.25rem;
		place-items: center;
		border-radius: 0.625rem;
		background: transparent;
		color: #94a3b8;
		transition:
			background 180ms ease,
			color 180ms ease;
	}

	.goal-card__menu:hover,
	.goal-modal__close:hover {
		background: rgb(255 255 255 / 0.08);
		color: white;
	}

	.goal-card__description {
		display: -webkit-box;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		overflow: hidden;
		min-height: 2.6rem;
		margin: 0 0 1rem;
		color: #94a3b8;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.goal-progress {
		margin: 1rem 0;
	}

	.goal-progress__header,
	.goal-card__meta {
		justify-content: space-between;
		gap: 1rem;
	}

	.goal-progress__header {
		margin-bottom: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.goal-progress__header strong {
		color: white;
		font-weight: 600;
	}

	.goal-progress__track {
		height: 0.5rem;
		overflow: hidden;
		border-radius: 999px;
		background: rgb(15 23 42 / 0.9);
	}

	.goal-progress__bar {
		height: 100%;
		border-radius: inherit;
		transition: width 500ms ease;
	}

	.goal-progress__bar[data-tone='complete'] {
		background: linear-gradient(90deg, #10b981, #22c55e);
	}

	.goal-progress__bar[data-tone='strong'] {
		background: linear-gradient(90deg, #3b82f6, #06b6d4);
	}

	.goal-progress__bar[data-tone='warning'] {
		background: linear-gradient(90deg, #f59e0b, #f97316);
	}

	.goal-progress__bar[data-tone='danger'] {
		background: linear-gradient(90deg, #ef4444, #f97316);
	}

	.goal-card__meta {
		color: #64748b;
		font-size: 0.875rem;
	}

	.goal-modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: grid;
		place-items: center;
		padding: 1rem;
		background: rgb(2 6 23 / 0.72);
		backdrop-filter: blur(8px);
	}

	.goal-modal {
		width: min(100%, 36rem);
		max-height: 90vh;
		overflow: hidden;
		border-radius: 1rem;
	}

	.goal-modal__header {
		border-bottom: 1px solid rgb(255 255 255 / 0.1);
		padding: 1.5rem;
	}

	.goal-modal__title-row {
		justify-content: space-between;
		gap: 1rem;
	}

	.goal-modal h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
	}

	.goal-modal__body {
		display: grid;
		max-height: calc(90vh - 9rem);
		gap: 1.5rem;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.goal-modal__footer {
		justify-content: flex-end;
		gap: 1rem;
		border-top: 1px solid rgb(255 255 255 / 0.1);
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
	.type-picker-label {
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.type-picker-label {
		display: block;
		margin-bottom: 0.75rem;
	}

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

	.form-control:focus {
		border-color: #10b981;
		box-shadow: 0 0 0 3px rgb(16 185 129 / 0.16);
		outline: none;
	}

	.form-control::placeholder {
		color: #64748b;
	}

	.form-control--textarea {
		min-height: 5rem;
		resize: vertical;
	}

	.type-picker {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.75rem;
	}

	.type-option {
		display: grid;
		justify-items: start;
		gap: 0.45rem;
		min-height: 8rem;
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 0.75rem;
		background: rgb(255 255 255 / 0.05);
		color: #cbd5e1;
		padding: 1rem;
		text-align: left;
		transition:
			border-color 180ms ease,
			background 180ms ease,
			transform 180ms ease;
	}

	.type-option:hover,
	.type-option.is-selected {
		border-color: rgb(16 185 129 / 0.45);
		background: rgb(16 185 129 / 0.12);
		transform: translateY(-1px);
	}

	.type-option__icon {
		color: #34d399;
	}

	.type-option__label {
		color: white;
		font-weight: 700;
	}

	.type-option__description {
		color: #94a3b8;
		font-size: 0.8125rem;
	}

	@media (min-width: 768px) {
		.goals-page__container {
			padding: 2rem;
		}

		.goals-stats {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}

		.goals-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.goals-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (max-width: 760px) {
		.goals-header,
		.goals-header__actions,
		.goal-modal__footer {
			align-items: stretch;
			flex-direction: column;
		}

		.goals-header {
			align-items: flex-start;
		}

		.goals-header__actions,
		.goals-header__actions :global(.period-selector),
		.primary-action,
		.secondary-action,
		.state-card__button {
			width: 100%;
		}

		.type-picker {
			grid-template-columns: 1fr;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.loading-spinner {
			animation: none;
		}

		.primary-action,
		.secondary-action,
		.goal-filter,
		.state-card__button,
		.goal-card,
		.goal-card__menu,
		.goal-modal__close,
		.type-option,
		.goal-progress__bar {
			transition: none;
		}

		.primary-action:hover:not(:disabled),
		.state-card__button:hover,
		.goal-filter:hover,
		.goal-card:hover,
		.type-option:hover,
		.type-option.is-selected {
			transform: none;
		}
	}

	@keyframes goals-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
