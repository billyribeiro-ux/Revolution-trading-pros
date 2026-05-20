<script lang="ts">
	interface Props {
		statusFilter: string;
		onchange: (status: string) => void;
		onclear: () => void;
	}

	let { statusFilter = $bindable(''), onchange, onclear }: Props = $props();
</script>

<div class="filters-panel">
	<div class="filter-group">
		<label for="status-filter">Status</label>
		<select
			id="status-filter"
			bind:value={statusFilter}
			onchange={() => onchange(statusFilter)}
		>
			<option value="">All Statuses</option>
			<option value="pending">Pending</option>
			<option value="completed">Completed</option>
			<option value="refunded">Refunded</option>
			<option value="partial_refund">Partial Refund</option>
			<option value="failed">Failed</option>
		</select>
	</div>

	<button class="clear-filters" onclick={onclear}> Clear All </button>
</div>

<style>
	.filters-panel {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		background: rgba(22, 27, 34, 0.85);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		margin-bottom: 1rem;
		backdrop-filter: blur(10px);
	}

	.filter-group {
		flex: 1;
	}

	.filter-group label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.5rem;
	}

	.filter-group select {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(22, 27, 34, 0.95);
		border: 1px solid var(--border-default);
		border-radius: 12px;
		color: var(--text-primary);
		font-size: 0.875rem;
		cursor: pointer;
	}

	.clear-filters {
		padding: 0.625rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		color: var(--text-primary);
		font-weight: 500;
		cursor: pointer;
		align-self: flex-end;
	}
</style>
