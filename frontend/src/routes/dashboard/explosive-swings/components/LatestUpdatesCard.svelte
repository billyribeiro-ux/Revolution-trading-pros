<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * LatestUpdatesCard Component - Compact Video Updates Grid
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Compact 3-item grid of recent video updates for sidebar
	 * @version 4.1.0 - Visual Polish Pass
	 * @requires Svelte 5.0+ (January 2026 syntax)
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 *
	 * DESIGN DECISION: Max 3 items to prevent sidebar bloat
	 * Per ICT 7: "Density should serve function, not overwhelm"
	 */
	import type { Video } from '../types';

	interface Props {
		updates: Video[];
		isLoading?: boolean;
		maxItems?: number;
	}

	const { updates, isLoading = false, maxItems = 3 }: Props = $props();

	const displayUpdates = $derived(updates.slice(0, maxItems));
</script>

<div class="updates-card" role="region" aria-labelledby="updates-heading">
	<div class="card-header">
		<h3 id="updates-heading" class="card-title">
			<svg viewBox="0 0 20 20" fill="currentColor" class="title-icon" aria-hidden="true">
				<path d="M4.75 3A1.75 1.75 0 003 4.75v2.5c0 .966.784 1.75 1.75 1.75h2.5A1.75 1.75 0 009 7.25v-2.5A1.75 1.75 0 007.25 3h-2.5zM4.75 11A1.75 1.75 0 003 12.75v2.5c0 .966.784 1.75 1.75 1.75h2.5A1.75 1.75 0 009 15.25v-2.5A1.75 1.75 0 007.25 11h-2.5zM11 4.75c0-.966.784-1.75 1.75-1.75h2.5c.966 0 1.75.784 1.75 1.75v2.5A1.75 1.75 0 0115.25 9h-2.5A1.75 1.75 0 0111 7.25v-2.5zM12.75 11a1.75 1.75 0 00-1.75 1.75v2.5c0 .966.784 1.75 1.75 1.75h2.5A1.75 1.75 0 0017 15.25v-2.5A1.75 1.75 0 0015.25 11h-2.5z" />
			</svg>
			Latest Updates
		</h3>
		<a href="/dashboard/explosive-swings/video-library" class="view-all-link">
			View All
		</a>
	</div>

	{#if isLoading}
		<div class="updates-grid">
			{#each [1, 2, 3] as _}
				<div class="update-skeleton">
					<div class="skeleton-thumb"></div>
					<div class="skeleton-title"></div>
				</div>
			{/each}
		</div>
	{:else if displayUpdates.length === 0}
		<div class="empty-state">
			<p>No recent updates</p>
		</div>
	{:else}
		<div class="updates-grid">
			{#each displayUpdates as update (update.id)}
				<a 
					href={update.videoUrl} 
					class="update-item" 
					target="_blank" 
					rel="noopener noreferrer"
					aria-label="{update.title} - {update.duration}"
				>
					<div 
						class="update-thumb" 
						class:no-thumb={!update.thumbnailUrl}
						style={update.thumbnailUrl ? `background-image: url('${update.thumbnailUrl}')` : ''}
					>
						<div class="play-icon" aria-hidden="true">
							<svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
								<path d="M8 5v14l11-7z" />
							</svg>
						</div>
						<span class="duration-badge">{update.duration}</span>
					</div>
					<span class="update-title">{update.title}</span>
				</a>
			{/each}
		</div>
	{/if}
</div>

<style>
	.updates-card {
		background: #ffffff;
		border: 1px solid #e2e8f0;
		border-radius: 14px;
		padding: 18px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04), 0 1px 2px rgba(0, 0, 0, 0.06);
	}

	.card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 14px;
	}

	.card-title {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		font-weight: 700;
		color: #475569;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin: 0;
	}

	.title-icon {
		width: 16px;
		height: 16px;
	}

	.view-all-link {
		font-size: 12px;
		font-weight: 600;
		color: #143e59;
		text-decoration: none;
		transition: color 0.2s;
	}

	.view-all-link:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.view-all-link:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
		border-radius: 2px;
	}

	.updates-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 10px;
	}

	.update-item {
		display: flex;
		flex-direction: column;
		gap: 8px;
		text-decoration: none;
		transition: transform 0.2s ease-out;
	}

	.update-item:hover {
		transform: translateY(-2px);
	}

	.update-item:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 4px;
		border-radius: 8px;
	}

	.update-thumb {
		position: relative;
		aspect-ratio: 16 / 9;
		background-size: cover;
		background-position: center;
		background-color: #1e293b;
		border-radius: 8px;
		overflow: hidden;
	}

	.update-thumb.no-thumb {
		background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.update-thumb.no-thumb::before {
		content: '';
		width: 24px;
		height: 24px;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke='%2364748b' stroke-width='1.5'%3E%3Cpath stroke-linecap='round' stroke-linejoin='round' d='m15.75 10.5 4.72-4.72a.75.75 0 0 1 1.28.53v11.38a.75.75 0 0 1-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 0 0 2.25-2.25v-9a2.25 2.25 0 0 0-2.25-2.25h-9A2.25 2.25 0 0 0 2.25 7.5v9a2.25 2.25 0 0 0 2.25 2.25Z' /%3E%3C/svg%3E");
		background-size: contain;
		background-repeat: no-repeat;
		opacity: 0.5;
	}

	.play-icon {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 28px;
		height: 28px;
		background: rgba(255, 255, 255, 0.9);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.play-icon svg {
		color: #143e59;
		margin-left: 2px;
	}

	.update-item:hover .play-icon,
	.update-item:focus-visible .play-icon {
		opacity: 1;
	}

	.duration-badge {
		position: absolute;
		bottom: 4px;
		right: 4px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		font-size: 10px;
		font-weight: 600;
		padding: 2px 5px;
		border-radius: 3px;
		font-variant-numeric: tabular-nums;
	}

	.update-title {
		font-size: 11px;
		font-weight: 500;
		color: #334155;
		line-height: 1.3;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.update-item:hover .update-title {
		color: #143e59;
	}

	.empty-state {
		text-align: center;
		padding: 20px;
		color: #94a3b8;
		font-size: 13px;
	}

	/* Skeleton Loading */
	.update-skeleton {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.skeleton-thumb {
		aspect-ratio: 16 / 9;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	.skeleton-title {
		height: 12px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	/* Responsive: Stack on smaller screens */
	@media (max-width: 640px) {
		.updates-grid {
			grid-template-columns: repeat(3, 1fr);
			gap: 8px;
		}

		.update-title {
			font-size: 10px;
		}

		.duration-badge {
			font-size: 9px;
			padding: 1px 4px;
		}
	}
</style>
