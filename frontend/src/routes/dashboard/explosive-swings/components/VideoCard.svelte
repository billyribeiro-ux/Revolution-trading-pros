<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * VideoCard Component - Video Thumbnail Card
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays a video with thumbnail, overlay badge, and metadata
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { Video } from '../types';
	import { formatDate } from '../utils/formatters';

	interface Props {
		video: Video;
		variant?: 'default' | 'featured' | 'compact';
	}

	const { video, variant = 'default' }: Props = $props();

	const typeColors: Record<string, string> = {
		ENTRY: 'bg-teal-600',
		EXIT: 'bg-emerald-600',
		UPDATE: 'bg-amber-600',
		BREAKDOWN: 'bg-slate-600'
	};

	const badgeColor = $derived(video.type ? typeColors[video.type] || 'bg-slate-600' : 'bg-slate-600');
</script>

<a
	href={video.videoUrl}
	class="video-card"
	class:featured={variant === 'featured'}
	class:compact={variant === 'compact'}
	aria-label="Watch {video.title}"
>
	<div class="thumbnail-container">
		<img
			src={video.thumbnailUrl}
			alt=""
			class="thumbnail"
			loading="lazy"
		/>
		<div class="play-overlay">
			<div class="play-button">
				<svg viewBox="0 0 24 24" fill="currentColor" class="play-icon">
					<path d="M8 5v14l11-7z" />
				</svg>
			</div>
		</div>

		<!-- Ticker + Type Badge -->
		{#if video.ticker || video.type}
			<div class="ticker-badge {badgeColor}">
				{#if video.ticker}<span class="ticker">{video.ticker}</span>{/if}
				{#if video.ticker && video.type}<span class="badge-separator">▶</span>{/if}
				{#if video.type}<span class="type">{video.type}</span>{/if}
			</div>
		{/if}

		<!-- Duration -->
		<span class="duration-badge">{video.duration}</span>
	</div>

	<div class="video-meta">
		<h3 class="video-title">{video.title}</h3>
		<time class="video-date" datetime={video.publishedAt.toISOString()}>
			{formatDate(video.publishedAt)}
		</time>
	</div>
</a>

<style>
	.video-card {
		display: block;
		text-decoration: none;
		color: inherit;
		border-radius: 12px;
		overflow: hidden;
		background: #fff;
		border: 1px solid #e2e8f0;
		transition: all 0.2s ease-out;
	}

	.video-card:hover {
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}

	.video-card:hover .play-overlay {
		opacity: 1;
	}

	.video-card:hover .play-button {
		transform: scale(1.05);
	}

	.thumbnail-container {
		position: relative;
		aspect-ratio: 16 / 9;
		overflow: hidden;
	}

	.thumbnail {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.3s ease-out;
	}

	.video-card:hover .thumbnail {
		transform: scale(1.03);
	}

	.play-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.25);
		opacity: 0;
		transition: opacity 0.2s;
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

	.featured .play-button {
		width: 72px;
		height: 72px;
	}

	.compact .play-button {
		width: 40px;
		height: 40px;
	}

	.play-icon {
		width: 24px;
		height: 24px;
		color: #0f172a;
		margin-left: 3px;
	}

	.featured .play-icon {
		width: 32px;
		height: 32px;
	}

	.compact .play-icon {
		width: 18px;
		height: 18px;
	}

	.ticker-badge {
		position: absolute;
		top: 12px;
		left: 12px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 6px 10px;
		color: #fff;
		font-size: 11px;
		font-weight: 700;
		border-radius: 6px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.ticker {
		font-weight: 800;
	}

	.badge-separator {
		font-size: 8px;
		opacity: 0.8;
	}

	.type {
		font-weight: 600;
	}

	.duration-badge {
		position: absolute;
		bottom: 12px;
		right: 12px;
		padding: 4px 8px;
		background: rgba(0, 0, 0, 0.75);
		color: #fff;
		font-size: 12px;
		font-weight: 600;
		border-radius: 4px;
		font-variant-numeric: tabular-nums;
	}

	.video-meta {
		padding: 16px;
	}

	.featured .video-meta {
		padding: 20px;
	}

	.compact .video-meta {
		padding: 12px;
	}

	.video-title {
		font-size: 14px;
		font-weight: 600;
		color: #0f172a;
		margin: 0 0 4px 0;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.featured .video-title {
		font-size: 18px;
	}

	.compact .video-title {
		font-size: 13px;
		-webkit-line-clamp: 1;
	}

	.video-date {
		font-size: 12px;
		color: #64748b;
	}

	.featured .video-date {
		font-size: 13px;
	}
</style>
