<!--
/**
 * RelatedVideos - Related Video Suggestions Component
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 7 Principal Engineer Grade - February 2026
 *
 * FEATURES:
 * 1. Shows related videos based on tags and content type
 * 2. Compact card design for sidebar/below video
 * 3. Mobile-first responsive design
 *
 * @version 1.0.0
 */
-->
<script lang="ts">
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════

	interface Video {
		id: number;
		title: string;
		slug: string;
		thumbnail_url: string | null;
		duration: number | null;
		formatted_duration: string;
		video_date: string;
		formatted_date: string;
		views_count: number;
		content_type: string;
	}

	interface Props {
		/** Current video ID */
		videoId: number;
		/** Maximum videos to show */
		limit?: number;
		/** Title for the section */
		title?: string;
		/** Layout mode */
		layout?: 'vertical' | 'horizontal';
		/** Custom class */
		class?: string;
	}

	let {
		videoId,
		limit = 6,
		title = 'Related Videos',
		layout = 'vertical',
		class: className = ''
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════

	let videos = $state<Video[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════

	// Fetch on mount and refetch when videoId changes
	$effect(() => {
		if (!browser || !videoId) return;
		fetchRelated();
	});

	// ═══════════════════════════════════════════════════════════════════════
	// METHODS
	// ═══════════════════════════════════════════════════════════════════════

	async function fetchRelated() {
		try {
			isLoading = true;
			error = null;
			const response = await fetch(`/api/videos/${videoId}/related`);
			const data = await response.json();

			if (data.success && Array.isArray(data.data)) {
				videos = data.data.slice(0, limit);
			}
		} catch (e) {
			error = 'Failed to load related videos';
			console.error('Related videos fetch error:', e);
		} finally {
			isLoading = false;
		}
	}

	function formatViews(count: number): string {
		if (count >= 1000000) {
			return `${(count / 1000000).toFixed(1)}M views`;
		}
		if (count >= 1000) {
			return `${(count / 1000).toFixed(1)}K views`;
		}
		return `${count} views`;
	}
</script>

{#if !isLoading && videos.length > 0}
	<section class="related-videos {className}" class:related-videos--horizontal={layout === 'horizontal'}>
		<h3 class="related-videos__title">{title}</h3>

		<div class="related-videos__list">
			{#each videos as video (video.id)}
				<a href="/dashboard/videos/{video.slug}" class="related-card">
					<!-- Thumbnail -->
					<div class="related-card__thumbnail">
						{#if video.thumbnail_url}
							<img src={video.thumbnail_url} alt={video.title} loading="lazy" />
						{:else}
							<div class="related-card__placeholder">
								<svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
									<path d="M8 5v14l11-7z" />
								</svg>
							</div>
						{/if}

						{#if video.formatted_duration}
							<span class="related-card__duration">{video.formatted_duration}</span>
						{/if}
					</div>

					<!-- Info -->
					<div class="related-card__info">
						<h4 class="related-card__title">{video.title}</h4>
						<p class="related-card__meta">
							<span>{formatViews(video.views_count)}</span>
							<span class="related-card__dot"></span>
							<span>{video.formatted_date}</span>
						</p>
					</div>
				</a>
			{/each}
		</div>
	</section>
{:else if isLoading}
	<section class="related-videos {className}">
		<h3 class="related-videos__title">{title}</h3>
		<div class="related-videos__loading">
			<div class="related-videos__skeleton"></div>
			<div class="related-videos__skeleton"></div>
			<div class="related-videos__skeleton"></div>
		</div>
	</section>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   RELATED VIDEOS - 2026 Mobile-First Responsive Design
	   ═══════════════════════════════════════════════════════════════════════════ */

	.related-videos {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.related-videos__title {
		font-size: 1rem;
		font-weight: 600;
		color: var(--color-text, #1f2937);
		margin: 0;
	}

	.related-videos__list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	/* Card */
	.related-card {
		display: flex;
		gap: 0.75rem;
		text-decoration: none;
		color: inherit;
		border-radius: 8px;
		padding: 0.25rem;
		transition: background 0.15s ease;
	}

	.related-card:hover {
		background: var(--color-bg-hover, #f9fafb);
	}

	/* Thumbnail */
	.related-card__thumbnail {
		position: relative;
		flex-shrink: 0;
		width: 120px;
		aspect-ratio: 16 / 9;
		border-radius: 6px;
		overflow: hidden;
		background: #1f2937;
	}

	.related-card__thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.related-card__placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
		color: rgba(255, 255, 255, 0.5);
	}

	.related-card__duration {
		position: absolute;
		bottom: 4px;
		right: 4px;
		padding: 2px 4px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		font-size: 0.6875rem;
		font-weight: 500;
		border-radius: 3px;
	}

	/* Info */
	.related-card__info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		min-width: 0;
		padding-top: 0.125rem;
	}

	.related-card__title {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--color-text, #1f2937);
		margin: 0;
		line-height: 1.3;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.related-card__meta {
		display: flex;
		align-items: center;
		font-size: 0.75rem;
		color: var(--color-text-muted, #6b7280);
		margin: 0;
	}

	.related-card__dot {
		width: 3px;
		height: 3px;
		border-radius: 50%;
		background: currentColor;
		margin: 0 0.375rem;
	}

	/* Loading */
	.related-videos__loading {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.related-videos__skeleton {
		display: flex;
		gap: 0.75rem;
		padding: 0.25rem;
	}

	.related-videos__skeleton::before {
		content: '';
		flex-shrink: 0;
		width: 120px;
		aspect-ratio: 16 / 9;
		border-radius: 6px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: skeleton 1.2s ease-in-out infinite;
	}

	.related-videos__skeleton::after {
		content: '';
		flex: 1;
		height: 40px;
		border-radius: 4px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: skeleton 1.2s ease-in-out infinite;
	}

	@keyframes skeleton {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Horizontal layout */
	.related-videos--horizontal .related-videos__list {
		flex-direction: row;
		overflow-x: auto;
		padding-bottom: 0.5rem;
		-webkit-overflow-scrolling: touch;
	}

	.related-videos--horizontal .related-card {
		flex-direction: column;
		flex-shrink: 0;
		width: 160px;
	}

	.related-videos--horizontal .related-card__thumbnail {
		width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 640px) {
		.related-card__thumbnail {
			width: 140px;
		}

		.related-card__title {
			font-size: 0.875rem;
		}

		.related-videos--horizontal .related-card {
			width: 180px;
		}
	}

	/* Dark mode */
	@media (prefers-color-scheme: dark) {
		.related-videos__title {
			color: #f9fafb;
		}

		.related-card:hover {
			background: #374151;
		}

		.related-card__title {
			color: #f9fafb;
		}

		.related-card__meta {
			color: #9ca3af;
		}

		.related-videos__skeleton::before,
		.related-videos__skeleton::after {
			background: linear-gradient(90deg, #374151 25%, #4b5563 50%, #374151 75%);
			background-size: 200% 100%;
		}
	}
</style>
