<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * WeeklyVideoCard Component - Weekly Breakdown Video
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Compact video card for the weekly breakdown in sidebar
	 * @version 4.1.0 - Visual Polish Pass
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { WeeklyVideo } from '../types';
	import { formatDateTime } from '../utils/formatters';

	interface Props {
		video: WeeklyVideo;
		isLoading?: boolean;
		onPlay?: () => void;
	}

	const { video, isLoading = false, onPlay }: Props = $props();
</script>

<div class="weekly-video-card" role="region" aria-labelledby="weekly-video-heading">
	<h3 id="weekly-video-heading" class="card-title">
		<svg viewBox="0 0 20 20" fill="currentColor" class="title-icon">
			<path
				d="M3.25 4A2.25 2.25 0 001 6.25v7.5A2.25 2.25 0 003.25 16h7.5A2.25 2.25 0 0013 13.75v-7.5A2.25 2.25 0 0010.75 4h-7.5zM19 4.75a.75.75 0 00-1.28-.53l-3 3a.75.75 0 00-.22.53v4.5c0 .199.079.39.22.53l3 3a.75.75 0 001.28-.53V4.75z"
			/>
		</svg>
		Weekly Breakdown
	</h3>

	{#if isLoading}
		<div class="skeleton-thumbnail"></div>
		<div class="skeleton-text"></div>
		<div class="skeleton-text short"></div>
	{:else}
		<div class="video-thumbnail-container">
			<img
				src={video.thumbnailUrl}
				alt="Thumbnail for {video.title}"
				class="video-thumbnail"
				loading="lazy"
			/>
			<button class="play-overlay" onclick={onPlay} aria-label="Play {video.title}">
				<div class="play-button">
					<svg viewBox="0 0 24 24" fill="currentColor" class="play-icon">
						<path d="M8 5v14l11-7z" />
					</svg>
				</div>
				<span class="duration-badge">{video.duration}</span>
			</button>
		</div>

		<h4 class="video-title">{video.title}</h4>
		<p class="video-date">Published {formatDateTime(video.publishedAt)}</p>

		<a href={video.videoUrl} class="watch-full-link" target="_blank" rel="noopener">
			Watch Full Video →
		</a>
	{/if}
</div>

<style>
	.weekly-video-card {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 14px;
		padding: 22px;
		box-shadow:
			0 1px 3px rgba(0, 0, 0, 0.04),
			0 1px 2px rgba(0, 0, 0, 0.06);
		transition: all 0.2s ease-out;
	}

	.weekly-video-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
	}

	.card-title {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		font-weight: 700;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin: 0 0 18px 0;
	}

	.title-icon {
		width: 16px;
		height: 16px;
	}

	.video-thumbnail-container {
		position: relative;
		width: 100%;
		aspect-ratio: 16 / 9;
		border-radius: 10px;
		overflow: hidden;
		margin-bottom: 14px;
		background: var(--color-text-primary);
	}

	.video-thumbnail {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.3s ease-out;
	}

	.weekly-video-card:hover .video-thumbnail {
		transform: scale(1.03);
	}

	.play-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.3);
		border: none;
		cursor: pointer;
		transition: background 0.2s;
	}

	.play-overlay:hover {
		background: rgba(0, 0, 0, 0.4);
	}

	.play-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 56px;
		height: 56px;
		background: rgba(255, 255, 255, 0.95);
		border-radius: 50%;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		transition: transform 0.2s;
	}

	.play-overlay:hover .play-button {
		transform: scale(1.05);
	}

	.play-icon {
		width: 24px;
		height: 24px;
		color: var(--color-text-primary);
		margin-left: 3px;
	}

	.duration-badge {
		position: absolute;
		bottom: 8px;
		right: 8px;
		padding: 4px 8px;
		background: rgba(0, 0, 0, 0.75);
		color: var(--color-bg-card);
		font-size: 12px;
		font-weight: 600;
		border-radius: 4px;
		font-variant-numeric: tabular-nums;
	}

	.video-title {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 4px 0;
		line-height: 1.4;
	}

	.video-date {
		font-size: 12px;
		color: var(--color-text-muted);
		margin: 0 0 12px 0;
	}

	.watch-full-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		font-size: 14px;
		font-weight: 600;
		color: var(--color-brand-primary);
		text-decoration: none;
		padding: 8px 12px;
		margin: 0 -12px;
		border-radius: 8px;
		transition: all 0.2s ease-out;
	}

	.watch-full-link:hover {
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
	}

	.skeleton-thumbnail {
		width: 100%;
		aspect-ratio: 16 / 9;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-border-default) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
		margin-bottom: 12px;
	}

	.skeleton-text {
		height: 16px;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-border-default) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
		margin-bottom: 8px;
	}

	.skeleton-text.short {
		width: 60%;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════ */
	@media (max-width: 640px) {
		.weekly-video-card {
			padding: 18px;
			border-radius: 12px;
		}

		.card-title {
			font-size: 12px;
			margin-bottom: 14px;
		}

		.video-title {
			font-size: 13px;
		}

		.play-button {
			width: 48px;
			height: 48px;
		}

		.play-icon {
			width: 20px;
			height: 20px;
		}
	}
</style>
