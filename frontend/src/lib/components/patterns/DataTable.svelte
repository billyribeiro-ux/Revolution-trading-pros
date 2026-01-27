<script lang="ts">
	/**
	 * DataTable Component
	 * Enterprise data table with sorting, filtering, and pagination
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	interface Column {
		key: string;
		label: string;
		sortable?: boolean;
		width?: string;
	}

	interface Props {
		columns: Column[];
		data: Record<string, unknown>[];
		loading?: boolean;
		emptyMessage?: string;
		onRowClick?: (row: Record<string, unknown>) => void;
	}

	const { columns, data, loading = false, emptyMessage = 'No data available', onRowClick }: Props = $props();

	let sortKey = $state<string | null>(null);
	let sortDirection = $state<'asc' | 'desc'>('asc');

	const sortedData = $derived.by(() => {
		if (!sortKey) return data;
		return [...data].sort((a, b) => {
			const aVal = a[sortKey!];
			const bVal = b[sortKey!];
			if (aVal === bVal) return 0;
			const comparison = aVal! < bVal! ? -1 : 1;
			return sortDirection === 'asc' ? comparison : -comparison;
		});
	});

	function handleSort(key: string) {
		if (sortKey === key) {
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			sortKey = key;
			sortDirection = 'asc';
		}
	}

	function handleRowClick(row: Record<string, unknown>) {
		onRowClick?.(row);
	}
</script>

<div class="data-table-container">
	{#if loading}
		<div class="loading-overlay">
			<div class="loading-spinner"></div>
		</div>
	{/if}

	<table class="data-table">
		<thead>
			<tr>
				{#each columns as column}
					<th
						style:width={column.width}
						class:sortable={column.sortable}
						onclick={() => column.sortable && handleSort(column.key)}
						onkeydown={(e) => e.key === 'Enter' && column.sortable && handleSort(column.key)}
						tabindex={column.sortable ? 0 : -1}
						role={column.sortable ? 'button' : undefined}
					>
						{column.label}
						{#if column.sortable && sortKey === column.key}
							<span class="sort-indicator">{sortDirection === 'asc' ? '↑' : '↓'}</span>
						{/if}
					</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#if sortedData.length === 0}
				<tr>
					<td colspan={columns.length} class="empty-message">{emptyMessage}</td>
				</tr>
			{:else}
				{#each sortedData as row, i}
					<tr
						class:clickable={!!onRowClick}
						onclick={() => handleRowClick(row)}
						onkeydown={(e) => e.key === 'Enter' && handleRowClick(row)}
						tabindex={onRowClick ? 0 : -1}
						role={onRowClick ? 'button' : undefined}
					>
						{#each columns as column}
							<td>{row[column.key] ?? ''}</td>
						{/each}
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>

<style>
	.data-table-container {
		position: relative;
		width: 100%;
		overflow-x: auto;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	th, td {
		padding: 0.75rem 1rem;
		text-align: left;
		border-bottom: 1px solid var(--color-border-default, #e5e7eb);
	}

	th {
		font-weight: 600;
		background: var(--color-bg-secondary, #f9fafb);
		color: var(--color-text-secondary, #6b7280);
		text-transform: uppercase;
		font-size: 0.75rem;
		letter-spacing: 0.05em;
	}

	th.sortable {
		cursor: pointer;
		user-select: none;
	}

	th.sortable:hover {
		background: var(--color-bg-tertiary, #f3f4f6);
	}

	.sort-indicator {
		margin-left: 0.25rem;
	}

	tr.clickable {
		cursor: pointer;
	}

	tr.clickable:hover td {
		background: var(--color-bg-hover, #f9fafb);
	}

	.empty-message {
		text-align: center;
		color: var(--color-text-muted, #9ca3af);
		padding: 2rem;
	}

	.loading-overlay {
		position: absolute;
		inset: 0;
		background: rgba(255, 255, 255, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.loading-spinner {
		width: 24px;
		height: 24px;
		border: 2px solid var(--color-border-default, #e5e7eb);
		border-top-color: var(--color-primary, #6366f1);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Mobile-first responsive */
	@media (min-width: 640px) {
		th, td {
			padding: 1rem 1.25rem;
		}
	}
</style>
