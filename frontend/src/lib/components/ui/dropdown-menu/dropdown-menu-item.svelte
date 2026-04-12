<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';

	type ItemProps = DropdownMenuPrimitive.ItemProps & {
		inset?: boolean;
		variant?: 'default' | 'destructive';
	};

	let {
		ref = $bindable(null),
		class: className,
		inset,
		variant = 'default',
		...restProps
	}: ItemProps = $props();
</script>

<DropdownMenuPrimitive.Item
	bind:ref
	data-slot="dropdown-menu-item"
	data-inset={inset}
	data-variant={variant}
	class={className}
	{...restProps}
/>

<style>
	:global([data-slot='dropdown-menu-item']) {
		position: relative;
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

		&[data-highlighted] {
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

	:global([data-slot='dropdown-menu-item'][data-variant='destructive']) {
		color: var(--destructive);

		&[data-highlighted] {
			background-color: oklch(from var(--destructive) l c h / 10%);
			color: var(--destructive);
		}

		& :global(svg) {
			color: var(--destructive) !important;
		}
	}

	:global(.dark [data-slot='dropdown-menu-item'][data-variant='destructive'][data-highlighted]) {
		background-color: oklch(from var(--destructive) l c h / 20%);
	}
</style>
