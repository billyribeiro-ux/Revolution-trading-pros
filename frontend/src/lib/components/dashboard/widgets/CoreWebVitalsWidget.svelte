<script lang="ts">
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	export let data: {
		lcp: { value: number; rating: 'good' | 'needs-improvement' | 'poor' };
		fid: { value: number; rating: 'good' | 'needs-improvement' | 'poor' };
		cls: { value: number; rating: 'good' | 'needs-improvement' | 'poor' };
		ttfb: { value: number; rating: 'good' | 'needs-improvement' | 'poor' };
		fcp: { value: number; rating: 'good' | 'needs-improvement' | 'poor' };
		inp: { value: number; rating: 'good' | 'needs-improvement' | 'poor' };
		overall_score: number;
		origin_summary: {
			good_percent: number;
			needs_improvement_percent: number;
			poor_percent: number;
		};
	};
	// Config available for widget customization
	export const config: Record<string, unknown> = {};

	const overallScore = tweened(0, { duration: 1500, easing: cubicOut });

	onMount(() => {
		overallScore.set(data?.overall_score || 0);
	});

	interface Vital {
		key: string;
		name: string;
		description: string;
		value: number;
		unit: string;
		rating: 'good' | 'needs-improvement' | 'poor';
		thresholds: { good: number; poor: number };
	}

	$: vitals = [
		{
			key: 'lcp',
			name: 'LCP',
			description: 'Largest Contentful Paint',
			value: data?.lcp?.value || 0,
			unit: 's',
			rating: data?.lcp?.rating || 'poor',
			thresholds: { good: 2.5, poor: 4.0 }
		},
		{
			key: 'fid',
			name: 'FID',
			description: 'First Input Delay',
			value: data?.fid?.value || 0,
			unit: 'ms',
			rating: data?.fid?.rating || 'poor',
			thresholds: { good: 100, poor: 300 }
		},
		{
			key: 'cls',
			name: 'CLS',
			description: 'Cumulative Layout Shift',
			value: data?.cls?.value || 0,
			unit: '',
			rating: data?.cls?.rating || 'poor',
			thresholds: { good: 0.1, poor: 0.25 }
		},
		{
			key: 'inp',
			name: 'INP',
			description: 'Interaction to Next Paint',
			value: data?.inp?.value || 0,
			unit: 'ms',
			rating: data?.inp?.rating || 'poor',
			thresholds: { good: 200, poor: 500 }
		},
		{
			key: 'fcp',
			name: 'FCP',
			description: 'First Contentful Paint',
			value: data?.fcp?.value || 0,
			unit: 's',
			rating: data?.fcp?.rating || 'poor',
			thresholds: { good: 1.8, poor: 3.0 }
		},
		{
			key: 'ttfb',
			name: 'TTFB',
			description: 'Time to First Byte',
			value: data?.ttfb?.value || 0,
			unit: 'ms',
			rating: data?.ttfb?.rating || 'poor',
			thresholds: { good: 800, poor: 1800 }
		}
	];

	function getRatingColor(rating: string): string {
		switch (rating) {
			case 'good':
				return '#22c55e';
			case 'needs-improvement':
				return '#f59e0b';
			case 'poor':
				return '#ef4444';
			default:
				return '#9ca3af';
		}
	}

	function getRatingBg(rating: string): string {
		switch (rating) {
			case 'good':
				return 'linear-gradient(135deg, #dcfce7 0%, #bbf7d0 100%)';
			case 'needs-improvement':
				return 'linear-gradient(135deg, #fef3c7 0%, #fde68a 100%)';
			case 'poor':
				return 'linear-gradient(135deg, #fee2e2 0%, #fecaca 100%)';
			default:
				return '#f3f4f6';
		}
	}

	function getRatingLabel(rating: string): string {
		switch (rating) {
			case 'good':
				return 'Good';
			case 'needs-improvement':
				return 'Needs Work';
			case 'poor':
				return 'Poor';
			default:
				return 'Unknown';
		}
	}

	function getScoreColor(score: number): string {
		if (score >= 90) return '#22c55e';
		if (score >= 50) return '#f59e0b';
		return '#ef4444';
	}

	function formatValue(value: number, unit: string): string {
		if (unit === 's') return value.toFixed(2) + 's';
		if (unit === 'ms') return Math.round(value) + 'ms';
		return value.toFixed(3);
	}

	function getProgressWidth(vital: Vital): number {
		const { value, thresholds } = vital;
		const maxValue = thresholds.poor * 1.5;
		return Math.min((value / maxValue) * 100, 100);
	}
</script>

<div class="cwv-widget">
	<!-- Overall Score Circle -->
	<div class="score-section">
		<div class="score-circle" style="--score-color: {getScoreColor($overallScore)}">
			<svg viewBox="0 0 100 100" class="score-ring">
				<circle class="score-ring-bg" cx="50" cy="50" r="42" />
				<circle
					class="score-ring-fill"
					cx="50"
					cy="50"
					r="42"
					style="stroke-dashoffset: {264 - (264 * $overallScore) / 100}; stroke: {getScoreColor(
						$overallScore
					)}"
				/>
			</svg>
			<div class="score-content">
				<span class="score-value" style="color: {getScoreColor($overallScore)}"
					>{Math.round($overallScore)}</span
				>
				<span class="score-label">Performance</span>
			</div>
		</div>

		<div class="origin-summary">
			<div class="origin-bar">
				<div
					class="origin-segment good"
					style="width: {data?.origin_summary?.good_percent || 0}%"
				></div>
				<div
					class="origin-segment warning"
					style="width: {data?.origin_summary?.needs_improvement_percent || 0}%"
				></div>
				<div class="origin-segment poor" style="width: {data?.origin_summary?.poor_percent || 0}%"></div>
			</div>
			<div class="origin-legend">
				<span class="legend-item">
					<span class="legend-dot good"></span>
					{data?.origin_summary?.good_percent || 0}% Good
				</span>
				<span class="legend-item">
					<span class="legend-dot warning"></span>
					{data?.origin_summary?.needs_improvement_percent || 0}% Needs Work
				</span>
				<span class="legend-item">
					<span class="legend-dot poor"></span>
					{data?.origin_summary?.poor_percent || 0}% Poor
				</span>
			</div>
		</div>
	</div>

	<!-- Vitals Grid -->
	<div class="vitals-grid">
		{#each vitals as vital}
			<div class="vital-card" style="background: {getRatingBg(vital.rating)}">
				<div class="vital-header">
					<span class="vital-name">{vital.name}</span>
					<span class="vital-badge" style="background: {getRatingColor(vital.rating)}"
						>{getRatingLabel(vital.rating)}</span
					>
				</div>
				<div class="vital-value" style="color: {getRatingColor(vital.rating)}">
					{formatValue(vital.value, vital.unit)}
				</div>
				<div class="vital-description">{vital.description}</div>
				<div class="vital-progress">
					<div class="progress-track">
						<div class="progress-markers">
							<div
								class="marker good"
								style="left: {(vital.thresholds.good / (vital.thresholds.poor * 1.5)) * 100}%"
							></div>
							<div
								class="marker poor"
								style="left: {(vital.thresholds.poor / (vital.thresholds.poor * 1.5)) * 100}%"
							></div>
						</div>
						<div
							class="progress-fill"
							style="width: {getProgressWidth(vital)}%; background: {getRatingColor(vital.rating)}"
						></div>
					</div>
				</div>
			</div>
		{/each}
	</div>

	<div class="widget-footer">
		<span class="data-source">Data from Google PageSpeed Insights</span>
		<a href="/admin/seo/performance" class="view-details">View Details →</a>
	</div>
</div>

<style>
	.cwv-widget {
		height: 100%;
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		padding: 0.5rem;
	}

	.score-section {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		padding: 1rem;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border-radius: 16px;
	}

	.score-circle {
		position: relative;
		width: 100px;
		height: 100px;
		flex-shrink: 0;
	}

	.score-ring {
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.score-ring-bg {
		fill: none;
		stroke: #e5e7eb;
		stroke-width: 8;
	}

	.score-ring-fill {
		fill: none;
		stroke-width: 8;
		stroke-linecap: round;
		stroke-dasharray: 264;
		transition: stroke-dashoffset 1.5s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.score-content {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
	}

	.score-value {
		font-size: 1.75rem;
		font-weight: 800;
		line-height: 1;
	}

	.score-label {
		font-size: 0.6rem;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.origin-summary {
		flex: 1;
	}

	.origin-bar {
		display: flex;
		height: 10px;
		border-radius: 5px;
		overflow: hidden;
		background: #e5e7eb;
		margin-bottom: 0.75rem;
	}

	.origin-segment {
		transition: width 1s ease-out;
	}

	.origin-segment.good {
		background: #22c55e;
	}
	.origin-segment.warning {
		background: #f59e0b;
	}
	.origin-segment.poor {
		background: #ef4444;
	}

	.origin-legend {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.7rem;
		color: #4b5563;
	}

	.legend-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.legend-dot.good {
		background: #22c55e;
	}
	.legend-dot.warning {
		background: #f59e0b;
	}
	.legend-dot.poor {
		background: #ef4444;
	}

	.vitals-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.75rem;
		flex: 1;
	}

	.vital-card {
		border-radius: 12px;
		padding: 0.875rem;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.vital-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.vital-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.vital-name {
		font-weight: 700;
		font-size: 0.875rem;
		color: #1f2937;
	}

	.vital-badge {
		font-size: 0.55rem;
		font-weight: 600;
		color: white;
		padding: 0.2rem 0.4rem;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.vital-value {
		font-size: 1.25rem;
		font-weight: 800;
	}

	.vital-description {
		font-size: 0.6rem;
		color: #6b7280;
	}

	.vital-progress {
		margin-top: auto;
	}

	.progress-track {
		position: relative;
		height: 4px;
		background: rgba(0, 0, 0, 0.1);
		border-radius: 2px;
		overflow: visible;
	}

	.progress-markers {
		position: absolute;
		top: -4px;
		left: 0;
		right: 0;
		height: 12px;
	}

	.marker {
		position: absolute;
		width: 2px;
		height: 12px;
		border-radius: 1px;
	}

	.marker.good {
		background: rgba(34, 197, 94, 0.5);
	}
	.marker.poor {
		background: rgba(239, 68, 68, 0.5);
	}

	.progress-fill {
		height: 100%;
		border-radius: 2px;
		transition: width 1s ease-out;
	}

	.widget-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 0.75rem;
		border-top: 1px solid #e5e7eb;
		margin-top: auto;
	}

	.data-source {
		font-size: 0.65rem;
		color: #9ca3af;
	}

	.view-details {
		font-size: 0.75rem;
		color: #3b82f6;
		text-decoration: none;
		font-weight: 500;
	}

	.view-details:hover {
		color: #2563eb;
	}

	@media (max-width: 768px) {
		.vitals-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.score-section {
			flex-direction: column;
		}
	}
</style>
