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
	/* ═══════════════════════════════════════════════════════════════════════
	   CARD BASE
	   ═══════════════════════════════════════════════════════════════════════ */
	.video-card {
		display: block;
		text-decoration: none;
		color: inherit;
		border-radius: 14px;
		overflow: hidden;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
		transition: all 0.25s ease-out;
	}

	.video-card:hover {
		border-color: var(--color-border-strong);
		box-shadow: 0 12px 32px rgba(0, 0, 0, 0.12);
		transform: translateY(-4px);
	}

	.video-card:hover .thumbnail {
		transform: scale(1.05);
	}

	.video-card:hover .play-overlay {
		opacity: 1;
	}

	.video-card:hover .play-button {
		transform: scale(1.08);
	}

	/* Featured Variant */
	.video-card.featured {
		border-radius: 16px;
	}

	/* Compact Variant */
	.video-card.compact {
		border-radius: 10px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   THUMBNAIL
	   ═══════════════════════════════════════════════════════════════════════ */
	.thumbnail-wrapper {
		position: relative;
		aspect-ratio: 16 / 9;
		overflow: hidden;
		background: var(--color-text-primary);
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
	   PLAY OVERLAY
	   ═══════════════════════════════════════════════════════════════════════ */
	.play-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.2);
		opacity: 0;
		transition: opacity 0.25s ease;
	}

	.play-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 60px;
		height: 60px;
		background: rgba(255, 255, 255, 0.95);
		border-radius: 50%;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
		transition: transform 0.25s ease;
	}

	.featured .play-button {
		width: 80px;
		height: 80px;
	}

	.compact .play-button {
		width: 44px;
		height: 44px;
	}

	.play-icon {
		width: 26px;
		height: 26px;
		color: var(--color-text-primary);
		margin-left: 4px;
	}

	.featured .play-icon {
		width: 34px;
		height: 34px;
	}

	.compact .play-icon {
		width: 20px;
		height: 20px;
		margin-left: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   BADGES
	   ═══════════════════════════════════════════════════════════════════════ */
	.ticker-badge {
		position: absolute;
		top: 14px;
		left: 14px;
		display: flex;
		align-items: center;
		gap: 5px;
		padding: 7px 12px;
		color: var(--color-bg-card);
		font-size: 11px;
		font-weight: 700;
		border-radius: 8px;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
	}

	.featured .ticker-badge {
		top: 18px;
		left: 18px;
		padding: 8px 14px;
		font-size: 12px;
		border-radius: 10px;
	}

	.compact .ticker-badge {
		top: 10px;
		left: 10px;
		padding: 5px 10px;
		font-size: 10px;
	}

	.badge-ticker {
		font-weight: 900;
	}

	.badge-divider {
		font-size: 8px;
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

	.duration-badge {
		position: absolute;
		bottom: 14px;
		right: 14px;
		display: flex;
		align-items: center;
		gap: 5px;
		padding: 6px 10px;
		background: rgba(0, 0, 0, 0.8);
		color: var(--color-bg-card);
		font-size: 12px;
		font-weight: 600;
		border-radius: 6px;
		font-variant-numeric: tabular-nums;
		backdrop-filter: blur(4px);
	}

	.featured .duration-badge {
		bottom: 18px;
		right: 18px;
		padding: 7px 12px;
		font-size: 13px;
	}

	.compact .duration-badge {
		bottom: 10px;
		right: 10px;
		padding: 4px 8px;
		font-size: 11px;
	}

	.duration-icon {
		width: 14px;
		height: 14px;
		opacity: 0.8;
	}

	.compact .duration-icon {
		width: 12px;
		height: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   VIDEO INFO
	   ═══════════════════════════════════════════════════════════════════════ */
	.video-info {
		padding: 18px;
	}

	.featured .video-info {
		padding: 22px;
	}

	.compact .video-info {
		padding: 14px;
	}

	.video-title {
		font-size: 15px;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 6px 0;
		line-height: 1.45;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.featured .video-title {
		font-size: 18px;
		font-weight: 700;
		margin-bottom: 8px;
	}

	.compact .video-title {
		font-size: 13px;
		-webkit-line-clamp: 1;
		line-clamp: 1;
		margin-bottom: 4px;
	}

	.video-date {
		font-size: 13px;
		color: var(--color-text-muted);
	}

	.featured .video-date {
		font-size: 14px;
	}

	.compact .video-date {
		font-size: 12px;
	}
</style>
