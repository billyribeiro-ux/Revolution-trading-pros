<script lang="ts">
	/**
	 * HeroChartScene - Netflix L11+ Cinematic Chart Container
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Wraps the lightweight-charts canvas with:
	 * - Volumetric glow effect
	 * - Depth grid background
	 * - Ambient light pulses
	 * - Floating price tags
	 * - Support/Resistance bands
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';

	// ============================================================================
	// PROPS
	// ============================================================================
	interface Props {
		chartContainer: HTMLElement | null;
		chartReady: boolean;
		isVisible: boolean;
		currentPrice?: number;
		priceChange?: number;
	}

	let { chartContainer = $bindable(), chartReady, isVisible, currentPrice = 475.50, priceChange = 0.42 }: Props = $props();

	// ============================================================================
	// STATE
	// ============================================================================
	let sceneRef = $state<HTMLElement | null>(null);
	let glowIntensity = $state(0.6);
	let priceTagsVisible = $state(false);
	let gsapLib: any = null;

	// Floating price tags data
	const floatingPrices = [
		{ price: 476.20, type: 'resistance', label: 'R1', x: 85, y: 15 },
		{ price: 475.80, type: 'target', label: 'PT', x: 90, y: 35 },
		{ price: 474.90, type: 'support', label: 'S1', x: 88, y: 75 },
		{ price: 474.50, type: 'stop', label: 'SL', x: 82, y: 88 }
	];

	// Order flow indicators
	const orderFlowIndicators = [
		{ type: 'buy', volume: '2.4K', x: 25, y: 40 },
		{ type: 'sell', volume: '1.8K', x: 45, y: 55 },
		{ type: 'buy', volume: '3.1K', x: 65, y: 30 }
	];

	// ============================================================================
	// GSAP ENTRANCE ANIMATION
	// ============================================================================
	async function animateEntrance(): Promise<void> {
		if (!browser || !sceneRef) return;

		try {
			const gsapModule = await import('gsap');
			gsapLib = gsapModule?.gsap || gsapModule?.default;

			if (!gsapLib) return;

			const tl = gsapLib.timeline({ defaults: { ease: 'power3.out' } });

			// Scene container fade in
			tl.fromTo(sceneRef,
				{ opacity: 0, scale: 0.95 },
				{ opacity: 1, scale: 1, duration: 1.2 },
				0
			);

			// Grid lines draw in
			const gridLines = sceneRef.querySelectorAll('.grid-line');
			tl.fromTo(gridLines,
				{ scaleX: 0, opacity: 0 },
				{ scaleX: 1, opacity: 1, duration: 0.8, stagger: 0.05 },
				0.3
			);

			// Glow pulse up
			tl.to({ val: 0.3 },
				{
					val: 0.8,
					duration: 1.5,
					ease: 'sine.inOut',
					onUpdate: function() {
						glowIntensity = this.targets()[0].val;
					}
				},
				0.5
			);

			// Price tags float in
			tl.call(() => { priceTagsVisible = true; }, [], 1.0);

			const priceTags = sceneRef.querySelectorAll('.price-tag');
			tl.fromTo(priceTags,
				{ opacity: 0, x: 20, scale: 0.8 },
				{ opacity: 1, x: 0, scale: 1, duration: 0.6, stagger: 0.1, ease: 'back.out(1.4)' },
				1.1
			);

			// Order flow indicators
			const orderFlow = sceneRef.querySelectorAll('.order-flow');
			tl.fromTo(orderFlow,
				{ opacity: 0, y: 10 },
				{ opacity: 0.6, y: 0, duration: 0.5, stagger: 0.15 },
				1.4
			);

		} catch (e) {
			console.warn('GSAP load failed in HeroChartScene:', e);
			priceTagsVisible = true;
		}
	}

	// ============================================================================
	// AMBIENT GLOW ANIMATION
	// ============================================================================
	let glowInterval: ReturnType<typeof setInterval> | null = null;

	function startAmbientGlow(): void {
		if (!browser || !gsapLib) return;

		// Subtle breathing glow effect
		gsapLib.to({ val: glowIntensity }, {
			val: 0.9,
			duration: 3,
			ease: 'sine.inOut',
			yoyo: true,
			repeat: -1,
			onUpdate: function() {
				if (isVisible) {
					glowIntensity = this.targets()[0].val;
				}
			}
		});
	}

	// ============================================================================
	// LIFECYCLE
	// ============================================================================
	onMount(() => {
		if (browser && sceneRef) {
			animateEntrance().then(() => {
				startAmbientGlow();
			});
		}
	});

	onDestroy(() => {
		if (glowInterval) clearInterval(glowInterval);
	});
</script>

<div
	bind:this={sceneRef}
	class="chart-scene"
	class:chart-scene--ready={chartReady}
	aria-label="Live trading chart visualization"
>
	<!-- Depth Grid Background -->
	<div class="chart-grid" aria-hidden="true">
		{#each Array(8) as _, i}
			<div class="grid-line grid-line--horizontal" style="top: {12.5 * (i + 1)}%"></div>
		{/each}
		{#each Array(6) as _, i}
			<div class="grid-line grid-line--vertical" style="left: {16.67 * (i + 1)}%"></div>
		{/each}
	</div>

	<!-- Volumetric Glow Layer -->
	<div
		class="chart-glow"
		style="--glow-intensity: {glowIntensity}"
		aria-hidden="true"
	></div>

	<!-- Support/Resistance Bands -->
	<div class="sr-bands" aria-hidden="true">
		<div class="sr-band sr-band--resistance" style="top: 18%">
			<span class="sr-band__label">R1 476.20</span>
		</div>
		<div class="sr-band sr-band--support" style="top: 72%">
			<span class="sr-band__label">S1 474.90</span>
		</div>
	</div>

	<!-- Chart Container (receives bind from parent) -->
	<div
		bind:this={chartContainer}
		class="chart-canvas"
		aria-hidden="true"
	></div>

	<!-- Current Price Badge -->
	<div class="current-price">
		<span class="current-price__value">${currentPrice.toFixed(2)}</span>
		<span class="current-price__change" class:positive={priceChange >= 0} class:negative={priceChange < 0}>
			{priceChange >= 0 ? '+' : ''}{priceChange.toFixed(2)}%
		</span>
	</div>

	<!-- Floating Price Tags -->
	{#if priceTagsVisible}
		<div class="floating-prices" aria-hidden="true">
			{#each floatingPrices as tag (tag.label)}
				<div
					class="price-tag price-tag--{tag.type}"
					style="right: {100 - tag.x}%; top: {tag.y}%"
				>
					<span class="price-tag__label">{tag.label}</span>
					<span class="price-tag__value">${tag.price.toFixed(2)}</span>
				</div>
			{/each}
		</div>
	{/if}

	<!-- Order Flow Indicators -->
	<div class="order-flow-container" aria-hidden="true">
		{#each orderFlowIndicators as flow, i (i)}
			<div
				class="order-flow order-flow--{flow.type}"
				style="left: {flow.x}%; top: {flow.y}%"
			>
				<span class="order-flow__icon">{flow.type === 'buy' ? '▲' : '▼'}</span>
				<span class="order-flow__volume">{flow.volume}</span>
			</div>
		{/each}
	</div>

	<!-- Ambient Light Beam -->
	<div class="light-beam" aria-hidden="true"></div>

	<!-- Corner Accent -->
	<div class="corner-accent corner-accent--top-right" aria-hidden="true"></div>
	<div class="corner-accent corner-accent--bottom-left" aria-hidden="true"></div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Chart Scene Container
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.chart-scene {
		position: relative;
		width: 100%;
		height: 100%;
		min-height: 400px;
		border-radius: 1.5rem;
		overflow: hidden;
		background: linear-gradient(
			135deg,
			rgba(15, 23, 42, 0.95) 0%,
			rgba(30, 41, 59, 0.9) 50%,
			rgba(15, 23, 42, 0.95) 100%
		);
		border: 1px solid rgba(99, 102, 241, 0.15);
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.5),
			0 0 0 1px rgba(99, 102, 241, 0.1),
			inset 0 1px 0 rgba(255, 255, 255, 0.05);

		/* Performance */
		contain: layout style paint;
		transform: translateZ(0);
	}

	.chart-scene--ready {
		opacity: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Depth Grid Background
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.chart-grid {
		position: absolute;
		inset: 0;
		z-index: 1;
		opacity: 0.4;
		pointer-events: none;
	}

	.grid-line {
		position: absolute;
		background: linear-gradient(
			90deg,
			transparent,
			rgba(99, 102, 241, 0.2),
			transparent
		);
		transform-origin: left center;
	}

	.grid-line--horizontal {
		left: 0;
		right: 0;
		height: 1px;
	}

	.grid-line--vertical {
		top: 0;
		bottom: 0;
		width: 1px;
		background: linear-gradient(
			180deg,
			transparent,
			rgba(99, 102, 241, 0.2),
			transparent
		);
		transform-origin: top center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Volumetric Glow
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.chart-glow {
		position: absolute;
		inset: 0;
		z-index: 2;
		background: radial-gradient(
			ellipse 80% 60% at 50% 50%,
			rgba(99, 102, 241, calc(0.15 * var(--glow-intensity, 0.6))) 0%,
			rgba(139, 92, 246, calc(0.08 * var(--glow-intensity, 0.6))) 40%,
			transparent 70%
		);
		pointer-events: none;
		mix-blend-mode: screen;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Support/Resistance Bands
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.sr-bands {
		position: absolute;
		inset: 0;
		z-index: 3;
		pointer-events: none;
	}

	.sr-band {
		position: absolute;
		left: 0;
		right: 0;
		height: 2px;
		display: flex;
		align-items: center;
	}

	.sr-band--resistance {
		background: linear-gradient(90deg, transparent 5%, rgba(239, 68, 68, 0.4) 30%, rgba(239, 68, 68, 0.4) 70%, transparent 95%);
	}

	.sr-band--support {
		background: linear-gradient(90deg, transparent 5%, rgba(52, 211, 153, 0.4) 30%, rgba(52, 211, 153, 0.4) 70%, transparent 95%);
	}

	.sr-band__label {
		position: absolute;
		right: 1rem;
		transform: translateY(-50%);
		font-family: var(--font-mono, ui-monospace);
		font-size: 0.65rem;
		font-weight: 500;
		padding: 0.15rem 0.4rem;
		border-radius: 0.25rem;
		white-space: nowrap;
	}

	.sr-band--resistance .sr-band__label {
		background: rgba(239, 68, 68, 0.2);
		color: rgb(252, 165, 165);
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.sr-band--support .sr-band__label {
		background: rgba(52, 211, 153, 0.2);
		color: rgb(167, 243, 208);
		border: 1px solid rgba(52, 211, 153, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Chart Canvas Container
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.chart-canvas {
		position: absolute;
		inset: 0;
		z-index: 4;
		opacity: 0.65;

		/* Soft edge mask */
		mask-image: radial-gradient(
			ellipse 90% 85% at 50% 50%,
			black 60%,
			transparent 100%
		);
		-webkit-mask-image: radial-gradient(
			ellipse 90% 85% at 50% 50%,
			black 60%,
			transparent 100%
		);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Current Price Badge
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.current-price {
		position: absolute;
		top: 1rem;
		left: 1rem;
		z-index: 10;
		display: flex;
		align-items: baseline;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(15, 23, 42, 0.9);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 0.5rem;
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
	}

	.current-price__value {
		font-family: var(--font-mono, ui-monospace);
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
		letter-spacing: -0.02em;
	}

	.current-price__change {
		font-family: var(--font-mono, ui-monospace);
		font-size: 0.75rem;
		font-weight: 600;
		padding: 0.15rem 0.35rem;
		border-radius: 0.25rem;
	}

	.current-price__change.positive {
		background: rgba(52, 211, 153, 0.2);
		color: rgb(52, 211, 153);
	}

	.current-price__change.negative {
		background: rgba(239, 68, 68, 0.2);
		color: rgb(248, 113, 113);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Floating Price Tags
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.floating-prices {
		position: absolute;
		inset: 0;
		z-index: 8;
		pointer-events: none;
	}

	.price-tag {
		position: absolute;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		padding: 0.35rem 0.5rem;
		border-radius: 0.375rem;
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		font-family: var(--font-mono, ui-monospace);
		transform: translateZ(0);
	}

	.price-tag__label {
		font-size: 0.6rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		opacity: 0.8;
	}

	.price-tag__value {
		font-size: 0.75rem;
		font-weight: 700;
	}

	.price-tag--resistance {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: rgb(252, 165, 165);
	}

	.price-tag--target {
		background: rgba(250, 204, 21, 0.15);
		border: 1px solid rgba(250, 204, 21, 0.3);
		color: rgb(253, 224, 71);
	}

	.price-tag--support {
		background: rgba(52, 211, 153, 0.15);
		border: 1px solid rgba(52, 211, 153, 0.3);
		color: rgb(167, 243, 208);
	}

	.price-tag--stop {
		background: rgba(239, 68, 68, 0.2);
		border: 1px solid rgba(239, 68, 68, 0.4);
		color: rgb(248, 113, 113);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Order Flow Indicators
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.order-flow-container {
		position: absolute;
		inset: 0;
		z-index: 6;
		pointer-events: none;
	}

	.order-flow {
		position: absolute;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.2rem 0.4rem;
		border-radius: 0.25rem;
		font-family: var(--font-mono, ui-monospace);
		font-size: 0.6rem;
		font-weight: 600;
		opacity: 0.6;
		transform: translateZ(0);
	}

	.order-flow--buy {
		background: rgba(52, 211, 153, 0.1);
		color: rgb(167, 243, 208);
	}

	.order-flow--sell {
		background: rgba(239, 68, 68, 0.1);
		color: rgb(252, 165, 165);
	}

	.order-flow__icon {
		font-size: 0.5rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Light Beam Effect
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.light-beam {
		position: absolute;
		top: -50%;
		left: 50%;
		width: 150%;
		height: 200%;
		background: linear-gradient(
			180deg,
			transparent 0%,
			rgba(99, 102, 241, 0.03) 30%,
			rgba(139, 92, 246, 0.05) 50%,
			rgba(99, 102, 241, 0.03) 70%,
			transparent 100%
		);
		transform: translateX(-50%) rotate(-15deg);
		pointer-events: none;
		z-index: 5;
		mix-blend-mode: screen;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Corner Accents
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.corner-accent {
		position: absolute;
		width: 60px;
		height: 60px;
		pointer-events: none;
		z-index: 9;
	}

	.corner-accent--top-right {
		top: 0;
		right: 0;
		border-top: 2px solid rgba(99, 102, 241, 0.3);
		border-right: 2px solid rgba(99, 102, 241, 0.3);
		border-top-right-radius: 1.5rem;
	}

	.corner-accent--bottom-left {
		bottom: 0;
		left: 0;
		border-bottom: 2px solid rgba(99, 102, 241, 0.3);
		border-left: 2px solid rgba(99, 102, 241, 0.3);
		border-bottom-left-radius: 1.5rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Responsive
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1024px) {
		.chart-scene {
			min-height: 350px;
			border-radius: 1rem;
		}

		.current-price {
			top: 0.75rem;
			left: 0.75rem;
			padding: 0.4rem 0.6rem;
		}

		.current-price__value {
			font-size: 1.1rem;
		}

		.sr-band__label {
			font-size: 0.6rem;
		}

		.price-tag {
			padding: 0.25rem 0.4rem;
		}

		.price-tag__value {
			font-size: 0.65rem;
		}
	}

	@media (max-width: 768px) {
		.chart-scene {
			min-height: 280px;
		}

		.floating-prices,
		.order-flow-container {
			display: none;
		}

		.sr-band__label {
			display: none;
		}

		.corner-accent {
			width: 40px;
			height: 40px;
		}
	}

	@media (max-width: 480px) {
		.chart-scene {
			min-height: 220px;
			border-radius: 0.75rem;
		}

		.current-price {
			top: 0.5rem;
			left: 0.5rem;
			padding: 0.3rem 0.5rem;
			gap: 0.35rem;
		}

		.current-price__value {
			font-size: 1rem;
		}

		.current-price__change {
			font-size: 0.65rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.chart-glow,
		.light-beam {
			display: none;
		}

		.price-tag,
		.order-flow {
			transition: none;
		}
	}
</style>
