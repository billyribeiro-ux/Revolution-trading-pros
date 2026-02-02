<!--
  ResourceAnalytics.svelte
  Apple Principal Engineer ICT 7 Grade - February 2026

  Admin analytics dashboard for resources:
  - Total counts (resources, views, downloads, favorites)
  - Breakdown by type and access level
  - Top viewed/downloaded resources
  - Recent uploads
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import type { ResourceAnalytics } from '$lib/api/room-resources';
	import { getResourceAnalytics } from '$lib/api/room-resources';

	interface Props {
		roomId?: number;
		initialData?: ResourceAnalytics;
	}

	const { roomId, initialData }: Props = $props();

	const hasInitialData = initialData !== undefined;
	let analytics = $state<ResourceAnalytics | null>(initialData ?? null);
	let loading = $state(!hasInitialData);
	let error = $state('');

	onMount(async () => {
		if (!initialData) {
			await loadAnalytics();
		}
	});

	async function loadAnalytics() {
		loading = true;
		error = '';

		try {
			const response = await getResourceAnalytics(roomId);
			analytics = response.data;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load analytics';
		} finally {
			loading = false;
		}
	}

	function formatNumber(num: number): string {
		if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
		if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
		return num.toString();
	}

	function getTypeIcon(type: string): string {
		switch (type) {
			case 'video':
				return 'M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z';
			case 'pdf':
				return 'M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z';
			case 'image':
				return 'M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z';
			case 'document':
				return 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z';
			case 'spreadsheet':
				return 'M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z';
			default:
				return 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z';
		}
	}

	function getAccessLevelColor(level: string): string {
		switch (level) {
			case 'free':
				return 'bg-green-500';
			case 'member':
				return 'bg-blue-500';
			case 'premium':
				return 'bg-amber-500';
			case 'vip':
				return 'bg-purple-500';
			default:
				return 'bg-gray-500';
		}
	}
</script>

<div class="resource-analytics space-y-6">
	{#if loading}
		<!-- Loading skeleton -->
		<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
			{#each Array(4) as _}
				<div class="animate-pulse rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
					<div class="h-4 w-24 rounded bg-gray-200 dark:bg-gray-700"></div>
					<div class="mt-2 h-8 w-16 rounded bg-gray-200 dark:bg-gray-700"></div>
				</div>
			{/each}
		</div>
	{:else if error}
		<div class="rounded-lg border border-red-200 bg-red-50 p-6 text-center dark:border-red-800 dark:bg-red-900/20">
			<svg class="mx-auto h-10 w-10 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
			</svg>
			<p class="mt-2 text-red-600 dark:text-red-400">{error}</p>
			<button
				class="mt-4 rounded-lg bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700"
				onclick={loadAnalytics}
			>
				Retry
			</button>
		</div>
	{:else if analytics}
		<!-- Summary Cards -->
		<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
			<div class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30">
						<svg class="h-5 w-5 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
						</svg>
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">Total Resources</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-white">{formatNumber(analytics.total_resources)}</p>
					</div>
				</div>
			</div>

			<div class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-green-100 dark:bg-green-900/30">
						<svg class="h-5 w-5 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
						</svg>
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">Total Views</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-white">{formatNumber(analytics.total_views)}</p>
					</div>
				</div>
			</div>

			<div class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-100 dark:bg-purple-900/30">
						<svg class="h-5 w-5 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
						</svg>
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">Total Downloads</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-white">{formatNumber(analytics.total_downloads)}</p>
					</div>
				</div>
			</div>

			<div class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-amber-100 dark:bg-amber-900/30">
						<svg class="h-5 w-5 text-amber-600 dark:text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
						</svg>
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">Total Favorites</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-white">{formatNumber(analytics.total_favorites)}</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Charts Grid -->
		<div class="grid gap-6 lg:grid-cols-2">
			<!-- By Type -->
			<div class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Resources by Type</h3>
				<div class="space-y-3">
					{#each analytics.by_type as item}
						{@const percentage = analytics.total_resources > 0 ? (item.count / analytics.total_resources) * 100 : 0}
						<div>
							<div class="flex items-center justify-between text-sm">
								<div class="flex items-center gap-2">
									<svg class="h-4 w-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={getTypeIcon(item.resource_type)} />
									</svg>
									<span class="capitalize text-gray-700 dark:text-gray-300">{item.resource_type}</span>
								</div>
								<span class="font-medium text-gray-900 dark:text-white">{item.count}</span>
							</div>
							<div class="mt-1 h-2 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700">
								<div class="h-full rounded-full bg-blue-500" style="width: {percentage}%"></div>
							</div>
							<div class="mt-1 flex justify-between text-xs text-gray-500 dark:text-gray-400">
								<span>{item.total_views} views</span>
								<span>{item.total_downloads} downloads</span>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- By Access Level -->
			<div class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Resources by Access Level</h3>
				<div class="space-y-3">
					{#each analytics.by_access_level as item}
						{@const percentage = analytics.total_resources > 0 ? (item.count / analytics.total_resources) * 100 : 0}
						<div>
							<div class="flex items-center justify-between text-sm">
								<div class="flex items-center gap-2">
									<div class="h-3 w-3 rounded-full {getAccessLevelColor(item.access_level)}"></div>
									<span class="capitalize text-gray-700 dark:text-gray-300">{item.access_level}</span>
								</div>
								<span class="font-medium text-gray-900 dark:text-white">{item.count}</span>
							</div>
							<div class="mt-1 h-2 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700">
								<div class="h-full rounded-full {getAccessLevelColor(item.access_level)}" style="width: {percentage}%"></div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<!-- Top Lists -->
		<div class="grid gap-6 lg:grid-cols-3">
			<!-- Top Viewed -->
			<div class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Top Viewed</h3>
				<div class="space-y-3">
					{#each analytics.top_viewed.slice(0, 5) as item, i}
						<div class="flex items-start gap-3">
							<span class="flex h-6 w-6 flex-shrink-0 items-center justify-center rounded-full bg-gray-100 text-xs font-medium text-gray-600 dark:bg-gray-800 dark:text-gray-400">
								{i + 1}
							</span>
							<div class="min-w-0 flex-1">
								<p class="truncate text-sm font-medium text-gray-900 dark:text-white">{item.title}</p>
								<p class="text-xs text-gray-500 dark:text-gray-400">{formatNumber(item.views_count)} views</p>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Top Downloaded -->
			<div class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Top Downloaded</h3>
				<div class="space-y-3">
					{#each analytics.top_downloaded.slice(0, 5) as item, i}
						<div class="flex items-start gap-3">
							<span class="flex h-6 w-6 flex-shrink-0 items-center justify-center rounded-full bg-gray-100 text-xs font-medium text-gray-600 dark:bg-gray-800 dark:text-gray-400">
								{i + 1}
							</span>
							<div class="min-w-0 flex-1">
								<p class="truncate text-sm font-medium text-gray-900 dark:text-white">{item.title}</p>
								<p class="text-xs text-gray-500 dark:text-gray-400">{formatNumber(item.downloads_count)} downloads</p>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Recent Uploads -->
			<div class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900">
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Recent Uploads</h3>
				<div class="space-y-3">
					{#each analytics.recent_uploads.slice(0, 5) as item}
						<div class="flex items-start gap-3">
							<svg class="h-5 w-5 flex-shrink-0 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={getTypeIcon(item.resource_type)} />
							</svg>
							<div class="min-w-0 flex-1">
								<p class="truncate text-sm font-medium text-gray-900 dark:text-white">{item.title}</p>
								<p class="text-xs text-gray-500 dark:text-gray-400">
									{new Date(item.created_at).toLocaleDateString()}
								</p>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>
