<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
	import { CaretRight as ChevronRightIcon } from 'phosphor-svelte';

	type SubTriggerProps = DropdownMenuPrimitive.SubTriggerProps & {
		inset?: boolean;
	};

	let {
		ref = $bindable(null),
		class: className,
		inset,
		children,
		...restProps
	}: SubTriggerProps = $props();
</script>

<DropdownMenuPrimitive.SubTrigger
	bind:ref
	data-slot="dropdown-menu-sub-trigger"
	data-inset={inset}
	class={className}
	{...restProps}
>
	{@render children?.()}
	<ChevronRightIcon class="ms-auto size-4" />
</DropdownMenuPrimitive.SubTrigger>

<style>
	:global([data-slot='dropdown-menu-sub-trigger']) {
		display: flex;
		cursor: default;
		align-items: center;
		gap: var(--space-2);
		border-radius: var(--radius-sm);
		padding-inline: var(--space-2);
		padding-block: var(--space-1-5);
		font-size: var(--text-sm);
		outline: none;
		user-select: none;

		&[data-highlighted],
		&[data-state='open'] {
			background-color: var(--accent);
			color: var(--accent-foreground);
		}

		&[data-disabled] {
			pointer-events: none;
			opacity: 0.5;
		}

		&[data-inset] {
			padding-inline-start: var(--space-8);
		}

		& :global(svg) {
			pointer-events: none;
			flex-shrink: 0;
		}

		& :global(svg:not([class*='size-'])) {
			inline-size: 1rem;
			block-size: 1rem;
		}

		& :global(svg:not([class*='text-'])) {
			color: var(--muted-foreground);
		}
	}

	:global(.ddm-sub-chevron) {
		margin-inline-start: auto;
		inline-size: 1rem;
		block-size: 1rem;
	}
</style>
