<!--
	Pagination.svelte - Reusable Pagination Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple Principal Engineer ICT 11+ Standards
	Svelte 5 January 2026 Syntax
	
	Single Responsibility: Previous/Next navigation with page numbers
	
	Features:
	- Page numbers with ellipsis for large page counts
	- Previous/Next buttons with arrow icons
	- "Showing X-Y of Z items" count
	- Keyboard accessible (Enter/Space)
	- ARIA labels for screen readers
	- Mobile responsive
	
	@version 1.0.0
	@since January 2026
-->
<script lang="ts">
	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 $props() pattern
	// ═══════════════════════════════════════════════════════════════════════════
	interface Props {
		currentPage: number;
		totalPages: number;
		totalItems: number;
		itemsPerPage: number;
		onPageChange: (page: number) => void;
		itemLabel?: string;
	}

	let props: Props = $props();

	// Derived props with defaults
	let currentPage = $derived(props.currentPage);
	let totalPages = $derived(props.totalPages);
	let totalItems = $derived(props.totalItems);
	let itemsPerPage = $derived(props.itemsPerPage);
	let onPageChange = $derived(props.onPageChange);
	let itemLabel = $derived(props.itemLabel ?? 'items');

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════
	const showingFrom = $derived(totalItems > 0 ? (currentPage - 1) * itemsPerPage + 1 : 0);
	const showingTo = $derived(Math.min(currentPage * itemsPerPage, totalItems));

	// ═══════════════════════════════════════════════════════════════════════════
	// PAGE NUMBER LOGIC
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Generate visible page numbers with ellipsis for pagination UI
	 */
	function getVisiblePages(current: number, total: number): (number | 'ellipsis')[] {
		if (total <= 7) return Array.from({ length: total }, (_, i) => i + 1);

		const pages: (number | 'ellipsis')[] = [];

		// Always show first page
		pages.push(1);

		// Calculate range around current
		const rangeStart = Math.max(2, current - 1);
		const rangeEnd = Math.min(total - 1, current + 1);

		// Add ellipsis before range if needed
		if (rangeStart > 2) pages.push('ellipsis');

		// Add range
		for (let i = rangeStart; i <= rangeEnd; i++) {
			pages.push(i);
		}

		// Add ellipsis after range if needed
		if (rangeEnd < total - 1) pages.push('ellipsis');

		// Always show last page
		if (total > 1) pages.push(total);

		return pages;
	}

	const visiblePages = $derived(getVisiblePages(currentPage, totalPages));

	// ═══════════════════════════════════════════════════════════════════════════
	// HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function goToPage(page: number) {
		if (page < 1 || page > totalPages || page === currentPage) return;
		onPageChange(page);
	}

	function handleKeyDown(event: KeyboardEvent, page: number) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			goToPage(page);
		}
	}
</script>

<nav class="pagination" aria-label="Pagination">
	<div class="pagination-controls">
		<!-- Previous Button -->
		<button
			class="pagination-btn pagination-nav"
			disabled={currentPage === 1}
			onclick={() => goToPage(currentPage - 1)}
			onkeydown={(e) => handleKeyDown(e, currentPage - 1)}
			aria-label="Go to previous page"
		>
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				width="18"
				height="18"
				aria-hidden="true"
			>
				<path d="M15 19l-7-7 7-7" />
			</svg>
			<span class="nav-text">Previous</span>
		</button>

		<!-- Page Numbers -->
		<div class="pagination-pages">
			{#each visiblePages as page, idx (page === 'ellipsis' ? `ellipsis-${idx}` : page)}
				{#if page === 'ellipsis'}
					<span class="pagination-ellipsis" aria-hidden="true">…</span>
				{:else}
					<button
						class="pagination-btn pagination-page"
						class:active={currentPage === page}
						onclick={() => goToPage(page)}
						onkeydown={(e) => handleKeyDown(e, page)}
						aria-label="Go to page {page}"
						aria-current={currentPage === page ? 'page' : undefined}
					>
						{page}
					</button>
				{/if}
			{/each}
		</div>

		<!-- Next Button -->
		<button
			class="pagination-btn pagination-nav"
			disabled={currentPage === totalPages}
			onclick={() => goToPage(currentPage + 1)}
			onkeydown={(e) => handleKeyDown(e, currentPage + 1)}
			aria-label="Go to next page"
		>
			<span class="nav-text">Next</span>
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				width="18"
				height="18"
				aria-hidden="true"
			>
				<path d="M9 5l7 7-7 7" />
			</svg>
		</button>
	</div>

	<!-- Results Count -->
	<p class="pagination-info" aria-live="polite">
		Showing {showingFrom}-{showingTo} of {totalItems}
		{itemLabel}
	</p>
</nav>

<style>
	.pagination {
		margin-top: 24px;
		padding: 16px 24px;
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		flex-wrap: wrap;
	}

	.pagination-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		min-width: 40px;
		min-height: 40px;
		padding: 8px 12px;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.pagination-btn:hover:not(:disabled) {
		background: #e5e7eb;
		color: #333;
	}

	.pagination-btn:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
	}

	.pagination-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.pagination-btn.active {
		background: #143e59;
		color: #fff;
		font-weight: 700;
	}

	.pagination-nav {
		padding: 8px 16px;
	}

	.pagination-pages {
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.pagination-ellipsis {
		padding: 8px 4px;
		color: #888;
		font-size: 14px;
	}

	.pagination-info {
		text-align: center;
		margin: 12px 0 0 0;
		font-size: 13px;
		color: #666;
	}

	/* Mobile responsive */
	@media (max-width: 640px) {
		.nav-text {
			display: none;
		}

		.pagination-nav {
			padding: 8px;
		}

		.pagination-btn {
			min-width: 36px;
			min-height: 36px;
			padding: 6px;
		}

		.pagination {
			padding: 12px 16px;
		}
	}

	/* Touch devices - 44pt minimum touch targets */
	@media (hover: none) and (pointer: coarse) {
		.pagination-btn {
			min-width: 44px;
			min-height: 44px;
		}
	}
</style>
