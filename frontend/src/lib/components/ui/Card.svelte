<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		padding?: boolean;
		hover?: boolean;
		class?: string;
		children?: Snippet;
	}

	let { padding = true, hover = false, class: className = '', children }: Props = $props();
</script>

<div
	class="standalone-card {className}"
	data-padding={padding || undefined}
	data-hover={hover || undefined}
>
	{@render children?.()}
</div>

<style>
	.standalone-card {
		background-color: var(--card, oklch(1 0 0));
		border-radius: var(--radius-lg);
		border: 1px solid var(--border);
		box-shadow: var(--shadow-sm);

		&[data-padding] {
			padding: var(--space-6);
		}

		&[data-hover] {
			transition: box-shadow var(--duration-fast) var(--ease-default);

			&:hover {
				box-shadow: var(--shadow-md);
			}
		}
	}
</style>
