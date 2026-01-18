<!--
/**
 * Coupon Create Page - Clean & Functional
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Features:
 * - Clear $ vs % discount type toggle
 * - Prominent discount value input
 * - Usage limits (total and per-user)
 * - Include/Exclude product selection tabs
 * - Start/end dates
 * - Visible save button
 *
 * @version 2.0.0
 */
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import { slide } from 'svelte/transition';
	import {
		IconCheck,
		IconX,
		IconTag,
		IconRefresh,
		IconAlertCircle,
		IconSparkles,
		IconPercentage,
		IconCurrencyDollar
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// Types
	// ═══════════════════════════════════════════════════════════════════════════

	interface CouponFormData {
		code: string;
		description: string;
		discount_type: 'percentage' | 'fixed';
		discount_value: number;
		min_purchase: number | null;
		max_discount: number | null;
		usage_limit: number | null;
		usage_limit_per_user: number | null;
		starts_at: string;
		expires_at: string;
		is_active: boolean;
		applicable_products: number[];
		applicable_plans: number[];
	}

	interface Product {
		id: number;
		name: string;
		price: number;
	}

	interface MembershipPlan {
		id: number;
		name: string;
		price: number;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	let saving = $state(false);
	let generating = $state(false);
	let errors = $state<string[]>([]);
	let successMessage = $state<string | null>(null);
	let restrictionTab = $state<'include' | 'exclude'>('include');

	// Products and plans (will load from API)
	let availableProducts = $state<Product[]>([
		{ id: 1, name: 'Trading Course - Beginner', price: 197 },
		{ id: 2, name: 'Trading Course - Advanced', price: 497 },
		{ id: 3, name: 'Swing Trading Mastery', price: 997 },
		{ id: 4, name: 'Options Bootcamp', price: 1497 }
	]);

	let availablePlans = $state<MembershipPlan[]>([
		{ id: 1, name: 'Basic Membership', price: 47 },
		{ id: 2, name: 'Pro Membership', price: 97 },
		{ id: 3, name: 'Elite Membership', price: 197 },
		{ id: 4, name: 'Explosive Swings Room', price: 147 }
	]);

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
		applicable_plans: []
	});

	// Track selected items for include/exclude
	let selectedProducts = $state<Set<number>>(new Set());
	let selectedPlans = $state<Set<number>>(new Set());

	// ═══════════════════════════════════════════════════════════════════════════
	// Computed Values
	// ═══════════════════════════════════════════════════════════════════════════

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

	// ═══════════════════════════════════════════════════════════════════════════
	// Helpers
	// ═══════════════════════════════════════════════════════════════════════════

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

	function toggleProduct(productId: number): void {
		const newSet = new Set(selectedProducts);
		if (newSet.has(productId)) {
			newSet.delete(productId);
		} else {
			newSet.add(productId);
		}
		selectedProducts = newSet;
	}

	function togglePlan(planId: number): void {
		const newSet = new Set(selectedPlans);
		if (newSet.has(planId)) {
			newSet.delete(planId);
		} else {
			newSet.add(planId);
		}
		selectedPlans = newSet;
	}

	function selectAllProducts(): void {
		selectedProducts = new Set(availableProducts.map((p) => p.id));
	}

	function clearAllProducts(): void {
		selectedProducts = new Set();
	}

	function selectAllPlans(): void {
		selectedPlans = new Set(availablePlans.map((p) => p.id));
	}

	function clearAllPlans(): void {
		selectedPlans = new Set();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Validation
	// ═══════════════════════════════════════════════════════════════════════════

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

		return errs;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Submit
	// ═══════════════════════════════════════════════════════════════════════════

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
			// Build the payload for the API
			const payload = {
				code: formData.code.toUpperCase(),
				description: formData.description || null,
				discount_type: formData.discount_type,
				discount_value: formData.discount_value,
				min_purchase: formData.min_purchase || null,
				max_discount: formData.max_discount || null,
				usage_limit: formData.usage_limit || null,
				starts_at: formData.starts_at ? new Date(formData.starts_at).toISOString() : null,
				expires_at: formData.expires_at ? new Date(formData.expires_at).toISOString() : null,
				applicable_products:
					restrictionTab === 'include' && selectedProducts.size > 0
						? Array.from(selectedProducts)
						: null,
				applicable_plans:
					restrictionTab === 'include' && selectedPlans.size > 0
						? Array.from(selectedPlans)
						: null,
				is_active: formData.is_active
			};

			// Call the API
			const response = await fetch('/api/admin/coupons', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(payload)
			});

			const data = await response.json();

			if (!response.ok) {
				throw new Error(data.message || 'Failed to create coupon');
			}

			successMessage = `Coupon "${formData.code}" created successfully!`;

			// Redirect after short delay
			setTimeout(() => {
				goto('/admin/coupons');
			}, 1500);
		} catch (err) {
			console.error('Failed to create coupon:', err);
			errors = [err instanceof Error ? err.message : 'Failed to create coupon'];
		} finally {
			saving = false;
		}
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
		<!-- Header -->
		<header class="page-header">
			<div class="header-content">
				<button class="btn-back" onclick={() => goto('/admin/coupons')}>
					<IconX size={20} />
				</button>
				<div>
					<h1>Create New Coupon</h1>
					<p class="subtitle">Set up a discount code for your products and memberships</p>
				</div>
			</div>
		</header>

	<!-- Success Message -->
	{#if successMessage}
		<div class="alert alert-success" transition:slide={{ duration: 200 }}>
			<IconCheck size={20} />
			<span>{successMessage}</span>
		</div>
	{/if}

	<!-- Errors -->
	{#if errors.length > 0}
		<div class="alerts" transition:slide={{ duration: 200 }}>
			{#each errors as error}
				<div class="alert alert-error">
					<IconAlertCircle size={20} />
					<span>{error}</span>
				</div>
			{/each}
		</div>
	{/if}

	<form onsubmit={handleSubmit} class="coupon-form">
		<!-- SECTION: Basic Info -->
		<div class="form-section">
			<h2><IconTag size={20} /> Coupon Details</h2>

			<!-- Code -->
			<div class="form-group">
				<label for="code">Coupon Code *</label>
				<div class="input-with-button">
					<input
						type="text"
						id="code"
						bind:value={formData.code}
						placeholder="e.g., SUMMER20"
						class="input input-code"
						oninput={() => (formData.code = formData.code.toUpperCase())}
						required
					/>
					<button type="button" class="btn-generate" onclick={generateCode} disabled={generating}>
						<IconSparkles size={16} />
						{generating ? 'Generating...' : 'Generate'}
					</button>
				</div>
			</div>

			<!-- Description -->
			<div class="form-group">
				<label for="description">Description (optional)</label>
				<input
					type="text"
					id="description"
					bind:value={formData.description}
					placeholder="e.g., Summer sale discount"
					class="input"
				/>
			</div>
		</div>

		<!-- SECTION: Discount Configuration -->
		<div class="form-section">
			<h2>Discount Amount</h2>

			<!-- Discount Type Toggle -->
			<div class="form-group">
				<span id="discount-type-label" class="form-label">Discount Type *</span>
				<div class="discount-type-toggle" role="group" aria-labelledby="discount-type-label">
					<button
						type="button"
						class="type-btn"
						class:active={formData.discount_type === 'percentage'}
						onclick={() => (formData.discount_type = 'percentage')}
					>
						<IconPercentage size={20} />
						Percentage (%)
					</button>
					<button
						type="button"
						class="type-btn"
						class:active={formData.discount_type === 'fixed'}
						onclick={() => (formData.discount_type = 'fixed')}
					>
						<IconCurrencyDollar size={20} />
						Fixed Amount ($)
					</button>
				</div>
			</div>

			<!-- Discount Value -->
			<div class="form-group">
				<label for="discount_value">
					{formData.discount_type === 'percentage' ? 'Percentage Off *' : 'Dollar Amount Off *'}
				</label>
				<div class="value-input-wrapper">
					<span class="value-prefix">
						{formData.discount_type === 'percentage' ? '' : '$'}
					</span>
					<input
						type="number"
						id="discount_value"
						bind:value={formData.discount_value}
						min="0"
						max={formData.discount_type === 'percentage' ? 100 : undefined}
						step="0.01"
						class="input input-value"
						required
					/>
					<span class="value-suffix">
						{formData.discount_type === 'percentage' ? '%' : ''}
					</span>
				</div>
				{#if formData.discount_value > 0}
					<div class="discount-preview">
						Preview: <strong>{discountPreview}</strong>
					</div>
				{/if}
			</div>

			<!-- Min Purchase & Max Discount -->
			<div class="form-row">
				<div class="form-group">
					<label for="min_purchase">Minimum Purchase ($)</label>
					<input
						type="number"
						id="min_purchase"
						bind:value={formData.min_purchase}
						min="0"
						step="0.01"
						placeholder="No minimum"
						class="input"
					/>
					<span class="help-text">Customer must spend at least this amount</span>
				</div>

				{#if formData.discount_type === 'percentage'}
					<div class="form-group">
						<label for="max_discount">Maximum Discount ($)</label>
						<input
							type="number"
							id="max_discount"
							bind:value={formData.max_discount}
							min="0"
							step="0.01"
							placeholder="No cap"
							class="input"
						/>
						<span class="help-text">Cap the discount at this dollar amount</span>
					</div>
				{/if}
			</div>
		</div>

		<!-- SECTION: Usage Limits -->
		<div class="form-section">
			<h2>Usage Limits</h2>

			<div class="form-row">
				<div class="form-group">
					<label for="usage_limit">Total Uses</label>
					<input
						type="number"
						id="usage_limit"
						bind:value={formData.usage_limit}
						min="1"
						placeholder="Unlimited"
						class="input"
					/>
					<span class="help-text">How many times this coupon can be used in total</span>
				</div>

				<div class="form-group">
					<label for="usage_limit_per_user">Uses Per Customer</label>
					<input
						type="number"
						id="usage_limit_per_user"
						bind:value={formData.usage_limit_per_user}
						min="1"
						placeholder="Unlimited"
						class="input"
					/>
					<span class="help-text">How many times each customer can use this</span>
				</div>
			</div>
		</div>

		<!-- SECTION: Schedule -->
		<div class="form-section">
			<h2>Schedule</h2>

			<div class="form-row">
				<div class="form-group">
					<label for="starts_at">Start Date & Time</label>
					<input
						type="datetime-local"
						id="starts_at"
						bind:value={formData.starts_at}
						class="input"
					/>
				</div>

				<div class="form-group">
					<label for="expires_at">Expiration Date & Time</label>
					<input
						type="datetime-local"
						id="expires_at"
						bind:value={formData.expires_at}
						class="input"
					/>
				</div>
			</div>

			<div class="form-group">
				<label class="checkbox-label">
					<input type="checkbox" bind:checked={formData.is_active} />
					<span>Coupon is active immediately</span>
				</label>
			</div>
		</div>

		<!-- SECTION: Product/Plan Restrictions -->
		<div class="form-section">
			<h2>Product & Membership Restrictions</h2>
			<p class="section-description">
				Choose which products and memberships this coupon applies to. Leave empty to apply to all.
			</p>

			<!-- Include/Exclude Tabs -->
			<div class="restriction-tabs">
				<button
					type="button"
					class="restriction-tab"
					class:active={restrictionTab === 'include'}
					onclick={() => (restrictionTab = 'include')}
				>
					Include Only
				</button>
				<button
					type="button"
					class="restriction-tab"
					class:active={restrictionTab === 'exclude'}
					onclick={() => (restrictionTab = 'exclude')}
				>
					Exclude
				</button>
			</div>

			<div class="restriction-content">
				{#if restrictionTab === 'include'}
					<p class="tab-hint">Coupon will ONLY work for selected items below</p>
				{:else}
					<p class="tab-hint">Coupon will work for everything EXCEPT selected items below</p>
				{/if}

				<!-- Products -->
				<div class="restriction-group">
					<div class="restriction-header">
						<h3>Products</h3>
						<div class="restriction-actions">
							<button type="button" class="btn-link" onclick={selectAllProducts}>Select All</button
							>
							<button type="button" class="btn-link" onclick={clearAllProducts}>Clear</button>
						</div>
					</div>
					<div class="checkbox-grid">
						{#each availableProducts as product}
							<label class="item-checkbox">
								<input
									type="checkbox"
									checked={selectedProducts.has(product.id)}
									onchange={() => toggleProduct(product.id)}
								/>
								<span class="item-name">{product.name}</span>
								<span class="item-price">${product.price}</span>
							</label>
						{/each}
					</div>
				</div>

				<!-- Membership Plans -->
				<div class="restriction-group">
					<div class="restriction-header">
						<h3>Membership Plans</h3>
						<div class="restriction-actions">
							<button type="button" class="btn-link" onclick={selectAllPlans}>Select All</button>
							<button type="button" class="btn-link" onclick={clearAllPlans}>Clear</button>
						</div>
					</div>
					<div class="checkbox-grid">
						{#each availablePlans as plan}
							<label class="item-checkbox">
								<input
									type="checkbox"
									checked={selectedPlans.has(plan.id)}
									onchange={() => togglePlan(plan.id)}
								/>
								<span class="item-name">{plan.name}</span>
								<span class="item-price">${plan.price}/mo</span>
							</label>
						{/each}
					</div>
				</div>

				{#if !hasRestrictions}
					<div class="no-restrictions-notice">
						No items selected - coupon will apply to <strong>all products and plans</strong>
					</div>
				{/if}
			</div>
		</div>

		<!-- SAVE BUTTON - Always Visible -->
		<div class="form-actions">
			<button type="button" class="btn-cancel" onclick={() => goto('/admin/coupons')}>
				Cancel
			</button>
			<button type="submit" class="btn-save" disabled={saving}>
				{#if saving}
					<IconRefresh size={20} class="spinning" />
					Saving...
				{:else}
					<IconCheck size={20} />
					Save Coupon
				{/if}
			</button>
		</div>
	</form>
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	.admin-coupon-create {
		min-height: 100vh;
		background: linear-gradient(135deg, var(--bg-base) 0%, var(--bg-elevated) 50%, var(--bg-base) 100%);
		position: relative;
		overflow: hidden;
	}

	/* Header */
	.page-header {
		margin-bottom: 2rem;
	}

	.header-content {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.btn-back {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.btn-back:hover {
		background: rgba(100, 116, 139, 0.3);
		color: #cbd5e1;
	}

	.page-header h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	/* Alerts */
	.alerts {
		margin-bottom: 1.5rem;
	}

	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-radius: 8px;
		margin-bottom: 0.75rem;
		font-weight: 500;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.alert-success {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
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

	.form-section h2 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.25rem 0;
	}

	.section-description {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: -0.75rem 0 1.25rem 0;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.25rem;
	}

	label {
		display: block;
		font-weight: 600;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
		font-size: 0.9rem;
	}

	.input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.95rem;
		transition: all 0.2s;
	}

	.input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.input::placeholder {
		color: #64748b;
	}

	.input-code {
		font-family: 'SF Mono', 'Monaco', 'Inconsolata', monospace;
		font-size: 1.1rem;
		font-weight: 600;
		letter-spacing: 0.05em;
	}

	.input-with-button {
		display: flex;
		gap: 0.75rem;
	}

	.input-with-button .input {
		flex: 1;
	}

	.btn-generate {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		color: #cbd5e1;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.btn-generate:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
	}

	.btn-generate:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.help-text {
		display: block;
		margin-top: 0.375rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	/* Discount Type Toggle */
	.discount-type-toggle {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.type-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		font-weight: 600;
		font-size: 0.95rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-btn:hover {
		border-color: rgba(230, 184, 0, 0.3);
		color: #cbd5e1;
	}

	.type-btn.active {
		background: rgba(230, 184, 0, 0.1);
		border-color: #e6b800;
		color: #e6b800;
	}

	/* Value Input */
	.value-input-wrapper {
		display: flex;
		align-items: center;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		overflow: hidden;
	}

	.value-input-wrapper:focus-within {
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.value-prefix,
	.value-suffix {
		padding: 0.75rem 1rem;
		color: #64748b;
		font-weight: 600;
		font-size: 1.1rem;
		background: rgba(30, 41, 59, 0.5);
	}

	.value-prefix {
		border-right: 1px solid rgba(148, 163, 184, 0.2);
	}

	.value-suffix {
		border-left: 1px solid rgba(148, 163, 184, 0.2);
	}

	.input-value {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 1.25rem;
		font-weight: 600;
		text-align: center;
	}

	.input-value:focus {
		outline: none;
		box-shadow: none;
	}

	.discount-preview {
		margin-top: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #e6b800;
		font-size: 0.9rem;
		text-align: center;
	}

	.discount-preview strong {
		font-size: 1.1rem;
	}

	/* Checkbox */
	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		color: #cbd5e1;
		font-weight: 500;
	}

	.checkbox-label input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: #e6b800;
	}

	/* Restriction Tabs */
	.restriction-tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.restriction-tab {
		flex: 1;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.restriction-tab:hover {
		border-color: rgba(148, 163, 184, 0.4);
	}

	.restriction-tab.active {
		background: rgba(230, 184, 0, 0.1);
		border-color: #e6b800;
		color: #e6b800;
	}

	.restriction-content {
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 8px;
	}

	.tab-hint {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
		padding: 0.5rem;
		background: rgba(100, 116, 139, 0.1);
		border-radius: 6px;
		text-align: center;
	}

	.restriction-group {
		margin-bottom: 1.5rem;
	}

	.restriction-group:last-child {
		margin-bottom: 0;
	}

	.restriction-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.restriction-header h3 {
		font-size: 0.95rem;
		font-weight: 600;
		color: #e2e8f0;
		margin: 0;
	}

	.restriction-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-link {
		background: none;
		border: none;
		color: #60a5fa;
		font-size: 0.8rem;
		cursor: pointer;
		padding: 0;
	}

	.btn-link:hover {
		text-decoration: underline;
	}

	.checkbox-grid {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.item-checkbox {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.item-checkbox:hover {
		border-color: rgba(230, 184, 0, 0.3);
	}

	.item-checkbox input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: #e6b800;
		flex-shrink: 0;
	}

	.item-name {
		flex: 1;
		color: #e2e8f0;
		font-weight: 500;
	}

	.item-price {
		color: #64748b;
		font-size: 0.875rem;
	}

	.no-restrictions-notice {
		padding: 1rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.2);
		border-radius: 8px;
		color: #60a5fa;
		text-align: center;
		font-size: 0.9rem;
	}

	/* Form Actions - Always Visible Save Button */
	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding: 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		position: sticky;
		bottom: 1rem;
		backdrop-filter: blur(8px);
	}

	.btn-cancel {
		padding: 0.875rem 1.5rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		color: #cbd5e1;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-cancel:hover {
		background: rgba(100, 116, 139, 0.3);
	}

	.btn-save {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 2rem;
		background: linear-gradient(135deg, #e6b800, #b38f00);
		border: none;
		border-radius: 8px;
		color: #0d1117;
		font-weight: 700;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-save:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(230, 184, 0, 0.3);
	}

	.btn-save:disabled {
		opacity: 0.7;
		cursor: not-allowed;
		transform: none;
	}

	/* Animations */
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

	/* Responsive */
	@media (max-width: 640px) {
		.admin-coupon-create {
			padding: 1rem;
		}

		.discount-type-toggle {
			grid-template-columns: 1fr;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.input-with-button {
			flex-direction: column;
		}

		.form-actions {
			flex-direction: column;
		}

		.btn-save,
		.btn-cancel {
			width: 100%;
			justify-content: center;
		}
	}
</style>
