<script lang="ts">
	/**
	 * Admin Create Lesson - Add New Learning Content
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Form for creating new lessons and assigning them to trading rooms.
	 * IMPORTANT: Always asks which trading room(s) the content belongs to.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { learningCenterStore } from '$lib/stores/learningCenter';
	import type { CreateLessonInput, LessonType, AccessLevel, LessonStatus } from '$lib/types/learning-center';
	import { get } from 'svelte/store';
	import { RoomSelector } from '$lib/components/learning-center';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconUpload from '@tabler/icons-svelte/icons/upload';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconFileText from '@tabler/icons-svelte/icons/file-text';
	import IconFile from '@tabler/icons-svelte/icons/file';
	import IconHelp from '@tabler/icons-svelte/icons/help';
	import IconCast from '@tabler/icons-svelte/icons/cast';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconPlus from '@tabler/icons-svelte/icons/plus';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isSubmitting = $state(false);
	let currentStep = $state(1);
	let errors = $state<Record<string, string>>({});

	// Form data
	let formData = $state<CreateLessonInput>({
		title: '',
		description: '',
		fullDescription: '',
		type: 'video',
		videoUrl: '',
		posterUrl: '',
		thumbnailUrl: '',
		duration: '',
		tradingRoomIds: [],
		trainerId: '',
		categoryId: '',
		tags: [],
		accessLevel: 'member',
		moduleId: '',
		status: 'draft'
	});

	let newTag = $state('');

	// ═══════════════════════════════════════════════════════════════════════════
	// STORE DATA
	// ═══════════════════════════════════════════════════════════════════════════

	let storeData = $derived(get(learningCenterStore));

	// Trading rooms with learning centers
	let tradingRooms = $derived(
		storeData.tradingRooms.filter(r => r.hasLearningCenter && r.isActive)
	);

	// Trainers
	let trainers = $derived(storeData.trainers.filter(t => t.isActive));

	// Categories
	let categories = $derived(storeData.categories.filter(c => c.isActive));

	// Modules for selected rooms
	let availableModules = $derived.by(() => {
		if (formData.tradingRoomIds.length === 0) return [];
		// Get modules for the first selected room
		const store = get(learningCenterStore);
		return store.modules.filter(m => m.tradingRoomId === formData.tradingRoomIds[0]);
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FORM DATA FROM URL (for duplication)
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser) {
			const duplicateData = $page.url.searchParams.get('duplicate');
			if (duplicateData) {
				try {
					const data = JSON.parse(decodeURIComponent(duplicateData));
					formData = { ...formData, ...data };
				} catch (e) {
					console.error('Failed to parse duplicate data:', e);
				}
			}
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LESSON TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	const lessonTypes: { value: LessonType; label: string; icon: any; description: string }[] = [
		{
			value: 'video',
			label: 'Video',
			icon: IconVideo,
			description: 'Upload or embed a video lesson'
		},
		{
			value: 'article',
			label: 'Article',
			icon: IconFileText,
			description: 'Written content with text and images'
		},
		{
			value: 'pdf',
			label: 'PDF',
			icon: IconFile,
			description: 'Downloadable PDF document'
		},
		{
			value: 'quiz',
			label: 'Quiz',
			icon: IconHelp,
			description: 'Interactive quiz or assessment'
		},
		{
			value: 'webinar-replay',
			label: 'Webinar Replay',
			icon: IconCast,
			description: 'Recorded webinar or live session'
		}
	];

	const accessLevels: { value: AccessLevel; label: string; description: string }[] = [
		{ value: 'free', label: 'Free', description: 'Available to everyone' },
		{ value: 'member', label: 'Members Only', description: 'Requires an active subscription' },
		{ value: 'premium', label: 'Premium', description: 'Requires premium membership' }
	];

	const statusOptions: { value: LessonStatus; label: string }[] = [
		{ value: 'draft', label: 'Draft' },
		{ value: 'published', label: 'Published' },
		{ value: 'archived', label: 'Archived' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// VALIDATION
	// ═══════════════════════════════════════════════════════════════════════════

	function validateStep(step: number): boolean {
		const newErrors: Record<string, string> = {};

		if (step === 1) {
			// Step 1: Trading Rooms selection
			if (formData.tradingRoomIds.length === 0) {
				newErrors.tradingRoomIds = 'Please select at least one trading room';
			}
		}

		if (step === 2) {
			// Step 2: Basic info
			if (!formData.title.trim()) {
				newErrors.title = 'Title is required';
			}
			if (!formData.description.trim()) {
				newErrors.description = 'Description is required';
			}
			if (!formData.trainerId) {
				newErrors.trainerId = 'Please select a trainer';
			}
			if (!formData.categoryId) {
				newErrors.categoryId = 'Please select a category';
			}
		}

		if (step === 3) {
			// Step 3: Content
			if (formData.type === 'video' && !formData.videoUrl) {
				newErrors.videoUrl = 'Video URL is required';
			}
		}

		errors = newErrors;
		return Object.keys(newErrors).length === 0;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleRoomSelect(roomIds: string[]) {
		formData.tradingRoomIds = roomIds;
		if (errors.tradingRoomIds) {
			errors = { ...errors, tradingRoomIds: '' };
		}
	}

	function handleNextStep() {
		if (validateStep(currentStep)) {
			currentStep++;
		}
	}

	function handlePrevStep() {
		currentStep--;
	}

	function addTag() {
		const tag = newTag.trim().toLowerCase();
		if (tag && !formData.tags?.includes(tag)) {
			formData.tags = [...(formData.tags || []), tag];
			newTag = '';
		}
	}

	function removeTag(tag: string) {
		formData.tags = formData.tags?.filter(t => t !== tag) || [];
	}

	function handleTagKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			addTag();
		}
	}

	async function handleSubmit() {
		if (!validateStep(currentStep)) return;

		isSubmitting = true;

		try {
			// TODO: Implement create functionality via API
			console.log('Create lesson:', formData);
			// Redirect to the lesson edit page or listing
			await goto('/admin/learning-center');
		} catch (err) {
			console.error('Failed to create lesson:', err);
			errors = { ...errors, submit: 'Failed to create lesson. Please try again.' };
		} finally {
			isSubmitting = false;
		}
	}

	// Generate slug from title
	function generateSlug(title: string): string {
		return title
			.toLowerCase()
			.replace(/[^a-z0-9]+/g, '-')
			.replace(/^-|-$/g, '');
	}

	// Steps config
	const steps = [
		{ number: 1, title: 'Select Rooms', description: 'Choose which trading rooms this lesson belongs to' },
		{ number: 2, title: 'Basic Info', description: 'Title, description, and metadata' },
		{ number: 3, title: 'Content', description: 'Add video, article, or other content' },
		{ number: 4, title: 'Settings', description: 'Access level, status, and tags' }
	];
</script>

<svelte:head>
	<title>Create Lesson | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-page">
	<!-- Header -->
	<header class="page-header">
		<a href="/admin/learning-center" class="back-link">
			<IconArrowLeft size={18} />
			Back to Learning Center
		</a>
		<h1>Create New Lesson</h1>
	</header>

	<!-- Progress Steps -->
	<div class="steps-nav">
		{#each steps as step}
			<div
				class="step"
				class:active={currentStep === step.number}
				class:completed={currentStep > step.number}
			>
				<div class="step-number">
					{#if currentStep > step.number}
						<IconCheck size={14} />
					{:else}
						{step.number}
					{/if}
				</div>
				<div class="step-info">
					<span class="step-title">{step.title}</span>
					<span class="step-desc">{step.description}</span>
				</div>
			</div>
		{/each}
	</div>

	<!-- Form -->
	<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
		<!-- Step 1: Select Trading Rooms -->
		{#if currentStep === 1}
			<div class="form-step">
				<div class="step-header">
					<h2>Which Trading Room(s) is this lesson for?</h2>
					<p>Select one or more trading rooms where this lesson will be available.</p>
				</div>

				<RoomSelector
					rooms={tradingRooms}
					selectedRoomIds={formData.tradingRoomIds}
					onSelect={handleRoomSelect}
					multiple={true}
					showDescription={true}
				/>

				{#if errors.tradingRoomIds}
					<p class="error-message">{errors.tradingRoomIds}</p>
				{/if}
			</div>
		{/if}

		<!-- Step 2: Basic Information -->
		{#if currentStep === 2}
			<div class="form-step">
				<div class="step-header">
					<h2>Lesson Details</h2>
					<p>Enter the basic information about this lesson.</p>
				</div>

				<div class="form-grid">
					<!-- Title -->
					<div class="form-group full-width">
						<label for="title">Title <span class="required">*</span></label>
						<input
							type="text"
							id="title"
							bind:value={formData.title}
							placeholder="Enter lesson title"
							class:error={errors.title}
						/>
						{#if errors.title}
							<span class="error-text">{errors.title}</span>
						{/if}
					</div>

					<!-- Description -->
					<div class="form-group full-width">
						<label for="description">Short Description <span class="required">*</span></label>
						<textarea
							id="description"
							bind:value={formData.description}
							placeholder="Brief description (shown in listings)"
							rows="2"
							class:error={errors.description}
						></textarea>
						{#if errors.description}
							<span class="error-text">{errors.description}</span>
						{/if}
					</div>

					<!-- Full Description -->
					<div class="form-group full-width">
						<label for="fullDescription">Full Description</label>
						<textarea
							id="fullDescription"
							bind:value={formData.fullDescription}
							placeholder="Detailed description (shown on lesson page)"
							rows="4"
						></textarea>
					</div>

					<!-- Trainer -->
					<div class="form-group">
						<label for="trainer">Trainer <span class="required">*</span></label>
						<select
							id="trainer"
							bind:value={formData.trainerId}
							class:error={errors.trainerId}
						>
							<option value="">Select trainer...</option>
							{#each trainers as trainer}
								<option value={trainer.id}>{trainer.name}</option>
							{/each}
						</select>
						{#if errors.trainerId}
							<span class="error-text">{errors.trainerId}</span>
						{/if}
					</div>

					<!-- Category -->
					<div class="form-group">
						<label for="category">Category <span class="required">*</span></label>
						<select
							id="category"
							bind:value={formData.categoryId}
							class:error={errors.categoryId}
						>
							<option value="">Select category...</option>
							{#each categories as category}
								<option value={category.id}>{category.name}</option>
							{/each}
						</select>
						{#if errors.categoryId}
							<span class="error-text">{errors.categoryId}</span>
						{/if}
					</div>

					<!-- Module -->
					{#if availableModules.length > 0}
						<div class="form-group">
							<label for="module">Module (Optional)</label>
							<select id="module" bind:value={formData.moduleId}>
								<option value="">No module</option>
								{#each availableModules as module}
									<option value={module.id}>{module.name}</option>
								{/each}
							</select>
						</div>
					{/if}

					<!-- Duration -->
					<div class="form-group">
						<label for="duration">Duration</label>
						<input
							type="text"
							id="duration"
							bind:value={formData.duration}
							placeholder="e.g., 45:00 or 1:30:00"
						/>
					</div>
				</div>
			</div>
		{/if}

		<!-- Step 3: Content -->
		{#if currentStep === 3}
			<div class="form-step">
				<div class="step-header">
					<h2>Lesson Content</h2>
					<p>Select the content type and add your lesson material.</p>
				</div>

				<!-- Content Type Selection -->
				<div class="type-selector">
					<label class="label">Content Type</label>
					<div class="type-options">
						{#each lessonTypes as type (type.value)}
							{@const Icon = type.icon}
							<button
								type="button"
								class="type-option"
								class:selected={formData.type === type.value}
								onclick={() => formData.type = type.value}
							>
								<Icon size={24} />
								<span class="type-label">{type.label}</span>
								<span class="type-desc">{type.description}</span>
							</button>
						{/each}
					</div>
				</div>

				<!-- Video URL -->
				{#if formData.type === 'video' || formData.type === 'webinar-replay'}
					<div class="form-group">
						<label for="videoUrl">Video URL <span class="required">*</span></label>
						<input
							type="url"
							id="videoUrl"
							bind:value={formData.videoUrl}
							placeholder="https://..."
							class:error={errors.videoUrl}
						/>
						<span class="helper-text">
							Supports YouTube, Vimeo, Wistia, or direct video URLs (S3, etc.)
						</span>
						{#if errors.videoUrl}
							<span class="error-text">{errors.videoUrl}</span>
						{/if}
					</div>
				{/if}

				<!-- Thumbnail -->
				<div class="form-grid">
					<div class="form-group">
						<label for="thumbnailUrl">Thumbnail URL</label>
						<input
							type="url"
							id="thumbnailUrl"
							bind:value={formData.thumbnailUrl}
							placeholder="https://..."
						/>
						<span class="helper-text">
							Image shown in listings (16:9 ratio recommended)
						</span>
					</div>

					<div class="form-group">
						<label for="posterUrl">Poster Image URL</label>
						<input
							type="url"
							id="posterUrl"
							bind:value={formData.posterUrl}
							placeholder="https://..."
						/>
						<span class="helper-text">
							Displayed before video plays
						</span>
					</div>
				</div>

				<!-- Preview -->
				{#if formData.thumbnailUrl || formData.posterUrl}
					<div class="preview-section">
						<label>Preview</label>
						<div class="preview-image">
							<img src={formData.thumbnailUrl || formData.posterUrl} alt="Preview" />
						</div>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Step 4: Settings -->
		{#if currentStep === 4}
			<div class="form-step">
				<div class="step-header">
					<h2>Settings</h2>
					<p>Configure access level, status, and tags.</p>
				</div>

				<!-- Access Level -->
				<div class="form-group">
					<label>Access Level</label>
					<div class="radio-options">
						{#each accessLevels as level}
							<label class="radio-option">
								<input
									type="radio"
									name="accessLevel"
									value={level.value}
									bind:group={formData.accessLevel}
								/>
								<div class="radio-content">
									<span class="radio-label">{level.label}</span>
									<span class="radio-desc">{level.description}</span>
								</div>
							</label>
						{/each}
					</div>
				</div>

				<!-- Status -->
				<div class="form-group">
					<label for="status">Status</label>
					<select id="status" bind:value={formData.status}>
						{#each statusOptions as status}
							<option value={status.value}>{status.label}</option>
						{/each}
					</select>
					<span class="helper-text">
						Draft lessons are not visible to members
					</span>
				</div>

				<!-- Tags -->
				<div class="form-group">
					<label>Tags</label>
					<div class="tags-input">
						<div class="tags-list">
							{#each formData.tags || [] as tag}
								<span class="tag">
									{tag}
									<button type="button" onclick={() => removeTag(tag)}>
										<IconX size={12} />
									</button>
								</span>
							{/each}
						</div>
						<div class="tag-add">
							<input
								type="text"
								bind:value={newTag}
								placeholder="Add a tag..."
								onkeydown={handleTagKeydown}
							/>
							<button type="button" onclick={addTag}>
								<IconPlus size={16} />
							</button>
						</div>
					</div>
				</div>

				<!-- Summary -->
				<div class="summary-section">
					<h3>Summary</h3>
					<div class="summary-grid">
						<div class="summary-item">
							<span class="summary-label">Title</span>
							<span class="summary-value">{formData.title || 'Not set'}</span>
						</div>
						<div class="summary-item">
							<span class="summary-label">Trading Rooms</span>
							<span class="summary-value">
								{formData.tradingRoomIds.length} room(s) selected
							</span>
						</div>
						<div class="summary-item">
							<span class="summary-label">Type</span>
							<span class="summary-value">{formData.type}</span>
						</div>
						<div class="summary-item">
							<span class="summary-label">Status</span>
							<span class="summary-value">{formData.status}</span>
						</div>
					</div>
				</div>

				{#if errors.submit}
					<p class="error-message">{errors.submit}</p>
				{/if}
			</div>
		{/if}

		<!-- Navigation -->
		<div class="form-nav">
			{#if currentStep > 1}
				<button type="button" class="btn-secondary" onclick={handlePrevStep}>
					Back
				</button>
			{:else}
				<div></div>
			{/if}

			{#if currentStep < 4}
				<button type="button" class="btn-primary" onclick={handleNextStep}>
					Continue
				</button>
			{:else}
				<button
					type="submit"
					class="btn-primary"
					disabled={isSubmitting}
				>
					{isSubmitting ? 'Creating...' : 'Create Lesson'}
				</button>
			{/if}
		</div>
	</form>
</div>

<style>
	.admin-page {
		padding: 32px;
		max-width: 900px;
		margin: 0 auto;
	}

	/* Header */
	.page-header {
		margin-bottom: 32px;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		margin-bottom: 12px;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #f97316;
	}

	.page-header h1 {
		margin: 0;
		font-size: 1.75rem;
		font-weight: 700;
		color: white;
	}

	/* Steps Navigation */
	.steps-nav {
		display: flex;
		gap: 8px;
		margin-bottom: 32px;
		padding: 20px;
		background: #1e293b;
		border-radius: 12px;
		border: 1px solid #334155;
		overflow-x: auto;
	}

	.step {
		display: flex;
		align-items: center;
		gap: 12px;
		flex: 1;
		padding: 8px 12px;
		border-radius: 8px;
		opacity: 0.5;
		transition: all 0.2s;
	}

	.step.active,
	.step.completed {
		opacity: 1;
	}

	.step.active {
		background: rgba(249, 115, 22, 0.1);
	}

	.step-number {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		background: #334155;
		color: #94a3b8;
		font-size: 0.8rem;
		font-weight: 600;
		flex-shrink: 0;
	}

	.step.active .step-number {
		background: #f97316;
		color: white;
	}

	.step.completed .step-number {
		background: #22c55e;
		color: white;
	}

	.step-info {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.step-title {
		font-size: 0.8rem;
		font-weight: 600;
		color: white;
	}

	.step-desc {
		font-size: 0.7rem;
		color: #64748b;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* Form Steps */
	.form-step {
		background: #1e293b;
		border-radius: 12px;
		border: 1px solid #334155;
		padding: 32px;
		margin-bottom: 24px;
	}

	.step-header {
		margin-bottom: 24px;
	}

	.step-header h2 {
		margin: 0 0 8px;
		font-size: 1.25rem;
		font-weight: 600;
		color: white;
	}

	.step-header p {
		margin: 0;
		font-size: 0.9rem;
		color: #94a3b8;
	}

	/* Form Grid */
	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 20px;
	}

	.full-width {
		grid-column: 1 / -1;
	}

	/* Form Group */
	.form-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group label,
	.label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #cbd5e1;
	}

	.required {
		color: #ef4444;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 12px 16px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.9rem;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #f97316;
	}

	.form-group input.error,
	.form-group select.error,
	.form-group textarea.error {
		border-color: #ef4444;
	}

	.form-group input::placeholder,
	.form-group textarea::placeholder {
		color: #64748b;
	}

	.helper-text {
		font-size: 0.75rem;
		color: #64748b;
	}

	.error-text {
		font-size: 0.75rem;
		color: #ef4444;
	}

	.error-message {
		padding: 12px 16px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: #ef4444;
		font-size: 0.875rem;
		margin-top: 16px;
	}

	/* Type Selector */
	.type-selector {
		margin-bottom: 24px;
	}

	.type-options {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
		gap: 12px;
		margin-top: 12px;
	}

	.type-option {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		padding: 20px 16px;
		background: #0f172a;
		border: 2px solid #334155;
		border-radius: 10px;
		cursor: pointer;
		text-align: center;
		transition: all 0.2s;
		color: #94a3b8;
	}

	.type-option:hover {
		border-color: #475569;
	}

	.type-option.selected {
		border-color: #f97316;
		background: rgba(249, 115, 22, 0.05);
		color: #f97316;
	}

	.type-label {
		font-size: 0.875rem;
		font-weight: 600;
		color: white;
	}

	.type-option.selected .type-label {
		color: #f97316;
	}

	.type-desc {
		font-size: 0.7rem;
		color: #64748b;
	}

	/* Radio Options */
	.radio-options {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.radio-option {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 16px;
		background: #0f172a;
		border: 2px solid #334155;
		border-radius: 10px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.radio-option:has(input:checked) {
		border-color: #f97316;
		background: rgba(249, 115, 22, 0.05);
	}

	.radio-option input {
		margin-top: 2px;
	}

	.radio-content {
		display: flex;
		flex-direction: column;
	}

	.radio-label {
		font-size: 0.9rem;
		font-weight: 500;
		color: white;
	}

	.radio-desc {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Tags */
	.tags-input {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.tags-list {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 9999px;
		font-size: 0.8rem;
		color: #f97316;
	}

	.tag button {
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		color: inherit;
		opacity: 0.7;
		display: flex;
	}

	.tag button:hover {
		opacity: 1;
	}

	.tag-add {
		display: flex;
		gap: 8px;
	}

	.tag-add input {
		flex: 1;
		padding: 10px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
	}

	.tag-add button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		background: #334155;
		border: none;
		border-radius: 8px;
		color: white;
		cursor: pointer;
	}

	.tag-add button:hover {
		background: #475569;
	}

	/* Preview */
	.preview-section {
		margin-top: 20px;
	}

	.preview-image {
		margin-top: 8px;
		max-width: 400px;
		border-radius: 8px;
		overflow: hidden;
	}

	.preview-image img {
		width: 100%;
		height: auto;
		display: block;
	}

	/* Summary */
	.summary-section {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid #334155;
	}

	.summary-section h3 {
		margin: 0 0 16px;
		font-size: 1rem;
		font-weight: 600;
		color: white;
	}

	.summary-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
	}

	.summary-item {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.summary-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
	}

	.summary-value {
		font-size: 0.9rem;
		color: white;
	}

	/* Form Navigation */
	.form-nav {
		display: flex;
		justify-content: space-between;
		gap: 16px;
	}

	.btn-primary,
	.btn-secondary {
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 0.9rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: #334155;
		color: white;
	}

	.btn-secondary:hover {
		background: #475569;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.admin-page {
			padding: 16px;
		}

		.steps-nav {
			flex-direction: column;
			gap: 4px;
		}

		.step {
			padding: 12px;
		}

		.form-step {
			padding: 20px;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}

		.type-options {
			grid-template-columns: repeat(2, 1fr);
		}

		.summary-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
