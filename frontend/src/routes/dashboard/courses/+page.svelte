<script lang="ts">
	/**
	 * Dashboard - My Courses Page
	 * Shows user's enrolled courses and progress
	 */
	import SEOHead from '$lib/components/SEOHead.svelte';
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import {
		IconBook,
		IconClock,
		IconPlayerPlay,
		IconCheck,
		IconLock
	} from '@tabler/icons-svelte';

	// Redirect if not authenticated - use replaceState to prevent history pollution
	onMount(() => {
		if (!$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/courses', { replaceState: true });
		}
	});

	// Mock enrolled courses data
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

<SEOHead
	title="My Courses"
	description="Access your enrolled courses and track your learning progress."
	noindex
/>

{#if $isAuthenticated}
	<main class="dashboard-page">
		<div class="container">
			<!-- Header -->
			<header class="page-header" in:fly={{ y: -20, duration: 400 }}>
				<div class="page-header__content">
					<h1 class="page-header__title">My Courses</h1>
					<p class="page-header__subtitle">Continue learning and track your progress</p>
				</div>
				<a href="/courses" class="browse-btn">
					Browse All Courses
				</a>
			</header>

			<!-- Courses Grid -->
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

			{#if enrolledCourses.length === 0}
				<div class="empty-state" in:fade>
					<IconBook size={64} />
					<h2>No Courses Yet</h2>
					<p>You haven't enrolled in any courses yet. Browse our catalog to get started!</p>
					<a href="/courses" class="empty-state__btn">Browse Courses</a>
				</div>
			{/if}
		</div>
	</main>
{:else}
	<div class="loading-state">
		<p>Redirecting to login...</p>
	</div>
{/if}

<style>
	.dashboard-page {
		min-height: 100vh;
		background: var(--rtp-bg, #0a0f1a);
		color: var(--rtp-text, #e5e7eb);
		padding: 8rem 1.5rem 4rem;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 3rem;
		gap: 2rem;
		flex-wrap: wrap;
	}

	.page-header__title {
		font-size: 2.5rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 0.5rem;
	}

	.page-header__subtitle {
		color: #94a3b8;
		font-size: 1.1rem;
	}

	.browse-btn {
		padding: 0.75rem 1.5rem;
		background: rgba(250, 204, 21, 0.1);
		border: 1px solid rgba(250, 204, 21, 0.2);
		border-radius: 8px;
		color: #facc15;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.browse-btn:hover {
		background: rgba(250, 204, 21, 0.2);
	}

	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
		gap: 1.5rem;
	}

	.course-card {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		overflow: hidden;
		transition: all 0.3s ease;
	}

	.course-card:hover {
		border-color: rgba(250, 204, 21, 0.3);
		transform: translateY(-2px);
	}

	.course-card__thumbnail {
		position: relative;
		height: 160px;
		background: linear-gradient(135deg, rgba(250, 204, 21, 0.1), rgba(139, 92, 246, 0.1));
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
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.course-card__content {
		padding: 1.5rem;
	}

	.course-card__title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 0.5rem;
	}

	.course-card__description {
		font-size: 0.875rem;
		color: #94a3b8;
		margin-bottom: 1rem;
		line-height: 1.5;
	}

	.course-card__progress {
		margin-bottom: 1rem;
	}

	.progress-bar {
		height: 6px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 3px;
		overflow: hidden;
		margin-bottom: 0.5rem;
	}

	.progress-bar__fill {
		height: 100%;
		background: linear-gradient(90deg, #facc15, #f97316);
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
		background: linear-gradient(135deg, #facc15, #f97316);
		border-radius: 8px;
		color: #0a0f1a;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.course-card__btn:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(250, 204, 21, 0.3);
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #64748b;
	}

	.empty-state h2 {
		font-size: 1.5rem;
		color: #fff;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
	}

	.empty-state__btn {
		display: inline-flex;
		padding: 0.75rem 2rem;
		background: linear-gradient(135deg, #facc15, #f97316);
		border-radius: 8px;
		color: #0a0f1a;
		font-weight: 600;
		text-decoration: none;
	}

	.loading-state {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--rtp-bg, #0a0f1a);
		color: #94a3b8;
	}

	@media (max-width: 768px) {
		.dashboard-page {
			padding: 7rem 1rem 3rem;
		}

		.page-header {
			flex-direction: column;
		}

		.page-header__title {
			font-size: 2rem;
		}

		.courses-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
