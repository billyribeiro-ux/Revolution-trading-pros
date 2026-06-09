<!--
  OptimizedImage Component
  ═══════════════════════════════════════════════════════════════════════════

  High-performance image component with:
  - BlurHash placeholder rendering
  - LQIP (Low Quality Image Placeholder) fallback
  - Progressive loading with smooth fade-in
  - Responsive srcset support
  - WebP/AVIF format negotiation
  - Lazy loading with Intersection Observer

  @version 2.0.0 - Svelte 5 Runes Migration
-->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import Icon from '$lib/components/Icon.svelte';

	// Props - Svelte 5 $props() pattern with interface
	interface Props {
		src: string;
		alt?: string;
		blurhash?: string | null;
		lqip?: string | null;
		width?: number | null;
		height?: number | null;
		srcset?: Record<string, string> | null;
		sizes?: string;
		loading?: 'lazy' | 'eager';
		className?: string;
		aspectRatio?: string | null;
		objectFit?: 'cover' | 'contain' | 'fill' | 'none';
		priority?: boolean;
		onLoad?: (() => void) | null;
		onError?: ((error: Error) => void) | null;
	}

	let {
		src,
		alt = '',
		blurhash = null,
		lqip = null,
		width = null,
		height = null,
		srcset = null,
		sizes = '100vw',
		loading = 'lazy',
		className = '',
		aspectRatio = null,
		objectFit = 'cover',
		priority = false,
		onLoad: onLoadCallback = null,
		onError: onErrorCallback = null
	}: Props = $props();

	// State - Svelte 5 $state() pattern
	let loaded = $state(false);
	let hasError = $state(false);
	let containerRef = $state<HTMLDivElement | null>(null);
	let isInView = $state(false);
	let observer: IntersectionObserver | null = null;
	let blurhashPromise: Promise<typeof import('blurhash')> | null = null;

	function loadBlurhash() {
		blurhashPromise ??= import('blurhash');
		return blurhashPromise;
	}

	function captureContainer(element: HTMLDivElement) {
		containerRef = element;

		return () => {
			if (containerRef === element) {
				containerRef = null;
			}
		};
	}

	// Computed aspect ratio style - Svelte 5 $derived() pattern
	let aspectRatioStyle = $derived(
		aspectRatio
			? `aspect-ratio: ${aspectRatio};`
			: width && height
				? `aspect-ratio: ${width}/${height};`
				: ''
	);

	// Decode BlurHash to canvas
	function renderBlurhash(node: HTMLCanvasElement) {
		if (!blurhash || !node) return;

		void loadBlurhash()
			.then(({ decode }) => {
				if (!node.isConnected || !blurhash) return;

				const pixels = decode(blurhash, 32, 32);
				const ctx = node.getContext('2d');
				if (!ctx) return;

				const imageData = ctx.createImageData(32, 32);
				imageData.data.set(pixels);
				ctx.putImageData(imageData, 0, 0);
			})
			.catch((err: unknown) => {
				console.warn('BlurHash decode failed:', err);
			});
	}

	// Build srcset string
	function buildSrcset(): string | undefined {
		if (!srcset) return undefined;

		return Object.entries(srcset)
			.map(([size, url]) => {
				const w = size.replace(/[^0-9]/g, '');
				return `${url} ${w}w`;
			})
			.join(', ');
	}

	// Handle image load
	function handleLoad() {
		loaded = true;
		onLoadCallback?.();
	}

	// Handle image error
	function handleError(_e: Event) {
		hasError = true;
		onErrorCallback?.(new Error('Image failed to load'));
	}

	// Setup Intersection Observer for lazy loading
	onMount(() => {
		if (priority || loading === 'eager') {
			isInView = true;
			return;
		}

		observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						isInView = true;
						observer?.disconnect();
					}
				});
			},
			{
				rootMargin: '50px',
				threshold: 0.01
			}
		);

		if (containerRef) {
			observer.observe(containerRef);
		}
	});

	onDestroy(() => {
		observer?.disconnect();
	});
</script>

<div
	{@attach captureContainer}
	class={['optimized-image-container', className]}
	style={aspectRatioStyle}
>
	<!-- BlurHash placeholder canvas -->
	{#if blurhash && !loaded}
		<canvas
			{@attach renderBlurhash}
			width="32"
			height="32"
			class={[
				'optimized-image__placeholder',
				'optimized-image__placeholder--blurhash',
				loaded && 'optimized-image__placeholder--hidden'
			]}
			aria-hidden="true"
		></canvas>
	{/if}

	<!-- LQIP fallback -->
	{#if lqip && !blurhash && !loaded}
		<img
			src={lqip}
			alt=""
			class={[
				'optimized-image__placeholder',
				'optimized-image__placeholder--lqip',
				loaded && 'optimized-image__placeholder--hidden'
			]}
			style:object-fit={objectFit}
			aria-hidden="true"
			width={width ?? undefined}
			height={height ?? undefined}
		/>
	{/if}

	<!-- Main image with picture element for format negotiation -->
	{#if isInView || priority}
		<picture>
			<!-- AVIF source (best compression) -->
			{#if srcset?.['avif']}
				<source srcset={srcset['avif']} type="image/avif" />
			{/if}

			<!-- WebP source (good compression, wide support) -->
			{#if srcset?.['webp']}
				<source srcset={srcset['webp']} type="image/webp" />
			{/if}

			<!-- Responsive srcset -->
			{#if srcset && !srcset['avif'] && !srcset['webp']}
				<source srcset={buildSrcset()} {sizes} />
			{/if}

			<!-- Fallback image -->
			<img
				{src}
				{alt}
				{width}
				{height}
				loading={priority ? 'eager' : loading}
				decoding={priority ? 'sync' : 'async'}
				fetchpriority={priority ? 'high' : 'auto'}
				onload={handleLoad}
				onerror={handleError}
				class={['optimized-image__image', !loaded && 'optimized-image__image--hidden']}
				style:object-fit={objectFit}
			/>
		</picture>
	{/if}

	<!-- Error state -->
	{#if hasError}
		<div class="optimized-image__error">
			<div class="optimized-image__error-content">
				<Icon name="IconPhoto" size={48} class="optimized-image__error-icon" />
				<p>Failed to load image</p>
			</div>
		</div>
	{/if}

	<!-- Loading skeleton (if no placeholder available) -->
	{#if !blurhash && !lqip && !loaded && !hasError && isInView}
		<div class="optimized-image__skeleton"></div>
	{/if}
</div>

<style>
	.optimized-image-container {
		position: relative;
		overflow: hidden;
		background-color: transparent;
	}

	/* Smooth blur-to-sharp transition */
	.optimized-image__placeholder--blurhash {
		filter: blur(24px);
		image-rendering: pixelated;
		image-rendering: crisp-edges;
		transform: scale(1.1);
	}

	.optimized-image__placeholder--lqip {
		filter: blur(16px);
		transform: scale(1.05);
	}

	.optimized-image__placeholder,
	.optimized-image__image,
	.optimized-image__error,
	.optimized-image__skeleton {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
	}

	.optimized-image__placeholder,
	.optimized-image__image {
		transition: opacity 0.5s ease;
	}

	.optimized-image__placeholder--hidden,
	.optimized-image__image--hidden {
		opacity: 0;
	}

	.optimized-image__image {
		color: transparent;
	}

	.optimized-image__error {
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f3f4f6;
	}

	:global(.dark) .optimized-image__error {
		background: #1f2937;
	}

	.optimized-image__error-content {
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.25rem;
		text-align: center;
	}

	.optimized-image__error-content p {
		margin: 0;
	}

	:global(.dark) .optimized-image__error-content {
		color: #9ca3af;
	}

	.optimized-image-container :global(.optimized-image__error-icon) {
		margin: 0 auto 0.5rem;
	}

	.optimized-image__skeleton {
		background: #e5e7eb;
		animation: optimized-image-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	:global(.dark) .optimized-image__skeleton {
		background: #374151;
	}

	@keyframes optimized-image-pulse {
		50% {
			opacity: 0.5;
		}
	}
</style>
