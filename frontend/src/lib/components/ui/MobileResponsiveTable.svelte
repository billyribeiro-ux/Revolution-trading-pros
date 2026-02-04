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
	let sortBy = $state(props.sortBy ?? '');
	let sortDir = $state<'asc' | 'desc'>(props.sortDir ?? 'asc');
	let selectable = $derived(props.selectable ?? false);
	let selectedIds = $state<(string | number)[]>(props.selectedIds ?? []);
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
	let tableRef: HTMLDivElement;

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

	// Setup resize listener for mobile detection
	$effect(() => {
		if (!browser) return;

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

<div class="mobile-responsive-table" bind:this={tableRef}>
	<!-- Header with export -->
	{#if exportable}
		<div class="flex justify-end mb-4">
			<ExportButton {data} columns={exportColumns} filename={exportFilename} />
		</div>
	{/if}

	{#if loading}
		<div class="space-y-3">
			{#each Array(5) as _}
				<SkeletonLoader variant="rectangular" width="100%" height="60px" />
			{/each}
		</div>
	{:else if data.length === 0}
		<div class="flex flex-col items-center justify-center py-12 text-slate-500">
			<svg class="w-16 h-16 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="1.5"
					d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
				/>
			</svg>
			<p class="text-sm">{emptyMessage}</p>
		</div>
	{:else if isMobile}
		<!-- Mobile Card View -->
		<div class="space-y-4">
			{#each data as row, index (getRowId(row))}
				<div
					class="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 space-y-3
						transition-all duration-150 hover:bg-slate-800/70 cursor-pointer
						active:scale-[0.98] active:bg-slate-800/80"
					onclick={() => handleRowClick(row)}
					onkeypress={(e: KeyboardEvent) => e.key === 'Enter' && handleRowClick(row)}
					onkeydown={(e: KeyboardEvent) =>
						e.key === ' ' && (e.preventDefault(), handleRowClick(row))}
					role="button"
					tabindex="0"
					aria-label={`Row ${index + 1}`}
				>
					{#if selectable}
						<div class="flex items-center gap-3 pb-2 border-b border-slate-700/50">
							<input
								type="checkbox"
								checked={isRowSelected(row)}
								onclick={(e: MouseEvent) => {
									e.stopPropagation();
									toggleSelect(getRowId(row));
								}}
								class="w-4 h-4 rounded border-slate-600 text-indigo-500 focus:ring-indigo-500"
							/>
							<span class="text-xs text-slate-500">#{index + 1}</span>
						</div>
					{/if}

					{#each visibleColumns as column}
						<div class="flex justify-between items-start gap-4">
							<span class="text-xs text-slate-500 font-medium">{column.label}</span>
							<span
								class="text-sm text-white font-medium text-right flex-1"
								class:text-left={column.align === 'left'}
								class:text-center={column.align === 'center'}
							>
								{@html getValue(row, column)}
							</span>
						</div>
					{/each}
				</div>
			{/each}
		</div>
	{:else}
		<!-- Desktop Table View -->
		<div class="table-mobile-scroll rounded-xl border border-slate-700/50 overflow-hidden">
			<table class="w-full">
				<thead class="bg-slate-800/50">
					<tr>
						{#if selectable}
							<th class="w-12 px-4 py-3">
								<input
									type="checkbox"
									checked={isAllSelected}
									onchange={toggleSelectAll}
									class="w-4 h-4 rounded border-slate-600 text-indigo-500 focus:ring-indigo-500"
								/>
							</th>
						{/if}
						{#each visibleColumns as column}
							<th
								class="px-4 py-3 text-left text-xs font-semibold text-slate-400 uppercase tracking-wider
									{column.sortable ? 'cursor-pointer hover:text-white transition-colors' : ''}"
								style={column.width ? `width: ${column.width}` : ''}
								onclick={() => handleSort(column)}
							>
								<div
									class="flex items-center gap-2"
									class:justify-center={column.align === 'center'}
									class:justify-end={column.align === 'right'}
								>
									<span>{column.label}</span>
									{#if column.sortable && sortBy === column.key}
										<svg
											class="w-4 h-4 transition-transform"
											class:rotate-180={sortDir === 'desc'}
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
				<tbody class="divide-y divide-slate-700/50">
					{#each data as row (getRowId(row))}
						<tr
							class="hover:bg-slate-800/30 transition-colors cursor-pointer {isRowSelected(row)
								? 'bg-indigo-500/10'
								: ''}"
							onclick={() => handleRowClick(row)}
						>
							{#if selectable}
								<td class="px-4 py-3">
									<input
										type="checkbox"
										checked={isRowSelected(row)}
										onclick={(e: MouseEvent) => {
											e.stopPropagation();
											toggleSelect(getRowId(row));
										}}
										class="w-4 h-4 rounded border-slate-600 text-indigo-500 focus:ring-indigo-500"
									/>
								</td>
							{/if}
							{#each visibleColumns as column}
								<td
									class="px-4 py-3 text-sm text-slate-300"
									class:text-center={column.align === 'center'}
									class:text-right={column.align === 'right'}
								>
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
		<div
			class="fixed bottom-4 left-1/2 -translate-x-1/2 px-6 py-3 bg-indigo-600 text-white
				rounded-full shadow-xl flex items-center gap-4 z-50"
		>
			<span class="font-medium">{selectedIds.length} selected</span>
			<button
				onclick={() => {
					selectedIds = [];
					onselect?.({ selectedIds: [] });
				}}
				class="text-sm underline hover:no-underline"
			>
				Clear
			</button>
		</div>
	{/if}
</div>

<style>
	/* Ensure table cells don't overflow */
	td {
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	/* Custom checkbox styling */
	input[type='checkbox'] {
		appearance: none;
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
