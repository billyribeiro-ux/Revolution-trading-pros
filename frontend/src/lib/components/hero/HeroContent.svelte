<script lang="ts">
	/**
	 * HeroContent - Netflix L11+ Content Zone
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Left content area with:
	 * - Powerful headline with gradient text
	 * - Value-driven subheadline
	 * - Feature bullets with micro-animations
	 * - Primary and secondary CTAs
	 * - Trust proof section (logos, stats)
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// ============================================================================
	// STATE
	// ============================================================================
	let contentRef = $state<HTMLElement | null>(null);
	let isAnimated = $state(false);
	let gsapLib: any = null;

	// Feature bullets with icons
	const features = [
		{
			icon: '🎯',
			text: 'Live trading rooms with real-time execution',
			highlight: 'Live trading rooms'
		},
		{
			icon: '⚡',
			text: 'SPX alerts with entry, stop, and target levels',
			highlight: 'SPX alerts'
		},
		{
			icon: '📊',
			text: 'Professional indicators and volume analysis',
			highlight: 'Professional indicators'
		},
		{
			icon: '🎓',
			text: 'Structured frameworks for consistent execution',
			highlight: 'Structured frameworks'
		}
	];

	// Trust logos/stats
	const trustStats = [
		{ value: '15K+', label: 'Active Traders' },
		{ value: '94%', label: 'Alert Accuracy' },
		{ value: '5+', label: 'Years Experience' }
	];

	// ============================================================================
	// GSAP ENTRANCE ANIMATION
	// ============================================================================
	export async function animateEntrance(): Promise<void> {
		if (!browser || !contentRef || isAnimated) return;

		try {
			const gsapModule = await import('gsap');
			gsapLib = gsapModule?.gsap || gsapModule?.default;

			if (!gsapLib) {
				isAnimated = true;
				return;
			}

			const tl = gsapLib.timeline({
				defaults: { ease: 'power3.out' },
				onComplete: () => { isAnimated = true; }
			});

			// Eyebrow label
			const eyebrow = contentRef.querySelector('.hero-eyebrow');
			tl.fromTo(eyebrow,
				{ opacity: 0, y: 20 },
				{ opacity: 1, y: 0, duration: 0.6 },
				0
			);

			// Headline - word by word reveal
			const headlineWords = contentRef.querySelectorAll('.hero-headline__word');
			tl.fromTo(headlineWords,
				{ opacity: 0, y: 40, rotateX: -20 },
				{ opacity: 1, y: 0, rotateX: 0, duration: 0.8, stagger: 0.08, ease: 'power2.out' },
				0.2
			);

			// Subheadline
			const subheadline = contentRef.querySelector('.hero-subheadline');
			tl.fromTo(subheadline,
				{ opacity: 0, y: 20 },
				{ opacity: 1, y: 0, duration: 0.7 },
				0.6
			);

			// Feature list items
			const features = contentRef.querySelectorAll('.hero-feature');
			tl.fromTo(features,
				{ opacity: 0, x: -30 },
				{ opacity: 1, x: 0, duration: 0.5, stagger: 0.1 },
				0.9
			);

			// CTA buttons
			const ctas = contentRef.querySelectorAll('.hero-cta');
			tl.fromTo(ctas,
				{ opacity: 0, y: 20, scale: 0.95 },
				{ opacity: 1, y: 0, scale: 1, duration: 0.6, stagger: 0.12, ease: 'back.out(1.4)' },
				1.2
			);

			// Trust section
			const trustItems = contentRef.querySelectorAll('.trust-stat');
			tl.fromTo(trustItems,
				{ opacity: 0, y: 15 },
				{ opacity: 1, y: 0, duration: 0.5, stagger: 0.1 },
				1.5
			);

		} catch (e) {
			console.warn('GSAP load failed in HeroContent:', e);
			isAnimated = true;
		}
	}

	// ============================================================================
	// LIFECYCLE
	// ============================================================================
	onMount(() => {
		// Animation triggered by parent
	});
</script>

<div
	bind:this={contentRef}
	class="hero-content"
	class:hero-content--animated={isAnimated}
>
	<!-- Eyebrow Label -->
	<div class="hero-eyebrow">
		<span class="hero-eyebrow__dot"></span>
		<span class="hero-eyebrow__text">Professional Trading Platform</span>
	</div>

	<!-- Main Headline -->
	<h1 class="hero-headline">
		<span class="hero-headline__word">Trade</span>
		<span class="hero-headline__word">with</span>
		<span class="hero-headline__word hero-headline__word--gradient">Precision</span>
		<br class="headline-break" />
		<span class="hero-headline__word">and</span>
		<span class="hero-headline__word hero-headline__word--gradient">Consistency</span>
	</h1>

	<!-- Subheadline -->
	<p class="hero-subheadline">
		Join live trading sessions, receive context-rich alerts, and master institutional-grade frameworks for repeatable success in the markets.
	</p>

	<!-- Feature List -->
	<ul class="hero-features">
		{#each features as feature, i (i)}
			<li class="hero-feature">
				<span class="hero-feature__icon">{feature.icon}</span>
				<span class="hero-feature__text">{feature.text}</span>
			</li>
		{/each}
	</ul>

	<!-- CTA Buttons -->
	<div class="hero-ctas">
		<a href="/live-trading-rooms/day-trading" class="hero-cta hero-cta--primary">
			<span class="hero-cta__text">Join Live Trading Room</span>
			<span class="hero-cta__icon">→</span>
		</a>
		<a href="/alerts/spx-profit-pulse" class="hero-cta hero-cta--secondary">
			<span class="hero-cta__text">Get SPX Alerts</span>
		</a>
	</div>

	<!-- Trust Section -->
	<div class="hero-trust">
		<div class="trust-stats">
			{#each trustStats as stat, i (i)}
				<div class="trust-stat">
					<span class="trust-stat__value">{stat.value}</span>
					<span class="trust-stat__label">{stat.label}</span>
				</div>
				{#if i < trustStats.length - 1}
					<span class="trust-divider"></span>
				{/if}
			{/each}
		</div>
		<p class="trust-disclaimer">
			Join thousands of traders improving their execution daily
		</p>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Content Container
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-content {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		max-width: 40rem;
	}

	/* Initial state before animation */
	.hero-content:not(.hero-content--animated) .hero-eyebrow,
	.hero-content:not(.hero-content--animated) .hero-headline__word,
	.hero-content:not(.hero-content--animated) .hero-subheadline,
	.hero-content:not(.hero-content--animated) .hero-feature,
	.hero-content:not(.hero-content--animated) .hero-cta,
	.hero-content:not(.hero-content--animated) .trust-stat {
		opacity: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Eyebrow Label
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-eyebrow {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.4rem 0.75rem 0.4rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 9999px;
		width: fit-content;
	}

	.hero-eyebrow__dot {
		width: 8px;
		height: 8px;
		background: linear-gradient(135deg, #34d399, #10b981);
		border-radius: 50%;
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; transform: scale(1); }
		50% { opacity: 0.7; transform: scale(1.1); }
	}

	.hero-eyebrow__text {
		font-family: var(--font-heading, system-ui);
		font-size: 0.75rem;
		font-weight: 600;
		letter-spacing: 0.05em;
		text-transform: uppercase;
		color: rgba(255, 255, 255, 0.8);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Headline
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-headline {
		font-family: var(--font-heading, system-ui);
		font-size: clamp(2.5rem, 5vw, 4rem);
		font-weight: 800;
		line-height: 1.1;
		color: white;
		letter-spacing: -0.02em;
	}

	.hero-headline__word {
		display: inline-block;
		margin-right: 0.3em;
		transform-style: preserve-3d;
	}

	.hero-headline__word--gradient {
		background: linear-gradient(
			135deg,
			#facc15 0%,
			#f59e0b 50%,
			#facc15 100%
		);
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-size: 200% 100%;
	}

	.headline-break {
		display: none;
	}

	@media (min-width: 640px) {
		.headline-break {
			display: block;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Subheadline
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-subheadline {
		font-family: var(--font-body, system-ui);
		font-size: clamp(1rem, 2vw, 1.2rem);
		line-height: 1.7;
		color: rgba(255, 255, 255, 0.75);
		max-width: 36rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Feature List
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-features {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		list-style: none;
		padding: 0;
		margin: 0.5rem 0;
	}

	.hero-feature {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 0;
	}

	.hero-feature__icon {
		font-size: 1.1rem;
		flex-shrink: 0;
	}

	.hero-feature__text {
		font-family: var(--font-body, system-ui);
		font-size: 0.95rem;
		color: rgba(255, 255, 255, 0.85);
		line-height: 1.5;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CTA Buttons
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-ctas {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		margin-top: 0.5rem;
	}

	.hero-cta {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem 1.75rem;
		font-family: var(--font-heading, system-ui);
		font-size: 0.95rem;
		font-weight: 600;
		text-decoration: none;
		border-radius: 0.75rem;
		cursor: pointer;
		transition: all 0.25s cubic-bezier(0.25, 0.8, 0.25, 1);
		transform: translateY(0) translateZ(0);
	}

	.hero-cta--primary {
		background: linear-gradient(135deg, #facc15 0%, #f59e0b 100%);
		color: #1f2937;
		box-shadow:
			0 4px 15px rgba(250, 204, 21, 0.3),
			0 0 0 1px rgba(250, 204, 21, 0.1);
	}

	.hero-cta--primary:hover {
		transform: translateY(-2px) translateZ(0);
		box-shadow:
			0 8px 25px rgba(250, 204, 21, 0.4),
			0 0 0 1px rgba(250, 204, 21, 0.2);
	}

	.hero-cta--primary:active {
		transform: translateY(0) scale(0.98) translateZ(0);
	}

	.hero-cta__icon {
		font-size: 1.1rem;
		transition: transform 0.25s ease;
	}

	.hero-cta--primary:hover .hero-cta__icon {
		transform: translateX(4px);
	}

	.hero-cta--secondary {
		background: rgba(255, 255, 255, 0.05);
		color: white;
		border: 1px solid rgba(255, 255, 255, 0.15);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}

	.hero-cta--secondary:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.25);
		transform: translateY(-2px) translateZ(0);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Trust Section
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-trust {
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.trust-stats {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		flex-wrap: wrap;
	}

	.trust-stat {
		display: flex;
		flex-direction: column;
		gap: 0.15rem;
	}

	.trust-stat__value {
		font-family: var(--font-heading, system-ui);
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
		letter-spacing: -0.02em;
	}

	.trust-stat__label {
		font-family: var(--font-body, system-ui);
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.6);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.trust-divider {
		width: 1px;
		height: 2.5rem;
		background: rgba(255, 255, 255, 0.15);
	}

	.trust-disclaimer {
		margin-top: 1rem;
		font-family: var(--font-body, system-ui);
		font-size: 0.8rem;
		color: rgba(255, 255, 255, 0.5);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Responsive
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1024px) {
		.hero-content {
			max-width: 100%;
			text-align: center;
			align-items: center;
		}

		.hero-subheadline {
			max-width: 100%;
		}

		.hero-features {
			align-items: center;
		}

		.hero-feature {
			justify-content: center;
		}

		.hero-ctas {
			justify-content: center;
		}

		.trust-stats {
			justify-content: center;
		}

		.trust-disclaimer {
			text-align: center;
		}
	}

	@media (max-width: 640px) {
		.hero-content {
			gap: 1.25rem;
		}

		.hero-headline {
			font-size: clamp(2rem, 8vw, 2.5rem);
		}

		.hero-subheadline {
			font-size: 0.95rem;
		}

		.hero-features {
			gap: 0.5rem;
		}

		.hero-feature {
			padding: 0.35rem 0;
		}

		.hero-feature__text {
			font-size: 0.875rem;
		}

		.hero-ctas {
			flex-direction: column;
			width: 100%;
			max-width: 320px;
		}

		.hero-cta {
			width: 100%;
			padding: 0.9rem 1.5rem;
		}

		.trust-stats {
			gap: 1rem;
		}

		.trust-stat__value {
			font-size: 1.25rem;
		}

		.trust-divider {
			height: 2rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.hero-eyebrow__dot {
			animation: none;
		}

		.hero-cta,
		.hero-cta__icon {
			transition: none;
		}
	}
</style>
