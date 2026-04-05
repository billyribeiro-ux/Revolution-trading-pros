<script lang="ts">
	/**
	 * MobileResponsiveTable - Enterprise Mobile-First Data Table
	 * Adapts between table and card view based on screen size
	 *
	 * @version 1.1.0 - Security hardened
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 * @security XSS protection via DOMPurify sanitization
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import SkeletonLoader from './SkeletonLoader.svelte';
	import ExportButton from './ExportButton.svelte';
	import type { ExportColumn } from '$lib/utils/export';
	import { sanitizeHtml } from '$lib/utils/sanitize';

	// Debounce utility for resize handler
	function debounce<T extends (...args: unknown[]) => void>(fn: T, delay: number): T {
		let timeoutId: ReturnType<typeof setTimeout>;
		return ((...args: Parameters<T>) => {
			clearTimeout(timeoutId);
			timeoutId = setTimeout(() => fn(...args), delay);
		}) as T;
	}

	interface Column {
		key: string;
		label: string;
		sortable?: boolean;
		align?: 'left' | 'center' | 'right';
		render?: (value: unknown, row: Record<string, unknown>) => string;
		mobileHidden?: boolean;
		width?: string;
	}

	interface Props {
		columns?: Column[];
		data?: Record<string, unknown>[];
		loading?: boolean;
		sortBy?: string;
		sortDir?: 'asc' | 'desc';
		selectable?: boolean;
		selectedIds?: (string | number)[];
		idKey?: string;
		exportable?: boolean;
		exportFilename?: string;
		emptyMessage?: string;
		mobileBreakpoint?: number;
		onsort?: (data: { key: string; direction: 'asc' | 'desc' }) => void;
		onselect?: (data: { selectedIds: (string | number)[] }) => void;
		onrowClick?: (row: Record<string, unknown>) => void;
	}

	let props: Props = $props();
	let columns = $derived(props.columns ?? []);
	let data = $derived(props.data ?? []);
	let loading = $derived(props.loading ?? false);
	let sortBy = $state('');
	let sortDir = $state<'asc' | 'desc'>('asc');
	let selectable = $derived(props.selectable ?? false);
	let selectedIds = $state<(string | number)[]>([]);
	let idKey = $derived(props.idKey ?? 'id');
	let exportable = $derived(props.exportable ?? false);
	let exportFilename = $derived(props.exportFilename ?? 'export');
	let emptyMessage = $derived(props.emptyMessage ?? 'No data available');
	let mobileBreakpoint = $derived(props.mobileBreakpoint ?? 768);
	let onsort = $derived(props.onsort);
	let onselect = $derived(props.onselect);
	let onrowClick = $derived(props.onrowClick);

	// Sync with external prop changes
	$effect(() => {
		if (props.sortBy !== undefined && props.sortBy !== sortBy) {
			sortBy = props.sortBy;
		}
	});
	$effect(() => {
		if (props.sortDir !== undefined && props.sortDir !== sortDir) {
			sortDir = props.sortDir;
		}
	});
	$effect(() => {
		if (props.selectedIds !== undefined) {
			selectedIds = props.selectedIds;
		}
	});

	let isMobile = $state(false);

	function handleSort(column: Column) {
		if (!column.sortable) return;

		if (sortBy === column.key) {
			sortDir = sortDir === 'asc' ? 'desc' : 'asc';
		} else {
			sortBy = column.key;
			sortDir = 'asc';
		}

		onsort?.({ key: sortBy, direction: sortDir });
	}

	function toggleSelect(id: string | number) {
		if (selectedIds.includes(id)) {
			selectedIds = selectedIds.filter((i) => i !== id);
		} else {
			selectedIds = [...selectedIds, id];
		}
		onselect?.({ selectedIds });
	}

	function toggleSelectAll() {
		if (selectedIds.length === data.length) {
			selectedIds = [];
		} else {
			selectedIds = data.map((row) => row[idKey] as string | number);
		}
		onselect?.({ selectedIds });
	}

	function handleRowClick(row: Record<string, unknown>) {
		onrowClick?.(row);
	}

	/**
	 * Get sanitized value for display - prevents XSS attacks
	 * @security All output is sanitized through DOMPurify
	 */
	function getValue(row: Record<string, unknown>, column: Column): string {
		const value = row[column.key];
		let rawValue: string;
		if (column.render) {
			rawValue = column.render(value, row);
		} else {
			rawValue = String(value ?? '');
		}
		// SECURITY: Sanitize all HTML output to prevent XSS
		return sanitizeHtml(rawValue, 'standard');
	}

	function getRowId(row: Record<string, unknown>): string | number {
		return row[idKey] as string | number;
	}

	function isRowSelected(row: Record<string, unknown>): boolean {
		return selectedIds.includes(getRowId(row));
	}

	function checkMobile() {
		if (browser) {
			isMobile = window.innerWidth < mobileBreakpoint;
		}
	}

	// Debounced resize handler for performance
	const debouncedCheckMobile = debounce(checkMobile, 100);

	onMount(() => {
		checkMobile();
		window.addEventListener('resize', debouncedCheckMobile);
		return () => window.removeEventListener('resize', debouncedCheckMobile);
	});

	// Generate export columns (adapt 2-param render to 1-param format)
	let exportColumns = $derived(
		columns.map((col) => {
			const base = { key: col.key, label: col.label };
			if (col.render) {
				return { ...base, format: (value: unknown) => col.render!(value, {}) } as ExportColumn;
			}
			return base as ExportColumn;
		})
	);

	let visibleColumns = $derived(isMobile ? columns.filter((c) => !c.mobileHidden) : columns);
	let isAllSelected = $derived(data.length > 0 && selectedIds.length === data.length);
</script>

<div class="mobile-responsive-table">
	<!-- Header with export -->
	{#if exportable}
		<div class="mrt-export-bar">
			<ExportButton {data} columns={exportColumns} filename={exportFilename} />
		</div>
	{/if}

	{#if loading}
		<div class="mrt-loading-stack">
			{#each Array(5) as _}
				<SkeletonLoader variant="rectangular" width="100%" height="60px" />
			{/each}
		</div>
	{:else if data.length === 0}
		<div class="mrt-empty">
			<svg class="mrt-empty-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="1.5"
					d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
				/>
			</svg>
			<p>{emptyMessage}</p>
		</div>
	{:else if isMobile}
		<!-- Mobile Card View -->
		<div class="mrt-card-list">
			{#each data as row, index (getRowId(row))}
				<div
					class="mrt-card"
					onclick={() => handleRowClick(row)}
					onkeypress={(e: KeyboardEvent) => e.key === 'Enter' && handleRowClick(row)}
					onkeydown={(e: KeyboardEvent) =>
						e.key === ' ' && (e.preventDefault(), handleRowClick(row))}
					role="button"
					tabindex="0"
					aria-label={`Row ${index + 1}`}
				>
					{#if selectable}
						<div class="mrt-card-select">
							<input
								type="checkbox"
								checked={isRowSelected(row)}
								onclick={(e: MouseEvent) => {
									e.stopPropagation();
									toggleSelect(getRowId(row));
								}}
							/>
							<span class="mrt-card-index">#{index + 1}</span>
						</div>
					{/if}

					{#each visibleColumns as column}
						<div class="mrt-card-field">
							<span class="mrt-card-label">{column.label}</span>
							<span class="mrt-card-value" data-align={column.align ?? 'right'}>
								{@html getValue(row, column)}
							</span>
						</div>
					{/each}
				</div>
			{/each}
		</div>
	{:else}
		<!-- Desktop Table View -->
		<div class="mrt-table-wrap">
			<table class="mrt-table">
				<thead class="mrt-thead">
					<tr>
						{#if selectable}
							<th class="mrt-th mrt-th-checkbox">
								<input type="checkbox" checked={isAllSelected} onchange={toggleSelectAll} />
							</th>
						{/if}
						{#each visibleColumns as column}
							<th
								class="mrt-th"
								data-sortable={column.sortable || undefined}
								style={column.width ? `width: ${column.width}` : ''}
								onclick={() => handleSort(column)}
							>
								<div class="mrt-th-inner" data-align={column.align ?? 'left'}>
									<span>{column.label}</span>
									{#if column.sortable && sortBy === column.key}
										<svg
											class="mrt-sort-icon"
											class:mrt-sort-desc={sortDir === 'desc'}
											fill="none"
											stroke="currentColor"
											viewBox="0 0 24 24"
										>
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M5 15l7-7 7 7"
											/>
										</svg>
									{/if}
								</div>
							</th>
						{/each}
					</tr>
				</thead>
				<tbody class="mrt-tbody">
					{#each data as row (getRowId(row))}
						<tr
							class="mrt-row"
							data-selected={isRowSelected(row) || undefined}
							onclick={() => handleRowClick(row)}
						>
							{#if selectable}
								<td class="mrt-td">
									<input
										type="checkbox"
										checked={isRowSelected(row)}
										onclick={(e: MouseEvent) => {
											e.stopPropagation();
											toggleSelect(getRowId(row));
										}}
									/>
								</td>
							{/if}
							{#each visibleColumns as column}
								<td class="mrt-td" data-align={column.align ?? 'left'}>
									{@html getValue(row, column)}
								</td>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}

	<!-- Selected count -->
	{#if selectable && selectedIds.length > 0}
		<div class="mrt-selection-bar">
			<span class="mrt-selection-count">{selectedIds.length} selected</span>
			<button
				onclick={() => {
					selectedIds = [];
					onselect?.({ selectedIds: [] });
				}}
				class="mrt-selection-clear"
			>
				Clear
			</button>
		</div>
	{/if}
</div>

<style>
	.mrt-export-bar {
		display: flex;
		justify-content: flex-end;
		margin-block-end: var(--space-4);
	}

	.mrt-loading-stack {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	/* Empty state */
	.mrt-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: var(--space-12);
		color: oklch(0.55 0.01 250);

		& p {
			font-size: var(--text-sm);
		}
	}

	.mrt-empty-icon {
		inline-size: 4rem;
		block-size: 4rem;
		margin-block-end: var(--space-4);
		opacity: 0.5;
	}

	/* Mobile card list */
	.mrt-card-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.mrt-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		background-color: oklch(0.25 0.01 250 / 50%);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-4);
		transition: background-color var(--duration-fast) var(--ease-default);
		cursor: pointer;

		&:hover {
			background-color: oklch(0.25 0.01 250 / 70%);
		}
		&:active {
			transform: scale(0.98);
			background-color: oklch(0.25 0.01 250 / 80%);
		}
	}

	.mrt-card-select {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-block-end: var(--space-2);
		border-block-end: 1px solid oklch(0.38 0.01 250 / 50%);
	}

	.mrt-card-index {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
	}

	.mrt-card-field {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: var(--space-4);
	}

	.mrt-card-label {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
		font-weight: var(--weight-medium);
	}

	.mrt-card-value {
		font-size: var(--text-sm);
		color: oklch(1 0 0);
		font-weight: var(--weight-medium);
		flex: 1;

		&[data-align='right'] {
			text-align: end;
		}
		&[data-align='center'] {
			text-align: center;
		}
		&[data-align='left'] {
			text-align: start;
		}
	}

	/* Desktop table */
	.mrt-table-wrap {
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		overflow: hidden;
	}

	.mrt-table {
		inline-size: 100%;
		border-collapse: collapse;
	}

	.mrt-thead {
		background-color: oklch(0.25 0.01 250 / 50%);
	}

	.mrt-th {
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		text-align: start;
		font-size: var(--text-xs);
		font-weight: var(--weight-semibold);
		color: oklch(0.65 0.01 250);
		text-transform: uppercase;
		letter-spacing: 0.05em;

		&[data-sortable] {
			cursor: pointer;
			transition: color var(--duration-fast) var(--ease-default);
			&:hover {
				color: oklch(1 0 0);
			}
		}
	}

	.mrt-th-checkbox {
		inline-size: 3rem;
	}

	.mrt-th-inner {
		display: flex;
		align-items: center;
		gap: var(--space-2);

		&[data-align='center'] {
			justify-content: center;
		}
		&[data-align='right'] {
			justify-content: flex-end;
		}
	}

	.mrt-sort-icon {
		inline-size: 1rem;
		block-size: 1rem;
		transition: transform var(--duration-fast) var(--ease-default);
	}

	.mrt-sort-desc {
		transform: rotate(180deg);
	}

	.mrt-tbody {
		& .mrt-row + .mrt-row {
			border-block-start: 1px solid oklch(0.38 0.01 250 / 50%);
		}
	}

	.mrt-row {
		transition: background-color var(--duration-fast) var(--ease-default);
		cursor: pointer;

		&:hover {
			background-color: oklch(0.25 0.01 250 / 30%);
		}
		&[data-selected] {
			background-color: oklch(0.55 0.2 260 / 10%);
		}
	}

	.mrt-td {
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		font-size: var(--text-sm);
		color: oklch(0.7 0.01 250);
		max-inline-size: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;

		&[data-align='center'] {
			text-align: center;
		}
		&[data-align='right'] {
			text-align: end;
		}
	}

	/* Selection bar */
	.mrt-selection-bar {
		position: fixed;
		inset-block-end: var(--space-4);
		inset-inline-start: 50%;
		transform: translateX(-50%);
		padding-inline: var(--space-6);
		padding-block: var(--space-3);
		background-color: oklch(0.45 0.2 260);
		color: oklch(1 0 0);
		border-radius: 9999px;
		box-shadow: var(--shadow-xl);
		display: flex;
		align-items: center;
		gap: var(--space-4);
		z-index: 50;
	}

	.mrt-selection-count {
		font-weight: var(--weight-medium);
	}

	.mrt-selection-clear {
		font-size: var(--text-sm);
		text-decoration: underline;
		background: none;
		border: none;
		color: inherit;
		cursor: pointer;

		&:hover {
			text-decoration: none;
		}
	}

	/* Custom checkbox styling */
	input[type='checkbox'] {
		inline-size: 1rem;
		block-size: 1rem;
		appearance: none;
		background-color: oklch(0.3 0.01 250);
		border: 1px solid oklch(0.4 0.01 250);
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease-default);

		&:checked {
			background-color: oklch(0.55 0.2 260);
			border-color: oklch(0.55 0.2 260);
			background-image: url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3cpath d='M12.207 4.793a1 1 0 010 1.414l-5 5a1 1 0 01-1.414 0l-2-2a1 1 0 011.414-1.414L6.5 9.086l4.293-4.293a1 1 0 011.414 0z'/%3e%3c/svg%3e");
			background-size: 100% 100%;
			background-position: center;
			background-repeat: no-repeat;
		}

		&:focus {
			outline: none;
			box-shadow: 0 0 0 3px oklch(0.55 0.2 260 / 30%);
		}
	}
</style>
