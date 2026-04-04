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

<div class="rp-container {className}">
	<!-- Header -->
	<div class="rp-header">
		<h3 class="rp-title">Responsive Variants</h3>
		{#if originalSize > 0}
			<span class="rp-original">Original: {formatBytes(originalSize)}</span>
		{/if}
	</div>

	<!-- Variants grid -->
	<div class="rp-grid">
		{#each sortedVariants as variant (variant.sizeName)}
			<button
				class="rp-variant"
				data-selected={selectedSize === variant.sizeName || undefined}
				data-interactive={interactive || undefined}
				onclick={() => handleSelect(variant.sizeName)}
				onmouseenter={(e: MouseEvent) => handleMouseEnter(e, variant)}
				onmousemove={handleMouseMove}
				onmouseleave={handleMouseLeave}
				transition:scale={{ duration: 200 }}
			>
				<!-- Thumbnail -->
				<div class="rp-thumb">
					<img
						src={variant.url}
						alt="{variant.sizeName} preview"
						loading="lazy"
						class="rp-thumb-img"
					/>
				</div>

				<!-- Info -->
				<div class="rp-info">
					<div class="rp-label">
						{breakpointLabels[variant.sizeName] || variant.sizeName}
					</div>
					<div class="rp-dims">
						{variant.width} x {variant.height}
					</div>
					{#if showSizes}
						<div class="rp-sizes">
							<span class="rp-size-val">{formatBytes(variant.size)}</span>
							{#if originalSize > 0}
								<span class="rp-savings">-{getSavings(variant.size)}%</span>
							{/if}
						</div>
					{/if}
				</div>

				<!-- Breakpoint indicator -->
				<div class="rp-bp-track">
					<div
						class="rp-bp-fill"
						style="width: {((breakpointWidths[variant.sizeName] ?? 0) / 1920) * 100}%"
					></div>
				</div>
			</button>
		{/each}
	</div>

	<!-- Size comparison chart -->
	{#if showSizes && sortedVariants.length > 0}
		<div class="rp-chart-section">
			<div class="rp-chart-label">
				<span>Size Comparison</span>
			</div>
			<div class="rp-chart">
				{#each sortedVariants as variant (variant.sizeName)}
					<div class="rp-bar-col">
						<div
							class="rp-bar"
							style="height: {(variant.size / originalSize) * 100}%"
						></div>
						<span class="rp-bar-label">{variant.sizeName}</span>
					</div>
				{/each}
				<div class="rp-bar-col">
					<div class="rp-bar rp-bar-orig" style="height: 100%"></div>
					<span class="rp-bar-label">orig</span>
				</div>
			</div>
		</div>
	{/if}

	<!-- Hover preview tooltip -->
	{#if hoveredVariant}
		<div
			class="rp-tooltip"
			style="left: {previewPosition.x}px; top: {previewPosition.y}px;"
			transition:fade={{ duration: 150 }}
		>
			<img
				src={hoveredVariant.url}
				alt="Preview"
				class="rp-tooltip-img"
			/>
			<div class="rp-tooltip-footer">
				<span class="rp-tooltip-dim">{hoveredVariant.width}x{hoveredVariant.height}</span>
				<span class="rp-tooltip-size">{formatBytes(hoveredVariant.size)}</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.rp-container {
		background-color: oklch(0.97 0.002 265);
		border-radius: var(--radius-xl);
		padding: var(--space-4);
	}

	.rp-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-4);
	}

	.rp-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(0.35 0.01 265);
	}

	.rp-original { font-size: var(--text-xs); color: oklch(0.55 0.01 265); }

	.rp-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-3);

		@media (min-width: 640px) { grid-template-columns: repeat(3, 1fr); }
		@media (min-width: 768px) { grid-template-columns: repeat(6, 1fr); }
	}

	.rp-variant {
		background-color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		overflow: hidden;
		border: 2px solid transparent;
		transition: all 200ms var(--ease-default);
		cursor: default;
		padding: 0;

		&[data-selected] {
			border-color: oklch(0.6 0.2 260);
			box-shadow: 0 0 0 2px oklch(0.8 0.1 260);
		}

		&[data-interactive] {
			cursor: pointer;

			&:hover {
				border-color: oklch(0.75 0.12 260);
				box-shadow: 0 4px 12px oklch(0 0 0 / 8%);
			}
		}
	}

	.rp-thumb {
		aspect-ratio: 1 / 1;
		background-color: oklch(0.95 0.002 265);
		overflow: hidden;
	}

	.rp-thumb-img {
		inline-size: 100%;
		block-size: 100%;
		object-fit: cover;
	}

	.rp-info {
		padding: var(--space-2);
		text-align: center;
	}

	.rp-label {
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: oklch(0.35 0.01 265);
	}

	.rp-dims { font-size: 0.625rem; color: oklch(0.55 0.01 265); }

	.rp-sizes {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		margin-block-start: var(--space-1);
	}

	.rp-size-val { font-size: 0.625rem; color: oklch(0.55 0.01 265); }
	.rp-savings { font-size: 0.625rem; font-weight: var(--weight-medium); color: oklch(0.55 0.18 160); }

	.rp-bp-track {
		block-size: 0.25rem;
		background-color: oklch(0.9 0.005 265);
	}

	.rp-bp-fill {
		block-size: 100%;
		background-color: oklch(0.6 0.2 260);
	}

	/* ─── Chart ─── */
	.rp-chart-section {
		margin-block-start: var(--space-4);
		padding-block-start: var(--space-4);
		border-block-start: 1px solid oklch(0.9 0.005 265);
	}

	.rp-chart-label {
		margin-block-end: var(--space-2);
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	.rp-chart {
		display: flex;
		align-items: flex-end;
		justify-content: center;
		gap: var(--space-2);
		block-size: 5rem;
	}

	.rp-bar-col {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-1);
	}

	.rp-bar {
		inline-size: 1rem;
		background-color: oklch(0.6 0.2 260);
		border-start-start-radius: var(--radius-sm);
		border-start-end-radius: var(--radius-sm);
		transition: all 300ms var(--ease-default);

		&.rp-bar-orig { background-color: oklch(0.65 0.01 265); }
	}

	.rp-bar-label { font-size: 0.625rem; color: oklch(0.55 0.01 265); }

	/* ─── Tooltip ─── */
	.rp-tooltip {
		position: fixed;
		z-index: 50;
		background-color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		box-shadow: 0 10px 30px oklch(0 0 0 / 15%);
		border: 1px solid oklch(0.9 0.005 265);
		overflow: hidden;
		pointer-events: none;
	}

	.rp-tooltip-img {
		max-inline-size: 200px;
		max-block-size: 150px;
		object-fit: contain;
	}

	.rp-tooltip-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
		padding-inline: var(--space-2);
		padding-block: var(--space-1);
		font-size: var(--text-xs);
		background-color: oklch(0.97 0.002 265);
	}

	.rp-tooltip-dim { font-weight: var(--weight-medium); }
	.rp-tooltip-size { color: oklch(0.55 0.01 265); }
</style>
