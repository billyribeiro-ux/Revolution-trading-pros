<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * VideoCard Component - Video Thumbnail Card
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays a video with thumbnail, overlay badge, and metadata
	 * @version 4.1.0 - Visual Polish Pass
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { Video } from '../types';
	import { formatDate } from '../utils/formatters';

	interface Props {
		video: Video;
		variant?: 'default' | 'featured' | 'compact';
	}

	const { video, variant = 'default' }: Props = $props();

	const TYPE_STYLES = {
		ENTRY: 'type-entry',
		EXIT: 'type-exit',
		UPDATE: 'type-update',
		BREAKDOWN: 'type-breakdown'
	} as const;

	const typeClass = $derived(
		video.type ? (TYPE_STYLES[video.type] ?? 'type-default') : 'type-default'
	);

	const typeText = $derived(video.type || '');
</script>

<a
	href={video.videoUrl}
	class="video-card"
	class:featured={variant === 'featured'}
	class:compact={variant === 'compact'}
	aria-label="Watch {video.title}"
>
	<div class="thumbnail-wrapper">
		<img src={video.thumbnailUrl} alt="" role="presentation" class="thumbnail" loading="lazy" />

		<!-- Gradient Overlay -->
		<div class="thumbnail-gradient"></div>

		<!-- Play Button Overlay -->
		<div class="play-overlay">
			<div class="play-button">
				<svg viewBox="0 0 24 24" fill="currentColor" class="play-icon">
					<path d="M8 5v14l11-7z" />
				</svg>
			</div>
		</div>

		<!-- Ticker + Type Badge -->
		{#if video.ticker || video.type}
			<div class="ticker-badge {typeClass}">
				{#if video.ticker}
					<span class="badge-ticker">{video.ticker}</span>
				{/if}
				{#if video.ticker && video.type}
					<span class="badge-divider">▶</span>
				{/if}
				{#if video.type}
					<span class="badge-type">{typeText}</span>
				{/if}
			</div>
		{/if}

		<!-- Duration Badge -->
		<div class="duration-badge">
			<svg viewBox="0 0 20 20" fill="currentColor" class="duration-icon">
				<path
					fill-rule="evenodd"
					d="M10 18a8 8 0 100-16 8 8 0 000 16zm.75-13a.75.75 0 00-1.5 0v5c0 .414.336.75.75.75h4a.75.75 0 000-1.5h-3.25V5z"
					clip-rule="evenodd"
				/>
			</svg>
			{video.duration}
		</div>
	</div>

	<div class="video-info">
		<h3 class="video-title">{video.title}</h3>
		<time class="video-date" datetime={video.publishedAt.toISOString()}>
			{formatDate(video.publishedAt)}
		</time>
	</div>
</a>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   EXPLOSIVE SWINGS VIDEO CARD - 2026 Mobile-First Responsive Design
	   ═══════════════════════════════════════════════════════════════════════════
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   Touch targets: 44x44px minimum for all interactive elements
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* CARD BASE - Mobile first */
	.video-card {
		display: block;
		text-decoration: none;
		color: inherit;
		border-radius: 10px;
		overflow: hidden;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
		transition: all 0.25s ease-out;
		-webkit-tap-highlight-color: transparent;
	}

	/* Featured Variant */
	.video-card.featured {
		border-radius: 12px;
	}

	/* Compact Variant */
	.video-card.compact {
		border-radius: 8px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   THUMBNAIL - Mobile first with aspect-ratio
	   ═══════════════════════════════════════════════════════════════════════ */
	.thumbnail-wrapper {
		position: relative;
		width: 100%;
		aspect-ratio: 16 / 9;
		overflow: hidden;
		background: var(--color-text-primary);
	}

	/* Fallback for browsers without aspect-ratio */
	@supports not (aspect-ratio: 16 / 9) {
		.thumbnail-wrapper {
			padding-top: 56.25%;
		}

		.thumbnail-wrapper > * {
			position: absolute;
			top: 0;
			left: 0;
		}
	}

	.thumbnail {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.4s ease-out;
	}

	.thumbnail-gradient {
		position: absolute;
		inset: 0;
		background: linear-gradient(180deg, rgba(0, 0, 0, 0) 50%, rgba(0, 0, 0, 0.4) 100%);
		pointer-events: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   PLAY OVERLAY - Always visible on mobile for discoverability
	   ═══════════════════════════════════════════════════════════════════════ */
	.play-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.25);
		opacity: 1;
		transition: opacity 0.25s ease;
	}

	/* Play button - Mobile first: 48px touch target (exceeds 44px minimum) */
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
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
		transition: transform 0.25s ease;
		touch-action: manipulation;
	}

	.featured .play-button {
		width: 56px;
		height: 56px;
	}

	.compact .play-button {
		width: 44px;
		height: 44px;
	}

	.play-icon {
		width: 22px;
		height: 22px;
		color: var(--color-text-primary);
		margin-left: 3px;
	}

	.featured .play-icon {
		width: 26px;
		height: 26px;
	}

	.compact .play-icon {
		width: 18px;
		height: 18px;
		margin-left: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   BADGES - Mobile first
	   ═══════════════════════════════════════════════════════════════════════ */
	.ticker-badge {
		position: absolute;
		top: 10px;
		left: 10px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 5px 10px;
		color: var(--color-bg-card);
		font-size: 10px;
		font-weight: 700;
		border-radius: 6px;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
	}

	.featured .ticker-badge {
		top: 12px;
		left: 12px;
		padding: 6px 12px;
		font-size: 11px;
	}

	.compact .ticker-badge {
		top: 8px;
		left: 8px;
		padding: 4px 8px;
		font-size: 9px;
	}

	.badge-ticker {
		font-weight: 900;
	}

	.badge-divider {
		font-size: 7px;
		opacity: 0.7;
	}

	.badge-type {
		font-weight: 600;
	}

	/* Badge Type Colors */
	.ticker-badge.type-entry {
		background: #0d9488;
	}
	.ticker-badge.type-exit {
		background: var(--color-profit);
	}
	.ticker-badge.type-update {
		background: var(--color-watching-hover);
	}
	.ticker-badge.type-breakdown {
		background: var(--color-text-tertiary);
	}
	.ticker-badge.type-default {
		background: var(--color-text-tertiary);
	}

	/* Duration badge - Mobile first */
	.duration-badge {
		position: absolute;
		bottom: 10px;
		right: 10px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 4px 8px;
		background: rgba(0, 0, 0, 0.8);
		color: var(--color-bg-card);
		font-size: 11px;
		font-weight: 600;
		border-radius: 4px;
		font-variant-numeric: tabular-nums;
		backdrop-filter: blur(4px);
	}

	.featured .duration-badge {
		bottom: 12px;
		right: 12px;
		padding: 5px 10px;
		font-size: 12px;
	}

	.compact .duration-badge {
		bottom: 8px;
		right: 8px;
		padding: 3px 6px;
		font-size: 10px;
	}

	.duration-icon {
		width: 12px;
		height: 12px;
		opacity: 0.8;
	}

	.compact .duration-icon {
		width: 10px;
		height: 10px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   VIDEO INFO - Mobile first
	   ═══════════════════════════════════════════════════════════════════════ */
	.video-info {
		padding: 14px;
	}

	.featured .video-info {
		padding: 16px;
	}

	.compact .video-info {
		padding: 12px;
	}

	.video-title {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 4px 0;
		line-height: 1.45;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.featured .video-title {
		font-size: 15px;
		font-weight: 700;
		margin-bottom: 6px;
	}

	.compact .video-title {
		font-size: 12px;
		-webkit-line-clamp: 1;
		line-clamp: 1;
		margin-bottom: 3px;
	}

	.video-date {
		font-size: 12px;
		color: var(--color-text-muted);
	}

	.featured .video-date {
		font-size: 13px;
	}

	.compact .video-date {
		font-size: 11px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Mobile First (min-width queries)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ */
	@media (min-width: 360px) {
		.video-info {
			padding: 16px;
		}

		.featured .video-info {
			padding: 18px;
		}

		.video-title {
			font-size: 14px;
			margin-bottom: 5px;
		}

		.featured .video-title {
			font-size: 16px;
		}
	}

	/* sm: 640px+ - Enable hover effects */
	@media (min-width: 640px) {
		.video-card {
			border-radius: 12px;
		}

		.video-card:hover {
			border-color: var(--color-border-strong);
			box-shadow: 0 12px 32px rgba(0, 0, 0, 0.12);
			transform: translateY(-4px);
		}

		.video-card:hover .thumbnail {
			transform: scale(1.05);
		}

		.video-card:hover .play-button {
			transform: scale(1.08);
		}

		.play-overlay {
			opacity: 0;
		}

		.video-card:hover .play-overlay {
			opacity: 1;
		}

		.play-button {
			width: 56px;
			height: 56px;
		}

		.featured .play-button {
			width: 72px;
			height: 72px;
		}

		.play-icon {
			width: 24px;
			height: 24px;
			margin-left: 4px;
		}

		.featured .play-icon {
			width: 32px;
			height: 32px;
		}

		.video-card.featured {
			border-radius: 14px;
		}

		.ticker-badge {
			top: 12px;
			left: 12px;
			padding: 6px 11px;
			font-size: 11px;
			gap: 5px;
		}

		.featured .ticker-badge {
			top: 16px;
			left: 16px;
			padding: 7px 13px;
			font-size: 12px;
			border-radius: 9px;
		}

		.duration-badge {
			bottom: 12px;
			right: 12px;
			padding: 5px 9px;
			font-size: 12px;
			gap: 5px;
			border-radius: 5px;
		}

		.featured .duration-badge {
			bottom: 16px;
			right: 16px;
			padding: 6px 11px;
			font-size: 13px;
		}

		.duration-icon {
			width: 13px;
			height: 13px;
		}

		.video-info {
			padding: 17px;
		}

		.featured .video-info {
			padding: 20px;
		}

		.video-title {
			font-size: 15px;
			margin-bottom: 6px;
		}

		.featured .video-title {
			font-size: 17px;
			margin-bottom: 7px;
		}

		.video-date {
			font-size: 13px;
		}

		.featured .video-date {
			font-size: 14px;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.video-card {
			border-radius: 14px;
		}

		.video-card.featured {
			border-radius: 16px;
		}

		.play-button {
			width: 60px;
			height: 60px;
		}

		.featured .play-button {
			width: 80px;
			height: 80px;
		}

		.play-icon {
			width: 26px;
			height: 26px;
		}

		.featured .play-icon {
			width: 34px;
			height: 34px;
		}

		.ticker-badge {
			top: 14px;
			left: 14px;
			padding: 7px 12px;
			border-radius: 8px;
		}

		.featured .ticker-badge {
			top: 18px;
			left: 18px;
			padding: 8px 14px;
			border-radius: 10px;
		}

		.duration-badge {
			bottom: 14px;
			right: 14px;
			padding: 6px 10px;
			border-radius: 6px;
		}

		.featured .duration-badge {
			bottom: 18px;
			right: 18px;
			padding: 7px 12px;
		}

		.duration-icon {
			width: 14px;
			height: 14px;
		}

		.video-info {
			padding: 18px;
		}

		.featured .video-info {
			padding: 22px;
		}

		.featured .video-title {
			font-size: 18px;
			margin-bottom: 8px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.video-card,
		.thumbnail,
		.play-overlay,
		.play-button {
			transition: none;
		}

		.video-card:hover {
			transform: none;
		}

		.video-card:hover .thumbnail {
			transform: none;
		}
	}

	.video-card:focus-visible {
		outline: 2px solid var(--color-brand-primary, #0984ae);
		outline-offset: 2px;
	}
</style>
