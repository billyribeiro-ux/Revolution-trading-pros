<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLElement>> = $props();

	const captureRef: Attachment<HTMLElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<caption
	{@attach captureRef}
	data-slot="table-caption"
	class={['ui-table-caption', className]}
	{...restProps}
>
	{@render children?.()}
</caption>

<style>
	.ui-table-caption {
		margin-top: 1rem;
		color: #64748b;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	@media (prefers-color-scheme: dark) {
		.ui-table-caption {
			color: #94a3b8;
		}
	}
</style>
