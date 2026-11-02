<script lang="ts">
	/**
	 * StatsPanel — collapsible media-library statistics row (5 stat cards + progress bar).
	 * Extracted from `admin/media/+page.svelte` (R10-C).
	 *
	 * Props:
	 *   - statistics: OptimizationStatistics  (data)
	 *   - showStatsPanel: boolean             ($bindable — caller toggles visibility)
	 *   - statsProgress: number               (tweened percent, 0-100)
	 *
	 * 1 bindable scalar, 0 callbacks.
	 */
	import { slide } from 'svelte/transition';
	import type { OptimizationStatistics } from '#lib/api/media.js';
	import IconPhoto from '@tabler/icons-svelte-runes/icons/photo';
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconBox from '@tabler/icons-svelte-runes/icons/box';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconChevronUp from '@tabler/icons-svelte-runes/icons/chevron-up';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';

	let {
		statistics,
		showStatsPanel = $bindable(true),
		statsProgress
	}: {
		statistics: OptimizationStatistics | null;
		showStatsPanel?: boolean;
		statsProgress: number;
	} = $props();

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(2) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
</script>

{#if showStatsPanel && statistics}
	<div class="stats-panel" transition:slide={{ duration: 300 }}>
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon blue">
					<IconPhoto size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{statistics.total_images}</span>
					<span class="stat-label">Total Images</span>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon green">
					<IconCircleCheck size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{statistics.optimized_images}</span>
					<span class="stat-label">Optimized</span>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon orange">
					<IconClock size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{statistics.pending_optimization}</span>
					<span class="stat-label">Pending</span>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon gold">
					<IconBox size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{formatBytes(statistics.total_storage)}</span>
					<span class="stat-label">Storage Used</span>
				</div>
			</div>

			<div class="stat-card highlight">
				<div class="stat-icon emerald">
					<IconBolt size={24} aria-hidden="true" />
				</div>
				<div class="stat-info">
					<span class="stat-value">{formatBytes(statistics.total_savings_bytes)}</span>
					<span class="stat-label">Total Savings</span>
				</div>
				<div class="stat-progress">
					<div class="progress-bar" style:width={`${statsProgress}%`}></div>
				</div>
			</div>
		</div>

		<button
			class="stats-toggle"
			onclick={() => (showStatsPanel = false)}
			aria-label="Hide statistics"
		>
			<IconChevronUp size={20} aria-hidden="true" />
		</button>
	</div>
{:else}
	<button class="stats-show" onclick={() => (showStatsPanel = true)}>
		<IconChartBar size={20} aria-hidden="true" />
		Show Statistics
	</button>
{/if}

<style>
	.stats-panel {
		padding: 1rem;
		background: var(--bg-elevated, #161b22);
		border-radius: 8px;
		margin-bottom: 1.5rem;
		position: relative;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
		gap: 1rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: var(--bg-base, #0d1117);
		border-radius: 8px;
		border: 1px solid var(--border-muted, #21262d);
		position: relative;
		overflow: hidden;
	}

	.stat-card.highlight {
		background: rgba(63, 185, 80, 0.15);
		border-color: rgba(63, 185, 80, 0.3);
	}

	.stat-icon {
		width: 40px;
		height: 40px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon :global(svg) {
		width: 20px;
		height: 20px;
	}

	.stat-icon.blue {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--info-emphasis, #58a6ff);
	}
	.stat-icon.green {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}
	.stat-icon.orange {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		color: var(--warning-emphasis, #d29922);
	}
	.stat-icon.gold {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		color: var(--primary-600);
	}
	.stat-icon.emerald {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	.stat-info {
		display: flex;
		flex-direction: column;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.stat-progress {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: var(--bg-surface, #1c2128);
	}

	.progress-bar {
		height: 100%;
		background: var(--success-base, #2ea043);
		transition: width 1s ease-out;
	}

	.stats-toggle {
		position: absolute;
		right: 1rem;
		top: 50%;
		transform: translateY(-50%);
		padding: 0.5rem;
		background: var(--bg-surface, #1c2128);
		border: none;
		border-radius: 6px;
		cursor: pointer;
		color: var(--text-secondary, #8b949e);
	}

	.stats-toggle:hover {
		background: var(--bg-hover, #252b33);
	}

	.stats-toggle :global(svg) {
		width: 16px;
		height: 16px;
	}

	.stats-show {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		padding: 0.5rem 1rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.stats-show:hover {
		background: var(--bg-elevated, #161b22);
		border-color: rgba(230, 184, 0, 0.3);
	}

	.stats-show :global(svg) {
		width: 16px;
		height: 16px;
	}
</style>
