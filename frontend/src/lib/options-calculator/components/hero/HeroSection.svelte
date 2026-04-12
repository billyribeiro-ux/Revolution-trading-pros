<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let heroEl: HTMLElement | undefined = $state();
	let titleEl: HTMLElement | undefined = $state();
	let subtitleEl: HTMLElement | undefined = $state();
	let featuresEl: HTMLElement | undefined = $state();
	let ctaEl: HTMLElement | undefined = $state();
	let scrollIndicatorEl: HTMLElement | undefined = $state();
	let HeroBackground3D: any = $state(null);
	let ParticleField: any = $state(null);
	let isMobile = $state(false);
	let mounted = $state(false);

	const features = [
		{ label: 'Real-Time Data', icon: '⚡' },
		{ label: 'Full Greeks Suite', icon: 'Δ' },
		{ label: 'Monte Carlo Simulation', icon: '🎲' },
		{ label: 'Strategy Builder', icon: '📐' }
	];

	function scrollToCalculator() {
		const target = document.getElementById('calculator');
		if (target) {
			target.scrollIntoView({ behavior: 'smooth' });
		}
	}

	onMount(() => {
		if (!browser) return;

		// Detect mobile for fallback
		isMobile = window.innerWidth < 768 || /Mobi|Android/i.test(navigator.userAgent);

		let scrollTriggerRef: typeof import('gsap/ScrollTrigger').ScrollTrigger | null = null;

		(async () => {
		const gsap = (await import('gsap')).default;
		const { ScrollTrigger } = await import('gsap/ScrollTrigger');
		gsap.registerPlugin(ScrollTrigger);
		scrollTriggerRef = ScrollTrigger;

		// --- Title stagger animation (word by word) ---
		if (titleEl) {
			const words = titleEl.querySelectorAll('.hero-word');
			gsap.set(words, { opacity: 0, y: 30, rotateX: -40 });
			gsap.to(words, {
				opacity: 1,
				y: 0,
				rotateX: 0,
				duration: 0.7,
				stagger: 0.1,
				ease: 'power3.out',
				delay: 0.3
			});
		}

		// --- Subtitle fade in after title ---
		if (subtitleEl) {
			gsap.set(subtitleEl, { opacity: 0, y: 20 });
			gsap.to(subtitleEl, {
				opacity: 1,
				y: 0,
				duration: 0.8,
				ease: 'power2.out',
				delay: 1.1
			});
		}

		// --- Feature pills stagger ---
		if (featuresEl) {
			const items = featuresEl.querySelectorAll('.hero-feature');
			gsap.set(items, { opacity: 0, y: 20, scale: 0.9 });
			gsap.to(items, {
				opacity: 1,
				y: 0,
				scale: 1,
				duration: 0.5,
				stagger: 0.12,
				ease: 'back.out(1.4)',
				delay: 1.5
			});
		}

		// --- CTA button entrance ---
		if (ctaEl) {
			gsap.set(ctaEl, { opacity: 0, y: 15 });
			gsap.to(ctaEl, {
				opacity: 1,
				y: 0,
				duration: 0.6,
				ease: 'power2.out',
				delay: 2.1
			});
		}

		// --- Scroll indicator bounce ---
		if (scrollIndicatorEl) {
			gsap.set(scrollIndicatorEl, { opacity: 0 });
			gsap.to(scrollIndicatorEl, {
				opacity: 1,
				duration: 0.5,
				delay: 2.5
			});
			gsap.to(scrollIndicatorEl, {
				y: 8,
				duration: 1.2,
				ease: 'power1.inOut',
				yoyo: true,
				repeat: -1,
				delay: 2.8
			});
		}

		// --- ScrollTrigger: fade out hero with parallax on scroll ---
		if (heroEl) {
			ScrollTrigger.create({
				trigger: heroEl,
				start: 'top top',
				end: 'bottom top',
				scrub: true,
				onUpdate: (self) => {
					const progress = self.progress;
					if (heroEl) {
						heroEl.style.opacity = String(1 - progress * 1.2);
						heroEl.style.transform = `translateY(${progress * -80}px) scale(${1 - progress * 0.05})`;
					}
				}
			});
		}

		// --- Lazy load 3D background or 2D particle fallback ---
		try {
			if (isMobile) {
				const mod = await import('./ParticleField.svelte');
				ParticleField = mod.default;
			} else {
				const mod = await import('./HeroBackground3D.svelte');
				HeroBackground3D = mod.default;
			}
		} catch {
			// Silently fail - hero works without background
		}

		mounted = true;
		})();

		return () => {
			scrollTriggerRef?.getAll().forEach((t: any) => t.kill());
		};
	});
</script>

<section
	bind:this={heroEl}
	class="hero-section relative flex min-h-screen w-full items-center justify-center overflow-hidden"
	aria-label="Options Calculator Hero"
>
	<!-- Background gradient -->
	<div class="hero-gradient absolute inset-0 -z-20" aria-hidden="true"></div>

	<!-- 3D Background (desktop) -->
	{#if HeroBackground3D}
		<div class="absolute inset-0 -z-10" aria-hidden="true">
			<HeroBackground3D />
		</div>
	{/if}

	<!-- 2D Particle fallback (mobile) -->
	{#if ParticleField}
		<div class="absolute inset-0 -z-10" aria-hidden="true">
			<ParticleField />
		</div>
	{/if}

	<!-- Glassmorphism overlay card -->
	<div class="hero-card relative z-10 mx-4 flex max-w-2xl flex-col items-center px-8 py-12 text-center sm:mx-8 sm:px-12 sm:py-16">
		<!-- Calculator icon -->
		<div class="hero-icon mb-6 flex h-16 w-16 items-center justify-center rounded-2xl">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="32"
				height="32"
				viewBox="0 0 256 256"
				fill="currentColor"
				class="text-[#6366f1]"
				aria-hidden="true"
			>
				<path
					d="M200,24H56A16,16,0,0,0,40,40V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24Zm0,192H56V40H200ZM76,76A8,8,0,0,1,84,68h88a8,8,0,0,1,8,8v32a8,8,0,0,1-8,8H84a8,8,0,0,1-8-8Zm0,0V108h96V68H84Zm24,60a12,12,0,1,1-12-12A12,12,0,0,1,100,136Zm48,0a12,12,0,1,1-12-12A12,12,0,0,1,148,136Zm48,0a12,12,0,1,1-12-12A12,12,0,0,1,196,136Zm-96,40a12,12,0,1,1-12-12A12,12,0,0,1,100,176Zm48,0a12,12,0,1,1-12-12A12,12,0,0,1,148,176Zm48,0a12,12,0,1,1-12-12A12,12,0,0,1,196,176Z"
				/>
			</svg>
		</div>

		<!-- Title with word-by-word stagger -->
		<h1
			bind:this={titleEl}
			class="hero-title mb-4 text-3xl font-bold tracking-tight sm:text-4xl md:text-5xl"
			style="font-family: var(--calc-font-display, 'Plus Jakarta Sans', system-ui, sans-serif); perspective: 400px;"
		>
			{#each 'Black-Scholes Options Calculator'.split(' ') as word, i (word)}
				<span
					class="hero-word inline-block"
					style="display: inline-block;"
				>
					{word}{i < 3 ? '\u00A0' : ''}
				</span>
			{/each}
		</h1>

		<!-- Subtitle -->
		<p
			bind:this={subtitleEl}
			class="hero-subtitle mb-8 text-base text-white/60 sm:text-lg md:text-xl"
			style="font-family: var(--calc-font-body, 'Inter', system-ui, sans-serif);"
		>
			Institutional-Grade Options Pricing Engine
		</p>

		<!-- Feature pills -->
		<div
			bind:this={featuresEl}
			class="mb-10 flex flex-wrap items-center justify-center gap-3"
		>
			{#each features as feature (feature.label)}
				<span class="hero-feature inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-xs font-medium text-white/80 backdrop-blur-sm sm:text-sm">
					<span class="text-sm" aria-hidden="true">{feature.icon}</span>
					{feature.label}
				</span>
			{/each}
		</div>

		<!-- CTA Button -->
		<div bind:this={ctaEl}>
			<button
				onclick={scrollToCalculator}
				class="hero-cta group relative inline-flex items-center gap-2 overflow-hidden rounded-xl px-8 py-3.5 text-sm font-semibold text-white transition-all duration-300 hover:scale-105 hover:shadow-[0_0_30px_rgba(99,102,241,0.4)] sm:text-base"
				style="background: linear-gradient(135deg, #6366f1, #143e59); cursor: pointer;"
			>
				<span class="relative z-10">Explore Calculator</span>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="relative z-10 transition-transform duration-300 group-hover:translate-y-0.5"
					aria-hidden="true"
				>
					<path d="M12 5v14" />
					<path d="m19 12-7 7-7-7" />
				</svg>
				<!-- Hover shimmer -->
				<div
					class="absolute inset-0 -translate-x-full bg-gradient-to-r from-transparent via-white/10 to-transparent transition-transform duration-700 group-hover:translate-x-full"
					aria-hidden="true"
				></div>
			</button>
		</div>
	</div>

	<!-- Scroll-down indicator -->
	<div
		bind:this={scrollIndicatorEl}
		class="absolute bottom-8 left-1/2 flex -translate-x-1/2 flex-col items-center gap-2 opacity-0"
		aria-hidden="true"
	>
		<span class="text-[10px] font-medium uppercase tracking-widest text-white/30">
			Scroll
		</span>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="20"
			height="20"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="text-white/40"
		>
			<path d="m6 9 6 6 6-6" />
		</svg>
	</div>
</section>

<style>
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

	.hero-icon {
		background: rgba(99, 102, 241, 0.12);
		border: 1px solid rgba(99, 102, 241, 0.2);
		box-shadow: 0 0 24px rgba(99, 102, 241, 0.15);
	}

	.hero-title {
		color: #e8e8f0;
		line-height: 1.15;
	}

	.hero-word {
		will-change: transform, opacity;
	}

	.hero-section {
		will-change: opacity, transform;
	}

	/* Subtle top-edge highlight on card */
	.hero-card::before {
		content: '';
		position: absolute;
		top: 0;
		left: 10%;
		right: 10%;
		height: 1px;
		background: linear-gradient(
			90deg,
			transparent,
			rgba(99, 102, 241, 0.4),
			transparent
		);
		border-radius: 1px;
	}
</style>
