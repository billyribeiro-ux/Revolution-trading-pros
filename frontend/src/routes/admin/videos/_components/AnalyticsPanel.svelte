<script lang="ts">
	import { IconChartBar, IconX } from '$lib/icons';
	import type { AnalyticsDashboard } from '$lib/api/video-advanced';

	interface UsedCategory {
		id: string;
		name: string;
		color: string;
	}

	interface Props {
		open: boolean;
		analyticsData: AnalyticsDashboard | null;
		analyticsPeriod: '7d' | '30d' | '90d';
		isLoadingAnalytics: boolean;
		usedCategories: UsedCategory[];
		formatViews: (views: number) => string;
		onClose: () => void;
	}

	let {
		open,
		analyticsData,
		analyticsPeriod = $bindable(),
		isLoadingAnalytics,
		usedCategories,
		formatViews,
		onClose
	}: Props = $props();
</script>

{#if open}
	<div class="analytics-panel">
		<div class="analytics-header">
			<h3><IconChartBar size={20} /> Video Analytics</h3>
			<div class="analytics-controls">
				<label for="analytics-period" class="sr-only">Analytics period</label>
				<select id="analytics-period" class="filter-select" bind:value={analyticsPeriod}>
					<option value="7d">Last 7 days</option>
					<option value="30d">Last 30 days</option>
					<option value="90d">Last 90 days</option>
				</select>
				<button class="btn-icon" onclick={onClose} title="Close">
					<IconX size={18} />
				</button>
			</div>
		</div>

		{#if isLoadingAnalytics}
			<div class="analytics-loading">
				<div class="spinner"></div>
				<p>Loading analytics...</p>
			</div>
		{:else if analyticsData}
			<div class="analytics-grid">
				<div class="analytics-stat">
					<span class="stat-value">{formatViews(analyticsData.total_views)}</span>
					<span class="stat-label">Total Views</span>
				</div>
				<div class="analytics-stat">
					<span class="stat-value">{analyticsData.unique_viewers.toLocaleString()}</span>
					<span class="stat-label">Unique Viewers</span>
				</div>
				<div class="analytics-stat">
					<span class="stat-value">{analyticsData.total_watch_time_hours.toFixed(1)}h</span>
					<span class="stat-label">Watch Time</span>
				</div>
				<div class="analytics-stat">
					<span class="stat-value">{(analyticsData.avg_completion_rate * 100).toFixed(0)}%</span>
					<span class="stat-label">Avg Completion</span>
				</div>
			</div>

			{#if analyticsData.top_videos && analyticsData.top_videos.length > 0}
				<div class="top-videos-section">
					<h4>Top Performing Videos</h4>
					<div class="top-videos-list">
						{#each analyticsData.top_videos.slice(0, 5) as topVideo, index (topVideo.video_id)}
							<div class="top-video-item">
								<span class="rank">#{index + 1}</span>
								<span class="title">{topVideo.title}</span>
								<span class="views">{formatViews(topVideo.views)} views</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Used Categories Quick Stats -->
			{#if usedCategories.length > 0}
				<div class="categories-stats-section">
					<h4>Categories in Use</h4>
					<div class="used-categories-tags">
						{#each usedCategories as category (category.id)}
							<span class="category-tag" style:--tag-color={category.color}>
								{category.name}
							</span>
						{/each}
					</div>
				</div>
			{/if}
		{:else}
			<div class="analytics-empty">
				<p>No analytics data available yet.</p>
			</div>
		{/if}
	</div>
{/if}

<style>
	.analytics-panel {
		background: rgba(22, 27, 34, 0.4);
		border-radius: 8px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.analytics-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.analytics-header h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.1rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.analytics-controls {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.analytics-loading,
	.analytics-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		color: var(--text-tertiary);
	}

	.analytics-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.analytics-stat {
		text-align: center;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 10px;
	}

	.analytics-stat .stat-value {
		display: block;
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
		margin-bottom: 0.25rem;
	}

	.analytics-stat .stat-label {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	.top-videos-section,
	.categories-stats-section {
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.top-videos-section h4,
	.categories-stats-section h4 {
		font-size: 0.9rem;
		font-weight: 600;
		color: var(--text-secondary);
		margin: 0 0 1rem 0;
	}

	.top-videos-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.top-video-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		background: rgba(13, 17, 23, 0.6);
		border-radius: 8px;
	}

	.top-video-item .rank {
		font-weight: 700;
		color: var(--warning-emphasis);
		min-width: 24px;
	}

	.top-video-item .title {
		flex: 1;
		color: var(--text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.top-video-item .views {
		color: var(--text-tertiary);
		font-size: 0.85rem;
	}

	.used-categories-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	/* Shared classes duplicated locally so the panel renders standalone */
	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(13, 17, 23, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		color: var(--text-primary);
		font-size: 0.9rem;
		cursor: pointer;
	}

	.filter-select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 6px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
	}

	.category-tag {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.5rem;
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, var(--primary-500)) 30%, transparent);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 500;
		color: var(--tag-color, var(--primary-500));
		white-space: nowrap;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 767.98px) {
		.analytics-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
