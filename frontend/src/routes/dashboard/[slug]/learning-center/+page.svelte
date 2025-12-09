<script lang="ts">
	/**
	 * Learning Center Page - WordPress Simpler Trading Exact
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/mastering-the-trade/learning-center
	 * Shows course modules, tutorials, and educational content for a membership.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconLock from '@tabler/icons-svelte/icons/lock';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconFilter from '@tabler/icons-svelte/icons/filter';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let selectedCategory = $state('all');

	// ═══════════════════════════════════════════════════════════════════════════
	// CATEGORIES
	// ═══════════════════════════════════════════════════════════════════════════

	const categories = [
		{ id: 'all', name: 'All Content' },
		{ id: 'getting-started', name: 'Getting Started' },
		{ id: 'strategies', name: 'Trading Strategies' },
		{ id: 'technical', name: 'Technical Analysis' },
		{ id: 'risk', name: 'Risk Management' },
		{ id: 'psychology', name: 'Trading Psychology' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// COURSE MODULES (Mock data)
	// ═══════════════════════════════════════════════════════════════════════════

	interface Lesson {
		id: number;
		title: string;
		duration: string;
		completed: boolean;
		locked: boolean;
	}

	interface CourseModule {
		id: number;
		title: string;
		description: string;
		category: string;
		lessons: Lesson[];
		totalDuration: string;
		progress: number;
	}

	const courseModules: CourseModule[] = [
		{
			id: 1,
			title: 'Welcome & Getting Started',
			description: 'Introduction to the trading room and platform setup',
			category: 'getting-started',
			totalDuration: '45 min',
			progress: 100,
			lessons: [
				{ id: 1, title: 'Welcome to Revolution Trading', duration: '10:00', completed: true, locked: false },
				{ id: 2, title: 'Platform Setup Guide', duration: '15:00', completed: true, locked: false },
				{ id: 3, title: 'Trading Room Navigation', duration: '12:00', completed: true, locked: false },
				{ id: 4, title: 'Setting Up Your Workspace', duration: '8:00', completed: true, locked: false }
			]
		},
		{
			id: 2,
			title: 'Core Trading Strategies',
			description: 'Learn the fundamental strategies used in our trading room',
			category: 'strategies',
			totalDuration: '2h 30min',
			progress: 60,
			lessons: [
				{ id: 5, title: 'Understanding Market Structure', duration: '25:00', completed: true, locked: false },
				{ id: 6, title: 'Entry & Exit Techniques', duration: '30:00', completed: true, locked: false },
				{ id: 7, title: 'Position Sizing Fundamentals', duration: '20:00', completed: true, locked: false },
				{ id: 8, title: 'Advanced Entry Patterns', duration: '35:00', completed: false, locked: false },
				{ id: 9, title: 'Multi-Timeframe Analysis', duration: '40:00', completed: false, locked: false }
			]
		},
		{
			id: 3,
			title: 'Technical Analysis Deep Dive',
			description: 'Master chart reading and technical indicators',
			category: 'technical',
			totalDuration: '3h 15min',
			progress: 25,
			lessons: [
				{ id: 10, title: 'Candlestick Patterns Mastery', duration: '45:00', completed: true, locked: false },
				{ id: 11, title: 'Support & Resistance Levels', duration: '35:00', completed: false, locked: false },
				{ id: 12, title: 'Moving Averages Explained', duration: '30:00', completed: false, locked: false },
				{ id: 13, title: 'Volume Analysis', duration: '40:00', completed: false, locked: false },
				{ id: 14, title: 'Advanced Indicators', duration: '45:00', completed: false, locked: false }
			]
		},
		{
			id: 4,
			title: 'Risk Management Essentials',
			description: 'Protect your capital with proper risk management',
			category: 'risk',
			totalDuration: '1h 45min',
			progress: 0,
			lessons: [
				{ id: 15, title: 'Risk-Reward Fundamentals', duration: '25:00', completed: false, locked: false },
				{ id: 16, title: 'Stop Loss Strategies', duration: '30:00', completed: false, locked: false },
				{ id: 17, title: 'Portfolio Management', duration: '25:00', completed: false, locked: false },
				{ id: 18, title: 'Drawdown Recovery', duration: '25:00', completed: false, locked: false }
			]
		},
		{
			id: 5,
			title: 'Trading Psychology',
			description: 'Develop the mindset of a professional trader',
			category: 'psychology',
			totalDuration: '2h',
			progress: 0,
			lessons: [
				{ id: 19, title: 'Emotional Discipline', duration: '30:00', completed: false, locked: true },
				{ id: 20, title: 'Overcoming Fear & Greed', duration: '25:00', completed: false, locked: true },
				{ id: 21, title: 'Building Trading Confidence', duration: '35:00', completed: false, locked: true },
				{ id: 22, title: 'Daily Routine for Success', duration: '30:00', completed: false, locked: true }
			]
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredModules = $derived.by(() => {
		return courseModules.filter(module => {
			const matchesCategory = selectedCategory === 'all' || module.category === selectedCategory;
			const matchesSearch = searchQuery === '' ||
				module.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				module.lessons.some(l => l.title.toLowerCase().includes(searchQuery.toLowerCase()));
			return matchesCategory && matchesSearch;
		});
	});

	const overallProgress = $derived.by(() => {
		const totalLessons = courseModules.reduce((acc, m) => acc + m.lessons.length, 0);
		const completedLessons = courseModules.reduce((acc, m) => acc + m.lessons.filter(l => l.completed).length, 0);
		return Math.round((completedLessons / totalLessons) * 100);
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Learning Center | Dashboard | Revolution Trading Pros</title>
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
			<a href="/dashboard/{slug}">{slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ')}</a>
			<span class="separator">/</span>
			<span class="current">Learning Center</span>
		</nav>
		<h1 class="dashboard__page-title">
			<span class="st-icon-learning-center"></span>
			Learning Center
		</h1>
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
		<li class="nav-item">
			<a href="/dashboard/{slug}/archive">
				<span class="st-icon-chatroom-archive nav-icon"></span>
				<span class="nav-text">Video Archive</span>
			</a>
		</li>
	</ul>
</nav>

<!-- ═══════════════════════════════════════════════════════════════════════════
     CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<!-- Filters Bar -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search lessons..."
				bind:value={searchQuery}
			/>
		</div>
		<div class="category-filters">
			{#each categories as category}
				<button
					class="filter-btn"
					class:active={selectedCategory === category.id}
					onclick={() => selectedCategory = category.id}
				>
					{category.name}
				</button>
			{/each}
		</div>
	</div>

	<!-- Course Modules -->
	<div class="modules-list">
		{#each filteredModules as module (module.id)}
			<div class="module-card">
				<div class="module-header">
					<div class="module-info">
						<h2 class="module-title">{module.title}</h2>
						<p class="module-description">{module.description}</p>
						<div class="module-meta">
							<span class="meta-item">
								<IconBook size={14} />
								{module.lessons.length} lessons
							</span>
							<span class="meta-item">
								<IconClock size={14} />
								{module.totalDuration}
							</span>
						</div>
					</div>
					<div class="module-progress">
						<div class="progress-circle" style="--progress: {module.progress}">
							<span>{module.progress}%</span>
						</div>
					</div>
				</div>

				<div class="lessons-list">
					{#each module.lessons as lesson (lesson.id)}
						<a
							href="/dashboard/{slug}/learning-center/{lesson.id}"
							class="lesson-item"
							class:completed={lesson.completed}
							class:locked={lesson.locked}
						>
							<div class="lesson-status">
								{#if lesson.completed}
									<IconCheck size={18} />
								{:else if lesson.locked}
									<IconLock size={18} />
								{:else}
									<IconPlayerPlay size={18} />
								{/if}
							</div>
							<div class="lesson-info">
								<span class="lesson-title">{lesson.title}</span>
								<span class="lesson-duration">{lesson.duration}</span>
							</div>
							<IconChevronRight size={18} class="lesson-arrow" />
						</a>
					{/each}
				</div>
			</div>
		{/each}

		{#if filteredModules.length === 0}
			<div class="empty-state">
				<IconBook size={48} />
				<h3>No lessons found</h3>
				<p>Try adjusting your search or filter criteria</p>
			</div>
		{/if}
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* Dashboard Header */
	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid var(--st-border-color, #dbdbdb);
		padding: 20px 30px;
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
		color: var(--st-text-muted, #64748b);
	}

	.dashboard__breadcrumb a {
		color: var(--st-link-color, #1e73be);
		text-decoration: none;
	}

	.dashboard__breadcrumb a:hover {
		text-decoration: underline;
	}

	.dashboard__breadcrumb .separator {
		margin: 0 8px;
		color: #999;
	}

	.dashboard__page-title {
		display: flex;
		align-items: center;
		gap: 12px;
		color: var(--st-text-color, #333);
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 32px;
		font-weight: 700;
		margin: 0;
	}

	.dashboard__page-title .st-icon-learning-center {
		font-size: 32px;
		color: var(--st-primary, #0984ae);
	}

	/* Overall Progress */
	.overall-progress {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.progress-label {
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
	}

	.progress-bar {
		width: 120px;
		height: 8px;
		background: #e5e7eb;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-bar__fill {
		height: 100%;
		background: linear-gradient(90deg, var(--st-primary, #0984ae), #0ea5e9);
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.progress-value {
		font-size: 14px;
		font-weight: 700;
		color: var(--st-primary, #0984ae);
	}

	/* Secondary Navigation */
	.dashboard__nav-secondary {
		background: #fff;
		border-bottom: 1px solid var(--st-border-color, #dbdbdb);
		padding: 0 30px;
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
		padding: 16px 20px;
		color: var(--st-text-muted, #64748b);
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		border-bottom: 3px solid transparent;
		transition: all 0.15s ease;
	}

	.nav-item a:hover {
		color: var(--st-primary, #0984ae);
	}

	.nav-item.is-active a {
		color: var(--st-primary, #0984ae);
		border-bottom-color: var(--st-primary, #0984ae);
	}

	.nav-icon {
		font-size: 18px;
	}

	/* Content */
	.dashboard__content {
		padding: 30px;
		background: #f8f9fa;
		min-height: calc(100vh - 200px);
	}

	/* Filters Bar */
	.filters-bar {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 24px;
		align-items: center;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		flex: 1;
		max-width: 300px;
		color: var(--st-text-muted, #64748b);
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 14px;
		background: transparent;
	}

	.category-filters {
		display: flex;
		gap: 8px;
		flex-wrap: wrap;
	}

	.filter-btn {
		padding: 8px 16px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 20px;
		font-size: 13px;
		font-weight: 500;
		color: var(--st-text-muted, #64748b);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.filter-btn:hover {
		border-color: var(--st-primary, #0984ae);
		color: var(--st-primary, #0984ae);
	}

	.filter-btn.active {
		background: var(--st-primary, #0984ae);
		border-color: var(--st-primary, #0984ae);
		color: #fff;
	}

	/* Modules List */
	.modules-list {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.module-card {
		background: #fff;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
		overflow: hidden;
	}

	.module-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 24px;
		border-bottom: 1px solid #f1f5f9;
	}

	.module-info {
		flex: 1;
	}

	.module-title {
		font-size: 20px;
		font-weight: 700;
		color: var(--st-text-color, #333);
		margin: 0 0 8px;
	}

	.module-description {
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
		margin: 0 0 12px;
	}

	.module-meta {
		display: flex;
		gap: 16px;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 13px;
		color: var(--st-text-muted, #64748b);
	}

	/* Progress Circle */
	.module-progress {
		flex-shrink: 0;
	}

	.progress-circle {
		width: 64px;
		height: 64px;
		border-radius: 50%;
		background: conic-gradient(
			var(--st-primary, #0984ae) calc(var(--progress) * 1%),
			#e5e7eb calc(var(--progress) * 1%)
		);
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
	}

	.progress-circle::before {
		content: '';
		position: absolute;
		width: 52px;
		height: 52px;
		background: #fff;
		border-radius: 50%;
	}

	.progress-circle span {
		position: relative;
		font-size: 14px;
		font-weight: 700;
		color: var(--st-text-color, #333);
	}

	/* Lessons List */
	.lessons-list {
		display: flex;
		flex-direction: column;
	}

	.lesson-item {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 16px 24px;
		border-bottom: 1px solid #f1f5f9;
		text-decoration: none;
		transition: background 0.15s ease;
	}

	.lesson-item:last-child {
		border-bottom: none;
	}

	.lesson-item:hover {
		background: #f8f9fa;
	}

	.lesson-item.completed {
		opacity: 0.7;
	}

	.lesson-item.locked {
		opacity: 0.5;
		pointer-events: none;
	}

	.lesson-status {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		background: #f1f5f9;
		color: var(--st-text-muted, #64748b);
	}

	.lesson-item.completed .lesson-status {
		background: #10b981;
		color: #fff;
	}

	.lesson-item.locked .lesson-status {
		background: #e5e7eb;
		color: #999;
	}

	.lesson-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.lesson-title {
		font-size: 15px;
		font-weight: 500;
		color: var(--st-text-color, #333);
	}

	.lesson-duration {
		font-size: 13px;
		color: var(--st-text-muted, #64748b);
	}

	:global(.lesson-arrow) {
		color: #ccc;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: var(--st-text-muted, #64748b);
	}

	.empty-state h3 {
		margin: 16px 0 8px;
		color: var(--st-text-color, #333);
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 24px;
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

		.filters-bar {
			flex-direction: column;
			align-items: stretch;
		}

		.search-box {
			max-width: none;
		}

		.module-header {
			flex-direction: column;
			gap: 16px;
		}

		.lesson-item {
			padding: 12px 16px;
		}
	}
</style>
