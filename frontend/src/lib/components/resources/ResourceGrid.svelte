<!--
  ResourceGrid.svelte
  Apple Principal Engineer ICT 7 Grade - February 2026

  Grid display for resources with:
  - Filtering by type, section, access level
  - Search functionality
  - Pagination
  - Empty states
  - Loading states
-->
<script lang="ts">
	import type { RoomResource, ResourceListQuery, ContentType, ResourceType, AccessLevel } from '$lib/api/room-resources';
	import { listResources } from '$lib/api/room-resources';
	import ResourceCard from './ResourceCard.svelte';

	interface Props {
		roomId?: number;
		contentType?: ContentType;
		resourceType?: ResourceType;
		section?: string;
		accessLevel?: AccessLevel;
		courseId?: number;
		lessonId?: number;
		showSearch?: boolean;
		showFilters?: boolean;
		showAccessLevel?: boolean;
		showVersion?: boolean;
		columns?: 2 | 3 | 4;
		compact?: boolean;
		perPage?: number;
		initialResources?: RoomResource[];
		onSelect?: (resource: RoomResource) => void;
		onDownload?: (resource: RoomResource) => void;
		onPreview?: (resource: RoomResource) => void;
	}

	let {
		roomId = undefined,
		contentType = undefined,
		resourceType = undefined,
		section = undefined,
		accessLevel = undefined,
		courseId = undefined,
		lessonId = undefined,
		showSearch = true,
		showFilters = true,
		showAccessLevel = true,
		showVersion = false,
		columns = 3,
		compact = false,
		perPage = 12,
		initialResources = undefined,
		onSelect,
		onDownload,
		onPreview
	}: Props = $props();

	let resources = $state<RoomResource[]>(initialResources ?? []);
	let loading = $state(!initialResources);
	let error = $state('');
	let currentPage = $state(1);
	let totalPages = $state(1);
	let total = $state(0);
	let searchQuery = $state('');
	let selectedType = $state<ResourceType | ''>(resourceType ?? '');
	let selectedSection = $state(section ?? '');
	let selectedAccessLevel = $state<AccessLevel | ''>(accessLevel ?? '');

	// Resource type options
	const resourceTypes: { value: ResourceType | ''; label: string }[] = [
		{ value: '', label: 'All Types' },
		{ value: 'video', label: 'Videos' },
		{ value: 'pdf', label: 'PDFs' },
		{ value: 'document', label: 'Documents' },
		{ value: 'image', label: 'Images' },
		{ value: 'spreadsheet', label: 'Spreadsheets' },
		{ value: 'archive', label: 'Archives' }
	];

	// Section options
	const sectionOptions = [
		{ value: '', label: 'All Sections' },
		{ value: 'introduction', label: 'Introduction' },
		{ value: 'latest_updates', label: 'Latest Updates' },
		{ value: 'premium_daily_videos', label: 'Daily Videos' },
		{ value: 'watchlist', label: 'Watchlist' },
		{ value: 'weekly_alerts', label: 'Weekly Alerts' },
		{ value: 'learning_center', label: 'Learning Center' }
	];

	// Access level options
	const accessLevelOptions = [
		{ value: '', label: 'All Access Levels' },
		{ value: 'free', label: 'Free' },
		{ value: 'member', label: 'Member' },
		{ value: 'premium', label: 'Premium' },
		{ value: 'vip', label: 'VIP' }
	];

	async function loadResources() {
		loading = true;
		error = '';

		const query: ResourceListQuery = {
			page: currentPage,
			per_page: perPage,
			room_id: roomId,
			content_type: contentType,
			resource_type: selectedType || undefined,
			search: searchQuery || undefined,
			access_level: selectedAccessLevel ? (selectedAccessLevel as AccessLevel) : undefined
		};

		// Add section filter if specified
		if (selectedSection) {
			// Note: section filtering is handled via the section query param
			(query as any).section = selectedSection;
		}

		// Add course/lesson filters
		if (courseId) {
			(query as any).course_id = courseId;
		}
		if (lessonId) {
			(query as any).lesson_id = lessonId;
		}

		try {
			const response = await listResources(query);
			resources = response.data;
			total = response.meta.total;
			totalPages = response.meta.last_page;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load resources';
			resources = [];
		} finally {
			loading = false;
		}
	}

	// Initial load
	$effect(() => {
		if (!initialResources) {
			loadResources();
		}
	});

	// Debounced search
	let searchTimeout: ReturnType<typeof setTimeout>;
	function handleSearch(event: Event) {
		const target = event.target as HTMLInputElement;
		clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			searchQuery = target.value;
			currentPage = 1;
			loadResources();
		}, 300);
	}

	function handleFilterChange() {
		currentPage = 1;
		loadResources();
	}

	function handlePageChange(page: number) {
		currentPage = page;
		loadResources();
	}

	function handleResourceClick(resource: RoomResource) {
		onSelect?.(resource);
	}

	function handleResourceDownload(resource: RoomResource) {
		onDownload?.(resource);
	}

	function handleResourcePreview(resource: RoomResource) {
		onPreview?.(resource);
	}

	// Grid column classes
	let gridClasses = $derived({
		2: 'grid-cols-1 sm:grid-cols-2',
		3: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3',
		4: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4'
	}[columns]);
</script>

<div class="resource-grid">
	<!-- Filters and Search -->
	{#if showSearch || showFilters}
		<div class="filters mb-6 flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
			<!-- Search -->
			{#if showSearch}
				<div class="relative flex-1 lg:max-w-md">
					<input
						type="search"
						placeholder="Search resources..."
						class="w-full rounded-lg border border-gray-300 bg-white py-2 pl-10 pr-4 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
						oninput={handleSearch}
					/>
					<svg
						class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
					</svg>
				</div>
			{/if}

			<!-- Filter dropdowns -->
			{#if showFilters}
				<div class="flex flex-wrap gap-2">
					<!-- Type filter -->
					<select
						bind:value={selectedType}
						onchange={handleFilterChange}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
					>
						{#each resourceTypes as type}
							<option value={type.value}>{type.label}</option>
						{/each}
					</select>

					<!-- Section filter -->
					<select
						bind:value={selectedSection}
						onchange={handleFilterChange}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
					>
						{#each sectionOptions as opt}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>

					<!-- Access level filter -->
					<select
						bind:value={selectedAccessLevel}
						onchange={handleFilterChange}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
					>
						{#each accessLevelOptions as opt}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Results count -->
	{#if total > 0 && !loading}
		<p class="mb-4 text-sm text-gray-500 dark:text-gray-400">
			Showing {(currentPage - 1) * perPage + 1} - {Math.min(currentPage * perPage, total)} of {total} resources
		</p>
	{/if}

	<!-- Loading state -->
	{#if loading}
		<div class="grid {gridClasses} gap-6">
			{#each Array(perPage) as _}
				<div class="animate-pulse">
					<div class="aspect-video rounded-t-xl bg-gray-200 dark:bg-gray-700"></div>
					<div class="rounded-b-xl border border-t-0 border-gray-200 p-4 dark:border-gray-700">
						<div class="mb-2 h-4 w-3/4 rounded bg-gray-200 dark:bg-gray-700"></div>
						<div class="mb-3 h-3 w-full rounded bg-gray-200 dark:bg-gray-700"></div>
						<div class="flex gap-2">
							<div class="h-5 w-16 rounded bg-gray-200 dark:bg-gray-700"></div>
							<div class="h-5 w-16 rounded bg-gray-200 dark:bg-gray-700"></div>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{:else if error}
		<!-- Error state -->
		<div class="rounded-lg border border-red-200 bg-red-50 p-8 text-center dark:border-red-800 dark:bg-red-900/20">
			<svg class="mx-auto h-12 w-12 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
			</svg>
			<h3 class="mt-4 text-lg font-medium text-red-800 dark:text-red-200">Failed to load resources</h3>
			<p class="mt-2 text-sm text-red-600 dark:text-red-300">{error}</p>
			<button
				class="mt-4 rounded-lg bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700"
				onclick={loadResources}
			>
				Try Again
			</button>
		</div>
	{:else if resources.length === 0}
		<!-- Empty state -->
		<div class="rounded-lg border border-gray-200 bg-gray-50 p-8 text-center dark:border-gray-700 dark:bg-gray-800">
			<svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
			</svg>
			<h3 class="mt-4 text-lg font-medium text-gray-900 dark:text-white">No resources found</h3>
			<p class="mt-2 text-sm text-gray-500 dark:text-gray-400">
				{#if searchQuery}
					No resources match your search criteria.
				{:else}
					There are no resources available in this section.
				{/if}
			</p>
			{#if searchQuery}
				<button
					class="mt-4 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
					onclick={() => { searchQuery = ''; loadResources(); }}
				>
					Clear Search
				</button>
			{/if}
		</div>
	{:else}
		<!-- Resource grid -->
		<div class="grid {gridClasses} gap-6">
			{#each resources as resource (resource.id)}
				<ResourceCard
					{resource}
					{showAccessLevel}
					{showVersion}
					{compact}
					onClick={handleResourceClick}
					onDownload={handleResourceDownload}
					onPreview={handleResourcePreview}
				/>
			{/each}
		</div>

		<!-- Pagination -->
		{#if totalPages > 1}
			<nav class="mt-8 flex items-center justify-center gap-2" aria-label="Pagination">
				<!-- Previous -->
				<button
					class="rounded-lg border border-gray-300 px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					disabled={currentPage === 1}
					onclick={() => handlePageChange(currentPage - 1)}
				>
					Previous
				</button>

				<!-- Page numbers -->
				{#each Array(Math.min(5, totalPages)) as _, i}
					{@const pageNum = currentPage <= 3 ? i + 1 : currentPage + i - 2}
					{#if pageNum > 0 && pageNum <= totalPages}
						<button
							class="rounded-lg px-3 py-2 text-sm font-medium {pageNum === currentPage ? 'bg-blue-600 text-white' : 'border border-gray-300 text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700'}"
							onclick={() => handlePageChange(pageNum)}
						>
							{pageNum}
						</button>
					{/if}
				{/each}

				<!-- Next -->
				<button
					class="rounded-lg border border-gray-300 px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
					disabled={currentPage === totalPages}
					onclick={() => handlePageChange(currentPage + 1)}
				>
					Next
				</button>
			</nav>
		{/if}
	{/if}
</div>
