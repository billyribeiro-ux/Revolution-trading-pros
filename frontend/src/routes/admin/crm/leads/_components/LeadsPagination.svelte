<script lang="ts">
	interface Props {
		currentPage: number;
		totalPages: number;
		perPage: number;
		totalCount: number;
		onPageChange: (page: number) => void;
	}

	let { currentPage, totalPages, perPage, totalCount, onPageChange }: Props = $props();

	let startIndex = $derived((currentPage - 1) * perPage + 1);
	let endIndex = $derived(Math.min(currentPage * perPage, totalCount));
</script>

<div class="table-footer">
	<span class="results-count">
		Showing {startIndex} - {endIndex} of {totalCount} leads
	</span>
	<div class="pagination">
		<button
			class="pagination-btn"
			disabled={currentPage === 1}
			onclick={() => onPageChange(currentPage - 1)}
		>
			Previous
		</button>
		<span class="pagination-info">
			Page {currentPage} of {totalPages}
		</span>
		<button
			class="pagination-btn"
			disabled={currentPage === totalPages}
			onclick={() => onPageChange(currentPage + 1)}
		>
			Next
		</button>
	</div>
</div>

<style>
	.table-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
		border-top: 1px solid var(--border-default, #30363d);
	}

	.results-count {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
	}

	.pagination {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.pagination-btn {
		padding: 8px 16px;
		background: var(--bg-hover, #252b33);
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.pagination-btn:hover:not(:disabled) {
		background: var(--bg-active, #2d333b);
	}

	.pagination-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination-info {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
	}
</style>
