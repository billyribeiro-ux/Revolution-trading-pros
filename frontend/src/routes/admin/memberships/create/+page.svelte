<script lang="ts">
	import { goto } from '$app/navigation';
	import { IconCrown, IconPlus, IconX, IconCheck } from '@tabler/icons-svelte';

	let membership = {
		name: '',
		slug: '',
		description: '',
		price: '',
		billing_cycle: 'monthly',
		is_active: true,
		features: [{ feature_code: 'feature_1', feature_name: '', description: '' }]
	};

	let saving = false;

	function generateSlug() {
		if (!membership.slug && membership.name) {
			membership.slug = membership.name
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
		}
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
		if (!membership.name || !membership.price) {
			alert('Please fill in name and price');
			return;
		}

		// Filter out empty features
		const validFeatures = membership.features.filter((f) => f.feature_name.trim());

		saving = true;
		try {
			const response = await fetch('/api/admin/membership-plans', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					...membership,
					features: validFeatures
				})
			});

			if (response.ok) {
				goto('/admin/memberships');
			} else {
				const error = await response.json();
				alert(error.message || 'Failed to create membership plan');
			}
		} catch (error) {
			console.error('Failed to save membership:', error);
			alert('Failed to save membership plan');
		} finally {
			saving = false;
		}
	}

	function getBillingLabel(cycle: string) {
		return (
			{
				monthly: '/month',
				quarterly: '/quarter',
				annual: '/year'
			}[cycle] || '/month'
		);
	}
</script>

<svelte:head>
	<title>Add New Membership | Admin</title>
</svelte:head>

<div class="create-page">
	<div class="page-header">
		<div>
			<h1><IconCrown size={32} /> Add New Membership Plan</h1>
			<p>Create a premium membership plan for your subscribers</p>
		</div>
	</div>

	<div class="content-container">
		<!-- Preview Card -->
		<div class="preview-section">
			<h3>Preview</h3>
			<div class="membership-card">
				<div class="card-crown">
					<IconCrown size={32} />
				</div>
				<div class="card-header">
					<h4>{membership.name || 'Plan Name'}</h4>
					<div class="pricing">
						<span class="amount">${membership.price || '0'}</span>
						<span class="period">{getBillingLabel(membership.billing_cycle)}</span>
					</div>
				</div>
				<div class="card-description">
					<p>{membership.description || 'Add a description...'}</p>
				</div>
				<div class="features-list">
					{#each membership.features.filter((f) => f.feature_name) as feature}
						<div class="feature-item">
							<IconCheck size={18} />
							<span>{feature.feature_name}</span>
						</div>
					{/each}
					{#if membership.features.filter((f) => f.feature_name).length === 0}
						<div class="empty-features">Add features below</div>
					{/if}
				</div>
				<button class="subscribe-btn" disabled> Subscribe Now </button>
			</div>
		</div>

		<!-- Form Section -->
		<div class="form-section">
			<div class="form-card">
				<h3>Membership Details</h3>

				<div class="form-group">
					<label for="name">Plan Name *</label>
					<input
						id="name"
						type="text"
						bind:value={membership.name}
						on:blur={generateSlug}
						placeholder="e.g., Pro Membership"
					/>
				</div>

				<div class="form-group">
					<label for="slug">URL Slug</label>
					<input id="slug" type="text" bind:value={membership.slug} placeholder="pro-membership" />
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
					<div class="section-header">
						<h4>Features</h4>
						<button type="button" class="add-feature-btn" on:click={addFeature}>
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
									on:click={() => removeFeature(index)}
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
					<button class="btn-secondary" on:click={() => goto('/admin/memberships')}>
						Cancel
					</button>
					<button class="btn-primary" on:click={saveMembership} disabled={saving}>
						{saving ? 'Creating...' : 'Create Membership Plan'}
					</button>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.create-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #94a3b8;
		font-size: 1rem;
	}

	.content-container {
		display: grid;
		grid-template-columns: 400px 1fr;
		gap: 2rem;
		align-items: start;
	}

	.preview-section h3,
	.form-section h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	/* Membership Card Preview */
	.membership-card {
		background: linear-gradient(135deg, #1e1b4b 0%, #0f0a2e 100%);
		border-radius: 24px;
		padding: 2rem;
		border: 2px solid transparent;
		background-clip: padding-box;
		position: relative;
		transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
		position: sticky;
		top: 2rem;
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
		0%,
		100% {
			background-position: 0% 50%;
			opacity: 0.6;
		}
		50% {
			background-position: 100% 50%;
			opacity: 1;
		}
	}

	.membership-card:hover {
		transform: translateY(-12px) scale(1.03);
		box-shadow:
			0 30px 60px -15px rgba(251, 191, 36, 0.5),
			0 0 0 1px rgba(251, 191, 36, 0.2),
			inset 0 1px 0 rgba(255, 255, 255, 0.15),
			0 0 60px rgba(251, 191, 36, 0.3);
	}

	.card-crown {
		display: flex;
		justify-content: center;
		margin-bottom: 1.5rem;
		color: #fbbf24;
		filter: drop-shadow(0 0 20px rgba(251, 191, 36, 0.8));
		animation: crown-float 3s ease-in-out infinite;
	}

	@keyframes crown-float {
		0%,
		100% {
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
		margin-bottom: 1.5rem;
	}

	.card-header h4 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.pricing {
		display: flex;
		align-items: baseline;
		justify-content: center;
		gap: 0.5rem;
	}

	.amount {
		font-size: 3rem;
		font-weight: 700;
		background: linear-gradient(135deg, #fbbf24, #f59e0b, #fbbf24);
		background-size: 200% 200%;
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		animation: gold-shimmer 3s ease infinite;
		position: relative;
	}

	.amount::before {
		content: attr(data-price);
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #fbbf24, #f59e0b);
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		filter: blur(12px);
		opacity: 0.6;
	}

	@keyframes gold-shimmer {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	.period {
		color: #94a3b8;
		font-size: 1.125rem;
	}

	.card-description {
		text-align: center;
		padding: 0 0.5rem;
		margin-bottom: 2rem;
	}

	.card-description p {
		color: #cbd5e1;
		font-size: 0.9375rem;
		line-height: 1.6;
		min-height: 2.5rem;
	}

	.features-list {
		margin-bottom: 2rem;
	}

	.feature-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.feature-item :global(svg) {
		color: #fbbf24;
		flex-shrink: 0;
	}

	.empty-features {
		text-align: center;
		color: #64748b;
		padding: 2rem;
		font-style: italic;
	}

	.subscribe-btn {
		width: 100%;
		padding: 1rem;
		background: linear-gradient(135deg, #fbbf24, #f59e0b);
		color: #1e1b4b;
		border: none;
		border-radius: 14px;
		font-weight: 700;
		font-size: 1.125rem;
		cursor: pointer;
		transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
		position: relative;
		overflow: hidden;
		box-shadow: 0 4px 14px rgba(251, 191, 36, 0.3);
	}

	.subscribe-btn::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #f59e0b, #d97706);
		opacity: 0;
		transition: opacity 0.4s;
	}

	.subscribe-btn::after {
		content: '';
		position: absolute;
		inset: 0;
		background: radial-gradient(circle at center, rgba(255, 255, 255, 0.8) 0%, transparent 70%);
		opacity: 0;
		transform: scale(0);
		transition: all 0.6s;
	}

	.subscribe-btn:hover:not(:disabled) {
		transform: translateY(-3px);
		box-shadow:
			0 16px 32px rgba(251, 191, 36, 0.5),
			0 0 40px rgba(251, 191, 36, 0.4);
	}

	.subscribe-btn:hover:not(:disabled)::before {
		opacity: 1;
	}

	.subscribe-btn:hover:not(:disabled)::after {
		opacity: 0.3;
		transform: scale(1);
	}

	.subscribe-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Form Section */
	.form-card {
		background: linear-gradient(135deg, #1e293b 0%, #1a2332 100%);
		border-radius: 20px;
		padding: 2rem;
		border: 1px solid rgba(251, 191, 36, 0.2);
		box-shadow:
			0 4px 6px rgba(0, 0, 0, 0.1),
			inset 0 1px 0 rgba(255, 255, 255, 0.05);
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-group label {
		display: block;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.9375rem;
	}

	.form-group input[type='text'],
	.form-group input[type='number'],
	.form-group textarea,
	.form-group select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(251, 191, 36, 0.25);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: #fbbf24;
		background: rgba(15, 23, 42, 0.8);
		box-shadow:
			0 0 0 3px rgba(251, 191, 36, 0.2),
			0 4px 12px rgba(251, 191, 36, 0.15);
		transform: translateY(-1px);
	}

	.form-group textarea {
		resize: vertical;
		font-family: inherit;
	}

	.form-group select {
		cursor: pointer;
	}

	.price-input {
		position: relative;
		display: flex;
		align-items: center;
	}

	.price-input .currency {
		position: absolute;
		left: 1rem;
		color: #fbbf24;
		font-weight: 600;
		font-size: 1.125rem;
	}

	.price-input input {
		padding-left: 2.5rem;
	}

	.features-section {
		margin: 2rem 0;
		padding: 1.5rem;
		background: rgba(251, 191, 36, 0.05);
		border-radius: 12px;
		border: 1px solid rgba(251, 191, 36, 0.1);
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.section-header h4 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.add-feature-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(251, 191, 36, 0.1);
		border: 1px solid rgba(251, 191, 36, 0.3);
		border-radius: 6px;
		color: #fde68a;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.add-feature-btn:hover {
		background: rgba(251, 191, 36, 0.2);
	}

	.feature-form {
		display: flex;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.feature-inputs {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.feature-name-input,
	.feature-desc-input {
		padding: 0.625rem 0.875rem;
		background: #0f172a;
		border: 1px solid rgba(251, 191, 36, 0.2);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.feature-name-input {
		font-weight: 500;
	}

	.feature-desc-input {
		font-size: 0.8125rem;
	}

	.remove-feature-btn {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #fca5a5;
		width: 36px;
		height: 36px;
		border-radius: 6px;
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
		gap: 0.75rem;
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
		gap: 1rem;
		justify-content: flex-end;
		margin-top: 2rem;
		padding-top: 2rem;
		border-top: 1px solid rgba(251, 191, 36, 0.1);
	}

	.btn-secondary,
	.btn-primary {
		padding: 0.75rem 1.5rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		border: none;
		position: relative;
		overflow: hidden;
	}

	.btn-secondary {
		background: rgba(251, 191, 36, 0.1);
		color: #fde68a;
		border: 1px solid rgba(251, 191, 36, 0.3);
	}

	.btn-secondary::before {
		content: '';
		position: absolute;
		inset: 0;
		background: rgba(251, 191, 36, 0.2);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.btn-secondary:hover::before {
		opacity: 1;
	}

	.btn-secondary:hover {
		border-color: rgba(251, 191, 36, 0.5);
		transform: translateY(-1px);
	}

	.btn-primary {
		background: linear-gradient(135deg, #fbbf24, #f59e0b);
		color: #1e1b4b;
		box-shadow: 0 4px 14px rgba(251, 191, 36, 0.25);
	}

	.btn-primary::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #f59e0b, #d97706);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow:
			0 12px 24px rgba(251, 191, 36, 0.4),
			0 0 20px rgba(245, 158, 11, 0.3);
	}

	.btn-primary:hover:not(:disabled)::before {
		opacity: 1;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	@media (max-width: 1024px) {
		.content-container {
			grid-template-columns: 1fr;
		}

		.membership-card {
			position: static;
		}

		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>
