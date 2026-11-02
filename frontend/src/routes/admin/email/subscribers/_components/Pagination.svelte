<script lang="ts">
	interface Props {
		page: number;
		totalPages: number;
		total: number;
		onpageChange: (newPage: number) => void;
	}

	let { page, totalPages, total, onpageChange }: Props = $props();
</script>

<div class="pagination">
	<button class="btn-secondary small" disabled={page === 1} onclick={() => onpageChange(page - 1)}>
		Previous
	</button>
	<span class="page-info">
		Page {page} of {totalPages} ({total.toLocaleString()} total)
	</span>
	<button
		class="btn-secondary small"
		disabled={page === totalPages}
		onclick={() => onpageChange(page + 1)}
	>
		Next
	</button>
</div>

<style>
	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 1rem;
		border-top: 1px solid var(--border-muted, #21262d);
	}

	.page-info {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		background: var(--bg-surface, #1c2128);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
		font-size: 0.875rem;
	}

	.btn-secondary.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
