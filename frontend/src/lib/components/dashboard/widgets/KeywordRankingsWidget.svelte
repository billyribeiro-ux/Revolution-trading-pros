<script lang="ts">
	export let data: {
		keywords: Array<{
			keyword: string;
			position: number;
			previous_position: number;
			change: number;
			trend: 'up' | 'down' | 'stable';
			url: string;
			search_volume: number;
		}>;
		total_keywords: number;
		avg_position: number;
		top_10_count: number;
		improved_count: number;
		declined_count: number;
	};
	export let config: any = {};

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toString();
	}

	function getPositionColor(position: number): string {
		if (position <= 3) return '#22c55e';
		if (position <= 10) return '#84cc16';
		if (position <= 20) return '#f59e0b';
		return '#ef4444';
	}

	function getTrendIcon(trend: string): string {
		switch (trend) {
			case 'up':
				return '↑';
			case 'down':
				return '↓';
			default:
				return '—';
		}
	}

	function getTrendColor(trend: string): string {
		switch (trend) {
			case 'up':
				return '#22c55e';
			case 'down':
				return '#ef4444';
			default:
				return '#9ca3af';
		}
	}
</script>

<div class="keyword-rankings-widget">
	<!-- Summary Stats -->
	<div class="summary-stats">
		<div class="summary-card highlight">
			<div class="summary-icon">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<circle cx="11" cy="11" r="8" />
					<line x1="21" y1="21" x2="16.65" y2="16.65" />
				</svg>
			</div>
			<div class="summary-content">
				<span class="summary-value">{data?.total_keywords || 0}</span>
				<span class="summary-label">Keywords Tracked</span>
			</div>
		</div>

		<div class="summary-card">
			<div class="summary-icon position">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<line x1="12" y1="20" x2="12" y2="10" />
					<line x1="18" y1="20" x2="18" y2="4" />
					<line x1="6" y1="20" x2="6" y2="16" />
				</svg>
			</div>
			<div class="summary-content">
				<span class="summary-value">{data?.avg_position?.toFixed(1) || '—'}</span>
				<span class="summary-label">Avg Position</span>
			</div>
		</div>

		<div class="summary-card">
			<div class="summary-icon top10">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
				</svg>
			</div>
			<div class="summary-content">
				<span class="summary-value">{data?.top_10_count || 0}</span>
				<span class="summary-label">Top 10</span>
			</div>
		</div>

		<div class="summary-card improved">
			<div class="summary-icon">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<polyline points="23 6 13.5 15.5 8.5 10.5 1 18" />
					<polyline points="17 6 23 6 23 12" />
				</svg>
			</div>
			<div class="summary-content">
				<span class="summary-value">{data?.improved_count || 0}</span>
				<span class="summary-label">Improved</span>
			</div>
		</div>
	</div>

	<!-- Keywords Table -->
	<div class="keywords-table-container">
		<table class="keywords-table">
			<thead>
				<tr>
					<th>Keyword</th>
					<th>Position</th>
					<th>Change</th>
					<th>Volume</th>
				</tr>
			</thead>
			<tbody>
				{#if data?.keywords?.length > 0}
					{#each data.keywords.slice(0, 8) as kw}
						<tr>
							<td class="keyword-cell">
								<span class="keyword-text">{kw.keyword}</span>
								<span class="keyword-url">{kw.url}</span>
							</td>
							<td>
								<span
									class="position-badge"
									style="background: {getPositionColor(kw.position)}20; color: {getPositionColor(
										kw.position
									)}"
								>
									#{kw.position}
								</span>
							</td>
							<td>
								<span class="change-indicator" style="color: {getTrendColor(kw.trend)}">
									<span class="trend-icon">{getTrendIcon(kw.trend)}</span>
									{#if kw.change !== 0}
										<span class="change-value">{Math.abs(kw.change)}</span>
									{/if}
								</span>
							</td>
							<td>
								<span class="volume">{formatNumber(kw.search_volume)}</span>
							</td>
						</tr>
					{/each}
				{:else}
					<tr>
						<td colspan="4" class="no-data">No keyword data available</td>
					</tr>
				{/if}
			</tbody>
		</table>
	</div>

	<!-- Performance Indicators -->
	<div class="performance-section">
		<div class="perf-item good">
			<span class="perf-count">{data?.improved_count || 0}</span>
			<span class="perf-label">Improved</span>
			<div class="perf-bar">
				<div
					class="perf-fill"
					style="width: {data?.total_keywords
						? (data.improved_count / data.total_keywords) * 100
						: 0}%"
				/>
			</div>
		</div>
		<div class="perf-item bad">
			<span class="perf-count">{data?.declined_count || 0}</span>
			<span class="perf-label">Declined</span>
			<div class="perf-bar">
				<div
					class="perf-fill"
					style="width: {data?.total_keywords
						? (data.declined_count / data.total_keywords) * 100
						: 0}%"
				/>
			</div>
		</div>
	</div>

	<div class="widget-footer">
		<a href="/admin/seo/keywords" class="view-all-link">View All Keywords →</a>
	</div>
</div>

<style>
	.keyword-rankings-widget {
		height: 100%;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 0.25rem;
	}

	.summary-stats {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.75rem;
	}

	.summary-card {
		background: #f9fafb;
		border-radius: 10px;
		padding: 0.875rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		gap: 0.5rem;
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.summary-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
	}

	.summary-card.highlight {
		background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
	}

	.summary-card.highlight .summary-value,
	.summary-card.highlight .summary-label {
		color: white;
	}

	.summary-card.highlight .summary-icon {
		background: rgba(255, 255, 255, 0.2);
		color: white;
	}

	.summary-card.improved {
		background: linear-gradient(135deg, #dcfce7 0%, #bbf7d0 100%);
	}

	.summary-card.improved .summary-icon {
		background: #22c55e;
		color: white;
	}

	.summary-icon {
		width: 36px;
		height: 36px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e5e7eb;
		color: #6b7280;
	}

	.summary-icon svg {
		width: 18px;
		height: 18px;
	}

	.summary-icon.position {
		background: #dbeafe;
		color: #2563eb;
	}

	.summary-icon.top10 {
		background: #fef3c7;
		color: #f59e0b;
	}

	.summary-content {
		display: flex;
		flex-direction: column;
	}

	.summary-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #1f2937;
	}

	.summary-label {
		font-size: 0.6rem;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	.keywords-table-container {
		flex: 1;
		overflow: auto;
		border-radius: 10px;
		border: 1px solid #e5e7eb;
	}

	.keywords-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.8rem;
	}

	.keywords-table thead {
		background: #f9fafb;
		position: sticky;
		top: 0;
	}

	.keywords-table th {
		padding: 0.75rem 0.875rem;
		text-align: left;
		font-weight: 600;
		color: #6b7280;
		font-size: 0.7rem;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-bottom: 1px solid #e5e7eb;
	}

	.keywords-table td {
		padding: 0.75rem 0.875rem;
		border-bottom: 1px solid #f3f4f6;
		color: #4b5563;
	}

	.keywords-table tr:last-child td {
		border-bottom: none;
	}

	.keywords-table tr:hover {
		background: #f9fafb;
	}

	.keyword-cell {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.keyword-text {
		font-weight: 500;
		color: #1f2937;
	}

	.keyword-url {
		font-size: 0.65rem;
		color: #9ca3af;
		max-width: 150px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.position-badge {
		font-size: 0.75rem;
		font-weight: 600;
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
	}

	.change-indicator {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-weight: 600;
	}

	.trend-icon {
		font-size: 0.875rem;
	}

	.change-value {
		font-size: 0.75rem;
	}

	.volume {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.no-data {
		text-align: center;
		color: #9ca3af;
		padding: 2rem !important;
	}

	.performance-section {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.perf-item {
		background: #f9fafb;
		border-radius: 8px;
		padding: 0.75rem;
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		align-items: center;
	}

	.perf-item.good {
		border-left: 3px solid #22c55e;
	}

	.perf-item.bad {
		border-left: 3px solid #ef4444;
	}

	.perf-count {
		font-size: 1.125rem;
		font-weight: 700;
		color: #1f2937;
	}

	.perf-label {
		font-size: 0.7rem;
		color: #6b7280;
		text-transform: uppercase;
	}

	.perf-bar {
		width: 100%;
		height: 4px;
		background: #e5e7eb;
		border-radius: 2px;
		overflow: hidden;
	}

	.perf-fill {
		height: 100%;
		border-radius: 2px;
		transition: width 1s ease-out;
	}

	.perf-item.good .perf-fill {
		background: #22c55e;
	}

	.perf-item.bad .perf-fill {
		background: #ef4444;
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

	@media (max-width: 640px) {
		.summary-stats {
			grid-template-columns: repeat(2, 1fr);
		}

		.performance-section {
			grid-template-columns: 1fr;
		}
	}
</style>
