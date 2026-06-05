<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLTdAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLTdAttributes> = $props();

	const captureRef: Attachment<HTMLTableCellElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<td {@attach captureRef} data-slot="table-cell" class={['ui-table-cell', className]} {...restProps}>
	{@render children?.()}
</td>

<style>
	.ui-table-cell {
		padding: 0.5rem;
		background-clip: padding-box;
		vertical-align: middle;
		white-space: nowrap;
	}
</style>
