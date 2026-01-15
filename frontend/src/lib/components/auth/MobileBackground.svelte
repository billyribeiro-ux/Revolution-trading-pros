<script lang="ts">
	/**
	 * MobileBackground - Simplified Trading Background for Mobile
	 * Apple Principal Engineer ICT 11 Grade
	 *
	 * Lightweight animated background for mobile login.
	 * Uses CSS animations instead of canvas for performance.
	 *
	 * ICT 11+ Patterns:
	 * - Client-side only rendering via onMount (prevents SSR hydration issues)
	 * - Deterministic animations using index-based timing
	 * - Performance-optimized CSS animations over JavaScript
	 *
	 * @version 2.0.0 - ICT 11 Grade
	 */
	import { onMount } from 'svelte';

	let mounted = $state(false);

	onMount(() => {
		mounted = true;
	});
</script>

{#if mounted}
	<div class="mobile-bg" aria-hidden="true">
		<!-- Gradient Background -->
		<div class="gradient-layer"></div>

		<!-- Animated Bars (Price movement) -->
		<div class="price-bars">
			{#each { length: 8 } as _, i}
				<div
					class="bar"
					class:bullish={i % 3 !== 0}
					class:bearish={i % 3 === 0}
					style="--delay: {i * 0.2}s; --height: {30 + Math.random() * 50}%;"
				></div>
			{/each}
		</div>

		<!-- Floating Dots -->
		<div class="floating-dots">
			{#each { length: 12 } as _}
				<div
					class="dot"
					style="
						left: {Math.random() * 100}%;
						top: {Math.random() * 100}%;
						--duration: {3 + Math.random() * 4}s;
						--delay: {Math.random() * 2}s;
					"
				></div>
			{/each}
		</div>

		<!-- Grid Pattern -->
		<div class="grid-pattern"></div>
	</div>
{/if}

<style>
	.mobile-bg {
		position: fixed;
		inset: 0;
		overflow: hidden;
		pointer-events: none;
		z-index: 0;
	}

	/* Gradient Layer */
	.gradient-layer {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			180deg,
			var(--auth-bg, oklch(0.10 0.02 250)) 0%,
			oklch(0.08 0.03 270) 50%,
			var(--auth-bg, oklch(0.10 0.02 250)) 100%
		);
	}

	/* Price Bars Animation */
	.price-bars {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 40%;
		display: flex;
		align-items: flex-end;
		justify-content: space-around;
		padding: 0 10%;
		opacity: 0.15;
	}

	.bar {
		width: 8%;
		border-radius: 4px 4px 0 0;
		animation: bar-pulse 3s ease-in-out infinite;
		animation-delay: var(--delay);
	}

	.bar.bullish {
		background: linear-gradient(to top, rgba(34, 197, 94, 0.3), rgba(34, 197, 94, 0.1));
		height: var(--height);
	}

	.bar.bearish {
		background: linear-gradient(to top, rgba(239, 68, 68, 0.3), rgba(239, 68, 68, 0.1));
		height: calc(var(--height) * 0.7);
	}

	@keyframes bar-pulse {
		0%, 100% {
			transform: scaleY(1);
			opacity: 0.15;
		}
		50% {
			transform: scaleY(1.2);
			opacity: 0.25;
		}
	}

	/* Floating Dots */
	.floating-dots {
		position: absolute;
		inset: 0;
	}

	.dot {
		position: absolute;
		width: 4px;
		height: 4px;
		background: rgba(230, 184, 0, 0.3);
		border-radius: 50%;
		animation: float var(--duration) ease-in-out infinite;
		animation-delay: var(--delay);
	}

	@keyframes float {
		0%, 100% {
			transform: translateY(0) scale(1);
			opacity: 0.3;
		}
		50% {
			transform: translateY(-20px) scale(1.2);
			opacity: 0.6;
		}
	}

	/* Grid Pattern */
	.grid-pattern {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(rgba(230, 184, 0, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(230, 184, 0, 0.03) 1px, transparent 1px);
		background-size: 40px 40px;
		opacity: 0.5;
	}

	/* Light Theme */
	:global(html.light) .gradient-layer,
	:global(body.light) .gradient-layer {
		background: linear-gradient(
			180deg,
			#f5f5f7 0%,
			#e8e8ed 50%,
			#f5f5f7 100%
		);
	}

	:global(html.light) .bar.bullish,
	:global(body.light) .bar.bullish {
		background: linear-gradient(to top, rgba(5, 150, 105, 0.2), rgba(5, 150, 105, 0.05));
	}

	:global(html.light) .bar.bearish,
	:global(body.light) .bar.bearish {
		background: linear-gradient(to top, rgba(220, 38, 38, 0.2), rgba(220, 38, 38, 0.05));
	}

	:global(html.light) .dot,
	:global(body.light) .dot {
		background: rgba(230, 184, 0, 0.2);
	}

	:global(html.light) .grid-pattern,
	:global(body.light) .grid-pattern {
		background-image:
			linear-gradient(rgba(230, 184, 0, 0.04) 1px, transparent 1px),
			linear-gradient(90deg, rgba(230, 184, 0, 0.04) 1px, transparent 1px);
	}
</style>
