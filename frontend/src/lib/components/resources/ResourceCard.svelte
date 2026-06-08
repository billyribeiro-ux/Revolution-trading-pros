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
	import Icon from '$lib/components/Icon.svelte';

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

<!--
	`<article role=button>` triggers a11y_no_noninteractive_element_to_interactive_role
	— the only specific suppression we still need. Adding tabindex + onkeydown
	(Enter/Space) makes the card keyboard-operable, removing the two previously
	suppressed warnings (a11y_no_noninteractive_element_interactions and
	a11y_click_events_have_key_events). Net: 2 suppressions → 1.
-->
<!-- svelte-ignore a11y_no_noninteractive_element_to_interactive_role -->
<article
	class={[
		'resource-card group relative overflow-hidden rounded-xl border border-gray-200 bg-white transition-all duration-300 hover:border-blue-300 hover:shadow-lg dark:border-gray-700 dark:bg-gray-800 dark:hover:border-blue-600 cursor-pointer',
		{ compact }
	]}
	role="button"
	tabindex="0"
	onclick={handleCardClick}
	onkeydown={(e) => {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			handleCardClick();
		}
	}}
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
				width="400"
				height="225"
			/>
		{:else}
			<div class="flex h-full w-full items-center justify-center">
				{#if resource.resource_type === 'video'}
					<Icon name="IconVideo" size={64} class="text-gray-400 dark:text-gray-500" />
				{:else if resource.resource_type === 'pdf'}
					<Icon name="IconPdf" size={64} class="text-gray-400 dark:text-gray-500" />
				{:else if resource.resource_type === 'image'}
					<Icon name="IconPhoto" size={64} class="text-gray-400 dark:text-gray-500" />
				{:else if resource.resource_type === 'spreadsheet'}
					<Icon name="IconFileSpreadsheet" size={64} class="text-gray-400 dark:text-gray-500" />
				{:else}
					<Icon name="IconFileDescription" size={64} class="text-gray-400 dark:text-gray-500" />
				{/if}
			</div>
		{/if}

		<!-- Overlay badges -->
		<div
			class="absolute inset-0 bg-linear-to-t from-black/60 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100"
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
				class={[
					'absolute right-2 top-2 rounded-md px-2 py-1 text-xs font-medium',
					getAccessLevelColor(resource.access_level ?? 'premium')
				]}
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
					<Icon name="IconPlayerPlay" size={32} />
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
					<Icon name="IconDatabase" size={12} />
					{formattedSize}
				</span>
			{/if}

			<!-- Date -->
			<span class="flex items-center gap-1">
				<Icon name="IconCalendar" size={12} />
				{resource.formatted_date}
			</span>

			<!-- Version -->
			{#if showVersion && hasVersion}
				<span class="flex items-center gap-1 text-blue-600 dark:text-blue-400">
					<Icon name="IconRefresh" size={12} />
					v{resource.version}
				</span>
			{/if}
		</div>

		<!-- Stats -->
		<div class="mb-3 flex items-center justify-between text-xs text-gray-500 dark:text-gray-400">
			<div class="flex items-center gap-4">
				<span class="flex items-center gap-1">
					<Icon name="IconEye" size={16} />
					{resource.views_count}
				</span>
				<span class="flex items-center gap-1">
					<Icon name="IconDownload" size={16} />
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
