<script lang="ts">
	import gsap from 'gsap';
	import { browser } from '$app/env';
	import { prefersReducedMotion } from 'svelte/motion';
	import { MediaQuery } from 'svelte/reactivity';
	import type { Attachment } from 'svelte/attachments';
	import type { ScrollTrigger } from 'gsap/ScrollTrigger';
	import IconCalculator from '@tabler/icons-svelte-runes/icons/calculator';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconDice5 from '@tabler/icons-svelte-runes/icons/dice-5';
	import IconRuler2 from '@tabler/icons-svelte-runes/icons/ruler-2';
	import IconArrowDown from '@tabler/icons-svelte-runes/icons/arrow-down';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';

	interface Props {
		/** Id of the element the call-to-action scrolls to. */
		targetId: string;
	}

	let { targetId }: Props = $props();

	// The entrance is pure CSS (keyframes below): it runs from the server-rendered
	// first paint, needs no JavaScript, and never flashes on hydration.

	// ── Background: WebGL scene on desktop, 2D particles on small or touch
	//    screens and for reduced motion, nothing at all under Save-Data. ──
	const compact = new MediaQuery('(max-width: 767px), (pointer: coarse)');

	/**
	 * Decided in the browser only: the server renders the gradient alone and never loads a
	 * scene module. It follows the media queries live (a rotated tablet, a resized window,
	 * a reduced-motion toggle). Save-Data is fixed for the page's lifetime.
	 */
	const background = $derived.by((): '3d' | '2d' | null => {
		if (!browser) return null;
		const { connection } = navigator as Navigator & { connection?: { saveData?: boolean } };
		if (connection?.saveData) return null;
		return compact.current || prefersReducedMotion.current ? '2d' : '3d';
	});

	const backgroundModule = $derived(
		background === '3d'
			? import('./HeroBackground3D.svelte')
			: background === '2d'
				? import('./ParticleField.svelte')
				: null
	);

	/** Fades and lifts the hero away as it scrolls out of view. */
	const scrollFade: Attachment<HTMLElement> = (hero) => {
		if (prefersReducedMotion.current) return;
		let disposed = false;
		let fadeTrigger: ScrollTrigger | undefined;
		void import('gsap/ScrollTrigger').then(({ ScrollTrigger: plugin }) => {
			if (disposed) return;
			gsap.registerPlugin(plugin);
			fadeTrigger = plugin.create({
				trigger: hero,
				start: 'top top',
				end: 'bottom top',
				onUpdate: ({ progress }) => {
					hero.style.opacity = String(Math.max(0, 1 - progress * 1.2));
					hero.style.transform = `translateY(${progress * -80}px) scale(${1 - progress * 0.05})`;
				}
			});
		});
		return () => {
			disposed = true;
			fadeTrigger?.kill();
			hero.style.opacity = '';
			hero.style.transform = '';
		};
	};

	function scrollToTarget(event: MouseEvent) {
		const target = document.getElementById(targetId);
		if (!target) return;
		event.preventDefault();
		target.scrollIntoView({ behavior: prefersReducedMotion.current ? 'auto' : 'smooth' });
	}
</script>

<section
	{@attach scrollFade}
	class="calc-hero isolate relative flex w-full items-center justify-center overflow-hidden"
	aria-labelledby="calc-hero-title"
>
	<div class="hero-gradient absolute inset-0 -z-20" aria-hidden="true"></div>

	{#if backgroundModule}
		{#await backgroundModule then { default: Background }}
			<div class="absolute inset-0 -z-10" aria-hidden="true">
				<Background />
			</div>
		{:catch}
			<!-- Decorative only: the gradient alone is a complete hero. -->
		{/await}
	{/if}

	<div
		class="hero-card relative z-10 mx-4 flex max-w-2xl flex-col items-center px-8 py-12 text-center sm:mx-8 sm:px-12 sm:py-16"
	>
		<div class="hero-icon mb-6 flex h-16 w-16 items-center justify-center rounded-2xl">
			<IconCalculator size={32} aria-hidden="true" />
		</div>

		<h1
			id="calc-hero-title"
			class="hero-title mb-4 text-3xl font-bold tracking-tight sm:text-4xl md:text-5xl"
		>
			<span class="hero-word" style:--i={0}>Black-Scholes</span>
			<span class="hero-word" style:--i={1}>Options</span>
			<span class="hero-word" style:--i={2}>Calculator</span>
		</h1>

		<p class="hero-subtitle hero-reveal mb-8 text-base sm:text-lg md:text-xl">
			Institutional-Grade Options Pricing Engine
		</p>

		<ul class="mb-10 flex flex-wrap items-center justify-center gap-3">
			<li class="hero-feature" style:--i={0}>
				<IconBolt size={16} aria-hidden="true" />
				Real-Time Data
			</li>
			<li class="hero-feature" style:--i={1}>
				<span class="hero-feature-glyph" aria-hidden="true">Δ</span>
				Full Greeks Suite
			</li>
			<li class="hero-feature" style:--i={2}>
				<IconDice5 size={16} aria-hidden="true" />
				Monte Carlo Simulation
			</li>
			<li class="hero-feature" style:--i={3}>
				<IconRuler2 size={16} aria-hidden="true" />
				Strategy Builder
			</li>
		</ul>

		<a
			href="#{targetId}"
			onclick={scrollToTarget}
			class="hero-cta group relative inline-flex items-center gap-2 overflow-hidden rounded-xl px-8 py-3.5 text-sm font-semibold text-white sm:text-base"
		>
			<span class="relative z-10">Explore Calculator</span>
			<span
				class="relative z-10 inline-flex transition-transform motion-reduce:transition-none duration-300 group-hover:translate-y-0.5"
			>
				<IconArrowDown size={18} aria-hidden="true" />
			</span>
			<span class="hero-cta-shimmer" aria-hidden="true"></span>
		</a>
	</div>

	<div class="hero-scroll-indicator" aria-hidden="true">
		<span class="text-[10px] font-medium uppercase tracking-widest">Scroll</span>
		<IconChevronDown size={20} aria-hidden="true" />
	</div>
</section>

<style>
	/* The site navbar is sticky and sits in flow above the hero (plus the admin toolbar
	   when it shows), so a full-viewport hero would push its scroll cue below the fold.
	   The hero takes the viewport minus that chrome. The heights mirror --nav-height in
	   NavBar.svelte; svh is the viewport with mobile browser bars shown, so the cue is
	   visible on first paint there too (vh is the fallback for older browsers). */
	.calc-hero {
		--calc-hero-nav-height: 80px;
		--calc-hero-chrome: calc(var(--calc-hero-nav-height) + var(--admin-toolbar-height, 0px));
		min-height: calc(100vh - var(--calc-hero-chrome));
		min-height: calc(100svh - var(--calc-hero-chrome));
	}

	@media (max-width: 767px) {
		.calc-hero {
			--calc-hero-nav-height: 74px;
		}
	}

	@media (min-width: 768px) and (max-width: 1023px) {
		.calc-hero {
			--calc-hero-nav-height: 84px;
		}
	}

	@media (min-width: 1152px) {
		.calc-hero {
			--calc-hero-nav-height: 88px;
		}
	}

	@media (min-width: 1440px) {
		.calc-hero {
			--calc-hero-nav-height: 94px;
		}
	}

	@media (min-width: 1536px) {
		.calc-hero {
			--calc-hero-nav-height: 104px;
		}
	}

	.hero-gradient {
		background: linear-gradient(165deg, #0a0f1a 0%, #0d1b2a 40%, #143e59 100%);
	}

	.hero-card {
		background: rgba(10, 15, 26, 0.6);
		backdrop-filter: blur(24px);
		-webkit-backdrop-filter: blur(24px);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 24px;
		box-shadow:
			0 8px 32px rgba(0, 0, 0, 0.4),
			0 0 60px rgba(99, 102, 241, 0.06),
			inset 0 1px 0 rgba(255, 255, 255, 0.05);
	}

	/* Subtle top-edge highlight on the card. */
	.hero-card::before {
		content: '';
		position: absolute;
		top: 0;
		left: 10%;
		right: 10%;
		height: 1px;
		background: linear-gradient(90deg, transparent, rgba(99, 102, 241, 0.4), transparent);
		border-radius: 1px;
	}

	.hero-icon {
		color: #6366f1;
		background: rgba(99, 102, 241, 0.12);
		border: 1px solid rgba(99, 102, 241, 0.2);
		box-shadow: 0 0 24px rgba(99, 102, 241, 0.15);
	}

	.hero-title {
		color: #e8e8f0;
		line-height: 1.15;
		font-family: var(--calc-font-display, 'Plus Jakarta Sans', system-ui, sans-serif);
		perspective: 400px;
	}

	.hero-word {
		display: inline-block;
		animation: hero-word-in 0.7s cubic-bezier(0.215, 0.61, 0.355, 1) backwards;
		animation-delay: calc(0.3s + var(--i) * 0.1s);
	}

	.hero-subtitle {
		color: rgba(255, 255, 255, 0.6);
		font-family: var(--calc-font-body, 'Inter', system-ui, sans-serif);
	}

	.hero-reveal {
		animation: hero-rise 0.8s cubic-bezier(0.25, 0.46, 0.45, 0.94) 1.1s backwards;
	}

	.hero-feature {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 9999px;
		background: rgba(255, 255, 255, 0.05);
		padding: 0.5rem 1rem;
		font-size: 0.75rem;
		font-weight: 500;
		color: rgba(255, 255, 255, 0.8);
		backdrop-filter: blur(4px);
		animation: hero-pop 0.5s cubic-bezier(0.34, 1.56, 0.64, 1) backwards;
		animation-delay: calc(1.5s + var(--i) * 0.12s);
	}

	@media (min-width: 640px) {
		.hero-feature {
			font-size: 0.875rem;
		}
	}

	.hero-feature-glyph {
		font-size: 0.875rem;
		font-weight: 700;
		line-height: 1;
	}

	.hero-cta {
		background: linear-gradient(135deg, #6366f1, #143e59);
		transition:
			transform 0.3s ease,
			box-shadow 0.3s ease;
		animation: hero-rise 0.6s cubic-bezier(0.25, 0.46, 0.45, 0.94) 2.1s backwards;
	}

	.hero-cta:hover {
		transform: scale(1.05);
		box-shadow: 0 0 30px rgba(99, 102, 241, 0.4);
	}

	.hero-cta:focus-visible {
		outline: 2px solid #a5b4fc;
		outline-offset: 3px;
	}

	.hero-cta-shimmer {
		position: absolute;
		inset: 0;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
		transform: translateX(-100%);
		transition: transform 0.7s ease;
	}

	.hero-cta:hover .hero-cta-shimmer {
		transform: translateX(100%);
	}

	.hero-scroll-indicator {
		position: absolute;
		bottom: 2rem;
		left: 50%;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		color: rgba(255, 255, 255, 0.35);
		translate: -50% 0;
		animation:
			hero-fade 0.5s ease 2.5s backwards,
			hero-bob 1.2s ease-in-out 2.8s infinite alternate;
	}

	@keyframes hero-word-in {
		from {
			opacity: 0;
			transform: translateY(30px) rotateX(-40deg);
		}
	}

	@keyframes hero-rise {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
	}

	@keyframes hero-pop {
		from {
			opacity: 0;
			transform: translateY(20px) scale(0.9);
		}
	}

	@keyframes hero-fade {
		from {
			opacity: 0;
		}
	}

	@keyframes hero-bob {
		to {
			transform: translateY(8px);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.hero-word,
		.hero-reveal,
		.hero-feature,
		.hero-cta,
		.hero-scroll-indicator {
			animation: none;
		}

		.hero-cta,
		.hero-cta-shimmer {
			transition: none;
		}

		.hero-cta:hover {
			transform: none;
		}
	}
</style>
