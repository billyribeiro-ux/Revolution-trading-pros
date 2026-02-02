<script lang="ts">
	/**
	 * ModuleFormModal - Create/Edit Module Modal
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Enterprise-grade modal for course module CRUD operations.
	 */
	import {
		adminCoursesApi,
		type CourseModule,
		type CreateModuleRequest,
		type UpdateModuleRequest
	} from '$lib/api/courses';
	import { IconX, IconFolder, IconPlus, IconEdit, IconCalendar, IconClock } from '$lib/icons';

	interface Props {
		isOpen: boolean;
		mode?: 'create' | 'edit';
		courseId: string | null;
		module?: CourseModule | null;
		nextSortOrder?: number;
		onSave?: (module: CourseModule) => void;
		onSaved?: (module: CourseModule) => void;
		onClose: () => void;
	}

	let props: Props = $props();

	// Destructure with defaults for internal use
	const isOpen = $derived(props.isOpen);
	const modeProp = $derived(props.mode);
	const courseId = $derived(props.courseId);
	const module = $derived(props.module ?? null);
	const nextSortOrder = $derived(props.nextSortOrder ?? 1);
	const onSave = $derived(props.onSave);
	const onSaved = $derived(props.onSaved);
	const onClose = $derived(props.onClose);

	// Derive mode from props
	let mode = $derived(modeProp ?? (module ? 'edit' : 'create'));

	const handleSaved = (m: CourseModule) => {
		onSave?.(m);
		onSaved?.(m);
	};

	// Form state
	let title = $state('');
	let description = $state('');
	let sortOrder = $state(1);
	let isPublished = $state(true);
	let dripEnabled = $state(false);
	let dripDays = $state(0);

	// UI state
	let isLoading = $state(false);
	let error = $state('');

	// Initialize form when module changes
	$effect(() => {
		if (isOpen && mode === 'edit' && module) {
			title = module.title || '';
			description = module.description || '';
			sortOrder = module.sort_order || 1;
			isPublished = module.is_published ?? true;
			dripEnabled = module.drip_enabled ?? false;
			dripDays = module.drip_days || 0;
		} else if (isOpen && mode === 'create') {
			title = '';
			description = '';
			sortOrder = nextSortOrder;
			isPublished = true;
			dripEnabled = false;
			dripDays = 0;
		}
		error = '';
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

	function validateForm(): boolean {
		if (!title.trim()) {
			error = 'Module title is required';
			return false;
		}
		if (!courseId) {
			error = 'Course ID is missing';
			return false;
		}
		if (dripEnabled && dripDays < 0) {
			error = 'Drip days must be 0 or greater';
			return false;
		}
		return true;
	}

	async function handleSubmit() {
		if (!validateForm() || !courseId) return;

		isLoading = true;
		error = '';

		try {
			if (mode === 'create') {
				const data: CreateModuleRequest = {
					title: title.trim(),
					description: description.trim() || undefined,
					sort_order: sortOrder,
					is_published: isPublished,
					drip_enabled: dripEnabled,
					drip_days: dripEnabled ? dripDays : undefined
				};

				const result = await adminCoursesApi.createModule(courseId, data);
				handleSaved(result);
				onClose();
			} else if (mode === 'edit' && module) {
				const data: UpdateModuleRequest = {};

				if (title.trim() !== module.title) data.title = title.trim();
				if (description.trim() !== (module.description || ''))
					data.description = description.trim() || undefined;
				if (sortOrder !== module.sort_order) data.sort_order = sortOrder;
				if (isPublished !== module.is_published) data.is_published = isPublished;
				if (dripEnabled !== (module.drip_enabled ?? false)) data.drip_enabled = dripEnabled;
				if (dripDays !== (module.drip_days || 0))
					data.drip_days = dripEnabled ? dripDays : undefined;

				const result = await adminCoursesApi.updateModule(courseId, module.id, data);
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
					{mode === 'create' ? 'Add Module' : 'Edit Module'}
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

				<div class="form-group">
					<label for="title" class="form-label">Module Title <span class="required">*</span></label>
					<input
						id="title"
						type="text"
						class="form-input"
						placeholder="e.g., Introduction to Options"
						bind:value={title}
						disabled={isLoading}
					/>
				</div>

				<div class="form-group">
					<label for="description" class="form-label">Description</label>
					<textarea
						id="description"
						class="form-textarea"
						placeholder="Brief description of this module..."
						bind:value={description}
						disabled={isLoading}
						rows="3"
					></textarea>
				</div>

				<div class="form-row">
					<div class="form-group half">
						<label for="sortOrder" class="form-label">Position</label>
						<input
							id="sortOrder"
							type="number"
							class="form-input"
							min="1"
							bind:value={sortOrder}
							disabled={isLoading}
						/>
						<span class="form-hint">Order in course curriculum</span>
					</div>

					<div class="form-group half">
						<span class="form-label">Status</span>
						<div class="status-toggle" role="group" aria-label="Module status">
							<button
								type="button"
								class="status-btn"
								class:active={isPublished}
								onclick={() => (isPublished = true)}
								disabled={isLoading}
							>
								Published
							</button>
							<button
								type="button"
								class="status-btn"
								class:active={!isPublished}
								onclick={() => (isPublished = false)}
								disabled={isLoading}
							>
								Draft
							</button>
						</div>
					</div>
				</div>

				<div class="form-group">
					<label class="toggle-label">
						<input type="checkbox" bind:checked={dripEnabled} disabled={isLoading} />
						<span class="toggle-text">
							<strong>Content Dripping</strong>
							<span>Release this module after a delay</span>
						</span>
					</label>
				</div>

				{#if dripEnabled}
					<div class="form-group drip-config">
						<label for="dripDays" class="form-label">
							<IconClock size={16} />
							Days After Enrollment
						</label>
						<div class="drip-input-wrapper">
							<input
								id="dripDays"
								type="number"
								class="form-input drip-input"
								min="0"
								bind:value={dripDays}
								disabled={isLoading}
							/>
							<span class="drip-suffix">days</span>
						</div>
						<span class="form-hint">
							Students will unlock this module {dripDays === 0
								? 'immediately'
								: `${dripDays} day${dripDays > 1 ? 's' : ''} after enrolling`}
						</span>
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
							{mode === 'create' ? 'Adding...' : 'Saving...'}
						{:else}
							{mode === 'create' ? 'Add Module' : 'Save Changes'}
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MODULE FORM MODAL - ICT 7 Principal Engineer Grade
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
		max-width: 480px;
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

	.modal-form {
		padding: var(--space-6);
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

	.form-row {
		display: flex;
		gap: var(--space-4);
	}

	.form-group.half {
		flex: 1;
	}

	.form-label {
		display: flex;
		align-items: center;
		gap: var(--space-2);
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
	.form-textarea {
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

	.form-input:focus,
	.form-textarea:focus {
		outline: none;
		border-color: var(--admin-accent-primary);
		box-shadow: var(--admin-focus-ring);
	}

	.form-input::placeholder,
	.form-textarea::placeholder {
		color: var(--admin-text-muted);
	}

	.form-input:disabled,
	.form-textarea:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Status Toggle */
	.status-toggle {
		display: flex;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md);
		overflow: hidden;
	}

	.status-btn {
		flex: 1;
		padding: var(--space-3);
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		cursor: pointer;
		transition: var(--transition-all);
		min-height: 44px;
	}

	.status-btn:first-child {
		border-right: 1px solid var(--admin-border-subtle);
	}

	.status-btn.active {
		background: var(--admin-accent-bg);
		color: var(--admin-accent-primary);
	}

	.status-btn:hover:not(.active):not(:disabled) {
		background: var(--admin-surface-hover);
		color: var(--admin-text-secondary);
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

	/* Drip Config */
	.drip-config {
		margin-left: var(--space-9);
		padding-left: var(--space-4);
		border-left: 2px solid var(--admin-accent-primary);
		animation: fadeIn var(--duration-fast) var(--ease-default);
	}

	.drip-input-wrapper {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.drip-input {
		width: 100px;
	}

	.drip-suffix {
		color: var(--admin-text-muted);
		font-size: var(--text-sm);
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
	   RESPONSIVE - Mobile (< sm: 640px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: calc(var(--breakpoint-sm) - 1px)) {
		.modal-container {
			max-height: 95vh;
		}

		.modal-header {
			padding: var(--space-4);
		}

		.modal-form {
			padding: var(--space-4);
		}

		.form-row {
			flex-direction: column;
		}

		.modal-actions {
			flex-direction: column;
		}

		.drip-config {
			margin-left: 0;
			padding-left: var(--space-4);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.modal-backdrop,
		.modal-container,
		.drip-config {
			animation: none;
		}

		.btn-close,
		.btn-cancel,
		.btn-submit,
		.status-btn,
		.form-input,
		.form-textarea {
			transition: none;
		}
	}
</style>
