<script lang="ts">
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	export let data: {
		total_backlinks: number;
		referring_domains: number;
		domain_rating: number;
		new_backlinks_30d: number;
		lost_backlinks_30d: number;
		dofollow_ratio: number;
		top_anchors: Array<{ text: string; count: number }>;
		top_referring_domains: Array<{
			domain: string;
			dr: number;
			backlinks: number;
		}>;
		quality_breakdown: {
			high: number;
			medium: number;
			low: number;
		};
	};
	// Config available for widget customization
	export const config: Record<string, unknown> = {};

	const totalBacklinks = tweened(0, { duration: 1500, easing: cubicOut });
	const referringDomains = tweened(0, { duration: 1500, easing: cubicOut });
	const domainRating = tweened(0, { duration: 2000, easing: cubicOut });

	onMount(() => {
		totalBacklinks.set(data?.total_backlinks || 0);
		referringDomains.set(data?.referring_domains || 0);
		domainRating.set(data?.domain_rating || 0);
	});

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toLocaleString();
	}

	function getDRColor(dr: number): string {
		if (dr >= 70) return '#22c55e';
		if (dr >= 40) return '#f59e0b';
		return '#ef4444';
	}

	function getQualityColor(quality: string): string {
		switch (quality) {
			case 'high':
				return '#22c55e';
			case 'medium':
				return '#f59e0b';
			case 'low':
				return '#ef4444';
			default:
				return '#9ca3af';
		}
	}
</script>

<div class="backlink-widget">
	<!-- Hero Stats -->
	<div class="hero-stats">
		<div class="dr-card">
			<div class="dr-circle" style="--dr-color: {getDRColor($domainRating)}">
				<svg viewBox="0 0 100 100" class="dr-ring">
					<circle class="dr-ring-bg" cx="50" cy="50" r="42" />
					<circle
						class="dr-ring-fill"
						cx="50"
						cy="50"
						r="42"
						style="stroke-dashoffset: {264 - (264 * $domainRating) / 100}; stroke: {getDRColor(
							$domainRating
						)}"
					/>
				</svg>
				<div class="dr-content">
					<span class="dr-value">{Math.round($domainRating)}</span>
					<span class="dr-label">DR</span>
				</div>
			</div>
			<div class="dr-info">
				<span class="dr-title">Domain Rating</span>
				<span class="dr-desc">Website authority score</span>
			</div>
		</div>

		<div class="stats-grid">
			<div class="stat-card total">
				<div class="stat-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path
							d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"
						/><path
							d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"
						/>
					</svg>
				</div>
				<span class="stat-value">{formatNumber(Math.round($totalBacklinks))}</span>
				<span class="stat-label">Total Backlinks</span>
			</div>

			<div class="stat-card domains">
				<div class="stat-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10" /><line x1="2" y1="12" x2="22" y2="12" /><path
							d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"
						/>
					</svg>
				</div>
				<span class="stat-value">{formatNumber(Math.round($referringDomains))}</span>
				<span class="stat-label">Referring Domains</span>
			</div>

			<div class="stat-card new">
				<div class="stat-icon success">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<line x1="12" y1="5" x2="12" y2="19" /><line x1="5" y1="12" x2="19" y2="12" />
					</svg>
				</div>
				<span class="stat-value positive">+{formatNumber(data?.new_backlinks_30d || 0)}</span>
				<span class="stat-label">New (30d)</span>
			</div>

			<div class="stat-card lost">
				<div class="stat-icon danger">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<line x1="5" y1="12" x2="19" y2="12" />
					</svg>
				</div>
				<span class="stat-value negative">-{formatNumber(data?.lost_backlinks_30d || 0)}</span>
				<span class="stat-label">Lost (30d)</span>
			</div>
		</div>
	</div>

	<!-- Quality Breakdown -->
	<div class="quality-section">
		<div class="section-header">
			<span class="section-title">Link Quality</span>
			<span class="dofollow-badge">{data?.dofollow_ratio || 0}% DoFollow</span>
		</div>
		<div class="quality-bar">
			<div class="quality-segment high" style="width: {data?.quality_breakdown?.high || 0}%"></div>
			<div class="quality-segment medium" style="width: {data?.quality_breakdown?.medium || 0}%"></div>
			<div class="quality-segment low" style="width: {data?.quality_breakdown?.low || 0}%"></div>
		</div>
		<div class="quality-legend">
			<div class="legend-item">
				<span class="legend-dot high"></span>
				<span class="legend-text">High ({data?.quality_breakdown?.high || 0}%)</span>
			</div>
			<div class="legend-item">
				<span class="legend-dot medium"></span>
				<span class="legend-text">Medium ({data?.quality_breakdown?.medium || 0}%)</span>
			</div>
			<div class="legend-item">
				<span class="legend-dot low"></span>
				<span class="legend-text">Low ({data?.quality_breakdown?.low || 0}%)</span>
			</div>
		</div>
	</div>

	<!-- Top Referring Domains -->
	<div class="domains-section">
		<div class="section-header">
			<span class="section-title">Top Referring Domains</span>
		</div>
		<div class="domains-list">
			{#each (data?.top_referring_domains || []).slice(0, 4) as domain}
				<div class="domain-item">
					<div class="domain-info">
						<span class="domain-name">{domain.domain}</span>
						<span class="domain-links">{domain.backlinks} links</span>
					</div>
					<div class="domain-dr" style="background: {getDRColor(domain.dr)}15">
						<span class="dr-text" style="color: {getDRColor(domain.dr)}">DR {domain.dr}</span>
					</div>
				</div>
			{/each}
		</div>
	</div>

	<!-- Top Anchors -->
	<div class="anchors-section">
		<div class="section-header">
			<span class="section-title">Top Anchor Texts</span>
		</div>
		<div class="anchors-cloud">
			{#each (data?.top_anchors || []).slice(0, 6) as anchor, i}
				<span
					class="anchor-tag"
					style="font-size: {0.65 + (6 - i) * 0.03}rem; opacity: {1 - i * 0.1}"
				>
					{anchor.text}
					<span class="anchor-count">{anchor.count}</span>
				</span>
			{/each}
		</div>
	</div>

	<div class="widget-footer">
		<a href="/admin/seo/backlinks" class="view-all-link">View All Backlinks →</a>
	</div>
</div>

<style>
	.backlink-widget {
		height: 100%;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 0.25rem;
	}

	.hero-stats {
		display: flex;
		gap: 1rem;
		align-items: center;
	}

	.dr-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.dr-circle {
		position: relative;
		width: 80px;
		height: 80px;
	}

	.dr-ring {
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.dr-ring-bg {
		fill: none;
		stroke: #e5e7eb;
		stroke-width: 6;
	}

	.dr-ring-fill {
		fill: none;
		stroke-width: 6;
		stroke-linecap: round;
		stroke-dasharray: 264;
		transition: stroke-dashoffset 2s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.dr-content {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
	}

	.dr-value {
		display: block;
		font-size: 1.5rem;
		font-weight: 800;
		color: #1f2937;
		line-height: 1;
	}

	.dr-label {
		font-size: 0.6rem;
		color: #6b7280;
		font-weight: 600;
	}

	.dr-info {
		text-align: center;
	}

	.dr-title {
		display: block;
		font-size: 0.7rem;
		font-weight: 600;
		color: #1f2937;
	}

	.dr-desc {
		font-size: 0.55rem;
		color: #9ca3af;
	}

	.stats-grid {
		flex: 1;
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.5rem;
	}

	.stat-card {
		background: #f9fafb;
		border-radius: 10px;
		padding: 0.75rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		gap: 0.25rem;
	}

	.stat-icon {
		width: 28px;
		height: 28px;
		border-radius: 6px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e5e7eb;
		color: #6b7280;
	}

	.stat-icon svg {
		width: 14px;
		height: 14px;
	}

	.stat-icon.success {
		background: #dcfce7;
		color: #22c55e;
	}

	.stat-icon.danger {
		background: #fee2e2;
		color: #ef4444;
	}

	.stat-card.total .stat-icon {
		background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%);
		color: #3b82f6;
	}

	.stat-card.domains .stat-icon {
		background: linear-gradient(135deg, #f3e8ff 0%, #e9d5ff 100%);
		color: #8b5cf6;
	}

	.stat-value {
		font-size: 1rem;
		font-weight: 700;
		color: #1f2937;
	}

	.stat-value.positive {
		color: #22c55e;
	}

	.stat-value.negative {
		color: #ef4444;
	}

	.stat-label {
		font-size: 0.55rem;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	.quality-section {
		background: #f9fafb;
		border-radius: 12px;
		padding: 0.875rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.625rem;
	}

	.section-title {
		font-size: 0.75rem;
		font-weight: 600;
		color: #1f2937;
	}

	.dofollow-badge {
		font-size: 0.6rem;
		font-weight: 600;
		color: #22c55e;
		background: #dcfce7;
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
	}

	.quality-bar {
		display: flex;
		height: 8px;
		border-radius: 4px;
		overflow: hidden;
		background: #e5e7eb;
		margin-bottom: 0.5rem;
	}

	.quality-segment {
		transition: width 1s ease-out;
	}

	.quality-segment.high {
		background: #22c55e;
	}
	.quality-segment.medium {
		background: #f59e0b;
	}
	.quality-segment.low {
		background: #ef4444;
	}

	.quality-legend {
		display: flex;
		justify-content: center;
		gap: 1rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.legend-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.legend-dot.high {
		background: #22c55e;
	}
	.legend-dot.medium {
		background: #f59e0b;
	}
	.legend-dot.low {
		background: #ef4444;
	}

	.legend-text {
		font-size: 0.6rem;
		color: #6b7280;
	}

	.domains-section {
		flex: 1;
		min-height: 0;
		display: flex;
		flex-direction: column;
	}

	.domains-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.domain-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0.625rem;
		background: #f9fafb;
		border-radius: 8px;
		transition: transform 0.2s;
	}

	.domain-item:hover {
		transform: translateX(4px);
	}

	.domain-info {
		display: flex;
		flex-direction: column;
	}

	.domain-name {
		font-size: 0.75rem;
		font-weight: 600;
		color: #1f2937;
	}

	.domain-links {
		font-size: 0.6rem;
		color: #9ca3af;
	}

	.domain-dr {
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
	}

	.dr-text {
		font-size: 0.65rem;
		font-weight: 700;
	}

	.anchors-section {
		padding: 0.5rem 0;
	}

	.anchors-cloud {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 0.5rem;
	}

	.anchor-tag {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		background: #f3f4f6;
		padding: 0.25rem 0.625rem;
		border-radius: 20px;
		color: #4b5563;
		font-weight: 500;
		transition: background 0.2s;
	}

	.anchor-tag:hover {
		background: #e5e7eb;
	}

	.anchor-count {
		font-size: 0.55rem;
		background: #d1d5db;
		padding: 0.1rem 0.3rem;
		border-radius: 10px;
		color: #6b7280;
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
