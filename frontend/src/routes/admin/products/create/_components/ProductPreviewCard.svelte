<script lang="ts">
	/**
	 * R24-C extraction (2026-05-20): live preview sidebar — thumbnail / badge /
	 * name / description / price / features / status. Pure presentational —
	 * receives `formData`, `pricePreview`, `validFeatures`, and the type's
	 * icon/color as props. No callbacks.
	 *
	 * NOTE: the original page used `previewTypeIcon` twice — once for the
	 * thumbnail-placeholder (`SvelteComponent`) and once for the type badge
	 * (`SvelteComponent_1` — Svelte compiler-generated alias). Folding both
	 * into one local `const Icon = $derived(typeIcon)` here keeps the render
	 * identical without the duplicate-name workaround.
	 */
	import type { Component } from 'svelte';
	import { IconCheck } from '$lib/icons';
	import type { ProductFormData, ProductPricePreview } from './types';

	interface Props {
		formData: ProductFormData;
		pricePreview: ProductPricePreview;
		validFeatures: string[];
		typeIcon: Component<{ size?: number }>;
		typeColor: string;
	}

	let { formData, pricePreview, validFeatures, typeIcon, typeColor }: Props = $props();

	const Icon = $derived(typeIcon);
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
				<Icon size={48} />
			</div>
		{/if}

		<div class="preview-badge" style:background={typeColor}>
			<Icon size={14} />
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
</div>

<style>
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

	@media (max-width: 1023.98px) {
		.preview-section {
			position: static;
			order: -1;
		}
	}
</style>
