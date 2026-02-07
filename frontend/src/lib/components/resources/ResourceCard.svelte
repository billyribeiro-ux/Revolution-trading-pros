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

	// Get access level badge color
	function getAccessLevelColor(level: string): string {
		const colors: Record<string, string> = {
			free: 'bg-emerald-500 text-white',
			member: 'bg-blue-500 text-white',
			premium: 'bg-amber-500 text-white',
			vip: 'bg-purple-500 text-white'
		};
		return colors[level] || colors.premium;
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<article
	class="resource-card group relative overflow-hidden rounded-xl border border-gray-200 bg-white transition-all duration-300 hover:shadow-lg hover:border-blue-300 dark:border-gray-700 dark:bg-gray-800 dark:hover:border-blue-600 cursor-pointer"
	class:compact
	onclick={handleCardClick}
>
	<!-- Thumbnail / Preview -->
	<div
		class="thumbnail-container relative aspect-video overflow-hidden bg-gray-100 dark:bg-gray-700"
	>
		{#if resource.thumbnail_url}
			<img
				src={resource.thumbnail_url}
				alt={resource.title}
				class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
				loading="lazy"
			/>
		{:else}
			<div class="flex h-full w-full items-center justify-center">
				<svg
					class="h-16 w-16 text-gray-400 dark:text-gray-500"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d={getTypeIcon(resource.resource_type)}
					/>
				</svg>
			</div>
		{/if}

		<!-- Overlay badges -->
		<div
			class="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100"
		></div>

		<!-- Type badge -->
		<span
			class="absolute left-2 top-2 rounded-md bg-black/70 px-2 py-1 text-xs font-medium text-white backdrop-blur-sm"
		>
			{displayInfo.typeLabel}
		</span>

		<!-- Access level badge -->
		{#if showAccessLevel}
			<span
				class="absolute right-2 top-2 rounded-md px-2 py-1 text-xs font-medium {getAccessLevelColor(
					resource.access_level ?? 'premium'
				)}"
			>
				{resource.access_level === 'free' ? 'Free' : 'Premium'}
			</span>
		{/if}

		<!-- Duration for videos -->
		{#if isVideo && resource.formatted_duration}
			<span
				class="absolute bottom-2 right-2 rounded bg-black/80 px-2 py-0.5 text-xs font-medium text-white"
			>
				{resource.formatted_duration}
			</span>
		{/if}

		<!-- Play button overlay for videos -->
		{#if isVideo}
			<div
				class="absolute inset-0 flex items-center justify-center opacity-0 transition-opacity duration-300 group-hover:opacity-100"
			>
				<button
					class="flex h-16 w-16 items-center justify-center rounded-full bg-blue-600 text-white shadow-lg transition-transform hover:scale-110"
					onclick={handlePreview}
					aria-label="Play video"
				>
					<svg class="h-8 w-8 pl-1" fill="currentColor" viewBox="0 0 24 24">
						<path d="M8 5v14l11-7z" />
					</svg>
				</button>
			</div>
		{/if}
	</div>

	<!-- Content -->
	<div class="content p-4">
		<!-- Title -->
		<h3
			class="mb-1 line-clamp-2 font-semibold text-gray-900 dark:text-white"
			title={resource.title}
		>
			{resource.title}
		</h3>

		<!-- Description -->
		{#if resource.description && !compact}
			<p class="mb-3 line-clamp-2 text-sm text-gray-600 dark:text-gray-400">
				{resource.description}
			</p>
		{/if}

		<!-- Meta info -->
		<div class="mb-3 flex flex-wrap items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
			<!-- Content type -->
			<span class="rounded bg-gray-100 px-2 py-0.5 dark:bg-gray-700">
				{displayInfo.contentLabel}
			</span>

			<!-- File size -->
			{#if formattedSize}
				<span class="flex items-center gap-1">
					<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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

			<!-- Date -->
			<span class="flex items-center gap-1">
				<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
					/>
				</svg>
				{resource.formatted_date}
			</span>

			<!-- Version -->
			{#if showVersion && hasVersion}
				<span class="flex items-center gap-1 text-blue-600 dark:text-blue-400">
					<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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

		<!-- Stats -->
		<div class="mb-3 flex items-center justify-between text-xs text-gray-500 dark:text-gray-400">
			<div class="flex items-center gap-4">
				<span class="flex items-center gap-1">
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
				<span class="flex items-center gap-1">
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
			<!-- Favorite button -->
			{#if showFavorite}
				<FavoriteButton resourceId={resource.id} size="sm" />
			{/if}
		</div>

		<!-- Action button -->
		<button
			class="w-full rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
			onclick={isVideo ? handlePreview : handleDownload}
		>
			{displayInfo.actionLabel}
		</button>
	</div>

	<!-- Featured/Pinned indicator -->
	{#if resource.is_featured || resource.is_pinned}
		<div
			class="absolute -right-8 top-3 rotate-45 bg-blue-600 px-8 py-0.5 text-xs font-medium text-white shadow-sm"
		>
			{resource.is_pinned ? 'Pinned' : 'Featured'}
		</div>
	{/if}
</article>

<style>
	.resource-card.compact .content {
		padding: 0.75rem;
	}

	.resource-card.compact h3 {
		font-size: 0.875rem;
	}

	.resource-card.compact .thumbnail-container {
		aspect-ratio: 16 / 10;
	}
</style>
