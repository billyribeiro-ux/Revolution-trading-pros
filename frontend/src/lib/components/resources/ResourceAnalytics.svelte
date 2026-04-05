<!--
  ResourceAnalytics.svelte
  Apple Principal Engineer ICT 7 Grade - February 2026

  Admin analytics dashboard for resources:
  - Total counts (resources, views, downloads, favorites)
  - Breakdown by type and access level
  - Top viewed/downloaded resources
  - Recent uploads
-->
<script lang="ts">
	import type { ResourceAnalytics } from '$lib/api/room-resources';
	import { getResourceAnalytics } from '$lib/api/room-resources';

	interface Props {
		roomId?: number;
		initialData?: ResourceAnalytics;
	}

	let { roomId = undefined, initialData = undefined }: Props = $props();

	let analytics: ResourceAnalytics | null = $state(null);
	let loading = $state(true);
	let error = $state('');

	$effect(() => {
		if (initialData) {
			analytics = initialData;
			loading = false;
		} else {
			loadAnalytics();
		}
	});

	async function loadAnalytics() {
		loading = true;
		error = '';

		try {
			const response = await getResourceAnalytics(roomId);
			analytics = response.data;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load analytics';
		} finally {
			loading = false;
		}
	}

	function formatNumber(num: number): string {
		if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
		if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
		return num.toString();
	}

	function getTypeIcon(type: string): string {
		switch (type) {
			case 'video':
				return 'M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z';
			case 'pdf':
				return 'M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z';
			case 'image':
				return 'M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z';
			case 'document':
				return 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z';
			case 'spreadsheet':
				return 'M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z';
			default:
				return 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z';
		}
	}

	function getAccessLevel(level: string): string {
		switch (level) {
			case 'free':
				return 'free';
			case 'member':
				return 'member';
			case 'premium':
				return 'premium';
			case 'vip':
				return 'vip';
			default:
				return 'default';
		}
	}
</script>

<div class="ra-container">
	{#if loading}
		<div class="ra-summary-grid">
			{#each Array(4) as _, i (i)}
				<div class="ra-card ra-skeleton-card">
					<div class="ra-skel-line ra-skel-w24"></div>
					<div class="ra-skel-line ra-skel-w16 ra-skel-tall"></div>
				</div>
			{/each}
		</div>
	{:else if error}
		<div class="ra-error">
			<svg class="ra-error-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
				/>
			</svg>
			<p class="ra-error-msg">{error}</p>
			<button class="ra-retry-btn" onclick={loadAnalytics}>Retry</button>
		</div>
	{:else if analytics}
		<!-- Summary Cards -->
		<div class="ra-summary-grid">
			<div class="ra-card">
				<div class="ra-card-row">
					<div class="ra-icon-box" data-variant="blue">
						<svg class="ra-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
							/>
						</svg>
					</div>
					<div>
						<p class="ra-card-label">Total Resources</p>
						<p class="ra-card-value">{formatNumber(analytics.total_resources)}</p>
					</div>
				</div>
			</div>

			<div class="ra-card">
				<div class="ra-card-row">
					<div class="ra-icon-box" data-variant="green">
						<svg class="ra-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
							/>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
							/>
						</svg>
					</div>
					<div>
						<p class="ra-card-label">Total Views</p>
						<p class="ra-card-value">{formatNumber(analytics.total_views)}</p>
					</div>
				</div>
			</div>

			<div class="ra-card">
				<div class="ra-card-row">
					<div class="ra-icon-box" data-variant="purple">
						<svg class="ra-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
							/>
						</svg>
					</div>
					<div>
						<p class="ra-card-label">Total Downloads</p>
						<p class="ra-card-value">{formatNumber(analytics.total_downloads)}</p>
					</div>
				</div>
			</div>

			<div class="ra-card">
				<div class="ra-card-row">
					<div class="ra-icon-box" data-variant="amber">
						<svg class="ra-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
							/>
						</svg>
					</div>
					<div>
						<p class="ra-card-label">Total Favorites</p>
						<p class="ra-card-value">{formatNumber(analytics.total_favorites)}</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Charts Grid -->
		<div class="ra-charts-grid">
			<!-- By Type -->
			<div class="ra-card">
				<h3 class="ra-section-title">Resources by Type</h3>
				<div class="ra-list">
					{#each analytics.by_type as item (item.resource_type)}
						{@const percentage =
							analytics.total_resources > 0 ? (item.count / analytics.total_resources) * 100 : 0}
						<div>
							<div class="ra-row-between">
								<div class="ra-row-label">
									<svg class="ra-icon-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d={getTypeIcon(item.resource_type)}
										/>
									</svg>
									<span class="ra-type-name">{item.resource_type}</span>
								</div>
								<span class="ra-count">{item.count}</span>
							</div>
							<div class="ra-bar-track">
								<div class="ra-bar-fill" style="width: {percentage}%"></div>
							</div>
							<div class="ra-bar-meta">
								<span>{item.total_views} views</span>
								<span>{item.total_downloads} downloads</span>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- By Access Level -->
			<div class="ra-card">
				<h3 class="ra-section-title">Resources by Access Level</h3>
				<div class="ra-list">
					{#each analytics.by_access_level as item (item.access_level)}
						{@const percentage =
							analytics.total_resources > 0 ? (item.count / analytics.total_resources) * 100 : 0}
						<div>
							<div class="ra-row-between">
								<div class="ra-row-label">
									<div class="ra-level-dot" data-level={getAccessLevel(item.access_level)}></div>
									<span class="ra-type-name">{item.access_level}</span>
								</div>
								<span class="ra-count">{item.count}</span>
							</div>
							<div class="ra-bar-track">
								<div
									class="ra-bar-fill"
									data-level={getAccessLevel(item.access_level)}
									style="width: {percentage}%"
								></div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<!-- Top Lists -->
		<div class="ra-top-grid">
			<div class="ra-card">
				<h3 class="ra-section-title">Top Viewed</h3>
				<div class="ra-list">
					{#each analytics.top_viewed.slice(0, 5) as item, i (item.id)}
						<div class="ra-top-item">
							<span class="ra-rank">{i + 1}</span>
							<div class="ra-item-info">
								<p class="ra-item-title">{item.title}</p>
								<p class="ra-item-meta">{formatNumber(item.views_count)} views</p>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<div class="ra-card">
				<h3 class="ra-section-title">Top Downloaded</h3>
				<div class="ra-list">
					{#each analytics.top_downloaded.slice(0, 5) as item, i (item.id)}
						<div class="ra-top-item">
							<span class="ra-rank">{i + 1}</span>
							<div class="ra-item-info">
								<p class="ra-item-title">{item.title}</p>
								<p class="ra-item-meta">{formatNumber(item.downloads_count)} downloads</p>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<div class="ra-card">
				<h3 class="ra-section-title">Recent Uploads</h3>
				<div class="ra-list">
					{#each analytics.recent_uploads.slice(0, 5) as item (item.id)}
						<div class="ra-top-item">
							<svg
								class="ra-icon-sm ra-icon-muted"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d={getTypeIcon(item.resource_type)}
								/>
							</svg>
							<div class="ra-item-info">
								<p class="ra-item-title">{item.title}</p>
								<p class="ra-item-meta">{new Date(item.created_at).toLocaleDateString()}</p>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.ra-container {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.ra-summary-grid {
		display: grid;
		gap: var(--space-6);
		grid-template-columns: 1fr;

		@media (min-width: 640px) {
			grid-template-columns: repeat(2, 1fr);
		}
		@media (min-width: 1024px) {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.ra-charts-grid {
		display: grid;
		gap: var(--space-6);
		grid-template-columns: 1fr;

		@media (min-width: 1024px) {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.ra-top-grid {
		display: grid;
		gap: var(--space-6);
		grid-template-columns: 1fr;

		@media (min-width: 1024px) {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.ra-card {
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.9 0.005 265);
		background-color: oklch(1 0 0);
		padding: var(--space-6);
	}

	.ra-card-row {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.ra-icon-box {
		display: flex;
		align-items: center;
		justify-content: center;
		inline-size: 2.5rem;
		block-size: 2.5rem;
		border-radius: var(--radius-lg);

		&[data-variant='blue'] {
			background-color: oklch(0.92 0.06 260);
			color: oklch(0.5 0.2 260);
		}
		&[data-variant='green'] {
			background-color: oklch(0.92 0.06 160);
			color: oklch(0.5 0.18 160);
		}
		&[data-variant='purple'] {
			background-color: oklch(0.92 0.06 300);
			color: oklch(0.5 0.2 300);
		}
		&[data-variant='amber'] {
			background-color: oklch(0.92 0.08 80);
			color: oklch(0.6 0.2 80);
		}
	}

	.ra-icon {
		inline-size: 1.25rem;
		block-size: 1.25rem;
	}
	.ra-icon-sm {
		inline-size: 1rem;
		block-size: 1rem;
		flex-shrink: 0;
	}
	.ra-icon-muted {
		color: oklch(0.65 0.01 265);
	}

	.ra-card-label {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
	}
	.ra-card-value {
		font-size: var(--text-2xl);
		font-weight: var(--weight-bold);
		color: oklch(0.15 0.01 265);
	}

	.ra-section-title {
		margin-block-end: var(--space-4);
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.ra-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.ra-row-between {
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: var(--text-sm);
	}

	.ra-row-label {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	.ra-type-name {
		text-transform: capitalize;
		color: oklch(0.35 0.01 265);
	}
	.ra-count {
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
	}

	.ra-bar-track {
		margin-block-start: 0.25rem;
		block-size: 0.5rem;
		inline-size: 100%;
		overflow: hidden;
		border-radius: 9999px;
		background-color: oklch(0.9 0.005 265);
	}

	.ra-bar-fill {
		block-size: 100%;
		border-radius: 9999px;
		background-color: oklch(0.6 0.2 260);

		&[data-level='free'] {
			background-color: oklch(0.6 0.18 160);
		}
		&[data-level='member'] {
			background-color: oklch(0.6 0.2 260);
		}
		&[data-level='premium'] {
			background-color: oklch(0.75 0.18 80);
		}
		&[data-level='vip'] {
			background-color: oklch(0.55 0.2 300);
		}
		&[data-level='default'] {
			background-color: oklch(0.55 0.01 265);
		}
	}

	.ra-level-dot {
		inline-size: 0.75rem;
		block-size: 0.75rem;
		border-radius: 9999px;

		&[data-level='free'] {
			background-color: oklch(0.6 0.18 160);
		}
		&[data-level='member'] {
			background-color: oklch(0.6 0.2 260);
		}
		&[data-level='premium'] {
			background-color: oklch(0.75 0.18 80);
		}
		&[data-level='vip'] {
			background-color: oklch(0.55 0.2 300);
		}
		&[data-level='default'] {
			background-color: oklch(0.55 0.01 265);
		}
	}

	.ra-bar-meta {
		margin-block-start: 0.25rem;
		display: flex;
		justify-content: space-between;
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	/* ─── Top items ─── */
	.ra-top-item {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
	}

	.ra-rank {
		display: flex;
		align-items: center;
		justify-content: center;
		inline-size: 1.5rem;
		block-size: 1.5rem;
		flex-shrink: 0;
		border-radius: 9999px;
		background-color: oklch(0.95 0.002 265);
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: oklch(0.45 0.01 265);
	}

	.ra-item-info {
		min-inline-size: 0;
		flex: 1;
	}
	.ra-item-title {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
	}
	.ra-item-meta {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	/* ─── Error state ─── */
	.ra-error {
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.75 0.15 25);
		background-color: oklch(0.97 0.02 25);
		padding: var(--space-6);
		text-align: center;
	}

	.ra-error-icon {
		margin-inline: auto;
		inline-size: 2.5rem;
		block-size: 2.5rem;
		color: oklch(0.65 0.2 25);
	}

	.ra-error-msg {
		margin-block-start: var(--space-2);
		color: oklch(0.5 0.2 25);
	}

	.ra-retry-btn {
		margin-block-start: var(--space-4);
		border-radius: var(--radius-lg);
		background-color: oklch(0.55 0.22 25);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		border: none;
		cursor: pointer;

		&:hover {
			background-color: oklch(0.45 0.22 25);
		}
	}

	/* ─── Skeleton ─── */
	.ra-skeleton-card {
		animation: pulse 2s ease-in-out infinite;
	}

	.ra-skel-line {
		border-radius: var(--radius-sm);
		background-color: oklch(0.9 0.005 265);
	}

	.ra-skel-w24 {
		block-size: 1rem;
		inline-size: 6rem;
	}
	.ra-skel-w16 {
		block-size: 2rem;
		inline-size: 4rem;
		margin-block-start: var(--space-2);
	}
	.ra-skel-tall {
		block-size: 2rem;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}
</style>
