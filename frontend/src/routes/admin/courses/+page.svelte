<script lang="ts">
	/**
	 * Admin Courses List Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * 
	 * Features streamlined QuickCreate modal for instant course creation
	 * that redirects directly to the Page Builder for visual design.
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
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

	// QuickCreate Modal State
	let showQuickCreate = $state(false);
	let quickCreateTitle = $state('');
	let quickCreateSlug = $state('');
	let quickCreateDescription = $state('');
	let quickCreateLoading = $state(false);
	let quickCreateError = $state<string | null>(null);

	// Auto-generate slug from title
	const generateSlug = (title: string) => {
		return title
			.toLowerCase()
			.replace(/[^a-z0-9\s-]/g, '')
			.replace(/\s+/g, '-')
			.replace(/-+/g, '-')
			.trim();
	};

	// Handle title input - auto-generate slug
	const handleTitleInput = () => {
		if (!quickCreateSlug || quickCreateSlug === generateSlug(quickCreateTitle.slice(0, -1))) {
			quickCreateSlug = generateSlug(quickCreateTitle);
		}
	};

	// Create course and redirect to page builder
	const handleQuickCreate = async () => {
		if (!quickCreateTitle.trim()) {
			quickCreateError = 'Please enter a course title';
			return;
		}

		quickCreateLoading = true;
		quickCreateError = null;

		try {
			const res = await fetch('/api/admin/courses', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					title: quickCreateTitle.trim(),
					slug: quickCreateSlug || generateSlug(quickCreateTitle),
					description: quickCreateDescription || `Welcome to ${quickCreateTitle}`,
					card_description: quickCreateDescription || `Learn with ${quickCreateTitle}`,
					price_cents: 0,
					is_free: true,
					status: 'draft'
				})
			});

			const data = await res.json();

			if (data.success && data.data?.id) {
				// Redirect to page builder with new course
				goto(`/admin/page-builder?course=${data.data.id}`);
			} else {
				quickCreateError = data.error || 'Failed to create course';
			}
		} catch (e) {
			quickCreateError = 'Failed to connect to server';
		} finally {
			quickCreateLoading = false;
		}
	};

	// Close modal and reset
	const closeQuickCreate = () => {
		showQuickCreate = false;
		quickCreateTitle = '';
		quickCreateSlug = '';
		quickCreateDescription = '';
		quickCreateError = null;
	};

	// Handle keyboard in modal
	const handleModalKeydown = (e: KeyboardEvent) => {
		if (e.key === 'Escape') closeQuickCreate();
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleQuickCreate();
		}
	};

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
		<button class="btn-create" onclick={() => showQuickCreate = true}>
			<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" x2="12" y1="5" y2="19"/><line x1="5" x2="19" y1="12" y2="12"/></svg>
			Create Course
		</button>
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
			<button class="btn-primary" onclick={() => showQuickCreate = true}>Create Course</button>
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
						<a href="/admin/page-builder?course={course.id}" class="action-btn builder">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/></svg>
							Builder
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

<!-- QuickCreate Modal -->
{#if showQuickCreate}
	<div class="modal-overlay" onclick={closeQuickCreate} onkeydown={handleModalKeydown} role="dialog" aria-modal="true" aria-labelledby="modal-title" tabindex="-1">
		<div class="modal" role="document">
			<button class="modal-close" onclick={closeQuickCreate} aria-label="Close">
				<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
			</button>

			<div class="modal-icon">
				<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/></svg>
			</div>

			<h2 id="modal-title">Create New Course</h2>
			<p class="modal-subtitle">Enter a name and we'll take you straight to the visual builder</p>

			{#if quickCreateError}
				<div class="modal-error">
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/></svg>
					{quickCreateError}
				</div>
			{/if}

			<div class="modal-form">
				<div class="form-group">
					<label for="course-title">Course Title <span class="required">*</span></label>
					<input 
						id="course-title"
						type="text" 
						placeholder="e.g., Options Trading Mastery"
						bind:value={quickCreateTitle}
						oninput={handleTitleInput}
						disabled={quickCreateLoading}
					/>
				</div>

				<div class="form-group">
					<label for="course-slug">URL Slug</label>
					<div class="slug-preview">
						<span class="slug-prefix">/classes/</span>
						<input 
							id="course-slug"
							type="text" 
							placeholder="options-trading-mastery"
							bind:value={quickCreateSlug}
							disabled={quickCreateLoading}
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="course-desc">Short Description <span class="optional">(optional)</span></label>
					<textarea 
						id="course-desc"
						placeholder="What will students learn?"
						rows="2"
						bind:value={quickCreateDescription}
						disabled={quickCreateLoading}
					></textarea>
				</div>
			</div>

			<div class="modal-actions">
				<button class="btn-cancel" onclick={closeQuickCreate} disabled={quickCreateLoading}>Cancel</button>
				<button class="btn-create-course" onclick={handleQuickCreate} disabled={quickCreateLoading || !quickCreateTitle.trim()}>
					{#if quickCreateLoading}
						<span class="spinner-small"></span>
						Creating...
					{:else}
						<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/></svg>
						Open in Builder
					{/if}
				</button>
			</div>

			<p class="modal-hint">
				<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
				You can add pricing, modules, and videos in the builder
			</p>
		</div>
	</div>
{/if}

<style>
	.admin-courses { padding: 24px; max-width: 1400px; margin: 0 auto; }

	.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
	.header-content h1 { font-size: 28px; font-weight: 700; color: #1f2937; margin: 0; }
	.subtitle { color: #6b7280; margin: 4px 0 0; font-size: 14px; }

	/* Buttons - RTP Admin Color System */
	.btn-primary, .btn-create { display: inline-flex; align-items: center; gap: 8px; padding: 10px 20px; background: var(--admin-btn-primary-bg); color: var(--admin-btn-primary-text); border-radius: 8px; text-decoration: none; font-weight: 500; font-size: 14px; transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1); border: none; cursor: pointer; box-shadow: 0 2px 8px rgba(230, 184, 0, 0.25); }
	.btn-primary:hover, .btn-create:hover { transform: translateY(-1px); box-shadow: 0 4px 16px rgba(230, 184, 0, 0.4); }
	.btn-create { background: var(--admin-btn-primary-bg); }
	.btn-create:hover { transform: translateY(-2px); box-shadow: 0 6px 20px rgba(230, 184, 0, 0.4); }

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
	.action-btn.builder { background: #f0f9ff; color: #143e59; }
	.action-btn.builder:hover { background: #e0f2fe; }
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

	/* QuickCreate Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 24px;
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.modal {
		background: #fff;
		border-radius: 16px;
		padding: 32px;
		width: 100%;
		max-width: 480px;
		position: relative;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
		animation: slideUp 0.3s ease;
	}

	@keyframes slideUp {
		from { opacity: 0; transform: translateY(20px); }
		to { opacity: 1; transform: translateY(0); }
	}

	.modal-close {
		position: absolute;
		top: 16px;
		right: 16px;
		background: none;
		border: none;
		color: #9ca3af;
		cursor: pointer;
		padding: 8px;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: #f3f4f6;
		color: #1f2937;
	}

	.modal-icon {
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #143E59 0%, #1E73BE 100%);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 20px;
		color: #fff;
	}

	.modal h2 {
		font-size: 24px;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 8px;
		text-align: center;
	}

	.modal-subtitle {
		color: #6b7280;
		text-align: center;
		margin: 0 0 24px;
		font-size: 14px;
	}

	.modal-error {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 16px;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		color: #dc2626;
		font-size: 14px;
		margin-bottom: 20px;
	}

	.modal-form {
		display: flex;
		flex-direction: column;
		gap: 16px;
		margin-bottom: 24px;
	}

	.modal-form .form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.modal-form label {
		font-size: 13px;
		font-weight: 600;
		color: #374151;
	}

	.modal-form .required {
		color: #dc2626;
	}

	.modal-form .optional {
		color: #9ca3af;
		font-weight: 400;
	}

	.modal-form input,
	.modal-form textarea {
		padding: 12px 14px;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		font-size: 15px;
		transition: all 0.2s;
	}

	.modal-form input:focus,
	.modal-form textarea:focus {
		outline: none;
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.modal-form input:disabled,
	.modal-form textarea:disabled {
		background: #f9fafb;
		cursor: not-allowed;
	}

	.slug-preview {
		display: flex;
		align-items: center;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		overflow: hidden;
	}

	.slug-prefix {
		padding: 12px;
		background: #f3f4f6;
		color: #6b7280;
		font-size: 14px;
		font-family: monospace;
		border-right: 1px solid #e5e7eb;
	}

	.slug-preview input {
		border: none;
		border-radius: 0;
		flex: 1;
		font-family: monospace;
	}

	.slug-preview input:focus {
		box-shadow: none;
	}

	.modal-actions {
		display: flex;
		gap: 12px;
	}

	.btn-cancel {
		flex: 1;
		padding: 12px 20px;
		background: #f3f4f6;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		color: #374151;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-cancel:hover {
		background: #e5e7eb;
	}

	.btn-cancel:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-create-course {
		flex: 2;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px 24px;
		background: linear-gradient(135deg, #143E59 0%, #1E73BE 100%);
		border: none;
		border-radius: 8px;
		font-size: 15px;
		font-weight: 600;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-create-course:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
	}

	.btn-create-course:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
		box-shadow: none;
	}

	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255,255,255,0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	.modal-hint {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		margin-top: 16px;
		font-size: 13px;
		color: #9ca3af;
	}

	@media (max-width: 480px) {
		.modal {
			padding: 24px;
		}

		.modal-actions {
			flex-direction: column-reverse;
		}

		.btn-cancel,
		.btn-create-course {
			flex: none;
			width: 100%;
		}
	}
</style>
