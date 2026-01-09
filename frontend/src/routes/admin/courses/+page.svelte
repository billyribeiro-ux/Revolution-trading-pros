<script lang="ts">
	/**
	 * Admin Courses List Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { onMount } from 'svelte';
	import { CourseCard } from '$lib/components/courses';

	interface Course {
		id: string;
		title: string;
		slug: string;
		card_image_url?: string;
		card_description?: string;
		card_badge?: string;
		card_badge_color?: string;
		instructor_name?: string;
		level?: string;
		price_cents: number;
		is_free?: boolean;
		is_published: boolean;
		status?: string;
		lesson_count?: number;
		module_count?: number;
		enrollment_count?: number;
		created_at: string;
	}

	let courses = $state<Course[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let searchQuery = $state('');
	let statusFilter = $state('all');
	let currentPage = $state(1);
	let totalPages = $state(1);
	let total = $state(0);

	const fetchCourses = async () => {
		loading = true;
		error = null;
		try {
			const params = new URLSearchParams({
				page: currentPage.toString(),
				per_page: '12'
			});
			if (searchQuery) params.set('search', searchQuery);
			if (statusFilter !== 'all') params.set('status', statusFilter);

			const res = await fetch(`/api/admin/courses?${params}`);
			const data = await res.json();

			if (data.success) {
				courses = data.data.courses;
				total = data.data.total;
				totalPages = data.data.total_pages;
			} else {
				error = data.error || 'Failed to fetch courses';
			}
		} catch (e) {
			error = 'Failed to connect to server';
		} finally {
			loading = false;
		}
	};

	const handleSearch = () => {
		currentPage = 1;
		fetchCourses();
	};

	const handleDelete = async (courseId: string) => {
		if (!confirm('Are you sure you want to delete this course? This action cannot be undone.')) return;

		try {
			const res = await fetch(`/api/admin/courses/${courseId}`, { method: 'DELETE' });
			const data = await res.json();
			if (data.success) {
				courses = courses.filter(c => c.id !== courseId);
				total -= 1;
			} else {
				alert(data.error || 'Failed to delete course');
			}
		} catch {
			alert('Failed to delete course');
		}
	};

	const handlePublish = async (course: Course) => {
		const endpoint = course.is_published ? 'unpublish' : 'publish';
		try {
			const res = await fetch(`/api/admin/courses/${course.id}/${endpoint}`, { method: 'POST' });
			const data = await res.json();
			if (data.success) {
				course.is_published = !course.is_published;
				course.status = course.is_published ? 'published' : 'draft';
				courses = [...courses];
			}
		} catch {
			alert('Failed to update course status');
		}
	};

	onMount(fetchCourses);
</script>

<svelte:head>
	<title>Manage Courses | Admin</title>
</svelte:head>

<div class="admin-courses">
	<header class="page-header">
		<div class="header-content">
			<h1>Courses</h1>
			<p class="subtitle">{total} courses total</p>
		</div>
		<a href="/admin/courses/create" class="btn-primary">
			<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" x2="12" y1="5" y2="19"/><line x1="5" x2="19" y1="12" y2="12"/></svg>
			Create Course
		</a>
	</header>

	<div class="filters">
		<div class="search-box">
			<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" x2="16.65" y1="21" y2="16.65"/></svg>
			<input 
				type="text" 
				placeholder="Search courses..." 
				bind:value={searchQuery}
				onkeydown={(e) => e.key === 'Enter' && handleSearch()}
			/>
		</div>
		<select bind:value={statusFilter} onchange={handleSearch}>
			<option value="all">All Status</option>
			<option value="draft">Draft</option>
			<option value="published">Published</option>
			<option value="archived">Archived</option>
		</select>
	</div>

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading courses...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/></svg>
			<p>{error}</p>
			<button onclick={fetchCourses}>Retry</button>
		</div>
	{:else if courses.length === 0}
		<div class="empty-state">
			<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"/></svg>
			<h3>No courses found</h3>
			<p>Get started by creating your first course</p>
			<a href="/admin/courses/create" class="btn-primary">Create Course</a>
		</div>
	{:else}
		<div class="courses-grid">
			{#each courses as course}
				<div class="course-card-wrapper">
					<div class="card-status" class:published={course.is_published}>
						<span class="status-dot"></span>
						{course.status || (course.is_published ? 'Published' : 'Draft')}
					</div>
					<CourseCard {course} variant="default" />
					<div class="card-actions">
						<a href="/admin/courses/{course.id}" class="action-btn edit">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/></svg>
							Edit
						</a>
						<button class="action-btn publish" onclick={() => handlePublish(course)}>
							{#if course.is_published}
								<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" x2="23" y1="1" y2="23"/></svg>
								Unpublish
							{:else}
								<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
								Publish
							{/if}
						</button>
						<button class="action-btn delete" onclick={() => handleDelete(course.id)} aria-label="Delete course">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>
						</button>
					</div>
				</div>
			{/each}
		</div>

		{#if totalPages > 1}
			<div class="pagination">
				<button disabled={currentPage === 1} onclick={() => { currentPage--; fetchCourses(); }} aria-label="Previous page">
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m15 18-6-6 6-6"/></svg>
				</button>
				<span>Page {currentPage} of {totalPages}</span>
				<button disabled={currentPage === totalPages} onclick={() => { currentPage++; fetchCourses(); }} aria-label="Next page">
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m9 18 6-6-6-6"/></svg>
				</button>
			</div>
		{/if}
	{/if}
</div>

<style>
	.admin-courses { padding: 24px; max-width: 1400px; margin: 0 auto; }

	.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
	.header-content h1 { font-size: 28px; font-weight: 700; color: #1f2937; margin: 0; }
	.subtitle { color: #6b7280; margin: 4px 0 0; font-size: 14px; }

	.btn-primary { display: inline-flex; align-items: center; gap: 8px; padding: 10px 20px; background: #143e59; color: #fff; border-radius: 8px; text-decoration: none; font-weight: 500; font-size: 14px; transition: background 0.2s; }
	.btn-primary:hover { background: #0f2d42; }

	.filters { display: flex; gap: 12px; margin-bottom: 24px; }
	.search-box { flex: 1; max-width: 400px; position: relative; }
	.search-box svg { position: absolute; left: 12px; top: 50%; transform: translateY(-50%); color: #9ca3af; }
	.search-box input { width: 100%; padding: 10px 12px 10px 40px; border: 1px solid #e5e7eb; border-radius: 8px; font-size: 14px; }
	.search-box input:focus { outline: none; border-color: #143e59; }

	.filters select { padding: 10px 16px; border: 1px solid #e5e7eb; border-radius: 8px; font-size: 14px; background: #fff; cursor: pointer; }

	.loading, .error-state, .empty-state { text-align: center; padding: 64px 24px; }
	.spinner { width: 40px; height: 40px; border: 3px solid #e5e7eb; border-top-color: #143e59; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto 16px; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.error-state svg, .empty-state svg { color: #9ca3af; margin-bottom: 16px; }
	.error-state p, .empty-state p { color: #6b7280; margin: 0 0 16px; }
	.empty-state h3 { font-size: 18px; color: #1f2937; margin: 0 0 8px; }

	.courses-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 24px; }

	.course-card-wrapper { position: relative; }
	.card-status { position: absolute; top: 12px; right: 12px; z-index: 10; display: flex; align-items: center; gap: 6px; padding: 4px 10px; background: rgba(0,0,0,0.7); border-radius: 4px; font-size: 11px; color: #fff; text-transform: uppercase; }
	.status-dot { width: 6px; height: 6px; border-radius: 50%; background: #fbbf24; }
	.card-status.published .status-dot { background: #10b981; }

	.card-actions { display: flex; gap: 8px; padding: 12px; background: #f9fafb; border-radius: 0 0 12px 12px; margin-top: -12px; }
	.action-btn { display: inline-flex; align-items: center; gap: 6px; padding: 8px 12px; border: none; border-radius: 6px; font-size: 13px; cursor: pointer; transition: all 0.2s; text-decoration: none; }
	.action-btn.edit { background: #e5e7eb; color: #1f2937; }
	.action-btn.edit:hover { background: #d1d5db; }
	.action-btn.publish { background: #dbeafe; color: #1d4ed8; }
	.action-btn.publish:hover { background: #bfdbfe; }
	.action-btn.delete { background: #fee2e2; color: #dc2626; margin-left: auto; }
	.action-btn.delete:hover { background: #fecaca; }

	.pagination { display: flex; align-items: center; justify-content: center; gap: 16px; margin-top: 32px; }
	.pagination button { padding: 8px 12px; background: #f3f4f6; border: none; border-radius: 6px; cursor: pointer; }
	.pagination button:disabled { opacity: 0.5; cursor: not-allowed; }
	.pagination span { font-size: 14px; color: #6b7280; }

	@media (max-width: 768px) {
		.page-header { flex-direction: column; align-items: flex-start; gap: 16px; }
		.filters { flex-direction: column; }
		.search-box { max-width: none; }
		.courses-grid { grid-template-columns: 1fr; }
	}
</style>
