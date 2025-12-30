<script lang="ts">
	/**
	 * CardGrid - Responsive card grid layout
	 * Svelte 5 with $props() and snippet
	 */
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		columns?: 2 | 3 | 4;
	}

	let { children, columns = 3 }: Props = $props();
</script>

<div class="card-grid" class:card-grid--2col={columns === 2} class:card-grid--4col={columns === 4}>
	{@render children()}
</div>

<style>
	.card-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 30px;
	}

	/* 2 column max */
	.card-grid--2col {
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
	}

	/* 4 column on large screens */
	@media (min-width: 1400px) {
		.card-grid--4col {
			grid-template-columns: repeat(4, 1fr);
		}
	}
</style>
