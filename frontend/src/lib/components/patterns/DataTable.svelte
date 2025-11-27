<script lang="ts">
	/**
	 * DataTable - Enterprise data table with sorting, pagination, and actions
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import type { Snippet } from 'svelte';
	import {
		IconSortAscending,
		IconSortDescending,
		IconArrowLeft,
		IconArrowRight
	} from '@tabler/icons-svelte';
	import SkeletonLoader from '$lib/components/ui/SkeletonLoader.svelte';

	interface Column {
		key: string;
		label: string;
		sortable?: boolean;
		width?: string;
		align?: 'left' | 'center' | 'right';
	}

	interface Props {
		columns?: Column[];
		data?: any[];
		loading?: boolean;
		sortKey?: string;
		sortDirection?: 'asc' | 'desc';
		page?: number;
		pageSize?: number;
		totalItems?: number;
		showPagination?: boolean;
		emptyMessage?: string;
		stickyHeader?: boolean;
		onsort?: (detail: { key: string; direction: 'asc' | 'desc' }) => void;
		onpage?: (detail: { page: number }) => void;
		onrowclick?: (detail: { row: any; index: number }) => void;
		headerActions?: Snippet;
		empty?: Snippet;
		cell?: Snippet<[{ row: any; column: Column; value: any }]>;
		rowActions?: Snippet<[{ row: any; index: number }]>;
	}

	let {
		columns = [],
		data = [],
		loading = false,
		sortKey = $bindable(''),
		sortDirection = $bindable<'asc' | 'desc'>('asc'),
		page = $bindable(1),
		pageSize = 10,
		totalItems = 0,
		showPagination = true,
		emptyMessage = 'No data available',
		stickyHeader = false,
		onsort,
		onpage,
		onrowclick,
		headerActions,
		empty,
		cell,
		rowActions
	}: Props = $props();

	let totalPages = $derived(Math.ceil(totalItems / pageSize));
	let startItem = $derived((page - 1) * pageSize + 1);
	let endItem = $derived(Math.min(page * pageSize, totalItems));

	function handleSort(column: Column) {
		if (!column.sortable) return;
		
		if (sortKey === column.key) {
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			sortKey = column.key;
			sortDirection = 'asc';
		}
		
		onsort?.({ key: sortKey, direction: sortDirection });
	}

	function handlePageChange(newPage: number) {
		if (newPage < 1 || newPage > totalPages) return;
		page = newPage;
		onpage?.({ page });
	}

	function handleRowClick(row: any, index: number) {
		onrowclick?.({ row, index });
	}
</script>

<div class="data-table-container">
	<div class="table-wrapper" class:sticky-header={stickyHeader}>
		<table class="data-table">
			<thead>
				<tr>
					{#each columns as column}
						<th 
							style={column.width ? `width: ${column.width}` : ''}
							class:sortable={column.sortable}
							class:sorted={sortKey === column.key}
							class="align-{column.align || 'left'}"
							onclick={() => handleSort(column)}
						>
							<div class="th-content">
								<span>{column.label}</span>
								{#if column.sortable}
									<span class="sort-icon">
										{#if sortKey === column.key}
											{#if sortDirection === 'asc'}
												<IconSortAscending size={14} />
											{:else}
												<IconSortDescending size={14} />
											{/if}
										{:else}
											<IconSortAscending size={14} class="inactive" />
										{/if}
									</span>
								{/if}
							</div>
						</th>
					{/each}
					{@render headerActions?.()}
				</tr>
			</thead>
			<tbody>
				{#if loading}
					{#each Array(pageSize) as _, i}
						<tr>
							{#each columns as column}
								<td>
									<SkeletonLoader variant="text" width="80%" />
								</td>
							{/each}
						</tr>
					{/each}
				{:else if data.length === 0}
					<tr>
						<td colspan={columns.length} class="empty-cell">
							<div class="empty-state">
								{#if empty}
									{@render empty()}
								{:else}
									<p>{emptyMessage}</p>
								{/if}
							</div>
						</td>
					</tr>
				{:else}
					{#each data as row, index}
						<tr 
							onclick={() => handleRowClick(row, index)}
							class:clickable={rowActions}
						>
							{#each columns as column}
								<td class="align-{column.align || 'left'}">
									{#if cell}
										{@render cell({ row, column, value: row[column.key] })}
									{:else}
										{row[column.key] ?? '-'}
									{/if}
								</td>
							{/each}
							{@render rowActions?.({ row, index })}
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>

	{#if showPagination && totalItems > pageSize}
		<div class="pagination">
			<div class="pagination-info">
				Showing {startItem} to {endItem} of {totalItems.toLocaleString()} results
			</div>
			<div class="pagination-controls">
				<button 
					class="page-btn"
					disabled={page === 1}
					onclick={() => handlePageChange(page - 1)}
				>
					<IconArrowLeft size={18} />
				</button>
				
				{#each Array(Math.min(5, totalPages)) as _, i}
					{@const pageNum = page <= 3 ? i + 1 : page + i - 2}
					{#if pageNum > 0 && pageNum <= totalPages}
						<button 
							class="page-btn"
							class:active={pageNum === page}
							onclick={() => handlePageChange(pageNum)}
						>
							{pageNum}
						</button>
					{/if}
				{/each}
				
				<button 
					class="page-btn"
					disabled={page === totalPages}
					onclick={() => handlePageChange(page + 1)}
				>
					<IconArrowRight size={18} />
				</button>
			</div>
		</div>
	{/if}
</div>

<style>
	.data-table-container {
		background: var(--color-rtp-surface, rgba(15, 23, 42, 0.6));
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: var(--radius-xl, 1rem);
		overflow: hidden;
	}

	.table-wrapper {
		overflow-x: auto;
	}

	.table-wrapper.sticky-header {
		max-height: 600px;
		overflow-y: auto;
	}

	.table-wrapper.sticky-header thead {
		position: sticky;
		top: 0;
		z-index: 10;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table th {
		padding: 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--color-rtp-muted, #64748b);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: rgba(99, 102, 241, 0.05);
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
		white-space: nowrap;
	}

	.data-table th.sortable {
		cursor: pointer;
		user-select: none;
		transition: background 0.2s;
	}

	.data-table th.sortable:hover {
		background: rgba(99, 102, 241, 0.1);
	}

	.data-table th.sorted {
		color: var(--color-rtp-primary, #818cf8);
	}

	.th-content {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.sort-icon {
		display: flex;
		opacity: 0.5;
	}

	.sort-icon :global(.inactive) {
		opacity: 0.3;
	}

	.data-table th.sorted .sort-icon {
		opacity: 1;
	}

	.data-table td {
		padding: 1rem;
		font-size: 0.9rem;
		color: var(--color-rtp-text, #e2e8f0);
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
	}

	.data-table tbody tr {
		transition: background 0.2s;
	}

	.data-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.data-table tbody tr.clickable {
		cursor: pointer;
	}

	.align-left { text-align: left; }
	.align-center { text-align: center; }
	.align-right { text-align: right; }

	.empty-cell {
		text-align: center;
		padding: 3rem 1rem !important;
	}

	.empty-state {
		color: var(--color-rtp-muted, #64748b);
	}

	/* Pagination */
	.pagination {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		flex-wrap: wrap;
		gap: 1rem;
	}

	.pagination-info {
		font-size: 0.875rem;
		color: var(--color-rtp-muted, #64748b);
	}

	.pagination-controls {
		display: flex;
		gap: 0.25rem;
	}

	.page-btn {
		min-width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--color-rtp-muted, #94a3b8);
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.1);
		color: var(--color-rtp-text, #f1f5f9);
	}

	.page-btn.active {
		background: var(--color-rtp-primary, #6366f1);
		border-color: var(--color-rtp-primary, #6366f1);
		color: white;
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	@media (max-width: 640px) {
		.pagination {
			flex-direction: column;
			align-items: stretch;
		}

		.pagination-info {
			text-align: center;
		}

		.pagination-controls {
			justify-content: center;
		}
	}
</style>
