<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLTableAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLTableAttributes> = $props();

	const captureRef: Attachment<HTMLTableElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<div data-slot="table-container" class="ui-table-container">
	<table {@attach captureRef} data-slot="table" class={['ui-table', className]} {...restProps}>
		{@render children?.()}
	</table>
</div>

<style>
	.ui-table-container {
		position: relative;
		width: 100%;
		overflow-x: auto;
	}

	.ui-table {
		width: 100%;
		border-collapse: collapse;
		caption-side: bottom;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}
</style>
