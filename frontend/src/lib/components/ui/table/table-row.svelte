<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLTableRowElement>> = $props();

	const captureRef: Attachment<HTMLTableRowElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<tr {@attach captureRef} data-slot="table-row" class={['ui-table-row', className]} {...restProps}>
	{@render children?.()}
</tr>

<style>
	.ui-table-row {
		border-bottom: 1px solid #e2e8f0;
		transition:
			background-color 0.18s ease,
			border-color 0.18s ease;
	}

	.ui-table-row:hover,
	.ui-table-row[data-state='selected'] {
		background: rgba(241, 245, 249, 0.7);
	}

	.ui-table-row:hover :global(th),
	.ui-table-row:hover :global(td) {
		background: rgba(241, 245, 249, 0.7);
	}

	@media (prefers-color-scheme: dark) {
		.ui-table-row {
			border-bottom-color: #1e293b;
		}

		.ui-table-row:hover,
		.ui-table-row[data-state='selected'],
		.ui-table-row:hover :global(th),
		.ui-table-row:hover :global(td) {
			background: rgba(30, 41, 59, 0.55);
		}
	}
</style>
