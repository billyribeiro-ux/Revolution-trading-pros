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
	class={['ui-dropdown-menu-item', className]}
	{...restProps}
/>

<style>
	:global(.ui-dropdown-menu-item) {
		position: relative;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: 0.125rem;
		padding: 0.375rem 0.5rem;
		color: var(--dropdown-item-foreground, #0f172a);
		font-size: 0.875rem;
		line-height: 1.25rem;
		cursor: default;
		outline: none;
		user-select: none;
	}

	:global(.ui-dropdown-menu-item[data-inset]) {
		padding-inline-start: 2rem;
	}

	:global(.ui-dropdown-menu-item[data-highlighted]) {
		background: var(--dropdown-item-highlight-background, #f1f5f9);
		color: var(--dropdown-item-highlight-foreground, #0f172a);
	}

	:global(.ui-dropdown-menu-item[data-variant='destructive']) {
		color: var(--dropdown-item-destructive, #dc2626);
	}

	:global(.ui-dropdown-menu-item[data-variant='destructive'][data-highlighted]) {
		background: var(--dropdown-item-destructive-background, rgba(220, 38, 38, 0.1));
		color: var(--dropdown-item-destructive, #dc2626);
	}

	:global(.ui-dropdown-menu-item[data-disabled]) {
		pointer-events: none;
		opacity: 0.5;
	}

	:global(.ui-dropdown-menu-item svg) {
		pointer-events: none;
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
		color: var(--dropdown-item-icon, #64748b);
	}

	:global(.ui-dropdown-menu-item[data-variant='destructive'] svg) {
		color: var(--dropdown-item-destructive, #dc2626);
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-dropdown-menu-item) {
			--dropdown-item-foreground: #f8fafc;
			--dropdown-item-highlight-background: #1e293b;
			--dropdown-item-highlight-foreground: #f8fafc;
			--dropdown-item-icon: #94a3b8;
			--dropdown-item-destructive-background: rgba(220, 38, 38, 0.2);
		}
	}
</style>
