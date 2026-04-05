<!--
  ResourceCard.svelte
  Apple Principal Engineer ICT 7 Grade - February 2026

  Displays a single resource card with:
  - Thumbnail/preview
  - Title and description
  - Resource type badge
  - Access level indicator (free/premium)
  - Download/view action button
  - Version indicator
-->
<script lang="ts">
	import type { RoomResource } from '$lib/api/room-resources';
	import { formatResourceForDisplay, trackDownload, trackAccess } from '$lib/api/room-resources';
	import FavoriteButton from './FavoriteButton.svelte';

	interface Props {
		resource: RoomResource;
		showAccessLevel?: boolean;
		showVersion?: boolean;
		showFavorite?: boolean;
		compact?: boolean;
		onClick?: (resource: RoomResource) => void;
		onDownload?: (resource: RoomResource) => void;
		onPreview?: (resource: RoomResource) => void;
	}

	let {
		resource,
		showAccessLevel = true,
		showVersion = false,
		showFavorite = true,
		compact = false,
		onClick,
		onDownload,
		onPreview
	}: Props = $props();

	let displayInfo = $derived(formatResourceForDisplay(resource));
	let isVideo = $derived(resource.resource_type === 'video');
	let formattedSize = $derived(resource.formatted_size || '');
	let hasVersion = $derived((resource.version ?? 1) > 1);

	async function handleDownload(event: MouseEvent) {
		event.stopPropagation();
		// Track download
		try {
			await trackDownload(resource.id);
		} catch (_e) {
			// Silent fail for tracking
		}
		onDownload?.(resource);
	}

	function handlePreview(event: MouseEvent) {
		event.stopPropagation();
		onPreview?.(resource);
	}

	async function handleCardClick() {
		// Track access for recently accessed feature
		try {
			await trackAccess(resource.id);
		} catch (_e) {
			// Silent fail for tracking
		}
		onClick?.(resource);
	}

	// Get appropriate icon for resource type
	function getTypeIcon(type: string): string {
		const icons: Record<string, string> = {
			video:
				'M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z',
			pdf: 'M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z',
			document:
				'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z',
			image:
				'M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z',
			spreadsheet:
				'M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2',
			archive: 'M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4',
			other:
				'M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z'
		};
		return icons[type] || icons.other;
	}

	function getAccessLevel(level: string): string {
		switch (level) {
			case 'free':
				return 'free';
			case 'member':
				return 'member';
			case 'premium':
				return 'premium';
			case 'vip':
				return 'vip';
			default:
				return 'premium';
		}
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<article class="rc-card" data-compact={compact || undefined} onclick={handleCardClick}>
	<!-- Thumbnail / Preview -->
	<div class="rc-thumb">
		{#if resource.thumbnail_url}
			<img src={resource.thumbnail_url} alt={resource.title} class="rc-thumb-img" loading="lazy" />
		{:else}
			<div class="rc-thumb-placeholder">
				<svg class="rc-icon-xl" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d={getTypeIcon(resource.resource_type)}
					/>
				</svg>
			</div>
		{/if}

		<div class="rc-thumb-overlay"></div>

		<span class="rc-badge rc-type-badge">{displayInfo.typeLabel}</span>

		{#if showAccessLevel}
			<span
				class="rc-badge rc-access-badge"
				data-level={getAccessLevel(resource.access_level ?? 'premium')}
			>
				{resource.access_level === 'free' ? 'Free' : 'Premium'}
			</span>
		{/if}

		{#if isVideo && resource.formatted_duration}
			<span class="rc-duration">{resource.formatted_duration}</span>
		{/if}

		{#if isVideo}
			<div class="rc-play-overlay">
				<button class="rc-play-btn" onclick={handlePreview} aria-label="Play video">
					<svg class="rc-play-icon" fill="currentColor" viewBox="0 0 24 24">
						<path d="M8 5v14l11-7z" />
					</svg>
				</button>
			</div>
		{/if}
	</div>

	<!-- Content -->
	<div class="rc-content">
		<h3 class="rc-title" title={resource.title}>{resource.title}</h3>

		{#if resource.description && !compact}
			<p class="rc-desc">{resource.description}</p>
		{/if}

		<div class="rc-meta">
			<span class="rc-meta-tag">{displayInfo.contentLabel}</span>

			{#if formattedSize}
				<span class="rc-meta-item">
					<svg class="rc-icon-xs" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"
						/>
					</svg>
					{formattedSize}
				</span>
			{/if}

			<span class="rc-meta-item">
				<svg class="rc-icon-xs" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
					/>
				</svg>
				{resource.formatted_date}
			</span>

			{#if showVersion && hasVersion}
				<span class="rc-meta-item rc-version">
					<svg class="rc-icon-xs" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
						/>
					</svg>
					v{resource.version}
				</span>
			{/if}
		</div>

		<div class="rc-stats">
			<div class="rc-stats-left">
				<span class="rc-meta-item">
					<svg class="rc-icon-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
						/>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
						/>
					</svg>
					{resource.views_count}
				</span>
				<span class="rc-meta-item">
					<svg class="rc-icon-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
						/>
					</svg>
					{resource.downloads_count}
				</span>
			</div>
			{#if showFavorite}
				<FavoriteButton resourceId={resource.id} size="sm" />
			{/if}
		</div>

		<button class="rc-action-btn" onclick={isVideo ? handlePreview : handleDownload}>
			{displayInfo.actionLabel}
		</button>
	</div>

	{#if resource.is_featured || resource.is_pinned}
		<div class="rc-ribbon">{resource.is_pinned ? 'Pinned' : 'Featured'}</div>
	{/if}
</article>

<style>
	.rc-card {
		position: relative;
		overflow: hidden;
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.9 0.005 265);
		background-color: oklch(1 0 0);
		cursor: pointer;
		transition: all 300ms var(--ease-default);

		&:hover {
			box-shadow: 0 10px 25px oklch(0 0 0 / 10%);
			border-color: oklch(0.75 0.12 260);
		}

		&:hover .rc-thumb-img {
			transform: scale(1.05);
		}
		&:hover .rc-thumb-overlay {
			opacity: 1;
		}
		&:hover .rc-play-overlay {
			opacity: 1;
		}
	}

	/* ─── Thumbnail ─── */
	.rc-thumb {
		position: relative;
		aspect-ratio: 16 / 9;
		overflow: hidden;
		background-color: oklch(0.96 0.002 265);
	}

	.rc-thumb-img {
		inline-size: 100%;
		block-size: 100%;
		object-fit: cover;
		transition: transform 300ms var(--ease-default);
	}

	.rc-thumb-placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		inline-size: 100%;
		block-size: 100%;
	}

	.rc-thumb-overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(to top, oklch(0 0 0 / 60%), transparent, transparent);
		opacity: 0;
		transition: opacity 300ms var(--ease-default);
	}

	/* ─── Badges ─── */
	.rc-badge {
		position: absolute;
		border-radius: var(--radius-md);
		padding-inline: var(--space-2);
		padding-block: 0.25rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
	}

	.rc-type-badge {
		inset-block-start: var(--space-2);
		inset-inline-start: var(--space-2);
		background-color: oklch(0 0 0 / 70%);
		color: oklch(1 0 0);
		backdrop-filter: blur(4px);
	}

	.rc-access-badge {
		inset-block-start: var(--space-2);
		inset-inline-end: var(--space-2);
		color: oklch(1 0 0);

		&[data-level='free'] {
			background-color: oklch(0.6 0.18 160);
		}
		&[data-level='member'] {
			background-color: oklch(0.6 0.2 260);
		}
		&[data-level='premium'] {
			background-color: oklch(0.75 0.18 80);
		}
		&[data-level='vip'] {
			background-color: oklch(0.55 0.2 300);
		}
	}

	.rc-duration {
		position: absolute;
		inset-block-end: var(--space-2);
		inset-inline-end: var(--space-2);
		border-radius: var(--radius-sm);
		background-color: oklch(0 0 0 / 80%);
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
	}

	/* ─── Play overlay ─── */
	.rc-play-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		opacity: 0;
		transition: opacity 300ms var(--ease-default);
	}

	.rc-play-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		inline-size: 4rem;
		block-size: 4rem;
		border-radius: 9999px;
		background-color: oklch(0.55 0.2 260);
		color: oklch(1 0 0);
		border: none;
		cursor: pointer;
		box-shadow: 0 4px 12px oklch(0 0 0 / 20%);
		transition: transform 200ms var(--ease-default);

		&:hover {
			transform: scale(1.1);
		}
	}

	.rc-play-icon {
		inline-size: 2rem;
		block-size: 2rem;
		padding-inline-start: 0.25rem;
	}

	/* ─── Content ─── */
	.rc-content {
		padding: var(--space-4);
	}

	.rc-title {
		margin-block-end: 0.25rem;
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.rc-desc {
		margin-block-end: var(--space-3);
		font-size: var(--text-sm);
		color: oklch(0.45 0.01 265);
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* ─── Meta ─── */
	.rc-meta {
		margin-block-end: var(--space-3);
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	.rc-meta-tag {
		border-radius: var(--radius-sm);
		background-color: oklch(0.95 0.002 265);
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
	}

	.rc-meta-item {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}
	.rc-version {
		color: oklch(0.5 0.2 260);
	}

	.rc-icon-xs {
		inline-size: 0.75rem;
		block-size: 0.75rem;
	}
	.rc-icon-sm {
		inline-size: 1rem;
		block-size: 1rem;
	}
	.rc-icon-xl {
		inline-size: 4rem;
		block-size: 4rem;
		color: oklch(0.65 0.01 265);
	}

	/* ─── Stats ─── */
	.rc-stats {
		margin-block-end: var(--space-3);
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	.rc-stats-left {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	/* ─── Action button ─── */
	.rc-action-btn {
		inline-size: 100%;
		border-radius: var(--radius-lg);
		background-color: oklch(0.55 0.2 260);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		border: none;
		cursor: pointer;
		transition: background-color 200ms var(--ease-default);

		&:hover {
			background-color: oklch(0.48 0.2 260);
		}
		&:focus-visible {
			outline: 2px solid oklch(0.6 0.2 260);
			outline-offset: 2px;
		}
	}

	/* ─── Ribbon ─── */
	.rc-ribbon {
		position: absolute;
		inset-inline-end: -2rem;
		inset-block-start: 0.75rem;
		transform: rotate(45deg);
		background-color: oklch(0.55 0.2 260);
		padding-inline: 2rem;
		padding-block: 0.125rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		box-shadow: 0 1px 3px oklch(0 0 0 / 10%);
	}

	/* ─── Compact variant ─── */
	.rc-card[data-compact] .rc-content {
		padding: var(--space-3);
	}
	.rc-card[data-compact] .rc-title {
		font-size: var(--text-sm);
	}
	.rc-card[data-compact] .rc-thumb {
		aspect-ratio: 16 / 10;
	}
</style>
