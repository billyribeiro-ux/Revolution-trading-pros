<script lang="ts">
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	interface Props {
		data: {
			sources: Array<{
				name: string;
				visitors: number;
				percentage: number;
				change: number;
				color: string;
			}>;
			total_visitors: number;
			period: string;
		};
		config?: Record<string, unknown>;
	}

	let { data, config: _config = {} }: Props = $props();

	const totalVisitors = tweened(0, { duration: 1500, easing: cubicOut });

	onMount(() => {
		totalVisitors.set(data?.total_visitors || 0);
	});

	let sortedSources = $derived((data?.sources || []).sort((a, b) => b.percentage - a.percentage));

	let donutSegments = $derived(calculateDonutSegments(sortedSources));

	function calculateDonutSegments(
		sources: Array<{ name: string; percentage: number; color: string }>
	) {
		let offset = 0;
		return sources.map((source) => {
			const segment = {
				...source,
				offset: offset,
				dashArray: `${source.percentage} ${100 - source.percentage}`
			};
			offset += source.percentage;
			return segment;
		});
	}

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toLocaleString();
	}

	const defaultColors = ['#4f46e5', '#22c55e', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4'];

	function getSourceIcon(name: string): string {
		const lower = name.toLowerCase();
		if (lower.includes('organic') || lower.includes('search'))
			return '<circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>';
		if (lower.includes('direct'))
			return '<line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/>';
		if (lower.includes('social'))
			return '<path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"/>';
		if (lower.includes('referral'))
			return '<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>';
		if (lower.includes('email'))
			return '<path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/><polyline points="22,6 12,13 2,6"/>';
		if (lower.includes('paid') || lower.includes('ads'))
			return '<rect x="2" y="7" width="20" height="14" rx="2" ry="2"/><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/>';
		return '<circle cx="12" cy="12" r="10"/>';
	}
</script>

<div class="traffic-widget">
	<!-- Donut Chart Section -->
	<div class="chart-section">
		<div class="donut-container">
			<svg viewBox="0 0 42 42" class="donut-chart">
				<circle class="donut-hole" cx="21" cy="21" r="15.91549430918954" fill="#fff" />
				<circle
					class="donut-ring"
					cx="21"
					cy="21"
					r="15.91549430918954"
					fill="transparent"
					stroke="#e5e7eb"
					stroke-width="4"
				/>
				{#each donutSegments as segment, i}
					<circle
						class="donut-segment"
						cx="21"
						cy="21"
						r="15.91549430918954"
						fill="transparent"
						stroke={segment.color || defaultColors[i % defaultColors.length]}
						stroke-width="4"
						stroke-dasharray={segment.dashArray}
						stroke-dashoffset={25 - segment.offset}
						style="--delay: {i * 0.1}s"
					/>
				{/each}
			</svg>
			<div class="donut-center">
				<span class="center-value">{formatNumber(Math.round($totalVisitors))}</span>
				<span class="center-label">Total Visitors</span>
			</div>
		</div>
	</div>

	<!-- Sources List -->
	<div class="sources-section">
		<div class="sources-header">
			<span class="header-title">Traffic Sources</span>
			<span class="header-period">{data?.period || 'Last 30 days'}</span>
		</div>

		<div class="sources-list">
			{#each sortedSources as source, i}
				<div class="source-item">
					<div class="source-left">
						<div
							class="source-icon"
							style="background: {source.color || defaultColors[i % defaultColors.length]}15; color: {source.color ||
								defaultColors[i % defaultColors.length]}"
						>
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								{@html getSourceIcon(source.name)}
							</svg>
						</div>
						<div class="source-info">
							<span class="source-name">{source.name}</span>
							<span class="source-visitors">{formatNumber(source.visitors)} visitors</span>
						</div>
					</div>
					<div class="source-right">
						<div class="source-percentage">
							<span
								class="percentage-value"
								style="color: {source.color || defaultColors[i % defaultColors.length]}"
								>{source.percentage.toFixed(1)}%</span
							>
							<div
								class="percentage-bar"
								style="background: {source.color || defaultColors[i % defaultColors.length]}20"
							>
								<div
									class="percentage-fill"
									style="width: {source.percentage}%; background: {source.color ||
										defaultColors[i % defaultColors.length]}"
								></div>
							</div>
						</div>
						{#if source.change !== 0}
							<span class="source-change" class:positive={source.change > 0}>
								{source.change > 0 ? '↑' : '↓'}
								{Math.abs(source.change)}%
							</span>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>

	<div class="widget-footer">
		<a href="/admin/analytics/traffic" class="view-all-link">View Traffic Report →</a>
	</div>
</div>

<style>
	.traffic-widget {
		height: 100%;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 0.25rem;
	}

	.chart-section {
		display: flex;
		justify-content: center;
		padding: 0.5rem;
	}

	.donut-container {
		position: relative;
		width: 140px;
		height: 140px;
	}

	.donut-chart {
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.donut-segment {
		transform-origin: center;
		animation: donut-fill 1.5s cubic-bezier(0.4, 0, 0.2, 1) forwards;
		animation-delay: var(--delay);
		opacity: 0;
	}

	@keyframes donut-fill {
		from {
			opacity: 0;
			stroke-dasharray: 0 100;
		}
		to {
			opacity: 1;
		}
	}

	.donut-center {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
	}

	.center-value {
		display: block;
		font-size: 1.375rem;
		font-weight: 800;
		color: #1f2937;
		line-height: 1.1;
	}

	.center-label {
		font-size: 0.6rem;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	.sources-section {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-height: 0;
	}

	.sources-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
		padding: 0 0.25rem;
	}

	.header-title {
		font-size: 0.8rem;
		font-weight: 600;
		color: #1f2937;
	}

	.header-period {
		font-size: 0.65rem;
		color: #9ca3af;
	}

	.sources-list {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
		overflow-y: auto;
	}

	.source-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.625rem 0.75rem;
		background: #f9fafb;
		border-radius: 10px;
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.source-item:hover {
		transform: translateX(4px);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
	}

	.source-left {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.source-icon {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.source-icon svg {
		width: 16px;
		height: 16px;
	}

	.source-info {
		display: flex;
		flex-direction: column;
	}

	.source-name {
		font-size: 0.8rem;
		font-weight: 600;
		color: #1f2937;
	}

	.source-visitors {
		font-size: 0.65rem;
		color: #6b7280;
	}

	.source-right {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.source-percentage {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 0.25rem;
	}

	.percentage-value {
		font-size: 0.8rem;
		font-weight: 700;
	}

	.percentage-bar {
		width: 60px;
		height: 4px;
		border-radius: 2px;
		overflow: hidden;
	}

	.percentage-fill {
		height: 100%;
		border-radius: 2px;
		transition: width 1s ease-out;
	}

	.source-change {
		font-size: 0.65rem;
		font-weight: 600;
		padding: 0.2rem 0.4rem;
		border-radius: 4px;
		background: #fee2e2;
		color: #ef4444;
	}

	.source-change.positive {
		background: #dcfce7;
		color: #22c55e;
	}

	.widget-footer {
		display: flex;
		justify-content: flex-end;
		padding-top: 0.75rem;
		border-top: 1px solid #e5e7eb;
		margin-top: auto;
	}

	.view-all-link {
		font-size: 0.75rem;
		color: #3b82f6;
		text-decoration: none;
		font-weight: 500;
		transition: color 0.2s;
	}

	.view-all-link:hover {
		color: #2563eb;
	}
</style>
