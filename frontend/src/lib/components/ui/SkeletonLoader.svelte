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

	let {
		variant = 'text',
		width = '100%',
		height = 'auto',
		lines = 1,
		animated = true,
		class: className = ''
	}: Props = $props();
</script>

{#if variant === 'text'}
	<div class="space-y-2 {className}" style="width: {width};">
		{#each Array(lines) as _, i}
			<div
				class="skeleton-text rounded {animated ? 'animate-shimmer' : ''}"
				style="width: {i === lines - 1 && lines > 1 ? '75%' : '100%'}; height: {height === 'auto' ? '1em' : height};"
			></div>
		{/each}
	</div>
{:else if variant === 'circular'}
	<div
		class="skeleton-circular rounded-full {animated ? 'animate-shimmer' : ''} {className}"
		style="width: {width}; height: {width};"
	></div>
{:else if variant === 'rectangular'}
	<div
		class="skeleton-rect rounded-lg {animated ? 'animate-shimmer' : ''} {className}"
		style="width: {width}; height: {height};"
	></div>
{:else if variant === 'stat'}
	<div class="skeleton-stat p-6 rounded-2xl border border-slate-700/50 bg-slate-800/50 {className}">
		<div class="flex items-start justify-between mb-4">
			<div class="skeleton-circular w-12 h-12 rounded-xl {animated ? 'animate-shimmer' : ''}"></div>
			<div class="skeleton-text w-16 h-5 rounded {animated ? 'animate-shimmer' : ''}"></div>
		</div>
		<div class="skeleton-text w-24 h-4 rounded mb-2 {animated ? 'animate-shimmer' : ''}"></div>
		<div class="skeleton-text w-32 h-8 rounded mb-3 {animated ? 'animate-shimmer' : ''}"></div>
		<div class="skeleton-text w-20 h-4 rounded {animated ? 'animate-shimmer' : ''}"></div>
	</div>
{:else if variant === 'card'}
	<div class="skeleton-card p-6 rounded-2xl border border-slate-700/50 bg-slate-800/50 {className}">
		<div class="skeleton-rect w-full h-40 rounded-xl mb-4 {animated ? 'animate-shimmer' : ''}"></div>
		<div class="skeleton-text w-3/4 h-6 rounded mb-2 {animated ? 'animate-shimmer' : ''}"></div>
		<div class="skeleton-text w-full h-4 rounded mb-1 {animated ? 'animate-shimmer' : ''}"></div>
		<div class="skeleton-text w-2/3 h-4 rounded {animated ? 'animate-shimmer' : ''}"></div>
	</div>
{/if}

<style>
	.skeleton-text,
	.skeleton-circular,
	.skeleton-rect {
		background: linear-gradient(
			90deg,
			rgba(51, 65, 85, 0.5) 0%,
			rgba(71, 85, 105, 0.5) 50%,
			rgba(51, 65, 85, 0.5) 100%
		);
		background-size: 200% 100%;
	}

	.animate-shimmer {
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

	/* Ensure smooth animations */
	.skeleton-text,
	.skeleton-circular,
	.skeleton-rect,
	.skeleton-stat,
	.skeleton-card {
		will-change: background-position;
	}
</style>
