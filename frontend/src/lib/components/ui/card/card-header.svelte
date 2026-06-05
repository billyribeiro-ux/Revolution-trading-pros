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
	data-slot="card-header"
	class={['ui-card-header', className]}
	{...restProps}
>
	{@render children?.()}
</div>

<style>
	.ui-card-header {
		container: card-header / inline-size;
		display: grid;
		grid-auto-rows: min-content;
		grid-template-columns: 1fr auto;
		grid-template-rows: auto auto;
		align-items: start;
		gap: 0.375rem;
		padding-inline: 1.5rem;
	}
</style>
