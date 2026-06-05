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
	data-slot="dialog-footer"
	class={['ui-dialog-footer', className]}
	{...restProps}
>
	{@render children?.()}
</div>

<style>
	.ui-dialog-footer {
		position: sticky;
		bottom: 0;
		z-index: 10;
		display: flex;
		flex-direction: column-reverse;
		gap: 0.5rem;
		border-top: 1px solid var(--dialog-footer-border, #e2e8f0);
		background: var(--dialog-footer-background, rgba(255, 255, 255, 0.95));
		padding: 0.75rem 1rem calc(1rem + env(safe-area-inset-bottom));
		backdrop-filter: blur(8px);
	}

	.ui-dialog-footer > :global(button) {
		min-height: 44px;
		touch-action: manipulation;
	}

	@media (min-width: 640px) {
		.ui-dialog-footer {
			position: static;
			flex-direction: row;
			justify-content: flex-end;
			border-top: 0;
			background: transparent;
			padding: 0;
			backdrop-filter: none;
		}
	}

	@media (prefers-color-scheme: dark) {
		.ui-dialog-footer {
			--dialog-footer-border: #334155;
			--dialog-footer-background: rgba(2, 6, 23, 0.95);
		}
	}
</style>
