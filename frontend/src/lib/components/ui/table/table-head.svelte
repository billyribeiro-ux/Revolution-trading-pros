<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLThAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLThAttributes> = $props();

	const captureRef: Attachment<HTMLTableCellElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<th {@attach captureRef} data-slot="table-head" class={['ui-table-head', className]} {...restProps}>
	{@render children?.()}
</th>

<style>
	.ui-table-head {
		height: 2.5rem;
		padding-inline: 0.5rem;
		background-clip: padding-box;
		color: #0f172a;
		font-weight: 500;
		text-align: start;
		vertical-align: middle;
		white-space: nowrap;
	}

	@media (prefers-color-scheme: dark) {
		.ui-table-head {
			color: #f8fafc;
		}
	}
</style>
