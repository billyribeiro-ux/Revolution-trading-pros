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

	// State using Svelte 5 runes
	let isLoaded = $state(false);
	let hasError = $state(false);
	let imgElement = $state<HTMLImageElement | null>(null);

	// Generate a simple gradient placeholder if no blurhash
	let placeholderStyle = $derived.by(() => {
		if (blurhash) {
			// If blurhash is provided, use it as a data URL
			// The blurhash should be pre-converted to a data URL on the server
			return `background-image: url(${blurhash}); background-size: cover; background-position: center;`;
		}
		// Fallback gradient placeholder
		return 'background: linear-gradient(135deg, #1e293b 0%, #334155 50%, #1e293b 100%);';
	});

	// Handle image load
	function handleLoad() {
		isLoaded = true;
	}

	// Handle image error
	function handleError() {
		hasError = true;
	}

	// Preload image when component mounts
	$effect(() => {
		if (imgElement && src) {
			// Check if image is already cached
			if (imgElement.complete && imgElement.naturalHeight !== 0) {
				isLoaded = true;
			}
		}
	});
</script>

<div
	class="blurhash-container {className}"
	style="width: {typeof width === 'number' ? `${width}px` : width}; height: {typeof height ===
	'number'
		? `${height}px`
		: height};"
>
	<!-- Placeholder layer -->
	<div class="placeholder" class:hidden={isLoaded} style={placeholderStyle}></div>

	<!-- Actual image -->
	{#if !hasError}
		<img
			bind:this={imgElement}
			{src}
			{alt}
			{loading}
			{decoding}
			width={typeof width === 'number' ? width : undefined}
			height={typeof height === 'number' ? height : undefined}
			sizes={generateSizes() || sizes}
			srcset={generateSrcset(src, breakpoints) || srcset}
			class="image"
			class:loaded={isLoaded}
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
