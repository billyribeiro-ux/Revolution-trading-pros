<!--
	VideoCard.svelte - Video Thumbnail Card Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple Principal Engineer ICT 11+ Standards
	Svelte 5 January 2026 Syntax
	
	Single Responsibility: Display video thumbnail with title, date, description
	
	Features:
	- Video thumbnail with play overlay
	- Duration badge
	- Hover effects
	- Responsive layout
	
	@version 1.0.0
	@since January 2026
-->
<script lang="ts">
	import type { VideoUpdate } from '../alerts/types';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 $props() pattern
	// ═══════════════════════════════════════════════════════════════════════════
	interface Props {
		video: VideoUpdate;
		variant?: 'default' | 'compact';
	}

	let { video, variant = 'default' }: Props = $props();
</script>

{#if variant === 'compact'}
	<!-- Compact variant for sidebar -->
	<a href={video.href} class="video-card-compact">
		<div class="compact-thumb" style="background-image: url('{video.image}')">
			<div class="compact-play-icon">
				<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20" aria-hidden="true">
					<path d="M8 5v14l11-7z" />
				</svg>
			</div>
			<span class="compact-duration">{video.duration}</span>
		</div>
		<div class="compact-title">{video.title}</div>
	</a>
{:else}
	<!-- Default full card variant -->
	<a href={video.href} class="video-card">
		<div class="video-thumbnail" style="background-image: url('{video.image}')">
			<div class="play-overlay">
				<svg viewBox="0 0 24 24" fill="currentColor" width="48" height="48" aria-hidden="true">
					<path d="M8 5v14l11-7z" />
				</svg>
			</div>
			<div class="video-duration">{video.duration}</div>
		</div>
		<div class="video-content">
			<h3>{video.title}</h3>
			<p class="video-date">{video.date}</p>
			<p class="video-excerpt">{video.excerpt}</p>
		</div>
	</a>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DEFAULT VARIANT - Full Card
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-card {
		background: #fff;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
		transition: all 0.3s ease;
		text-decoration: none;
		color: inherit;
		display: block;
	}

	.video-card:hover {
		transform: translateY(-8px);
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
	}

	.video-thumbnail {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background-size: cover;
		background-position: center;
		overflow: hidden;
	}

	.play-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: background 0.3s;
	}

	.video-card:hover .play-overlay {
		background: rgba(0, 0, 0, 0.6);
	}

	.play-overlay svg {
		color: #fff;
		opacity: 0.9;
		transition: all 0.3s;
	}

	.video-card:hover .play-overlay svg {
		opacity: 1;
		transform: scale(1.1);
	}

	.video-duration {
		position: absolute;
		bottom: 12px;
		right: 12px;
		background: rgba(0, 0, 0, 0.85);
		color: #fff;
		padding: 5px 10px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
	}

	.video-content {
		padding: 20px;
		text-align: center;
	}

	.video-content h3 {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 8px 0;
		color: #333;
		line-height: 1.4;
	}

	.video-date {
		font-size: 12px;
		color: #888;
		margin: 0 0 12px 0;
	}

	.video-excerpt {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COMPACT VARIANT - For Sidebar
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-card-compact {
		text-decoration: none;
		display: block;
	}

	.compact-thumb {
		position: relative;
		aspect-ratio: 1;
		border-radius: 8px;
		background-size: cover;
		background-position: center;
		background-color: #1a1a1a;
		margin-bottom: 8px;
	}

	.compact-play-icon {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 32px;
		height: 32px;
		background: rgba(255, 255, 255, 0.9);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform 0.2s ease;
	}

	.video-card-compact:hover .compact-play-icon {
		transform: translate(-50%, -50%) scale(1.1);
	}

	.compact-play-icon svg {
		color: #1a1a1a;
		margin-left: 2px;
	}

	.compact-duration {
		position: absolute;
		bottom: 4px;
		right: 4px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		font-size: 0.625rem;
		padding: 2px 4px;
		border-radius: 3px;
		font-family: ui-monospace, monospace;
	}

	.compact-title {
		font-size: 0.6875rem;
		font-weight: 600;
		color: #333;
		line-height: 1.3;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.video-card-compact:hover .compact-title {
		color: #143e59;
	}
</style>
