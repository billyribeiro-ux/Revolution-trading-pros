<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { writable } from 'svelte/store';

	const dispatch = createEventDispatcher();

	export let filters = {
		dateRange: {
			from: '',
			to: '',
			preset: 'last_30_days'
		},
		widgetTypes: [] as string[],
		metrics: {
			min: null as number | null,
			max: null as number | null
		},
		status: [] as string[],
		tags: [] as string[]
	};

	const datePresets = [
		{ value: 'today', label: 'Today' },
		{ value: 'yesterday', label: 'Yesterday' },
		{ value: 'last_7_days', label: 'Last 7 Days' },
		{ value: 'last_30_days', label: 'Last 30 Days' },
		{ value: 'last_90_days', label: 'Last 90 Days' },
		{ value: 'this_month', label: 'This Month' },
		{ value: 'last_month', label: 'Last Month' },
		{ value: 'this_year', label: 'This Year' },
		{ value: 'custom', label: 'Custom Range' }
	];

	const widgetTypeOptions = [
		'system_health',
		'revenue_mrr',
		'user_growth',
		'subscription_churn',
		'email_performance',
		'crm_pipeline',
		'recent_activity',
		'trading_performance',
		'notifications'
	];

	const statusOptions = ['active', 'inactive', 'warning', 'error'];

	let showCustomDateRange = false;

	function handlePresetChange() {
		showCustomDateRange = filters.dateRange.preset === 'custom';

		if (!showCustomDateRange) {
			const dates = calculatePresetDates(filters.dateRange.preset);
			filters.dateRange.from = dates.from;
			filters.dateRange.to = dates.to;
		}

		applyFilters();
	}

	function calculatePresetDates(preset: string): { from: string; to: string } {
		const now = new Date();
		const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

		let from: Date;
		let to: Date = today;

		switch (preset) {
			case 'today':
				from = today;
				break;
			case 'yesterday':
				from = new Date(today);
				from.setDate(from.getDate() - 1);
				to = from;
				break;
			case 'last_7_days':
				from = new Date(today);
				from.setDate(from.getDate() - 7);
				break;
			case 'last_30_days':
				from = new Date(today);
				from.setDate(from.getDate() - 30);
				break;
			case 'last_90_days':
				from = new Date(today);
				from.setDate(from.getDate() - 90);
				break;
			case 'this_month':
				from = new Date(now.getFullYear(), now.getMonth(), 1);
				break;
			case 'last_month':
				from = new Date(now.getFullYear(), now.getMonth() - 1, 1);
				to = new Date(now.getFullYear(), now.getMonth(), 0);
				break;
			case 'this_year':
				from = new Date(now.getFullYear(), 0, 1);
				break;
			default:
				from = today;
		}

		return {
			from: from.toISOString().split('T')[0],
			to: to.toISOString().split('T')[0]
		};
	}

	function applyFilters() {
		dispatch('apply', filters);
	}

	function resetFilters() {
		filters = {
			dateRange: {
				from: '',
				to: '',
				preset: 'last_30_days'
			},
			widgetTypes: [],
			metrics: {
				min: null,
				max: null
			},
			status: [],
			tags: []
		};
		handlePresetChange();
	}

	function toggleWidgetType(type: string) {
		const index = filters.widgetTypes.indexOf(type);
		if (index > -1) {
			filters.widgetTypes.splice(index, 1);
		} else {
			filters.widgetTypes.push(type);
		}
		filters.widgetTypes = [...filters.widgetTypes];
		applyFilters();
	}

	function toggleStatus(status: string) {
		const index = filters.status.indexOf(status);
		if (index > -1) {
			filters.status.splice(index, 1);
		} else {
			filters.status.push(status);
		}
		filters.status = [...filters.status];
		applyFilters();
	}

	// Initialize
	handlePresetChange();
</script>

<div class="advanced-filters">
	<div class="filter-header">
		<h3>Advanced Filters</h3>
		<button class="btn-reset" on:click={resetFilters}>Reset All</button>
	</div>

	<div class="filter-section">
		<h4>Date Range</h4>
		<div class="form-group">
			<label for="date-preset">Preset</label>
			<select id="date-preset" bind:value={filters.dateRange.preset} on:change={handlePresetChange}>
				{#each datePresets as preset}
					<option value={preset.value}>{preset.label}</option>
				{/each}
			</select>
		</div>

		{#if showCustomDateRange}
			<div class="date-range-inputs">
				<div class="form-group">
					<label for="date-from">From</label>
					<input
						id="date-from"
						type="date"
						bind:value={filters.dateRange.from}
						on:change={applyFilters}
					/>
				</div>
				<div class="form-group">
					<label for="date-to">To</label>
					<input
						id="date-to"
						type="date"
						bind:value={filters.dateRange.to}
						on:change={applyFilters}
					/>
				</div>
			</div>
		{/if}
	</div>

	<div class="filter-section">
		<h4>Widget Types</h4>
		<div class="checkbox-grid">
			{#each widgetTypeOptions as type}
				<label class="checkbox-label">
					<input
						type="checkbox"
						checked={filters.widgetTypes.includes(type)}
						on:change={() => toggleWidgetType(type)}
					/>
					<span>{type.replace(/_/g, ' ')}</span>
				</label>
			{/each}
		</div>
	</div>

	<div class="filter-section">
		<h4>Metric Range</h4>
		<div class="range-inputs">
			<div class="form-group">
				<label for="metric-min">Min Value</label>
				<input
					id="metric-min"
					type="number"
					placeholder="0"
					bind:value={filters.metrics.min}
					on:change={applyFilters}
				/>
			</div>
			<div class="form-group">
				<label for="metric-max">Max Value</label>
				<input
					id="metric-max"
					type="number"
					placeholder="1000"
					bind:value={filters.metrics.max}
					on:change={applyFilters}
				/>
			</div>
		</div>
	</div>

	<div class="filter-section">
		<h4>Status</h4>
		<div class="status-pills">
			{#each statusOptions as status}
				<button
					class="status-pill"
					class:active={filters.status.includes(status)}
					on:click={() => toggleStatus(status)}
				>
					{status}
				</button>
			{/each}
		</div>
	</div>

	<div class="filter-actions">
		<button class="btn-apply" on:click={applyFilters}>Apply Filters</button>
	</div>
</div>

<style>
	.advanced-filters {
		background: white;
		border-radius: 12px;
		padding: 1.5rem;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.filter-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	h4 {
		font-size: 0.95rem;
		font-weight: 600;
		color: #374151;
		margin: 0 0 0.75rem 0;
	}

	.btn-reset {
		padding: 0.5rem 1rem;
		background: #f3f4f6;
		color: #374151;
		border: none;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
	}

	.btn-reset:hover {
		background: #e5e7eb;
	}

	.filter-section {
		margin-bottom: 1.5rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.filter-section:last-of-type {
		border-bottom: none;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	select,
	input[type='date'],
	input[type='number'] {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.date-range-inputs,
	.range-inputs {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.checkbox-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.75rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
		text-transform: capitalize;
	}

	.status-pills {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.status-pill {
		padding: 0.5rem 1rem;
		background: #f3f4f6;
		color: #374151;
		border: 1px solid #d1d5db;
		border-radius: 20px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		text-transform: capitalize;
		transition: all 0.2s;
	}

	.status-pill:hover {
		background: #e5e7eb;
	}

	.status-pill.active {
		background: #3b82f6;
		color: white;
		border-color: #3b82f6;
	}

	.filter-actions {
		margin-top: 1.5rem;
	}

	.btn-apply {
		width: 100%;
		padding: 0.75rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-apply:hover {
		background: #2563eb;
	}
</style>
