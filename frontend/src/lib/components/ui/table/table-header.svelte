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

<thead
	{@attach captureRef}
	data-slot="table-header"
	class={['ui-table-header', className]}
	{...restProps}
>
	{@render children?.()}
</thead>

<style>
	.ui-table-header :global(tr) {
		border-bottom: 1px solid #e2e8f0;
	}

	@media (prefers-color-scheme: dark) {
		.ui-table-header :global(tr) {
			border-bottom-color: #1e293b;
		}
	}
</style>
