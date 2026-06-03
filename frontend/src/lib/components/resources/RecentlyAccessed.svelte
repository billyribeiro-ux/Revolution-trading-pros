<!--
  RecentlyAccessed.svelte
  Apple Principal Engineer ICT 7 Grade - February 2026

  Display user's recently accessed resources:
  - Horizontal scrolling carousel
  - Thumbnail previews
  - Quick access links
  - Empty state handling
-->
<script lang="ts">
	import type { RecentlyAccessed } from '$lib/api/room-resources';
	import { getRecentlyAccessed } from '$lib/api/room-resources';
	import Icon from '$lib/components/Icon.svelte';

	interface Props {
		limit?: number;
		showTitle?: boolean;
		compact?: boolean;
		initialData?: RecentlyAccessed[];
		onSelect?: (item: RecentlyAccessed) => void;
	}

	let {
		limit = 10,
		showTitle = true,
		compact = false,
		initialData = undefined,
		onSelect
	}: Props = $props();

	let items = $state<RecentlyAccessed[]>([]);
	let loading = $state(true);
	let error = $state('');

	$effect(() => {
		if (initialData) {
			items = initialData;
			loading = false;
		} else {
			loadRecentlyAccessed();
		}
	});

	async function loadRecentlyAccessed() {
		loading = true;
		error = '';

		try {
			const response = await getRecentlyAccessed(limit);
			items = response.data;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load recently accessed';
		} finally {
			loading = false;
		}
	}

	function handleItemClick(item: RecentlyAccessed) {
		onSelect?.(item);
	}

	function formatTimeAgo(dateString: string): string {
		const date = new Date(dateString);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffMins = Math.floor(diffMs / 60000);
		const diffHours = Math.floor(diffMs / 3600000);
		const diffDays = Math.floor(diffMs / 86400000);

		if (diffMins < 1) return 'Just now';
		if (diffMins < 60) return `${diffMins}m ago`;
		if (diffHours < 24) return `${diffHours}h ago`;
		if (diffDays < 7) return `${diffDays}d ago`;
		return date.toLocaleDateString();
	}
</script>

<div class="recently-accessed">
	{#if showTitle}
		<div class="mb-4 flex items-center justify-between">
			<h3 class="text-lg font-semibold text-gray-900 dark:text-white">Recently Accessed</h3>
			{#if items.length > 0}
				<button
					class="text-sm text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
					onclick={loadRecentlyAccessed}
				>
					Refresh
				</button>
			{/if}
		</div>
	{/if}

	{#if loading}
		<div class="flex gap-4 overflow-x-auto pb-2">
			{#each Array(4) as _, i (i)}
				<div class="w-40 flex-shrink-0 animate-pulse {compact ? '' : 'w-48'}">
					<div class="aspect-video rounded-lg bg-gray-200 dark:bg-gray-700"></div>
					<div class="mt-2 h-4 w-3/4 rounded bg-gray-200 dark:bg-gray-700"></div>
					<div class="mt-1 h-3 w-1/2 rounded bg-gray-200 dark:bg-gray-700"></div>
				</div>
			{/each}
		</div>
	{:else if error}
		<div
			class="rounded-lg border border-red-200 bg-red-50 p-4 text-center dark:border-red-800 dark:bg-red-900/20"
		>
			<p class="text-sm text-red-600 dark:text-red-400">{error}</p>
			<button
				class="mt-2 text-sm text-red-700 underline hover:no-underline dark:text-red-300"
				onclick={loadRecentlyAccessed}
			>
				Try again
			</button>
		</div>
	{:else if items.length === 0}
		<div
			class="rounded-lg border border-gray-200 bg-gray-50 p-6 text-center dark:border-gray-700 dark:bg-gray-800"
		>
			<Icon name="IconClock" size={40} class="mx-auto text-gray-400" />
			<p class="mt-2 text-sm text-gray-500 dark:text-gray-400">No recently accessed resources</p>
			<p class="text-xs text-gray-400 dark:text-gray-500">Resources you view will appear here</p>
		</div>
	{:else}
		<div
			class="flex gap-4 overflow-x-auto pb-2 scrollbar-thin scrollbar-thumb-gray-300 scrollbar-track-transparent dark:scrollbar-thumb-gray-600"
		>
			{#each items as item (item.id)}
				<button
					class="group w-40 flex-shrink-0 text-left transition-transform hover:scale-[1.02] {compact
						? ''
						: 'w-48'}"
					onclick={() => handleItemClick(item)}
				>
					<!-- Thumbnail -->
					<div
						class="relative aspect-video overflow-hidden rounded-lg bg-gray-200 dark:bg-gray-700"
					>
						{#if item.resource_thumbnail}
							<img
								src={item.resource_thumbnail}
								alt={item.resource_title}
								class="h-full w-full object-cover transition-transform group-hover:scale-105"
								loading="lazy"
								width="400"
								height="225"
							/>
						{:else}
							<div
								class="flex h-full w-full items-center justify-center bg-linear-to-br from-gray-100 to-gray-200 dark:from-gray-700 dark:to-gray-800"
							>
								{#if item.resource_type === 'video'}
									<Icon name="IconVideo" size={32} class="text-gray-400" />
								{:else if item.resource_type === 'pdf'}
									<Icon name="IconPdf" size={32} class="text-gray-400" />
								{:else if item.resource_type === 'image'}
									<Icon name="IconPhoto" size={32} class="text-gray-400" />
								{:else if item.resource_type === 'spreadsheet'}
									<Icon name="IconFileSpreadsheet" size={32} class="text-gray-400" />
								{:else}
									<Icon name="IconFileDescription" size={32} class="text-gray-400" />
								{/if}
							</div>
						{/if}

						<!-- Type badge -->
						<span
							class="absolute bottom-1 left-1 rounded bg-black/60 px-1.5 py-0.5 text-xs font-medium capitalize text-white"
						>
							{item.resource_type}
						</span>
					</div>

					<!-- Title and time -->
					<div class="mt-2">
						<p
							class="line-clamp-2 text-sm font-medium text-gray-900 group-hover:text-blue-600 dark:text-white dark:group-hover:text-blue-400"
						>
							{item.resource_title}
						</p>
						<p class="mt-0.5 text-xs text-gray-500 dark:text-gray-400">
							{formatTimeAgo(item.accessed_at)}
						</p>
					</div>
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	/* Custom scrollbar styles */
	.scrollbar-thin::-webkit-scrollbar {
		height: 6px;
	}

	.scrollbar-thumb-gray-300::-webkit-scrollbar-thumb {
		background-color: rgb(209 213 219);
		border-radius: 3px;
	}

	:global(.dark .scrollbar-thumb-gray-600::-webkit-scrollbar-thumb) {
		background-color: rgb(75 85 99);
	}

	.scrollbar-track-transparent::-webkit-scrollbar-track {
		background: transparent;
	}
</style>
