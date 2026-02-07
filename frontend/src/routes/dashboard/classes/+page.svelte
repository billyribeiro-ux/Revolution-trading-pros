<!--
	My Classes List Page
	Apple ICT 7 Principal Engineer Grade - February 2026

	Features:
	- Fetches enrolled courses from API
	- Shows progress tracking for each course
	- Resume functionality
	- Certificate status
	- Svelte 5 $state and $derived runes

	@version 2.0.0 - February 2026
-->
<script lang="ts">
	import { onMount } from 'svelte';
	interface EnrolledCourse {
		id: number;
		course_id: string;
		progress_percent: number;
		status: string;
		enrolled_at: string;
		last_accessed_at?: string;
		current_lesson_id?: string;
		certificate_issued?: boolean;
		course: {
			id: string;
			title: string;
			slug: string;
			card_image_url?: string;
			card_description?: string;
			instructor_name?: string;
			level?: string;
			lesson_count?: number;
			total_duration_minutes?: number;
			price_cents: number;
			is_free?: boolean;
		};
	}

	// State
	let enrolledCourses = $state<EnrolledCourse[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let currentPage = $state(1);
	const itemsPerPage = 12;

	// Fetch enrolled courses on mount
	onMount(async () => {
		try {
			const response = await fetch('/api/my/courses');
			const data = await response.json();

			if (data.success) {
				enrolledCourses = data.data || [];
			} else {
				error = data.error || 'Failed to load courses';
			}
		} catch (e) {
			console.error('Failed to fetch enrolled courses:', e);
			error = 'Failed to load your courses. Please try again.';
		} finally {
			loading = false;
		}
	});

	// Resume course function
	async function resumeCourse(slug: string) {
		try {
			const response = await fetch(`/api/my/courses/${slug}/resume`);
			const data = await response.json();

			if (data.success && data.data.resume_url) {
				window.location.href = data.data.resume_url;
			} else {
				window.location.href = `/classes/${slug}`;
			}
		} catch {
			window.location.href = `/classes/${slug}`;
		}
	}

	// Computed pagination values
	let totalPages = $derived(Math.ceil(enrolledCourses.length / itemsPerPage));
	let startIndex = $derived((currentPage - 1) * itemsPerPage);
	let endIndex = $derived(startIndex + itemsPerPage);
	let paginatedCourses = $derived(enrolledCourses.slice(startIndex, endIndex));
	let pageNumbers = $derived(Array.from({ length: totalPages }, (_, i) => i + 1));

	// Separate in-progress and completed courses
	let inProgressCourses = $derived(paginatedCourses.filter((c) => c.progress_percent < 100));
	let completedCourses = $derived(paginatedCourses.filter((c) => c.progress_percent >= 100));

	// Pagination navigation
	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function nextPage() {
		if (currentPage < totalPages) goToPage(currentPage + 1);
	}

	function previousPage() {
		if (currentPage > 1) goToPage(currentPage - 1);
	}

	// Format date helper
	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			year: 'numeric'
		});
	}
</script>

<svelte:head>
	<title>My Classes - Revolution Trading Pros</title>
	<meta name="description" content="Access your enrolled trading classes and track your progress" />
</svelte:head>

<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<aside class="dashboard__sidebar">
			<!-- Sidebar content if needed -->
		</aside>

		<div class="dashboard__main">
			<!-- Page Header -->
			<header class="dashboard__header">
				<div class="dashboard__header-left">
					<h1 class="dashboard__page-title">My Classes</h1>
					{#if !loading && enrolledCourses.length > 0}
						<p class="dashboard__subtitle">
							{enrolledCourses.length} course{enrolledCourses.length !== 1 ? 's' : ''} enrolled
						</p>
					{/if}
				</div>
				<div class="dashboard__header-right">
					<a href="/courses" class="btn btn-primary">Browse All Courses</a>
				</div>
			</header>

			<!-- Loading State -->
			{#if loading}
				<div class="dashboard__content">
					<div class="loading-state">
						<div class="spinner"></div>
						<p>Loading your classes...</p>
					</div>
				</div>

				<!-- Error State -->
			{:else if error}
				<div class="dashboard__content">
					<div class="error-state">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="48"
							height="48"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<circle cx="12" cy="12" r="10" /><line x1="12" x2="12" y1="8" y2="12" /><line
								x1="12"
								x2="12.01"
								y1="16"
								y2="16"
							/>
						</svg>
						<p>{error}</p>
						<button class="btn btn-primary" onclick={() => window.location.reload()}
							>Try Again</button
						>
					</div>
				</div>

				<!-- Empty State -->
			{:else if enrolledCourses.length === 0}
				<div class="dashboard__content">
					<div class="dashboard__content-main">
						<section class="dashboard__content-section">
							<div class="empty-state">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="64"
									height="64"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="1.5"
								>
									<path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" /><path
										d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"
									/>
								</svg>
								<h3>No Classes Yet</h3>
								<p class="empty-state__message">
									You haven't enrolled in any classes yet. Browse our course catalog to get started!
								</p>
								<a href="/courses" class="btn btn-orange">Explore Courses</a>
							</div>
						</section>
					</div>
				</div>

				<!-- Enrolled Courses -->
			{:else}
				<div class="dashboard__content">
					<div class="dashboard__content-main">
						<!-- In Progress Section -->
						{#if inProgressCourses.length > 0}
							<section class="dashboard__content-section">
								<h2 class="section-title">Continue Learning</h2>
								<div class="class-grid">
									{#each inProgressCourses as enrollment (enrollment.id)}
										<article class="class-grid__item">
											<div class="class-card">
												{#if enrollment.course.card_image_url}
													<div class="class-card__image">
														<img
															src={enrollment.course.card_image_url}
															alt={enrollment.course.title}
															loading="lazy"
														/>
														<div class="progress-overlay">
															<div class="progress-bar">
																<div
																	class="progress-fill"
																	style="width: {enrollment.progress_percent}%"
																></div>
															</div>
															<span class="progress-text"
																>{enrollment.progress_percent}% complete</span
															>
														</div>
													</div>
												{/if}
												<section class="class-card__body">
													<h4 class="class-card__title">
														<a href="/classes/{enrollment.course.slug}">{enrollment.course.title}</a
														>
													</h4>
													{#if enrollment.course.instructor_name}
														<p class="class-card__instructor">
															by {enrollment.course.instructor_name}
														</p>
													{/if}
													<p class="class-card__meta">
														{#if enrollment.course.lesson_count}
															<span>{enrollment.course.lesson_count} lessons</span>
														{/if}
														{#if enrollment.last_accessed_at}
															<span>Last viewed {formatDate(enrollment.last_accessed_at)}</span>
														{/if}
													</p>
												</section>
												<footer class="class-card__footer">
													<button
														class="btn btn-primary btn-small"
														onclick={() => resumeCourse(enrollment.course.slug)}
													>
														Continue
													</button>
												</footer>
											</div>
										</article>
									{/each}
								</div>
							</section>
						{/if}

						<!-- Completed Section -->
						{#if completedCourses.length > 0}
							<section class="dashboard__content-section">
								<h2 class="section-title">Completed</h2>
								<div class="class-grid">
									{#each completedCourses as enrollment (enrollment.id)}
										<article class="class-grid__item">
											<div class="class-card completed">
												{#if enrollment.course.card_image_url}
													<div class="class-card__image">
														<img
															src={enrollment.course.card_image_url}
															alt={enrollment.course.title}
															loading="lazy"
														/>
														<div class="completed-badge">
															<svg
																xmlns="http://www.w3.org/2000/svg"
																width="16"
																height="16"
																viewBox="0 0 24 24"
																fill="none"
																stroke="currentColor"
																stroke-width="2"
															>
																<polyline points="20 6 9 17 4 12" />
															</svg>
															Completed
														</div>
													</div>
												{/if}
												<section class="class-card__body">
													<h4 class="class-card__title">
														<a href="/classes/{enrollment.course.slug}">{enrollment.course.title}</a
														>
													</h4>
													{#if enrollment.course.instructor_name}
														<p class="class-card__instructor">
															by {enrollment.course.instructor_name}
														</p>
													{/if}
												</section>
												<footer class="class-card__footer">
													<a
														class="btn btn-secondary btn-small"
														href="/classes/{enrollment.course.slug}">Review</a
													>
													{#if enrollment.certificate_issued}
														<a
															class="btn btn-outline btn-small"
															href="/api/my/courses/{enrollment.course.slug}/certificate"
														>
															<svg
																xmlns="http://www.w3.org/2000/svg"
																width="14"
																height="14"
																viewBox="0 0 24 24"
																fill="none"
																stroke="currentColor"
																stroke-width="2"
															>
																<circle cx="12" cy="8" r="6" /><path
																	d="M15.477 12.89 17 22l-5-3-5 3 1.523-9.11"
																/>
															</svg>
															Certificate
														</a>
													{/if}
												</footer>
											</div>
										</article>
									{/each}
								</div>
							</section>
						{/if}

						<!-- Pagination -->
						{#if totalPages > 1}
							<div class="pagination-wrapper">
								<nav class="pagination" aria-label="Pagination">
									<ul class="page-numbers">
										<li>
											<button
												class="page-numbers prev"
												onclick={previousPage}
												disabled={currentPage === 1}
												aria-label="Previous page"
											>
												Previous
											</button>
										</li>
										{#each pageNumbers as pageNum}
											<li>
												{#if pageNum === currentPage}
													<span class="page-numbers current" aria-current="page">{pageNum}</span>
												{:else}
													<button
														class="page-numbers"
														onclick={() => goToPage(pageNum)}
														aria-label="Go to page {pageNum}"
													>
														{pageNum}
													</button>
												{/if}
											</li>
										{/each}
										<li>
											<button
												class="page-numbers next"
												onclick={nextPage}
												disabled={currentPage === totalPages}
												aria-label="Next page"
											>
												Next
											</button>
										</li>
									</ul>
								</nav>
							</div>
						{/if}
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * My Classes Dashboard - ICT 7 Principal Engineer Grade
	 * February 2026
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px;
	}

	@media (min-width: 1024px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__page-title {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.dashboard__subtitle {
		font-size: 14px;
		color: #666;
		margin: 4px 0 0;
	}

	.dashboard__header-right {
		display: flex;
		gap: 12px;
	}

	/* Section Title */
	.section-title {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 20px;
		padding-bottom: 12px;
		border-bottom: 1px solid #e5e7eb;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px 20px;
		text-align: center;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-state p {
		color: #6b7280;
		font-size: 16px;
	}

	/* Error State */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px 20px;
		text-align: center;
	}

	.error-state svg {
		color: #dc2626;
		margin-bottom: 16px;
	}

	.error-state p {
		color: #6b7280;
		font-size: 16px;
		margin: 0 0 20px;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
		background: #f9fafb;
		border-radius: 12px;
	}

	.empty-state svg {
		color: #9ca3af;
		margin-bottom: 20px;
	}

	.empty-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.empty-state__message {
		font-size: 16px;
		color: #6b7280;
		margin: 0 0 24px;
		max-width: 400px;
	}

	/* Class Grid */
	.class-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 20px;
		margin-bottom: 30px;
	}

	@media (min-width: 640px) {
		.class-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.class-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.class-grid__item {
		display: flex;
	}

	/* Class Card */
	.class-card {
		display: flex;
		flex-direction: column;
		width: 100%;
		background: #fff;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
		transition: all 0.2s;
	}

	.class-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.class-card.completed {
		border: 2px solid #10b981;
	}

	.class-card__image {
		position: relative;
		aspect-ratio: 16/9;
		overflow: hidden;
	}

	.class-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	/* Progress Overlay */
	.progress-overlay {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 12px;
		background: linear-gradient(transparent, rgba(0, 0, 0, 0.7));
	}

	.progress-bar {
		height: 4px;
		background: rgba(255, 255, 255, 0.3);
		border-radius: 2px;
		overflow: hidden;
		margin-bottom: 6px;
	}

	.progress-fill {
		height: 100%;
		background: #10b981;
		border-radius: 2px;
		transition: width 0.3s;
	}

	.progress-text {
		font-size: 12px;
		color: #fff;
		font-weight: 500;
	}

	/* Completed Badge */
	.completed-badge {
		position: absolute;
		top: 12px;
		right: 12px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 6px 10px;
		background: #10b981;
		color: #fff;
		font-size: 12px;
		font-weight: 600;
		border-radius: 4px;
	}

	.class-card__body {
		padding: 16px;
		flex: 1;
	}

	.class-card__title {
		font-size: 16px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
		line-height: 1.3;
	}

	.class-card__title a {
		color: inherit;
		text-decoration: none;
	}

	.class-card__title a:hover {
		color: #143e59;
	}

	.class-card__instructor {
		font-size: 14px;
		color: #6b7280;
		margin: 0 0 8px;
	}

	.class-card__meta {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		font-size: 12px;
		color: #9ca3af;
	}

	.class-card__meta span {
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.class-card__footer {
		display: flex;
		gap: 8px;
		padding: 12px 16px;
		border-top: 1px solid #f3f4f6;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 10px 20px;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-small {
		padding: 8px 14px;
		font-size: 13px;
	}

	.btn-primary {
		background: #143e59;
		color: #fff;
	}

	.btn-primary:hover {
		background: #0f2d42;
	}

	.btn-secondary {
		background: #f3f4f6;
		color: #374151;
	}

	.btn-secondary:hover {
		background: #e5e7eb;
	}

	.btn-outline {
		background: transparent;
		border: 1px solid #d1d5db;
		color: #374151;
	}

	.btn-outline:hover {
		background: #f9fafb;
		border-color: #9ca3af;
	}

	.btn-orange {
		background-color: #ff8c00;
		color: #fff;
	}

	.btn-orange:hover {
		background-color: #e67e00;
	}

	/* Pagination */
	.pagination-wrapper {
		padding: 40px 0;
		text-align: center;
	}

	.pagination {
		display: inline-block;
	}

	.page-numbers {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		gap: 8px;
		align-items: center;
		justify-content: center;
		flex-wrap: wrap;
	}

	.page-numbers li {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.page-numbers button,
	.page-numbers span {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 40px;
		height: 40px;
		padding: 8px 12px;
		border: 1px solid #e5e7eb;
		background: #fff;
		color: #333;
		font-size: 14px;
		font-weight: 600;
		border-radius: 6px;
		transition: all 0.2s;
		cursor: pointer;
	}

	.page-numbers button:hover:not(:disabled) {
		background: #f3f4f6;
		border-color: #143e59;
		color: #143e59;
	}

	.page-numbers button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-numbers .current {
		background: #143e59;
		color: #fff;
		border-color: #143e59;
		cursor: default;
	}

	.page-numbers .prev,
	.page-numbers .next {
		min-width: auto;
		padding: 8px 16px;
	}

	/* Responsive */
	@media (max-width: 767px) {
		.dashboard__header {
			flex-direction: column;
			align-items: flex-start;
		}

		.dashboard__page-title {
			font-size: 24px;
		}

		.section-title {
			font-size: 18px;
		}
	}

	/* Dashboard Content Sections */
	.dashboard__content-section {
		margin-bottom: 40px;
	}

	.dashboard__content-section:last-child {
		margin-bottom: 0;
	}
</style>
