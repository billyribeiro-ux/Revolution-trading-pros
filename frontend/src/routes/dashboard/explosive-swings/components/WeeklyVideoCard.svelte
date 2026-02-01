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
	/* ═══════════════════════════════════════════════════════════════════════════
	   WEEKLY VIDEO CARD - 2026 Mobile-First Responsive Design
	   ═══════════════════════════════════════════════════════════════════════════
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   Touch targets: 44x44px minimum for play button and links
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Card container - Mobile first */
	.weekly-video-card {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: 16px;
		box-shadow:
			0 1px 3px rgba(0, 0, 0, 0.04),
			0 1px 2px rgba(0, 0, 0, 0.06);
		transition: all 0.2s ease-out;
	}

	.card-title {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 11px;
		font-weight: 700;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin: 0 0 14px 0;
	}

	.title-icon {
		width: 14px;
		height: 14px;
	}

	/* Thumbnail container with aspect-ratio */
	.video-thumbnail-container {
		position: relative;
		width: 100%;
		aspect-ratio: 16 / 9;
		border-radius: 8px;
		overflow: hidden;
		margin-bottom: 12px;
		background: var(--color-text-primary);
	}

	/* Fallback for browsers without aspect-ratio */
	@supports not (aspect-ratio: 16 / 9) {
		.video-thumbnail-container {
			padding-top: 56.25%;
		}

		.video-thumbnail-container > * {
			position: absolute;
			top: 0;
			left: 0;
		}
	}

	.video-thumbnail {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.3s ease-out;
	}

	/* Play overlay - Always visible on mobile */
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
		-webkit-tap-highlight-color: transparent;
	}

	/* Play button - Mobile first: 48px touch target */
	.play-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		min-width: 44px;
		min-height: 44px;
		background: rgba(255, 255, 255, 0.95);
		border-radius: 50%;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		transition: transform 0.2s;
		touch-action: manipulation;
	}

	.play-icon {
		width: 20px;
		height: 20px;
		color: var(--color-text-primary);
		margin-left: 2px;
	}

	/* Duration badge - Mobile first */
	.duration-badge {
		position: absolute;
		bottom: 6px;
		right: 6px;
		padding: 3px 6px;
		background: rgba(0, 0, 0, 0.75);
		color: var(--color-bg-card);
		font-size: 11px;
		font-weight: 600;
		border-radius: 4px;
		font-variant-numeric: tabular-nums;
	}

	.video-title {
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 3px 0;
		line-height: 1.4;
	}

	.video-date {
		font-size: 11px;
		color: var(--color-text-muted);
		margin: 0 0 10px 0;
	}

	/* Watch link - 44px min touch target */
	.watch-full-link {
		display: inline-flex;
		align-items: center;
		gap: 5px;
		min-height: 44px;
		font-size: 13px;
		font-weight: 600;
		color: var(--color-brand-primary);
		text-decoration: none;
		padding: 8px 10px;
		margin: 0 -10px;
		border-radius: 8px;
		transition: all 0.2s ease-out;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	/* Skeleton loading states */
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
		margin-bottom: 10px;
	}

	.skeleton-text {
		height: 14px;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-border-default) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
		margin-bottom: 6px;
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

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Mobile First (min-width queries)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ */
	@media (min-width: 360px) {
		.weekly-video-card {
			padding: 18px;
		}

		.card-title {
			font-size: 12px;
			gap: 7px;
			margin-bottom: 16px;
		}

		.title-icon {
			width: 15px;
			height: 15px;
		}

		.video-thumbnail-container {
			border-radius: 9px;
			margin-bottom: 13px;
		}

		.duration-badge {
			padding: 4px 8px;
			font-size: 12px;
			bottom: 8px;
			right: 8px;
		}

		.video-title {
			font-size: 14px;
			margin-bottom: 4px;
		}

		.video-date {
			font-size: 12px;
			margin-bottom: 12px;
		}

		.watch-full-link {
			font-size: 14px;
			gap: 6px;
			padding: 8px 12px;
			margin: 0 -12px;
		}
	}

	/* sm: 640px+ - Enable hover effects */
	@media (min-width: 640px) {
		.weekly-video-card {
			padding: 20px;
			border-radius: 13px;
		}

		.weekly-video-card:hover {
			box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
		}

		.weekly-video-card:hover .video-thumbnail {
			transform: scale(1.03);
		}

		.card-title {
			font-size: 13px;
			gap: 8px;
			margin-bottom: 17px;
		}

		.title-icon {
			width: 16px;
			height: 16px;
		}

		.video-thumbnail-container {
			border-radius: 10px;
			margin-bottom: 14px;
		}

		.play-overlay:hover {
			background: rgba(0, 0, 0, 0.4);
		}

		.play-button {
			width: 52px;
			height: 52px;
		}

		.play-overlay:hover .play-button {
			transform: scale(1.05);
		}

		.play-icon {
			width: 22px;
			height: 22px;
			margin-left: 3px;
		}

		.watch-full-link:hover {
			background: var(--color-bg-subtle);
			color: var(--color-text-primary);
		}

		.skeleton-thumbnail {
			margin-bottom: 12px;
		}

		.skeleton-text {
			height: 16px;
			margin-bottom: 8px;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.weekly-video-card {
			padding: 22px;
			border-radius: 14px;
		}

		.card-title {
			margin-bottom: 18px;
		}

		.play-button {
			width: 56px;
			height: 56px;
		}

		.play-icon {
			width: 24px;
			height: 24px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.weekly-video-card,
		.video-thumbnail,
		.play-overlay,
		.play-button,
		.watch-full-link {
			transition: none;
		}

		.weekly-video-card:hover .video-thumbnail {
			transform: none;
		}

		@keyframes shimmer {
			0%,
			100% {
				background-position: 0 0;
			}
		}
	}

	.play-overlay:focus-visible,
	.watch-full-link:focus-visible {
		outline: 2px solid var(--color-brand-primary, #0984ae);
		outline-offset: 2px;
	}
</style>
