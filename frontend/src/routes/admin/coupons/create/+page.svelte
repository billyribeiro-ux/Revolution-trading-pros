<!--
/**
 * Coupon Create Page — R25-C refactor
 *
 * 2026-05-20: shell parent — wires data-loading, validation, and submission
 * to the form sections in ./_components/. Visual layout / DOM structure
 * preserved exactly; behavior is byte-for-byte identical.
 *
 * Children:
 *   - CouponPageHeader        — back button + title
 *   - CouponAlerts            — success + validation banners
 *   - CouponBasicFields       — code, type, value, min/max
 *   - CouponSubscriptionDuration — Stripe duration semantics
 *   - CouponLimitsAndDates    — usage limits, dates, is_active
 *   - CouponRestrictions      — product / plan include/exclude (discriminated)
 *   - CouponFormActions       — sticky save/cancel bar
 *   - types.ts                — CouponFormData / discount types
 *
 */
-->

<script lang="ts">
	import { SvelteSet } from 'svelte/reactivity';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		productsApi,
		subscriptionPlansApi,
		couponsApi,
		type Product,
		type SubscriptionPlan
	} from '$lib/api/admin';
	import CouponPageHeader from './_components/CouponPageHeader.svelte';
	import CouponAlerts from './_components/CouponAlerts.svelte';
	import CouponBasicFields from './_components/CouponBasicFields.svelte';
	import CouponSubscriptionDuration from './_components/CouponSubscriptionDuration.svelte';
	import CouponLimitsAndDates from './_components/CouponLimitsAndDates.svelte';
	import CouponRestrictions, {
		type CouponRestrictionAction
	} from './_components/CouponRestrictions.svelte';
	import CouponFormActions from './_components/CouponFormActions.svelte';
	import type { CouponFormData, RestrictionTab } from './_components/types';

	// State

	let saving = $state(false);
	let generating = $state(false);
	let loadingData = $state(true);
	let errors = $state<string[]>([]);
	let successMessage = $state<string | null>(null);
	let restrictionTab = $state<RestrictionTab>('include');

	// Products and plans - loaded from API
	let availableProducts = $state<Product[]>([]);
	let availablePlans = $state<SubscriptionPlan[]>([]);

	// Form data with defaults
	let formData = $state<CouponFormData>({
		code: '',
		description: '',
		discount_type: 'percentage',
		discount_value: 10,
		min_purchase: null,
		max_discount: null,
		usage_limit: null,
		usage_limit_per_user: 1,
		starts_at: formatDateForInput(new Date()),
		expires_at: formatDateForInput(addDays(new Date(), 30)),
		is_active: true,
		applicable_products: [],
		applicable_plans: [],
		duration: 'once',
		duration_in_months: null
	});

	// Track selected items for include/exclude
	let selectedProducts = $state<Set<number>>(new SvelteSet());
	let selectedPlans = $state<Set<number>>(new SvelteSet());

	// Computed Values

	let discountPreview = $derived.by(() => {
		if (!formData.discount_value) return '';
		if (formData.discount_type === 'percentage') {
			return `${formData.discount_value}% off`;
		}
		return `$${formData.discount_value} off`;
	});

	let hasRestrictions = $derived.by(() => {
		return selectedProducts.size > 0 || selectedPlans.size > 0;
	});

	// Helpers

	function formatDateForInput(date: Date): string {
		return date.toISOString().slice(0, 16);
	}

	function addDays(date: Date, days: number): Date {
		const result = new Date(date);
		result.setDate(result.getDate() + days);
		return result;
	}

	function generateCode(): void {
		generating = true;
		const chars = 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789';
		let code = '';
		for (let i = 0; i < 8; i++) {
			code += chars.charAt(Math.floor(Math.random() * chars.length));
		}
		formData.code = code;
		setTimeout(() => (generating = false), 300);
	}

	// Data Loading

	async function loadProductsAndPlans(): Promise<void> {
		loadingData = true;
		errors = [];
		try {
			// Load products and plans in parallel
			const [productsResponse, plansResponse] = await Promise.all([
				productsApi.list({ per_page: 100 }),
				subscriptionPlansApi.list()
			]);

			availableProducts = productsResponse.data || [];
			availablePlans = plansResponse.data || [];
		} catch (err) {
			console.error('Failed to load products and plans:', err);
			errors = ['Failed to load products and membership plans. Please refresh the page.'];
		} finally {
			loadingData = false;
		}
	}

	// Load data on mount
	onMount(() => {
		loadProductsAndPlans();
	});

	// Restriction action handler (R8-C discriminated callback)

	function handleRestrictionAction(action: CouponRestrictionAction): void {
		switch (action.kind) {
			case 'tab-change':
				restrictionTab = action.tab;
				return;
			case 'toggle-product': {
				// Set mutations are reactive - modify directly
				if (selectedProducts.has(action.productId)) selectedProducts.delete(action.productId);
				else selectedProducts.add(action.productId);
				return;
			}
			case 'toggle-plan': {
				// Set mutations are reactive - modify directly
				if (selectedPlans.has(action.planId)) selectedPlans.delete(action.planId);
				else selectedPlans.add(action.planId);
				return;
			}
			case 'select-all-products':
				selectedProducts = new SvelteSet(availableProducts.map((p) => p.id));
				return;
			case 'clear-products':
				selectedProducts.clear();
				return;
			case 'select-all-plans':
				selectedPlans = new SvelteSet(availablePlans.map((p) => p.id));
				return;
			case 'clear-plans':
				selectedPlans.clear();
				return;
		}
	}

	// Validation

	function validateForm(): string[] {
		const errs: string[] = [];

		if (!formData.code.trim()) {
			errs.push('Coupon code is required');
		} else if (!/^[A-Z0-9_-]+$/i.test(formData.code)) {
			errs.push('Code can only contain letters, numbers, dashes, and underscores');
		}

		if (!formData.discount_value || formData.discount_value <= 0) {
			errs.push('Discount value must be greater than 0');
		}

		if (formData.discount_type === 'percentage' && formData.discount_value > 100) {
			errs.push('Percentage discount cannot exceed 100%');
		}

		if (formData.starts_at && formData.expires_at) {
			const start = new Date(formData.starts_at);
			const end = new Date(formData.expires_at);
			if (end <= start) {
				errs.push('Expiration date must be after start date');
			}
		}

		if (formData.usage_limit_per_user && formData.usage_limit) {
			if (formData.usage_limit_per_user > formData.usage_limit) {
				errs.push('Per-user limit cannot exceed total usage limit');
			}
		}

		if (formData.duration === 'repeating') {
			if (!formData.duration_in_months || formData.duration_in_months < 1) {
				errs.push('Repeating coupons require a duration of at least 1 month');
			}
		} else if (formData.duration_in_months !== null) {
			errs.push('Duration in months only applies when duration is "repeating"');
		}

		return errs;
	}

	// Submit

	async function handleSubmit(e: Event): Promise<void> {
		e.preventDefault();
		errors = [];
		successMessage = null;

		// Validate
		const validationErrors = validateForm();
		if (validationErrors.length > 0) {
			errors = validationErrors;
			return;
		}

		saving = true;

		try {
			// FIX-2026-04-26 (P0-3, P1-7, CC-1): send canonical backend field
			// names (`discount_type`, `discount_value`, `min_purchase`,
			// `max_discount`, `starts_at`, `expires_at`, `applicable_plans`).
			// Previously every key was renamed away from what
			// `admin.rs::CreateCouponRequest` expects, so serde dropped them
			// and silently inserted a coupon with empty discount_type and
			// discount_value=0.
			//
			// Also wire `applicable_plans` (P1-7), which the UI collected in
			// `selectedPlans` but never sent.
			const payload = {
				code: formData.code.toUpperCase(),
				description: formData.description || undefined,
				discount_type: formData.discount_type, // 'fixed' or 'percentage'
				discount_value: formData.discount_value,
				min_purchase: formData.min_purchase ?? undefined,
				max_discount: formData.max_discount ?? undefined,
				usage_limit: formData.usage_limit ?? undefined,
				starts_at: formData.starts_at ? new Date(formData.starts_at).toISOString() : undefined,
				expires_at: formData.expires_at ? new Date(formData.expires_at).toISOString() : undefined,
				applicable_products:
					restrictionTab === 'include' && selectedProducts.size > 0
						? Array.from(selectedProducts)
						: undefined,
				applicable_plans:
					restrictionTab === 'include' && selectedPlans.size > 0
						? Array.from(selectedPlans)
						: undefined,
				is_active: formData.is_active,
				// Batch 3.5+: subscription duration semantics. Backend validates
				// that duration_in_months is set iff duration === 'repeating'.
				duration: formData.duration,
				duration_in_months:
					formData.duration === 'repeating' ? (formData.duration_in_months ?? undefined) : undefined
			};

			// Call the API using couponsApi
			await couponsApi.create(payload);

			successMessage = `Coupon "${formData.code}" created successfully!`;

			// Redirect after short delay
			setTimeout(() => {
				goto('/admin/coupons');
			}, 1500);
		} catch (err: unknown) {
			console.error('Failed to create coupon:', err);
			const e = err as { response?: { errors?: Record<string, string[]> }; message?: string };
			if (e.response?.errors) {
				// Validation errors from backend
				const validationErrors = Object.entries(e.response.errors)
					.map(([field, messages]) => `${field}: ${(messages as string[]).join(', ')}`)
					.join('; ');
				errors = [validationErrors];
			} else {
				errors = [e.message || 'Failed to create coupon. Please check your input and try again.'];
			}
		} finally {
			saving = false;
		}
	}

	function cancel(): void {
		goto('/admin/coupons');
	}
</script>

<svelte:head>
	<title>Create Coupon | Admin</title>
</svelte:head>

<div class="admin-coupon-create">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<CouponPageHeader onCancel={cancel} />

		<CouponAlerts {successMessage} {errors} />

		<form onsubmit={handleSubmit} class="coupon-form">
			<!-- SECTION: Basic Info + Duration + Limits/Dates -->
			<div class="form-section">
				<CouponBasicFields bind:formData {generating} {discountPreview} onGenerate={generateCode} />

				<CouponSubscriptionDuration bind:formData />

				<CouponLimitsAndDates bind:formData />
			</div>

			<!-- SECTION: Product/Plan Restrictions -->
			<CouponRestrictions
				{restrictionTab}
				{loadingData}
				{availableProducts}
				{availablePlans}
				{selectedProducts}
				{selectedPlans}
				{hasRestrictions}
				onAction={handleRestrictionAction}
			/>

			<!-- SAVE BUTTON - Always Visible -->
			<CouponFormActions {saving} onCancel={cancel} />
		</form>
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	.admin-coupon-create {
		background: linear-gradient(
			135deg,
			var(--bg-base) 0%,
			var(--bg-elevated) 50%,
			var(--bg-base) 100%
		);
		position: relative;
		overflow: hidden;
	}

	/* Form */
	.coupon-form {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.form-section {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
	}

	/* Responsive */
	@media (max-width: 639.98px) {
		.admin-coupon-create {
			padding: 1rem;
		}
	}
</style>
