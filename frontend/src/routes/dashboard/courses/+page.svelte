<script lang="ts">
	/**
	 * Dashboard - My Courses Page
	 * Shows user's enrolled courses and progress
	 */
	import { fly, fade } from 'svelte/transition';
	import {
		IconBook,
		IconClock,
		IconPlayerPlay,
		IconCheck
	} from '@tabler/icons-svelte';

	// Mock enrolled courses data - will be fetched from API
	const enrolledCourses = [
		{
			id: 1,
			title: 'Options Trading Mastery',
			description: 'Complete guide to options trading strategies',
			progress: 65,
			totalLessons: 24,
			completedLessons: 16,
			thumbnail: '/images/courses/options.jpg',
			lastAccessed: '2 days ago'
		},
		{
			id: 2,
			title: 'Technical Analysis Fundamentals',
			description: 'Learn to read charts and identify patterns',
			progress: 100,
			totalLessons: 18,
			completedLessons: 18,
			thumbnail: '/images/courses/technical.jpg',
			lastAccessed: '1 week ago'
		},
		{
			id: 3,
			title: 'Day Trading Strategies',
			description: 'Master intraday trading techniques',
			progress: 30,
			totalLessons: 20,
			completedLessons: 6,
			thumbnail: '/images/courses/daytrading.jpg',
			lastAccessed: 'Today'
		}
	];
</script>

<!-- Dashboard Header -->
<div class="wc-content-sction">
	<div class="dashb_headr">
		<div class="dashb_headr-left">
			<h1 class="dashb_pg-titl">My Courses</h1>
		</div>
		<div class="dashb_headr-right">
			<a href="/courses" class="btn btn-xs btn-link start-here-btn">
				Browse All Courses
			</a>
		</div>
	</div>
</div>

<!-- Courses Content -->
<div class="wc-accontent-inner">
	{#if enrolledCourses.length > 0}
		<div class="courses-grid">
			{#each enrolledCourses as course, i (course.id)}
				<div class="course-card" in:fly={{ y: 20, delay: 100 * i, duration: 400 }}>
					<div class="course-card__thumbnail">
						<div class="course-card__placeholder">
							<IconBook size={48} />
						</div>
						{#if course.progress === 100}
							<div class="course-card__badge course-card__badge--complete">
								<IconCheck size={14} />
								Completed
							</div>
						{:else}
							<div class="course-card__badge">
								{course.progress}% Complete
							</div>
						{/if}
					</div>
					<div class="course-card__content">
						<h3 class="course-card__title">{course.title}</h3>
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

						<a href="/courses/{course.id}" class="course-card__btn">
							<IconPlayerPlay size={18} />
							{course.progress === 100 ? 'Review Course' : 'Continue Learning'}
						</a>
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<div class="empty-state" in:fade>
			<IconBook size={64} />
			<h2>No Courses Yet</h2>
			<p>You haven't enrolled in any courses yet. Browse our catalog to get started!</p>
			<a href="/courses" class="btn btn-orange">Browse Courses</a>
		</div>
	{/if}
</div>

<style>
	/* Content Section */
	.wc-content-sction {
		width: 100%;
		margin: auto;
		padding: 20px;
	}

	/* Dashboard Header */
	.dashb_headr {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		max-width: 100%;
		padding: 20px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
	}

	.dashb_headr-left,
	.dashb_headr-right {
		align-items: center;
		display: flex;
		flex-direction: row;
	}

	.dashb_pg-titl {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
	}

	.start-here-btn {
		font-size: 14px;
		line-height: 18px;
		padding: 8px 14px;
		font-weight: 600;
		color: #0984ae;
		background: #f4f4f4;
		border-color: transparent;
		text-decoration: none;
		border-radius: 5px;
		transition: all 0.15s ease-in-out;
	}

	.start-here-btn:hover {
		color: #0984ae;
		background: #e7e7e7;
	}

	/* Inner Content */
	.wc-accontent-inner {
		padding: 4% 2%;
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 1px 2px rgb(0 0 0 / 15%);
		position: relative;
		margin: 20px;
	}

	/* Courses Grid */
	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	/* Course Card */
	.course-card {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		overflow: hidden;
		transition: all 0.3s ease;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.course-card:hover {
		border-color: #0984ae;
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

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
		color: #64748b;
	}

	.course-card__badge {
		position: absolute;
		top: 1rem;
		right: 1rem;
		padding: 0.375rem 0.75rem;
		background: rgba(0, 0, 0, 0.7);
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #facc15;
	}

	.course-card__badge--complete {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		background: rgba(16, 185, 129, 0.9);
		color: #fff;
	}

	.course-card__content {
		padding: 1.5rem;
	}

	.course-card__title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #333;
		margin-bottom: 0.5rem;
	}

	.course-card__description {
		font-size: 0.875rem;
		color: #64748b;
		margin-bottom: 1rem;
		line-height: 1.5;
	}

	.course-card__progress {
		margin-bottom: 1rem;
	}

	.progress-bar {
		height: 6px;
		background: #e5e7eb;
		border-radius: 3px;
		overflow: hidden;
		margin-bottom: 0.5rem;
	}

	.progress-bar__fill {
		height: 100%;
		background: linear-gradient(90deg, #0984ae, #0ea5e9);
		border-radius: 3px;
		transition: width 0.3s ease;
	}

	.progress-text {
		font-size: 0.75rem;
		color: #64748b;
	}

	.course-card__meta {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	.course-card__btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem;
		background: #0984ae;
		border-radius: 8px;
		color: #fff;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.course-card__btn:hover {
		background: #076787;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #64748b;
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: #333;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
	}

	.btn-orange {
		display: inline-flex;
		padding: 0.75rem 2rem;
		background: #f99e31;
		border-radius: 8px;
		color: #fff;
		font-weight: 600;
		text-decoration: none;
	}

	.btn-orange:hover {
		background: #f88b09;
	}

	@media (max-width: 768px) {
		.courses-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
