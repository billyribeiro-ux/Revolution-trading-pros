<script lang="ts">
	/**
	 * R27-C extraction (2026-05-20): search input + All/Active/Inactive
	 * filter tabs for the coupons list. The parent owns the source of
	 * truth via `$bindable` so both inputs stay in sync with the parent's
	 * `searchQuery` / `filterStatus` runes (no event indirection).
	 */
	import { IconSearch } from '$lib/icons';
	import type { FilterStatus } from './types';

	interface Props {
		searchQuery: string;
		filterStatus: FilterStatus;
		total: number;
		active: number;
	}

	let { searchQuery = $bindable(), filterStatus = $bindable(), total, active }: Props = $props();
	let inactive = $derived(total - active);
</script>

<div class="filters-bar">
	<div class="search-box">
		<label for="search-coupons" class="visually-hidden">Search coupons</label>
		<IconSearch size={18} />
		<input
			type="text"
			id="search-coupons"
			name="search"
			placeholder="Search by code or type..."
			bind:value={searchQuery}
		/>
	</div>
	<div class="filter-tabs">
		<button
			class={{ 'filter-tab': true, active: filterStatus === 'all' }}
			onclick={() => (filterStatus = 'all')}
		>
			All ({total})
		</button>
		<button
			class={{ 'filter-tab': true, active: filterStatus === 'active' }}
			onclick={() => (filterStatus = 'active')}
		>
			Active ({active})
		</button>
		<button
			class={{ 'filter-tab': true, active: filterStatus === 'inactive' }}
			onclick={() => (filterStatus = 'inactive')}
		>
			Inactive ({inactive})
		</button>
	</div>
</div>

<style>
	.filters-bar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: var(--bg-surface);
		border: 1px solid var(--border-default);
		border-radius: 8px;
		flex: 1;
		max-width: 320px;
		color: var(--text-tertiary);
	}

	.search-box:focus-within {
		border-color: var(--primary-500);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.search-box input {
		flex: 1;
		background: transparent;
		border: none;
		color: var(--text-primary);
		font-size: 0.875rem;
		outline: none;
		min-width: 0;
	}

	.search-box input::placeholder {
		color: var(--text-muted);
	}

	.filter-tabs {
		display: flex;
		gap: 0.375rem;
	}

	.filter-tab {
		padding: 0.5rem 0.875rem;
		background: var(--bg-surface);
		border: 1px solid var(--border-default);
		border-radius: 6px;
		color: var(--text-secondary);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.filter-tab:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.filter-tab.active {
		background: rgba(230, 184, 0, 0.12);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	@media (max-width: 767.98px) {
		.filters-bar {
			flex-direction: column;
			align-items: stretch;
		}

		.search-box {
			max-width: none;
		}

		.filter-tabs {
			justify-content: center;
		}
	}

	@media (max-width: 479.98px) {
		.filter-tabs {
			flex-wrap: wrap;
		}

		.filter-tab {
			flex: 1;
			min-width: 80px;
			text-align: center;
		}
	}
</style>
