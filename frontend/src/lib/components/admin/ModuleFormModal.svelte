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

	let {
		isOpen,
		mode: modeProp,
		courseId,
		module = null,
		nextSortOrder = 1,
		onSave,
		onSaved,
		onClose
	}: Props = $props();

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
		border-radius: var(--radius-xl, 1rem);
		max-width: 480px;
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
		background: var(--admin-accent-bg);
		color: var(--admin-accent-primary);
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

	.modal-form {
		padding: 1.5rem;
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

	.form-row {
		display: flex;
		gap: 1rem;
	}

	.form-group.half {
		flex: 1;
	}

	.form-label {
		display: flex;
		align-items: center;
		gap: 0.375rem;
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
	.form-textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--admin-text-primary);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.9375rem;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.form-textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-input:focus,
	.form-textarea:focus {
		outline: none;
		border-color: var(--admin-accent-primary);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
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
		border-radius: var(--radius-md, 0.5rem);
		overflow: hidden;
	}

	.status-btn {
		flex: 1;
		padding: 0.625rem;
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
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

	/* Drip Config */
	.drip-config {
		margin-left: 2.25rem;
		padding-left: 1rem;
		border-left: 2px solid var(--admin-accent-primary);
		animation: fadeIn 0.2s ease;
	}

	.drip-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.drip-input {
		width: 100px;
	}

	.drip-suffix {
		color: var(--admin-text-muted);
		font-size: 0.875rem;
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
		background: var(--admin-accent-primary);
		border: none;
		color: #0d1117;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
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
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
