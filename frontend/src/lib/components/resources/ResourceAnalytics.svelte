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
	import type { ResourceAnalytics } from '$lib/api/room-resources';
	import { getResourceAnalytics } from '$lib/api/room-resources';
	import Icon from '$lib/components/Icon.svelte';
	import { onMount } from 'svelte';

	interface Props {
		roomId?: number;
		initialData?: ResourceAnalytics;
	}

	let { roomId = undefined, initialData = undefined }: Props = $props();

	let analytics = $derived<ResourceAnalytics | null>(initialData ?? null);
	let loading = $derived<boolean>(!initialData);
	let error = $state('');

	onMount(() => {
		if (initialData) {
			analytics = initialData;
			loading = false;
			return;
		}

		void loadAnalytics();
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
			{#each Array(4) as _, i (i)}
				<div
					class="animate-pulse rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
				>
					<div class="h-4 w-24 rounded bg-gray-200 dark:bg-gray-700"></div>
					<div class="mt-2 h-8 w-16 rounded bg-gray-200 dark:bg-gray-700"></div>
				</div>
			{/each}
		</div>
	{:else if error}
		<div
			class="rounded-lg border border-red-200 bg-red-50 p-6 text-center dark:border-red-800 dark:bg-red-900/20"
		>
			<Icon name="IconAlertTriangle" size={40} class="mx-auto text-red-400" />
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
			<div
				class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
			>
				<div class="flex items-center gap-3">
					<div
						class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30"
					>
						<Icon name="IconFileText" size={20} class="text-blue-600 dark:text-blue-400" />
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">Total Resources</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-white">
							{formatNumber(analytics.total_resources)}
						</p>
					</div>
				</div>
			</div>

			<div
				class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
			>
				<div class="flex items-center gap-3">
					<div
						class="flex h-10 w-10 items-center justify-center rounded-lg bg-green-100 dark:bg-green-900/30"
					>
						<Icon name="IconEye" size={20} class="text-green-600 dark:text-green-400" />
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">Total Views</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-white">
							{formatNumber(analytics.total_views)}
						</p>
					</div>
				</div>
			</div>

			<div
				class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
			>
				<div class="flex items-center gap-3">
					<div
						class="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-100 dark:bg-purple-900/30"
					>
						<Icon name="IconDownload" size={20} class="text-purple-600 dark:text-purple-400" />
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">Total Downloads</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-white">
							{formatNumber(analytics.total_downloads)}
						</p>
					</div>
				</div>
			</div>

			<div
				class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
			>
				<div class="flex items-center gap-3">
					<div
						class="flex h-10 w-10 items-center justify-center rounded-lg bg-amber-100 dark:bg-amber-900/30"
					>
						<Icon name="IconHeart" size={20} class="text-amber-600 dark:text-amber-400" />
					</div>
					<div>
						<p class="text-sm text-gray-500 dark:text-gray-400">Total Favorites</p>
						<p class="text-2xl font-bold text-gray-900 dark:text-white">
							{formatNumber(analytics.total_favorites)}
						</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Charts Grid -->
		<div class="grid gap-6 lg:grid-cols-2">
			<!-- By Type -->
			<div
				class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
			>
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Resources by Type</h3>
				<div class="space-y-3">
					{#each analytics.by_type as item (item.resource_type)}
						{const percentage =
							analytics.total_resources > 0 ? (item.count / analytics.total_resources) * 100 : 0}
						<div>
							<div class="flex items-center justify-between text-sm">
								<div class="flex items-center gap-2">
									{#if item.resource_type === 'video'}
										<Icon name="IconVideo" size={16} class="text-gray-500" />
									{:else if item.resource_type === 'pdf'}
										<Icon name="IconPdf" size={16} class="text-gray-500" />
									{:else if item.resource_type === 'image'}
										<Icon name="IconPhoto" size={16} class="text-gray-500" />
									{:else if item.resource_type === 'spreadsheet'}
										<Icon name="IconFileSpreadsheet" size={16} class="text-gray-500" />
									{:else}
										<Icon name="IconFileDescription" size={16} class="text-gray-500" />
									{/if}
									<span class="capitalize text-gray-700 dark:text-gray-300"
										>{item.resource_type}</span
									>
								</div>
								<span class="font-medium text-gray-900 dark:text-white">{item.count}</span>
							</div>
							<div
								class="mt-1 h-2 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700"
							>
								<div class="h-full rounded-full bg-blue-500" style:width={`${percentage}%`}></div>
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
			<div
				class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
			>
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">
					Resources by Access Level
				</h3>
				<div class="space-y-3">
					{#each analytics.by_access_level as item (item.access_level)}
						{const percentage =
							analytics.total_resources > 0 ? (item.count / analytics.total_resources) * 100 : 0}
						<div>
							<div class="flex items-center justify-between text-sm">
								<div class="flex items-center gap-2">
									<div
										class={['h-3 w-3 rounded-full', getAccessLevelColor(item.access_level)]}
									></div>
									<span class="capitalize text-gray-700 dark:text-gray-300"
										>{item.access_level}</span
									>
								</div>
								<span class="font-medium text-gray-900 dark:text-white">{item.count}</span>
							</div>
							<div
								class="mt-1 h-2 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700"
							>
								<div
									class={['h-full rounded-full', getAccessLevelColor(item.access_level)]}
									style:width={`${percentage}%`}
								></div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<!-- Top Lists -->
		<div class="grid gap-6 lg:grid-cols-3">
			<!-- Top Viewed -->
			<div
				class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
			>
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Top Viewed</h3>
				<div class="space-y-3">
					{#each analytics.top_viewed.slice(0, 5) as item, i (item.id)}
						<div class="flex items-start gap-3">
							<span
								class="flex h-6 w-6 flex-shrink-0 items-center justify-center rounded-full bg-gray-100 text-xs font-medium text-gray-600 dark:bg-gray-800 dark:text-gray-400"
							>
								{i + 1}
							</span>
							<div class="min-w-0 flex-1">
								<p class="truncate text-sm font-medium text-gray-900 dark:text-white">
									{item.title}
								</p>
								<p class="text-xs text-gray-500 dark:text-gray-400">
									{formatNumber(item.views_count)} views
								</p>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Top Downloaded -->
			<div
				class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
			>
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Top Downloaded</h3>
				<div class="space-y-3">
					{#each analytics.top_downloaded.slice(0, 5) as item, i (item.id)}
						<div class="flex items-start gap-3">
							<span
								class="flex h-6 w-6 flex-shrink-0 items-center justify-center rounded-full bg-gray-100 text-xs font-medium text-gray-600 dark:bg-gray-800 dark:text-gray-400"
							>
								{i + 1}
							</span>
							<div class="min-w-0 flex-1">
								<p class="truncate text-sm font-medium text-gray-900 dark:text-white">
									{item.title}
								</p>
								<p class="text-xs text-gray-500 dark:text-gray-400">
									{formatNumber(item.downloads_count)} downloads
								</p>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Recent Uploads -->
			<div
				class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-700 dark:bg-gray-900"
			>
				<h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Recent Uploads</h3>
				<div class="space-y-3">
					{#each analytics.recent_uploads.slice(0, 5) as item (item.id)}
						<div class="flex items-start gap-3">
							{#if item.resource_type === 'video'}
								<Icon name="IconVideo" size={20} class="flex-shrink-0 text-gray-400" />
							{:else if item.resource_type === 'pdf'}
								<Icon name="IconPdf" size={20} class="flex-shrink-0 text-gray-400" />
							{:else if item.resource_type === 'image'}
								<Icon name="IconPhoto" size={20} class="flex-shrink-0 text-gray-400" />
							{:else if item.resource_type === 'spreadsheet'}
								<Icon name="IconFileSpreadsheet" size={20} class="flex-shrink-0 text-gray-400" />
							{:else}
								<Icon name="IconFileDescription" size={20} class="flex-shrink-0 text-gray-400" />
							{/if}
							<div class="min-w-0 flex-1">
								<p class="truncate text-sm font-medium text-gray-900 dark:text-white">
									{item.title}
								</p>
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
