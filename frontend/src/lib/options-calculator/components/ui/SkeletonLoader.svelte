<script lang="ts">
	interface Props {
		width?: string;
		height?: string;
		borderRadius?: string;
		count?: number;
		gap?: string;
	}

	let {
		width = '100%',
		height = '16px',
		borderRadius = '8px',
		count = 1,
		gap = '8px',
	}: Props = $props();
</script>

<div class="flex flex-col" style="gap: {gap};" aria-busy="true" aria-label="Loading">
	{#each Array(count) as _, i (i)}
		<div
			class="skeleton-shimmer"
			style="width: {width}; height: {height}; border-radius: {borderRadius};"
		></div>
	{/each}
</div>

<style>
	.skeleton-shimmer {
		background: linear-gradient(
			90deg,
			var(--calc-surface-hover, rgba(255, 255, 255, 0.04)) 25%,
			var(--calc-border, rgba(255, 255, 255, 0.08)) 50%,
			var(--calc-surface-hover, rgba(255, 255, 255, 0.04)) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s ease-in-out infinite;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}
</style>
