<script lang="ts">
	/**
	 * BlurHashImage - Svelte 5 Component
	 * Progressive image loading with BlurHash placeholder
	 *
	 * @version 1.0.0 - December 2024
	 * Uses Svelte 5 runes ($state, $derived, $effect)
	 */

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
	}

	let {
		src,
		alt,
		blurhash = null,
		width = '100%',
		height = 'auto',
		class: className = '',
		loading = 'lazy',
		decoding = 'async',
		sizes,
		srcset
	}: Props = $props();

	// State using Svelte 5 runes
	let isLoaded = $state(false);
	let hasError = $state(false);
	let imgElement = $state<HTMLImageElement | null>(null);

	// Generate a simple gradient placeholder if no blurhash
	let placeholderStyle = $derived(() => {
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
	style="width: {typeof width === 'number' ? `${width}px` : width}; height: {typeof height === 'number' ? `${height}px` : height};"
>
	<!-- Placeholder layer -->
	<div
		class="placeholder"
		class:hidden={isLoaded}
		style={placeholderStyle()}
	></div>

	<!-- Actual image -->
	{#if !hasError}
		<img
			bind:this={imgElement}
			{src}
			{alt}
			{loading}
			{decoding}
			{sizes}
			{srcset}
			class="image"
			class:loaded={isLoaded}
			onload={handleLoad}
			onerror={handleError}
		/>
	{:else}
		<!-- Error fallback -->
		<div class="error-placeholder">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
				<circle cx="8.5" cy="8.5" r="1.5"></circle>
				<polyline points="21 15 16 10 5 21"></polyline>
			</svg>
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
