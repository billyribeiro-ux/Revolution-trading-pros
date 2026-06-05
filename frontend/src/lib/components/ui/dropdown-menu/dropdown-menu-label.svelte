<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	type LabelProps = WithElementRef<HTMLAttributes<HTMLDivElement>> & {
		inset?: boolean;
	};

	let {
		ref = $bindable(null),
		class: className,
		inset,
		children,
		...restProps
	}: LabelProps = $props();

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
	data-slot="dropdown-menu-label"
	data-inset={inset}
	class={['ui-dropdown-menu-label', className]}
	{...restProps}
>
	{@render children?.()}
</div>

<style>
	.ui-dropdown-menu-label {
		padding: 0.375rem 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25rem;
	}

	.ui-dropdown-menu-label[data-inset] {
		padding-inline-start: 2rem;
	}
</style>
