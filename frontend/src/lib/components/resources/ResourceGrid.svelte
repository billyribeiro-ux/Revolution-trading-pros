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
	import type {
		RoomResource,
		ResourceListQuery,
		ContentType,
		ResourceType,
		AccessLevel
	} from '$lib/api/room-resources';
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

	let resources = $state<RoomResource[]>([]);
	let loading = $state(true);
	let error = $state('');
	let currentPage = $state(1);
	let totalPages = $state(1);
	let total = $state(0);
	let searchQuery = $state('');
	let selectedType = $state<ResourceType | ''>('');
	let selectedSection = $state('');
	let selectedAccessLevel = $state<AccessLevel | ''>('');

	// Sync state from props when they change
	$effect(() => {
		if (initialResources !== undefined) {
			resources = initialResources;
			loading = false;
		}
	});

	$effect(() => {
		if (resourceType !== undefined) {
			selectedType = resourceType;
		}
	});

	$effect(() => {
		if (section !== undefined) {
			selectedSection = section;
		}
	});

	$effect(() => {
		if (accessLevel !== undefined) {
			selectedAccessLevel = accessLevel;
		}
	});

	// Grid column classes based on columns prop
	let gridClasses = $derived(
		columns === 4
			? 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-4'
			: columns === 2
				? 'grid-cols-1 sm:grid-cols-2'
				: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3'
	);

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
</script>

<div class="rg-root">
	{#if showSearch || showFilters}
		<div class="rg-filters">
			{#if showSearch}
				<div class="rg-search-wrap">
					<input
						type="search"
						placeholder="Search resources..."
						class="rg-search-input"
						oninput={handleSearch}
					/>
					<svg class="rg-search-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
						/>
					</svg>
				</div>
			{/if}

			{#if showFilters}
				<div class="flex flex-wrap gap-2">
					<!-- Type filter -->
					<select
						bind:value={selectedType}
						onchange={handleFilterChange}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
					>
						{#each resourceTypes as type (type.value)}
							<option value={type.value}>{type.label}</option>
						{/each}
					</select>

					<!-- Section filter -->
					<select
						bind:value={selectedSection}
						onchange={handleFilterChange}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
					>
						{#each sectionOptions as opt (opt.value)}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>

					<!-- Access level filter -->
					<select
						bind:value={selectedAccessLevel}
						onchange={handleFilterChange}
						class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
					>
						{#each accessLevelOptions as opt (opt.value)}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>
				</div>
			{/if}
		</div>
	{/if}

	{#if total > 0 && !loading}
		<p class="rg-count">
			Showing {(currentPage - 1) * perPage + 1} - {Math.min(currentPage * perPage, total)} of {total}
			resources
		</p>
	{/if}

	{#if loading}
		<div class="grid {gridClasses} gap-6">
			{#each Array(perPage) as _, i (i)}
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
		<div class="rg-error">
			<svg
				class="rg-state-icon rg-error-color"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
				/>
			</svg>
			<h3 class="rg-error-title">Failed to load resources</h3>
			<p class="rg-error-msg">{error}</p>
			<button class="rg-error-btn" onclick={loadResources}>Try Again</button>
		</div>
	{:else if resources.length === 0}
		<div class="rg-empty">
			<svg
				class="rg-state-icon rg-muted-color"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
				/>
			</svg>
			<h3 class="rg-empty-title">No resources found</h3>
			<p class="rg-empty-text">
				{#if searchQuery}
					No resources match your search criteria.
				{:else}
					There are no resources available in this section.
				{/if}
			</p>
			{#if searchQuery}
				<button
					class="rg-primary-btn"
					onclick={() => {
						searchQuery = '';
						loadResources();
					}}>Clear Search</button
				>
			{/if}
		</div>
	{:else}
		<div class="rg-grid" data-columns={columns}>
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

		{#if totalPages > 1}
			<nav class="rg-pagination" aria-label="Pagination">
				<button
					class="rg-page-btn"
					disabled={currentPage === 1}
					onclick={() => handlePageChange(currentPage - 1)}>Previous</button
				>

				<!-- Page numbers -->
				{#each Array(Math.min(5, totalPages)) as _, i (i)}
					{@const pageNum = currentPage <= 3 ? i + 1 : currentPage + i - 2}
					{#if pageNum > 0 && pageNum <= totalPages}
						<button
							class="rg-page-btn"
							data-active={pageNum === currentPage || undefined}
							onclick={() => handlePageChange(pageNum)}>{pageNum}</button
						>
					{/if}
				{/each}

				<button
					class="rg-page-btn"
					disabled={currentPage === totalPages}
					onclick={() => handlePageChange(currentPage + 1)}>Next</button
				>
			</nav>
		{/if}
	{/if}
</div>

<style>
	.rg-root {
		display: flex;
		flex-direction: column;
	}

	/* ─── Filters ─── */
	.rg-filters {
		margin-block-end: var(--space-6);
		display: flex;
		flex-direction: column;
		gap: var(--space-4);

		@media (min-width: 1024px) {
			flex-direction: row;
			align-items: center;
			justify-content: space-between;
		}
	}

	.rg-search-wrap {
		position: relative;
		flex: 1;

		@media (min-width: 1024px) {
			max-inline-size: 28rem;
		}
	}

	.rg-search-input {
		inline-size: 100%;
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.82 0.005 265);
		background-color: oklch(1 0 0);
		padding-block: var(--space-2);
		padding-inline-start: 2.5rem;
		padding-inline-end: var(--space-4);
		font-size: var(--text-sm);
		color: oklch(0.15 0.01 265);

		&:focus {
			outline: none;
			border-color: oklch(0.6 0.2 260);
			box-shadow: 0 0 0 1px oklch(0.6 0.2 260);
		}
	}

	.rg-search-icon {
		position: absolute;
		inset-inline-start: 0.75rem;
		inset-block-start: 50%;
		transform: translateY(-50%);
		inline-size: 1rem;
		block-size: 1rem;
		color: oklch(0.65 0.01 265);
	}

	.rg-filter-group {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
	}

	.rg-select {
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.82 0.005 265);
		background-color: oklch(1 0 0);
		padding-inline: var(--space-3);
		padding-block: var(--space-2);
		font-size: var(--text-sm);
		color: oklch(0.15 0.01 265);

		&:focus {
			outline: none;
			border-color: oklch(0.6 0.2 260);
			box-shadow: 0 0 0 1px oklch(0.6 0.2 260);
		}
	}

	/* ─── Count ─── */
	.rg-count {
		margin-block-end: var(--space-4);
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
	}

	/* ─── Grid ─── */
	.rg-grid {
		display: grid;
		gap: var(--space-6);
		grid-template-columns: 1fr;

		@media (min-width: 640px) {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.rg-grid[data-columns='2'] {
		@media (min-width: 640px) {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.rg-grid[data-columns='3'] {
		@media (min-width: 640px) {
			grid-template-columns: repeat(2, 1fr);
		}
		@media (min-width: 1024px) {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.rg-grid[data-columns='4'] {
		@media (min-width: 640px) {
			grid-template-columns: repeat(2, 1fr);
		}
		@media (min-width: 1024px) {
			grid-template-columns: repeat(3, 1fr);
		}
		@media (min-width: 1280px) {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	/* ─── Skeleton ─── */
	.rg-skel {
		animation: pulse 2s ease-in-out infinite;
	}

	.rg-skel-thumb {
		aspect-ratio: 16 / 9;
		border-start-start-radius: var(--radius-xl);
		border-start-end-radius: var(--radius-xl);
		background-color: oklch(0.9 0.005 265);
	}

	.rg-skel-body {
		border-end-start-radius: var(--radius-xl);
		border-end-end-radius: var(--radius-xl);
		border: 1px solid oklch(0.9 0.005 265);
		border-block-start: none;
		padding: var(--space-4);
	}

	.rg-skel-line {
		border-radius: var(--radius-sm);
		background-color: oklch(0.9 0.005 265);
	}

	.rg-skel-w34 {
		block-size: 1rem;
		inline-size: 75%;
		margin-block-end: var(--space-2);
	}
	.rg-skel-wfull {
		block-size: 0.75rem;
		inline-size: 100%;
		margin-block-end: var(--space-3);
	}

	.rg-skel-tags {
		display: flex;
		gap: var(--space-2);
	}
	.rg-skel-tag {
		block-size: 1.25rem;
		inline-size: 4rem;
		border-radius: var(--radius-sm);
		background-color: oklch(0.9 0.005 265);
	}

	/* ─── States ─── */
	.rg-state-icon {
		margin-inline: auto;
		inline-size: 3rem;
		block-size: 3rem;
	}
	.rg-error-color {
		color: oklch(0.65 0.15 25);
	}
	.rg-muted-color {
		color: oklch(0.65 0.01 265);
	}

	.rg-error {
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.75 0.15 25);
		background-color: oklch(0.97 0.02 25);
		padding: var(--space-8);
		text-align: center;
	}

	.rg-error-title {
		margin-block-start: var(--space-4);
		font-size: var(--text-lg);
		font-weight: var(--weight-medium);
		color: oklch(0.4 0.15 25);
	}

	.rg-error-msg {
		margin-block-start: var(--space-2);
		font-size: var(--text-sm);
		color: oklch(0.5 0.2 25);
	}

	.rg-error-btn {
		margin-block-start: var(--space-4);
		border-radius: var(--radius-lg);
		background-color: oklch(0.5 0.2 25);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		border: none;
		cursor: pointer;

		&:hover {
			background-color: oklch(0.45 0.2 25);
		}
	}

	.rg-empty {
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.9 0.005 265);
		background-color: oklch(0.97 0.002 265);
		padding: var(--space-8);
		text-align: center;
	}

	.rg-empty-title {
		margin-block-start: var(--space-4);
		font-size: var(--text-lg);
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
	}

	.rg-empty-text {
		margin-block-start: var(--space-2);
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
	}

	.rg-primary-btn {
		margin-block-start: var(--space-4);
		border-radius: var(--radius-lg);
		background-color: oklch(0.55 0.2 260);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		border: none;
		cursor: pointer;

		&:hover {
			background-color: oklch(0.48 0.2 260);
		}
	}

	/* ─── Pagination ─── */
	.rg-pagination {
		margin-block-start: var(--space-8);
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
	}

	.rg-page-btn {
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.82 0.005 265);
		padding-inline: var(--space-3);
		padding-block: var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.35 0.01 265);
		background: none;
		cursor: pointer;
		transition: background-color 200ms var(--ease-default);

		&:hover {
			background-color: oklch(0.97 0.002 265);
		}
		&:disabled {
			cursor: not-allowed;
			opacity: 0.5;
		}

		&[data-active] {
			background-color: oklch(0.55 0.2 260);
			color: oklch(1 0 0);
			border-color: oklch(0.55 0.2 260);
		}
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}
</style>
