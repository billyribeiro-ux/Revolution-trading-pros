<!--
  ResponsivePreview Component
  ═══════════════════════════════════════════════════════════════════════════

  Preview image at different responsive breakpoints:
  - Shows all 6 responsive sizes side by side
  - File size comparison for each variant
  - Interactive selection
  - Zoom preview on hover

  @version 2.0.0
-->
<script lang="ts">
	import { fade, scale } from 'svelte/transition';

	interface Variant {
		sizeName: string;
		width: number;
		height: number;
		url: string;
		size: number;
	}

	interface Props {
		variants?: Variant[];
		originalSize?: number;
		selectedSize?: string | null;
		showSizes?: boolean;
		interactive?: boolean;
		className?: string;
		onSelect?: (sizeName: string) => void;
	}

	let {
		variants = [],
		originalSize = 0,
		selectedSize = $bindable(null),
		showSizes = true,
		interactive = true,
		className = '',
		onSelect
	}: Props = $props();

	// Breakpoint labels
	const breakpointLabels: Record<string, string> = {
		xs: 'Mobile S',
		sm: 'Mobile L',
		md: 'Tablet',
		lg: 'Laptop',
		xl: 'Desktop',
		'2xl': 'Large'
	};

	// Breakpoint widths
	const breakpointWidths: Record<string, number> = {
		xs: 320,
		sm: 640,
		md: 768,
		lg: 1024,
		xl: 1280,
		'2xl': 1920
	};

	// State
	let hoveredVariant = $state<Variant | null>(null);
	let previewPosition = $state({ x: 0, y: 0 });

	// Derived state
	const sortedVariants = $derived([...variants].sort((a, b) => a.width - b.width));

	// Format bytes
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
	}

	// Get savings percentage
	function getSavings(variantSize: number): number {
		if (!originalSize || originalSize === 0) return 0;
		return Math.round((1 - variantSize / originalSize) * 100);
	}

	// Handle variant selection
	function handleSelect(sizeName: string) {
		if (!interactive) return;
		selectedSize = sizeName;
		onSelect?.(sizeName);
	}

	// Handle hover preview
	function handleMouseEnter(e: MouseEvent, variant: Variant) {
		if (!interactive) return;
		hoveredVariant = variant;
		updatePreviewPosition(e);
	}

	function handleMouseMove(e: MouseEvent) {
		if (hoveredVariant) {
			updatePreviewPosition(e);
		}
	}

	function handleMouseLeave() {
		hoveredVariant = null;
	}

	function updatePreviewPosition(e: MouseEvent) {
		const rect = (e.target as HTMLElement).getBoundingClientRect();
		previewPosition = {
			x: rect.right + 10,
			y: rect.top
		};
	}
</script>

<div class={['responsive-preview', className]}>
	<!-- Header -->
	<div class="responsive-preview__header">
		<h3 class="responsive-preview__title">Responsive Variants</h3>
		{#if originalSize > 0}
			<span class="responsive-preview__original-size">
				Original: {formatBytes(originalSize)}
			</span>
		{/if}
	</div>

	<!-- Variants grid -->
	<div class="responsive-preview__grid">
		{#each sortedVariants as variant (variant.sizeName)}
			<button
				class={[
					'responsive-preview__variant',
					selectedSize === variant.sizeName && 'responsive-preview__variant--selected',
					interactive && 'responsive-preview__variant--interactive'
				]}
				onclick={() => handleSelect(variant.sizeName)}
				onmouseenter={(e: MouseEvent) => handleMouseEnter(e, variant)}
				onmousemove={handleMouseMove}
				onmouseleave={handleMouseLeave}
				transition:scale={{ duration: 200 }}
			>
				<!-- Thumbnail -->
				<div class="responsive-preview__thumbnail">
					<img
						src={variant.url}
						alt="{variant.sizeName} preview"
						loading="lazy"
						class="responsive-preview__thumbnail-image"
						width={variant.width}
						height={variant.height}
					/>
				</div>

				<!-- Info -->
				<div class="responsive-preview__variant-info">
					<div class="responsive-preview__variant-label">
						{breakpointLabels[variant.sizeName] || variant.sizeName}
					</div>
					<div class="responsive-preview__variant-dimensions">
						{variant.width} x {variant.height}
					</div>
					{#if showSizes}
						<div class="responsive-preview__variant-stats">
							<span>{formatBytes(variant.size)}</span>
							{#if originalSize > 0}
								<span class="responsive-preview__savings">-{getSavings(variant.size)}%</span>
							{/if}
						</div>
					{/if}
				</div>

				<!-- Breakpoint indicator -->
				<div class="responsive-preview__breakpoint-track">
					<div
						class="responsive-preview__breakpoint-fill"
						style:width={`${((breakpointWidths[variant.sizeName] ?? 0) / 1920) * 100}%`}
					></div>
				</div>
			</button>
		{/each}
	</div>

	<!-- Size comparison chart -->
	{#if showSizes && sortedVariants.length > 0}
		<div class="responsive-preview__comparison">
			<div class="responsive-preview__comparison-heading">
				<span>Size Comparison</span>
			</div>
			<div class="responsive-preview__chart">
				{#each sortedVariants as variant (variant.sizeName)}
					<div class="responsive-preview__chart-item">
						<div
							class="responsive-preview__chart-bar responsive-preview__chart-bar--variant"
							style:height={`${(variant.size / originalSize) * 100}%`}
						></div>
						<span>{variant.sizeName}</span>
					</div>
				{/each}
				<div class="responsive-preview__chart-item">
					<div
						class="responsive-preview__chart-bar responsive-preview__chart-bar--original"
						style:height="100%"
					></div>
					<span>orig</span>
				</div>
			</div>
		</div>
	{/if}

	<!-- Hover preview tooltip -->
	{#if hoveredVariant}
		<div
			class="responsive-preview__tooltip"
			style:left={`${previewPosition.x}px`}
			style:top={`${previewPosition.y}px`}
			transition:fade={{ duration: 150 }}
		>
			<img
				src={hoveredVariant.url}
				alt="Preview"
				class="responsive-preview__tooltip-image"
				width={hoveredVariant.width}
				height={hoveredVariant.height}
				loading="lazy"
			/>
			<div class="responsive-preview__tooltip-meta">
				<span class="responsive-preview__tooltip-dimensions"
					>{hoveredVariant.width}x{hoveredVariant.height}</span
				>
				<span class="responsive-preview__tooltip-size">{formatBytes(hoveredVariant.size)}</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.responsive-preview {
		border-radius: 0.75rem;
		background: #f9fafb;
		padding: 1rem;
	}

	:global(.dark) .responsive-preview {
		background: rgb(31 41 55 / 0.5);
	}

	.responsive-preview__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.responsive-preview__title {
		margin: 0;
		color: #374151;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25rem;
	}

	:global(.dark) .responsive-preview__title {
		color: #d1d5db;
	}

	.responsive-preview__original-size,
	.responsive-preview__comparison-heading,
	.responsive-preview__variant-label,
	.responsive-preview__variant-dimensions,
	.responsive-preview__variant-stats,
	.responsive-preview__chart-item,
	.responsive-preview__tooltip-meta {
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.responsive-preview__original-size,
	.responsive-preview__comparison-heading,
	.responsive-preview__variant-stats,
	.responsive-preview__chart-item,
	.responsive-preview__tooltip-size {
		color: #6b7280;
	}

	.responsive-preview__grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.75rem;
	}

	.responsive-preview__variant {
		overflow: hidden;
		border: 2px solid transparent;
		border-radius: 0.5rem;
		background: #ffffff;
		padding: 0;
		text-align: inherit;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	:global(.dark) .responsive-preview__variant {
		background: #1f2937;
	}

	.responsive-preview__variant--interactive {
		cursor: pointer;
	}

	.responsive-preview__variant--interactive:hover {
		border-color: #93c5fd;
		box-shadow:
			0 4px 6px -1px rgb(0 0 0 / 0.1),
			0 2px 4px -2px rgb(0 0 0 / 0.1);
	}

	:global(.dark) .responsive-preview__variant--interactive:hover {
		border-color: #2563eb;
	}

	.responsive-preview__variant--selected {
		border-color: #3b82f6;
		box-shadow: 0 0 0 2px #bfdbfe;
	}

	:global(.dark) .responsive-preview__variant--selected {
		box-shadow: 0 0 0 2px #1e40af;
	}

	.responsive-preview__thumbnail {
		aspect-ratio: 1;
		overflow: hidden;
		background: #f3f4f6;
	}

	:global(.dark) .responsive-preview__thumbnail {
		background: #374151;
	}

	.responsive-preview__thumbnail-image {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.responsive-preview__variant-info {
		padding: 0.5rem;
		text-align: center;
	}

	.responsive-preview__variant-label {
		color: #374151;
		font-weight: 500;
	}

	:global(.dark) .responsive-preview__variant-label {
		color: #d1d5db;
	}

	.responsive-preview__variant-dimensions {
		color: #6b7280;
		font-size: 0.625rem;
	}

	:global(.dark) .responsive-preview__variant-dimensions {
		color: #9ca3af;
	}

	.responsive-preview__variant-stats {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 0.25rem;
		font-size: 0.625rem;
	}

	.responsive-preview__savings {
		color: #22c55e;
		font-weight: 500;
	}

	.responsive-preview__breakpoint-track {
		height: 0.25rem;
		background: #e5e7eb;
	}

	:global(.dark) .responsive-preview__breakpoint-track {
		background: #374151;
	}

	.responsive-preview__breakpoint-fill {
		height: 100%;
		background: #3b82f6;
	}

	.responsive-preview__comparison {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
	}

	:global(.dark) .responsive-preview__comparison {
		border-top-color: #374151;
	}

	.responsive-preview__comparison-heading {
		margin-bottom: 0.5rem;
	}

	.responsive-preview__chart {
		display: flex;
		align-items: end;
		justify-content: center;
		gap: 0.5rem;
		height: 5rem;
	}

	.responsive-preview__chart-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.625rem;
	}

	.responsive-preview__chart-bar {
		width: 1rem;
		border-radius: 0.25rem 0.25rem 0 0;
		transition: height 0.3s ease;
	}

	.responsive-preview__chart-bar--variant {
		background: #3b82f6;
	}

	.responsive-preview__chart-bar--original {
		background: #9ca3af;
	}

	:global(.dark) .responsive-preview__chart-bar--original {
		background: #6b7280;
	}

	.responsive-preview__tooltip {
		position: fixed;
		z-index: 50;
		overflow: hidden;
		pointer-events: none;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		background: #ffffff;
		box-shadow:
			0 20px 25px -5px rgb(0 0 0 / 0.1),
			0 8px 10px -6px rgb(0 0 0 / 0.1);
	}

	:global(.dark) .responsive-preview__tooltip {
		border-color: #374151;
		background: #1f2937;
	}

	.responsive-preview__tooltip-image {
		max-width: 200px;
		max-height: 150px;
		object-fit: contain;
	}

	.responsive-preview__tooltip-meta {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		padding: 0.25rem 0.5rem;
		background: #f9fafb;
	}

	:global(.dark) .responsive-preview__tooltip-meta {
		background: #111827;
	}

	.responsive-preview__tooltip-dimensions {
		font-weight: 500;
	}

	@media (min-width: 640px) {
		.responsive-preview__grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (min-width: 768px) {
		.responsive-preview__grid {
			grid-template-columns: repeat(6, minmax(0, 1fr));
		}
	}
</style>
