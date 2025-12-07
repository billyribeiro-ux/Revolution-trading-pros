<script lang="ts">
	/**
	 * Dashboard - My Courses Page - WordPress Simpler Trading Exact
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Shows user's enrolled courses with progress tracking.
	 * Matches WordPress Simpler Trading dashboard structure exactly.
	 *
	 * @version 2.0.0 (WordPress Exact / December 2025)
	 */

	import {
		IconBook,
		IconClock,
		IconPlayerPlay,
		IconCheck,
		IconChevronRight,
		IconSearch
	} from '@tabler/icons-svelte';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let filterStatus = $state<'all' | 'in-progress' | 'completed'>('all');

	// ═══════════════════════════════════════════════════════════════════════════
	// ENROLLED COURSES (Mock data - will be fetched from API)
	// ═══════════════════════════════════════════════════════════════════════════

	interface EnrolledCourse {
		id: number;
		title: string;
		description: string;
		instructor: string;
		progress: number;
		totalLessons: number;
		completedLessons: number;
		thumbnail?: string;
		lastAccessed: string;
		slug: string;
	}

	const enrolledCourses: EnrolledCourse[] = [
		{
			id: 1,
			title: 'Options Trading Mastery',
			description: 'Complete guide to options trading strategies from basics to advanced techniques.',
			instructor: 'John Carter',
			progress: 65,
			totalLessons: 24,
			completedLessons: 16,
			lastAccessed: '2 days ago',
			slug: 'options-trading-mastery'
		},
		{
			id: 2,
			title: 'Technical Analysis Fundamentals',
			description: 'Learn to read charts, identify patterns, and make data-driven trading decisions.',
			instructor: 'Sarah Mitchell',
			progress: 100,
			totalLessons: 18,
			completedLessons: 18,
			lastAccessed: '1 week ago',
			slug: 'technical-analysis-fundamentals'
		},
		{
			id: 3,
			title: 'Day Trading Strategies',
			description: 'Master intraday trading techniques with proven scalping and momentum strategies.',
			instructor: 'Mike Thompson',
			progress: 30,
			totalLessons: 20,
			completedLessons: 6,
			lastAccessed: 'Today',
			slug: 'day-trading-strategies'
		},
		{
			id: 4,
			title: 'Risk Management Essentials',
			description: 'Protect your capital with proper position sizing and risk control techniques.',
			instructor: 'Lisa Chen',
			progress: 0,
			totalLessons: 12,
			completedLessons: 0,
			lastAccessed: 'Not started',
			slug: 'risk-management-essentials'
		},
		{
			id: 5,
			title: 'Swing Trading for Beginners',
			description: 'Learn to capture multi-day price movements with swing trading strategies.',
			instructor: 'David Park',
			progress: 45,
			totalLessons: 16,
			completedLessons: 7,
			lastAccessed: '3 days ago',
			slug: 'swing-trading-beginners'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredCourses = $derived.by(() => {
		return enrolledCourses.filter(course => {
			// Search filter
			const matchesSearch = searchQuery === '' ||
				course.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				course.instructor.toLowerCase().includes(searchQuery.toLowerCase());

			// Status filter
			let matchesStatus = true;
			if (filterStatus === 'in-progress') {
				matchesStatus = course.progress > 0 && course.progress < 100;
			} else if (filterStatus === 'completed') {
				matchesStatus = course.progress === 100;
			}

			return matchesSearch && matchesStatus;
		});
	});

	const stats = $derived({
		total: enrolledCourses.length,
		inProgress: enrolledCourses.filter(c => c.progress > 0 && c.progress < 100).length,
		completed: enrolledCourses.filter(c => c.progress === 100).length
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>My Courses | Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER - WordPress: .dashboard__header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">
			<span class="st-icon-courses"></span>
			My Courses
		</h1>
		<div class="dashboard__stats">
			<span class="stat-item">
				<strong>{stats.total}</strong> enrolled
			</span>
			<span class="stat-item stat-progress">
				<strong>{stats.inProgress}</strong> in progress
			</span>
			<span class="stat-item stat-complete">
				<IconCheck size={14} />
				<strong>{stats.completed}</strong> completed
			</span>
		</div>
	</div>
	<div class="dashboard__header-right">
		<a href="/courses" class="btn btn-link">
			Browse All Courses
			<IconChevronRight size={16} />
		</a>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD CONTENT - WordPress: .dashboard__content
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<!-- Filters Bar -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				id="course-search"
				type="text"
				placeholder="Search courses..."
				bind:value={searchQuery}
			/>
		</div>
		<div class="filter-tabs">
			<button
				class="filter-tab"
				class:active={filterStatus === 'all'}
				onclick={() => filterStatus = 'all'}
			>
				All ({stats.total})
			</button>
			<button
				class="filter-tab"
				class:active={filterStatus === 'in-progress'}
				onclick={() => filterStatus = 'in-progress'}
			>
				In Progress ({stats.inProgress})
			</button>
			<button
				class="filter-tab"
				class:active={filterStatus === 'completed'}
				onclick={() => filterStatus = 'completed'}
			>
				Completed ({stats.completed})
			</button>
		</div>
	</div>

	<!-- Courses Grid - WordPress: .membership-cards.row -->
	{#if filteredCourses.length > 0}
		<section class="dashboard__content-section">
			<div class="courses-grid row">
				{#each filteredCourses as course (course.id)}
					<div class="col-sm-6 col-xl-4">
						<article class="course-card" class:is-complete={course.progress === 100}>
							<div class="course-card__thumbnail">
								<div class="course-card__placeholder">
									<IconBook size={48} />
								</div>
								{#if course.progress === 100}
									<div class="course-card__badge course-card__badge--complete">
										<IconCheck size={14} />
										Completed
									</div>
								{:else if course.progress > 0}
									<div class="course-card__badge">
										{course.progress}% Complete
									</div>
								{:else}
									<div class="course-card__badge course-card__badge--new">
										Not Started
									</div>
								{/if}
							</div>

							<div class="course-card__content">
								<h3 class="course-card__title">{course.title}</h3>
								<p class="course-card__instructor">by {course.instructor}</p>
								<p class="course-card__description">{course.description}</p>

								<div class="course-card__progress">
									<div class="progress-bar">
										<div class="progress-bar__fill" style="width: {course.progress}%"></div>
									</div>
									<span class="progress-text">{course.completedLessons}/{course.totalLessons} lessons</span>
								</div>

								<div class="course-card__meta">
									<span class="meta-item">
										<IconClock size={14} />
										{course.lastAccessed}
									</span>
								</div>
							</div>

							<div class="course-card__actions">
								<a href="/dashboard/courses/{course.slug}">
									{#if course.progress === 100}
										Review Course
									{:else if course.progress > 0}
										Continue Learning
									{:else}
										Start Course
									{/if}
								</a>
								<a href="/dashboard/courses/{course.slug}/lessons">
									<IconPlayerPlay size={16} />
									Lessons
								</a>
							</div>
						</article>
					</div>
				{/each}
			</div>
		</section>
	{:else}
		<div class="empty-state">
			<IconBook size={64} />
			<h2>No Courses Found</h2>
			{#if searchQuery || filterStatus !== 'all'}
				<p>Try adjusting your search or filter criteria</p>
				<button class="btn btn-primary" onclick={() => { searchQuery = ''; filterStatus = 'all'; }}>
					Clear Filters
				</button>
			{:else}
				<p>You haven't enrolled in any courses yet. Browse our catalog to get started!</p>
				<a href="/courses" class="btn btn-orange">Browse Courses</a>
			{/if}
		</div>
	{/if}
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress: .dashboard__header
	   ═══════════════════════════════════════════════════════════════════════════ */

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

	.dashboard__header-left,
	.dashboard__header-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.dashboard__page-title {
		display: flex;
		align-items: center;
		gap: 12px;
		color: var(--st-text-color, #333);
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
	}

	.dashboard__page-title .st-icon-courses {
		font-size: 32px;
		color: var(--st-primary, #0984ae);
	}

	.dashboard__stats {
		display: flex;
		gap: 16px;
		margin-left: 24px;
	}

	.stat-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
	}

	.stat-item strong {
		color: var(--st-text-color, #333);
	}

	.stat-progress strong {
		color: var(--st-orange, #f99e31);
	}

	.stat-complete {
		color: #10b981;
	}

	.stat-complete strong {
		color: #10b981;
	}

	.btn-link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		color: var(--st-link-color, #1e73be);
		text-decoration: none;
		font-weight: 600;
		font-size: 14px;
	}

	.btn-link:hover {
		color: var(--st-primary, #0984ae);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FILTERS BAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.filters-bar {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 24px;
		align-items: center;
		justify-content: space-between;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #f8f9fa;
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

	.filter-tabs {
		display: flex;
		gap: 8px;
	}

	.filter-tab {
		padding: 8px 16px;
		background: #f8f9fa;
		border: 1px solid #e5e7eb;
		border-radius: 20px;
		font-size: 13px;
		font-weight: 500;
		color: var(--st-text-muted, #64748b);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.filter-tab:hover {
		border-color: var(--st-primary, #0984ae);
		color: var(--st-primary, #0984ae);
	}

	.filter-tab.active {
		background: var(--st-primary, #0984ae);
		border-color: var(--st-primary, #0984ae);
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT SECTION - WordPress: .dashboard__content-section
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		margin-bottom: 40px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COURSES GRID - Bootstrap Grid
	   ═══════════════════════════════════════════════════════════════════════════ */

	.courses-grid {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

	.courses-grid > .col-sm-6 {
		position: relative;
		width: 100%;
		padding-right: 15px;
		padding-left: 15px;
		margin-bottom: 30px;
	}

	@media (min-width: 576px) {
		.courses-grid > .col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.courses-grid > .col-xl-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COURSE CARD - Similar to membership-card
	   ═══════════════════════════════════════════════════════════════════════════ */

	.course-card {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgb(0 0 0 / 10%);
		transition: all 0.15s ease-in-out;
		overflow: hidden;
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.course-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 10px 40px rgb(0 0 0 / 15%);
	}

	.course-card.is-complete {
		border: 2px solid #10b981;
	}

	/* Thumbnail */
	.course-card__thumbnail {
		position: relative;
		height: 160px;
		background: linear-gradient(135deg, #e0f2fe, #dbeafe);
	}

	.course-card__placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--st-primary, #0984ae);
		opacity: 0.5;
	}

	.course-card__badge {
		position: absolute;
		top: 12px;
		right: 12px;
		padding: 6px 12px;
		background: rgba(0, 0, 0, 0.7);
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
		color: #facc15;
	}

	.course-card__badge--complete {
		display: flex;
		align-items: center;
		gap: 4px;
		background: rgba(16, 185, 129, 0.9);
		color: #fff;
	}

	.course-card__badge--new {
		background: rgba(100, 116, 139, 0.8);
		color: #fff;
	}

	/* Content */
	.course-card__content {
		padding: 20px;
		flex: 1;
	}

	.course-card__title {
		font-size: 18px;
		font-weight: 700;
		color: var(--st-text-color, #333);
		margin: 0 0 4px;
		line-height: 1.3;
	}

	.course-card__instructor {
		font-size: 13px;
		color: var(--st-primary, #0984ae);
		margin: 0 0 8px;
	}

	.course-card__description {
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
		margin: 0 0 16px;
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Progress */
	.course-card__progress {
		margin-bottom: 12px;
	}

	.progress-bar {
		height: 6px;
		background: #e5e7eb;
		border-radius: 3px;
		overflow: hidden;
		margin-bottom: 6px;
	}

	.progress-bar__fill {
		height: 100%;
		background: linear-gradient(90deg, var(--st-primary, #0984ae), #0ea5e9);
		border-radius: 3px;
		transition: width 0.3s ease;
	}

	.progress-text {
		font-size: 12px;
		color: var(--st-text-muted, #64748b);
	}

	/* Meta */
	.course-card__meta {
		display: flex;
		gap: 12px;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 13px;
		color: var(--st-text-muted, #64748b);
	}

	/* Actions - WordPress: .membership-card__actions */
	.course-card__actions {
		display: flex;
		font-size: 16px;
		border-top: 1px solid #ededed;
	}

	.course-card__actions a {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		flex: 1;
		padding: 15px;
		text-align: center;
		color: var(--st-link-color, #1e73be);
		font-weight: 500;
		font-size: 14px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.course-card__actions a:hover {
		background-color: #f4f4f4;
		color: var(--st-primary, #0984ae);
	}

	.course-card__actions a + a {
		border-left: 1px solid #ededed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		text-align: center;
		color: var(--st-text-muted, #64748b);
		padding: 40px 20px;
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: var(--st-text-color, #333);
		margin: 16px 0 8px;
	}

	.empty-state p {
		margin: 0 0 24px;
	}

	.btn-primary {
		padding: 12px 24px;
		background: var(--st-primary, #0984ae);
		border: none;
		border-radius: 8px;
		color: #fff;
		font-weight: 600;
		cursor: pointer;
		text-decoration: none;
	}

	.btn-primary:hover {
		background: #076787;
	}

	.btn-orange {
		display: inline-flex;
		padding: 12px 24px;
		background: var(--st-orange, #f99e31);
		border-radius: 8px;
		color: #fff;
		font-weight: 600;
		text-decoration: none;
	}

	.btn-orange:hover {
		background: var(--st-orange-hover, #dc7309);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__stats {
			display: none;
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

		.filter-tabs {
			overflow-x: auto;
			padding-bottom: 8px;
		}

		.courses-grid {
			margin-right: -8px;
			margin-left: -8px;
		}

		.courses-grid > .col-sm-6 {
			padding-right: 8px;
			padding-left: 8px;
			margin-bottom: 16px;
		}
	}
</style>
