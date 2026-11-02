<script lang="ts">
	/**
	 * Pagination — first/prev/next/last buttons + page indicator.
	 * Extracted from `admin/media/+page.svelte` (R10-C).
	 *
	 * Props:
	 *   - currentPage: number   ($bindable)
	 *   - totalPages: number
	 *   - onChange: (page: number) => void   (caller refetches data)
	 *
	 * 1 bindable scalar, 1 callback.
	 */
	import IconChevronsLeft from '@tabler/icons-svelte-runes/icons/chevrons-left';
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import IconChevronRight from '@tabler/icons-svelte-runes/icons/chevron-right';
	import IconChevronsRight from '@tabler/icons-svelte-runes/icons/chevrons-right';

	let {
		currentPage = $bindable(1),
		totalPages,
		onChange
	}: {
		currentPage?: number;
		totalPages: number;
		onChange: (page: number) => void;
	} = $props();

	function goTo(page: number) {
		currentPage = page;
		onChange(page);
	}
</script>

{#if totalPages > 1}
	<div class="pagination">
		<button
			class="btn-icon"
			disabled={currentPage === 1}
			onclick={() => goTo(1)}
			aria-label="First page"
		>
			<IconChevronsLeft size={20} aria-hidden="true" />
		</button>
		<button
			class="btn-icon"
			disabled={currentPage === 1}
			onclick={() => goTo(currentPage - 1)}
			aria-label="Previous page"
		>
			<IconChevronLeft size={20} aria-hidden="true" />
		</button>

		<span class="page-info">
			Page {currentPage} of {totalPages}
		</span>

		<button
			class="btn-icon"
			disabled={currentPage === totalPages}
			onclick={() => goTo(currentPage + 1)}
			aria-label="Next page"
		>
			<IconChevronRight size={20} aria-hidden="true" />
		</button>
		<button
			class="btn-icon"
			disabled={currentPage === totalPages}
			onclick={() => goTo(totalPages)}
			aria-label="Last page"
		>
			<IconChevronsRight size={20} aria-hidden="true" />
		</button>
	</div>
{/if}

<style>
	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1.5rem 0;
	}

	.page-info {
		padding: 0 1rem;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
		color: var(--text-secondary, #8b949e);
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400, #ffd11a);
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-icon :global(svg) {
		width: 18px;
		height: 18px;
	}
</style>
