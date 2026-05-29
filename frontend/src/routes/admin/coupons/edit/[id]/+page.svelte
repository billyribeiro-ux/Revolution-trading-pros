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
 * R23-C extraction (2026-05-20): the form is now composed from 7 leaf
 * components under `_components/`. Parent retains state, data loading,
 * submit/delete handlers, validation, and a thin page shell.
 *
 * @version 2.0.0 (Svelte 5 Nov/Dec 2025)
 * @component
 */
-->

<script lang="ts">
	import { untrack } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { couponsApi, AdminApiError, type Coupon, type CouponUpdateData } from '$lib/api/admin';
	import { IconRefresh } from '$lib/icons';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import CouponPageHeader from './_components/CouponPageHeader.svelte';
	import CouponValidationAlerts from './_components/CouponValidationAlerts.svelte';
	import CouponBasicFields from './_components/CouponBasicFields.svelte';
	import CouponUsageLimits from './_components/CouponUsageLimits.svelte';
	import CouponSchedule from './_components/CouponSchedule.svelte';
	import CouponSettings from './_components/CouponSettings.svelte';
	import CouponMetadata from './_components/CouponMetadata.svelte';
	import type { CouponFormData, ValidationError } from './_components/types';

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
		stackable: false,
		duration: 'once',
		duration_in_months: null
	});

	// Delete confirmation modal state
	let showDeleteModal = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// Derived State - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	// FIX-2026-04-26 (P2-11, CC-1): compare against canonical backend fields
	// with legacy fallback so the dirty-state check works for both shapes.
	let hasChanges = $derived.by(() => {
		if (!originalCoupon) return false;
		const origType = originalCoupon.discount_type ?? originalCoupon.type;
		const origValue = originalCoupon.discount_value ?? originalCoupon.value ?? 0;
		const origMin = originalCoupon.min_purchase ?? originalCoupon.minimum_amount ?? null;
		return (
			formData.code !== originalCoupon.code ||
			formData.type !== origType ||
			formData.value !== origValue ||
			formData.is_active !== originalCoupon.is_active ||
			formData.stackable !== (originalCoupon.stackable || false) ||
			formData.minimum_amount !== origMin ||
			formData.usage_limit !== (originalCoupon.usage_limit ?? null)
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

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	// FIX: wrap state-mutating data loader in `untrack` so the effect
	// only re-fires when `couponId` actually changes (matches the
	// precedent in admin/products/[id]/edit).
	$effect(() => {
		const id = couponId;
		if (id) {
			untrack(() => {
				loadCoupon();
			});
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

			// FIX-2026-04-26 (P2-11, CC-1): backend returns canonical
			// discount_type/discount_value/min_purchase/starts_at/expires_at.
			// Read those first; fall back to legacy aliases for old payloads.
			const dtype = (coupon.discount_type ?? coupon.type ?? 'percentage') as CouponFormData['type'];
			const dvalue = coupon.discount_value ?? coupon.value ?? 0;
			const minPurchase = coupon.min_purchase ?? coupon.minimum_amount ?? null;
			const maxDiscount = coupon.max_discount ?? coupon.max_discount_amount ?? null;
			const startsAt = coupon.starts_at ?? coupon.valid_from ?? '';
			const expiresAt = coupon.expires_at ?? coupon.valid_until ?? '';

			// Populate form data
			formData = {
				code: coupon.code,
				type: dtype,
				value: dvalue,
				description: coupon.description ?? '',
				minimum_amount: minPurchase,
				max_discount_amount: maxDiscount,
				usage_limit: coupon.usage_limit ?? null,
				valid_from: startsAt ? formatDateTimeLocal(new Date(startsAt)) : '',
				valid_until: expiresAt ? formatDateTimeLocal(new Date(expiresAt)) : '',
				is_active: coupon.is_active,
				stackable: coupon.stackable || false,
				duration: coupon.duration ?? 'once',
				duration_in_months: coupon.duration_in_months ?? null
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
			// FIX-2026-04-26 (P0-3, P2-11, CC-1): send canonical field names to
			// the backend (admin.rs::CreateCouponRequest). The legacy aliases
			// (`type`/`value`/`minimum_amount`/...) are silently dropped by
			// serde, so the previous payload corrupted every update.
			// `couponsApi.update` also normalizes these via
			// `normalizeCouponPayload`, but we send canonical to make the wire
			// shape obvious here.
			//
			// Edit page only models 'percentage' / 'fixed'; the type union is
			// wider for legacy reasons (P3-5) so coerce defensively.
			const dtype: 'percentage' | 'fixed' = formData.type === 'percentage' ? 'percentage' : 'fixed';

			const updateData: CouponUpdateData = {
				code: formData.code.trim().toUpperCase(),
				discount_type: dtype,
				discount_value: formData.value,
				description: formData.description || undefined,
				min_purchase: formData.minimum_amount ?? undefined,
				max_discount: formData.max_discount_amount ?? undefined,
				usage_limit: formData.usage_limit ?? undefined,
				starts_at: formData.valid_from ? new Date(formData.valid_from).toISOString() : undefined,
				expires_at: formData.valid_until ? new Date(formData.valid_until).toISOString() : undefined,
				is_active: formData.is_active,
				// Batch 3.5+: subscription duration. Backend recreates the Stripe
				// Coupon when discount math (type/value/duration) changes.
				duration: formData.duration,
				duration_in_months:
					formData.duration === 'repeating' ? (formData.duration_in_months ?? undefined) : undefined
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

	function handleDelete() {
		showDeleteModal = true;
	}

	async function confirmDelete() {
		showDeleteModal = false;
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

		// Duration validation (Batch 3.5+ Stripe coupon semantics).
		if (formData.duration === 'repeating') {
			if (!formData.duration_in_months || formData.duration_in_months < 1) {
				errors.push({
					field: 'duration_in_months',
					message: 'Repeating coupons require a duration of at least 1 month',
					severity: 'error'
				});
			}
		} else if (formData.duration_in_months !== null) {
			errors.push({
				field: 'duration_in_months',
				message: 'Duration in months only applies when duration is "repeating"',
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
	<CouponPageHeader
		{originalCoupon}
		isActive={formData.is_active}
		{discountDisplay}
		hasCode={!!formData.code}
		{loading}
		{saving}
		{deleting}
		{hasChanges}
		onCancel={() => goto('/admin/coupons')}
		onDelete={handleDelete}
		onSave={handleSubmit}
	/>

	{#if loading}
		<div class="loading-state">
			<IconRefresh size={32} class="spinning" />
			<p>Loading coupon...</p>
		</div>
	{:else}
		<CouponValidationAlerts {errors} />

		<form onsubmit={handleSubmit} class="coupon-form">
			<CouponBasicFields bind:formData {getFieldError} />
			<CouponUsageLimits
				bind:formData
				usageCount={originalCoupon?.usage_count ?? null}
				{getFieldError}
			/>
			<CouponSchedule bind:formData {getFieldError} />
			<CouponSettings bind:formData />
			{#if originalCoupon}
				<CouponMetadata coupon={originalCoupon} />
			{/if}
		</form>
	{/if}
</div>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Coupon"
	message="Are you sure you want to delete this coupon? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDelete}
	onCancel={() => {
		showDeleteModal = false;
	}}
/>

<style>
	.admin-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
		background: #0f172a;
	}

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

	.coupon-form {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	:global(.spinning) {
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
</style>
