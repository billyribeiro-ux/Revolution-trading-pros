<script lang="ts">
	/**
	 * TradingScene3D - Immersive 3D Trading Visualization
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Uses Threlte/Three.js for floating 3D candlesticks
	 * Low-poly, performance optimized
	 *
	 * @version 1.0.0
	 */
	import { Canvas } from '@threlte/core';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import Scene3D from './Scene3D.svelte';

	let mounted = $state(false);

	onMount(() => {
		if (browser) {
			mounted = true;
		}
	});
</script>

{#if mounted}
	<div class="scene-container">
		<Canvas>
			<Scene3D />
		</Canvas>
	</div>
{/if}

<style>
	.scene-container {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
	}

	.scene-container :global(canvas) {
		width: 100% !important;
		height: 100% !important;
	}
</style>
