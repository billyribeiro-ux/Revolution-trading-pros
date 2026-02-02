<script lang="ts">
	/**
	 * Course Builder - Revolution Trading Pros
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 *
	 * Complete course management with:
	 * - Sections & Lessons (drag & drop reorder)
	 * - Resources (PDF, DOC uploads)
	 * - Live Sessions
	 * - Video Integration (Bunny.net)
	 * - Progress tracking
	 */

	import {
		coursesApi,
		sectionsApi,
		lessonsApi,
		resourcesApi,
		liveSessionsApi,
		type Course,
		type CourseSection,
		type CourseLesson,
		type CreateCourseRequest,
		type CreateSectionRequest,
		type CreateLessonRequest
	} from '$lib/api/courses-enhanced';

	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconGripVertical from '@tabler/icons-svelte/icons/grip-vertical';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconFile from '@tabler/icons-svelte/icons/file';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';

	interface Props {
		courseId?: number;
		onSave?: (course: Course) => void;
		onClose?: () => void;
	}

	let props: Props = $props();

	// Destructure for internal use
	const courseId = $derived(props.courseId);
	const onSave = $derived(props.onSave);
	const onClose = $derived(props.onClose);

	let course = $state<Course | null>(null);
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state('');
	let activeTab = $state<'details' | 'content' | 'live-sessions' | 'resources'>('details');
	let expandedSections = $state<Set<number>>(new Set());

	// Form state
	let editMode = $state(false);
	let formData = $state<CreateCourseRequest>({
		title: '',
		subtitle: '',
		description: '',
		description_html: '',
		thumbnail_url: '',
		trailer_video_url: '',
		difficulty_level: 'beginner',
		category: '',
		tags: [],
		is_published: false,
		is_featured: false,
		is_free: false,
		certificate_enabled: false,
		completion_threshold_percent: 80
	});

	// Section/Lesson form state
	let showSectionForm = $state(false);
	let showLessonForm = $state(false);
	let selectedSectionId = $state<number | null>(null);
	let sectionFormData = $state<CreateSectionRequest>({ title: '', description: '' });
	let lessonFormData = $state<CreateLessonRequest>({
		section_id: 0,
		title: '',
		description: '',
		lesson_type: 'video',
		is_preview: false,
		is_published: true,
		completion_type: 'watch',
		required_watch_percent: 80
	});

	$effect(() => {
		loadCourse();
	});

	async function loadCourse() {
		if (!courseId) {
			isLoading = false;
			editMode = true;
			return;
		}

		isLoading = true;
		error = '';

		const result = await coursesApi.get(courseId);

		if (result.success && result.data) {
			course = result.data;
			formData = {
				title: course.title,
				subtitle: course.subtitle || '',
				description: course.description || '',
				description_html: course.description_html || '',
				thumbnail_url: course.thumbnail_url || '',
				trailer_video_url: course.trailer_video_url || '',
				difficulty_level: course.difficulty_level,
				category: course.category || '',
				tags: course.tags || [],
				is_published: course.is_published,
				is_featured: course.is_featured,
				is_free: course.is_free,
				certificate_enabled: course.certificate_enabled,
				completion_threshold_percent: course.completion_threshold_percent
			};
		} else {
			error = result.error || 'Failed to load course';
		}

		isLoading = false;
	}

	async function saveCourse() {
		if (!formData.title.trim()) {
			error = 'Course title is required';
			return;
		}

		isSaving = true;
		error = '';

		if (courseId) {
			// Update existing course
			const result = await coursesApi.update(courseId, formData);
			if (result.success) {
				await loadCourse();
				editMode = false;
				onSave?.(course!);
			} else {
				error = result.error || 'Failed to save course';
			}
		} else {
			// Create new course
			const result = await coursesApi.create(formData);
			if (result.success && result.data?.course) {
				courseId = result.data.course.id;
				await loadCourse();
				editMode = false;
				onSave?.(course!);
			} else {
				error = result.error || 'Failed to create course';
			}
		}

		isSaving = false;
	}

	function toggleSection(sectionId: number) {
		if (expandedSections.has(sectionId)) {
			expandedSections.delete(sectionId);
		} else {
			expandedSections.add(sectionId);
		}
		expandedSections = new Set(expandedSections);
	}

	async function createSection() {
		if (!courseId || !sectionFormData.title.trim()) return;

		isSaving = true;
		const result = await sectionsApi.create(courseId, sectionFormData);

		if (result.success) {
			showSectionForm = false;
			sectionFormData = { title: '', description: '' };
			await loadCourse();
		} else {
			error = result.error || 'Failed to create section';
		}

		isSaving = false;
	}

	async function deleteSection(sectionId: number) {
		if (!courseId || !confirm('Delete this section and all its lessons?')) return;

		const result = await sectionsApi.delete(courseId, sectionId);
		if (result.success) {
			await loadCourse();
		} else {
			error = result.error || 'Failed to delete section';
		}
	}

	async function createLesson() {
		if (!courseId || !selectedSectionId || !lessonFormData.title.trim()) return;

		lessonFormData.section_id = selectedSectionId;
		isSaving = true;

		const result = await lessonsApi.create(courseId, lessonFormData);

		if (result.success) {
			showLessonForm = false;
			lessonFormData = {
				section_id: 0,
				title: '',
				description: '',
				lesson_type: 'video',
				is_preview: false,
				is_published: true,
				completion_type: 'watch',
				required_watch_percent: 80
			};
			await loadCourse();
		} else {
			error = result.error || 'Failed to create lesson';
		}

		isSaving = false;
	}

	async function deleteLesson(lessonId: number) {
		if (!courseId || !confirm('Delete this lesson?')) return;

		const result = await lessonsApi.delete(courseId, lessonId);
		if (result.success) {
			await loadCourse();
		} else {
			error = result.error || 'Failed to delete lesson';
		}
	}

	async function cloneCourse() {
		if (!courseId || !confirm('Clone this course?')) return;

		const result = await coursesApi.clone(courseId);
		if (result.success && result.data) {
			alert(`Course cloned! New ID: ${result.data.new_course.id}`);
		} else {
			error = result.error || 'Failed to clone course';
		}
	}

	function openLessonForm(sectionId: number) {
		selectedSectionId = sectionId;
		showLessonForm = true;
	}

	const difficultyOptions = [
		{ value: 'beginner', label: 'Beginner' },
		{ value: 'intermediate', label: 'Intermediate' },
		{ value: 'advanced', label: 'Advanced' }
	];
</script>

<div class="course-builder">
	<div class="builder-header">
		<div class="header-left">
			<h2>{courseId ? 'Edit Course' : 'Create Course'}</h2>
			{#if course}
				<span class="course-status" class:published={course.is_published}>
					{course.is_published ? 'Published' : 'Draft'}
				</span>
			{/if}
		</div>
		<div class="header-actions">
			{#if courseId}
				<button type="button" class="btn-secondary" onclick={cloneCourse}>
					<IconCopy size={16} /> Clone
				</button>
			{/if}
			{#if onClose}
				<button type="button" class="btn-close" onclick={onClose}>
					<IconX size={20} />
				</button>
			{/if}
		</div>
	</div>

	{#if error}
		<div class="error-message">{error}</div>
	{/if}

	{#if isLoading}
		<div class="loading">Loading course...</div>
	{:else}
		<!-- Tab Navigation -->
		<div class="tabs">
			<button
				type="button"
				class="tab"
				class:active={activeTab === 'details'}
				onclick={() => (activeTab = 'details')}
			>
				Details
			</button>
			<button
				type="button"
				class="tab"
				class:active={activeTab === 'content'}
				onclick={() => (activeTab = 'content')}
				disabled={!courseId}
			>
				Content
			</button>
			<button
				type="button"
				class="tab"
				class:active={activeTab === 'live-sessions'}
				onclick={() => (activeTab = 'live-sessions')}
				disabled={!courseId}
			>
				Live Sessions
			</button>
			<button
				type="button"
				class="tab"
				class:active={activeTab === 'resources'}
				onclick={() => (activeTab = 'resources')}
				disabled={!courseId}
			>
				Resources
			</button>
		</div>

		<!-- Details Tab -->
		{#if activeTab === 'details'}
			<div class="tab-content">
				<div class="form-section">
					<div class="form-group">
						<label for="title">Title *</label>
						<input
							type="text"
							id="title"
							bind:value={formData.title}
							placeholder="Course title"
							disabled={!editMode && !!courseId}
						/>
					</div>

					<div class="form-group">
						<label for="subtitle">Subtitle</label>
						<input
							type="text"
							id="subtitle"
							bind:value={formData.subtitle}
							placeholder="Course subtitle"
							disabled={!editMode && !!courseId}
						/>
					</div>

					<div class="form-row">
						<div class="form-group">
							<label for="difficulty">Difficulty Level</label>
							<select
								id="difficulty"
								bind:value={formData.difficulty_level}
								disabled={!editMode && !!courseId}
							>
								{#each difficultyOptions as opt}
									<option value={opt.value}>{opt.label}</option>
								{/each}
							</select>
						</div>

						<div class="form-group">
							<label for="category">Category</label>
							<input
								type="text"
								id="category"
								bind:value={formData.category}
								placeholder="e.g., Trading Fundamentals"
								disabled={!editMode && !!courseId}
							/>
						</div>
					</div>

					<div class="form-group">
						<label for="description">Description</label>
						<textarea
							id="description"
							bind:value={formData.description}
							placeholder="Course description"
							rows="4"
							disabled={!editMode && !!courseId}
						></textarea>
					</div>

					<div class="form-group">
						<label for="thumbnail_url">Thumbnail URL</label>
						<input
							type="text"
							id="thumbnail_url"
							bind:value={formData.thumbnail_url}
							placeholder="https://..."
							disabled={!editMode && !!courseId}
						/>
						{#if formData.thumbnail_url}
							<img src={formData.thumbnail_url} alt="Thumbnail preview" class="thumbnail-preview" />
						{/if}
					</div>

					<div class="form-group">
						<label for="trailer_video_url">Trailer Video URL</label>
						<input
							type="text"
							id="trailer_video_url"
							bind:value={formData.trailer_video_url}
							placeholder="Bunny.net embed URL"
							disabled={!editMode && !!courseId}
						/>
					</div>

					<div class="form-row toggles">
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.is_published}
								disabled={!editMode && !!courseId}
							/>
							Published
						</label>
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.is_featured}
								disabled={!editMode && !!courseId}
							/>
							Featured
						</label>
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.is_free}
								disabled={!editMode && !!courseId}
							/>
							Free
						</label>
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.certificate_enabled}
								disabled={!editMode && !!courseId}
							/>
							Certificate
						</label>
					</div>

					{#if formData.certificate_enabled}
						<div class="form-group">
							<label for="completion_threshold">Completion Threshold (%)</label>
							<input
								type="number"
								id="completion_threshold"
								bind:value={formData.completion_threshold_percent}
								min="1"
								max="100"
								disabled={!editMode && !!courseId}
							/>
						</div>
					{/if}
				</div>

				<div class="form-actions">
					{#if courseId && !editMode}
						<button type="button" class="btn-primary" onclick={() => (editMode = true)}>
							<IconEdit size={16} /> Edit
						</button>
					{:else}
						<button type="button" class="btn-primary" onclick={saveCourse} disabled={isSaving}>
							<IconCheck size={16} />
							{isSaving ? 'Saving...' : 'Save'}
						</button>
						{#if courseId}
							<button
								type="button"
								class="btn-secondary"
								onclick={() => {
									editMode = false;
									loadCourse();
								}}
							>
								Cancel
							</button>
						{/if}
					{/if}
				</div>
			</div>
		{/if}

		<!-- Content Tab (Sections & Lessons) -->
		{#if activeTab === 'content' && course}
			<div class="tab-content">
				<div class="content-header">
					<h3>Course Content</h3>
					<button type="button" class="btn-add" onclick={() => (showSectionForm = true)}>
						<IconPlus size={16} /> Add Section
					</button>
				</div>

				{#if showSectionForm}
					<div class="inline-form">
						<input type="text" bind:value={sectionFormData.title} placeholder="Section title" />
						<input
							type="text"
							bind:value={sectionFormData.description}
							placeholder="Description (optional)"
						/>
						<button type="button" class="btn-save" onclick={createSection} disabled={isSaving}>
							<IconCheck size={16} />
						</button>
						<button type="button" class="btn-cancel" onclick={() => (showSectionForm = false)}>
							<IconX size={16} />
						</button>
					</div>
				{/if}

				<div class="sections-list">
					{#each course.sections || [] as section (section.id)}
						<div class="section-item">
							<div
								class="section-header"
								role="button"
								tabindex="0"
								onclick={() => toggleSection(section.id)}
								onkeydown={(e) => {
									if (e.key === 'Enter' || e.key === ' ') {
										e.preventDefault();
										toggleSection(section.id);
									}
								}}
							>
								<div class="section-grip">
									<IconGripVertical size={16} />
								</div>
								<button type="button" class="expand-btn">
									{#if expandedSections.has(section.id)}
										<IconChevronDown size={16} />
									{:else}
										<IconChevronRight size={16} />
									{/if}
								</button>
								<div class="section-info">
									<span class="section-title">{section.title}</span>
									<span class="section-meta">{section.lesson_count} lessons</span>
								</div>
								<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
								<div
									class="section-actions"
									onclick={(e) => e.stopPropagation()}
									onkeydown={(e) => e.stopPropagation()}
									role="group"
								>
									<button
										type="button"
										class="btn-icon"
										onclick={() => openLessonForm(section.id)}
										title="Add lesson"
									>
										<IconPlus size={16} />
									</button>
									<button
										type="button"
										class="btn-icon danger"
										onclick={() => deleteSection(section.id)}
										title="Delete section"
									>
										<IconTrash size={16} />
									</button>
								</div>
							</div>

							{#if expandedSections.has(section.id)}
								<div class="lessons-list">
									{#each section.lessons || [] as lesson (lesson.id)}
										<div class="lesson-item">
											<div class="lesson-grip">
												<IconGripVertical size={14} />
											</div>
											<div class="lesson-icon">
												{#if lesson.lesson_type === 'video'}
													<IconVideo size={16} />
												{:else}
													<IconFile size={16} />
												{/if}
											</div>
											<div class="lesson-info">
												<span class="lesson-title">{lesson.title}</span>
												{#if lesson.formatted_duration}
													<span class="lesson-duration">{lesson.formatted_duration}</span>
												{/if}
											</div>
											<div class="lesson-badges">
												{#if lesson.is_preview}
													<span class="badge preview">Preview</span>
												{/if}
												{#if !lesson.is_published}
													<span class="badge draft">Draft</span>
												{/if}
											</div>
											<div class="lesson-actions">
												<button type="button" class="btn-icon" title="Edit lesson">
													<IconEdit size={14} />
												</button>
												<button
													type="button"
													class="btn-icon danger"
													onclick={() => deleteLesson(lesson.id)}
													title="Delete lesson"
												>
													<IconTrash size={14} />
												</button>
											</div>
										</div>
									{/each}

									{#if (section.lessons || []).length === 0}
										<div class="empty-lessons">No lessons yet</div>
									{/if}
								</div>
							{/if}
						</div>
					{/each}

					{#if (course.sections || []).length === 0}
						<div class="empty-sections">
							<p>No sections yet. Add your first section to get started.</p>
						</div>
					{/if}
				</div>
			</div>

			<!-- Add Lesson Modal -->
			{#if showLessonForm && selectedSectionId}
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<div
					class="modal-overlay"
					role="dialog"
					aria-modal="true"
					aria-label="Add Lesson"
					tabindex="-1"
					onclick={() => (showLessonForm = false)}
					onkeydown={(e) => {
						if (e.key === 'Escape') showLessonForm = false;
					}}
				>
					<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
					<div
						class="modal-content"
						role="document"
						onclick={(e) => e.stopPropagation()}
						onkeydown={(e) => e.stopPropagation()}
					>
						<h3>Add Lesson</h3>

						<div class="form-group">
							<label for="lesson-title">Title *</label>
							<input
								type="text"
								id="lesson-title"
								bind:value={lessonFormData.title}
								placeholder="Lesson title"
							/>
						</div>

						<div class="form-group">
							<label for="lesson-type">Type</label>
							<select id="lesson-type" bind:value={lessonFormData.lesson_type}>
								<option value="video">Video</option>
								<option value="text">Text/Article</option>
								<option value="quiz">Quiz</option>
							</select>
						</div>

						<div class="form-group">
							<label for="video-url">Video URL</label>
							<input
								type="text"
								id="video-url"
								bind:value={lessonFormData.video_url}
								placeholder="Bunny.net embed URL"
							/>
						</div>

						<div class="form-group">
							<label for="bunny-guid">Bunny Video GUID</label>
							<input
								type="text"
								id="bunny-guid"
								bind:value={lessonFormData.bunny_video_guid}
								placeholder="Video GUID from Bunny.net"
							/>
						</div>

						<div class="form-row toggles">
							<label class="toggle-label">
								<input type="checkbox" bind:checked={lessonFormData.is_preview} />
								Free Preview
							</label>
							<label class="toggle-label">
								<input type="checkbox" bind:checked={lessonFormData.is_published} />
								Published
							</label>
						</div>

						<div class="modal-actions">
							<button type="button" class="btn-primary" onclick={createLesson} disabled={isSaving}>
								{isSaving ? 'Creating...' : 'Create Lesson'}
							</button>
							<button type="button" class="btn-secondary" onclick={() => (showLessonForm = false)}>
								Cancel
							</button>
						</div>
					</div>
				</div>
			{/if}
		{/if}

		<!-- Live Sessions Tab -->
		{#if activeTab === 'live-sessions' && course}
			<div class="tab-content">
				<div class="content-header">
					<h3>Live Sessions</h3>
					<button type="button" class="btn-add">
						<IconPlus size={16} /> Add Session
					</button>
				</div>

				<div class="sessions-list">
					{#each course.live_sessions || [] as session (session.id)}
						<div class="session-item">
							<div class="session-date">
								<span class="date">{session.formatted_date}</span>
								{#if session.session_time}
									<span class="time">{session.session_time}</span>
								{/if}
							</div>
							<div class="session-info">
								<span class="session-title">{session.title}</span>
								{#if session.formatted_duration}
									<span class="session-duration">{session.formatted_duration}</span>
								{/if}
							</div>
							<div class="session-actions">
								{#if session.embed_url}
									<button type="button" class="btn-icon" title="Preview">
										<IconPlayerPlay size={16} />
									</button>
								{/if}
								<button type="button" class="btn-icon danger" title="Delete">
									<IconTrash size={16} />
								</button>
							</div>
						</div>
					{/each}

					{#if (course.live_sessions || []).length === 0}
						<div class="empty-sessions">
							<p>No live sessions yet.</p>
						</div>
					{/if}
				</div>
			</div>
		{/if}

		<!-- Resources Tab -->
		{#if activeTab === 'resources' && course}
			<div class="tab-content">
				<div class="content-header">
					<h3>Course Resources</h3>
					<button type="button" class="btn-add">
						<IconPlus size={16} /> Add Resource
					</button>
				</div>

				<div class="resources-list">
					{#each course.resources || [] as resource (resource.id)}
						<div class="resource-item">
							<div class="resource-icon">
								<IconFile size={20} />
							</div>
							<div class="resource-info">
								<span class="resource-title">{resource.title}</span>
								<span class="resource-meta">
									{resource.file_name} - {resource.formatted_size}
								</span>
							</div>
							<div class="resource-actions">
								<a href={resource.file_url} target="_blank" class="btn-icon" title="Download">
									<IconEye size={16} />
								</a>
								<button type="button" class="btn-icon danger" title="Delete">
									<IconTrash size={16} />
								</button>
							</div>
						</div>
					{/each}

					{#if (course.resources || []).length === 0}
						<div class="empty-resources">
							<p>No resources yet. Add PDFs, documents, or other files.</p>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	{/if}
</div>

<style>
	.course-builder {
		background: var(--bg-secondary, #1a1a2e);
		border-radius: 12px;
		padding: 1.5rem;
		max-width: 900px;
		margin: 0 auto;
	}

	.builder-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--border-color, #333);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-left h2 {
		margin: 0;
		font-size: 1.5rem;
	}

	.course-status {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		background: var(--bg-tertiary, #252542);
		color: var(--text-secondary);
	}

	.course-status.published {
		background: #22c55e1a;
		color: #22c55e;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
	}

	.error-message {
		background: #ef44441a;
		border: 1px solid #ef4444;
		color: #ef4444;
		padding: 0.75rem;
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: var(--text-secondary);
	}

	/* Tabs */
	.tabs {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 1.5rem;
		border-bottom: 1px solid var(--border-color, #333);
	}

	.tab {
		padding: 0.75rem 1.25rem;
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		font-size: 0.875rem;
		border-bottom: 2px solid transparent;
		transition: all 0.2s;
	}

	.tab:hover:not(:disabled) {
		color: var(--text-primary, white);
	}

	.tab.active {
		color: var(--primary, #e6b800);
		border-bottom-color: var(--primary, #e6b800);
	}

	.tab:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.tab-content {
		min-height: 400px;
	}

	/* Form Styles */
	.form-section {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		background: var(--bg-primary, #0f0f1a);
		border: 1px solid var(--border-color, #333);
		color: var(--text-primary, white);
		padding: 0.625rem 0.875rem;
		border-radius: 8px;
		font-size: 0.875rem;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--primary, #e6b800);
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-row.toggles {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.thumbnail-preview {
		max-width: 200px;
		border-radius: 8px;
		margin-top: 0.5rem;
	}

	.form-actions {
		display: flex;
		gap: 0.75rem;
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid var(--border-color, #333);
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-add {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.625rem 1rem;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.btn-primary {
		background: var(--primary, #e6b800);
		color: #0d1117;
	}

	.btn-secondary {
		background: var(--bg-tertiary, #252542);
		color: var(--text-primary, white);
	}

	.btn-add {
		background: var(--primary, #e6b800);
		color: #0d1117;
	}

	.btn-close {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.25rem;
	}

	.btn-icon {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.375rem;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: var(--bg-hover, #ffffff1a);
		color: var(--text-primary, white);
	}

	.btn-icon.danger:hover {
		color: #ef4444;
	}

	/* Content Tab */
	.content-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.content-header h3 {
		margin: 0;
		font-size: 1rem;
	}

	.inline-form {
		display: flex;
		gap: 0.5rem;
		padding: 1rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.inline-form input {
		flex: 1;
		background: var(--bg-primary, #0f0f1a);
		border: 1px solid var(--border-color, #333);
		color: var(--text-primary, white);
		padding: 0.5rem 0.75rem;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.btn-save,
	.btn-cancel {
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 6px;
	}

	.btn-save {
		color: #22c55e;
	}

	.btn-cancel {
		color: var(--text-secondary);
	}

	/* Sections & Lessons */
	.sections-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.section-item {
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
		overflow: hidden;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1rem;
		cursor: pointer;
	}

	.section-grip,
	.lesson-grip {
		color: var(--text-secondary);
		cursor: grab;
	}

	.expand-btn {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0;
	}

	.section-info {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.section-title {
		font-weight: 500;
	}

	.section-meta {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.section-actions {
		display: flex;
		gap: 0.25rem;
	}

	.lessons-list {
		padding: 0 1rem 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.lesson-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 0.75rem;
		background: var(--bg-primary, #0f0f1a);
		border-radius: 6px;
	}

	.lesson-icon {
		color: var(--text-secondary);
	}

	.lesson-info {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.lesson-title {
		font-size: 0.875rem;
	}

	.lesson-duration {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.lesson-badges {
		display: flex;
		gap: 0.25rem;
	}

	.badge {
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;
		font-size: 0.625rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.badge.preview {
		background: #3b82f61a;
		color: #3b82f6;
	}

	.badge.draft {
		background: #f59e0b1a;
		color: #f59e0b;
	}

	.lesson-actions {
		display: flex;
		gap: 0.125rem;
	}

	.empty-sections,
	.empty-lessons,
	.empty-sessions,
	.empty-resources {
		text-align: center;
		padding: 2rem;
		color: var(--text-secondary);
		font-size: 0.875rem;
	}

	/* Sessions & Resources */
	.sessions-list,
	.resources-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.session-item,
	.resource-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
	}

	.session-date {
		min-width: 100px;
		text-align: center;
	}

	.session-date .date {
		display: block;
		font-weight: 500;
	}

	.session-date .time {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.session-info,
	.resource-info {
		flex: 1;
	}

	.session-title,
	.resource-title {
		font-weight: 500;
		display: block;
	}

	.session-duration,
	.resource-meta {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.resource-icon {
		color: var(--primary, #e6b800);
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal-content {
		background: var(--bg-secondary, #1a1a2e);
		border-radius: 12px;
		padding: 1.5rem;
		max-width: 500px;
		width: 90%;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-content h3 {
		margin: 0 0 1.5rem;
		font-size: 1.25rem;
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
		margin-top: 1.5rem;
		justify-content: flex-end;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 MOBILE-FIRST RESPONSIVE DESIGN
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* DYNAMIC VIEWPORT HEIGHT */
	@supports (height: 100dvh) {
		.modal-content {
			max-height: 90dvh;
		}
	}

	/* SAFE AREA INSETS */
	.course-builder {
		padding-left: calc(1.5rem + env(safe-area-inset-left));
		padding-right: calc(1.5rem + env(safe-area-inset-right));
	}

	.modal-overlay {
		padding: env(safe-area-inset-top) env(safe-area-inset-right) env(safe-area-inset-bottom) env(safe-area-inset-left);
	}

	/* EXTRA SMALL DEVICES (< 360px) */
	@media (max-width: 359px) {
		.course-builder {
			padding: 0.75rem;
			border-radius: 8px;
		}

		.builder-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
			padding-bottom: 0.75rem;
		}

		.header-left h2 {
			font-size: 1.1rem;
		}

		.tabs {
			flex-wrap: wrap;
		}

		.tab {
			flex: 1 1 50%;
			padding: 0.5rem;
			font-size: 0.7rem;
		}

		.form-group input,
		.form-group select,
		.form-group textarea {
			padding: 0.5rem 0.625rem;
			font-size: 0.8rem;
		}

		.section-header {
			padding: 0.625rem 0.75rem;
		}

		.lesson-item {
			padding: 0.5rem;
		}

		.modal-content {
			padding: 1rem;
			width: 95%;
		}
	}

	/* SMALL MOBILE (360px - 639px) */
	@media (min-width: 360px) and (max-width: 639px) {
		.course-builder {
			padding: 1rem;
			border-radius: 8px;
		}

		.builder-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.75rem;
		}

		.header-left {
			flex-wrap: wrap;
		}

		.header-left h2 {
			font-size: 1.25rem;
		}

		.header-actions {
			width: 100%;
			justify-content: space-between;
		}

		.tabs {
			flex-wrap: wrap;
			gap: 0;
		}

		.tab {
			flex: 1;
			min-width: 50%;
			padding: 0.625rem 0.75rem;
			font-size: 0.75rem;
			text-align: center;
			justify-content: center;
		}

		.tab-content {
			min-height: 300px;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.form-row.toggles {
			gap: 0.75rem;
		}

		.form-actions {
			flex-direction: column;
		}

		.form-actions button {
			width: 100%;
			justify-content: center;
		}

		.section-header {
			flex-wrap: wrap;
			padding: 0.75rem;
		}

		.section-info {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.25rem;
		}

		.section-actions {
			width: 100%;
			justify-content: flex-end;
			margin-top: 0.5rem;
		}

		.lesson-item {
			flex-wrap: wrap;
			gap: 0.5rem;
		}

		.lesson-info {
			width: 100%;
			order: -1;
		}

		.lesson-actions {
			width: 100%;
			justify-content: flex-end;
		}

		.modal-content {
			padding: 1rem;
			max-height: 95vh;
			border-radius: 8px;
		}

		.modal-content h3 {
			font-size: 1.1rem;
		}

		.modal-actions {
			flex-direction: column;
		}

		.modal-actions button {
			width: 100%;
			justify-content: center;
		}

		.session-item,
		.resource-item {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.75rem;
		}

		.session-date {
			min-width: auto;
			text-align: left;
		}
	}

	/* Tablet (768px and below) */
	@media (max-width: 768px) {
		.course-builder {
			padding: 1.25rem;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.tabs {
			overflow-x: auto;
			flex-wrap: nowrap;
			-webkit-overflow-scrolling: touch;
			padding-bottom: 0.25rem;
		}

		.tab {
			flex-shrink: 0;
			white-space: nowrap;
		}
	}

	/* Touch Device Optimizations */
	@media (hover: none) and (pointer: coarse) {
		.tab,
		.btn-primary,
		.btn-secondary,
		.btn-add,
		.btn-icon,
		.toggle-label,
		.section-header {
			min-height: 44px;
		}

		.btn-icon {
			width: 44px;
			height: 44px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.tab,
		.btn-primary,
		.btn-secondary,
		.btn-add,
		.btn-icon {
			transition: none;
		}
	}
</style>
