<script lang="ts">
	import { IconChevronLeft, IconChevronRight } from '$lib/icons';
	import type { PaginationInfo } from '$lib/api/members';

	interface Props {
		pagination: PaginationInfo;
		itemLabel?: string;
		onGoToPage: (page: number) => void;
	}

	let { pagination, itemLabel = 'members', onGoToPage }: Props = $props();

	let startIndex = $derived(
		pagination.total === 0 ? 0 : (pagination.current_page - 1) * pagination.per_page + 1
	);
	let endIndex = $derived(Math.min(pagination.current_page * pagination.per_page, pagination.total));
	let lastPage = $derived(Math.max(pagination.last_page, 1));
	let prevDisabled = $derived(pagination.current_page === 1);
	let nextDisabled = $derived(pagination.current_page >= lastPage || pagination.total === 0);
</script>

<div class="pagination">
	<div class="pagination-info">
		<!-- FIX-2026-04-26 (audit 02 §P2-7): off-by-one on empty list
			(used to show "Showing 1 to 0 of 0 members"). -->
		Showing {startIndex} to {endIndex} of {pagination.total} {itemLabel}
	</div>
	<div class="pagination-controls">
		<button
			class="page-btn"
			disabled={prevDisabled}
			onclick={() => onGoToPage(pagination.current_page - 1)}
		>
			<IconChevronLeft size={18} />
		</button>
		<span class="page-indicator">Page {pagination.current_page} of {lastPage}</span>
		<button
			class="page-btn"
			disabled={nextDisabled}
			onclick={() => onGoToPage(pagination.current_page + 1)}
		>
			<IconChevronRight size={18} />
		</button>
	</div>
</div>

<style>
	.pagination {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-top: 1px solid var(--border-muted);
	}

	.pagination-info {
		font-size: 0.875rem;
		color: var(--text-tertiary);
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.page-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid var(--border-default);
		border-radius: 12px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-indicator {
		font-size: 0.875rem;
		color: var(--text-secondary);
	}
</style>
