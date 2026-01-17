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

<div class="bg-gray-50 dark:bg-gray-800/50 rounded-xl p-4 {className}">
	<!-- Header -->
	<div class="flex items-center justify-between mb-4">
		<h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">Responsive Variants</h3>
		{#if originalSize > 0}
			<span class="text-xs text-gray-500">
				Original: {formatBytes(originalSize)}
			</span>
		{/if}
	</div>

	<!-- Variants grid -->
	<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-3">
		{#each sortedVariants as variant (variant.sizeName)}
			<button
				class="bg-white dark:bg-gray-800 rounded-lg overflow-hidden border-2 transition-all duration-200 {selectedSize ===
				variant.sizeName
					? 'border-blue-500 ring-2 ring-blue-200 dark:ring-blue-800'
					: 'border-transparent'} {interactive
					? 'cursor-pointer hover:border-blue-300 dark:hover:border-blue-600 hover:shadow-md'
					: ''}"
				onclick={() => handleSelect(variant.sizeName)}
				onmouseenter={(e: MouseEvent) => handleMouseEnter(e, variant)}
				onmousemove={handleMouseMove}
				onmouseleave={handleMouseLeave}
				transition:scale={{ duration: 200 }}
			>
				<!-- Thumbnail -->
				<div class="aspect-square bg-gray-100 dark:bg-gray-700 overflow-hidden">
					<img
						src={variant.url}
						alt="{variant.sizeName} preview"
						loading="lazy"
						class="w-full h-full object-cover"
					/>
				</div>

				<!-- Info -->
				<div class="p-2 text-center">
					<div class="text-xs font-medium text-gray-700 dark:text-gray-300">
						{breakpointLabels[variant.sizeName] || variant.sizeName}
					</div>
					<div class="text-[10px] text-gray-500 dark:text-gray-400">
						{variant.width} x {variant.height}
					</div>
					{#if showSizes}
						<div class="flex items-center justify-center gap-2 mt-1">
							<span class="text-[10px] text-gray-500">{formatBytes(variant.size)}</span>
							{#if originalSize > 0}
								<span class="text-[10px] font-medium text-green-500"
									>-{getSavings(variant.size)}%</span
								>
							{/if}
						</div>
					{/if}
				</div>

				<!-- Breakpoint indicator -->
				<div class="h-1 bg-gray-200 dark:bg-gray-700">
					<div
						class="h-full bg-blue-500"
						style="width: {((breakpointWidths[variant.sizeName] ?? 0) / 1920) * 100}%"
					></div>
				</div>
			</button>
		{/each}
	</div>

	<!-- Size comparison chart -->
	{#if showSizes && sortedVariants.length > 0}
		<div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
			<div class="mb-2">
				<span class="text-xs text-gray-500">Size Comparison</span>
			</div>
			<div class="flex items-end justify-center gap-2 h-20">
				{#each sortedVariants as variant}
					<div class="flex flex-col items-center gap-1">
						<div
							class="w-4 bg-blue-500 rounded-t transition-all duration-300"
							style="height: {(variant.size / originalSize) * 100}%"
						></div>
						<span class="text-[10px] text-gray-500">{variant.sizeName}</span>
					</div>
				{/each}
				<div class="flex flex-col items-center gap-1">
					<div
						class="w-4 bg-gray-400 dark:bg-gray-500 rounded-t transition-all duration-300"
						style="height: 100%"
					></div>
					<span class="text-[10px] text-gray-500">orig</span>
				</div>
			</div>
		</div>
	{/if}

	<!-- Hover preview tooltip -->
	{#if hoveredVariant}
		<div
			class="fixed z-50 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 overflow-hidden pointer-events-none"
			style="left: {previewPosition.x}px; top: {previewPosition.y}px;"
			transition:fade={{ duration: 150 }}
		>
			<img
				src={hoveredVariant.url}
				alt="Preview"
				class="max-w-[200px] max-h-[150px] object-contain"
			/>
			<div
				class="flex items-center justify-between gap-2 px-2 py-1 text-xs bg-gray-50 dark:bg-gray-900"
			>
				<span class="font-medium">{hoveredVariant.width}x{hoveredVariant.height}</span>
				<span class="text-gray-500">{formatBytes(hoveredVariant.size)}</span>
			</div>
		</div>
	{/if}
</div>
