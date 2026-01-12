<script lang="ts">
	/**
	 * My Courses Dashboard
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * Displays enrolled courses with progress tracking
	 */

	import { onMount } from 'svelte';
	import { CourseCard } from '$lib/components/courses';

	interface Course {
		id: string;
		title: string;
		slug: string;
		card_image_url?: string;
		card_description?: string;
		instructor_name?: string;
		lesson_count?: number;
		total_duration_minutes?: number;
		price_cents: number;
	}

	interface Enrollment {
		id: number;
		course_id: string;
		progress_percent: number;
		status: string;
		enrolled_at: string;
		last_accessed_at?: string;
		current_lesson_id?: string;
		course: Course;
	}

	let enrollments = $state<Enrollment[]>([]);
	let loading = $state(true);
	let activeTab = $state<'in-progress' | 'completed' | 'all'>('in-progress');

	const fetchEnrollments = async () => {
		loading = true;
		try {
			const res = await fetch('/api/my/courses');
			const data = await res.json();
			if (data.success) {
				enrollments = data.data;
			}
		} catch (e) {
			console.error('Failed to fetch enrollments:', e);
		} finally {
			loading = false;
		}
	};

	const filteredEnrollments = $derived(() => {
		if (activeTab === 'in-progress') {
			return enrollments.filter(e => e.status === 'active' && e.progress_percent < 100);
		} else if (activeTab === 'completed') {
			return enrollments.filter(e => e.status === 'completed' || e.progress_percent >= 100);
		}
		return enrollments;
	});

	const stats = $derived(() => {
		const inProgress = enrollments.filter(e => e.status === 'active' && e.progress_percent < 100).length;
		const completed = enrollments.filter(e => e.status === 'completed' || e.progress_percent >= 100).length;
		return { total: enrollments.length, inProgress, completed };
	});

	onMount(fetchEnrollments);
</script>

<svelte:head>
	<title>My Courses | Revolution Trading Pros</title>
</svelte:head>

<div class="my-courses">
	<header class="page-header">
		<h1>My Courses</h1>
		<p>Track your learning progress and continue where you left off</p>
	</header>

	<div class="stats-bar">
		<div class="stat">
			<span class="stat-value">{stats().total}</span>
			<span class="stat-label">Total Enrolled</span>
		</div>
		<div class="stat">
			<span class="stat-value">{stats().inProgress}</span>
			<span class="stat-label">In Progress</span>
		</div>
		<div class="stat">
			<span class="stat-value">{stats().completed}</span>
			<span class="stat-label">Completed</span>
		</div>
	</div>

	<nav class="tabs">
		<button class:active={activeTab === 'in-progress'} onclick={() => activeTab = 'in-progress'}>
			In Progress ({stats().inProgress})
		</button>
		<button class:active={activeTab === 'completed'} onclick={() => activeTab = 'completed'}>
			Completed ({stats().completed})
		</button>
		<button class:active={activeTab === 'all'} onclick={() => activeTab = 'all'}>
			All Courses ({stats().total})
		</button>
	</nav>

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading your courses...</p>
		</div>
	{:else if filteredEnrollments().length === 0}
		<div class="empty-state">
			{#if activeTab === 'in-progress'}
				<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"/></svg>
				<h3>No courses in progress</h3>
				<p>Start a new course or continue one you've started</p>
			{:else if activeTab === 'completed'}
				<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
				<h3>No completed courses yet</h3>
				<p>Keep learning to complete your first course</p>
			{:else}
				<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"/></svg>
				<h3>No enrolled courses</h3>
				<p>Browse our catalog and enroll in a course to get started</p>
				<a href="/classes" class="btn-primary">Browse Courses</a>
			{/if}
		</div>
	{:else}
		<div class="courses-grid">
			{#each filteredEnrollments() as enrollment}
				<div class="enrollment-card">
					<CourseCard 
						course={enrollment.course} 
						showProgress={true} 
						progress={enrollment.progress_percent}
					/>
					<div class="enrollment-meta">
						{#if enrollment.last_accessed_at}
							<span class="last-accessed">
								Last accessed: {new Date(enrollment.last_accessed_at).toLocaleDateString()}
							</span>
						{/if}
						<a href="/classes/{enrollment.course.slug}" class="continue-btn">
							{enrollment.progress_percent > 0 ? 'Continue Learning' : 'Start Course'}
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m9 18 6-6-6-6"/></svg>
						</a>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.my-courses {
		min-height: 100vh;
		background: #f9fafb;
		padding-bottom: 48px;
	}

	.page-header {
		background: #143e59;
		padding: 48px 24px;
		text-align: center;
		color: #fff;
	}

	.page-header h1 {
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 8px;
	}

	.page-header p {
		font-size: 16px;
		opacity: 0.9;
		margin: 0;
	}

	.stats-bar {
		display: flex;
		justify-content: center;
		gap: 48px;
		padding: 24px;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
	}

	.stat {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
	}

	.stat-value {
		font-size: 28px;
		font-weight: 700;
		color: #143e59;
	}

	.stat-label {
		font-size: 13px;
		color: #6b7280;
	}

	.tabs {
		display: flex;
		justify-content: center;
		gap: 8px;
		padding: 16px 24px;
		background: #fff;
		border-bottom: 1px solid #e5e7eb;
	}

	.tabs button {
		padding: 10px 20px;
		background: none;
		border: none;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 500;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tabs button:hover {
		background: #f3f4f6;
		color: #1f2937;
	}

	.tabs button.active {
		background: #143e59;
		color: #fff;
	}

	.loading, .empty-state {
		text-align: center;
		padding: 80px 24px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 16px;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.empty-state svg {
		color: #d1d5db;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		font-size: 20px;
		color: #1f2937;
		margin: 0 0 8px;
	}

	.empty-state p {
		color: #6b7280;
		margin: 0 0 24px;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #143e59;
		color: #fff;
		border-radius: 8px;
		text-decoration: none;
		font-weight: 500;
		transition: background 0.2s;
	}

	.btn-primary:hover {
		background: #0f2d42;
	}

	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 24px;
		padding: 32px 24px;
		max-width: 1400px;
		margin: 0 auto;
	}

	.enrollment-card {
		display: flex;
		flex-direction: column;
	}

	.enrollment-meta {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 16px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-top: none;
		border-radius: 0 0 12px 12px;
		margin-top: -12px;
	}

	.last-accessed {
		font-size: 12px;
		color: #9ca3af;
	}

	.continue-btn {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 8px 16px;
		background: #10b981;
		color: #fff;
		border-radius: 6px;
		text-decoration: none;
		font-size: 13px;
		font-weight: 500;
		transition: background 0.2s;
	}

	.continue-btn:hover {
		background: #059669;
	}

	@media (max-width: 768px) {
		.page-header {
			padding: 32px 16px;
		}

		.page-header h1 {
			font-size: 24px;
		}

		.stats-bar {
			gap: 24px;
			padding: 16px;
		}

		.stat-value {
			font-size: 24px;
		}

		.tabs {
			overflow-x: auto;
			justify-content: flex-start;
			padding: 12px 16px;
		}

		.tabs button {
			white-space: nowrap;
		}

		.courses-grid {
			grid-template-columns: 1fr;
			padding: 24px 16px;
		}

		.enrollment-meta {
			flex-direction: column;
			gap: 12px;
			align-items: stretch;
		}

		.continue-btn {
			justify-content: center;
		}
	}
</style>
