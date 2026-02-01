<!--
	VideoCard Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	
	Reusable video card component for learning center, daily videos, and archives.
	Matches WordPress article-card structure exactly.
	
	@version 1.0.0
-->
<script lang="ts">
	interface VideoData {
		// Core required properties
		id: number | string;
		title: string;
		slug: string;

		// Optional display properties
		description?: string | null;
		thumbnail_url?: string | null;
		tag_details?: Array<{ slug: string; name: string; color: string }>;
		trader?: { id: number; name: string; slug: string } | null;
		formatted_date?: string;
		duration?: number | null;
		formatted_duration?: string;

		// Allow any additional properties from API responses
		video_url?: string;
		embed_url?: string;
		video_platform?: string;
		content_type?: string;
		video_date?: string;
		is_published?: boolean;
		is_featured?: boolean;
		tags?: string[];
		views_count?: number;
		rooms?: Array<{ id: number; name: string; slug: string }>;
		created_at?: string;

		// Catch-all for any other properties
		[key: string]: unknown;
	}

	interface Props {
		video: VideoData;
		basePath?: string;
		showDate?: boolean;
		showDuration?: boolean;
	}

	let {
		video,
		basePath = '/learning-center',
		showDate = false,
		showDuration = false
	}: Props = $props();

	const defaultThumbnail =
		'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg';
</script>

<article class="article-card">
	<figure
		class="article-card__image"
		style="background-image: url({video.thumbnail_url || defaultThumbnail});"
	>
		<img src={defaultThumbnail} alt={video.title} loading="lazy" />
		{#if showDuration && video.formatted_duration}
			<span class="article-card__duration">{video.formatted_duration}</span>
		{/if}
	</figure>

	<div class="article-card__type">
		{#each video.tag_details || [] as tag}
			<span class="label label--info" style="background-color: {tag.color}">{tag.name}</span>
		{/each}
	</div>

	<h4 class="h5 article-card__title">
		<a href="{basePath}/{video.slug}">{video.title}</a>
	</h4>

	<div class="u--margin-top-0">
		{#if video.trader}
			<span class="trader_name"><i>With {video.trader.name}</i></span>
		{/if}
		{#if showDate && video.formatted_date}
			<span class="video-date">{video.formatted_date}</span>
		{/if}
	</div>

	<div class="article-card__excerpt u--hide-read-more">
		<p>{video.description || ''}</p>
	</div>

	<a href="{basePath}/{video.slug}" class="btn btn-tiny btn-default watch-now-btn"> Watch Now </a>
</article>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO CARD (Dashboard) - 2026 Mobile-First Responsive Design
	   ═══════════════════════════════════════════════════════════════════════════
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   Touch targets: 44x44px minimum for all interactive elements
	   ═══════════════════════════════════════════════════════════════════════════ */

	.article-card {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 6px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
		padding: 0 0 16px;
	}

	.article-card__image {
		position: relative;
		width: 100%;
		/* Modern aspect-ratio property */
		aspect-ratio: 16 / 9;
		background-size: cover;
		background-position: center;
		margin: 0;
		overflow: hidden;
	}

	/* Fallback for browsers without aspect-ratio */
	@supports not (aspect-ratio: 16 / 9) {
		.article-card__image {
			padding-top: 56.25%;
		}
	}

	.article-card__image img {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	/* Duration Badge - Mobile first */
	.article-card__duration {
		position: absolute;
		bottom: 8px;
		right: 8px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		padding: 3px 6px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	/* Tags - Mobile first */
	.article-card__type {
		padding: 12px 14px 8px;
		display: flex;
		flex-wrap: wrap;
		gap: 4px;
	}

	.label {
		display: inline-block;
		padding: 3px 8px;
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		border-radius: 4px;
	}

	.label--info {
		background: #e8f4fc;
		color: #0984ae;
	}

	/* Title - Mobile first */
	.article-card__title {
		padding: 0 14px;
		margin: 0 0 8px;
		font-size: 14px;
		font-weight: 700;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		-webkit-tap-highlight-color: transparent;
	}

	.u--margin-top-0 {
		margin-top: 0;
		padding: 0 14px;
	}

	.trader_name {
		display: block;
		font-size: 12px;
		color: #666;
		margin-bottom: 6px;
	}

	.video-date {
		display: block;
		font-size: 11px;
		color: #999;
		margin-top: 4px;
	}

	.article-card__excerpt {
		padding: 0 14px;
		font-size: 13px;
		color: #666;
		line-height: 1.5;
		flex-grow: 1;
	}

	.article-card__excerpt p {
		margin: 0 0 12px;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* CTA Button - 44px min touch target */
	.watch-now-btn {
		margin: auto 14px 0;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		padding: 10px 16px;
		background: #f5f5f5;
		color: #333;
		font-size: 12px;
		font-weight: 600;
		text-transform: uppercase;
		text-decoration: none;
		border-radius: 6px;
		transition: all 0.2s ease;
		text-align: center;
		width: fit-content;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Mobile First (min-width queries)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ */
	@media (min-width: 360px) {
		.article-card {
			padding-bottom: 18px;
		}

		.article-card__type {
			padding: 14px 16px 10px;
			gap: 5px;
		}

		.article-card__title {
			padding: 0 16px;
			font-size: 15px;
		}

		.u--margin-top-0,
		.article-card__excerpt {
			padding: 0 16px;
		}

		.watch-now-btn {
			margin-left: 16px;
			margin-right: 16px;
		}

		.article-card__duration {
			padding: 4px 8px;
			font-size: 12px;
		}
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		.article-card {
			border-radius: 8px;
			padding-bottom: 20px;
		}

		.article-card:hover {
			box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
		}

		.article-card__type {
			padding: 15px 20px 10px;
			gap: 6px;
		}

		.label {
			padding: 4px 10px;
			font-size: 11px;
		}

		.article-card__title {
			padding: 0 20px;
			margin-bottom: 10px;
			font-size: 16px;
		}

		.article-card__title a:hover {
			color: #0984ae;
		}

		.u--margin-top-0,
		.article-card__excerpt {
			padding: 0 20px;
		}

		.trader_name {
			font-size: 13px;
			margin-bottom: 8px;
		}

		.video-date {
			font-size: 12px;
		}

		.article-card__excerpt {
			font-size: 14px;
		}

		.article-card__excerpt p {
			margin-bottom: 15px;
			-webkit-line-clamp: 3;
			line-clamp: 3;
		}

		.watch-now-btn {
			margin-left: 20px;
			margin-right: 20px;
			padding: 8px 20px;
		}

		.watch-now-btn:hover {
			background: #0984ae;
			color: #fff;
		}

		.article-card__duration {
			bottom: 10px;
			right: 10px;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.article-card__title {
			font-size: 17px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.article-card,
		.watch-now-btn {
			transition: none;
		}
	}

	.article-card__title a:focus-visible,
	.watch-now-btn:focus-visible {
		outline: 2px solid #0984ae;
		outline-offset: 2px;
	}
</style>
