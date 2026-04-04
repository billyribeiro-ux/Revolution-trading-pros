<!--
  OptimizationStats Component
  ═══════════════════════════════════════════════════════════════════════════

  Display image optimization statistics with visual feedback:
  - Original vs optimized size comparison
  - Percentage savings
  - Format conversions
  - Processing time

  @version 2.0.0
-->
<script lang="ts">
	import { fade, slide } from 'svelte/transition';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';

	interface Stats {
		originalSize: number;
		optimizedSize: number;
		savingsPercent: number;
		originalFormat?: string;
		optimizedFormat?: string;
		processingTime?: number;
		variantsCount?: number;
	}

	// Props
	let {
		stats,
		showDetails = true,
		compact = false,
		animated = true,
		className = ''
	}: {
		stats: Stats;
		showDetails?: boolean;
		compact?: boolean;
		animated?: boolean;
		className?: string;
	} = $props();

	// Animated savings
	const animatedSavings = tweened(0, {
		duration: 1000,
		easing: cubicOut
	});

	$effect(() => {
		const duration = animated ? 1000 : 0;
		animatedSavings.set(stats.savingsPercent, { duration });
	});

	// Format bytes
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
	}

	// Get savings level for data attribute
	function getSavingsLevel(percent: number): string {
		if (percent >= 70) return 'excellent';
		if (percent >= 50) return 'good';
		if (percent >= 30) return 'fair';
		return 'low';
	}
</script>

{#if compact}
	<!-- Compact inline version -->
	<div class="opt-compact {className}" transition:fade>
		<span class="opt-original-size">{formatBytes(stats.originalSize)}</span>
		<svg class="opt-arrow-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
		</svg>
		<span class="opt-optimized-size">{formatBytes(stats.optimizedSize)}</span>
		<span class="opt-savings-badge" data-level={getSavingsLevel(stats.savingsPercent)}>
			-{Math.round($animatedSavings)}%
		</span>
	</div>
{:else}
	<!-- Full card version -->
	<div class="opt-card {className}" transition:slide>
		<!-- Success header -->
		<div class="opt-header">
			<div class="opt-check-icon">
				<svg class="opt-svg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
				</svg>
			</div>
			<div>
				<h3 class="opt-title">Optimized Successfully</h3>
				{#if stats.processingTime}
					<p class="opt-subtitle">Processed in {stats.processingTime}ms</p>
				{/if}
			</div>
		</div>

		<!-- Main stats -->
		<div class="opt-main">
			<!-- Circular progress -->
			<div class="opt-ring-wrapper">
				<svg viewBox="0 0 100 100" class="opt-ring-svg">
					<circle cx="50" cy="50" r="40" fill="none" stroke-width="8" class="ring-bg" />
					<circle
						cx="50" cy="50" r="40" fill="none" stroke-width="8" stroke-linecap="round"
						class="ring-progress"
						data-level={getSavingsLevel(stats.savingsPercent)}
						stroke-dasharray="{$animatedSavings * 2.51} 251"
						transform="rotate(-90 50 50)"
					/>
				</svg>
				<div class="opt-ring-label">
					<span class="opt-ring-pct" data-level={getSavingsLevel(stats.savingsPercent)}>
						{Math.round($animatedSavings)}%
					</span>
					<span class="opt-ring-sub">smaller</span>
				</div>
			</div>

			<!-- Size comparison -->
			<div class="opt-comparison">
				<div class="opt-size-row">
					<span class="opt-label">Original</span>
					<span class="opt-original-size">{formatBytes(stats.originalSize)}</span>
					{#if stats.originalFormat}
						<span class="opt-format-badge">{stats.originalFormat.toUpperCase()}</span>
					{/if}
				</div>
				<div class="opt-arrow-down">
					<svg class="opt-svg-sm opt-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
					</svg>
				</div>
				<div class="opt-size-row">
					<span class="opt-label">Optimized</span>
					<span class="opt-optimized-val">{formatBytes(stats.optimizedSize)}</span>
					{#if stats.optimizedFormat}
						<span class="opt-format-badge opt-format-green">{stats.optimizedFormat.toUpperCase()}</span>
					{/if}
				</div>
			</div>
		</div>

		<!-- Additional details -->
		{#if showDetails}
			<div class="opt-details">
				<div class="opt-detail-item">
					<svg class="opt-detail-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
					</svg>
					<span>Lossless quality</span>
				</div>
				{#if stats.variantsCount && stats.variantsCount > 0}
					<div class="opt-detail-item">
						<svg class="opt-detail-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z" />
						</svg>
						<span>{stats.variantsCount} responsive sizes</span>
					</div>
				{/if}
				<div class="opt-detail-item">
					<svg class="opt-detail-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
					</svg>
					<span>CDN-ready</span>
				</div>
			</div>
		{/if}
	</div>
{/if}

<style>
	/* ─── Compact ─── */
	.opt-compact {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-sm);
	}

	.opt-original-size {
		color: oklch(0.65 0.01 265);
		text-decoration: line-through;
	}

	.opt-arrow-icon {
		inline-size: 1rem;
		block-size: 1rem;
		color: oklch(0.65 0.01 265);
	}

	.opt-optimized-size {
		font-weight: var(--weight-medium);
		color: oklch(0.35 0.01 265);
	}

	.opt-savings-badge {
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
		border-radius: 9999px;
		font-size: var(--text-xs);
		font-weight: var(--weight-semibold);
		background-color: oklch(0.92 0.06 160);

		&[data-level='excellent'] { color: oklch(0.5 0.18 160); }
		&[data-level='good'] { color: oklch(0.55 0.16 160); }
		&[data-level='fair'] { color: oklch(0.6 0.18 90); }
		&[data-level='low'] { color: oklch(0.6 0.18 55); }
	}

	/* ─── Full card ─── */
	.opt-card {
		background-color: oklch(0.96 0.03 160);
		border: 1px solid oklch(0.88 0.06 160);
		border-radius: var(--radius-xl);
		overflow: hidden;
	}

	.opt-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-4);
		background-color: oklch(0.92 0.04 160 / 50%);
		border-block-end: 1px solid oklch(0.88 0.06 160);
	}

	.opt-check-icon {
		inline-size: 2.5rem;
		block-size: 2.5rem;
		border-radius: 9999px;
		background-color: oklch(0.6 0.18 160);
		color: oklch(1 0 0);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.opt-svg { inline-size: 1.5rem; block-size: 1.5rem; }
	.opt-svg-sm { inline-size: 1.25rem; block-size: 1.25rem; }

	.opt-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(0.3 0.1 160);
	}

	.opt-subtitle {
		font-size: var(--text-sm);
		color: oklch(0.45 0.1 160);
	}

	.opt-main {
		display: flex;
		align-items: center;
		justify-content: space-around;
		padding: var(--space-6);
		gap: var(--space-6);
	}

	.opt-ring-wrapper {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.opt-ring-svg { inline-size: 6rem; block-size: 6rem; }

	.ring-bg { stroke: oklch(0.9 0.005 265); }

	.ring-progress {
		transition: all 1000ms var(--ease-default);

		&[data-level='excellent'] { stroke: oklch(0.5 0.18 160); }
		&[data-level='good'] { stroke: oklch(0.55 0.16 160); }
		&[data-level='fair'] { stroke: oklch(0.6 0.18 90); }
		&[data-level='low'] { stroke: oklch(0.6 0.18 55); }
	}

	.opt-ring-label {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.opt-ring-pct {
		font-size: var(--text-2xl);
		font-weight: var(--weight-bold);

		&[data-level='excellent'] { color: oklch(0.5 0.18 160); }
		&[data-level='good'] { color: oklch(0.55 0.16 160); }
		&[data-level='fair'] { color: oklch(0.6 0.18 90); }
		&[data-level='low'] { color: oklch(0.6 0.18 55); }
	}

	.opt-ring-sub { font-size: var(--text-xs); color: oklch(0.55 0.01 265); }

	.opt-comparison {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-2);
	}

	.opt-size-row {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.opt-label {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
		inline-size: 4rem;
	}

	.opt-optimized-val {
		font-size: var(--text-sm);
		color: oklch(0.45 0.12 160);
		font-weight: var(--weight-semibold);
	}

	.opt-format-badge {
		padding-inline: 0.375rem;
		padding-block: 0.125rem;
		font-size: 0.625rem;
		font-weight: var(--weight-medium);
		border-radius: var(--radius-sm);
		background-color: oklch(0.9 0.005 265);
		color: oklch(0.45 0.01 265);

		&.opt-format-green {
			background-color: oklch(0.88 0.06 160);
			color: oklch(0.35 0.12 160);
		}
	}

	.opt-arrow-down { padding-block: var(--space-1); }
	.opt-green { color: oklch(0.6 0.18 160); }

	.opt-details {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-6);
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		border-block-start: 1px solid oklch(0.88 0.06 160);
		font-size: var(--text-xs);
		color: oklch(0.45 0.01 265);
	}

	.opt-detail-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.opt-detail-icon {
		inline-size: 1rem;
		block-size: 1rem;
		color: oklch(0.65 0.01 265);
	}
</style>
