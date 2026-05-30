<script lang="ts">
	/**
	 * TradingScene3D - Immersive 3D Trading Visualization
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Uses Threlte/Three.js for floating 3D candlesticks
	 * Low-poly, performance optimized
	 *
	 * ICT11+ Pattern: Dynamic imports to prevent SSR issues and improve TTFB
	 *
	 * @version 2.0.0
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import type { Canvas } from '@threlte/core';
	import type Scene3D from './Scene3D.svelte';

	let mounted = $state(false);
	// Lazily-imported Svelte components, typed from their modules so the markup
	// below renders them with the correct props.
	let ThrelteCanvas: typeof Canvas | null = $state(null);
	let Scene3DComponent: typeof Scene3D | null = $state(null);

	onMount(async () => {
		if (!browser) return;

		// Dynamic import to prevent SSR issues and reduce initial bundle
		const [threlte, scene] = await Promise.all([
			import('@threlte/core'),
			import('./Scene3D.svelte')
		]);

		ThrelteCanvas = threlte.Canvas;
		Scene3DComponent = scene.default;
		mounted = true;
	});
</script>

{#if mounted && ThrelteCanvas && Scene3DComponent}
	<div class="scene-container">
		<ThrelteCanvas>
			<Scene3DComponent />
		</ThrelteCanvas>
	</div>
{/if}

<style>
	.scene-container {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
	}

	/* Threlte's <Canvas> component runs a ResizeObserver internally and
	   calls WebGLRenderer.setSize() with the container's actual pixel
	   dimensions. The canvas's inline style.width/height is therefore
	   already correct — no CSS override needed. The previous defensive
	   override (width:100%, height:100%) fought the canonical Three.js
	   sizing pattern and has been removed (2026-04-25). */
</style>
