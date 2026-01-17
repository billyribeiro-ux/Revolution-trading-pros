<script lang="ts">
	/**
	 * CourseFormModal - Create/Edit Course Modal
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Enterprise-grade modal for course CRUD operations.
	 */
	import {
		adminCoursesApi,
		type Course,
		type CreateCourseRequest,
		type UpdateCourseRequest
	} from '$lib/api/courses';
	import {
		IconX,
		IconBook,
		IconPlus,
		IconEdit,
		IconCurrencyDollar,
		IconUser,
		IconTag
	} from '$lib/icons';

	interface Props {
		isOpen: boolean;
		mode?: 'create' | 'edit';
		course?: Course | null;
		onSave?: (course: Course) => void;
		onSaved?: (course: Course) => void;
		onClose: () => void;
	}

	let {
		isOpen,
		mode: modeProp,
		course = null,
		onSave,
		onSaved,
		onClose
	}: Props = $props();

	// Derive mode from props
	let mode = $derived(modeProp ?? (course ? 'edit' : 'create'));

	const handleSaved = (c: Course) => {
		onSave?.(c);
		onSaved?.(c);
	};

	// Form state
	let title = $state('');
	let slug = $state('');
	let description = $state('');
	let cardDescription = $state('');
	let priceCents = $state(0);
	let isFree = $state(true);
	let level = $state('');
	let instructorName = $state('');
	let instructorTitle = $state('');
	let instructorBio = $state('');
	let cardBadge = $state('');
	let cardBadgeColor = $state('#6366f1');
	let metaTitle = $state('');
	let metaDescription = $state('');

	// UI state
	let isLoading = $state(false);
	let error = $state('');
	let activeSection = $state<'basic' | 'pricing' | 'instructor' | 'seo'>('basic');

	const levels = [
		{ value: '', label: 'Select Level' },
		{ value: 'beginner', label: 'Beginner' },
		{ value: 'intermediate', label: 'Intermediate' },
		{ value: 'advanced', label: 'Advanced' },
		{ value: 'all-levels', label: 'All Levels' }
	];

	const badgeColors = [
		{ value: '#6366f1', label: 'Purple' },
		{ value: '#22c55e', label: 'Green' },
		{ value: '#f59e0b', label: 'Orange' },
		{ value: '#ef4444', label: 'Red' },
		{ value: '#3b82f6', label: 'Blue' },
		{ value: '#ec4899', label: 'Pink' }
	];

	// Initialize form when course changes
	$effect(() => {
		if (isOpen && mode === 'edit' && course) {
			title = course.title || '';
			slug = course.slug || '';
			description = course.description || '';
			cardDescription = course.card_description || '';
			priceCents = course.price_cents || 0;
			isFree = course.is_free ?? (course.price_cents === 0);
			level = course.level || '';
			instructorName = course.instructor_name || '';
			instructorTitle = course.instructor_title || '';
			instructorBio = course.instructor_bio || '';
			cardBadge = course.card_badge || '';
			cardBadgeColor = course.card_badge_color || '#6366f1';
			metaTitle = course.meta_title || '';
			metaDescription = course.meta_description || '';
		} else if (isOpen && mode === 'create') {
			title = '';
			slug = '';
			description = '';
			cardDescription = '';
			priceCents = 0;
			isFree = true;
			level = '';
			instructorName = '';
			instructorTitle = '';
			instructorBio = '';
			cardBadge = '';
			cardBadgeColor = '#6366f1';
			metaTitle = '';
			metaDescription = '';
		}
		error = '';
		activeSection = 'basic';
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

	// Auto-generate slug from title
	function generateSlug(text: string): string {
		return text
			.toLowerCase()
			.replace(/[^a-z0-9\s-]/g, '')
			.replace(/\s+/g, '-')
			.replace(/-+/g, '-')
			.trim();
	}

	function handleTitleInput() {
		if (mode === 'create' && (!slug || slug === generateSlug(title.slice(0, -1)))) {
			slug = generateSlug(title);
		}
	}

	function validateForm(): boolean {
		if (!title.trim()) {
			error = 'Course title is required';
			activeSection = 'basic';
			return false;
		}
		if (!slug.trim()) {
			error = 'Course slug is required';
			activeSection = 'basic';
			return false;
		}
		if (!isFree && priceCents <= 0) {
			error = 'Price must be greater than 0 for paid courses';
			activeSection = 'pricing';
			return false;
		}
		return true;
	}

	async function handleSubmit() {
		if (!validateForm()) return;

		isLoading = true;
		error = '';

		try {
			if (mode === 'create') {
				const data: CreateCourseRequest = {
					title: title.trim(),
					slug: slug.trim(),
					description: description.trim() || undefined,
					card_description: cardDescription.trim() || undefined,
					price_cents: isFree ? 0 : priceCents,
					is_free: isFree,
					level: level || undefined,
					instructor_name: instructorName.trim() || undefined,
					instructor_title: instructorTitle.trim() || undefined,
					instructor_bio: instructorBio.trim() || undefined,
					card_badge: cardBadge.trim() || undefined,
					card_badge_color: cardBadge ? cardBadgeColor : undefined,
					meta_title: metaTitle.trim() || undefined,
					meta_description: metaDescription.trim() || undefined
				};

				const result = await adminCoursesApi.create(data);
				handleSaved(result);
				onClose();
			} else if (mode === 'edit' && course) {
				const data: UpdateCourseRequest = {};

				if (title.trim() !== course.title) data.title = title.trim();
				if (slug.trim() !== course.slug) data.slug = slug.trim();
				if (description.trim() !== (course.description || '')) data.description = description.trim() || undefined;
				if (cardDescription.trim() !== (course.card_description || '')) data.card_description = cardDescription.trim() || undefined;
				if ((isFree ? 0 : priceCents) !== course.price_cents) data.price_cents = isFree ? 0 : priceCents;
				if (isFree !== (course.is_free ?? false)) data.is_free = isFree;
				if (level !== (course.level || '')) data.level = level || undefined;
				if (instructorName.trim() !== (course.instructor_name || '')) data.instructor_name = instructorName.trim() || undefined;
				if (instructorTitle.trim() !== (course.instructor_title || '')) data.instructor_title = instructorTitle.trim() || undefined;
				if (instructorBio.trim() !== (course.instructor_bio || '')) data.instructor_bio = instructorBio.trim() || undefined;
				if (cardBadge.trim() !== (course.card_badge || '')) data.card_badge = cardBadge.trim() || undefined;
				if (cardBadgeColor !== (course.card_badge_color || '#6366f1')) data.card_badge_color = cardBadgeColor;
				if (metaTitle.trim() !== (course.meta_title || '')) data.meta_title = metaTitle.trim() || undefined;
				if (metaDescription.trim() !== (course.meta_description || '')) data.meta_description = metaDescription.trim() || undefined;

				const result = await adminCoursesApi.update(course.id, data);
				handleSaved(result);
				onClose();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			isLoading = false;
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget && !isLoading) {
			onClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && !isLoading) {
			onClose();
		}
	}

	function formatPrice(cents: number): string {
		return (cents / 100).toFixed(2);
	}

	function handlePriceInput(e: Event) {
		const target = e.target as HTMLInputElement;
		const value = parseFloat(target.value) || 0;
		priceCents = Math.round(value * 100);
	}
</script>

{#if isOpen}
	<div
		class="modal-backdrop"
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		tabindex="-1"
	>
		<div class="modal-container">
			<!-- Header -->
			<div class="modal-header">
				<div class="header-icon">
					{#if mode === 'create'}
						<IconPlus size={24} />
					{:else}
						<IconEdit size={24} />
					{/if}
				</div>
				<h2 id="modal-title" class="modal-title">
					{mode === 'create' ? 'Create Course' : 'Edit Course'}
				</h2>
				<button type="button" class="btn-close" onclick={onClose} disabled={isLoading} aria-label="Close">
					<IconX size={20} />
				</button>
			</div>

			<!-- Section Tabs -->
			<nav class="section-tabs">
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'basic'}
					onclick={() => activeSection = 'basic'}
				>
					<IconBook size={16} />
					Basic
				</button>
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'pricing'}
					onclick={() => activeSection = 'pricing'}
				>
					<IconCurrencyDollar size={16} />
					Pricing
				</button>
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'instructor'}
					onclick={() => activeSection = 'instructor'}
				>
					<IconUser size={16} />
					Instructor
				</button>
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'seo'}
					onclick={() => activeSection = 'seo'}
				>
					<IconTag size={16} />
					SEO
				</button>
			</nav>

			<!-- Form -->
			<form class="modal-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				{#if error}
					<div class="error-banner">{error}</div>
				{/if}

				{#if activeSection === 'basic'}
					<div class="form-section">
						<div class="form-group">
							<label for="title" class="form-label">Course Title <span class="required">*</span></label>
							<input
								id="title"
								type="text"
								class="form-input"
								placeholder="e.g., Options Trading Mastery"
								bind:value={title}
								oninput={handleTitleInput}
								disabled={isLoading}
							/>
						</div>

						<div class="form-group">
							<label for="slug" class="form-label">URL Slug <span class="required">*</span></label>
							<div class="slug-input-wrapper">
								<span class="slug-prefix">/classes/</span>
								<input
									id="slug"
									type="text"
									class="form-input slug-input"
									placeholder="options-trading-mastery"
									bind:value={slug}
									disabled={isLoading}
								/>
							</div>
						</div>

						<div class="form-group">
							<label for="description" class="form-label">Description</label>
							<textarea
								id="description"
								class="form-textarea"
								placeholder="Full course description..."
								bind:value={description}
								disabled={isLoading}
								rows="3"
							></textarea>
						</div>

						<div class="form-group">
							<label for="cardDescription" class="form-label">Card Description</label>
							<textarea
								id="cardDescription"
								class="form-textarea"
								placeholder="Short description for course cards..."
								bind:value={cardDescription}
								disabled={isLoading}
								rows="2"
							></textarea>
							<span class="form-hint">Brief description shown on course cards</span>
						</div>

						<div class="form-group">
							<label for="level" class="form-label">Difficulty Level</label>
							<select
								id="level"
								class="form-select"
								bind:value={level}
								disabled={isLoading}
							>
								{#each levels as lvl}
									<option value={lvl.value}>{lvl.label}</option>
								{/each}
							</select>
						</div>
					</div>

				{:else if activeSection === 'pricing'}
					<div class="form-section">
						<div class="form-group">
							<label class="toggle-label">
								<input
									type="checkbox"
									bind:checked={isFree}
									disabled={isLoading}
								/>
								<span class="toggle-text">
									<strong>Free Course</strong>
									<span>Make this course available at no cost</span>
								</span>
							</label>
						</div>

						{#if !isFree}
							<div class="form-group">
								<label for="price" class="form-label">Price (USD) <span class="required">*</span></label>
								<div class="price-input-wrapper">
									<span class="price-symbol">$</span>
									<input
										id="price"
										type="number"
										class="form-input price-input"
										placeholder="0.00"
										step="0.01"
										min="0"
										value={formatPrice(priceCents)}
										oninput={handlePriceInput}
										disabled={isLoading}
									/>
								</div>
							</div>
						{/if}

						<div class="form-group">
							<label for="cardBadge" class="form-label">Card Badge</label>
							<input
								id="cardBadge"
								type="text"
								class="form-input"
								placeholder="e.g., New, Popular, Bestseller"
								bind:value={cardBadge}
								disabled={isLoading}
							/>
							<span class="form-hint">Badge displayed on course cards</span>
						</div>

						{#if cardBadge}
							<div class="form-group" role="radiogroup" aria-labelledby="badge-color-label">
								<span id="badge-color-label" class="form-label">Badge Color</span>
								<div class="color-options">
									{#each badgeColors as color}
										<button
											type="button"
											class="color-option"
											class:selected={cardBadgeColor === color.value}
											style="--color: {color.value}"
											onclick={() => cardBadgeColor = color.value}
											disabled={isLoading}
											title={color.label}
										></button>
									{/each}
								</div>
							</div>

							<div class="badge-preview">
								<span class="preview-label">Preview:</span>
								<span class="preview-badge" style="background: {cardBadgeColor}">{cardBadge}</span>
							</div>
						{/if}
					</div>

				{:else if activeSection === 'instructor'}
					<div class="form-section">
						<div class="form-group">
							<label for="instructorName" class="form-label">Instructor Name</label>
							<input
								id="instructorName"
								type="text"
								class="form-input"
								placeholder="e.g., John Smith"
								bind:value={instructorName}
								disabled={isLoading}
							/>
						</div>

						<div class="form-group">
							<label for="instructorTitle" class="form-label">Instructor Title</label>
							<input
								id="instructorTitle"
								type="text"
								class="form-input"
								placeholder="e.g., Senior Trading Coach"
								bind:value={instructorTitle}
								disabled={isLoading}
							/>
						</div>

						<div class="form-group">
							<label for="instructorBio" class="form-label">Instructor Bio</label>
							<textarea
								id="instructorBio"
								class="form-textarea"
								placeholder="Brief bio about the instructor..."
								bind:value={instructorBio}
								disabled={isLoading}
								rows="4"
							></textarea>
						</div>
					</div>

				{:else if activeSection === 'seo'}
					<div class="form-section">
						<div class="form-group">
							<label for="metaTitle" class="form-label">Meta Title</label>
							<input
								id="metaTitle"
								type="text"
								class="form-input"
								placeholder="SEO title for search engines"
								bind:value={metaTitle}
								disabled={isLoading}
							/>
							<span class="form-hint">{metaTitle.length}/60 characters</span>
						</div>

						<div class="form-group">
							<label for="metaDescription" class="form-label">Meta Description</label>
							<textarea
								id="metaDescription"
								class="form-textarea"
								placeholder="SEO description for search engines..."
								bind:value={metaDescription}
								disabled={isLoading}
								rows="3"
							></textarea>
							<span class="form-hint">{metaDescription.length}/160 characters</span>
						</div>
					</div>
				{/if}

				<!-- Actions -->
				<div class="modal-actions">
					<button type="button" class="btn-cancel" onclick={onClose} disabled={isLoading}>
						Cancel
					</button>
					<button type="submit" class="btn-submit" disabled={isLoading}>
						{#if isLoading}
							<span class="spinner"></span>
							{mode === 'create' ? 'Creating...' : 'Saving...'}
						{:else}
							{mode === 'create' ? 'Create Course' : 'Save Changes'}
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: var(--z-modal, 1000);
		padding: 1rem;
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.modal-container {
		background: var(--admin-surface-primary);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-xl, 1rem);
		max-width: 600px;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(20px) scale(0.95);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.modal-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--admin-border-subtle);
	}

	.header-icon {
		width: 40px;
		height: 40px;
		border-radius: var(--radius-md, 0.5rem);
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.modal-title {
		flex: 1;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--admin-text-primary);
		margin: 0;
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

	.btn-close:hover:not(:disabled) {
		background: var(--admin-surface-hover);
		color: var(--admin-text-primary);
	}

	/* Section Tabs */
	.section-tabs {
		display: flex;
		padding: 0 1rem;
		border-bottom: 1px solid var(--admin-border-subtle);
		background: var(--admin-surface-sunken);
		overflow-x: auto;
	}

	.section-tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.75rem 1rem;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--admin-text-muted);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.section-tab:hover {
		color: var(--admin-text-secondary);
	}

	.section-tab.active {
		color: var(--admin-accent-primary);
		border-bottom-color: var(--admin-accent-primary);
	}

	.modal-form {
		padding: 1.5rem;
	}

	.form-section {
		animation: fadeIn 0.2s ease;
	}

	.error-banner {
		background: var(--admin-error-bg);
		border: 1px solid var(--admin-error-border);
		color: var(--admin-error);
		padding: 0.75rem 1rem;
		border-radius: var(--radius-md, 0.5rem);
		margin-bottom: 1.25rem;
		font-size: 0.875rem;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-label {
		display: block;
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--admin-text-secondary);
		margin-bottom: 0.5rem;
	}

	.required {
		color: var(--admin-error);
	}

	.form-hint {
		display: block;
		font-size: 0.75rem;
		color: var(--admin-text-muted);
		margin-top: 0.375rem;
	}

	.form-input,
	.form-textarea,
	.form-select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--admin-text-primary);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.9375rem;
		transition: border-color 0.2s ease, box-shadow 0.2s ease;
	}

	.form-textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-select {
		cursor: pointer;
	}

	.form-input:focus,
	.form-textarea:focus,
	.form-select:focus {
		outline: none;
		border-color: var(--admin-accent-primary);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.form-input::placeholder,
	.form-textarea::placeholder {
		color: var(--admin-text-muted);
	}

	.form-input:disabled,
	.form-textarea:disabled,
	.form-select:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Slug Input */
	.slug-input-wrapper {
		display: flex;
		align-items: center;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		overflow: hidden;
	}

	.slug-prefix {
		padding: 0.75rem;
		background: var(--admin-surface-hover);
		color: var(--admin-text-muted);
		font-size: 0.875rem;
		font-family: monospace;
		border-right: 1px solid var(--admin-border-subtle);
	}

	.slug-input {
		border: none;
		border-radius: 0;
		flex: 1;
		font-family: monospace;
	}

	.slug-input:focus {
		box-shadow: none;
	}

	/* Price Input */
	.price-input-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.price-symbol {
		position: absolute;
		left: 1rem;
		color: var(--admin-text-muted);
		font-size: 1rem;
	}

	.price-input {
		padding-left: 2rem;
	}

	/* Toggle Label */
	.toggle-label {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		cursor: pointer;
	}

	.toggle-label input {
		width: 18px;
		height: 18px;
		accent-color: var(--admin-accent-primary);
		margin-top: 0.125rem;
	}

	.toggle-text {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.toggle-text strong {
		color: var(--admin-text-primary);
	}

	.toggle-text span:not(strong) {
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
	}

	/* Color Options */
	.color-options {
		display: flex;
		gap: 0.5rem;
	}

	.color-option {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		background: var(--color);
		border: 2px solid transparent;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.color-option:hover {
		transform: scale(1.1);
	}

	.color-option.selected {
		border-color: var(--admin-text-primary);
		box-shadow: 0 0 0 2px var(--admin-surface-primary);
	}

	/* Badge Preview */
	.badge-preview {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: var(--admin-surface-sunken);
		border-radius: var(--radius-md, 0.5rem);
	}

	.preview-label {
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
	}

	.preview-badge {
		padding: 0.25rem 0.625rem;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
		color: white;
	}

	/* Actions */
	.modal-actions {
		display: flex;
		gap: 0.75rem;
		padding-top: 1rem;
		border-top: 1px solid var(--admin-border-subtle);
		margin-top: 1.5rem;
	}

	.btn-cancel,
	.btn-submit {
		flex: 1;
		padding: 0.75rem 1.5rem;
		border-radius: var(--radius-md, 0.5rem);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.9375rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-cancel {
		background: transparent;
		border: 1px solid var(--admin-border-subtle);
		color: var(--admin-text-secondary);
	}

	.btn-cancel:hover:not(:disabled) {
		background: var(--admin-surface-hover);
		border-color: var(--admin-border-light);
	}

	.btn-submit {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border: none;
		color: white;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
	}

	.btn-submit:hover:not(:disabled) {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	.btn-cancel:disabled,
	.btn-submit:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.spinner {
		display: inline-block;
		width: 16px;
		height: 16px;
		border: 2px solid transparent;
		border-top-color: currentColor;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>
