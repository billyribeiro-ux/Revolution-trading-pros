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
	import type { OptimizationStatistics } from '$lib/api/media';
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
		background: rgba(30, 41, 59, 0.4);
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
		background: rgba(15, 23, 42, 0.5);
		border-radius: 8px;
		border: 1px solid rgba(148, 163, 184, 0.1);
		position: relative;
		overflow: hidden;
	}

	.stat-card.highlight {
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.15), rgba(6, 95, 70, 0.15));
		border-color: rgba(16, 185, 129, 0.3);
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
		background: #dbeafe;
		color: #2563eb;
	}
	.stat-icon.green {
		background: #d1fae5;
		color: #059669;
	}
	.stat-icon.orange {
		background: #fef3c7;
		color: #d97706;
	}
	.stat-icon.gold {
		background: #fff8e0;
		color: var(--primary-600);
	}
	.stat-icon.emerald {
		background: #d1fae5;
		color: #10b981;
	}

	.stat-info {
		display: flex;
		flex-direction: column;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	.stat-progress {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: rgba(148, 163, 184, 0.1);
	}

	.progress-bar {
		height: 100%;
		background: linear-gradient(135deg, #10b981, #059669);
		transition: width 1s ease-out;
	}

	.stats-toggle {
		position: absolute;
		right: 1rem;
		top: 50%;
		transform: translateY(-50%);
		padding: 0.5rem;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 6px;
		cursor: pointer;
		color: #94a3b8;
	}

	.stats-toggle:hover {
		background: rgba(148, 163, 184, 0.2);
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
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		font-size: 0.8125rem;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.stats-show:hover {
		background: rgba(30, 41, 59, 0.6);
		border-color: rgba(99, 102, 241, 0.3);
	}

	.stats-show :global(svg) {
		width: 16px;
		height: 16px;
	}
</style>
