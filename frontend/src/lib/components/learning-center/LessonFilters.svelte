<!--
/**
 * LessonFilters Component - Learning Center
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Search and filter controls for browsing lessons.
 * Includes category, trainer, type filters and search.
 *
 * @version 1.0.0 (December 2025)
 */
-->

<script lang="ts">
	import type { LessonCategory, Trainer, LessonType, LessonFilter } from '$lib/types/learning-center';
	import { IconSearch, IconFilter, IconX } from '$lib/icons';

	interface Props {
		categories?: LessonCategory[];
		trainers?: Trainer[];
		currentFilter?: LessonFilter;
		onFilterChange?: (filter: LessonFilter) => void;
		showTypeFilter?: boolean;
		showAccessFilter?: boolean;
		compact?: boolean;
	}

	let {
		categories = [],
		trainers = [],
		currentFilter = {},
		onFilterChange,
		showTypeFilter = true,
		showAccessFilter = false,
		compact = false
	}: Props = $props();

	// Local state
	let searchQuery = $state(currentFilter.search || '');
	let selectedCategory = $state(currentFilter.categoryId || '');
	let selectedTrainer = $state(currentFilter.trainerId || '');
	let selectedType = $state<LessonType | ''>(currentFilter.type || '');
	let showFiltersPanel = $state(false);

	// Lesson types
	const lessonTypes: { value: LessonType; label: string }[] = [
		{ value: 'video', label: 'Video' },
		{ value: 'article', label: 'Article' },
		{ value: 'pdf', label: 'PDF' },
		{ value: 'quiz', label: 'Quiz' },
		{ value: 'webinar-replay', label: 'Webinar Replay' }
	];

	// Count active filters
	let activeFilterCount = $derived(
		[selectedCategory, selectedTrainer, selectedType].filter(Boolean).length
	);

	// Emit filter changes
	function emitFilterChange() {
		const filter: LessonFilter = {};

		if (searchQuery.trim()) filter.search = searchQuery.trim();
		if (selectedCategory) filter.categoryId = selectedCategory;
		if (selectedTrainer) filter.trainerId = selectedTrainer;
		if (selectedType) filter.type = selectedType as LessonType;

		onFilterChange?.(filter);
	}

	// Handle search input
	function handleSearchInput(e: Event) {
		searchQuery = (e.target as HTMLInputElement).value;
		emitFilterChange();
	}

	// Handle search submit
	function handleSearchSubmit(e: SubmitEvent) {
		e.preventDefault();
		emitFilterChange();
	}

	// Clear all filters
	function clearAllFilters() {
		searchQuery = '';
		selectedCategory = '';
		selectedTrainer = '';
		selectedType = '';
		emitFilterChange();
	}

	// Watch for filter changes
	$effect(() => {
		emitFilterChange();
	});
</script>

<div class="lesson-filters" class:compact>
	<!-- Search bar -->
	<form class="search-form" onsubmit={handleSearchSubmit}>
		<div class="search-input-wrapper">
			<IconSearch size={18} />
			<input
				type="search"
				placeholder="Search lessons..."
				value={searchQuery}
				oninput={handleSearchInput}
				class="search-input"
			/>
			{#if searchQuery}
				<button
					type="button"
					class="clear-search"
					onclick={() => {
						searchQuery = '';
						emitFilterChange();
					}}
					aria-label="Clear search"
				>
					<IconX size={16} />
				</button>
			{/if}
		</div>
	</form>

	<!-- Filter toggle button (mobile/compact) -->
	{#if compact}
		<button
			type="button"
			class="filter-toggle"
			class:active={showFiltersPanel || activeFilterCount > 0}
			onclick={() => (showFiltersPanel = !showFiltersPanel)}
		>
			<IconFilter size={18} />
			<span>Filters</span>
			{#if activeFilterCount > 0}
				<span class="filter-count">{activeFilterCount}</span>
			{/if}
		</button>
	{/if}

	<!-- Filters panel -->
	{#if !compact || showFiltersPanel}
		<div class="filters-panel" class:mobile={compact}>
			<!-- Category filter -->
			{#if categories.length > 0}
				<div class="filter-group">
					<label for="category-filter" class="filter-label">Category</label>
					<select
						id="category-filter"
						bind:value={selectedCategory}
						class="filter-select"
					>
						<option value="">All Categories</option>
						{#each categories as category}
							<option value={category.id}>{category.name}</option>
						{/each}
					</select>
				</div>
			{/if}

			<!-- Trainer filter -->
			{#if trainers.length > 0}
				<div class="filter-group">
					<label for="trainer-filter" class="filter-label">Instructor</label>
					<select
						id="trainer-filter"
						bind:value={selectedTrainer}
						class="filter-select"
					>
						<option value="">All Instructors</option>
						{#each trainers as trainer}
							<option value={trainer.id}>{trainer.name}</option>
						{/each}
					</select>
				</div>
			{/if}

			<!-- Type filter -->
			{#if showTypeFilter}
				<div class="filter-group">
					<label for="type-filter" class="filter-label">Type</label>
					<select
						id="type-filter"
						bind:value={selectedType}
						class="filter-select"
					>
						<option value="">All Types</option>
						{#each lessonTypes as type}
							<option value={type.value}>{type.label}</option>
						{/each}
					</select>
				</div>
			{/if}

			<!-- Clear filters button -->
			{#if activeFilterCount > 0 || searchQuery}
				<button
					type="button"
					class="clear-filters-btn"
					onclick={clearAllFilters}
				>
					<IconX size={14} />
					Clear all
				</button>
			{/if}
		</div>
	{/if}

	<!-- Active filter tags -->
	{#if (selectedCategory || selectedTrainer || selectedType) && !compact}
		<div class="active-filters">
			{#if selectedCategory}
				{@const category = categories.find(c => c.id === selectedCategory)}
				{#if category}
					<span class="filter-tag">
						{category.name}
						<button
							type="button"
							onclick={() => {
								selectedCategory = '';
								emitFilterChange();
							}}
							aria-label="Remove category filter"
						>
							<IconX size={12} />
						</button>
					</span>
				{/if}
			{/if}

			{#if selectedTrainer}
				{@const trainer = trainers.find(t => t.id === selectedTrainer)}
				{#if trainer}
					<span class="filter-tag">
						{trainer.name}
						<button
							type="button"
							onclick={() => {
								selectedTrainer = '';
								emitFilterChange();
							}}
							aria-label="Remove trainer filter"
						>
							<IconX size={12} />
						</button>
					</span>
				{/if}
			{/if}

			{#if selectedType}
				{@const type = lessonTypes.find(t => t.value === selectedType)}
				{#if type}
					<span class="filter-tag">
						{type.label}
						<button
							type="button"
							onclick={() => {
								selectedType = '';
								emitFilterChange();
							}}
							aria-label="Remove type filter"
						>
							<IconX size={12} />
						</button>
					</span>
				{/if}
			{/if}
		</div>
	{/if}
</div>

<style>
	.lesson-filters {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.lesson-filters.compact {
		flex-direction: row;
		flex-wrap: wrap;
		align-items: center;
	}

	/* Search form */
	.search-form {
		flex: 1;
		min-width: 200px;
	}

	.search-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 8px;
		transition: all 0.2s ease;
	}

	.search-input-wrapper:focus-within {
		border-color: #f97316;
		box-shadow: 0 0 0 3px rgba(249, 115, 22, 0.1);
	}

	.search-input-wrapper :global(svg) {
		color: #64748b;
		flex-shrink: 0;
	}

	.search-input {
		flex: 1;
		background: none;
		border: none;
		outline: none;
		font-size: 0.875rem;
		color: white;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.clear-search {
		background: none;
		border: none;
		padding: 4px;
		cursor: pointer;
		color: #64748b;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: color 0.2s ease;
	}

	.clear-search:hover {
		color: white;
	}

	/* Filter toggle (compact mode) */
	.filter-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 8px;
		cursor: pointer;
		color: #94a3b8;
		font-size: 0.875rem;
		transition: all 0.2s ease;
	}

	.filter-toggle:hover,
	.filter-toggle.active {
		border-color: #f97316;
		color: white;
	}

	.filter-count {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 20px;
		height: 20px;
		padding: 0 6px;
		background: #f97316;
		border-radius: 10px;
		font-size: 0.7rem;
		font-weight: 600;
		color: white;
	}

	/* Filters panel */
	.filters-panel {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		align-items: flex-end;
	}

	.filters-panel.mobile {
		width: 100%;
		padding: 1rem;
		background: #1e293b;
		border-radius: 8px;
		border: 1px solid #334155;
		margin-top: 0.5rem;
	}

	.filter-group {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		flex: 1;
		min-width: 150px;
	}

	.filter-label {
		font-size: 0.7rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.filter-select {
		padding: 0.625rem 0.875rem;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 6px;
		color: white;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.filter-select:hover {
		border-color: #475569;
	}

	.filter-select:focus {
		outline: none;
		border-color: #f97316;
	}

	.clear-filters-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.625rem 0.875rem;
		background: none;
		border: 1px solid #475569;
		border-radius: 6px;
		cursor: pointer;
		color: #94a3b8;
		font-size: 0.8rem;
		transition: all 0.2s ease;
	}

	.clear-filters-btn:hover {
		background: rgba(255, 255, 255, 0.05);
		color: white;
	}

	/* Active filter tags */
	.active-filters {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.filter-tag {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 9999px;
		font-size: 0.75rem;
		color: #f97316;
	}

	.filter-tag button {
		background: none;
		border: none;
		padding: 2px;
		cursor: pointer;
		color: inherit;
		display: flex;
		align-items: center;
		justify-content: center;
		opacity: 0.7;
		transition: opacity 0.2s ease;
	}

	.filter-tag button:hover {
		opacity: 1;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.filters-panel:not(.mobile) {
			flex-direction: column;
		}

		.filter-group {
			width: 100%;
		}
	}
</style>
