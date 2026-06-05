<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLParagraphElement>> = $props();

	const captureRef: Attachment<HTMLParagraphElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<p
	{@attach captureRef}
	data-slot="card-description"
	class={['ui-card-description', className]}
	{...restProps}
>
	{@render children?.()}
</p>

<style>
	.ui-card-description {
		color: #64748b;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	@media (prefers-color-scheme: dark) {
		.ui-card-description {
			color: #94a3b8;
		}
	}
</style>
