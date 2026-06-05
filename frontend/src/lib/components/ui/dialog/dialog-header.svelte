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
	data-slot="dialog-header"
	class={['ui-dialog-header', className]}
	{...restProps}
>
	{@render children?.()}
</div>

<style>
	.ui-dialog-header {
		position: sticky;
		top: 0;
		z-index: 10;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		border-bottom: 1px solid var(--dialog-header-border, #e2e8f0);
		background: var(--dialog-header-background, rgba(255, 255, 255, 0.95));
		padding: calc(0.5rem + env(safe-area-inset-top)) 1rem 0.75rem;
		text-align: center;
		backdrop-filter: blur(8px);
	}

	@media (min-width: 640px) {
		.ui-dialog-header {
			position: static;
			border-bottom: 0;
			background: transparent;
			padding: 0;
			text-align: start;
			backdrop-filter: none;
		}
	}

	@media (prefers-color-scheme: dark) {
		.ui-dialog-header {
			--dialog-header-border: #334155;
			--dialog-header-background: rgba(2, 6, 23, 0.95);
		}
	}
</style>
