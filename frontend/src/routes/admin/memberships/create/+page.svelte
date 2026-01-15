<!--
	/admin/memberships/create - Create New Membership Plan
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Real-time plan preview
	- Feature list management
	- Pricing and billing configuration
	- Slug auto-generation
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	/**
	 * Create Membership Plan - Apple ICT 7 Principal Engineer Grade
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Premium membership plan creation with:
	 * - Live preview card
	 * - Dynamic feature management
	 * - Slug auto-generation
	 * - Billing cycle configuration
	 * - Form validation
	 *
	 * @version 1.0.0 (January 2026)
	 */

	import { goto } from '$app/navigation';
	import {
		IconCrown,
		IconPlus,
		IconX,
		IconCheck,
		IconArrowLeft,
		IconTarget
	} from '$lib/icons';
	import { adminFetch } from '$lib/utils/adminFetch';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface MembershipFeature {
		feature_code: string;
		feature_name: string;
		description: string;
	}

	interface MembershipFormData {
		name: string;
		slug: string;
		description: string;
		price: string;
		billing_cycle: 'monthly' | 'quarterly' | 'annual';
		is_active: boolean;
		features: MembershipFeature[];
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let membership = $state<MembershipFormData>({
		name: '',
		slug: '',
		description: '',
		price: '',
		billing_cycle: 'monthly',
		is_active: true,
		features: [{ feature_code: 'feature_1', feature_name: '', description: '' }]
	});

	let saving = $state(false);
	let error = $state('');
	let slugManuallyEdited = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Svelte 5 $derived
	// ═══════════════════════════════════════════════════════════════════════════

	let validFeatures = $derived(
		membership.features.filter((f) => f.feature_name.trim())
	);

	let isFormValid = $derived(
		membership.name.trim() !== '' &&
		membership.price !== '' &&
		parseFloat(membership.price) >= 0
	);

	let formattedPrice = $derived(
		membership.price
			? new Intl.NumberFormat('en-US', {
					style: 'currency',
					currency: 'USD',
					minimumFractionDigits: 0
				}).format(parseFloat(membership.price) || 0)
			: '$0'
	);

	let billingLabel = $derived(
		{
			monthly: '/month',
			quarterly: '/quarter',
			annual: '/year'
		}[membership.billing_cycle] || '/month'
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		// Auto-generate slug from name if not manually edited
		if (!slugManuallyEdited && membership.name) {
			membership.slug = membership.name
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleSlugInput() {
		slugManuallyEdited = true;
	}

	function addFeature() {
		membership.features = [
			...membership.features,
			{
				feature_code: `feature_${membership.features.length + 1}`,
				feature_name: '',
				description: ''
			}
		];
	}

	function removeFeature(index: number) {
		membership.features = membership.features.filter((_, i) => i !== index);
	}

	async function saveMembership() {
		if (!isFormValid) {
			error = 'Please fill in all required fields';
			return;
		}

		saving = true;
		error = '';

		try {
			await adminFetch('/api/admin/membership-plans', {
				method: 'POST',
				body: JSON.stringify({
					...membership,
					price: parseFloat(membership.price) || 0,
					features: validFeatures
				})
			});
			goto('/admin/memberships');
		} catch (err) {
			console.error('Failed to save membership:', err);
			error = err instanceof Error ? err.message : 'Failed to save membership plan';
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head>
	<title>Create Membership Plan | Admin Dashboard</title>
</svelte:head>

<div class="create-page">
	<!-- Apple ICT7 Grade Header -->
	<header class="grade-header">
		<div class="grade-header-content">
			<div class="grade-badge">
				<IconTarget size={16} />
				<span>ICT 7</span>
			</div>
			<div class="header-row">
				<button class="btn-back" onclick={() => goto('/admin/memberships')}>
					<IconArrowLeft size={20} />
				</button>
				<div>
					<h1>
						<IconCrown size={28} />
						Create Membership Plan
					</h1>
					<p class="header-subtitle">
						Design a premium membership tier for your subscribers
					</p>
				</div>
			</div>
		</div>
	</header>

	{#if error}
		<div class="error-banner">
			<IconX size={18} />
			<span>{error}</span>
			<button class="error-dismiss" onclick={() => (error = '')}>
				<IconX size={16} />
			</button>
		</div>
	{/if}

	<div class="content-container">
		<!-- Preview Card -->
		<div class="preview-section">
			<h3 class="section-title">Live Preview</h3>
			<div class="membership-card">
				<div class="card-crown">
					<IconCrown size={32} />
				</div>
				<div class="card-header">
					<h4>{membership.name || 'Plan Name'}</h4>
					<div class="pricing">
						<span class="amount">{formattedPrice}</span>
						<span class="period">{billingLabel}</span>
					</div>
				</div>
				<div class="card-description">
					<p>{membership.description || 'Add a description...'}</p>
				</div>
				<div class="features-list">
					{#each validFeatures as feature}
						<div class="feature-item">
							<IconCheck size={18} />
							<span>{feature.feature_name}</span>
						</div>
					{/each}
					{#if validFeatures.length === 0}
						<div class="empty-features">Add features below</div>
					{/if}
				</div>
				<button class="subscribe-btn" disabled> Subscribe Now </button>
			</div>
		</div>

		<!-- Form Section -->
		<div class="form-section">
			<div class="form-card">
				<h3 class="section-title">Membership Details</h3>

				<div class="form-group">
					<label for="name">Plan Name *</label>
					<input
						id="name"
						type="text"
						bind:value={membership.name}
						placeholder="e.g., Pro Membership"
					/>
				</div>

				<div class="form-group">
					<label for="slug">URL Slug</label>
					<input
						id="slug"
						type="text"
						bind:value={membership.slug}
						oninput={handleSlugInput}
						placeholder="pro-membership"
					/>
					<span class="field-hint">Used in URLs: /memberships/{membership.slug || 'slug'}</span>
				</div>

				<div class="form-group">
					<label for="description">Description</label>
					<textarea
						id="description"
						bind:value={membership.description}
						placeholder="Describe the benefits of this membership plan..."
						rows="3"
					></textarea>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="price">Price (USD) *</label>
						<div class="price-input">
							<span class="currency">$</span>
							<input
								id="price"
								type="number"
								bind:value={membership.price}
								placeholder="29.99"
								step="0.01"
								min="0"
							/>
						</div>
					</div>

					<div class="form-group">
						<label for="billing_cycle">Billing Cycle</label>
						<select id="billing_cycle" bind:value={membership.billing_cycle}>
							<option value="monthly">Monthly</option>
							<option value="quarterly">Quarterly</option>
							<option value="annual">Annual</option>
						</select>
					</div>
				</div>

				<div class="features-section">
					<div class="features-header">
						<h4>Features</h4>
						<button type="button" class="add-feature-btn" onclick={addFeature}>
							<IconPlus size={16} />
							Add Feature
						</button>
					</div>

					{#each membership.features as feature, index}
						<div class="feature-form">
							<div class="feature-inputs">
								<input
									type="text"
									bind:value={feature.feature_name}
									placeholder="Feature name"
									class="feature-name-input"
								/>
								<input
									type="text"
									bind:value={feature.description}
									placeholder="Description (optional)"
									class="feature-desc-input"
								/>
							</div>
							{#if membership.features.length > 1}
								<button
									type="button"
									class="remove-feature-btn"
									onclick={() => removeFeature(index)}
								>
									<IconX size={16} />
								</button>
							{/if}
						</div>
					{/each}
				</div>

				<div class="form-group">
					<label class="checkbox-label">
						<input type="checkbox" bind:checked={membership.is_active} />
						<span>Active (visible to customers)</span>
					</label>
				</div>

				<div class="form-actions">
					<button class="btn-secondary" onclick={() => goto('/admin/memberships')}>
						Cancel
					</button>
					<button
						class="btn-primary"
						onclick={saveMembership}
						disabled={saving || !isFormValid}
					>
						{#if saving}
							Creating...
						{:else}
							<IconCheck size={18} />
							Create Membership Plan
						{/if}
					</button>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CREATE MEMBERSHIP PAGE - Apple ICT 7 Principal Engineer Grade
	   ═══════════════════════════════════════════════════════════════════════════ */

	.create-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 24px;
	}

	/* Apple ICT7 Grade Header */
	.grade-header {
		margin-bottom: 24px;
		padding: 24px 28px;
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid #334155;
		border-radius: 16px;
		border-left: 4px solid #fbbf24;
	}

	.grade-header-content {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.grade-badge {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 4px 10px;
		background: rgba(251, 191, 36, 0.15);
		border: 1px solid rgba(251, 191, 36, 0.3);
		border-radius: 20px;
		color: #fbbf24;
		font-size: 0.7rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		width: fit-content;
	}

	.header-row {
		display: flex;
		align-items: flex-start;
		gap: 16px;
	}

	.btn-back {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: rgba(251, 191, 36, 0.1);
		border: 1px solid rgba(251, 191, 36, 0.3);
		border-radius: 10px;
		color: #fbbf24;
		cursor: pointer;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.btn-back:hover {
		background: rgba(251, 191, 36, 0.2);
		transform: translateX(-2px);
	}

	.grade-header h1 {
		display: flex;
		align-items: center;
		gap: 12px;
		margin: 0;
		font-size: 1.75rem;
		font-weight: 700;
		color: white;
	}

	.grade-header h1 :global(svg) {
		color: #fbbf24;
	}

	.header-subtitle {
		margin: 4px 0 0;
		font-size: 0.95rem;
		color: #64748b;
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 20px;
		margin-bottom: 24px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 12px;
		color: #f87171;
	}

	.error-banner span {
		flex: 1;
		font-size: 0.9rem;
	}

	.error-dismiss {
		display: flex;
		padding: 4px;
		background: transparent;
		border: none;
		color: #f87171;
		cursor: pointer;
		transition: opacity 0.2s;
	}

	.error-dismiss:hover {
		opacity: 0.7;
	}

	/* Content Container */
	.content-container {
		display: grid;
		grid-template-columns: 400px 1fr;
		gap: 24px;
		align-items: start;
	}

	.section-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 16px;
	}

	/* Membership Card Preview */
	.preview-section {
		position: sticky;
		top: 24px;
	}

	.membership-card {
		background: linear-gradient(135deg, #1e1b4b 0%, #0f0a2e 100%);
		border-radius: 24px;
		padding: 32px 24px;
		border: 2px solid transparent;
		position: relative;
		transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.membership-card::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: 24px;
		padding: 2px;
		background: linear-gradient(135deg, #fbbf24, #f59e0b, #d97706, #fbbf24);
		background-size: 300% 300%;
		-webkit-mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		-webkit-mask-composite: xor;
		mask-composite: exclude;
		animation: border-glow 4s ease infinite;
	}

	@keyframes border-glow {
		0%, 100% {
			background-position: 0% 50%;
			opacity: 0.6;
		}
		50% {
			background-position: 100% 50%;
			opacity: 1;
		}
	}

	.membership-card:hover {
		transform: translateY(-8px) scale(1.02);
		box-shadow:
			0 30px 60px -15px rgba(251, 191, 36, 0.4),
			0 0 0 1px rgba(251, 191, 36, 0.2),
			0 0 60px rgba(251, 191, 36, 0.2);
	}

	.card-crown {
		display: flex;
		justify-content: center;
		margin-bottom: 20px;
		color: #fbbf24;
		filter: drop-shadow(0 0 20px rgba(251, 191, 36, 0.8));
		animation: crown-float 3s ease-in-out infinite;
	}

	@keyframes crown-float {
		0%, 100% {
			transform: translateY(0) rotate(0deg);
			filter: drop-shadow(0 0 20px rgba(251, 191, 36, 0.8));
		}
		50% {
			transform: translateY(-8px) rotate(2deg);
			filter: drop-shadow(0 0 30px rgba(251, 191, 36, 1));
		}
	}

	.card-header {
		text-align: center;
		margin-bottom: 20px;
	}

	.card-header h4 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 12px;
	}

	.pricing {
		display: flex;
		align-items: baseline;
		justify-content: center;
		gap: 6px;
	}

	.amount {
		font-size: 2.75rem;
		font-weight: 700;
		background: linear-gradient(135deg, #fbbf24, #f59e0b, #fbbf24);
		background-size: 200% 200%;
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		animation: gold-shimmer 3s ease infinite;
	}

	@keyframes gold-shimmer {
		0%, 100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	.period {
		color: #94a3b8;
		font-size: 1rem;
	}

	.card-description {
		text-align: center;
		padding: 0 8px;
		margin-bottom: 24px;
	}

	.card-description p {
		color: #cbd5e1;
		font-size: 0.9rem;
		line-height: 1.6;
		min-height: 2.5rem;
		margin: 0;
	}

	.features-list {
		margin-bottom: 24px;
	}

	.feature-item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 12px;
		color: #f1f5f9;
		font-size: 0.9rem;
	}

	.feature-item :global(svg) {
		color: #fbbf24;
		flex-shrink: 0;
	}

	.empty-features {
		text-align: center;
		color: #64748b;
		padding: 24px;
		font-style: italic;
	}

	.subscribe-btn {
		width: 100%;
		padding: 14px;
		background: linear-gradient(135deg, #fbbf24, #f59e0b);
		color: #1e1b4b;
		border: none;
		border-radius: 14px;
		font-weight: 700;
		font-size: 1rem;
		cursor: not-allowed;
		opacity: 0.7;
		transition: all 0.3s;
	}

	/* Form Section */
	.form-card {
		background: linear-gradient(135deg, #1e293b 0%, #1a2332 100%);
		border-radius: 16px;
		padding: 24px;
		border: 1px solid #334155;
	}

	.form-group {
		margin-bottom: 20px;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.form-group label {
		display: block;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 8px;
		font-size: 0.9rem;
	}

	.form-group input[type='text'],
	.form-group input[type='number'],
	.form-group textarea,
	.form-group select {
		width: 100%;
		padding: 12px 16px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9rem;
		transition: all 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: #fbbf24;
		box-shadow: 0 0 0 3px rgba(251, 191, 36, 0.1);
	}

	.form-group input::placeholder,
	.form-group textarea::placeholder {
		color: #64748b;
	}

	.form-group textarea {
		resize: vertical;
		font-family: inherit;
		min-height: 80px;
	}

	.form-group select {
		cursor: pointer;
	}

	.field-hint {
		display: block;
		margin-top: 6px;
		font-size: 0.75rem;
		color: #64748b;
	}

	.price-input {
		position: relative;
		display: flex;
		align-items: center;
	}

	.price-input .currency {
		position: absolute;
		left: 16px;
		color: #fbbf24;
		font-weight: 600;
		font-size: 1rem;
	}

	.price-input input {
		padding-left: 36px;
	}

	.features-section {
		margin: 24px 0;
		padding: 20px;
		background: rgba(251, 191, 36, 0.05);
		border-radius: 12px;
		border: 1px solid rgba(251, 191, 36, 0.1);
	}

	.features-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
	}

	.features-header h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.add-feature-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 14px;
		background: rgba(251, 191, 36, 0.1);
		border: 1px solid rgba(251, 191, 36, 0.3);
		border-radius: 8px;
		color: #fbbf24;
		font-weight: 600;
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.add-feature-btn:hover {
		background: rgba(251, 191, 36, 0.2);
	}

	.feature-form {
		display: flex;
		gap: 12px;
		margin-bottom: 12px;
	}

	.feature-inputs {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.feature-name-input,
	.feature-desc-input {
		padding: 10px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.85rem;
	}

	.feature-name-input:focus,
	.feature-desc-input:focus {
		outline: none;
		border-color: #fbbf24;
	}

	.feature-name-input {
		font-weight: 500;
	}

	.feature-desc-input {
		font-size: 0.8rem;
	}

	.remove-feature-btn {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
		width: 36px;
		height: 36px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.remove-feature-btn:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 12px;
		cursor: pointer;
		color: #f1f5f9;
	}

	.checkbox-label input[type='checkbox'] {
		width: 20px;
		height: 20px;
		cursor: pointer;
	}

	.form-actions {
		display: flex;
		gap: 12px;
		justify-content: flex-end;
		margin-top: 28px;
		padding-top: 24px;
		border-top: 1px solid #334155;
	}

	.btn-secondary,
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-secondary {
		background: #0f172a;
		color: #e2e8f0;
		border: 1px solid #334155;
	}

	.btn-secondary:hover {
		background: #1e293b;
		border-color: #475569;
	}

	.btn-primary {
		background: linear-gradient(135deg, #fbbf24, #f59e0b);
		color: #1e1b4b;
		box-shadow: 0 4px 14px rgba(251, 191, 36, 0.25);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(251, 191, 36, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.content-container {
			grid-template-columns: 1fr;
		}

		.preview-section {
			position: static;
			order: -1;
		}

		.membership-card {
			max-width: 400px;
			margin: 0 auto;
		}

		.form-row {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.create-page {
			padding: 16px;
		}

		.grade-header {
			padding: 20px;
		}

		.grade-header h1 {
			font-size: 1.4rem;
		}

		.header-row {
			flex-direction: column;
			gap: 12px;
		}

		.btn-back {
			width: 40px;
			height: 40px;
		}

		.form-card {
			padding: 20px;
		}

		.form-actions {
			flex-direction: column;
		}

		.btn-secondary,
		.btn-primary {
			width: 100%;
			justify-content: center;
		}
	}
</style>
