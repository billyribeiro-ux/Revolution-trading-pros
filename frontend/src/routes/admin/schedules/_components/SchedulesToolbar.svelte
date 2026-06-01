<!--
	SchedulesToolbar — week navigation (◀ label ▶ Today) + filter selects + refresh.
	Extracted from /admin/schedules in R21-C.
-->
<script lang="ts">
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import IconChevronRight from '@tabler/icons-svelte-runes/icons/chevron-right';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';

	type FilterActive = 'all' | 'active' | 'inactive';

	interface Props {
		weekRangeText: string;
		filterActive: FilterActive;
		filterDay: number | null;
		days: readonly string[];
		onnavigateWeek: (direction: number) => void;
		ongoToCurrentWeek: () => void;
		onrefresh: () => void;
		onfilterActiveChange: (value: FilterActive) => void;
		onfilterDayChange: (value: number | null) => void;
	}

	let {
		weekRangeText,
		filterActive,
		filterDay,
		days,
		onnavigateWeek,
		ongoToCurrentWeek,
		onrefresh,
		onfilterActiveChange,
		onfilterDayChange
	}: Props = $props();

	// FIX-2026-04-26 (P3-3): coerce the empty <option value=""> back to null
	// for parents that model "no day filter" as null.
	function handleDayChange(e: Event) {
		const v = (e.currentTarget as HTMLSelectElement).value;
		onfilterDayChange(v === '' ? null : Number(v));
	}
</script>

<div class="toolbar">
	<!-- Week Navigation -->
	<div class="week-nav">
		<button class="btn-icon" onclick={() => onnavigateWeek(-1)} title="Previous week">
			<IconChevronLeft size={20} />
		</button>
		<span class="week-label">{weekRangeText}</span>
		<button class="btn-icon" onclick={() => onnavigateWeek(1)} title="Next week">
			<IconChevronRight size={20} />
		</button>
		<button class="btn btn-text" onclick={ongoToCurrentWeek}>Today</button>
	</div>

	<!-- Filters -->
	<div class="filters">
		<select
			value={filterActive}
			onchange={(e) =>
				onfilterActiveChange((e.currentTarget as HTMLSelectElement).value as FilterActive)}
			class="filter-select"
		>
			<option value="all">All Schedules</option>
			<option value="active">Active Only</option>
			<option value="inactive">Inactive Only</option>
		</select>

		<select value={filterDay ?? ''} onchange={handleDayChange} class="filter-select">
			<!-- FIX-2026-04-26 (P3-3): <option value={null}> silently coerces to "" — use explicit empty string. -->
			<option value="">All Days</option>
			{#each days as day, index (index)}
				<option value={index}>{day}</option>
			{/each}
		</select>

		<button class="btn-icon" onclick={onrefresh} title="Refresh">
			<IconRefresh size={18} />
		</button>
	</div>
</div>

<style>
	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
		margin-bottom: 20px;
		padding: 12px 16px;
		background: #f8fafc;
		border-radius: 8px;
		flex-wrap: wrap;
	}

	.week-nav {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.week-label {
		font-size: 15px;
		font-weight: 600;
		color: #1e293b;
		min-width: 180px;
		text-align: center;
	}

	.filters {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.filter-select {
		padding: 8px 12px;
		border: 1px solid #e2e8f0;
		border-radius: 6px;
		font-size: 13px;
		background: #fff;
		cursor: pointer;
	}

	.filter-select:focus {
		outline: none;
		border-color: #143e59;
	}

	.btn-icon {
		padding: 8px;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 6px;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
	}

	.btn-text {
		background: none;
		border: none;
		color: #143e59;
		font-weight: 600;
		cursor: pointer;
		padding: 8px 12px;
	}

	.btn-text:hover {
		text-decoration: underline;
	}

	@media (max-width: 767.98px) {
		.toolbar {
			flex-direction: column;
			align-items: stretch;
		}

		.week-nav {
			justify-content: center;
		}

		.filters {
			justify-content: center;
		}
	}
</style>
