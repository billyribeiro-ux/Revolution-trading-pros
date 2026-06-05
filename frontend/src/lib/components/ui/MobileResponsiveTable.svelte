<script lang="ts">
	/* eslint svelte/no-at-html-tags: "off" -- every {@html} in this file renders sanitizer-cleaned HTML (sanitizeHtml/sanitizeBlogContent/etc.) or serialized JSON-LD; audited 2026-05-30 */
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
	import Icon from '$lib/components/Icon.svelte';
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

	let {
		columns = [],
		data = [],
		loading = false,
		sortBy = $bindable(''),
		sortDir = $bindable<'asc' | 'desc'>('asc'),
		selectable = false,
		selectedIds = $bindable<(string | number)[]>([]),
		idKey = 'id',
		exportable = false,
		exportFilename = 'export',
		emptyMessage = 'No data available',
		mobileBreakpoint = 768,
		onsort,
		onselect,
		onrowClick
	}: Props = $props();

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

	function getAlignClass(align: Column['align'] | undefined, fallback: Column['align']): string {
		return `align-${align ?? fallback}`;
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
			const render = col.render;
			if (render) {
				return { ...base, format: (value: unknown) => render(value, {}) } as ExportColumn;
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
		<div class="export-actions">
			<ExportButton {data} columns={exportColumns} filename={exportFilename} />
		</div>
	{/if}

	{#if loading}
		<div class="skeleton-stack">
			{#each Array(5) as _, i (i)}
				<SkeletonLoader variant="rectangular" width="100%" height="60px" />
			{/each}
		</div>
	{:else if data.length === 0}
		<div class="empty-state">
			<div class="empty-state__icon">
				<Icon name="IconFileText" size={64} />
			</div>
			<p class="empty-state__copy">{emptyMessage}</p>
		</div>
	{:else if isMobile}
		<!-- Mobile Card View -->
		<div class="mobile-card-list">
			{#each data as row, index (getRowId(row))}
				<div
					class="mobile-card"
					onclick={() => handleRowClick(row)}
					onkeypress={(e: KeyboardEvent) => e.key === 'Enter' && handleRowClick(row)}
					onkeydown={(e: KeyboardEvent) =>
						e.key === ' ' && (e.preventDefault(), handleRowClick(row))}
					role="button"
					tabindex="0"
					aria-label={`Row ${index + 1}`}
				>
					{#if selectable}
						<div class="mobile-card__select-row">
							<input
								type="checkbox"
								checked={isRowSelected(row)}
								onclick={(e: MouseEvent) => {
									e.stopPropagation();
									toggleSelect(getRowId(row));
								}}
								class="table-checkbox"
							/>
							<span class="mobile-card__index">#{index + 1}</span>
						</div>
					{/if}

					{#each visibleColumns as column (column.key)}
						<div class="mobile-field">
							<span class="mobile-field__label">{column.label}</span>
							<span class={['mobile-field__value', getAlignClass(column.align, 'right')]}>
								{@html getValue(row, column)}
							</span>
						</div>
					{/each}
				</div>
			{/each}
		</div>
	{:else}
		<!-- Desktop Table View -->
		<div class="table-shell">
			<table class="data-table">
				<thead class="data-table__head">
					<tr>
						{#if selectable}
							<th class="select-header-cell">
								<input
									type="checkbox"
									checked={isAllSelected}
									onchange={toggleSelectAll}
									class="table-checkbox"
								/>
							</th>
						{/if}
						{#each visibleColumns as column (column.key)}
							<th
								class={[
									'header-cell',
									column.sortable && 'header-cell--sortable',
									getAlignClass(column.align, 'left')
								]}
								style={column.width ? `width: ${column.width}` : ''}
								onclick={() => handleSort(column)}
							>
								<div class={['header-cell__content', getAlignClass(column.align, 'left')]}>
									<span>{column.label}</span>
									{#if column.sortable && sortBy === column.key}
										<span class={['sort-icon', sortDir === 'desc' && 'sort-icon--desc']}>
											<Icon name="IconChevronUp" size={16} />
										</span>
									{/if}
								</div>
							</th>
						{/each}
					</tr>
				</thead>
				<tbody class="data-table__body">
					{#each data as row (getRowId(row))}
						<tr
							class={['data-row', isRowSelected(row) && 'data-row--selected']}
							onclick={() => handleRowClick(row)}
						>
							{#if selectable}
								<td class="cell">
									<input
										type="checkbox"
										checked={isRowSelected(row)}
										onclick={(e: MouseEvent) => {
											e.stopPropagation();
											toggleSelect(getRowId(row));
										}}
										class="table-checkbox"
									/>
								</td>
							{/if}
							{#each visibleColumns as column (column.key)}
								<td class={['cell', getAlignClass(column.align, 'left')]}>
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
		<div class="selected-bar">
			<span class="selected-bar__count">{selectedIds.length} selected</span>
			<button
				onclick={() => {
					selectedIds = [];
					onselect?.({ selectedIds: [] });
				}}
				class="selected-bar__clear"
			>
				Clear
			</button>
		</div>
	{/if}
</div>

<style>
	.export-actions {
		display: flex;
		justify-content: flex-end;
		margin-bottom: 1rem;
	}

	.skeleton-stack {
		display: grid;
		gap: 0.75rem;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 3rem;
		color: #64748b;
	}

	.empty-state__icon {
		margin-bottom: 1rem;
		color: #64748b;
		opacity: 0.5;
	}

	.empty-state__copy {
		margin: 0;
		font-size: 0.875rem;
	}

	.mobile-card-list {
		display: grid;
		gap: 1rem;
	}

	.mobile-card {
		display: grid;
		gap: 0.75rem;
		padding: 1rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		background: rgba(30, 41, 59, 0.5);
		cursor: pointer;
		transition:
			background-color 0.15s ease,
			transform 0.15s ease;
	}

	.mobile-card:hover {
		background: rgba(30, 41, 59, 0.7);
	}

	.mobile-card:active {
		background: rgba(30, 41, 59, 0.8);
		transform: scale(0.98);
	}

	.mobile-card__select-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.mobile-card__index,
	.mobile-field__label {
		color: #64748b;
		font-size: 0.75rem;
	}

	.mobile-field {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
	}

	.mobile-field__label {
		font-weight: 500;
	}

	.mobile-field__value {
		flex: 1 1 auto;
		color: #fff;
		font-size: 0.875rem;
		font-weight: 500;
		text-align: right;
	}

	.table-shell {
		overflow: hidden;
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table__head {
		background: rgba(30, 41, 59, 0.5);
	}

	.data-table__body .data-row + .data-row {
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	.select-header-cell {
		width: 3rem;
		padding: 0.75rem 1rem;
	}

	.header-cell {
		padding: 0.75rem 1rem;
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 600;
		letter-spacing: 0;
		text-align: left;
		text-transform: uppercase;
	}

	.header-cell--sortable {
		cursor: pointer;
		transition: color 0.15s ease;
	}

	.header-cell--sortable:hover {
		color: #fff;
	}

	.header-cell__content {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.sort-icon {
		display: inline-flex;
		transition: transform 0.15s ease;
	}

	.sort-icon--desc {
		transform: rotate(180deg);
	}

	.data-row {
		cursor: pointer;
		transition: background-color 0.15s ease;
	}

	.data-row:hover {
		background: rgba(30, 41, 59, 0.3);
	}

	.data-row--selected {
		background: rgba(99, 102, 241, 0.1);
	}

	.cell {
		padding: 0.75rem 1rem;
		color: #cbd5e1;
		font-size: 0.875rem;
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.align-left {
		text-align: left;
	}

	.align-center {
		text-align: center;
	}

	.align-right {
		text-align: right;
	}

	.header-cell__content.align-center {
		justify-content: center;
	}

	.header-cell__content.align-right {
		justify-content: flex-end;
	}

	.selected-bar {
		position: fixed;
		z-index: 50;
		bottom: 1rem;
		left: 50%;
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1.5rem;
		border-radius: 999px;
		background: #4f46e5;
		color: #fff;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.4);
		transform: translateX(-50%);
	}

	.selected-bar__count {
		font-weight: 500;
	}

	.selected-bar__clear {
		border: 0;
		background: transparent;
		color: inherit;
		cursor: pointer;
		font: inherit;
		font-size: 0.875rem;
		text-decoration: underline;
	}

	.selected-bar__clear:hover {
		text-decoration: none;
	}

	/* Custom checkbox styling */
	.table-checkbox,
	input[type='checkbox'] {
		appearance: none;
		width: 1rem;
		height: 1rem;
		background-color: rgb(51 65 85);
		border: 1px solid rgb(71 85 105);
		border-radius: 0.25rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	input[type='checkbox']:checked {
		background-color: rgb(99 102 241);
		border-color: rgb(99 102 241);
		background-image: url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3cpath d='M12.207 4.793a1 1 0 010 1.414l-5 5a1 1 0 01-1.414 0l-2-2a1 1 0 011.414-1.414L6.5 9.086l4.293-4.293a1 1 0 011.414 0z'/%3e%3c/svg%3e");
		background-size: 100% 100%;
		background-position: center;
		background-repeat: no-repeat;
	}

	input[type='checkbox']:focus {
		outline: none;
		box-shadow: 0 0 0 3px rgb(99 102 241 / 0.3);
	}
</style>
