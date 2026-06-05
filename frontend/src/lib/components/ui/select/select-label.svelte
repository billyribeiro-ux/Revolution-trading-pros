<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> & {} = $props();

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
	data-slot="select-label"
	class={['ui-select-label', className]}
	{...restProps}
>
	{@render children?.()}
</div>

<style>
	.ui-select-label {
		padding: 0.375rem 0.5rem;
		color: var(--select-label-color, #64748b);
		font-size: 0.75rem;
		line-height: 1rem;
	}

	@media (prefers-color-scheme: dark) {
		.ui-select-label {
			--select-label-color: #94a3b8;
		}
	}
</style>
