<script lang="ts">
	/**
	 * Pagination - Shared Pagination Component
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for pagination used across dashboard pages.
	 * Supports both simple page numbers and FacetwP-style pagination.
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */

	interface Props {
		/** Current page (1-indexed) */
		currentPage: number;
		/** Total number of pages */
		totalPages: number;
		/** Page change handler */
		onPageChange?: (page: number) => void;
		/** Maximum number of page buttons to show */
		maxButtons?: number;
	}

	let {
		currentPage = 1,
		totalPages = 1,
		onPageChange,
		maxButtons = 7
	}: Props = $props();

	// Calculate visible page numbers
	let pages = $derived.by(() => {
		const result: (number | 'ellipsis')[] = [];

		if (totalPages <= maxButtons) {
			// Show all pages
			for (let i = 1; i <= totalPages; i++) {
				result.push(i);
			}
		} else {
			// Show first, last, and pages around current
			const half = Math.floor((maxButtons - 3) / 2);
			let start = Math.max(2, currentPage - half);
			let end = Math.min(totalPages - 1, currentPage + half);

			// Adjust if at edges
			if (currentPage <= half + 1) {
				end = maxButtons - 2;
			}
			if (currentPage >= totalPages - half) {
				start = totalPages - maxButtons + 3;
			}

			// Always show first page
			result.push(1);

			// Add ellipsis if needed
			if (start > 2) {
				result.push('ellipsis');
			}

			// Add middle pages
			for (let i = start; i <= end; i++) {
				result.push(i);
			}

			// Add ellipsis if needed
			if (end < totalPages - 1) {
				result.push('ellipsis');
			}

			// Always show last page
			if (totalPages > 1) {
				result.push(totalPages);
			}
		}

		return result;
	});

	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages && page !== currentPage) {
			onPageChange?.(page);
		}
	}
</script>

{#if totalPages > 1}
	<nav class="pagination" aria-label="Pagination">
		<!-- Previous Button -->
		<button
			type="button"
			class="pagination__btn pagination__btn--prev"
			disabled={currentPage <= 1}
			onclick={() => goToPage(currentPage - 1)}
			aria-label="Previous page"
		>
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<polyline points="15 18 9 12 15 6"></polyline>
			</svg>
		</button>

		<!-- Page Numbers -->
		<div class="pagination__pages">
			{#each pages as page, i}
				{#if page === 'ellipsis'}
					<span class="pagination__ellipsis" aria-hidden="true">…</span>
				{:else}
					<button
						type="button"
						class="pagination__page"
						class:is-active={page === currentPage}
						onclick={() => goToPage(page)}
						aria-label="Page {page}"
						aria-current={page === currentPage ? 'page' : undefined}
					>
						{page}
					</button>
				{/if}
			{/each}
		</div>

		<!-- Next Button -->
		<button
			type="button"
			class="pagination__btn pagination__btn--next"
			disabled={currentPage >= totalPages}
			onclick={() => goToPage(currentPage + 1)}
			aria-label="Next page"
		>
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<polyline points="9 18 15 12 9 6"></polyline>
			</svg>
		</button>
	</nav>
{/if}

<style>
	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 5px;
		padding: 20px 0;
	}

	.pagination__pages {
		display: flex;
		align-items: center;
		gap: 5px;
	}

	.pagination__btn,
	.pagination__page {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 36px;
		height: 36px;
		padding: 0 12px;
		font-size: 14px;
		font-weight: 500;
		font-family: 'Open Sans', sans-serif;
		color: #333;
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
	}

	.pagination__btn:hover:not(:disabled),
	.pagination__page:hover:not(.is-active) {
		background: #f4f4f4;
		border-color: #999;
	}

	.pagination__btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination__page.is-active {
		background: #0984ae;
		border-color: #0984ae;
		color: #fff;
	}

	.pagination__ellipsis {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		color: #666;
		font-size: 14px;
	}

	.pagination__btn svg {
		width: 16px;
		height: 16px;
	}

	@media (max-width: 576px) {
		.pagination__btn,
		.pagination__page {
			min-width: 32px;
			height: 32px;
			padding: 0 8px;
			font-size: 13px;
		}
	}
</style>
