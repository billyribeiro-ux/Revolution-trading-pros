<script lang="ts">
	/**
	 * BlurHashImage - Svelte 5 Component
	 * Progressive image loading with BlurHash placeholder
	 * Includes automatic responsive srcset generation
	 *
	 * Uses Svelte 5 runes ($state, $derived, $effect)
	 */

	import Icon from '$lib/components/Icon.svelte';

	interface Props {
		src: string;
		alt: string;
		blurhash?: string | null;
		width?: number | string;
		height?: number | string;
		class?: string;
		loading?: 'lazy' | 'eager';
		decoding?: 'async' | 'sync' | 'auto';
		sizes?: string;
		srcset?: string;
		/** Enable automatic srcset generation for responsive images */
		autoSrcset?: boolean;
		/** Custom breakpoints for srcset generation (default: [320, 640, 768, 1024, 1280, 1536]) */
		breakpoints?: number[];
		/** Image quality for srcset variants (1-100) */
		quality?: number;
	}

	let props: Props = $props();
	let src = $derived(props.src);
	let alt = $derived(props.alt);
	let blurhash = $derived(props.blurhash ?? null);
	let width = $derived(props.width ?? '100%');
	let height = $derived(props.height ?? 'auto');
	let className = $derived(props.class ?? '');
	let loading = $derived(props.loading ?? 'lazy');
	let decoding = $derived(props.decoding ?? 'async');
	let sizes = $derived(props.sizes);
	let srcset = $derived(props.srcset);
	let autoSrcset = $derived(props.autoSrcset ?? false);
	let breakpoints = $derived(props.breakpoints ?? [320, 640, 768, 1024, 1280, 1536]);
	let quality = $derived(props.quality ?? 80);

	/**
	 * Generate responsive srcset from base URL
	 * Supports Cloudflare R2 and common image CDN patterns
	 */
	function generateSrcset(baseUrl: string, widths: number[]): string {
		// If srcset is already provided, use it
		if (srcset) return srcset;

		// If autoSrcset is disabled, return empty
		if (!autoSrcset) return '';

		// Detect image CDN patterns and generate appropriate srcset
		const url = new URL(baseUrl, 'https://example.com');

		// Check if it's a Cloudflare Images URL
		if (url.hostname.includes('imagedelivery.net')) {
			return widths.map((w) => `${url.origin}${url.pathname}/w=${w},q=${quality} ${w}w`).join(', ');
		}

		// Check if it's an R2/S3 URL with Sharp integration
		if (url.pathname.includes('/media/') || url.pathname.includes('/images/')) {
			return widths.map((w) => `${baseUrl}?w=${w}&q=${quality} ${w}w`).join(', ');
		}

		// For standard URLs, assume query param based resizing
		const separator = baseUrl.includes('?') ? '&' : '?';
		return widths
			.map((w) => `${baseUrl}${separator}width=${w}&quality=${quality} ${w}w`)
			.join(', ');
	}

	/**
	 * Generate default sizes attribute if not provided
	 */
	function generateSizes(): string {
		if (sizes) return sizes;
		if (!autoSrcset) return '';

		// Responsive sizes based on common viewport breakpoints
		return '(max-width: 639.98px) 100vw, (max-width: 1023.98px) 75vw, 50vw';
	}

	let isLoaded = $state(false);
	let hasError = $state(false);
	let containerWidth = $derived(typeof width === 'number' ? `${width}px` : width);
	let containerHeight = $derived(typeof height === 'number' ? `${height}px` : height);

	let placeholderImage = $derived(blurhash ? `url(${blurhash})` : undefined);
	let placeholderBackground = $derived(
		blurhash ? undefined : 'linear-gradient(135deg, #1e293b 0%, #334155 50%, #1e293b 100%)'
	);

	function handleLoad() {
		isLoaded = true;
	}

	function handleError() {
		hasError = true;
	}

	function trackCachedImage(element: HTMLImageElement) {
		if (element.complete && element.naturalHeight !== 0) {
			isLoaded = true;
		}
	}
</script>

<div
	class={['blurhash-container', className]}
	style:width={containerWidth}
	style:height={containerHeight}
>
	<!-- Placeholder layer -->
	<div
		class={{ placeholder: true, hidden: isLoaded }}
		style:background={placeholderBackground}
		style:background-image={placeholderImage}
		style:background-size={blurhash ? 'cover' : undefined}
		style:background-position={blurhash ? 'center' : undefined}
	></div>

	<!-- Actual image -->
	{#if !hasError}
		<img
			{@attach trackCachedImage}
			{src}
			{alt}
			{loading}
			{decoding}
			width={typeof width === 'number' ? width : undefined}
			height={typeof height === 'number' ? height : undefined}
			sizes={generateSizes() || sizes}
			srcset={generateSrcset(src, breakpoints) || srcset}
			class={{ image: true, loaded: isLoaded }}
			onload={handleLoad}
			onerror={handleError}
		/>
	{:else}
		<!-- Error fallback -->
		<div class="error-placeholder">
			<Icon name="IconPhoto" size={48} />
		</div>
	{/if}
</div>

<style>
	.blurhash-container {
		position: relative;
		overflow: hidden;
		background: #1e293b;
	}

	.placeholder {
		position: absolute;
		inset: 0;
		transition: opacity 0.3s ease-out;
		filter: blur(20px);
		transform: scale(1.1);
	}

	.placeholder.hidden {
		opacity: 0;
		pointer-events: none;
	}

	.image {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
		transition: opacity 0.3s ease-out;
	}

	.image.loaded {
		opacity: 1;
	}

	.error-placeholder {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #1e293b, #334155);
		color: #475569;
	}
</style>
