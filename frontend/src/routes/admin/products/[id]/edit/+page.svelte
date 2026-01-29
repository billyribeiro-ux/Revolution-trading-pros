<!--
	URL: /admin/products/[id]/edit
	Product Edit Page - Svelte 5 Runes Implementation
-->

<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { untrack } from 'svelte';
	import { productsApi, AdminApiError, type Product } from '$lib/api/admin';
	import {
		IconPlus,
		IconX,
		IconCheck,
		IconArrowLeft,
		IconBook,
		IconChartLine,
		IconCrown,
		IconShoppingCart,
		IconPhoto,
		IconTag,
		IconTrash
	} from '$lib/icons';

	// Product type definition
	type ProductType = 'course' | 'indicator' | 'membership' | 'bundle';

	// Get product ID from route params
	let productId = $derived(Number(page.params.id));

	// Loading states
	let loading = $state(true);
	let saving = $state(false);
	let deleting = $state(false);

	// Error and validation state
	let error = $state('');
	let loadError = $state('');
	let validationErrors = $state<Record<string, string[]>>({});

	// Original product data for comparison
	let originalProduct = $state<Product | null>(null);

	// Form state using Svelte 5 $state rune
	let formData = $state({
		name: '',
		slug: '',
		type: 'course' as ProductType,
		description: '',
		long_description: '',
		price: '',
		sale_price: '',
		currency: 'USD',
		features: [''] as string[],
		is_active: true,
		thumbnail: '',
		meta_title: '',
		meta_description: '',
		indexable: true,
		canonical_url: ''
	});

	// UI state
	let showSeoSettings = $state(false);
	let showDeleteConfirm = $state(false);

	// Product type options
	const productTypes = [
		{ value: 'course', label: 'Course', icon: IconBook, color: '#3b82f6' },
		{ value: 'indicator', label: 'Indicator', icon: IconChartLine, color: '#8b5cf6' },
		{ value: 'membership', label: 'Membership', icon: IconCrown, color: '#eab308' },
		{ value: 'bundle', label: 'Bundle', icon: IconShoppingCart, color: '#10b981' }
	] as const;

	// Derived state - form validation
	let isFormValid = $derived(
		formData.name.trim().length > 0 &&
			formData.slug.trim().length > 0 &&
			parseFloat(formData.price) > 0
	);

	// Derived state - has changes
	let hasChanges = $derived.by(() => {
		if (!originalProduct) return false;
		return (
			formData.name !== originalProduct.name ||
			formData.slug !== originalProduct.slug ||
			formData.type !== originalProduct.type ||
			formData.description !== (originalProduct.description || '') ||
			formData.long_description !== (originalProduct.long_description || '') ||
			formData.price !== String(originalProduct.price) ||
			formData.sale_price !==
				(originalProduct.sale_price ? String(originalProduct.sale_price) : '') ||
			formData.is_active !== originalProduct.is_active ||
			formData.thumbnail !== (originalProduct.thumbnail || '') ||
			JSON.stringify(formData.features.filter((f) => f.trim())) !==
				JSON.stringify(originalProduct.features || [])
		);
	});

	// Derived state - formatted price preview
	let pricePreview = $derived.by(() => {
		const price = parseFloat(formData.price) || 0;
		const salePrice = parseFloat(formData.sale_price) || null;
		if (salePrice && salePrice < price) {
			return { original: `$${price.toFixed(2)}`, sale: `$${salePrice.toFixed(2)}` };
		}
		return { original: `$${price.toFixed(2)}`, sale: null };
	});

	// Derived state - valid features count
	let validFeatures = $derived(formData.features.filter((f) => f.trim().length > 0));

	// Derived state - product type icon and color for preview
	let previewTypeIcon = $derived(
		productTypes.find((t) => t.value === formData.type)?.icon || IconShoppingCart
	);
	let previewTypeColor = $derived(
		productTypes.find((t) => t.value === formData.type)?.color || '#64748b'
	);

	// Load product data
	async function loadProduct() {
		loading = true;
		loadError = '';

		try {
			const response = await productsApi.get(productId);
			const product = response.data;
			originalProduct = product;

			// Populate form
			formData.name = product.name;
			formData.slug = product.slug;
			formData.type = product.type;
			formData.description = product.description || '';
			formData.long_description = product.long_description || '';
			formData.price = String(product.price);
			formData.sale_price = product.sale_price ? String(product.sale_price) : '';
			formData.currency = product.currency || 'USD';
			formData.features = product.features?.length ? [...product.features] : [''];
			formData.is_active = product.is_active;
			formData.thumbnail = product.thumbnail || '';
			formData.meta_title = product.meta_title || '';
			formData.meta_description = product.meta_description || '';
			formData.indexable = product.indexable ?? true;
			formData.canonical_url = product.canonical_url || '';
		} catch (err) {
			if (err instanceof AdminApiError) {
				if (err.status === 401) {
					goto('/login');
					return;
				}
				if (err.status === 404) {
					loadError = 'Product not found';
				} else {
					loadError = err.message;
				}
			} else {
				loadError = 'Failed to load product';
			}
			console.error('Failed to load product:', err);
		} finally {
			loading = false;
		}
	}

	// Feature management
	function addFeature() {
		formData.features = [...formData.features, ''];
	}

	function removeFeature(index: number) {
		formData.features = formData.features.filter((_, i) => i !== index);
	}

	function updateFeature(index: number, value: string) {
		formData.features[index] = value;
	}

	// Save product
	async function saveProduct() {
		if (!isFormValid) {
			error = 'Please fill in all required fields';
			return;
		}

		saving = true;
		error = '';
		validationErrors = {};

		try {
			const productData: Partial<Product> = {
				name: formData.name.trim(),
				slug: formData.slug.trim(),
				type: formData.type,
				description: formData.description.trim() || undefined,
				long_description: formData.long_description.trim() || null,
				price: parseFloat(formData.price),
				sale_price: formData.sale_price ? parseFloat(formData.sale_price) : null,
				currency: formData.currency,
				features: validFeatures,
				is_active: formData.is_active,
				thumbnail: formData.thumbnail.trim() || null,
				meta_title: formData.meta_title.trim() || null,
				meta_description: formData.meta_description.trim() || null,
				indexable: formData.indexable,
				canonical_url: formData.canonical_url.trim() || null
			};

			const response = await productsApi.update(productId, productData);
			originalProduct = response.data;

			// Show success briefly then redirect
			goto('/admin/products');
		} catch (err) {
			if (err instanceof AdminApiError) {
				if (err.status === 401) {
					goto('/login');
					return;
				}
				if (err.validationErrors) {
					validationErrors = err.validationErrors;
				}
				error = err.message;
			} else {
				error = 'Failed to update product';
			}
			console.error('Failed to update product:', err);
		} finally {
			saving = false;
		}
	}

	// Delete product
	async function deleteProduct() {
		deleting = true;
		error = '';

		try {
			await productsApi.delete(productId);
			goto('/admin/products');
		} catch (err) {
			if (err instanceof AdminApiError) {
				error = err.message;
			} else {
				error = 'Failed to delete product';
			}
			console.error('Failed to delete product:', err);
			showDeleteConfirm = false;
		} finally {
			deleting = false;
		}
	}

	// Get field error
	function getFieldError(field: string): string | undefined {
		return validationErrors[field]?.[0];
	}

	// Effect: Load product when ID changes
	$effect(() => {
		const id = productId;
		if (id > 0) {
			untrack(() => {
				loadProduct();
			});
		}
	});
</script>

<svelte:head>
	<title>{originalProduct?.name ? `Edit ${originalProduct.name}` : 'Edit Product'} | Admin</title>
</svelte:head>

<div class="edit-page">
	<div class="page-header">
		<button class="back-btn" onclick={() => goto('/admin/products')}>
			<IconArrowLeft size={20} />
			Back to Products
		</button>
		<div class="header-content">
			<div>
				<h1>Edit Product</h1>
				{#if originalProduct}
					<p>Editing: {originalProduct.name}</p>
				{/if}
			</div>
			{#if originalProduct}
				<button class="btn-danger" onclick={() => (showDeleteConfirm = true)} disabled={deleting}>
					<IconTrash size={18} />
					Delete Product
				</button>
			{/if}
		</div>
	</div>

	{#if loadError}
		<div class="load-error">
			<h3>Error Loading Product</h3>
			<p>{loadError}</p>
			<button class="btn-primary" onclick={() => goto('/admin/products')}>
				Back to Products
			</button>
		</div>
	{:else if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading product...</p>
		</div>
	{:else}
		{#if error}
			<div class="alert error">
				<span>{error}</span>
				<button class="alert-dismiss" onclick={() => (error = '')}>
					<IconX size={16} />
				</button>
			</div>
		{/if}

		<div class="content-grid">
			<!-- Main Form -->
			<div class="form-section">
				<div class="form-card">
					<div class="form-card-header">
						<h3>Product Details</h3>
						{#if hasChanges}
							<span class="unsaved-badge">Unsaved changes</span>
						{/if}
					</div>

					<!-- Product Type Selection -->
					<div class="form-group">
						<!-- svelte-ignore a11y_label_has_associated_control -->
						<label>Product Type *</label>
						<div class="type-selector">
							{#each productTypes as type}
								{@const Icon = type.icon}
								<button
									type="button"
									class="type-option"
									class:selected={formData.type === type.value}
									onclick={() => (formData.type = type.value)}
									style="--type-color: {type.color}"
								>
									<Icon size={24} />
									<span>{type.label}</span>
								</button>
							{/each}
						</div>
					</div>

					<!-- Name -->
					<div class="form-group" class:has-error={getFieldError('name')}>
						<label for="name">Product Name *</label>
						<input
							id="name"
							type="text"
							bind:value={formData.name}
							placeholder="e.g., Advanced Trading Course"
						/>
						{#if getFieldError('name')}
							<span class="field-error">{getFieldError('name')}</span>
						{/if}
					</div>

					<!-- Slug -->
					<div class="form-group" class:has-error={getFieldError('slug')}>
						<label for="slug">URL Slug *</label>
						<div class="slug-input">
							<span class="slug-prefix">/products/</span>
							<input
								id="slug"
								type="text"
								bind:value={formData.slug}
								placeholder="advanced-trading-course"
							/>
						</div>
						{#if getFieldError('slug')}
							<span class="field-error">{getFieldError('slug')}</span>
						{/if}
					</div>

					<!-- Description -->
					<div class="form-group">
						<label for="description">Short Description</label>
						<textarea
							id="description"
							bind:value={formData.description}
							placeholder="Brief description of the product..."
							rows="3"
						></textarea>
					</div>

					<!-- Long Description -->
					<div class="form-group">
						<label for="long_description">Full Description</label>
						<textarea
							id="long_description"
							bind:value={formData.long_description}
							placeholder="Detailed description with features and benefits..."
							rows="6"
						></textarea>
					</div>

					<!-- Pricing -->
					<div class="form-row">
						<div class="form-group" class:has-error={getFieldError('price')}>
							<label for="price">Price (USD) *</label>
							<div class="price-input">
								<span class="currency">$</span>
								<input
									id="price"
									type="number"
									bind:value={formData.price}
									placeholder="99.00"
									step="0.01"
									min="0"
								/>
							</div>
							{#if getFieldError('price')}
								<span class="field-error">{getFieldError('price')}</span>
							{/if}
						</div>

						<div class="form-group">
							<label for="sale_price">Sale Price (Optional)</label>
							<div class="price-input">
								<span class="currency">$</span>
								<input
									id="sale_price"
									type="number"
									bind:value={formData.sale_price}
									placeholder="79.00"
									step="0.01"
									min="0"
								/>
							</div>
						</div>
					</div>

					<!-- Thumbnail -->
					<div class="form-group">
						<label for="thumbnail">
							<IconPhoto size={16} />
							Thumbnail URL
						</label>
						<input
							id="thumbnail"
							type="url"
							bind:value={formData.thumbnail}
							placeholder="https://example.com/image.jpg"
						/>
					</div>

					<!-- Features -->
					<div class="features-section">
						<div class="section-header">
							<h4>
								<IconTag size={18} />
								Features
							</h4>
							<button type="button" class="add-feature-btn" onclick={addFeature}>
								<IconPlus size={16} />
								Add Feature
							</button>
						</div>

						{#each formData.features as feature, index}
							<div class="feature-row">
								<input
									id="edit-feature-{index}"
									name="edit-feature-{index}"
									type="text"
									value={feature}
									oninput={(e) => updateFeature(index, e.currentTarget.value)}
									placeholder="Feature description"
								/>
								{#if formData.features.length > 1}
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

					<!-- Active Status -->
					<div class="form-group">
						<label class="checkbox-label">
							<input
								id="edit-is-active"
								name="edit-is-active"
								type="checkbox"
								bind:checked={formData.is_active}
							/>
							<span>Active (visible to customers)</span>
						</label>
					</div>

					<!-- SEO Settings Toggle -->
					<button
						type="button"
						class="seo-toggle"
						onclick={() => (showSeoSettings = !showSeoSettings)}
					>
						{showSeoSettings ? 'Hide' : 'Show'} SEO Settings
					</button>

					{#if showSeoSettings}
						<div class="seo-settings">
							<div class="form-group">
								<label for="meta_title">Meta Title</label>
								<input
									id="meta_title"
									type="text"
									bind:value={formData.meta_title}
									placeholder="SEO title for search engines"
								/>
							</div>

							<div class="form-group">
								<label for="meta_description">Meta Description</label>
								<textarea
									id="meta_description"
									bind:value={formData.meta_description}
									placeholder="SEO description for search engines..."
									rows="2"
								></textarea>
							</div>

							<div class="form-group">
								<label class="checkbox-label">
									<input
										id="edit-indexable"
										name="edit-indexable"
										type="checkbox"
										bind:checked={formData.indexable}
									/>
									<span>Allow search engine indexing</span>
								</label>
							</div>

							<div class="form-group">
								<label for="canonical_url">Canonical URL</label>
								<input
									id="canonical_url"
									type="url"
									bind:value={formData.canonical_url}
									placeholder="https://example.com/canonical-page"
								/>
							</div>
						</div>
					{/if}

					<!-- Form Actions -->
					<div class="form-actions">
						<button type="button" class="btn-secondary" onclick={() => goto('/admin/products')}>
							Cancel
						</button>
						<button
							type="button"
							class="btn-primary"
							onclick={saveProduct}
							disabled={saving || !isFormValid}
						>
							{#if saving}
								<div class="btn-spinner"></div>
								Saving...
							{:else}
								<IconCheck size={18} />
								Save Changes
							{/if}
						</button>
					</div>
				</div>
			</div>

			<!-- Preview Sidebar -->
			<div class="preview-section">
				<h3>Preview</h3>
				<div class="product-preview-card">
					{#if formData.thumbnail}
						<div class="preview-thumbnail">
							<img src={formData.thumbnail} alt={formData.name || 'Product'} />
						</div>
					{:else}
						<div class="preview-thumbnail placeholder">
							<!-- svelte-ignore svelte_component_deprecated -->
							<svelte:component this={previewTypeIcon} size={48} />
						</div>
					{/if}

					<div class="preview-badge" style="background: {previewTypeColor}">
						<!-- svelte-ignore svelte_component_deprecated -->
						<svelte:component this={previewTypeIcon} size={14} />
						{formData.type}
					</div>

					<div class="preview-content">
						<h4>{formData.name || 'Product Name'}</h4>
						{#if formData.description}
							<p class="preview-description">{formData.description}</p>
						{/if}

						<div class="preview-price">
							{#if pricePreview.sale}
								<span class="preview-original">{pricePreview.original}</span>
								<span class="preview-sale">{pricePreview.sale}</span>
							{:else}
								<span class="preview-current">{pricePreview.original}</span>
							{/if}
						</div>

						{#if validFeatures.length > 0}
							<div class="preview-features">
								{#each validFeatures.slice(0, 4) as feature}
									<div class="preview-feature">
										<IconCheck size={14} />
										<span>{feature}</span>
									</div>
								{/each}
								{#if validFeatures.length > 4}
									<div class="preview-more">+{validFeatures.length - 4} more features</div>
								{/if}
							</div>
						{/if}
					</div>

					<div class="preview-status" class:active={formData.is_active}>
						{formData.is_active ? 'Active' : 'Inactive'}
					</div>
				</div>

				<!-- Product Info -->
				{#if originalProduct}
					<div class="product-info">
						<h4>Product Info</h4>
						<div class="info-row">
							<span class="info-label">ID:</span>
							<span class="info-value">{originalProduct.id}</span>
						</div>
						<div class="info-row">
							<span class="info-label">Created:</span>
							<span class="info-value"
								>{new Date(originalProduct.created_at).toLocaleDateString()}</span
							>
						</div>
						<div class="info-row">
							<span class="info-label">Updated:</span>
							<span class="info-value"
								>{new Date(originalProduct.updated_at).toLocaleDateString()}</span
							>
						</div>
					</div>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Delete Confirmation Modal -->
	{#if showDeleteConfirm}
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_interactive_supports_focus -->
		<div
			class="modal-overlay"
			onclick={() => (showDeleteConfirm = false)}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
			<div
				class="modal"
				onclick={(e) => e.stopPropagation()}
				onkeydown={(e) => e.key === 'Escape' && (showDeleteConfirm = false)}
				role="document"
				tabindex="-1"
			>
				<h3>Delete Product?</h3>
				<p>
					Are you sure you want to delete <strong>{originalProduct?.name}</strong>? This action
					cannot be undone.
				</p>
				<div class="modal-actions">
					<button class="btn-secondary" onclick={() => (showDeleteConfirm = false)}>
						Cancel
					</button>
					<button class="btn-danger" onclick={deleteProduct} disabled={deleting}>
						{#if deleting}
							<div class="btn-spinner danger"></div>
							Deleting...
						{:else}
							<IconTrash size={18} />
							Delete Product
						{/if}
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.edit-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
		background: #0f172a;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: transparent;
		border: none;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		margin-bottom: 1rem;
		padding: 0;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: #f1f5f9;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #94a3b8;
		font-size: 0.95rem;
	}

	/* Loading & Error States */
	.loading,
	.load-error {
		text-align: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.loading .spinner {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(148, 163, 184, 0.1);
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 1rem;
	}

	.load-error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 16px;
	}

	.load-error h3 {
		color: #f87171;
		margin-bottom: 0.5rem;
	}

	.load-error p {
		margin-bottom: 1.5rem;
	}

	/* Alert */
	.alert {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-radius: 8px;
		margin-bottom: 1.5rem;
	}

	.alert.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.alert-dismiss {
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		padding: 0.25rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* Content Grid */
	.content-grid {
		display: grid;
		grid-template-columns: 1fr 380px;
		gap: 2rem;
		align-items: start;
	}

	/* Form Section */
	.form-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 16px;
		padding: 2rem;
	}

	.form-card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.form-card-header h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.unsaved-badge {
		padding: 0.25rem 0.75rem;
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group.has-error input {
		border-color: #ef4444;
	}

	/* Keeping for potential future use */
	/*.form-group.has-error textarea {
		border-color: #ef4444;
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.9375rem;
	}

	.form-group input[type='text'],
	.form-group input[type='number'],
	.form-group input[type='url'],
	.form-group textarea,
	.form-group select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.25);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: all 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
	}

	.form-group textarea {
		resize: vertical;
		font-family: inherit;
	}

	.field-error {
		display: block;
		color: #f87171;
		font-size: 0.8125rem;
		margin-top: 0.375rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	/* Type Selector */
	.type-selector {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.75rem;
	}

	.type-option {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-option:hover {
		border-color: rgba(148, 163, 184, 0.4);
		color: #f1f5f9;
	}

	.type-option.selected {
		border-color: var(--type-color);
		background: color-mix(in srgb, var(--type-color) 10%, transparent);
		color: #f1f5f9;
	}

	.type-option span {
		font-size: 0.875rem;
		font-weight: 500;
	}

	/* Slug Input */
	.slug-input {
		display: flex;
		align-items: center;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.25);
		border-radius: 8px;
		overflow: hidden;
	}

	.slug-prefix {
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.8);
		color: #64748b;
		font-size: 0.875rem;
		font-family: monospace;
		border-right: 1px solid rgba(148, 163, 184, 0.2);
	}

	.slug-input input {
		border: none;
		border-radius: 0;
		background: transparent;
	}

	.slug-input input:focus {
		box-shadow: none;
	}

	/* Price Input */
	.price-input {
		position: relative;
		display: flex;
		align-items: center;
	}

	.price-input .currency {
		position: absolute;
		left: 1rem;
		color: #3b82f6;
		font-weight: 600;
		font-size: 1rem;
	}

	.price-input input {
		padding-left: 2.25rem;
	}

	/* Features Section */
	.features-section {
		margin: 1.5rem 0;
		padding: 1.5rem;
		background: rgba(59, 130, 246, 0.05);
		border-radius: 12px;
		border: 1px solid rgba(59, 130, 246, 0.15);
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.section-header h4 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.add-feature-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.3);
		border-radius: 6px;
		color: #60a5fa;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.add-feature-btn:hover {
		background: rgba(59, 130, 246, 0.2);
	}

	.feature-row {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.feature-row input {
		flex: 1;
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.remove-feature-btn {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
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

	/* Checkbox */
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
		accent-color: #3b82f6;
	}

	/* SEO Toggle */
	.seo-toggle {
		width: 100%;
		padding: 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px dashed rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		margin-bottom: 1rem;
	}

	.seo-toggle:hover {
		background: rgba(148, 163, 184, 0.15);
		color: #f1f5f9;
	}

	.seo-settings {
		padding: 1.5rem;
		background: rgba(148, 163, 184, 0.05);
		border-radius: 12px;
		border: 1px solid rgba(148, 163, 184, 0.15);
		margin-bottom: 1.5rem;
	}

	/* Form Actions */
	.form-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
		margin-top: 2rem;
		padding-top: 2rem;
		border-top: 1px solid rgba(148, 163, 184, 0.15);
	}

	.btn-primary,
	.btn-secondary,
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

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		color: #cbd5e1;
	}

	.btn-secondary:hover {
		background: rgba(30, 41, 59, 0.8);
	}

	.btn-danger {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.btn-danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.2);
	}

	.btn-danger:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.btn-spinner.danger {
		border-color: rgba(248, 113, 113, 0.3);
		border-top-color: #f87171;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Preview Section */
	.preview-section {
		position: sticky;
		top: 2rem;
	}

	.preview-section h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.product-preview-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 16px;
		overflow: hidden;
		position: relative;
		margin-bottom: 1.5rem;
	}

	.preview-thumbnail {
		height: 200px;
		background: rgba(15, 23, 42, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
	}

	.preview-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.preview-thumbnail.placeholder {
		color: #475569;
	}

	.preview-badge {
		position: absolute;
		top: 1rem;
		left: 1rem;
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
		color: white;
	}

	.preview-content {
		padding: 1.5rem;
	}

	.preview-content h4 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.preview-description {
		color: #94a3b8;
		font-size: 0.875rem;
		line-height: 1.5;
		margin-bottom: 1rem;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.preview-price {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.preview-current {
		font-size: 1.5rem;
		font-weight: 700;
		color: #3b82f6;
	}

	.preview-original {
		font-size: 1rem;
		color: #64748b;
		text-decoration: line-through;
	}

	.preview-sale {
		font-size: 1.5rem;
		font-weight: 700;
		color: #10b981;
	}

	.preview-features {
		border-top: 1px solid rgba(148, 163, 184, 0.15);
		padding-top: 1rem;
	}

	.preview-feature {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #cbd5e1;
		font-size: 0.875rem;
		padding: 0.375rem 0;
	}

	.preview-feature :global(svg) {
		color: #10b981;
		flex-shrink: 0;
	}

	.preview-more {
		color: #64748b;
		font-size: 0.8125rem;
		padding: 0.375rem 0;
		font-style: italic;
	}

	.preview-status {
		padding: 0.75rem;
		text-align: center;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
	}

	.preview-status.active {
		background: rgba(16, 185, 129, 0.1);
		color: #34d399;
	}

	/* Product Info */
	.product-info {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		padding: 1.25rem;
	}

	.product-info h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.info-row {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem 0;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.info-row:last-child {
		border-bottom: none;
	}

	.info-label {
		color: #64748b;
		font-size: 0.8125rem;
	}

	.info-value {
		color: #f1f5f9;
		font-size: 0.8125rem;
		font-weight: 500;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 2rem;
	}

	.modal {
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 16px;
		padding: 2rem;
		max-width: 480px;
		width: 100%;
	}

	.modal h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.modal p {
		color: #94a3b8;
		font-size: 0.9375rem;
		line-height: 1.5;
		margin-bottom: 1.5rem;
	}

	.modal strong {
		color: #f1f5f9;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.content-grid {
			grid-template-columns: 1fr;
		}

		.preview-section {
			position: static;
			order: -1;
		}

		.type-selector {
			grid-template-columns: repeat(2, 1fr);
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.header-content {
			flex-direction: column;
			gap: 1rem;
		}
	}
</style>
