<script lang="ts">
	/**
	 * CourseDetailDrawer - Full Course Profile Drawer
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Enterprise-grade slide-out drawer showing complete course details
	 * with tabs for modules, enrollments, and analytics.
	 */
	import {
		adminCoursesApi,
		type Course,
		type CourseWithContent,
		type CourseModule
	} from '$lib/api/courses';
	import {
		IconX,
		IconBook,
		IconUsers,
		IconChartBar,
		IconEdit,
		IconEye,
		IconEyeOff,
		IconPlus,
		IconTrash,
		IconGripVertical,
		IconClock,
		IconCalendar,
		IconCurrencyDollar,
		IconCheck,
		IconAlertTriangle,
		IconPlayerPlay,
		IconVideo,
		IconFile
	} from '$lib/icons';
	import ConfirmationModal from './ConfirmationModal.svelte';

	interface Props {
		isOpen: boolean;
		courseId: string | null;
		onClose: () => void;
		onEdit?: (course: Course) => void;
		onEditModule?: (module: CourseModule) => void;
		onAddModule?: (courseId: string) => void;
		onRefresh?: () => void;
	}

	let props: Props = $props();

	// Destructure for internal use
	const isOpen = $derived(props.isOpen);
	const courseId = $derived(props.courseId);
	const onClose = $derived(props.onClose);
	const onEdit = $derived(props.onEdit);
	const onEditModule = $derived(props.onEditModule);
	const onAddModule = $derived(props.onAddModule);
	const onRefresh = $derived(props.onRefresh);

	// State
	let courseData = $state<CourseWithContent | null>(null);
	let isLoading = $state(false);
	let error = $state('');
	let activeTab = $state<'modules' | 'enrollments' | 'analytics'>('modules');

	// Action modals
	let showPublishModal = $state(false);
	let showDeleteModuleModal = $state(false);
	let selectedModuleId = $state<number | null>(null);
	let isProcessingAction = $state(false);

	// Expanded modules
	let expandedModules = $state<Set<number>>(new Set());

	// Load course data when drawer opens
	$effect(() => {
		if (isOpen && courseId) {
			loadCourseData();
		} else {
			courseData = null;
			activeTab = 'modules';
			error = '';
			expandedModules = new Set();
		}
	});

	$effect(() => {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
		return () => {
			document.body.style.overflow = '';
		};
	});

	async function loadCourseData() {
		if (!courseId) return;

		isLoading = true;
		error = '';

		try {
			courseData = await adminCoursesApi.get(courseId);
			// Auto-expand first module
			if (courseData.modules.length > 0) {
				expandedModules = new Set([courseData.modules[0].id]);
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load course data';
		} finally {
			isLoading = false;
		}
	}

	async function handlePublishToggle() {
		if (!courseData) return;

		isProcessingAction = true;
		try {
			if (courseData.is_published) {
				await adminCoursesApi.unpublish(courseData.id);
			} else {
				await adminCoursesApi.publish(courseData.id);
			}
			await loadCourseData();
			showPublishModal = false;
			onRefresh?.();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update publish status';
		} finally {
			isProcessingAction = false;
		}
	}

	async function handleDeleteModule() {
		if (!courseData || selectedModuleId === null) return;

		isProcessingAction = true;
		try {
			await adminCoursesApi.deleteModule(courseData.id, selectedModuleId);
			await loadCourseData();
			showDeleteModuleModal = false;
			selectedModuleId = null;
			onRefresh?.();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete module';
		} finally {
			isProcessingAction = false;
		}
	}

	function toggleModuleExpand(moduleId: number) {
		const newSet = new Set(expandedModules);
		if (newSet.has(moduleId)) {
			newSet.delete(moduleId);
		} else {
			newSet.add(moduleId);
		}
		expandedModules = newSet;
	}

	function formatDate(dateStr: string | null | undefined): string {
		if (!dateStr) return 'N/A';
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function formatCurrency(cents: number | null | undefined): string {
		if (cents === null || cents === undefined) return 'Free';
		if (cents === 0) return 'Free';
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(cents / 100);
	}

	function formatDuration(minutes: number | null | undefined): string {
		if (!minutes) return '0 min';
		if (minutes < 60) return `${minutes} min`;
		const hours = Math.floor(minutes / 60);
		const mins = minutes % 60;
		return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
	}

	function getStatusColor(status: string | boolean): string {
		if (typeof status === 'boolean') {
			return status ? 'var(--admin-success)' : 'var(--admin-text-muted)';
		}
		switch (status) {
			case 'published':
				return 'var(--admin-success)';
			case 'draft':
				return 'var(--admin-warning)';
			case 'archived':
				return 'var(--admin-text-muted)';
			default:
				return 'var(--admin-text-muted)';
		}
	}

	function handleBackdropClick(e: MouseEvent | KeyboardEvent) {
		if (e.target === e.currentTarget) {
			onClose();
		}
	}
</script>

{#if isOpen}
	<div
		class="drawer-backdrop"
		role="presentation"
		onclick={handleBackdropClick}
		onkeydown={(e) => {
			if (e.key === 'Escape') handleBackdropClick(e);
		}}
	>
		<aside class="drawer" class:open={isOpen}>
			{#if isLoading}
				<div class="loading-state">
					<div class="spinner-large"></div>
					<p>Loading course data...</p>
				</div>
			{:else if error && !courseData}
				<div class="error-state">
					<IconAlertTriangle size={48} />
					<p>{error}</p>
					<button type="button" class="btn-retry" onclick={loadCourseData}>Try Again</button>
				</div>
			{:else if courseData}
				<!-- Header -->
				<header class="drawer-header">
					<div class="course-thumbnail">
						{#if courseData.card_image_url}
							<img src={courseData.card_image_url} alt={courseData.title} />
						{:else}
							<IconBook size={32} />
						{/if}
					</div>
					<div class="course-info">
						<h2 class="course-title">{courseData.title}</h2>
						<p class="course-slug">/{courseData.slug}</p>
						<div class="course-badges">
							<span
								class="status-badge"
								style="--badge-color: {getStatusColor(
									courseData.status || (courseData.is_published ? 'published' : 'draft')
								)}"
							>
								{courseData.status || (courseData.is_published ? 'Published' : 'Draft')}
							</span>
							{#if courseData.level}
								<span class="level-badge">{courseData.level}</span>
							{/if}
							{#if courseData.is_free}
								<span class="free-badge">Free</span>
							{/if}
						</div>
					</div>
					<button type="button" class="btn-close" onclick={onClose} aria-label="Close">
						<IconX size={24} />
					</button>
				</header>

				{#if error}
					<div class="error-banner">
						<IconAlertTriangle size={16} />
						{error}
					</div>
				{/if}

				<!-- Quick Stats -->
				<div class="quick-stats">
					<div class="stat-item">
						<span class="stat-value">{courseData.module_count || courseData.modules.length}</span>
						<span class="stat-label">Modules</span>
					</div>
					<div class="stat-item">
						<span class="stat-value"
							>{courseData.lesson_count ||
								courseData.modules.reduce((sum, m) => sum + (m.lessons?.length || 0), 0)}</span
						>
						<span class="stat-label">Lessons</span>
					</div>
					<div class="stat-item">
						<span class="stat-value">{courseData.enrollment_count || 0}</span>
						<span class="stat-label">Students</span>
					</div>
					<div class="stat-item">
						<span class="stat-value">{formatCurrency(courseData.price_cents)}</span>
						<span class="stat-label">Price</span>
					</div>
				</div>

				<!-- Action Buttons -->
				<div class="action-buttons">
					<button type="button" class="btn-action" onclick={() => onEdit?.(courseData!)}>
						<IconEdit size={16} />
						Edit
					</button>
					<a href="/admin/page-builder?course={courseData.id}" class="btn-action builder">
						<IconPlayerPlay size={16} />
						Builder
					</a>
					<button
						type="button"
						class="btn-action"
						class:success={!courseData.is_published}
						class:warning={courseData.is_published}
						onclick={() => (showPublishModal = true)}
					>
						{#if courseData.is_published}
							<IconEyeOff size={16} />
							Unpublish
						{:else}
							<IconEye size={16} />
							Publish
						{/if}
					</button>
				</div>

				<!-- Tabs -->
				<nav class="drawer-tabs">
					<button
						type="button"
						class="tab"
						class:active={activeTab === 'modules'}
						onclick={() => (activeTab = 'modules')}
					>
						<IconBook size={16} />
						Modules
					</button>
					<button
						type="button"
						class="tab"
						class:active={activeTab === 'enrollments'}
						onclick={() => (activeTab = 'enrollments')}
					>
						<IconUsers size={16} />
						Enrollments
					</button>
					<button
						type="button"
						class="tab"
						class:active={activeTab === 'analytics'}
						onclick={() => (activeTab = 'analytics')}
					>
						<IconChartBar size={16} />
						Analytics
					</button>
				</nav>

				<!-- Tab Content -->
				<div class="drawer-content">
					{#if activeTab === 'modules'}
						<div class="tab-content">
							<!-- Add Module Button -->
							<button
								type="button"
								class="btn-add-module"
								onclick={() => onAddModule?.(courseData!.id)}
							>
								<IconPlus size={18} />
								Add Module
							</button>

							{#if courseData.modules.length === 0}
								<div class="empty-state">
									<IconBook size={48} />
									<p>No modules yet</p>
									<span class="empty-hint">Click "Add Module" to create your first module</span>
								</div>
							{:else}
								<div class="modules-list">
									{#each courseData.modules as module, idx}
										<div class="module-card" class:expanded={expandedModules.has(module.id)}>
											<div
												class="module-header"
												role="button"
												tabindex="0"
												onclick={() => toggleModuleExpand(module.id)}
												onkeydown={(e) => {
													if (e.key === 'Enter' || e.key === ' ') {
														e.preventDefault();
														toggleModuleExpand(module.id);
													}
												}}
											>
												<div class="module-drag">
													<IconGripVertical size={16} />
												</div>
												<div class="module-number">{idx + 1}</div>
												<div class="module-info">
													<span class="module-title">{module.title}</span>
													<span class="module-meta">
														{module.lessons?.length || 0} lessons
														{#if module.total_duration_minutes}
															• {formatDuration(module.total_duration_minutes)}
														{/if}
													</span>
												</div>
												<div class="module-status">
													{#if module.is_published}
														<span class="published-indicator">
															<IconCheck size={14} />
														</span>
													{/if}
												</div>
												<div class="module-actions">
													<button
														type="button"
														class="btn-module-action"
														onclick={() => onEditModule?.(module)}
														title="Edit module"
													>
														<IconEdit size={14} />
													</button>
													<button
														type="button"
														class="btn-module-action danger"
														onclick={() => {
															selectedModuleId = module.id;
															showDeleteModuleModal = true;
														}}
														title="Delete module"
													>
														<IconTrash size={14} />
													</button>
												</div>
											</div>

											{#if expandedModules.has(module.id)}
												<div class="module-lessons">
													{#if !module.lessons || module.lessons.length === 0}
														<div class="no-lessons">No lessons in this module</div>
													{:else}
														{#each module.lessons as lesson}
															<div class="lesson-item">
																<div class="lesson-icon">
																	{#if lesson.bunny_video_guid}
																		<IconVideo size={14} />
																	{:else}
																		<IconFile size={14} />
																	{/if}
																</div>
																<div class="lesson-info">
																	<span class="lesson-title">{lesson.title}</span>
																	{#if lesson.duration_minutes}
																		<span class="lesson-duration"
																			>{lesson.duration_minutes} min</span
																		>
																	{/if}
																</div>
																<div class="lesson-badges">
																	{#if lesson.is_free || lesson.is_preview}
																		<span class="lesson-badge free">Preview</span>
																	{/if}
																	{#if lesson.is_published === false}
																		<span class="lesson-badge draft">Draft</span>
																	{/if}
																</div>
															</div>
														{/each}
													{/if}
												</div>
											{/if}
										</div>
									{/each}
								</div>
							{/if}
						</div>
					{:else if activeTab === 'enrollments'}
						<div class="tab-content">
							<div class="enrollment-stats">
								<div class="enroll-stat-card">
									<div class="enroll-stat-icon">
										<IconUsers size={20} />
									</div>
									<div class="enroll-stat-content">
										<span class="enroll-stat-value">{courseData.enrollment_count || 0}</span>
										<span class="enroll-stat-label">Total Enrolled</span>
									</div>
								</div>
								<div class="enroll-stat-card">
									<div class="enroll-stat-icon success">
										<IconCheck size={20} />
									</div>
									<div class="enroll-stat-content">
										<span class="enroll-stat-value">{courseData.completion_rate || 0}%</span>
										<span class="enroll-stat-label">Completion Rate</span>
									</div>
								</div>
								<div class="enroll-stat-card">
									<div class="enroll-stat-icon accent">
										<IconChartBar size={20} />
									</div>
									<div class="enroll-stat-content">
										<span class="enroll-stat-value"
											>{courseData.avg_rating?.toFixed(1) || 'N/A'}</span
										>
										<span class="enroll-stat-label">Avg Rating</span>
									</div>
								</div>
							</div>

							<div class="info-section">
								<h3 class="section-title">Recent Enrollments</h3>
								<div class="empty-state small">
									<IconUsers size={32} />
									<p>Enrollment data coming soon</p>
								</div>
							</div>
						</div>
					{:else if activeTab === 'analytics'}
						<div class="tab-content">
							<div class="analytics-grid">
								<div class="analytics-card">
									<h4>Course Performance</h4>
									<div class="analytics-metrics">
										<div class="metric-row">
											<span class="metric-label">Total Views</span>
											<span class="metric-value">-</span>
										</div>
										<div class="metric-row">
											<span class="metric-label">Unique Visitors</span>
											<span class="metric-value">-</span>
										</div>
										<div class="metric-row">
											<span class="metric-label">Conversion Rate</span>
											<span class="metric-value">-</span>
										</div>
									</div>
								</div>

								<div class="analytics-card">
									<h4>Engagement</h4>
									<div class="analytics-metrics">
										<div class="metric-row">
											<span class="metric-label">Avg Time on Course</span>
											<span class="metric-value">-</span>
										</div>
										<div class="metric-row">
											<span class="metric-label">Lesson Completion</span>
											<span class="metric-value">{courseData.completion_rate || 0}%</span>
										</div>
										<div class="metric-row">
											<span class="metric-label">Drop-off Rate</span>
											<span class="metric-value">-</span>
										</div>
									</div>
								</div>
							</div>

							<div class="info-section">
								<h3 class="section-title">Course Details</h3>
								<div class="info-grid">
									<div class="info-item">
										<IconCalendar size={16} />
										<div>
											<span class="info-label">Created</span>
											<span class="info-value">{formatDate(courseData.created_at)}</span>
										</div>
									</div>
									<div class="info-item">
										<IconCalendar size={16} />
										<div>
											<span class="info-label">Updated</span>
											<span class="info-value">{formatDate(courseData.updated_at)}</span>
										</div>
									</div>
									<div class="info-item">
										<IconClock size={16} />
										<div>
											<span class="info-label">Total Duration</span>
											<span class="info-value"
												>{formatDuration(
													courseData.total_duration_minutes || courseData.duration_minutes
												)}</span
											>
										</div>
									</div>
									<div class="info-item">
										<IconCurrencyDollar size={16} />
										<div>
											<span class="info-label">Revenue</span>
											<span class="info-value">Coming soon</span>
										</div>
									</div>
								</div>
							</div>

							{#if courseData.instructor_name}
								<div class="info-section">
									<h3 class="section-title">Instructor</h3>
									<div class="instructor-card">
										{#if courseData.instructor_avatar_url}
											<img
												src={courseData.instructor_avatar_url}
												alt={courseData.instructor_name}
												class="instructor-avatar"
											/>
										{:else}
											<div class="instructor-avatar-placeholder">
												{courseData.instructor_name[0].toUpperCase()}
											</div>
										{/if}
										<div class="instructor-info">
											<span class="instructor-name">{courseData.instructor_name}</span>
											{#if courseData.instructor_title}
												<span class="instructor-title">{courseData.instructor_title}</span>
											{/if}
										</div>
									</div>
								</div>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</aside>
	</div>
{/if}

<!-- Publish/Unpublish Modal -->
<ConfirmationModal
	isOpen={showPublishModal}
	title={courseData?.is_published ? 'Unpublish Course' : 'Publish Course'}
	message={courseData?.is_published
		? 'This will hide the course from students. Existing enrollments will retain access.'
		: 'This will make the course visible to students. Make sure all content is ready.'}
	confirmText={courseData?.is_published ? 'Unpublish' : 'Publish'}
	variant={courseData?.is_published ? 'warning' : 'success'}
	isLoading={isProcessingAction}
	onConfirm={handlePublishToggle}
	onCancel={() => (showPublishModal = false)}
/>

<!-- Delete Module Modal -->
<ConfirmationModal
	isOpen={showDeleteModuleModal}
	title="Delete Module"
	message="This will permanently delete this module and all its lessons. This action cannot be undone."
	confirmText="Delete Module"
	variant="danger"
	isLoading={isProcessingAction}
	onConfirm={handleDeleteModule}
	onCancel={() => {
		showDeleteModuleModal = false;
		selectedModuleId = null;
	}}
/>

<style>
	.drawer-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: var(--z-modal, 1000);
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

	.drawer {
		position: fixed;
		top: 0;
		right: 0;
		width: 560px;
		max-width: 100%;
		height: 100vh;
		background: var(--admin-surface-primary);
		border-left: 1px solid var(--admin-border-subtle);
		display: flex;
		flex-direction: column;
		transform: translateX(100%);
		transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		z-index: 1001;
	}

	.drawer.open {
		transform: translateX(0);
	}

	.loading-state,
	.error-state {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		color: var(--admin-text-muted);
	}

	.spinner-large {
		width: 48px;
		height: 48px;
		border: 3px solid var(--admin-border-subtle);
		border-top-color: var(--admin-accent-primary);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.btn-retry {
		background: var(--admin-accent-primary);
		color: #0d1117;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: var(--radius-md, 0.5rem);
		cursor: pointer;
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: var(--admin-error-bg);
		border: 1px solid var(--admin-error-border);
		color: var(--admin-error);
		padding: 0.75rem 1rem;
		margin: 0 1rem;
		border-radius: var(--radius-md, 0.5rem);
		font-size: 0.875rem;
	}

	/* Header */
	.drawer-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.5rem;
		border-bottom: 1px solid var(--admin-border-subtle);
	}

	.course-thumbnail {
		width: 72px;
		height: 48px;
		border-radius: var(--radius-md, 0.5rem);
		background: var(--admin-surface-sunken);
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		color: var(--admin-text-muted);
		flex-shrink: 0;
	}

	.course-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.course-info {
		flex: 1;
		min-width: 0;
	}

	.course-title {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--admin-text-primary);
		margin: 0 0 0.25rem;
		line-height: 1.3;
	}

	.course-slug {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
		font-family: 'JetBrains Mono', monospace;
		margin: 0 0 0.5rem;
	}

	.course-badges {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.5rem;
		background: color-mix(in srgb, var(--badge-color) 15%, transparent);
		color: var(--badge-color);
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.level-badge,
	.free-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.5rem;
		background: var(--admin-surface-hover);
		color: var(--admin-text-secondary);
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 500;
	}

	.free-badge {
		background: var(--admin-success-bg);
		color: var(--admin-success);
	}

	.btn-close {
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: 0.25rem;
		border-radius: var(--radius-sm, 0.25rem);
		transition: all 0.2s ease;
	}

	.btn-close:hover {
		background: var(--admin-surface-hover);
		color: var(--admin-text-primary);
	}

	/* Quick Stats */
	.quick-stats {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		background: var(--admin-surface-sunken);
	}

	.stat-item {
		text-align: center;
	}

	.stat-value {
		display: block;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1rem;
		font-weight: 600;
		color: var(--admin-text-primary);
	}

	.stat-label {
		font-size: 0.6875rem;
		color: var(--admin-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Action Buttons */
	.action-buttons {
		display: flex;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid var(--admin-border-subtle);
	}

	.btn-action {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background: var(--admin-surface-hover);
		border: 1px solid var(--admin-border-subtle);
		color: var(--admin-text-secondary);
		border-radius: var(--radius-md, 0.5rem);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.btn-action:hover {
		background: var(--admin-surface-primary);
		border-color: var(--admin-border-light);
	}

	.btn-action.builder {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.btn-action.builder:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.btn-action.success {
		color: var(--admin-success);
	}

	.btn-action.success:hover {
		background: var(--admin-success-bg);
		border-color: var(--admin-success);
	}

	.btn-action.warning {
		color: var(--admin-warning);
	}

	.btn-action.warning:hover {
		background: var(--admin-warning-bg);
		border-color: var(--admin-warning);
	}

	/* Tabs */
	.drawer-tabs {
		display: flex;
		padding: 0 1rem;
		border-bottom: 1px solid var(--admin-border-subtle);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.875rem 1rem;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--admin-text-muted);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.tab:hover {
		color: var(--admin-text-secondary);
	}

	.tab.active {
		color: var(--admin-accent-primary);
		border-bottom-color: var(--admin-accent-primary);
	}

	/* Content */
	.drawer-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.tab-content {
		animation: fadeIn 0.2s ease;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		color: var(--admin-text-muted);
		text-align: center;
	}

	.empty-state.small {
		padding: 2rem 1rem;
	}

	.empty-state p {
		margin: 0.75rem 0 0.25rem;
	}

	.empty-hint {
		font-size: 0.8125rem;
	}

	/* Add Module Button */
	.btn-add-module {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem;
		background: var(--admin-accent-bg);
		border: 1px dashed var(--admin-accent-primary);
		color: var(--admin-accent-primary);
		border-radius: var(--radius-md, 0.5rem);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
		margin-bottom: 1rem;
	}

	.btn-add-module:hover {
		background: var(--admin-accent-primary);
		border-style: solid;
		color: #0d1117;
	}

	/* Modules List */
	.modules-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.module-card {
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		overflow: hidden;
	}

	.module-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		cursor: pointer;
		transition: background 0.2s ease;
	}

	.module-header:hover {
		background: var(--admin-surface-hover);
	}

	.module-drag {
		color: var(--admin-text-muted);
		cursor: grab;
	}

	.module-number {
		width: 24px;
		height: 24px;
		border-radius: 50%;
		background: var(--admin-accent-bg);
		color: var(--admin-accent-primary);
		font-size: 0.75rem;
		font-weight: 600;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.module-info {
		flex: 1;
		min-width: 0;
	}

	.module-title {
		display: block;
		font-weight: 500;
		color: var(--admin-text-primary);
		font-size: 0.875rem;
	}

	.module-meta {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	.module-status {
		display: flex;
		align-items: center;
	}

	.published-indicator {
		color: var(--admin-success);
	}

	.module-actions {
		display: flex;
		gap: 0.25rem;
	}

	.btn-module-action {
		padding: 0.375rem;
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		border-radius: var(--radius-sm, 0.25rem);
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-module-action:hover {
		background: var(--admin-surface-hover);
		color: var(--admin-text-primary);
	}

	.btn-module-action.danger:hover {
		background: var(--admin-error-bg);
		color: var(--admin-error);
	}

	/* Lessons */
	.module-lessons {
		border-top: 1px solid var(--admin-border-subtle);
		padding: 0.5rem;
		background: var(--admin-surface-primary);
	}

	.no-lessons {
		padding: 1rem;
		text-align: center;
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
	}

	.lesson-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.625rem 0.75rem;
		border-radius: var(--radius-sm, 0.25rem);
		transition: background 0.2s ease;
	}

	.lesson-item:hover {
		background: var(--admin-surface-hover);
	}

	.lesson-icon {
		color: var(--admin-text-muted);
	}

	.lesson-info {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.lesson-title {
		font-size: 0.8125rem;
		color: var(--admin-text-secondary);
	}

	.lesson-duration {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	.lesson-badges {
		display: flex;
		gap: 0.25rem;
	}

	.lesson-badge {
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		font-size: 0.625rem;
		font-weight: 500;
		text-transform: uppercase;
	}

	.lesson-badge.free {
		background: var(--admin-success-bg);
		color: var(--admin-success);
	}

	.lesson-badge.draft {
		background: var(--admin-warning-bg);
		color: var(--admin-warning);
	}

	/* Enrollment Stats */
	.enrollment-stats {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.enroll-stat-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
	}

	.enroll-stat-icon {
		width: 40px;
		height: 40px;
		border-radius: var(--radius-md, 0.5rem);
		background: var(--admin-accent-bg);
		color: var(--admin-accent-primary);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.enroll-stat-icon.success {
		background: var(--admin-success-bg);
		color: var(--admin-success);
	}

	.enroll-stat-icon.accent {
		background: color-mix(in srgb, var(--admin-widget-purple-icon) 15%, transparent);
		color: var(--admin-widget-purple-icon);
	}

	.enroll-stat-content {
		display: flex;
		flex-direction: column;
	}

	.enroll-stat-value {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--admin-text-primary);
	}

	.enroll-stat-label {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	/* Info Sections */
	.info-section {
		margin-bottom: 1.5rem;
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--admin-text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 1rem;
	}

	.info-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.info-item {
		display: flex;
		gap: 0.75rem;
		color: var(--admin-text-muted);
	}

	.info-item div {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.info-label {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	.info-value {
		font-size: 0.875rem;
		color: var(--admin-text-primary);
	}

	/* Analytics */
	.analytics-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.analytics-card {
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		padding: 1rem;
	}

	.analytics-card h4 {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--admin-text-primary);
		margin: 0 0 0.75rem;
	}

	.analytics-metrics {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.metric-row {
		display: flex;
		justify-content: space-between;
		font-size: 0.8125rem;
	}

	.metric-label {
		color: var(--admin-text-muted);
	}

	.metric-value {
		color: var(--admin-text-primary);
		font-weight: 500;
	}

	/* Instructor */
	.instructor-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
	}

	.instructor-avatar {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		object-fit: cover;
	}

	.instructor-avatar-placeholder {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		background: linear-gradient(
			135deg,
			var(--admin-accent-primary),
			var(--admin-widget-purple-icon)
		);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
	}

	.instructor-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.instructor-name {
		font-weight: 500;
		color: var(--admin-text-primary);
	}

	.instructor-title {
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 MOBILE-FIRST RESPONSIVE DESIGN
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* DYNAMIC VIEWPORT HEIGHT */
	@supports (height: 100dvh) {
		.drawer {
			height: 100dvh;
		}
	}

	/* SAFE AREA INSETS */
	.drawer {
		padding-top: env(safe-area-inset-top);
		padding-bottom: env(safe-area-inset-bottom);
	}

	.drawer-content {
		padding-bottom: calc(1.5rem + env(safe-area-inset-bottom));
	}

	/* TOUCH TARGETS */
	@media (hover: none) and (pointer: coarse) {
		.btn-action,
		.btn-close,
		.btn-retry,
		.btn-add-module,
		.btn-module-action,
		.tab,
		.module-header {
			min-height: 44px;
			min-width: 44px;
		}

		.btn-module-action {
			padding: 0.5rem;
		}
	}

	/* EXTRA SMALL DEVICES (< 360px) */
	@media (max-width: 359px) {
		.drawer {
			width: 100%;
		}

		.drawer-header {
			padding: 1rem;
			flex-wrap: wrap;
			gap: 0.75rem;
		}

		.course-thumbnail {
			width: 56px;
			height: 38px;
		}

		.course-title {
			font-size: 1rem;
		}

		.quick-stats {
			grid-template-columns: repeat(2, 1fr);
			padding: 0.75rem 1rem;
		}

		.action-buttons {
			flex-wrap: wrap;
			padding: 0.75rem 1rem;
		}

		.btn-action {
			flex: 1 1 calc(50% - 0.25rem);
			font-size: 0.75rem;
			padding: 0.5rem;
		}

		.drawer-tabs {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
		}

		.tab {
			padding: 0.75rem;
			font-size: 0.75rem;
			white-space: nowrap;
		}

		.drawer-content {
			padding: 1rem;
		}

		.module-header {
			padding: 0.75rem;
			flex-wrap: wrap;
		}

		.module-info {
			width: 100%;
			order: -1;
			margin-bottom: 0.5rem;
		}

		.module-actions {
			margin-left: auto;
		}

		.enrollment-stats {
			grid-template-columns: 1fr;
		}

		.analytics-grid {
			grid-template-columns: 1fr;
		}

		.info-grid {
			grid-template-columns: 1fr;
		}
	}

	/* SMALL MOBILE (360px - 639px) */
	@media (min-width: 360px) and (max-width: 639px) {
		.drawer {
			width: 100%;
		}

		.drawer-header {
			padding: 1.25rem;
		}

		.quick-stats {
			grid-template-columns: repeat(2, 1fr);
		}

		.action-buttons {
			flex-wrap: wrap;
		}

		.btn-action {
			flex: 1 1 calc(50% - 0.25rem);
		}

		.drawer-tabs {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
		}

		.tab {
			white-space: nowrap;
		}

		.enrollment-stats {
			grid-template-columns: 1fr;
			gap: 0.5rem;
		}

		.analytics-grid {
			grid-template-columns: 1fr;
		}
	}

	/* TABLET (640px - 767px) */
	@media (min-width: 640px) and (max-width: 767px) {
		.drawer {
			width: 480px;
		}

		.quick-stats {
			grid-template-columns: repeat(4, 1fr);
		}

		.enrollment-stats {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	/* MEDIUM DEVICES (768px+) */
	@media (min-width: 768px) {
		.drawer {
			width: 560px;
		}
	}

	/* LANDSCAPE MOBILE */
	@media (max-height: 500px) and (orientation: landscape) {
		.drawer-header {
			padding: 0.75rem 1.5rem;
		}

		.quick-stats {
			padding: 0.5rem 1rem;
		}

		.action-buttons {
			padding: 0.5rem 1rem;
		}

		.drawer-content {
			padding: 1rem;
		}
	}

	/* REDUCED MOTION */
	@media (prefers-reduced-motion: reduce) {
		.drawer {
			transition: none;
		}

		.drawer-backdrop {
			animation: none;
		}

		.btn-action,
		.btn-close,
		.btn-module-action,
		.tab,
		.module-header,
		.lesson-item {
			transition: none;
		}
	}
</style>
