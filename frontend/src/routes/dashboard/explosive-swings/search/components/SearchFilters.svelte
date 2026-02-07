<script lang="ts">
	/**
	 * SearchFilters - Filter controls for content type, date range, and ticker
	 * @standards Svelte 5 January 2026 | Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import type { SearchableType, DateRange } from '../search.state.svelte';

	interface Props {
		selectedTypes: SearchableType[];
		dateRange: DateRange;
		tickerFilter: string | null;
		onToggleType: (type: SearchableType) => void;
		onSetDateRange: (from: string | null, to: string | null) => void;
		onSetTicker: (ticker: string | null) => void;
		onClearFilters: () => void;
	}

	let {
		selectedTypes,
		dateRange,
		tickerFilter,
		onToggleType,
		onSetDateRange,
		onSetTicker,
		onClearFilters
	}: Props = $props();

	let showDatePicker = $state(false);
	let showTickerInput = $state(false);
	let tickerInputValue = $state('');

	// Computed: has any active filter
	const hasActiveFilters = $derived(
		selectedTypes.length < 3 ||
			dateRange.from !== null ||
			dateRange.to !== null ||
			tickerFilter !== null
	);

	// Content type labels
	const typeLabels: Record<SearchableType, string> = {
		alerts: 'Alerts',
		trades: 'Trades',
		trade_plans: 'Trade Plans'
	};

	function handleTickerSubmit() {
		if (tickerInputValue.trim()) {
			onSetTicker(tickerInputValue.trim().toUpperCase());
		}
		showTickerInput = false;
		tickerInputValue = '';
	}

	function clearTicker() {
		onSetTicker(null);
	}

	function handleDateChange(type: 'from' | 'to', event: Event) {
		const target = event.target as HTMLInputElement;
		const value = target.value || null;

		if (type === 'from') {
			onSetDateRange(value, dateRange.to);
		} else {
			onSetDateRange(dateRange.from, value);
		}
	}

	function clearDates() {
		onSetDateRange(null, null);
		showDatePicker = false;
	}

	function _formatDate(dateStr: string): string {
		const date = new Date(dateStr + 'T00:00:00');
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}
	void _formatDate;
</script>

<div class="filters-container">
	<!-- Content Type Filters -->
	<div class="filter-group">
		<span class="filter-label">Show:</span>
		<div class="type-filters">
			{#each ['alerts', 'trades', 'trade_plans'] as const as type}
				<button
					class="type-btn"
					class:active={selectedTypes.includes(type)}
					onclick={() => onToggleType(type)}
					aria-pressed={selectedTypes.includes(type)}
				>
					{#if type === 'alerts'}
						<svg
							width="14"
							height="14"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
							<path d="M13.73 21a2 2 0 0 1-3.46 0" />
						</svg>
					{:else if type === 'trades'}
						<svg
							width="14"
							height="14"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<polyline points="22 7 13.5 15.5 8.5 10.5 2 17" />
							<polyline points="16 7 22 7 22 13" />
						</svg>
					{:else}
						<svg
							width="14"
							height="14"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
							<polyline points="14 2 14 8 20 8" />
							<line x1="16" y1="13" x2="8" y2="13" />
							<line x1="16" y1="17" x2="8" y2="17" />
						</svg>
					{/if}
					{typeLabels[type]}
				</button>
			{/each}
		</div>
	</div>

	<!-- Date Range Filter -->
	<div class="filter-group">
		{#if !showDatePicker && !dateRange.from && !dateRange.to}
			<button class="filter-add-btn" onclick={() => (showDatePicker = true)}>
				<svg
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
					<line x1="16" y1="2" x2="16" y2="6" />
					<line x1="8" y1="2" x2="8" y2="6" />
					<line x1="3" y1="10" x2="21" y2="10" />
				</svg>
				Date Range
			</button>
		{:else}
			<div class="date-filter">
				{#if showDatePicker || dateRange.from || dateRange.to}
					<div class="date-inputs">
						<input
							id="date-from"
							name="date-from"
							type="date"
							class="date-input"
							value={dateRange.from ?? ''}
							onchange={(e) => handleDateChange('from', e)}
							placeholder="From"
						/>
						<span class="date-separator">to</span>
						<input
							id="date-to"
							name="date-to"
							type="date"
							class="date-input"
							value={dateRange.to ?? ''}
							onchange={(e) => handleDateChange('to', e)}
							placeholder="To"
						/>
						<button class="filter-clear-btn" onclick={clearDates} aria-label="Clear date filter">
							<svg
								width="14"
								height="14"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<line x1="18" y1="6" x2="6" y2="18" />
								<line x1="6" y1="6" x2="18" y2="18" />
							</svg>
						</button>
					</div>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Ticker Filter -->
	<div class="filter-group">
		{#if !showTickerInput && !tickerFilter}
			<button class="filter-add-btn" onclick={() => (showTickerInput = true)}>
				<svg
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<line x1="12" y1="1" x2="12" y2="23" />
					<path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
				</svg>
				Ticker
			</button>
		{:else if tickerFilter}
			<div class="ticker-badge-wrapper">
				<span class="ticker-badge">{tickerFilter}</span>
				<button class="filter-clear-btn" onclick={clearTicker} aria-label="Clear ticker filter">
					<svg
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<line x1="18" y1="6" x2="6" y2="18" />
						<line x1="6" y1="6" x2="18" y2="18" />
					</svg>
				</button>
			</div>
		{:else}
			<div class="ticker-input-wrapper">
				<input
					id="ticker-filter"
					name="ticker-filter"
					type="text"
					class="ticker-input"
					placeholder="e.g. NVDA"
					bind:value={tickerInputValue}
					onkeydown={(e) => e.key === 'Enter' && handleTickerSubmit()}
					maxlength="5"
				/>
				<button class="ticker-submit-btn" onclick={handleTickerSubmit}> Add </button>
				<button
					class="filter-clear-btn"
					onclick={() => {
						showTickerInput = false;
						tickerInputValue = '';
					}}
					aria-label="Cancel"
				>
					<svg
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<line x1="18" y1="6" x2="6" y2="18" />
						<line x1="6" y1="6" x2="18" y2="18" />
					</svg>
				</button>
			</div>
		{/if}
	</div>

	<!-- Clear All Filters -->
	{#if hasActiveFilters}
		<button class="clear-all-btn" onclick={onClearFilters}> Clear Filters </button>
	{/if}
</div>

<style>
	.filters-container {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 12px;
		padding: 16px;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		margin-top: 16px;
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.filter-label {
		font-size: 13px;
		font-weight: 500;
		color: var(--color-text-tertiary);
	}

	/* Type Filters */
	.type-filters {
		display: flex;
		gap: 4px;
	}

	.type-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		background: var(--color-bg-subtle);
		border: 1px solid var(--color-border-default);
		border-radius: 6px;
		font-size: 13px;
		font-weight: 500;
		color: var(--color-text-secondary);
		cursor: pointer;
		transition: all 0.15s;
	}

	.type-btn:hover {
		border-color: var(--color-border-hover);
		color: var(--color-text-primary);
	}

	.type-btn.active {
		background: var(--color-brand-primary);
		border-color: var(--color-brand-primary);
		color: white;
	}

	/* Add Filter Buttons */
	.filter-add-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		background: transparent;
		border: 1px dashed var(--color-border-default);
		border-radius: 6px;
		font-size: 13px;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: all 0.15s;
	}

	.filter-add-btn:hover {
		border-color: var(--color-brand-primary);
		color: var(--color-brand-primary);
	}

	/* Date Filter */
	.date-filter {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.date-inputs {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 4px 8px;
		background: var(--color-bg-subtle);
		border: 1px solid var(--color-border-default);
		border-radius: 6px;
	}

	.date-input {
		width: 120px;
		padding: 4px 8px;
		border: none;
		background: transparent;
		font-size: 13px;
		color: var(--color-text-primary);
		outline: none;
	}

	.date-separator {
		font-size: 12px;
		color: var(--color-text-tertiary);
	}

	/* Ticker Filter */
	.ticker-badge-wrapper {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.ticker-badge {
		display: inline-flex;
		padding: 4px 10px;
		background: var(--color-brand-primary);
		color: white;
		font-size: 12px;
		font-weight: 700;
		border-radius: 4px;
		letter-spacing: 0.5px;
	}

	.ticker-input-wrapper {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 4px 8px;
		background: var(--color-bg-subtle);
		border: 1px solid var(--color-border-default);
		border-radius: 6px;
	}

	.ticker-input {
		width: 70px;
		padding: 4px;
		border: none;
		background: transparent;
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text-primary);
		text-transform: uppercase;
		outline: none;
	}

	.ticker-submit-btn {
		padding: 4px 8px;
		background: var(--color-brand-primary);
		border: none;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
		color: white;
		cursor: pointer;
	}

	.ticker-submit-btn:hover {
		background: var(--color-brand-primary-hover);
	}

	/* Clear Button */
	.filter-clear-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 4px;
		background: transparent;
		border: none;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: color 0.15s;
	}

	.filter-clear-btn:hover {
		color: var(--color-loss);
	}

	/* Clear All */
	.clear-all-btn {
		margin-left: auto;
		padding: 6px 12px;
		background: transparent;
		border: none;
		font-size: 13px;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: color 0.15s;
	}

	.clear-all-btn:hover {
		color: var(--color-brand-primary);
	}

	/* Mobile */
	@media (max-width: 768px) {
		.filters-container {
			padding: 12px;
		}

		.type-filters {
			flex-wrap: wrap;
		}

		.date-inputs {
			flex-wrap: wrap;
		}

		.date-input {
			width: 100px;
		}
	}
</style>
