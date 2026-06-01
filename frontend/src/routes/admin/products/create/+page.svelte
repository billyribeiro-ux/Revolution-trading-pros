<!--
	URL: /admin/products/create
	Product Creation Page — Svelte 5 Runes Implementation.

	R24-C refactor (2026-05-20): the layout was a single 1128-LOC mega-file
	with 9 visually distinct UI sections inlined. Each section was lifted
	into its own component under `_components/` so this file is now just
	state + orchestration. The save handler, validation rules, and the
	`productTypes` table still live here — those are the parent's
	responsibility (props down, callbacks up).
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import { productsApi, AdminApiError, type Product } from '$lib/api/admin';
	import { IconBook, IconChartLine, IconCrown, IconShoppingCart, IconX } from '$lib/icons';

	import ProductPageHeader from './_components/ProductPageHeader.svelte';
	import ProductTypeSelector from './_components/ProductTypeSelector.svelte';
	import ProductBasicFields from './_components/ProductBasicFields.svelte';
	import ProductPricingFields from './_components/ProductPricingFields.svelte';
	import ProductFeaturesEditor from './_components/ProductFeaturesEditor.svelte';
	import ProductSeoSettings from './_components/ProductSeoSettings.svelte';
	import ProductFormActions from './_components/ProductFormActions.svelte';
	import ProductPreviewCard from './_components/ProductPreviewCard.svelte';
	import type {
		ProductFormData,
		ProductPricePreview,
		ProductType,
		ProductTypeOption
	} from './_components/types';

	// Form state using Svelte 5 $state rune
	let formData = $state<ProductFormData>({
		name: '',
		slug: '',
		type: 'course',
		description: '',
		long_description: '',
		price: '',
		sale_price: '',
		currency: 'USD',
		features: [''],
		is_active: true,
		thumbnail: '',
		meta_title: '',
		meta_description: '',
		indexable: true,
		canonical_url: ''
	});

	// UI state
	let saving = $state(false);
	let error = $state('');
	let validationErrors = $state<Record<string, string[]>>({});

	// Product type options
	const productTypes: readonly ProductTypeOption[] = [
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

	// Derived state - formatted price preview
	let pricePreview = $derived.by<ProductPricePreview>(() => {
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
		productTypes.find((t) => t.value === formData.type)?.icon ?? IconShoppingCart
	);
	let previewTypeColor = $derived(
		productTypes.find((t) => t.value === formData.type)?.color ?? '#64748b'
	);

	// Generate slug from name
	function generateSlug() {
		if (!formData.slug && formData.name) {
			formData.slug = formData.name
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
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

			await productsApi.create(productData);
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
				error = 'Failed to create product';
			}
			console.error('Failed to create product:', err);
		} finally {
			saving = false;
		}
	}

	// Get field error
	function getFieldError(field: string): string | undefined {
		return validationErrors[field]?.[0];
	}

	function selectType(value: ProductType) {
		formData.type = value;
	}

	function goBack() {
		goto('/admin/products');
	}
</script>

<svelte:head>
	<title>Create Product | Admin</title>
</svelte:head>

<div class="create-page">
	<ProductPageHeader onBack={goBack} />

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
				<h3>Product Details</h3>

				<ProductTypeSelector {productTypes} selected={formData.type} onSelect={selectType} />

				<ProductBasicFields bind:formData {getFieldError} onSlugBlur={generateSlug} />

				<ProductPricingFields bind:formData {getFieldError} />

				<ProductFeaturesEditor
					features={formData.features}
					onAdd={addFeature}
					onUpdate={updateFeature}
					onRemove={removeFeature}
				/>

				<!-- Active Status -->
				<div class="form-group">
					<label class="checkbox-label">
						<input
							id="is-active"
							name="is-active"
							type="checkbox"
							bind:checked={formData.is_active}
						/>
						<span>Active (visible to customers)</span>
					</label>
				</div>

				<ProductSeoSettings bind:formData />

				<ProductFormActions
					{saving}
					disabled={saving || !isFormValid}
					onCancel={goBack}
					onSave={saveProduct}
				/>
			</div>
		</div>

		<ProductPreviewCard
			{formData}
			{pricePreview}
			{validFeatures}
			typeIcon={previewTypeIcon}
			typeColor={previewTypeColor}
		/>
	</div>
</div>

<style>
	.create-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
		background: #0f172a;
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

	.form-card h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1.5rem;
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

	/* Responsive */
	@media (max-width: 1023.98px) {
		.content-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
