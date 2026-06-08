<script lang="ts">
	interface IndicatorVideo {
		id: number;
		indicator_id: number;
		title: string;
		description?: string;
		video_url?: string;
		embed_url?: string;
		thumbnail_url?: string;
		duration_seconds?: number;
		display_order?: number;
		is_active?: boolean;
		created_at?: string;
	}

	interface Props {
		videos: IndicatorVideo[];
		loading: boolean;
		onAddClick: () => void;
		onDelete: (videoId: number) => void;
	}

	let { videos, loading, onAddClick, onDelete }: Props = $props();

	const formatDuration = (seconds?: number): string => {
		if (!seconds) return '';
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	};
</script>

<div class="form-section">
	<div class="section-header">
		<h2>Tutorial Videos</h2>
		<button class="btn-primary" onclick={onAddClick}>Add Video</button>
	</div>

	{#if loading}
		<div class="loading-inline">
			<div class="spinner-small"></div>
			<span>Loading videos...</span>
		</div>
	{:else if videos.length === 0}
		<div class="empty-state">
			<p>No videos added yet</p>
			<p class="hint">Add tutorial videos to help users learn the indicator</p>
		</div>
	{:else}
		<div class="videos-grid">
			{#each videos as video (video.id)}
				<div class="video-card">
					{#if video.thumbnail_url}
						<img
							src={video.thumbnail_url}
							alt={video.title}
							class="thumbnail"
							width="320"
							height="180"
							loading="lazy"
						/>
					{:else}
						<div class="thumbnail-placeholder">No Thumbnail</div>
					{/if}
					<div class="video-info">
						<h3>{video.title}</h3>
						<div class="video-meta">
							{#if video.duration_seconds}
								<span class="tag">{formatDuration(video.duration_seconds)}</span>
							{/if}
							<span class={['tag', { active: video.is_active }]}>
								{video.is_active !== false ? 'Active' : 'Inactive'}
							</span>
						</div>
					</div>
					<button
						class="btn-icon btn-danger"
						onclick={() => onDelete(video.id)}
						title="Delete video">X</button
					>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.form-section {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 24px;
		margin-bottom: 24px;
	}
	.form-section h2 {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}
	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		background: #143e59;
		color: #fff;
	}
	.btn-primary:hover {
		background: #0f2d42;
	}

	.spinner-small {
		width: 20px;
		height: 20px;
		border: 2px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	.loading-inline {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 24px;
		color: #6b7280;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state {
		text-align: center;
		padding: 40px;
		color: #6b7280;
	}
	.hint {
		font-size: 13px;
		color: #9ca3af;
	}

	.videos-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 16px;
	}
	.video-card {
		background: #f9fafb;
		border-radius: 8px;
		overflow: hidden;
		position: relative;
	}
	.thumbnail,
	.thumbnail-placeholder {
		width: 100%;
		aspect-ratio: 16/9;
		object-fit: cover;
		background: #e5e7eb;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #9ca3af;
	}
	.video-info {
		padding: 12px;
	}
	.video-info h3 {
		font-size: 14px;
		font-weight: 500;
		margin: 0 0 8px;
	}
	.video-meta {
		display: flex;
		gap: 6px;
	}
	.tag {
		font-size: 11px;
		padding: 2px 8px;
		background: #143e59;
		color: #fff;
		border-radius: 4px;
	}
	.tag.active {
		background: #10b981;
	}
	.video-card .btn-icon {
		position: absolute;
		top: 8px;
		right: 8px;
		background: rgba(0, 0, 0, 0.5);
		color: #fff;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: #f3f4f6;
		border-radius: 6px;
		color: #6b7280;
		cursor: pointer;
		min-height: 44px;
		min-width: 44px;
		font-weight: bold;
	}
	.btn-danger:hover {
		background: #fee2e2;
		color: #dc2626;
	}

	@media (max-width: 639px) {
		.form-section {
			padding: 16px;
		}

		.videos-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.spinner-small {
			animation: none;
		}
	}
</style>
