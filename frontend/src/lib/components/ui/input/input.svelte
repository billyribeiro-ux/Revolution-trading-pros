<script lang="ts">
	import type { HTMLInputAttributes, HTMLInputTypeAttribute } from 'svelte/elements';
	import type { WithElementRef } from '$lib/utils.js';

	type InputType = Exclude<HTMLInputTypeAttribute, 'file'>;

	type Props = WithElementRef<
		Omit<HTMLInputAttributes, 'type'> &
			({ type: 'file'; files?: FileList } | { type?: InputType; files?: undefined })
	>;

	let {
		ref = $bindable(null),
		value = $bindable(),
		type,
		files = $bindable(),
		class: className,
		'data-slot': dataSlot = 'input',
		...restProps
	}: Props = $props();
</script>

{#if type === 'file'}
	<input
		bind:this={ref}
		data-slot={dataSlot}
		class={className}
		type="file"
		bind:files
		bind:value
		{...restProps}
	/>
{:else}
	<input bind:this={ref} data-slot={dataSlot} class={className} {type} bind:value {...restProps} />
{/if}

<style>
	:global([data-slot='input']) {
		display: flex;
		min-block-size: 44px;
		inline-size: 100%;
		min-inline-size: 0;
		border-radius: var(--radius-md);
		border: 1px solid var(--input);
		background-color: var(--background);
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		font-size: var(--text-base);
		box-shadow: var(--shadow-xs);
		outline: none;
		touch-action: manipulation;
		transition:
			color var(--duration-fast) var(--ease-default),
			box-shadow var(--duration-fast) var(--ease-default);

		&::placeholder {
			color: var(--muted-foreground);
		}

		&::selection {
			background-color: var(--primary);
			color: var(--primary-foreground);
		}

		&:focus-visible {
			border-color: var(--ring);
			box-shadow: 0 0 0 3px oklch(from var(--ring) l c h / 50%);
		}

		&[aria-invalid='true'] {
			border-color: var(--destructive);
			box-shadow: 0 0 0 3px oklch(from var(--destructive) l c h / 20%);
		}

		&:disabled {
			cursor: not-allowed;
			opacity: 0.5;
		}
	}

	:global(.dark [data-slot='input']) {
		background-color: oklch(from var(--input) l c h / 30%);
	}

	:global(.dark [data-slot='input'][aria-invalid='true']) {
		box-shadow: 0 0 0 3px oklch(from var(--destructive) l c h / 40%);
	}

	:global(input[data-slot='input'][type='file']) {
		background-color: transparent;
		padding-block-start: var(--space-3);
		font-weight: var(--weight-medium);
	}
</style>
