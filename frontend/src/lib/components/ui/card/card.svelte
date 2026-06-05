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

<div {@attach captureRef} data-slot="card" class={['ui-card', className]} {...restProps}>
	{@render children?.()}
</div>

<style>
	.ui-card {
		--ui-card-bg: #ffffff;
		--ui-card-fg: #0f172a;
		--ui-card-border: #e2e8f0;

		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		border: 1px solid var(--ui-card-border);
		border-radius: 0.75rem;
		background: var(--ui-card-bg);
		color: var(--ui-card-fg);
		padding-block: 1.5rem;
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.05);
	}

	@media (prefers-color-scheme: dark) {
		.ui-card {
			--ui-card-bg: #020617;
			--ui-card-fg: #f8fafc;
			--ui-card-border: #1e293b;
		}
	}
</style>
