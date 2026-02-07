<script lang="ts" module>
	// Re-export types from the dedicated types file
	/* eslint-disable no-import-assign -- Svelte module context re-export */
	export {
		buttonVariants,
		type ButtonVariant,
		type ButtonSize,
		type ButtonProps
	} from './button.types';
	/* eslint-enable no-import-assign */
</script>

<script lang="ts">
	import { cn } from '$lib/utils.js';
	import { buttonVariants, type ButtonProps } from './button.types';
	let {
		class: className,
		variant = 'default',
		size = 'default',
		ref = $bindable(null),
		href = undefined,
		type = 'button',
		disabled,
		children,
		...restProps
	}: ButtonProps = $props();
</script>

{#if href}
	<a
		bind:this={ref}
		data-slot="button"
		class={cn(buttonVariants({ variant, size }), className)}
		href={disabled ? undefined : href}
		aria-disabled={disabled}
		role={disabled ? 'link' : undefined}
		tabindex={disabled ? -1 : undefined}
		{...restProps}
	>
		{@render children?.()}
	</a>
{:else}
	<button
		bind:this={ref}
		data-slot="button"
		class={cn(buttonVariants({ variant, size }), className)}
		{type}
		{disabled}
		{...restProps}
	>
		{@render children?.()}
	</button>
{/if}
