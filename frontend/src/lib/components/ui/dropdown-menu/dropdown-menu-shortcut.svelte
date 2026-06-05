<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLSpanElement>> = $props();

	const captureRef: Attachment<HTMLSpanElement> = (node) => {
		ref = node;
		return () => {
			if (ref === node) {
				ref = null;
			}
		};
	};
</script>

<span
	{@attach captureRef}
	data-slot="dropdown-menu-shortcut"
	class={['ui-dropdown-menu-shortcut', className]}
	{...restProps}
>
	{@render children?.()}
</span>

<style>
	.ui-dropdown-menu-shortcut {
		margin-inline-start: auto;
		color: var(--dropdown-shortcut-color, #64748b);
		font-size: 0.75rem;
		line-height: 1rem;
		letter-spacing: 0.1em;
	}

	@media (prefers-color-scheme: dark) {
		.ui-dropdown-menu-shortcut {
			--dropdown-shortcut-color: #94a3b8;
		}
	}
</style>
