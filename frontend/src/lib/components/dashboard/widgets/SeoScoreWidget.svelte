<script lang="ts">
	import { onMount } from 'svelte';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';

	export let data: {
		average_scores: {
			overall: number;
			title: number;
			content: number;
			readability: number;
			technical: number;
		};
		distribution: {
			excellent: number;
			good: number;
			needs_improvement: number;
			poor: number;
		};
		total_analyzed: number;
		recommendations: Array<{
			priority: string;
			category: string;
			message: string;
		}>;
	};
	export let config: any = {};

	const animatedScore = tweened(0, { duration: 1500, easing: cubicOut });

	$: if (data?.average_scores?.overall) {
		animatedScore.set(data.average_scores.overall);
	}

	$: scoreColor = getScoreColor($animatedScore);
	$: strokeDasharray = `${($animatedScore / 100) * 283} 283`;

	function getScoreColor(score: number): string {
		if (score >= 80) return '#22c55e';
		if (score >= 60) return '#f59e0b';
		if (score >= 40) return '#f97316';
		return '#ef4444';
	}

	function getScoreLabel(score: number): string {
		if (score >= 80) return 'Excellent';
		if (score >= 60) return 'Good';
		if (score >= 40) return 'Needs Work';
		return 'Poor';
	}

	function getPriorityColor(priority: string): string {
		switch (priority) {
			case 'high':
				return '#ef4444';
			case 'medium':
				return '#f59e0b';
			default:
				return '#3b82f6';
		}
	}
</script>

<div class="seo-score-widget">
	<!-- Main Score Circle -->
	<div class="score-section">
		<div class="score-circle-container">
			<svg class="score-ring" viewBox="0 0 100 100">
				<circle class="ring-bg" cx="50" cy="50" r="45" />
				<circle
					class="ring-progress"
					cx="50"
					cy="50"
					r="45"
					style="stroke: {scoreColor}; stroke-dasharray: {strokeDasharray};"
				/>
			</svg>
			<div class="score-content">
				<span class="score-value" style="color: {scoreColor}">{Math.round($animatedScore)}</span>
				<span class="score-label">{getScoreLabel($animatedScore)}</span>
			</div>
		</div>

		<div class="score-details">
			<h4>SEO Score Breakdown</h4>
			<div class="breakdown-items">
				{#each Object.entries(data?.average_scores || {}) as [key, value]}
					{#if key !== 'overall'}
						<div class="breakdown-item">
							<div class="breakdown-header">
								<span class="breakdown-label">{key.charAt(0).toUpperCase() + key.slice(1)}</span>
								<span class="breakdown-value" style="color: {getScoreColor(value)}"
									>{Math.round(value)}/100</span
								>
							</div>
							<div class="breakdown-bar">
								<div
									class="breakdown-fill"
									style="width: {value}%; background: {getScoreColor(value)}"
								/>
							</div>
						</div>
					{/if}
				{/each}
			</div>
		</div>
	</div>

	<!-- Distribution Stats -->
	<div class="distribution-section">
		<h4>Content Distribution</h4>
		<div class="distribution-grid">
			<div class="dist-item excellent">
				<div class="dist-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
						<polyline points="22 4 12 14.01 9 11.01" />
					</svg>
				</div>
				<span class="dist-value">{data?.distribution?.excellent || 0}</span>
				<span class="dist-label">Excellent</span>
			</div>
			<div class="dist-item good">
				<div class="dist-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10" />
						<polyline points="12 6 12 12 16 14" />
					</svg>
				</div>
				<span class="dist-value">{data?.distribution?.good || 0}</span>
				<span class="dist-label">Good</span>
			</div>
			<div class="dist-item needs-work">
				<div class="dist-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10" />
						<line x1="12" y1="8" x2="12" y2="12" />
						<line x1="12" y1="16" x2="12.01" y2="16" />
					</svg>
				</div>
				<span class="dist-value">{data?.distribution?.needs_improvement || 0}</span>
				<span class="dist-label">Needs Work</span>
			</div>
			<div class="dist-item poor">
				<div class="dist-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10" />
						<line x1="15" y1="9" x2="9" y2="15" />
						<line x1="9" y1="9" x2="15" y2="15" />
					</svg>
				</div>
				<span class="dist-value">{data?.distribution?.poor || 0}</span>
				<span class="dist-label">Poor</span>
			</div>
		</div>
	</div>

	<!-- Recommendations -->
	{#if data?.recommendations?.length > 0}
		<div class="recommendations-section">
			<h4>Top Recommendations</h4>
			<div class="recommendations-list">
				{#each data.recommendations.slice(0, 3) as rec}
					<div class="recommendation-item">
						<div
							class="rec-priority"
							style="background: {getPriorityColor(rec.priority)}20; color: {getPriorityColor(
								rec.priority
							)}"
						>
							{rec.priority}
						</div>
						<p class="rec-message">{rec.message}</p>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	<div class="widget-footer">
		<span class="total-analyzed">{data?.total_analyzed || 0} pages analyzed</span>
		<a href="/admin/seo/analysis" class="view-all-link">View Full Report →</a>
	</div>
</div>

<style>
	.seo-score-widget {
		height: 100%;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		padding: 0.5rem;
	}

	.score-section {
		display: grid;
		grid-template-columns: 1fr 1.5fr;
		gap: 2rem;
		align-items: center;
	}

	.score-circle-container {
		position: relative;
		width: 140px;
		height: 140px;
		margin: 0 auto;
	}

	.score-ring {
		transform: rotate(-90deg);
		width: 100%;
		height: 100%;
	}

	.ring-bg {
		fill: none;
		stroke: #f3f4f6;
		stroke-width: 8;
	}

	.ring-progress {
		fill: none;
		stroke-width: 8;
		stroke-linecap: round;
		transition: stroke-dasharray 1.5s ease-out;
	}

	.score-content {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
	}

	.score-value {
		font-size: 2.5rem;
		font-weight: 800;
		line-height: 1;
		display: block;
	}

	.score-label {
		font-size: 0.75rem;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-top: 4px;
		display: block;
	}

	.score-details h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 1rem 0;
	}

	.breakdown-items {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.breakdown-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.breakdown-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.breakdown-label {
		font-size: 0.75rem;
		color: #6b7280;
		text-transform: capitalize;
	}

	.breakdown-value {
		font-size: 0.75rem;
		font-weight: 600;
	}

	.breakdown-bar {
		height: 6px;
		background: #f3f4f6;
		border-radius: 3px;
		overflow: hidden;
	}

	.breakdown-fill {
		height: 100%;
		border-radius: 3px;
		transition: width 1s ease-out;
	}

	.distribution-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 1rem 0;
	}

	.distribution-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.75rem;
	}

	.dist-item {
		text-align: center;
		padding: 1rem 0.5rem;
		border-radius: 12px;
		background: #f9fafb;
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.dist-item:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.dist-item.excellent {
		background: linear-gradient(135deg, #dcfce7 0%, #bbf7d0 100%);
	}
	.dist-item.excellent .dist-icon {
		color: #22c55e;
	}

	.dist-item.good {
		background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
	}
	.dist-item.good .dist-icon {
		color: #f59e0b;
	}

	.dist-item.needs-work {
		background: linear-gradient(135deg, #ffedd5 0%, #fed7aa 100%);
	}
	.dist-item.needs-work .dist-icon {
		color: #f97316;
	}

	.dist-item.poor {
		background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
	}
	.dist-item.poor .dist-icon {
		color: #ef4444;
	}

	.dist-icon {
		width: 24px;
		height: 24px;
		margin: 0 auto 0.5rem;
	}

	.dist-value {
		font-size: 1.5rem;
		font-weight: 700;
		display: block;
		color: #1f2937;
	}

	.dist-label {
		font-size: 0.65rem;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.recommendations-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.75rem 0;
	}

	.recommendations-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.recommendation-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.75rem;
		background: #f9fafb;
		border-radius: 8px;
		border-left: 3px solid transparent;
	}

	.rec-priority {
		font-size: 0.65rem;
		font-weight: 600;
		text-transform: uppercase;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		white-space: nowrap;
	}

	.rec-message {
		font-size: 0.8rem;
		color: #4b5563;
		margin: 0;
		line-height: 1.4;
	}

	.widget-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
		margin-top: auto;
	}

	.total-analyzed {
		font-size: 0.75rem;
		color: #9ca3af;
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

	@media (max-width: 640px) {
		.score-section {
			grid-template-columns: 1fr;
		}

		.distribution-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
