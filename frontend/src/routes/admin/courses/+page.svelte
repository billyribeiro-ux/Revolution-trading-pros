<script lang="ts">
	/**
	 * Admin Courses List Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 *
	 * Features streamlined QuickCreate modal for instant course creation
	 * that redirects directly to the Page Builder for visual design.
	 * Includes enterprise CourseDetailDrawer, CourseFormModal, and ModuleFormModal.
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { CourseCard } from '$lib/components/courses';
	import { adminFetch } from '$lib/utils/adminFetch';
	import CourseDetailDrawer from '$lib/components/admin/CourseDetailDrawer.svelte';
	import CourseFormModal from '$lib/components/admin/CourseFormModal.svelte';
	import ModuleFormModal from '$lib/components/admin/ModuleFormModal.svelte';
	import type { Course as APICourse, CourseModule } from '$lib/api/courses';

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

	// Enterprise Components State
	let showDetailDrawer = $state(false);
	let showFormModal = $state(false);
	let showModuleModal = $state(false);
	let selectedCourseId = $state<string | null>(null);
	let selectedCourse = $state<APICourse | null>(null);
	let selectedModule = $state<CourseModule | null>(null);
	let formModalMode = $state<'create' | 'edit'>('create');
	let moduleModalMode = $state<'create' | 'edit'>('create');

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
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch('/api/admin/courses', {
				method: 'POST',
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

			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses?${params}`);

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
		if (!confirm('Are you sure you want to delete this course? This action cannot be undone.'))
			return;

		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${courseId}`, { method: 'DELETE' });
			if (data.success) {
				courses = courses.filter((c) => c.id !== courseId);
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
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${course.id}/${endpoint}`, {
				method: 'POST'
			});
			if (data.success) {
				course.is_published = !course.is_published;
				course.status = course.is_published ? 'published' : 'draft';
				courses = [...courses];
			}
		} catch {
			alert('Failed to update course status');
		}
	};

	// Enterprise component handlers
	const openCourseDetail = (course: Course) => {
		selectedCourseId = course.id;
		selectedCourse = course as unknown as APICourse;
		showDetailDrawer = true;
	};

	const openCreateCourseModal = () => {
		selectedCourse = null;
		formModalMode = 'create';
		showFormModal = true;
	};

	const openEditCourseModal = (course: APICourse) => {
		selectedCourse = course;
		formModalMode = 'edit';
		showFormModal = true;
		showDetailDrawer = false;
	};

	const openAddModuleModal = (courseId: string) => {
		selectedCourseId = courseId;
		selectedModule = null;
		moduleModalMode = 'create';
		showModuleModal = true;
	};

	const openEditModuleModal = (module: CourseModule) => {
		selectedModule = module;
		moduleModalMode = 'edit';
		showModuleModal = true;
	};

	const handleCourseSaved = () => {
		fetchCourses();
	};

	const handleModuleSaved = () => {
		// Refresh drawer data if open
		if (showDetailDrawer && selectedCourseId) {
			// Force drawer refresh by toggling
			const id = selectedCourseId;
			selectedCourseId = null;
			setTimeout(() => {
				selectedCourseId = id;
			}, 10);
		}
	};

	// Svelte 5: Initialize on mount
	$effect(() => {
		if (browser) fetchCourses();
	});
</script>

<svelte:head>
	<title>Manage Courses | Admin</title>
</svelte:head>

<div class="admin-courses">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<header class="page-header">
			<h1>Courses</h1>
			<p class="subtitle">Manage your courses and learning content</p>
		</header>

		<div class="actions-row">
			<div class="search-box">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="20"
					height="20"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					><circle cx="11" cy="11" r="8" /><line x1="21" x2="16.65" y1="21" y2="16.65" /></svg
				>
				<input
					id="courses-searchquery" name="courses-searchquery" type="text"
					placeholder="Search courses..."
					bind:value={searchQuery}
					onkeydown={(e) => e.key === 'Enter' && handleSearch()}
				/>
			</div>
			<select bind:value={statusFilter} onchange={handleSearch} class="filter-select">
				<option value="all">All Status</option>
				<option value="draft">Draft</option>
				<option value="published">Published</option>
				<option value="archived">Archived</option>
			</select>
			<button class="btn-primary" onclick={() => (showQuickCreate = true)}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					><line x1="12" x2="12" y1="5" y2="19" /><line x1="5" x2="19" y1="12" y2="12" /></svg
				>
				Create Course
			</button>
		</div>

		<div class="templates-count">{total} courses total</div>

		{#if loading}
			<div class="loading">
				<div class="spinner"></div>
				<p>Loading courses...</p>
			</div>
		{:else if error}
			<div class="error-state">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="48"
					height="48"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					><circle cx="12" cy="12" r="10" /><line x1="12" x2="12" y1="8" y2="12" /><line
						x1="12"
						x2="12.01"
						y1="16"
						y2="16"
					/></svg
				>
				<p>{error}</p>
				<button onclick={fetchCourses}>Retry</button>
			</div>
		{:else if courses.length === 0}
			<div class="empty-state">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="64"
					height="64"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" /></svg
				>
				<h3>No courses found</h3>
				<p>Get started by creating your first course</p>
				<button class="btn-primary" onclick={() => (showQuickCreate = true)}>Create Course</button>
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
							<button class="action-btn view" onclick={() => openCourseDetail(course)}>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" /><circle
										cx="12"
										cy="12"
										r="3"
									/></svg
								>
								View
							</button>
							<a href="/admin/courses/{course.id}" class="action-btn edit">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									><path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" /></svg
								>
								Edit
							</a>
							<a href="/admin/page-builder?course={course.id}" class="action-btn builder">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									><rect width="7" height="7" x="3" y="3" rx="1" /><rect
										width="7"
										height="7"
										x="14"
										y="3"
										rx="1"
									/><rect width="7" height="7" x="14" y="14" rx="1" /><rect
										width="7"
										height="7"
										x="3"
										y="14"
										rx="1"
									/></svg
								>
								Builder
							</a>
							<button class="action-btn publish" onclick={() => handlePublish(course)}>
								{#if course.is_published}
									<svg
										xmlns="http://www.w3.org/2000/svg"
										width="16"
										height="16"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										><path
											d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"
										/><line x1="1" x2="23" y1="1" y2="23" /></svg
									>
									Unpublish
								{:else}
									<svg
										xmlns="http://www.w3.org/2000/svg"
										width="16"
										height="16"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" /><circle
											cx="12"
											cy="12"
											r="3"
										/></svg
									>
									Publish
								{/if}
							</button>
							<button
								class="action-btn delete"
								onclick={() => handleDelete(course.id)}
								aria-label="Delete course"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									><path d="M3 6h18" /><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /><path
										d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"
									/></svg
								>
							</button>
						</div>
					</div>
				{/each}
			</div>

			{#if totalPages > 1}
				<div class="pagination">
					<button
						disabled={currentPage === 1}
						onclick={() => {
							currentPage--;
							fetchCourses();
						}}
						aria-label="Previous page"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"><path d="m15 18-6-6 6-6" /></svg
						>
					</button>
					<span>Page {currentPage} of {totalPages}</span>
					<button
						disabled={currentPage === totalPages}
						onclick={() => {
							currentPage++;
							fetchCourses();
						}}
						aria-label="Next page"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"><path d="m9 18 6-6-6-6" /></svg
						>
					</button>
				</div>
			{/if}
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<!-- QuickCreate Modal -->
{#if showQuickCreate}
	<div
		class="modal-overlay"
		onclick={(e) => {
			if (e.target === e.currentTarget) closeQuickCreate();
		}}
		onkeydown={handleModalKeydown}
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		tabindex="-1"
	>
		<div class="modal" role="document">
			<button class="modal-close" onclick={closeQuickCreate} aria-label="Close">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="20"
					height="20"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"><path d="M18 6 6 18" /><path d="m6 6 12 12" /></svg
				>
			</button>

			<div class="modal-icon">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="32"
					height="32"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					><rect width="7" height="7" x="3" y="3" rx="1" /><rect
						width="7"
						height="7"
						x="14"
						y="3"
						rx="1"
					/><rect width="7" height="7" x="14" y="14" rx="1" /><rect
						width="7"
						height="7"
						x="3"
						y="14"
						rx="1"
					/></svg
				>
			</div>

			<h2 id="modal-title">Create New Course</h2>
			<p class="modal-subtitle">Enter a name and we'll take you straight to the visual builder</p>

			{#if quickCreateError}
				<div class="modal-error">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						><circle cx="12" cy="12" r="10" /><line x1="12" x2="12" y1="8" y2="12" /><line
							x1="12"
							x2="12.01"
							y1="16"
							y2="16"
						/></svg
					>
					{quickCreateError}
				</div>
			{/if}

			<div class="modal-form">
				<div class="form-group">
					<label for="course-title">Course Title <span class="required">*</span></label>
					<input
						id="course-title" name="course-title"
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
							id="course-slug" name="course-slug"
							type="text"
							placeholder="options-trading-mastery"
							bind:value={quickCreateSlug}
							disabled={quickCreateLoading}
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="course-desc">Short Description <span class="optional">(optional)</span></label
					>
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
				<button class="btn-cancel" onclick={closeQuickCreate} disabled={quickCreateLoading}
					>Cancel</button
				>
				<button
					class="btn-create-course"
					onclick={handleQuickCreate}
					disabled={quickCreateLoading || !quickCreateTitle.trim()}
				>
					{#if quickCreateLoading}
						<span class="spinner-small"></span>
						Creating...
					{:else}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="18"
							height="18"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							><rect width="7" height="7" x="3" y="3" rx="1" /><rect
								width="7"
								height="7"
								x="14"
								y="3"
								rx="1"
							/><rect width="7" height="7" x="14" y="14" rx="1" /><rect
								width="7"
								height="7"
								x="3"
								y="14"
								rx="1"
							/></svg
						>
						Open in Builder
					{/if}
				</button>
			</div>

			<p class="modal-hint">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					><circle cx="12" cy="12" r="10" /><path d="M12 16v-4" /><path d="M12 8h.01" /></svg
				>
				You can add pricing, modules, and videos in the builder
			</p>
		</div>
	</div>
{/if}

<!-- Enterprise Components -->
<CourseDetailDrawer
	isOpen={showDetailDrawer}
	courseId={selectedCourseId}
	onClose={() => {
		showDetailDrawer = false;
		selectedCourseId = null;
	}}
	onEdit={openEditCourseModal}
	onEditModule={openEditModuleModal}
	onAddModule={openAddModuleModal}
	onRefresh={fetchCourses}
/>

<CourseFormModal
	isOpen={showFormModal}
	mode={formModalMode}
	course={selectedCourse}
	onClose={() => {
		showFormModal = false;
		selectedCourse = null;
	}}
	onSaved={handleCourseSaved}
/>

<ModuleFormModal
	isOpen={showModuleModal}
	mode={moduleModalMode}
	courseId={selectedCourseId}
	module={selectedModule}
	onClose={() => {
		showModuleModal = false;
		selectedModule = null;
	}}
	onSaved={handleModuleSaved}
/>

<style>
	/* Page Layout - Consistent Admin Style */
	.admin-courses {
		background: linear-gradient(
			135deg,
			var(--bg-base) 0%,
			var(--bg-elevated) 50%,
			var(--bg-base) 100%
		);
		position: relative;
		overflow: hidden;
	}

	/* Centered Header */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}

	/* Actions Row - Centered */
	.actions-row {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	/* Templates Count */
	.templates-count {
		font-size: 0.8125rem;
		color: #64748b;
		margin-bottom: 1rem;
	}

	/* Search Box - Uses global .search-box styles from admin-page-layout.css */
	.search-box {
		min-width: 280px;
	}

	.search-box svg {
		flex-shrink: 0;
		color: #64748b;
	}

	.search-box input {
		width: 100%;
	}

	/* Filter Select */
	.filter-select {
		padding: 0.5rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.filter-select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	/* Buttons */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	/* Loading State */
	.loading,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-state svg,
	.empty-state svg {
		color: #94a3b8;
		margin-bottom: 16px;
	}
	.error-state p,
	.empty-state p {
		color: #94a3b8;
		margin: 0 0 1.5rem;
	}
	.empty-state h3 {
		font-size: 1.125rem;
		color: #f1f5f9;
		margin: 0 0 0.5rem;
	}

	.empty-state {
		background: rgba(30, 41, 59, 0.4);
		border-radius: 12px;
		border: 1px solid rgba(148, 163, 184, 0.1);
	}

	/* Courses Grid */
	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	/* Course Card Wrapper */
	.course-card-wrapper {
		position: relative;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		overflow: hidden;
	}

	/* Status Badge */
	.card-status {
		position: absolute;
		top: 12px;
		right: 12px;
		z-index: 10;
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 0.25rem 0.5rem;
		background: rgba(148, 163, 184, 0.15);
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		color: #94a3b8;
		text-transform: capitalize;
	}

	.status-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: #94a3b8;
	}

	.card-status.published {
		background: rgba(34, 197, 94, 0.15);
		color: #22c55e;
	}

	.card-status.published .status-dot {
		background: #22c55e;
	}

	/* Card Actions */
	.card-actions {
		display: flex;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.6);
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
	}

	.action-btn:hover {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.action-btn.view {
		background: rgba(59, 130, 246, 0.1);
		color: #60a5fa;
	}
	.action-btn.view:hover {
		background: rgba(59, 130, 246, 0.2);
	}

	.action-btn.edit {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
	}
	.action-btn.edit:hover {
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
	}

	.action-btn.builder {
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
	}
	.action-btn.builder:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.action-btn.publish {
		background: rgba(59, 130, 246, 0.1);
		color: #60a5fa;
	}
	.action-btn.publish:hover {
		background: rgba(59, 130, 246, 0.2);
	}

	.action-btn.delete {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		margin-left: auto;
	}
	.action-btn.delete:hover {
		background: rgba(239, 68, 68, 0.2);
		border-color: rgba(239, 68, 68, 0.3);
	}

	/* Pagination */
	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		margin-top: 2rem;
	}

	.pagination button {
		padding: 0.5rem 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.pagination button:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.pagination button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination span {
		font-size: 0.875rem;
		color: #64748b;
	}

	@media (max-width: 768px) {
		.actions-row {
			flex-direction: column;
			gap: 0.75rem;
		}
		.search-box input {
			width: 100%;
		}
		.courses-grid {
			grid-template-columns: 1fr;
		}
	}

	/* QuickCreate Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1.5rem;
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.modal {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 16px;
		padding: 2rem;
		width: 100%;
		max-width: 480px;
		position: relative;
		box-shadow: 0 25px 50px rgba(0, 0, 0, 0.5);
		animation: slideUp 0.3s ease;
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.modal-close {
		position: absolute;
		top: 1rem;
		right: 1rem;
		background: none;
		border: none;
		color: #94a3b8;
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(148, 163, 184, 0.1);
		color: #f1f5f9;
	}

	.modal-icon {
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 1.25rem;
		color: #fff;
	}

	.modal h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem;
		text-align: center;
	}

	.modal-subtitle {
		color: #94a3b8;
		text-align: center;
		margin: 0 0 1.5rem;
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.modal-error {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: #fca5a5;
		font-size: 0.875rem;
		margin-bottom: 1.25rem;
	}

	.modal-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.modal-form .form-group {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.modal-form label {
		font-size: 0.8125rem;
		font-weight: 600;
		color: #cbd5e1;
	}

	.modal-form .required {
		color: #f87171;
	}

	.modal-form .optional {
		color: #64748b;
		font-weight: 400;
	}

	.modal-form input,
	.modal-form textarea {
		padding: 0.75rem 0.875rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		font-size: 0.9375rem;
		color: #f1f5f9;
		transition: all 0.2s;
	}

	.modal-form input::placeholder,
	.modal-form textarea::placeholder {
		color: #64748b;
	}

	.modal-form input:focus,
	.modal-form textarea:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.modal-form input:disabled,
	.modal-form textarea:disabled {
		background: rgba(15, 23, 42, 0.6);
		cursor: not-allowed;
		opacity: 0.6;
	}

	.slug-preview {
		display: flex;
		align-items: center;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		overflow: hidden;
	}

	.slug-prefix {
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.6);
		color: #64748b;
		font-size: 0.875rem;
		font-family: monospace;
		border-right: 1px solid rgba(148, 163, 184, 0.2);
	}

	.slug-preview input {
		border: none;
		border-radius: 0;
		flex: 1;
		font-family: monospace;
		background: transparent;
	}

	.slug-preview input:focus {
		box-shadow: none;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-cancel {
		flex: 1;
		padding: 0.75rem 1.25rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		color: #cbd5e1;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-cancel:hover {
		background: rgba(100, 116, 139, 0.3);
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
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		border: none;
		border-radius: 8px;
		font-size: 0.9375rem;
		font-weight: 600;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-create-course:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
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
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	.modal-hint {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		margin-top: 1rem;
		font-size: 0.8125rem;
		color: #64748b;
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
