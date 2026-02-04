<script lang="ts">
	/**
	 * Video Detail Page - Explosive Swings
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Dynamic video player page for weekly breakdowns and trade updates
	 *
	 * @version 2.0.0 - ICT 11 Principal Engineer Grade
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 */
	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import BunnyVideoPlayer from '$lib/components/video/BunnyVideoPlayer.svelte';
	import FavoriteButton from '$lib/components/dashboard/FavoriteButton.svelte';

	const ROOM_SLUG = 'explosive-swings';

	interface VideoData {
		id: number;
		title: string;
		description: string;
		video_url: string;
		video_platform: string;
		bunny_video_guid?: string;
		bunny_library_id?: string;
		thumbnail_url: string;
		duration: string;
		published_at: string;
		week_of?: string;
	}

	let video = $state<VideoData | null>(null);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	const slug = $derived($page.params.slug);

	async function fetchVideo() {
		isLoading = true;
		error = null;

		try {
			// Try to fetch from weekly video endpoint first
			if (slug === 'weekly' || slug?.startsWith('weekly-')) {
				const res = await fetch(`/api/weekly-video/${ROOM_SLUG}`);
				if (res.ok) {
					const data = await res.json();
					if (data.success && data.data) {
						video = {
							id: data.data.id,
							title: data.data.video_title || data.data.week_title,
							description: data.data.description || '',
							video_url: data.data.video_url,
							video_platform: data.data.video_platform || 'bunny',
							bunny_video_guid: data.data.bunny_video_guid,
							bunny_library_id: data.data.bunny_library_id,
							thumbnail_url: data.data.thumbnail_url || '',
							duration: data.data.duration || '',
							published_at: data.data.published_at,
							week_of: data.data.week_of
						};
						return;
					}
				}
			}

			// ICT 7 Standards: No fallback mock data
			// If API returns no data, show proper error state in UI
			video = null;
			error = 'No video found for this week';
		} catch (err) {
			console.error('Failed to fetch video:', err);
			error = 'Failed to load video';
		} finally {
			isLoading = false;
		}
	}

	function formatDate(dateStr: string): string {
		try {
			return new Date(dateStr).toLocaleDateString('en-US', {
				weekday: 'long',
				month: 'long',
				day: 'numeric',
				year: 'numeric'
			});
		} catch {
			return dateStr;
		}
	}

	$effect(() => {
		if (!browser) return;
		fetchVideo();
	});
</script>

<svelte:head>
	<title>{video?.title || 'Video'} | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
	showTradingRoomControls={false}
/>

<div class="video-page">
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading video...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<a href="/dashboard/explosive-swings" class="back-btn">Back to Dashboard</a>
		</div>
	{:else if video}
		<div class="video-container">
			{#if video.video_platform === 'bunny' && video.bunny_video_guid}
				<BunnyVideoPlayer
					videoGuid={video.bunny_video_guid}
					libraryId={video.bunny_library_id || ''}
					title={video.title}
					thumbnailUrl={video.thumbnail_url}
					controls={true}
				/>
			{:else}
				<div class="video-embed">
					<iframe
						src={video.video_url}
						title={video.title}
						frameborder="0"
						allow="autoplay; fullscreen; picture-in-picture"
						allowfullscreen
					></iframe>
				</div>
			{/if}
		</div>

		<div class="video-info">
			<div class="video-header">
				<div class="video-meta">
					<span class="video-date">{formatDate(video.published_at)}</span>
					{#if video.duration}
						<span class="video-duration">{video.duration}</span>
					{/if}
				</div>
				<FavoriteButton
					roomSlug={ROOM_SLUG}
					itemType="video"
					itemId={video.id}
					title={video.title}
					excerpt={video.description}
					href={`/dashboard/explosive-swings/video/${slug}`}
					thumbnailUrl={video.thumbnail_url}
					showLabel={true}
				/>
			</div>

			<h1>{video.title}</h1>

			{#if video.description}
				<p class="video-description">{video.description}</p>
			{/if}

			<div class="video-actions">
				<a href="/dashboard/explosive-swings/video-library" class="action-btn secondary">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<rect width="7" height="7" x="3" y="3" rx="1" />
						<rect width="7" height="7" x="14" y="3" rx="1" />
						<rect width="7" height="7" x="14" y="14" rx="1" />
						<rect width="7" height="7" x="3" y="14" rx="1" />
					</svg>
					Video Library
				</a>
				<a href="/dashboard/explosive-swings" class="action-btn primary">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="m15 18-6-6 6-6" />
					</svg>
					Back to Dashboard
				</a>
			</div>
		</div>
	{/if}
</div>

<style>
	.video-page {
		background: #f5f7fa;
		padding: 40px 20px;
	}

	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px 20px;
		text-align: center;
	}

	.spinner {
		width: 48px;
		height: 48px;
		border: 4px solid #e5e7eb;
		border-top-color: #f69532;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-state p,
	.error-state p {
		margin-top: 16px;
		color: #666;
	}

	.back-btn {
		display: inline-block;
		margin-top: 20px;
		padding: 12px 24px;
		background: #143e59;
		color: white;
		text-decoration: none;
		border-radius: 8px;
		font-weight: 600;
	}

	.video-container {
		max-width: 1200px;
		margin: 0 auto 30px;
	}

	.video-embed {
		position: relative;
		padding-bottom: 56.25%;
		height: 0;
		overflow: hidden;
		border-radius: 12px;
		box-shadow: 0 8px 40px rgba(0, 0, 0, 0.15);
	}

	.video-embed iframe {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}

	.video-info {
		max-width: 900px;
		margin: 0 auto;
		background: white;
		border-radius: 16px;
		padding: 30px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
	}

	.video-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
	}

	.video-meta {
		display: flex;
		gap: 16px;
		align-items: center;
	}

	.video-date {
		font-size: 14px;
		color: #666;
	}

	.video-duration {
		background: #f3f4f6;
		padding: 4px 10px;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		color: #333;
	}

	h1 {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0 0 16px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.video-description {
		font-size: 16px;
		color: #666;
		line-height: 1.7;
		margin: 0 0 30px 0;
	}

	.video-actions {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s;
	}

	.action-btn.primary {
		background: #143e59;
		color: white;
	}

	.action-btn.primary:hover {
		background: #0f2d42;
	}

	.action-btn.secondary {
		background: #f3f4f6;
		color: #333;
	}

	.action-btn.secondary:hover {
		background: #e5e7eb;
	}

	@media (max-width: 640px) {
		.video-info {
			padding: 20px;
		}

		h1 {
			font-size: 22px;
		}

		.video-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 12px;
		}

		.video-actions {
			flex-direction: column;
		}

		.action-btn {
			justify-content: center;
		}
	}
</style>
