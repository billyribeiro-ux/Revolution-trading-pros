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

	let props: Props = $props();

	// Destructure with defaults for internal use
	const isOpen = $derived(props.isOpen);
	const modeProp = $derived(props.mode);
	const course = $derived(props.course ?? null);
	const onSave = $derived(props.onSave);
	const onSaved = $derived(props.onSaved);
	const onClose = $derived(props.onClose);

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
			isFree = course.is_free ?? course.price_cents === 0;
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
				if (description.trim() !== (course.description || ''))
					data.description = description.trim() || undefined;
				if (cardDescription.trim() !== (course.card_description || ''))
					data.card_description = cardDescription.trim() || undefined;
				if ((isFree ? 0 : priceCents) !== course.price_cents)
					data.price_cents = isFree ? 0 : priceCents;
				if (isFree !== (course.is_free ?? false)) data.is_free = isFree;
				if (level !== (course.level || '')) data.level = level || undefined;
				if (instructorName.trim() !== (course.instructor_name || ''))
					data.instructor_name = instructorName.trim() || undefined;
				if (instructorTitle.trim() !== (course.instructor_title || ''))
					data.instructor_title = instructorTitle.trim() || undefined;
				if (instructorBio.trim() !== (course.instructor_bio || ''))
					data.instructor_bio = instructorBio.trim() || undefined;
				if (cardBadge.trim() !== (course.card_badge || ''))
					data.card_badge = cardBadge.trim() || undefined;
				if (cardBadgeColor !== (course.card_badge_color || '#6366f1'))
					data.card_badge_color = cardBadgeColor;
				if (metaTitle.trim() !== (course.meta_title || ''))
					data.meta_title = metaTitle.trim() || undefined;
				if (metaDescription.trim() !== (course.meta_description || ''))
					data.meta_description = metaDescription.trim() || undefined;

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
				<button
					type="button"
					class="btn-close"
					onclick={onClose}
					disabled={isLoading}
					aria-label="Close"
				>
					<IconX size={20} />
				</button>
			</div>

			<!-- Section Tabs -->
			<nav class="section-tabs">
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'basic'}
					onclick={() => (activeSection = 'basic')}
				>
					<IconBook size={16} />
					Basic
				</button>
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'pricing'}
					onclick={() => (activeSection = 'pricing')}
				>
					<IconCurrencyDollar size={16} />
					Pricing
				</button>
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'instructor'}
					onclick={() => (activeSection = 'instructor')}
				>
					<IconUser size={16} />
					Instructor
				</button>
				<button
					type="button"
					class="section-tab"
					class:active={activeSection === 'seo'}
					onclick={() => (activeSection = 'seo')}
				>
					<IconTag size={16} />
					SEO
				</button>
			</nav>

			<!-- Form -->
			<form
				class="modal-form"
				onsubmit={(e) => {
					e.preventDefault();
					handleSubmit();
				}}
			>
				{#if error}
					<div class="error-banner">{error}</div>
				{/if}

				{#if activeSection === 'basic'}
					<div class="form-section">
						<div class="form-group">
							<label for="title" class="form-label"
								>Course Title <span class="required">*</span></label
							>
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
							<select id="level" class="form-select" bind:value={level} disabled={isLoading}>
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
								<input type="checkbox" bind:checked={isFree} disabled={isLoading} />
								<span class="toggle-text">
									<strong>Free Course</strong>
									<span>Make this course available at no cost</span>
								</span>
							</label>
						</div>

						{#if !isFree}
							<div class="form-group">
								<label for="price" class="form-label"
									>Price (USD) <span class="required">*</span></label
								>
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
											onclick={() => (cardBadgeColor = color.value)}
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
	/* ═══════════════════════════════════════════════════════════════════════════
	   COURSE FORM MODAL - ICT 7 Principal Engineer Grade
	   ═══════════════════════════════════════════════════════════════════════════ */

	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: var(--z-modal);
		padding: var(--space-4);
		animation: fadeIn var(--duration-fast) var(--ease-default);
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.modal-container {
		background: var(--admin-surface-primary);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-xl);
		max-width: 600px;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: var(--shadow-xl);
		animation: slideUp var(--duration-normal) var(--ease-out);
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
		gap: var(--space-3);
		padding: var(--space-5) var(--space-6);
		border-bottom: 1px solid var(--admin-border-subtle);
	}

	.header-icon {
		width: 40px;
		height: 40px;
		border-radius: var(--radius-md);
		background: var(--admin-accent-bg);
		color: var(--admin-accent-primary);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.modal-title {
		flex: 1;
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: var(--font-semibold);
		color: var(--admin-text-primary);
		margin: 0;
	}

	.btn-close {
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: var(--space-2);
		border-radius: var(--radius-sm);
		transition: var(--transition-all);
		min-width: 44px;
		min-height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.btn-close:hover:not(:disabled) {
		background: var(--admin-surface-hover);
		color: var(--admin-text-primary);
	}

	/* Section Tabs */
	.section-tabs {
		display: flex;
		padding: 0 var(--space-4);
		border-bottom: 1px solid var(--admin-border-subtle);
		background: var(--admin-surface-sunken);
		overflow-x: auto;
	}

	.section-tab {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-4);
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--admin-text-muted);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		cursor: pointer;
		transition: var(--transition-all);
		white-space: nowrap;
		min-height: 44px;
	}

	.section-tab:hover {
		color: var(--admin-text-secondary);
	}

	.section-tab.active {
		color: var(--admin-accent-primary);
		border-bottom-color: var(--admin-accent-primary);
	}

	.modal-form {
		padding: var(--space-6);
	}

	.form-section {
		animation: fadeIn var(--duration-fast) var(--ease-default);
	}

	.error-banner {
		background: var(--color-loss-bg);
		border: 1px solid var(--color-loss);
		color: var(--color-loss);
		padding: var(--space-3) var(--space-4);
		border-radius: var(--radius-md);
		margin-bottom: var(--space-5);
		font-size: var(--text-sm);
	}

	.form-group {
		margin-bottom: var(--space-5);
	}

	.form-label {
		display: block;
		font-family: var(--font-sans);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		color: var(--admin-text-secondary);
		margin-bottom: var(--space-2);
	}

	.required {
		color: var(--color-loss);
	}

	.form-hint {
		display: block;
		font-size: var(--text-xs);
		color: var(--admin-text-muted);
		margin-top: var(--space-2);
	}

	.form-input,
	.form-textarea,
	.form-select {
		width: 100%;
		padding: var(--space-3) var(--space-4);
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md);
		color: var(--admin-text-primary);
		font-family: var(--font-sans);
		font-size: var(--text-base);
		transition: var(--transition-colors);
		min-height: 44px;
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
		box-shadow: var(--admin-focus-ring);
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
		border-radius: var(--radius-md);
		overflow: hidden;
	}

	.slug-prefix {
		padding: var(--space-3);
		background: var(--admin-surface-hover);
		color: var(--admin-text-muted);
		font-size: var(--text-sm);
		font-family: var(--font-mono);
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
		left: var(--space-4);
		color: var(--admin-text-muted);
		font-size: var(--text-base);
	}

	.price-input {
		padding-left: 2rem;
	}

	/* Toggle Label */
	.toggle-label {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		padding: var(--space-4);
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md);
		cursor: pointer;
		min-height: 44px;
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
		gap: var(--space-1);
	}

	.toggle-text strong {
		color: var(--admin-text-primary);
	}

	.toggle-text span:not(strong) {
		font-size: var(--text-sm);
		color: var(--admin-text-muted);
	}

	/* Color Options */
	.color-options {
		display: flex;
		gap: var(--space-2);
		flex-wrap: wrap;
	}

	.color-option {
		width: 36px;
		height: 36px;
		border-radius: var(--radius-full);
		background: var(--color);
		border: 2px solid transparent;
		cursor: pointer;
		transition: var(--transition-all);
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
		gap: var(--space-3);
		padding: var(--space-3) var(--space-4);
		background: var(--admin-surface-sunken);
		border-radius: var(--radius-md);
	}

	.preview-label {
		font-size: var(--text-sm);
		color: var(--admin-text-muted);
	}

	.preview-badge {
		padding: var(--space-1) var(--space-3);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
		font-weight: var(--font-semibold);
		color: white;
	}

	/* Actions */
	.modal-actions {
		display: flex;
		gap: var(--space-3);
		padding-top: var(--space-4);
		border-top: 1px solid var(--admin-border-subtle);
		margin-top: var(--space-6);
	}

	.btn-cancel,
	.btn-submit {
		flex: 1;
		padding: var(--space-3) var(--space-6);
		border-radius: var(--radius-md);
		font-family: var(--font-sans);
		font-size: var(--text-base);
		font-weight: var(--font-medium);
		cursor: pointer;
		transition: var(--transition-all);
		min-height: 44px;
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
		background: var(--admin-accent-primary);
		border: none;
		color: var(--color-text-primary);
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
	}

	.btn-submit:hover:not(:disabled) {
		background: var(--admin-accent-primary-hover);
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
		border-radius: var(--radius-full);
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   2026 MOBILE-FIRST RESPONSIVE DESIGN
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* DYNAMIC VIEWPORT HEIGHT */
	@supports (height: 100dvh) {
		.modal-container {
			max-height: 90dvh;
		}
	}

	/* SAFE AREA INSETS */
	.modal-backdrop {
		padding: env(safe-area-inset-top) env(safe-area-inset-right) env(safe-area-inset-bottom)
			env(safe-area-inset-left);
	}

	.modal-actions {
		padding-bottom: calc(var(--space-4) + env(safe-area-inset-bottom));
	}

	/* TOUCH TARGETS - All interactive elements minimum 44x44px */
	@media (hover: none) and (pointer: coarse) {
		.btn-close,
		.btn-cancel,
		.btn-submit,
		.section-tab,
		.color-option,
		.form-input,
		.form-textarea,
		.form-select,
		.toggle-label {
			min-height: 44px;
		}

		.color-option {
			width: 44px;
			height: 44px;
		}

		.section-tab {
			padding: var(--space-3) var(--space-4);
		}
	}

	/* EXTRA SMALL DEVICES (< 360px) */
	@media (max-width: 359px) {
		.modal-backdrop {
			padding: 0.5rem;
		}

		.modal-container {
			max-height: 98vh;
			border-radius: var(--radius-lg);
		}

		.modal-header {
			padding: 0.75rem;
			gap: 0.5rem;
		}

		.header-icon {
			width: 32px;
			height: 32px;
		}

		.modal-title {
			font-size: var(--text-base);
		}

		.section-tabs {
			padding: 0 0.5rem;
		}

		.section-tab {
			padding: var(--space-2) var(--space-3);
			font-size: 0.75rem;
			gap: var(--space-1);
		}

		.section-tab :global(svg) {
			width: 14px;
			height: 14px;
		}

		.modal-form {
			padding: 0.75rem;
		}

		.form-group {
			margin-bottom: var(--space-4);
		}

		.form-label {
			font-size: 0.75rem;
		}

		.form-input,
		.form-textarea,
		.form-select {
			padding: var(--space-2) var(--space-3);
			font-size: 0.875rem;
		}

		.modal-actions {
			flex-direction: column;
			gap: var(--space-2);
			margin-top: var(--space-4);
		}

		.slug-prefix {
			display: none;
		}

		.slug-input {
			border-radius: var(--radius-md);
		}

		.color-options {
			justify-content: center;
		}
	}

	/* SMALL MOBILE (360px - 639px) */
	@media (min-width: 360px) and (max-width: 639px) {
		.modal-backdrop {
			padding: 1rem;
		}

		.modal-container {
			max-height: 95vh;
		}

		.modal-header {
			padding: var(--space-4);
		}

		.modal-form {
			padding: var(--space-4);
		}

		.modal-actions {
			flex-direction: column;
			gap: var(--space-2);
		}

		.slug-prefix {
			display: none;
		}

		.slug-input {
			border-radius: var(--radius-md);
		}

		.toggle-label {
			padding: var(--space-3);
		}

		.color-options {
			justify-content: center;
		}
	}

	/* TABLET (640px - 767px) */
	@media (min-width: 640px) and (max-width: 767px) {
		.modal-container {
			max-width: 500px;
		}

		.modal-actions {
			flex-direction: row;
		}
	}

	/* MEDIUM DEVICES (768px+) */
	@media (min-width: 768px) {
		.modal-container {
			max-width: 600px;
		}

		.modal-header {
			padding: var(--space-5) var(--space-6);
		}

		.modal-form {
			padding: var(--space-6);
		}
	}

	/* LARGE DEVICES (1024px+) */
	@media (min-width: 1024px) {
		.modal-container {
			max-width: 650px;
		}
	}

	/* LANDSCAPE MOBILE */
	@media (max-height: 500px) and (orientation: landscape) {
		.modal-container {
			max-height: 95vh;
		}

		.modal-header {
			padding: var(--space-3) var(--space-4);
		}

		.modal-form {
			padding: var(--space-3) var(--space-4);
		}

		.form-group {
			margin-bottom: var(--space-3);
		}

		.form-textarea {
			min-height: 60px;
		}

		.modal-actions {
			margin-top: var(--space-3);
			padding-top: var(--space-3);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.modal-backdrop,
		.modal-container,
		.form-section {
			animation: none;
		}

		.btn-close,
		.btn-cancel,
		.btn-submit,
		.section-tab,
		.color-option,
		.form-input,
		.form-textarea,
		.form-select {
			transition: none;
		}
	}
</style>
