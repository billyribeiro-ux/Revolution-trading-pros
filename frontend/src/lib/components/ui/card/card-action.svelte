<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();

	const captureRef: Attachment<HTMLDivElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<div
	{@attach captureRef}
	data-slot="card-action"
	class={['ui-card-action', className]}
	{...restProps}
>
	{@render children?.()}
</div>

<style>
	.ui-card-action {
		grid-column-start: 2;
		grid-row: 1 / span 2;
		align-self: start;
		justify-self: end;
	}
</style>
