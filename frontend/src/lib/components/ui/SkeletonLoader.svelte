<script lang="ts">
	/**
	 * SkeletonLoader - Enterprise Loading State Component
	 * Provides consistent loading skeleton animations
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */

	interface Props {
		variant?: 'text' | 'circular' | 'rectangular' | 'card' | 'stat';
		width?: string;
		height?: string;
		lines?: number;
		animated?: boolean;
		class?: string;
	}

	let props: Props = $props();
	let variant = $derived(props.variant ?? 'text');
	let width = $derived(props.width ?? '100%');
	let height = $derived(props.height ?? 'auto');
	let lines = $derived(props.lines ?? 1);
	let animated = $derived(props.animated ?? true);
	let className = $derived(props.class ?? '');
</script>

{#if variant === 'text'}
	<div class="skeleton-lines {className}" style:inline-size={width}>
		{#each Array(lines) as _, i}
			<div
				class="skeleton-bone"
				data-animated={animated || undefined}
				style:inline-size={i === lines - 1 && lines > 1 ? '75%' : '100%'}
				style:block-size={height === 'auto' ? '1em' : height}
			></div>
		{/each}
	</div>
{:else if variant === 'circular'}
	<div
		class="skeleton-bone skeleton-circular {className}"
		data-animated={animated || undefined}
		style:inline-size={width}
		style:block-size={width}
	></div>
{:else if variant === 'rectangular'}
	<div
		class="skeleton-bone skeleton-rect {className}"
		data-animated={animated || undefined}
		style:inline-size={width}
		style:block-size={height}
	></div>
{:else if variant === 'stat'}
	<div class="skeleton-compound skeleton-compound-stat {className}">
		<div class="skeleton-stat-header">
			<div class="skeleton-bone skeleton-stat-icon" data-animated={animated || undefined}></div>
			<div class="skeleton-bone skeleton-stat-badge" data-animated={animated || undefined}></div>
		</div>
		<div class="skeleton-bone skeleton-stat-label" data-animated={animated || undefined}></div>
		<div class="skeleton-bone skeleton-stat-value" data-animated={animated || undefined}></div>
		<div class="skeleton-bone skeleton-stat-trend" data-animated={animated || undefined}></div>
	</div>
{:else if variant === 'card'}
	<div class="skeleton-compound skeleton-compound-card {className}">
		<div class="skeleton-bone skeleton-card-image" data-animated={animated || undefined}></div>
		<div class="skeleton-bone skeleton-card-title" data-animated={animated || undefined}></div>
		<div class="skeleton-bone skeleton-card-line" data-animated={animated || undefined}></div>
		<div class="skeleton-bone skeleton-card-line-short" data-animated={animated || undefined}></div>
	</div>
{/if}

<style>
	.skeleton-lines {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.skeleton-bone {
		background: linear-gradient(
			90deg,
			oklch(0.3 0.01 250 / 50%) 0%,
			oklch(0.38 0.01 250 / 50%) 50%,
			oklch(0.3 0.01 250 / 50%) 100%
		);
		background-size: 200% 100%;
		border-radius: var(--radius-sm);
		will-change: background-position;

		&[data-animated] {
			animation: shimmer 1.5s ease-in-out infinite;
		}
	}

	.skeleton-circular {
		border-radius: 9999px;
	}

	.skeleton-rect {
		border-radius: var(--radius-lg);
	}

	.skeleton-compound {
		padding: var(--space-6);
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		background-color: oklch(0.25 0.01 250 / 50%);
	}

	/* Stat variant */
	.skeleton-stat-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		margin-block-end: var(--space-4);
	}

	.skeleton-stat-icon {
		inline-size: 3rem;
		block-size: 3rem;
		border-radius: var(--radius-lg);
	}

	.skeleton-stat-badge {
		inline-size: 4rem;
		block-size: 1.25rem;
	}

	.skeleton-stat-label {
		inline-size: 6rem;
		block-size: 1rem;
		margin-block-end: var(--space-2);
	}

	.skeleton-stat-value {
		inline-size: 8rem;
		block-size: 2rem;
		margin-block-end: var(--space-3);
	}

	.skeleton-stat-trend {
		inline-size: 5rem;
		block-size: 1rem;
	}

	/* Card variant */
	.skeleton-card-image {
		inline-size: 100%;
		block-size: 10rem;
		border-radius: var(--radius-lg);
		margin-block-end: var(--space-4);
	}

	.skeleton-card-title {
		inline-size: 75%;
		block-size: 1.5rem;
		margin-block-end: var(--space-2);
	}

	.skeleton-card-line {
		inline-size: 100%;
		block-size: 1rem;
		margin-block-end: var(--space-1);
	}

	.skeleton-card-line-short {
		inline-size: 66%;
		block-size: 1rem;
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
