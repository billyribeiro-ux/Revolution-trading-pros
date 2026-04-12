<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		variant?: 'default' | 'success' | 'warning' | 'danger' | 'info';
		size?: 'sm' | 'md';
		children?: Snippet;
	}

	let { variant = 'default', size = 'sm', children }: Props = $props();

	const variants = {
		default: 'bg-gray-100 text-gray-800',
		success: 'bg-green-100 text-green-800',
		warning: 'bg-yellow-100 text-yellow-800',
		danger: 'bg-red-100 text-red-800',
		info: 'bg-blue-100 text-blue-800'
	};

	const sizes = {
		sm: 'px-2 py-0.5 text-xs',
		md: 'px-3 py-1 text-sm'
	};
</script>

<span class="inline-flex items-center rounded-full font-medium {variants[variant]} {sizes[size]}">
	{@render children?.()}
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
