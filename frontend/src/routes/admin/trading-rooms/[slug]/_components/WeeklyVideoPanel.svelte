<!--
	WeeklyVideoPanel — current weekly video card + archived list.
	Extracted from +page.svelte (R8-C) — read-only display + 1 callback prop.
-->
<script lang="ts">
	import IconVideo from '@tabler/icons-svelte-runes/icons/video';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import type { WeeklyVideo } from '$lib/api/room-content';

	interface Props {
		isLoadingVideo: boolean;
		currentVideo: WeeklyVideo | null;
		archivedVideos: WeeklyVideo[];
		hasArchivedVideos: boolean;
		onAddVideo: () => void;
	}

	const { isLoadingVideo, currentVideo, archivedVideos, hasArchivedVideos, onAddVideo }: Props =
		$props();
</script>

{#if isLoadingVideo}
	<div class="loading">Loading video...</div>
{:else}
	<!-- Current Video -->
	{#if currentVideo}
		<div class="current-video-card">
			<div class="video-badge">CURRENT</div>
			<div class="video-content">
				{#if currentVideo.thumbnail_url}
					<div
						class="video-thumbnail"
						style:background-image={`url('${currentVideo.thumbnail_url}')`}
					>
						{#if currentVideo.duration}
							<span class="duration">{currentVideo.duration}</span>
						{/if}
					</div>
				{/if}
				<div class="video-info">
					<h3>{currentVideo.week_title}</h3>
					<h4>{currentVideo.video_title}</h4>
					<p class="video-url">{currentVideo.video_url}</p>
					<span class="video-date"
						>Published: {new Date(currentVideo.published_at).toLocaleDateString()}</span
					>
				</div>
			</div>
		</div>
	{:else}
		<div class="empty-state">
			<IconVideo size={48} />
			<h3>No Weekly Video</h3>
			<p>Publish your first weekly video for this room</p>
			<button class="btn-primary" onclick={onAddVideo}>
				<IconPlus size={18} />
				Publish Video
			</button>
		</div>
	{/if}

	<!-- Archived Videos -->
	{#if hasArchivedVideos}
		<div class="archived-section">
			<h3>Archive</h3>
			<div class="archived-list">
				{#each archivedVideos as video (video.id)}
					<div class="archived-video">
						<span class="archived-week">{video.week_title}</span>
						<span class="archived-title">{video.video_title}</span>
						<span class="archived-date">{new Date(video.published_at).toLocaleDateString()}</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}
{/if}

<style>
	/* Loading & Empty States */
	.loading {
		text-align: center;
		padding: 60px 20px;
		color: #64748b;
		font-size: 16px;
	}

	.empty-state {
		text-align: center;
		padding: 80px 20px;
		background: #f8fafc;
		border-radius: 16px;
		color: #64748b;
	}

	.empty-state :global(svg) {
		opacity: 0.4;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: #1e293b;
		margin: 0 0 8px 0;
	}

	.empty-state p {
		margin: 0 0 24px 0;
	}

	/* Buttons */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		background: #0f2d42;
	}

	/* Weekly Video */
	.current-video-card {
		background: #fff;
		border: 2px solid #143e59;
		border-radius: 16px;
		overflow: hidden;
		position: relative;
	}

	.video-badge {
		position: absolute;
		top: 16px;
		left: 16px;
		background: #143e59;
		color: #fff;
		padding: 6px 14px;
		border-radius: 6px;
		font-size: 11px;
		font-weight: 700;
		z-index: 1;
	}

	.video-content {
		display: flex;
		gap: 24px;
		padding: 24px;
	}

	.video-thumbnail {
		width: 320px;
		height: 180px;
		background-size: cover;
		background-position: center;
		background-color: #f1f5f9;
		border-radius: 12px;
		flex-shrink: 0;
		position: relative;
	}

	.video-thumbnail .duration {
		position: absolute;
		bottom: 8px;
		right: 8px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		padding: 4px 8px;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
	}

	.video-info {
		flex: 1;
	}

	.video-info h3 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 8px 0;
	}

	.video-info h4 {
		font-size: 16px;
		font-weight: 500;
		color: #475569;
		margin: 0 0 16px 0;
	}

	.video-url {
		font-size: 13px;
		color: #3b82f6;
		word-break: break-all;
		margin: 0 0 12px 0;
	}

	.video-date {
		font-size: 13px;
		color: #94a3b8;
	}

	.archived-section {
		margin-top: 32px;
	}

	.archived-section h3 {
		font-size: 16px;
		font-weight: 600;
		color: #64748b;
		margin: 0 0 16px 0;
	}

	.archived-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.archived-video {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 14px 18px;
		background: #f8fafc;
		border-radius: 10px;
	}

	.archived-week {
		font-weight: 600;
		color: #1e293b;
		min-width: 200px;
	}

	.archived-title {
		flex: 1;
		color: #64748b;
	}

	.archived-date {
		font-size: 13px;
		color: #94a3b8;
	}

	@media (max-width: 767.98px) {
		.video-content {
			flex-direction: column;
		}

		.video-thumbnail {
			width: 100%;
			height: 200px;
		}
	}
</style>
