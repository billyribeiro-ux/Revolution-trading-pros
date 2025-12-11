<script lang="ts">
	/**
	 * Learning Center Page - Room-Specific Learning Content
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/[slug]/learning-center
	 * Shows course modules, tutorials, and educational content for a specific
	 * trading room or alert service.
	 *
	 * Uses centralized learningCenter store for room-specific content.
	 *
	 * @version 2.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import { learningCenterStore, learningCenterHelpers } from '$lib/stores/learningCenter';
	import type { TradingRoom, LessonWithRelations, LessonModule, LessonCategory, Trainer, UserRoomProgress, LessonFilter } from '$lib/types/learning-center';
	import { get } from 'svelte/store';
	import {
		LessonCard,
		ModuleAccordion,
		LessonFilters,
		ProgressTracker,
		TrainerCard
	} from '$lib/components/learning-center';
	import {
		IconBook,
		IconPlayerPlay,
		IconGridDots,
		IconList,
		IconChevronRight
	} from '$lib/icons';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let viewMode = $state<'modules' | 'grid'>('modules');
	let currentFilter = $state<LessonFilter>({});
	let expandedModules = $state<Set<string>>(new Set());
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// STORE DATA
	// ═══════════════════════════════════════════════════════════════════════════

	// Get store data
	let storeData = $derived(get(learningCenterStore));

	// Find the trading room by slug
	let tradingRoom = $derived.by((): TradingRoom | undefined => {
		return storeData.tradingRooms.find(r => r.slug === slug);
	});

	// Get room ID
	let roomId = $derived(tradingRoom?.id || '');

	// Get lessons for this room
	let roomLessons = $derived.by((): LessonWithRelations[] => {
		if (!roomId) return [];
		return learningCenterHelpers.getLessonsForRoom(roomId);
	});

	// Get modules for this room
	let roomModules = $derived.by((): LessonModule[] => {
		if (!roomId) return [];
		return learningCenterHelpers.getModulesForRoom(roomId);
	});

	// Get categories from store
	let categories = $derived(storeData.categories);

	// Get trainers from store
	let trainers = $derived(storeData.trainers);

	// Get user progress for this room
	let userProgress = $derived.by((): UserRoomProgress | undefined => {
		if (!roomId) return undefined;
		return storeData.userProgress[roomId];
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FILTERED DATA
	// ═══════════════════════════════════════════════════════════════════════════

	// Apply filters to lessons
	let filteredLessons = $derived.by((): LessonWithRelations[] => {
		if (!currentFilter.search && !currentFilter.categoryId && !currentFilter.trainerId && !currentFilter.type) {
			return roomLessons;
		}
		return learningCenterHelpers.filterLessons({ ...currentFilter, tradingRoomId: roomId });
	});

	// Group lessons by module for accordion view
	let lessonsByModule = $derived.by(() => {
		const grouped = new Map<string, LessonWithRelations[]>();

		// Initialize with modules
		roomModules.forEach(module => {
			grouped.set(module.id, []);
		});

		// Add "Uncategorized" for lessons without a module
		grouped.set('uncategorized', []);

		// Group lessons
		filteredLessons.forEach(lesson => {
			const moduleId = lesson.moduleId || 'uncategorized';
			const existing = grouped.get(moduleId) || [];
			existing.push(lesson);
			grouped.set(moduleId, existing);
		});

		return grouped;
	});

	// Get module progress
	let moduleProgress = $derived.by(() => {
		return userProgress?.moduleProgress || [];
	});

	// Featured lessons (for sidebar)
	let featuredLessons = $derived(roomLessons.filter(l => l.isFeatured).slice(0, 3));

	// Recent lessons (continue learning)
	let recentLessons = $derived.by(() => {
		return roomLessons
			.filter(l => l.userProgress && l.userProgress.progressPercent > 0 && !l.userProgress.isCompleted)
			.sort((a, b) => {
				const aTime = a.userProgress?.lastViewedAt || '';
				const bTime = b.userProgress?.lastViewedAt || '';
				return bTime.localeCompare(aTime);
			})
			.slice(0, 3);
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// COMPUTED VALUES
	// ═══════════════════════════════════════════════════════════════════════════

	let pageTitle = $derived(tradingRoom?.name || 'Learning Center');
	let overallProgress = $derived(userProgress?.progressPercent || 0);
	let completedLessons = $derived(userProgress?.completedLessons || 0);
	let totalLessons = $derived(userProgress?.totalLessons || roomLessons.length);

	// ═══════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleFilterChange(filter: LessonFilter) {
		currentFilter = filter;
	}

	function handleModuleToggle(moduleId: string) {
		const newExpanded = new Set(expandedModules);
		if (newExpanded.has(moduleId)) {
			newExpanded.delete(moduleId);
		} else {
			newExpanded.add(moduleId);
		}
		expandedModules = newExpanded;
	}

	async function handleBookmark(lessonId: string) {
		if (!roomId) return;
		try {
			await learningCenterHelpers.toggleBookmark(lessonId, roomId);
		} catch (err) {
			console.error('Failed to toggle bookmark:', err);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser) {
			// Mark as loaded after initial render
			isLoading = false;

			// Expand first module by default
			if (roomModules.length > 0 && expandedModules.size === 0) {
				expandedModules = new Set([roomModules[0].id]);
			}
		}
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>{pageTitle} - Learning Center | Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<nav class="dashboard__breadcrumb" aria-label="Breadcrumb">
			<a href="/dashboard">Dashboard</a>
			<span class="separator">/</span>
			<a href="/dashboard/{slug}">{tradingRoom?.shortName || pageTitle}</a>
			<span class="separator">/</span>
			<span class="current">Learning Center</span>
		</nav>
		<h1 class="dashboard__page-title">
			<span class="st-icon-learning-center"></span>
			Learning Center
		</h1>
		{#if tradingRoom?.description}
			<p class="dashboard__subtitle">{tradingRoom.description}</p>
		{/if}
	</div>
	<div class="dashboard__header-right">
		<div class="overall-progress">
			<span class="progress-label">Overall Progress</span>
			<div class="progress-bar">
				<div class="progress-bar__fill" style="width: {overallProgress}%"></div>
			</div>
			<span class="progress-value">{overallProgress}%</span>
		</div>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     SECONDARY NAVIGATION
     ═══════════════════════════════════════════════════════════════════════════ -->

<nav class="dashboard__nav-secondary">
	<ul class="nav-menu">
		<li class="nav-item">
			<a href="/dashboard/{slug}">
				<span class="st-icon-dashboard nav-icon"></span>
				<span class="nav-text">Dashboard</span>
			</a>
		</li>
		<li class="nav-item is-active">
			<a href="/dashboard/{slug}/learning-center">
				<span class="st-icon-learning-center nav-icon"></span>
				<span class="nav-text">Learning Center</span>
			</a>
		</li>
		{#if tradingRoom?.hasArchive}
			<li class="nav-item">
				<a href="/dashboard/{slug}/archive">
					<span class="st-icon-chatroom-archive nav-icon"></span>
					<span class="nav-text">Video Archive</span>
				</a>
			</li>
		{/if}
	</ul>
</nav>

<!-- ═══════════════════════════════════════════════════════════════════════════
     CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

{#if !tradingRoom}
	<div class="error-state">
		<IconBook size={48} />
		<h3>Trading Room Not Found</h3>
		<p>The trading room "{slug}" doesn't exist or doesn't have a learning center.</p>
		<a href="/dashboard" class="btn-back">Back to Dashboard</a>
	</div>
{:else}
	<div class="dashboard__content">
		<div class="content-layout">
			<!-- Main Content -->
			<main class="main-content">
				<!-- Filters and View Toggle -->
				<div class="toolbar">
					<LessonFilters
						{categories}
						{trainers}
						{currentFilter}
						onFilterChange={handleFilterChange}
						compact
					/>

					<div class="view-toggle">
						<button
							type="button"
							class="toggle-btn"
							class:active={viewMode === 'modules'}
							onclick={() => viewMode = 'modules'}
							aria-label="Module view"
						>
							<IconList size={18} />
						</button>
						<button
							type="button"
							class="toggle-btn"
							class:active={viewMode === 'grid'}
							onclick={() => viewMode = 'grid'}
							aria-label="Grid view"
						>
							<IconGridDots size={18} />
						</button>
					</div>
				</div>

				<!-- Module View -->
				{#if viewMode === 'modules'}
					<div class="modules-list">
						{#each roomModules as module (module.id)}
							{@const moduleLessons = lessonsByModule.get(module.id) || []}
							{@const modProgress = moduleProgress.find(p => p.moduleId === module.id)}
							{#if moduleLessons.length > 0}
								<ModuleAccordion
									{module}
									lessons={moduleLessons}
									progress={modProgress}
									isExpanded={expandedModules.has(module.id)}
									onToggle={handleModuleToggle}
									onBookmark={handleBookmark}
									basePath="/dashboard/{slug}/learning-center"
								/>
							{/if}
						{/each}

						<!-- Uncategorized lessons -->
						{@const uncategorizedLessons = lessonsByModule.get('uncategorized') || []}
						{#if uncategorizedLessons.length > 0}
							<ModuleAccordion
								module={{
									id: 'uncategorized',
									slug: 'uncategorized',
									tradingRoomId: roomId,
									name: 'Additional Lessons',
									description: 'More educational content',
									sortOrder: 999
								}}
								lessons={uncategorizedLessons}
								isExpanded={expandedModules.has('uncategorized')}
								onToggle={handleModuleToggle}
								onBookmark={handleBookmark}
								basePath="/dashboard/{slug}/learning-center"
							/>
						{/if}

						{#if filteredLessons.length === 0}
							<div class="empty-state">
								<IconBook size={48} />
								<h3>No lessons found</h3>
								<p>Try adjusting your search or filter criteria</p>
							</div>
						{/if}
					</div>
				{/if}

				<!-- Grid View -->
				{#if viewMode === 'grid'}
					<div class="lessons-grid">
						{#each filteredLessons as lesson (lesson.id)}
							<LessonCard
								{lesson}
								showProgress
								showTrainer
								showCategory
								href="/dashboard/{slug}/learning-center/{lesson.slug}"
								onBookmark={handleBookmark}
							/>
						{/each}

						{#if filteredLessons.length === 0}
							<div class="empty-state full-width">
								<IconBook size={48} />
								<h3>No lessons found</h3>
								<p>Try adjusting your search or filter criteria</p>
							</div>
						{/if}
					</div>
				{/if}
			</main>

			<!-- Sidebar -->
			<aside class="sidebar">
				<!-- Progress Tracker -->
				<ProgressTracker
					progress={userProgress}
					recentLessons={recentLessons}
					showStats
					showRecentActivity
				/>

				<!-- Featured Lessons -->
				{#if featuredLessons.length > 0}
					<div class="sidebar-section">
						<h3 class="sidebar-title">Featured Lessons</h3>
						<div class="featured-list">
							{#each featuredLessons as lesson}
								<a href="/dashboard/{slug}/learning-center/{lesson.slug}" class="featured-item">
									<div class="featured-thumb">
										{#if lesson.thumbnailUrl}
											<img src={lesson.thumbnailUrl} alt={lesson.title} />
										{/if}
										<div class="play-icon">
											<IconPlayerPlay size={16} />
										</div>
									</div>
									<div class="featured-info">
										<span class="featured-title">{lesson.title}</span>
										{#if lesson.duration}
											<span class="featured-duration">{lesson.duration}</span>
										{/if}
									</div>
								</a>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Trainers -->
				{#if trainers.length > 0}
					<div class="sidebar-section">
						<h3 class="sidebar-title">Instructors</h3>
						<div class="trainers-list">
							{#each trainers.slice(0, 3) as trainer}
								{@const trainerLessons = roomLessons.filter(l => l.trainerId === trainer.id).length}
								{#if trainerLessons > 0}
									<TrainerCard
										{trainer}
										variant="compact"
										lessonCount={trainerLessons}
									/>
								{/if}
							{/each}
						</div>
					</div>
				{/if}
			</aside>
		</div>
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* Dashboard Header */
	.dashboard__header {
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
		border-bottom: 1px solid #334155;
		padding: 24px 32px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
	}

	.dashboard__header-left {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.dashboard__breadcrumb {
		font-size: 13px;
		color: #94a3b8;
	}

	.dashboard__breadcrumb a {
		color: #94a3b8;
		text-decoration: none;
		transition: color 0.2s;
	}

	.dashboard__breadcrumb a:hover {
		color: #f97316;
	}

	.dashboard__breadcrumb .separator {
		margin: 0 8px;
		color: #64748b;
	}

	.dashboard__breadcrumb .current {
		color: white;
	}

	.dashboard__page-title {
		display: flex;
		align-items: center;
		gap: 12px;
		color: white;
		font-size: 28px;
		font-weight: 700;
		margin: 0;
	}

	.dashboard__page-title .st-icon-learning-center {
		font-size: 28px;
		color: #f97316;
	}

	.dashboard__subtitle {
		margin: 4px 0 0;
		font-size: 14px;
		color: #94a3b8;
	}

	/* Overall Progress */
	.overall-progress {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.progress-label {
		font-size: 14px;
		color: #94a3b8;
	}

	.progress-bar {
		width: 120px;
		height: 8px;
		background: #334155;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-bar__fill {
		height: 100%;
		background: linear-gradient(90deg, #f97316, #ea580c);
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.progress-value {
		font-size: 14px;
		font-weight: 700;
		color: #f97316;
	}

	/* Secondary Navigation */
	.dashboard__nav-secondary {
		background: #1e293b;
		border-bottom: 1px solid #334155;
		padding: 0 32px;
	}

	.nav-menu {
		display: flex;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.nav-item a {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 14px 20px;
		color: #94a3b8;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		border-bottom: 3px solid transparent;
		transition: all 0.15s ease;
	}

	.nav-item a:hover {
		color: white;
	}

	.nav-item.is-active a {
		color: #f97316;
		border-bottom-color: #f97316;
	}

	.nav-icon {
		font-size: 18px;
	}

	/* Content Layout */
	.dashboard__content {
		padding: 32px;
		background: #0f172a;
		min-height: calc(100vh - 200px);
	}

	.content-layout {
		display: grid;
		grid-template-columns: 1fr 320px;
		gap: 32px;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Main Content */
	.main-content {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 16px;
		flex-wrap: wrap;
	}

	.view-toggle {
		display: flex;
		background: #1e293b;
		border-radius: 8px;
		padding: 4px;
	}

	.toggle-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: none;
		border: none;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.toggle-btn:hover {
		color: white;
	}

	.toggle-btn.active {
		background: #f97316;
		color: white;
	}

	/* Modules List */
	.modules-list {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	/* Lessons Grid */
	.lessons-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
	}

	/* Sidebar */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.sidebar-section {
		background: #1e293b;
		border-radius: 12px;
		padding: 20px;
		border: 1px solid #334155;
	}

	.sidebar-title {
		margin: 0 0 16px;
		font-size: 14px;
		font-weight: 600;
		color: white;
	}

	/* Featured Lessons */
	.featured-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.featured-item {
		display: flex;
		gap: 12px;
		text-decoration: none;
		padding: 8px;
		margin: -8px;
		border-radius: 8px;
		transition: background 0.2s ease;
	}

	.featured-item:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.featured-thumb {
		position: relative;
		width: 80px;
		height: 45px;
		border-radius: 6px;
		overflow: hidden;
		background: #334155;
		flex-shrink: 0;
	}

	.featured-thumb img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.play-icon {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.4);
		color: white;
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	.featured-item:hover .play-icon {
		opacity: 1;
	}

	.featured-info {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.featured-title {
		font-size: 13px;
		font-weight: 500;
		color: white;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.featured-duration {
		font-size: 12px;
		color: #64748b;
		margin-top: 4px;
	}

	/* Trainers List */
	.trainers-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #64748b;
	}

	.empty-state.full-width {
		grid-column: 1 / -1;
	}

	.empty-state h3 {
		margin: 16px 0 8px;
		color: white;
		font-size: 1.125rem;
	}

	.empty-state p {
		margin: 0;
		font-size: 0.875rem;
	}

	/* Error State */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		text-align: center;
		padding: 40px;
		color: #64748b;
	}

	.error-state h3 {
		margin: 16px 0 8px;
		color: white;
		font-size: 1.25rem;
	}

	.error-state p {
		margin: 0 0 24px;
	}

	.btn-back {
		display: inline-flex;
		padding: 10px 20px;
		background: #f97316;
		color: white;
		text-decoration: none;
		border-radius: 8px;
		font-weight: 500;
		transition: background 0.2s ease;
	}

	.btn-back:hover {
		background: #ea580c;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.content-layout {
			grid-template-columns: 1fr;
		}

		.sidebar {
			display: grid;
			grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		}
	}

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 22px;
		}

		.overall-progress {
			flex-wrap: wrap;
		}

		.dashboard__nav-secondary {
			padding: 0 16px;
			overflow-x: auto;
		}

		.dashboard__content {
			padding: 16px;
		}

		.toolbar {
			flex-direction: column;
			align-items: stretch;
		}

		.view-toggle {
			align-self: flex-end;
		}

		.lessons-grid {
			grid-template-columns: 1fr;
		}

		.sidebar {
			grid-template-columns: 1fr;
		}
	}
</style>
