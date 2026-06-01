<!--
	URL: /admin/products/[id]/edit
	Product Edit Page - Svelte 5 Runes Implementation

	R20-C extraction (2026-05-20): split 1473-LOC form page into 8 leaf
	components under `_components/`. Parent is now the orchestrator only:
	state, fetch, mutation handlers, derived previews, validation glue.
	Mirrors the R18-C / R19-C pattern (indicators/[id], courses/[id]).
-->

<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { untrack } from 'svelte';
	import { productsApi, AdminApiError, type Product } from '$lib/api/admin';
	import {
		IconCheck,
		IconBook,
		IconChartLine,
		IconCrown,
		IconShoppingCart,
		IconX
	} from '$lib/icons';
	import ProductHeader from './_components/ProductHeader.svelte';
	import ProductTypeSelector from './_components/ProductTypeSelector.svelte';
	import ProductBasicFields from './_components/ProductBasicFields.svelte';
	import ProductPricingFields from './_components/ProductPricingFields.svelte';
	import ProductFeatureList from './_components/ProductFeatureList.svelte';
	import ProductSeoSettings from './_components/ProductSeoSettings.svelte';
	import ProductPreviewCard from './_components/ProductPreviewCard.svelte';
	import DeleteProductModal from './_components/DeleteProductModal.svelte';

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
	<ProductHeader
		productName={originalProduct?.name ?? null}
		{deleting}
		onBack={() => goto('/admin/products')}
		onDelete={() => (showDeleteConfirm = true)}
	/>

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

					<ProductTypeSelector
						value={formData.type}
						types={productTypes}
						onChange={(next) => (formData.type = next)}
					/>

					<ProductBasicFields bind:formData {getFieldError} />

					<ProductPricingFields bind:formData {getFieldError} />

					<ProductFeatureList
						features={formData.features}
						onAdd={addFeature}
						onRemove={removeFeature}
						onUpdate={updateFeature}
					/>

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

					<ProductSeoSettings bind:formData />

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
			<ProductPreviewCard
				formData={{
					name: formData.name,
					description: formData.description,
					thumbnail: formData.thumbnail,
					type: formData.type,
					is_active: formData.is_active
				}}
				{previewTypeIcon}
				{previewTypeColor}
				{pricePreview}
				{validFeatures}
				{originalProduct}
			/>
		</div>
	{/if}

	<DeleteProductModal
		open={showDeleteConfirm}
		productName={originalProduct?.name ?? ''}
		{deleting}
		onConfirm={deleteProduct}
		onCancel={() => (showDeleteConfirm = false)}
	/>
</div>

<style>
	.edit-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
		background: #0f172a;
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
	.btn-secondary {
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

	.btn-spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Responsive */
	@media (max-width: 1023.98px) {
		.content-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
