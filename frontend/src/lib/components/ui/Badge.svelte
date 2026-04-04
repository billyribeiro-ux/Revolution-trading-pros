<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		variant?: 'default' | 'success' | 'warning' | 'danger' | 'info';
		size?: 'sm' | 'md';
		children?: Snippet;
	}

	let props: Props = $props();
	let variant = $derived(props.variant ?? 'default');
	let size = $derived(props.size ?? 'sm');
</script>

<span class="standalone-badge" data-variant={variant} data-size={size}>
	{@render props.children?.()}
</span>

<style>
	.standalone-badge {
		display: inline-flex;
		align-items: center;
		border-radius: 9999px;
		font-weight: var(--weight-medium);

		&[data-size='sm'] {
			padding-inline: var(--space-2);
			padding-block: 0.125rem;
			font-size: var(--text-xs);
		}

		&[data-size='md'] {
			padding-inline: var(--space-3);
			padding-block: var(--space-1);
			font-size: var(--text-sm);
		}

		&[data-variant='default'] {
			background-color: oklch(0.95 0.005 265);
			color: oklch(0.35 0.01 265);
		}

		&[data-variant='success'] {
			background-color: oklch(0.96 0.04 145);
			color: oklch(0.35 0.1 150);
		}

		&[data-variant='warning'] {
			background-color: oklch(0.97 0.06 90);
			color: oklch(0.4 0.1 80);
		}

		&[data-variant='danger'] {
			background-color: oklch(0.96 0.03 25);
			color: oklch(0.4 0.15 25);
		}

		&[data-variant='info'] {
			background-color: oklch(0.96 0.03 250);
			color: oklch(0.4 0.12 250);
		}
	}
</style>
