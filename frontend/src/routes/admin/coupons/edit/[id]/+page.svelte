<!--
/**
 * Coupon Edit Component - Svelte 5 Enterprise Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * FEATURES:
 * - Full CRUD update functionality
 * - Svelte 5 runes ($state, $derived, $effect)
 * - Real-time validation
 * - Optimistic UI updates
 * - Complete API integration with couponsApi.update()
 *
 * @version 2.0.0 (Svelte 5 Nov/Dec 2025)
 * @component
 */
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { fade, slide } from 'svelte/transition';
	import { couponsApi, AdminApiError, type Coupon, type CouponUpdateData } from '$lib/api/admin';
	import {
		IconTicket,
		IconCheck,
		IconX,
		IconRefresh,
		IconAlertCircle,
		IconTrash
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// Type Definitions
	// ═══════════════════════════════════════════════════════════════════════════

	interface ValidationError {
		field: string;
		message: string;
		severity: 'error' | 'warning' | 'info';
	}

	interface CouponFormData {
		code: string;
		type:
			| 'fixed'
			| 'percentage'
			| 'bogo'
			| 'free_shipping'
			| 'tiered'
			| 'bundle'
			| 'cashback'
			| 'points';
		value: number;
		description: string;
		minimum_amount: number | null;
		max_discount_amount: number | null;
		usage_limit: number | null;
		valid_from: string;
		valid_until: string;
		is_active: boolean;
		stackable: boolean;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let couponId = $derived(parseInt(page.params.id ?? '0'));
	let loading = $state(true);
	let saving = $state(false);
	let deleting = $state(false);
	let errors = $state<ValidationError[]>([]);
	let originalCoupon = $state<Coupon | null>(null);

	// Form Data
	let formData = $state<CouponFormData>({
		code: '',
		type: 'percentage',
		value: 0,
		description: '',
		minimum_amount: null,
		max_discount_amount: null,
		usage_limit: null,
		valid_from: '',
		valid_until: '',
		is_active: true,
		stackable: false
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Derived State - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let hasChanges = $derived.by(() => {
		if (!originalCoupon) return false;
		return (
			formData.code !== originalCoupon.code ||
			formData.type !== originalCoupon.type ||
			formData.value !== originalCoupon.value ||
			formData.is_active !== originalCoupon.is_active ||
			formData.stackable !== (originalCoupon.stackable || false) ||
			formData.minimum_amount !== (originalCoupon.minimum_amount || null) ||
			formData.usage_limit !== (originalCoupon.usage_limit || null)
		);
	});

	let discountDisplay = $derived(
		formData.type === 'percentage'
			? `${formData.value}% off`
			: formData.type === 'fixed'
				? `$${formData.value} off`
				: formData.type === 'free_shipping'
					? 'Free Shipping'
					: 'Custom Discount'
	);

	let hasErrors = $derived(errors.some((e) => e.severity === 'error'));

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (couponId) {
			loadCoupon();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadCoupon() {
		loading = true;
		errors = [];

		try {
			const response = await couponsApi.get(couponId);
			const coupon = response.data;
			originalCoupon = coupon;

			// Populate form data
			formData = {
				code: coupon.code,
				type: coupon.type,
				value: coupon.value,
				description: '',
				minimum_amount: coupon.minimum_amount || null,
				max_discount_amount: null,
				usage_limit: coupon.usage_limit || null,
				valid_from: coupon.valid_from ? formatDateTimeLocal(new Date(coupon.valid_from)) : '',
				valid_until: coupon.valid_until ? formatDateTimeLocal(new Date(coupon.valid_until)) : '',
				is_active: coupon.is_active,
				stackable: coupon.stackable || false
			};
		} catch (err) {
			if (err instanceof AdminApiError) {
				if (err.status === 401) {
					goto('/login');
					return;
				} else if (err.status === 404) {
					errors = [{ field: 'general', message: 'Coupon not found', severity: 'error' }];
				} else if (err.status === 403) {
					errors = [
						{
							field: 'general',
							message: 'You are not authorized to edit this coupon',
							severity: 'error'
						}
					];
				} else {
					errors = [{ field: 'general', message: err.message, severity: 'error' }];
				}
			} else {
				errors = [{ field: 'general', message: 'Failed to load coupon', severity: 'error' }];
			}
			console.error('Failed to load coupon:', err);
		} finally {
			loading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Form Submission
	// ═══════════════════════════════════════════════════════════════════════════

	async function handleSubmit() {
		// Clear previous errors
		errors = [];

		// Validate form
		const validationErrors = validateForm();
		if (validationErrors.length > 0) {
			errors = validationErrors;
			const firstError = validationErrors.find((e) => e.severity === 'error');
			if (firstError) {
				scrollToError(firstError.field);
				return;
			}
		}

		saving = true;
		try {
			// Prepare update data
			const updateData: CouponUpdateData = {
				code: formData.code,
				type: formData.type,
				value: formData.value,
				minimum_amount: formData.minimum_amount || undefined,
				max_discount_amount: formData.max_discount_amount || undefined,
				usage_limit: formData.usage_limit || undefined,
				valid_from: formData.valid_from ? new Date(formData.valid_from).toISOString() : undefined,
				valid_until: formData.valid_until
					? new Date(formData.valid_until).toISOString()
					: undefined,
				is_active: formData.is_active,
				stackable: formData.stackable
			};

			// Update coupon
			await couponsApi.update(couponId, updateData);

			// Success - redirect to list
			goto('/admin/coupons');
		} catch (err) {
			if (err instanceof AdminApiError) {
				handleApiError(err);
			} else {
				errors = [{ field: 'general', message: 'Failed to update coupon', severity: 'error' }];
			}
			console.error('Failed to update coupon:', err);
		} finally {
			saving = false;
		}
	}

	async function handleDelete() {
		if (!confirm('Are you sure you want to delete this coupon? This action cannot be undone.')) {
			return;
		}

		deleting = true;
		try {
			await couponsApi.delete(couponId);
			goto('/admin/coupons');
		} catch (err) {
			if (err instanceof AdminApiError) {
				errors = [
					{ field: 'general', message: `Failed to delete: ${err.message}`, severity: 'error' }
				];
			} else {
				errors = [{ field: 'general', message: 'Failed to delete coupon', severity: 'error' }];
			}
			console.error('Failed to delete coupon:', err);
		} finally {
			deleting = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Validation
	// ═══════════════════════════════════════════════════════════════════════════

	function validateForm(): ValidationError[] {
		const errors: ValidationError[] = [];

		// Code validation
		if (!formData.code.trim()) {
			errors.push({ field: 'code', message: 'Coupon code is required', severity: 'error' });
		} else if (!/^[A-Z0-9_-]+$/i.test(formData.code)) {
			errors.push({
				field: 'code',
				message: 'Code can only contain letters, numbers, dashes and underscores',
				severity: 'error'
			});
		}

		// Value validation
		if (formData.value <= 0 && formData.type !== 'free_shipping') {
			errors.push({ field: 'value', message: 'Value must be greater than 0', severity: 'error' });
		}

		if (formData.type === 'percentage' && formData.value > 100) {
			errors.push({ field: 'value', message: 'Percentage cannot exceed 100%', severity: 'error' });
		}

		// Date validation
		if (formData.valid_from && formData.valid_until) {
			const start = new Date(formData.valid_from);
			const end = new Date(formData.valid_until);

			if (end <= start) {
				errors.push({
					field: 'valid_until',
					message: 'Expiration must be after start date',
					severity: 'error'
				});
			}
		}

		// Usage limit validation
		if (formData.usage_limit !== null && formData.usage_limit <= 0) {
			errors.push({
				field: 'usage_limit',
				message: 'Usage limit must be positive',
				severity: 'error'
			});
		}

		// Minimum amount validation
		if (formData.minimum_amount !== null && formData.minimum_amount < 0) {
			errors.push({
				field: 'minimum_amount',
				message: 'Minimum amount cannot be negative',
				severity: 'error'
			});
		}

		return errors;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helper Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDateTimeLocal(date: Date): string {
		const year = date.getFullYear();
		const month = String(date.getMonth() + 1).padStart(2, '0');
		const day = String(date.getDate()).padStart(2, '0');
		const hours = String(date.getHours()).padStart(2, '0');
		const minutes = String(date.getMinutes()).padStart(2, '0');

		return `${year}-${month}-${day}T${hours}:${minutes}`;
	}

	function scrollToError(field: string) {
		const element = document.querySelector(`[data-field="${field}"]`);
		element?.scrollIntoView({ behavior: 'smooth', block: 'center' });
	}

	function handleApiError(error: AdminApiError) {
		if (error.isValidationError && error.validationErrors) {
			errors = Object.entries(error.validationErrors).flatMap(([field, messages]) =>
				messages.map((message) => ({
					field,
					message,
					severity: 'error' as const
				}))
			);
		} else {
			errors = [{ field: 'general', message: error.message, severity: 'error' }];
		}
	}

	function getFieldError(field: string): string | null {
		const error = errors.find((e) => e.field === field);
		return error ? error.message : null;
	}
</script>

<svelte:head>
	<title>Edit Coupon | Admin</title>
</svelte:head>

<div class="admin-page">
	<!-- Header -->
	<div class="page-header">
		<div class="header-content">
			<div class="header-main">
				<h1>
					<IconTicket size={32} />
					Edit Coupon
				</h1>
				{#if originalCoupon}
					<p class="header-subtitle">Editing: <strong>{originalCoupon.code}</strong></p>
				{/if}
			</div>

			{#if !loading && formData.code}
				<div class="preview-badge">
					<span class="preview-value">{discountDisplay}</span>
					<span class="preview-status" class:active={formData.is_active}>
						{formData.is_active ? 'Active' : 'Inactive'}
					</span>
				</div>
			{/if}
		</div>

		<div class="header-actions">
			<button class="btn-ghost" onclick={() => goto('/admin/coupons')}>
				<IconX size={18} />
				Cancel
			</button>
			<button class="btn-danger" onclick={handleDelete} disabled={deleting || loading}>
				{#if deleting}
					<IconRefresh size={18} class="spinning" />
				{:else}
					<IconTrash size={18} />
				{/if}
				Delete
			</button>
			<button
				class="btn-primary"
				onclick={handleSubmit}
				disabled={saving || loading || !hasChanges}
			>
				{#if saving}
					<IconRefresh size={18} class="spinning" />
					Saving...
				{:else}
					<IconCheck size={18} />
					Save Changes
				{/if}
			</button>
		</div>
	</div>

	<!-- Loading State -->
	{#if loading}
		<div class="loading-state">
			<IconRefresh size={32} class="spinning" />
			<p>Loading coupon...</p>
		</div>
	{:else}
		<!-- Validation Errors -->
		{#if errors.length > 0}
			<div class="alerts" transition:slide={{ duration: 300 }}>
				{#each errors as error}
					<div class="alert alert-{error.severity}">
						<IconAlertCircle size={20} />
						<span>{error.message}</span>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Form -->
		<form onsubmit={handleSubmit} class="coupon-form">
			<!-- Basic Information -->
			<div class="form-section">
				<h2>Basic Information</h2>

				<div class="form-group" data-field="code">
					<label for="code">
						Coupon Code *
						<span class="label-hint">Unique identifier</span>
					</label>
					<input
						type="text"
						id="code"
						bind:value={formData.code}
						placeholder="SUMMER2024"
						class="input input-large"
						class:error={getFieldError('code')}
						required
						oninput={() => (formData.code = formData.code.toUpperCase())}
					/>
					{#if getFieldError('code')}
						<span class="field-error">{getFieldError('code')}</span>
					{/if}
				</div>

				<div class="form-row">
					<div class="form-group" data-field="type">
						<label for="type">Discount Type *</label>
						<select id="type" bind:value={formData.type} class="input">
							<option value="percentage">Percentage Off</option>
							<option value="fixed">Fixed Amount Off</option>
							<option value="free_shipping">Free Shipping</option>
						</select>
					</div>

					<div class="form-group" data-field="value">
						<label for="value">
							{formData.type === 'percentage' ? 'Percentage' : 'Amount'} *
						</label>
						<div class="input-with-suffix">
							<input
								type="number"
								id="value"
								bind:value={formData.value}
								min="0"
								max={formData.type === 'percentage' ? 100 : undefined}
								step="0.01"
								class="input"
								class:error={getFieldError('value')}
								disabled={formData.type === 'free_shipping'}
							/>
							<span class="input-suffix">
								{formData.type === 'percentage' ? '%' : '$'}
							</span>
						</div>
						{#if getFieldError('value')}
							<span class="field-error">{getFieldError('value')}</span>
						{/if}
					</div>
				</div>
			</div>

			<!-- Usage Limits -->
			<div class="form-section">
				<h2>Usage Limits</h2>

				<div class="form-row">
					<div class="form-group" data-field="minimum_amount">
						<label for="minimum_amount">Minimum Purchase ($)</label>
						<input
							type="number"
							id="minimum_amount"
							bind:value={formData.minimum_amount}
							min="0"
							step="0.01"
							class="input"
							placeholder="No minimum"
						/>
					</div>

					<div class="form-group" data-field="usage_limit">
						<label for="usage_limit">Total Uses</label>
						<input
							type="number"
							id="usage_limit"
							bind:value={formData.usage_limit}
							min="1"
							class="input"
							placeholder="Unlimited"
						/>
						{#if originalCoupon?.usage_count}
							<p class="help-text">Current usage: {originalCoupon.usage_count}</p>
						{/if}
					</div>
				</div>
			</div>

			<!-- Schedule -->
			<div class="form-section">
				<h2>Schedule</h2>

				<div class="form-row">
					<div class="form-group" data-field="valid_from">
						<label for="valid_from">Start Date</label>
						<input
							type="datetime-local"
							id="valid_from"
							bind:value={formData.valid_from}
							class="input"
						/>
					</div>

					<div class="form-group" data-field="valid_until">
						<label for="valid_until">Expiration Date</label>
						<input
							type="datetime-local"
							id="valid_until"
							bind:value={formData.valid_until}
							class="input"
							class:error={getFieldError('valid_until')}
						/>
						{#if getFieldError('valid_until')}
							<span class="field-error">{getFieldError('valid_until')}</span>
						{/if}
					</div>
				</div>
			</div>

			<!-- Settings -->
			<div class="form-section">
				<h2>Settings</h2>

				<div class="form-checkboxes">
					<label class="toggle-label">
						<input id="page-formdata-is-active" name="page-formdata-is-active" type="checkbox" bind:checked={formData.is_active} class="toggle-checkbox" />
						<span class="toggle-switch"></span>
						<span class="toggle-text">Active</span>
						<span class="toggle-hint">Coupon can be used by customers</span>
					</label>

					<label class="toggle-label">
						<input id="page-formdata-stackable" name="page-formdata-stackable" type="checkbox" bind:checked={formData.stackable} class="toggle-checkbox" />
						<span class="toggle-switch"></span>
						<span class="toggle-text">Stackable</span>
						<span class="toggle-hint">Can be combined with other coupons</span>
					</label>
				</div>
			</div>

			<!-- Metadata -->
			{#if originalCoupon}
				<div class="form-section metadata-section">
					<h2>Metadata</h2>
					<div class="metadata-grid">
						<div class="metadata-item">
							<span class="metadata-label">Created</span>
							<span class="metadata-value">
								{new Date(originalCoupon.created_at).toLocaleDateString('en-US', {
									dateStyle: 'medium'
								})}
							</span>
						</div>
						<div class="metadata-item">
							<span class="metadata-label">Last Updated</span>
							<span class="metadata-value">
								{new Date(originalCoupon.updated_at).toLocaleDateString('en-US', {
									dateStyle: 'medium'
								})}
							</span>
						</div>
						<div class="metadata-item">
							<span class="metadata-label">Total Uses</span>
							<span class="metadata-value">{originalCoupon.usage_count}</span>
						</div>
						<div class="metadata-item">
							<span class="metadata-label">ID</span>
							<span class="metadata-value">#{originalCoupon.id}</span>
						</div>
					</div>
				</div>
			{/if}
		</form>
	{/if}
</div>

<style>
	.admin-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
		background: #0f172a;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.header-content {
		flex: 1;
	}

	.header-main h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.header-subtitle {
		color: #94a3b8;
		font-size: 0.95rem;
		margin: 0;
	}

	.header-subtitle strong {
		color: #60a5fa;
		font-family: monospace;
	}

	.preview-badge {
		display: inline-flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1.25rem;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		margin-top: 1rem;
	}

	.preview-value {
		font-size: 1.25rem;
		font-weight: 600;
		color: #3b82f6;
	}

	.preview-status {
		padding: 0.25rem 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-radius: 6px;
		font-size: 0.8rem;
		font-weight: 600;
	}

	.preview-status.active {
		background: rgba(16, 185, 129, 0.1);
		color: #34d399;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Buttons */
	.btn-primary,
	.btn-ghost,
	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.btn-ghost {
		background: transparent;
		color: #94a3b8;
	}

	.btn-ghost:hover {
		background: rgba(148, 163, 184, 0.1);
	}

	.btn-danger {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.btn-danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.2);
	}

	button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Alerts */
	.alerts {
		margin-bottom: 1.5rem;
	}

	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-radius: 8px;
		margin-bottom: 0.75rem;
		font-weight: 500;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.alert-warning {
		background: rgba(245, 158, 11, 0.1);
		border: 1px solid rgba(245, 158, 11, 0.3);
		color: #fbbf24;
	}

	/* Loading */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #94a3b8;
	}

	.loading-state p {
		margin-top: 1rem;
	}

	/* Form */
	.coupon-form {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.form-section {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 2rem;
	}

	.form-section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.5rem 0;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.5rem;
	}

	.form-row .form-group {
		margin-bottom: 0;
	}

	label {
		display: block;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.label-hint {
		font-weight: 400;
		color: #94a3b8;
		font-size: 0.875rem;
		margin-left: 0.5rem;
	}

	.input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.95rem;
		transition: all 0.2s;
	}

	.input:focus {
		outline: none;
		border-color: rgba(59, 130, 246, 0.5);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.input.error {
		border-color: rgba(239, 68, 68, 0.5);
	}

	.input-large {
		font-size: 1.25rem;
		font-weight: 600;
		letter-spacing: 0.05em;
	}

	.input-with-suffix {
		position: relative;
	}

	.input-with-suffix .input {
		padding-right: 3rem;
	}

	.input-suffix {
		position: absolute;
		right: 1rem;
		top: 50%;
		transform: translateY(-50%);
		color: #64748b;
		font-weight: 600;
	}

	.field-error {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #f87171;
	}

	.help-text {
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	/* Toggle */
	.form-checkboxes {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 1rem;
		cursor: pointer;
		margin-bottom: 0;
	}

	.toggle-checkbox {
		display: none;
	}

	.toggle-switch {
		position: relative;
		width: 52px;
		height: 28px;
		background: rgba(148, 163, 184, 0.2);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 28px;
		transition: 0.3s;
		flex-shrink: 0;
	}

	.toggle-switch:before {
		content: '';
		position: absolute;
		height: 20px;
		width: 20px;
		left: 3px;
		bottom: 3px;
		background: #94a3b8;
		border-radius: 50%;
		transition: 0.3s;
	}

	.toggle-checkbox:checked + .toggle-switch {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		border-color: transparent;
	}

	.toggle-checkbox:checked + .toggle-switch:before {
		transform: translateX(24px);
		background: white;
	}

	.toggle-text {
		color: #f1f5f9;
		font-weight: 600;
	}

	.toggle-hint {
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 400;
	}

	/* Metadata */
	.metadata-section {
		background: rgba(15, 23, 42, 0.6);
	}

	.metadata-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1.5rem;
	}

	.metadata-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.metadata-label {
		font-size: 0.8rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.metadata-value {
		font-size: 1rem;
		color: #cbd5e1;
		font-weight: 500;
	}

	/* Animations */
	.spinning {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* Responsive */
	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			justify-content: flex-end;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.metadata-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
