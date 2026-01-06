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

	const defaultThumbnail = 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg';
</script>

<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
	<article class="article-card">
		<figure 
			class="article-card__image" 
			style="background-image: url({video.thumbnail_url || defaultThumbnail});"
		>
			<img 
				src={defaultThumbnail}
				alt={video.title}
				loading="lazy"
			/>
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
		
		<a href="{basePath}/{video.slug}" class="btn btn-tiny btn-default watch-now-btn">
			Watch Now
		</a>
	</article>
</div>

<style>
	.flex-grid-item {
		padding: 0 15px 30px;
		display: flex;
	}

	.article-card {
		display: flex;
		flex-direction: column;
		width: 100%;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 5px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
		padding: 0 0 20px;
	}

	.article-card:hover {
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
	}

	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		margin: 0;
		overflow: hidden;
	}

	.article-card__image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.article-card__duration {
		position: absolute;
		bottom: 10px;
		right: 10px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		padding: 4px 8px;
		border-radius: 3px;
		font-size: 12px;
		font-weight: 600;
	}

	.article-card__type {
		padding: 15px 20px 10px;
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		border-radius: 3px;
	}

	.label--info {
		background: #e8f4fc;
		color: #0984ae;
	}

	.article-card__title {
		padding: 0 20px;
		margin: 0 0 10px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.u--margin-top-0 {
		margin-top: 0;
		padding: 0 20px;
	}

	.trader_name {
		display: block;
		font-size: 13px;
		color: #666;
		margin-bottom: 8px;
	}

	.video-date {
		display: block;
		font-size: 12px;
		color: #999;
		margin-top: 4px;
	}

	.article-card__excerpt {
		padding: 0 20px;
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		flex-grow: 1;
	}

	.article-card__excerpt p {
		margin: 0 0 15px;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.watch-now-btn {
		margin: auto 20px 0;
		display: inline-block;
		padding: 8px 20px;
		background: #f5f5f5;
		color: #333;
		font-size: 12px;
		font-weight: 600;
		text-transform: uppercase;
		text-decoration: none;
		border-radius: 3px;
		transition: all 0.2s ease;
		text-align: center;
		width: fit-content;
	}

	.watch-now-btn:hover {
		background: #0984ae;
		color: #fff;
	}
</style>
