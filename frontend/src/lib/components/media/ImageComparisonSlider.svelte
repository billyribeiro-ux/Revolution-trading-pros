<script lang="ts">
	/**
	 * Before/After Image Comparison Slider
	 *
	 * Interactive slider to compare original vs optimized images.
	 * Drag the handle or use keyboard to compare.
	 */

	import { browser } from '$app/environment';

	interface Props {
		beforeSrc: string;
		afterSrc: string;
		beforeLabel?: string;
		afterLabel?: string;
		initialPosition?: number;
		showLabels?: boolean;
		showSizeInfo?: boolean;
		beforeSize?: number;
		afterSize?: number;
	}

	let {
		beforeSrc,
		afterSrc,
		beforeLabel = 'Original',
		afterLabel = 'Optimized',
		initialPosition = 50,
		showLabels = true,
		showSizeInfo = true,
		beforeSize = 0,
		afterSize = 0
	}: Props = $props();

	let container: HTMLElement | null = $state(null);
	let sliderPosition = $state(50);
	let isDragging = $state(false);
	let isLoaded = $state(false);

	$effect(() => {
		sliderPosition = initialPosition;
	});

	// Calculate savings
	let savingsPercent = $derived(
		beforeSize > 0 ? Math.round((1 - afterSize / beforeSize) * 100) : 0
	);
	let savingsBytes = $derived(beforeSize - afterSize);

	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}

	function handleMouseDown(e: MouseEvent) {
		isDragging = true;
		updatePosition(e);
	}

	function handleMouseMove(e: MouseEvent) {
		if (!isDragging) return;
		updatePosition(e);
	}

	function handleMouseUp() {
		isDragging = false;
	}

	function handleTouchStart(e: TouchEvent) {
		isDragging = true;
		updatePositionTouch(e);
	}

	function handleTouchMove(e: TouchEvent) {
		if (!isDragging) return;
		updatePositionTouch(e);
	}

	function handleTouchEnd() {
		isDragging = false;
	}

	function updatePosition(e: MouseEvent) {
		if (!container) return;
		const rect = container.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const percentage = Math.max(0, Math.min(100, (x / rect.width) * 100));
		sliderPosition = percentage;
	}

	function updatePositionTouch(e: TouchEvent) {
		if (!container || !e.touches[0]) return;
		const rect = container.getBoundingClientRect();
		const x = e.touches[0].clientX - rect.left;
		const percentage = Math.max(0, Math.min(100, (x / rect.width) * 100));
		sliderPosition = percentage;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'ArrowLeft') {
			sliderPosition = Math.max(0, sliderPosition - 5);
		} else if (e.key === 'ArrowRight') {
			sliderPosition = Math.min(100, sliderPosition + 5);
		}
	}

	$effect(() => {
		if (!browser) return;

		// Preload images
		const img1 = new Image();
		const img2 = new Image();
		let loaded = 0;

		const checkLoaded = () => {
			loaded++;
			if (loaded === 2) {
				isLoaded = true;
			}
		};

		img1.onload = checkLoaded;
		img2.onload = checkLoaded;
		img1.src = beforeSrc;
		img2.src = afterSrc;

		// Add global mouse listeners for smooth dragging
		const handleGlobalMouseMove = (e: MouseEvent) => handleMouseMove(e);
		const handleGlobalMouseUp = () => handleMouseUp();

		window.addEventListener('mousemove', handleGlobalMouseMove);
		window.addEventListener('mouseup', handleGlobalMouseUp);

		return () => {
			window.removeEventListener('mousemove', handleGlobalMouseMove);
			window.removeEventListener('mouseup', handleGlobalMouseUp);
		};
	});
</script>

<div class="comparison-wrapper">
	<!-- Size info bar -->
	{#if showSizeInfo && beforeSize > 0}
		<div class="size-info-bar">
			<div class="size-item before">
				<span class="label">{beforeLabel}</span>
				<span class="size">{formatBytes(beforeSize)}</span>
			</div>
			<div class="savings" class:positive={savingsPercent > 0}>
				{#if savingsPercent > 0}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 20 20"
						fill="currentColor"
						class="icon"
					>
						<path
							fill-rule="evenodd"
							d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
							clip-rule="evenodd"
						/>
					</svg>
					<span>{savingsPercent}% smaller</span>
					<span class="bytes">({formatBytes(savingsBytes)} saved)</span>
				{:else}
					<span>No change</span>
				{/if}
			</div>
			<div class="size-item after">
				<span class="label">{afterLabel}</span>
				<span class="size">{formatBytes(afterSize)}</span>
			</div>
		</div>
	{/if}

	<!-- Comparison container -->
	<div
		bind:this={container}
		class="comparison-container"
		class:loading={!isLoaded}
		role="slider"
		aria-label="Image comparison slider"
		aria-valuemin={0}
		aria-valuemax={100}
		aria-valuenow={Math.round(sliderPosition)}
		tabindex="0"
		onmousedown={handleMouseDown}
		ontouchstart={handleTouchStart}
		ontouchmove={handleTouchMove}
		ontouchend={handleTouchEnd}
		onkeydown={handleKeydown}
	>
		<!-- Loading spinner -->
		{#if !isLoaded}
			<div class="loading-overlay">
				<div class="spinner"></div>
				<span>Loading images...</span>
			</div>
		{/if}

		<!-- After image (full width, behind) -->
		<div class="image-layer after-layer">
			<img src={afterSrc} alt={afterLabel} draggable="false" />
			{#if showLabels}
				<span class="image-label after-label">{afterLabel}</span>
			{/if}
		</div>

		<!-- Before image (clipped) -->
		<div class="image-layer before-layer" style="clip-path: inset(0 {100 - sliderPosition}% 0 0);">
			<img src={beforeSrc} alt={beforeLabel} draggable="false" />
			{#if showLabels}
				<span class="image-label before-label">{beforeLabel}</span>
			{/if}
		</div>

		<!-- Slider handle -->
		<div class="slider-handle" style="left: {sliderPosition}%;">
			<div class="handle-line"></div>
			<div class="handle-button" class:active={isDragging}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M18 8L22 12L18 16" />
					<path d="M6 8L2 12L6 16" />
				</svg>
			</div>
		</div>
	</div>

	<!-- Keyboard hint -->
	<p class="keyboard-hint">
		<kbd>←</kbd> <kbd>→</kbd> Use arrow keys to adjust
	</p>
</div>

<style>
	.comparison-wrapper {
		width: 100%;
		max-width: 800px;
		margin: 0 auto;
	}

	.size-info-bar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: #f8fafc;
		border-radius: 8px 8px 0 0;
		border: 1px solid #e2e8f0;
		border-bottom: none;
	}

	.size-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.size-item .label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.size-item .size {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1e293b;
	}

	.savings {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #fef2f2;
		border-radius: 9999px;
		font-size: 0.875rem;
		color: #991b1b;
	}

	.savings.positive {
		background: #dcfce7;
		color: #166534;
	}

	.savings .icon {
		width: 1.25rem;
		height: 1.25rem;
	}

	.savings .bytes {
		color: #4ade80;
		font-size: 0.75rem;
	}

	.comparison-container {
		position: relative;
		width: 100%;
		aspect-ratio: 16 / 9;
		overflow: hidden;
		cursor: ew-resize;
		border: 1px solid #e2e8f0;
		border-radius: 0 0 8px 8px;
		background: #1e293b;
		user-select: none;
	}

	.comparison-container:focus {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.comparison-container.loading {
		cursor: wait;
	}

	.loading-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		background: rgba(30, 41, 59, 0.9);
		color: white;
		z-index: 10;
	}

	.spinner {
		width: 2.5rem;
		height: 2.5rem;
		border: 3px solid rgba(255, 255, 255, 0.2);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.image-layer {
		position: absolute;
		inset: 0;
	}

	.image-layer img {
		width: 100%;
		height: 100%;
		object-fit: contain;
		pointer-events: none;
	}

	.image-label {
		position: absolute;
		bottom: 1rem;
		padding: 0.375rem 0.75rem;
		background: rgba(0, 0, 0, 0.75);
		color: white;
		font-size: 0.75rem;
		font-weight: 500;
		border-radius: 4px;
		backdrop-filter: blur(4px);
	}

	.before-label {
		left: 1rem;
	}

	.after-label {
		right: 1rem;
	}

	.slider-handle {
		position: absolute;
		top: 0;
		bottom: 0;
		width: 0;
		transform: translateX(-50%);
		z-index: 5;
	}

	.handle-line {
		position: absolute;
		top: 0;
		bottom: 0;
		left: 50%;
		width: 2px;
		background: white;
		transform: translateX(-50%);
		box-shadow: 0 0 4px rgba(0, 0, 0, 0.3);
	}

	.handle-button {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 3rem;
		height: 3rem;
		background: white;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow:
			0 4px 12px rgba(0, 0, 0, 0.15),
			0 0 0 3px rgba(255, 255, 255, 0.5);
		transition:
			transform 0.15s ease,
			box-shadow 0.15s ease;
	}

	.handle-button:hover,
	.handle-button.active {
		transform: translate(-50%, -50%) scale(1.1);
		box-shadow:
			0 6px 16px rgba(0, 0, 0, 0.2),
			0 0 0 4px rgba(255, 255, 255, 0.7);
	}

	.handle-button svg {
		width: 1.5rem;
		height: 1.5rem;
		color: #475569;
	}

	.keyboard-hint {
		margin-top: 0.75rem;
		text-align: center;
		font-size: 0.75rem;
		color: #64748b;
	}

	.keyboard-hint kbd {
		display: inline-block;
		padding: 0.125rem 0.375rem;
		background: #f1f5f9;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		font-family: inherit;
		font-size: 0.75rem;
	}

	/* Mobile adjustments */
	@media (max-width: 640px) {
		.size-info-bar {
			flex-wrap: wrap;
			gap: 0.5rem;
		}

		.savings {
			order: 3;
			width: 100%;
			justify-content: center;
		}

		.handle-button {
			width: 2.5rem;
			height: 2.5rem;
		}

		.handle-button svg {
			width: 1.25rem;
			height: 1.25rem;
		}
	}
</style>
