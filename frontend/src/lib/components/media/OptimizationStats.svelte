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
	import { Tween } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import Icon from '$lib/components/Icon.svelte';

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

	const animatedSavings = Tween.of(() => stats.savingsPercent, {
		duration: () => (animated ? 1000 : 0),
		easing: cubicOut
	});

	// Format bytes
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
	}

	function getSavingsTone(percent: number): 'excellent' | 'strong' | 'moderate' | 'low' {
		if (percent >= 70) return 'excellent';
		if (percent >= 50) return 'strong';
		if (percent >= 30) return 'moderate';
		return 'low';
	}

	let savingsTone = $derived(getSavingsTone(stats.savingsPercent));
</script>

{#if compact}
	<!-- Compact inline version -->
	<div class={['optimization-stats', 'optimization-stats--compact', className]} transition:fade>
		<span class="optimization-stats__original">{formatBytes(stats.originalSize)}</span>
		<span class="optimization-stats__icon">
			<Icon name="IconArrowRight" size={16} />
		</span>
		<span class="optimization-stats__optimized">{formatBytes(stats.optimizedSize)}</span>
		<span class="optimization-stats__badge" data-tone={savingsTone}>
			-{Math.round(animatedSavings.current)}%
		</span>
	</div>
{:else}
	<!-- Full card version -->
	<div class={['optimization-card', className]} transition:slide>
		<!-- Success header -->
		<div class="optimization-card__header">
			<div class="optimization-card__status-icon">
				<Icon name="IconCheck" size={24} />
			</div>
			<div>
				<h3>Optimized Successfully</h3>
				{#if stats.processingTime}
					<p>Processed in {stats.processingTime}ms</p>
				{/if}
			</div>
		</div>

		<!-- Main stats -->
		<div class="optimization-card__main">
			<!-- Circular progress -->
			<div class="optimization-ring">
				<svg aria-hidden="true" viewBox="0 0 100 100">
					<!-- Background ring -->
					<circle
						cx="50"
						cy="50"
						r="40"
						fill="none"
						stroke="currentColor"
						stroke-width="8"
						class="optimization-ring__track"
					/>
					<!-- Progress ring -->
					<circle
						cx="50"
						cy="50"
						r="40"
						fill="none"
						stroke-width="8"
						stroke-linecap="round"
						class="optimization-ring__progress"
						data-tone={savingsTone}
						stroke-dasharray="{animatedSavings.current * 2.51} 251"
						transform="rotate(-90 50 50)"
					/>
				</svg>
				<div class="optimization-ring__label">
					<span class="optimization-ring__value" data-tone={savingsTone}>
						{Math.round(animatedSavings.current)}%
					</span>
					<span class="optimization-ring__caption">smaller</span>
				</div>
			</div>

			<!-- Size comparison -->
			<div class="optimization-comparison">
				<div class="optimization-comparison__row">
					<span class="optimization-comparison__label">Original</span>
					<span class="optimization-comparison__original">{formatBytes(stats.originalSize)}</span>
					{#if stats.originalFormat}
						<span class="optimization-comparison__format">
							{stats.originalFormat.toUpperCase()}
						</span>
					{/if}
				</div>
				<div class="optimization-comparison__arrow">
					<Icon name="IconArrowDown" size={20} />
				</div>
				<div class="optimization-comparison__row">
					<span class="optimization-comparison__label">Optimized</span>
					<span class="optimization-comparison__optimized">{formatBytes(stats.optimizedSize)}</span>
					{#if stats.optimizedFormat}
						<span
							class="optimization-comparison__format optimization-comparison__format--optimized"
						>
							{stats.optimizedFormat.toUpperCase()}
						</span>
					{/if}
				</div>
			</div>
		</div>

		<!-- Additional details -->
		{#if showDetails}
			<div class="optimization-card__details">
				<div class="optimization-card__detail">
					<Icon name="IconShieldCheck" size={16} />
					<span>Lossless quality</span>
				</div>
				{#if stats.variantsCount && stats.variantsCount > 0}
					<div class="optimization-card__detail">
						<Icon name="IconLayoutGrid" size={16} />
						<span>{stats.variantsCount} responsive sizes</span>
					</div>
				{/if}
				<div class="optimization-card__detail">
					<Icon name="IconBolt" size={16} />
					<span>CDN-ready</span>
				</div>
			</div>
		{/if}
	</div>
{/if}

<style>
	.optimization-stats--compact {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #374151;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.optimization-stats__original {
		color: #9ca3af;
		text-decoration: line-through;
	}

	.optimization-stats__icon {
		display: inline-flex;
		color: #9ca3af;
	}

	.optimization-stats__optimized {
		color: #374151;
		font-weight: 500;
	}

	:global(.dark) .optimization-stats__optimized {
		color: #d1d5db;
	}

	.optimization-stats__badge {
		border-radius: 999px;
		background: #dcfce7;
		font-size: 0.75rem;
		font-weight: 600;
		line-height: 1rem;
		padding: 0.125rem 0.5rem;
	}

	:global(.dark) .optimization-stats__badge {
		background: rgba(20, 83, 45, 0.3);
	}

	.optimization-card {
		overflow: hidden;
		border: 1px solid #bbf7d0;
		border-radius: 0.75rem;
		background: #f0fdf4;
	}

	:global(.dark) .optimization-card {
		border-color: #166534;
		background: rgba(20, 83, 45, 0.2);
	}

	.optimization-card__header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		border-bottom: 1px solid #bbf7d0;
		background: rgba(220, 252, 231, 0.5);
		padding: 1rem;
	}

	:global(.dark) .optimization-card__header {
		border-color: #166534;
		background: rgba(20, 83, 45, 0.3);
	}

	.optimization-card__status-icon {
		display: flex;
		width: 2.5rem;
		height: 2.5rem;
		align-items: center;
		justify-content: center;
		border-radius: 999px;
		background: #22c55e;
		color: #ffffff;
		flex: 0 0 auto;
	}

	.optimization-card h3 {
		margin: 0;
		color: #166534;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.5;
	}

	.optimization-card p {
		margin: 0;
		color: #16a34a;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	:global(.dark) .optimization-card h3 {
		color: #bbf7d0;
	}

	:global(.dark) .optimization-card p {
		color: #4ade80;
	}

	.optimization-card__main {
		display: flex;
		align-items: center;
		justify-content: space-around;
		gap: 1.5rem;
		padding: 1.5rem;
	}

	.optimization-ring {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.optimization-ring svg {
		width: 6rem;
		height: 6rem;
	}

	.optimization-ring__track {
		color: #e5e7eb;
	}

	:global(.dark) .optimization-ring__track {
		color: #374151;
	}

	.optimization-ring__progress {
		transition: stroke-dasharray 1000ms ease;
	}

	.optimization-ring__label {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
	}

	.optimization-ring__value {
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 2rem;
	}

	.optimization-ring__caption {
		color: #6b7280;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.optimization-comparison {
		display: flex;
		align-items: center;
		flex-direction: column;
		gap: 0.5rem;
	}

	.optimization-comparison__row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.optimization-comparison__label {
		width: 4rem;
		color: #6b7280;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	:global(.dark) .optimization-comparison__label {
		color: #9ca3af;
	}

	.optimization-comparison__original {
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.25rem;
		text-decoration: line-through;
	}

	.optimization-comparison__optimized {
		color: #16a34a;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25rem;
	}

	:global(.dark) .optimization-comparison__optimized {
		color: #4ade80;
	}

	.optimization-comparison__format {
		border-radius: 0.25rem;
		background: #e5e7eb;
		color: #4b5563;
		font-size: 0.625rem;
		font-weight: 500;
		line-height: 1;
		padding: 0.125rem 0.375rem;
	}

	:global(.dark) .optimization-comparison__format {
		background: #374151;
		color: #9ca3af;
	}

	.optimization-comparison__format--optimized {
		background: #bbf7d0;
		color: #15803d;
	}

	:global(.dark) .optimization-comparison__format--optimized {
		background: #166534;
		color: #86efac;
	}

	.optimization-comparison__arrow {
		display: flex;
		padding: 0.25rem 0;
		color: #22c55e;
	}

	.optimization-card__details {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1.5rem;
		border-top: 1px solid #bbf7d0;
		color: #4b5563;
		font-size: 0.75rem;
		line-height: 1rem;
		padding: 0.75rem 1rem;
	}

	:global(.dark) .optimization-card__details {
		border-color: #166534;
		color: #9ca3af;
	}

	.optimization-card__detail {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.optimization-card__detail :global(svg) {
		color: #9ca3af;
	}

	.optimization-stats__badge[data-tone='excellent'],
	.optimization-ring__progress[data-tone='excellent'],
	.optimization-ring__value[data-tone='excellent'] {
		color: #22c55e;
		stroke: #22c55e;
	}

	.optimization-stats__badge[data-tone='strong'],
	.optimization-ring__progress[data-tone='strong'],
	.optimization-ring__value[data-tone='strong'] {
		color: #4ade80;
		stroke: #4ade80;
	}

	.optimization-stats__badge[data-tone='moderate'],
	.optimization-ring__progress[data-tone='moderate'],
	.optimization-ring__value[data-tone='moderate'] {
		color: #eab308;
		stroke: #eab308;
	}

	.optimization-stats__badge[data-tone='low'],
	.optimization-ring__progress[data-tone='low'],
	.optimization-ring__value[data-tone='low'] {
		color: #f97316;
		stroke: #f97316;
	}

	@media (max-width: 640px) {
		.optimization-card__main,
		.optimization-card__details {
			align-items: center;
			flex-direction: column;
		}
	}
</style>
