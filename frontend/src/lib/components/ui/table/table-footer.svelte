<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLTableSectionElement>> = $props();

	const captureRef: Attachment<HTMLTableSectionElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<tfoot
	{@attach captureRef}
	data-slot="table-footer"
	class={['ui-table-footer', className]}
	{...restProps}
>
	{@render children?.()}
</tfoot>

<style>
	.ui-table-footer {
		border-top: 1px solid #e2e8f0;
		background: rgba(241, 245, 249, 0.5);
		font-weight: 500;
	}

	.ui-table-footer > :global(tr:last-child) {
		border-bottom: 0;
	}

	@media (prefers-color-scheme: dark) {
		.ui-table-footer {
			border-top-color: #1e293b;
			background: rgba(30, 41, 59, 0.45);
		}
	}
</style>
