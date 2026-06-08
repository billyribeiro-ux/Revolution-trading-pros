<script lang="ts">
	/**
	 * R20-C extraction (2026-05-20): right-rail product preview card +
	 * product-info block. Receives parent-computed derived data so this
	 * component stays a dumb renderer.
	 */
	import { IconCheck, IconShoppingCart } from '$lib/icons';
	import type { Component } from 'svelte';
	import type { Product } from '$lib/api/admin';

	type ProductType = 'course' | 'indicator' | 'membership' | 'bundle';

	interface PreviewFormData {
		name: string;
		description: string;
		thumbnail: string;
		type: ProductType;
		is_active: boolean;
	}

	interface PricePreview {
		original: string;
		sale: string | null;
	}

	interface Props {
		formData: PreviewFormData;
		previewTypeIcon: Component<{ size?: number }>;
		previewTypeColor: string;
		pricePreview: PricePreview;
		validFeatures: string[];
		originalProduct: Product | null;
	}

	let {
		formData,
		previewTypeIcon,
		previewTypeColor,
		pricePreview,
		validFeatures,
		originalProduct
	}: Props = $props();

	let PreviewTypeIcon = $derived(previewTypeIcon ?? IconShoppingCart);
</script>

<div class="preview-section">
	<h3>Preview</h3>
	<div class="product-preview-card">
		{#if formData.thumbnail}
			<div class="preview-thumbnail">
				<img
					src={formData.thumbnail}
					alt={formData.name || 'Product'}
					width="360"
					height="200"
					loading="lazy"
				/>
			</div>
		{:else}
			<div class="preview-thumbnail placeholder">
				<PreviewTypeIcon size={48} />
			</div>
		{/if}

		<div class="preview-badge" style:background={previewTypeColor}>
			<PreviewTypeIcon size={14} />
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
					{#each validFeatures.slice(0, 4) as feature (feature)}
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

		<div class={['preview-status', { active: formData.is_active }]}>
			{formData.is_active ? 'Active' : 'Inactive'}
		</div>
	</div>

	{#if originalProduct}
		<div class="product-info">
			<h4>Product Info</h4>
			<div class="info-row">
				<span class="info-label">ID:</span>
				<span class="info-value">{originalProduct.id}</span>
			</div>
			<div class="info-row">
				<span class="info-label">Created:</span>
				<span class="info-value">{new Date(originalProduct.created_at).toLocaleDateString()}</span>
			</div>
			<div class="info-row">
				<span class="info-label">Updated:</span>
				<span class="info-value">{new Date(originalProduct.updated_at).toLocaleDateString()}</span>
			</div>
		</div>
	{/if}
</div>

<style>
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

	@media (max-width: 1023.98px) {
		.preview-section {
			position: static;
			order: -1;
		}
	}
</style>
