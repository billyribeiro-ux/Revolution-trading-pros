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

<tbody
	{@attach captureRef}
	data-slot="table-body"
	class={['ui-table-body', className]}
	{...restProps}
>
	{@render children?.()}
</tbody>

<style>
	.ui-table-body :global(tr:last-child) {
		border-bottom: 0;
	}
</style>
