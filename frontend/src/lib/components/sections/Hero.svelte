<script lang="ts">
	/**
	 * HeroSection - Netflix L11+ Enhanced Slider Hero
	 * ══════════════════════════════════════════════════════════════════════════════
	 * ORIGINAL SLIDER STRUCTURE with cinematic enhancements:
	 * - 4 rotating slides with unique GSAP animations per slide
	 * - Enhanced chart background with volumetric glow
	 * - Cinematic slide transitions with blur/scale effects
	 * - Slide indicator dots with progress animation
	 * - Ambient depth layers and grid background
	 * - Premium button interactions
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, onDestroy, tick } from 'svelte';
	import { browser } from '$app/environment';

	// ============================================================================
	// MODULE-LEVEL PRE-COMPUTATION
	// ============================================================================
	const CANDLE_DATA = generateCandleData();

	function generateCandleData() {
		const candles = [];
		const baseTime = 1704715800;
		const rangeCenter = 475.5;
		const rangeHigh = 476.1;
		const rangeLow = 474.9;

		let previousClose = rangeCenter;
		let miniTrend = 0;

		for (let i = 0; i < 220; i++) {
			if (i % 7 === 0) miniTrend = (Math.random() - 0.5) * 0.15;
			miniTrend *= 0.85;

			const randomMove = (Math.random() - 0.5) * 0.12;
			const pullToCenter = -(previousClose - rangeCenter) * 0.2;
			const bigMove = Math.random() < 0.1 ? (Math.random() - 0.5) * 0.2 : 0;
			const priceMove = miniTrend + randomMove + pullToCenter + bigMove;

			const open = previousClose;
			let close = Math.max(rangeLow, Math.min(rangeHigh, open + priceMove));

			const bodySize = Math.abs(close - open);
			const wickRange = Math.max(bodySize * 0.08, 0.01);

			candles.push({
				time: baseTime + i * 60,
				open: +open.toFixed(2),
				high: +Math.min(rangeHigh, Math.max(open, close) + wickRange * Math.random()).toFixed(2),
				low: +Math.max(rangeLow, Math.min(open, close) - wickRange * Math.random()).toFixed(2),
				close: +close.toFixed(2)
			});

			previousClose = close;
		}

		return candles;
	}

	// Slide data - 4 rotating slides
	const SLIDES = Object.freeze([
		{
			title: 'Live Trading Rooms',
			subtitle: 'Institutional-Style Sessions',
			description:
				'Join structured sessions with clear levels, real-time execution, and disciplined risk management.',
			primaryCTA: { text: 'Explore Rooms', href: '/live-trading-rooms/day-trading' },
			secondaryCTA: { text: 'Create Free Account', href: '/signup' },
			accentColor: 'rgba(99, 102, 241, 0.5)'
		},
		{
			title: 'SPX Profit Pulse',
			subtitle: 'Context-Rich Alerts',
			description:
				'Receive SPX alerts with entry context, invalidation, and sizing guidance (not lottery tickets).',
			primaryCTA: { text: 'Get Alerts', href: '/alerts/spx-profit-pulse' },
			secondaryCTA: { text: 'Learn More', href: '/alerts/explosive-swings' },
			accentColor: 'rgba(250, 204, 21, 0.5)'
		},
		{
			title: 'Trading Frameworks',
			subtitle: 'Structured Education',
			description:
				'Master robust frameworks for execution, risk, and review workflows across market conditions.',
			primaryCTA: { text: 'Browse Courses', href: '/courses' },
			secondaryCTA: { text: 'View Mentorship', href: '/mentorship' },
			accentColor: 'rgba(52, 211, 153, 0.5)'
		},
		{
			title: 'Pro Tools & Indicators',
			subtitle: 'Professional-Grade Edge',
			description:
				'Volume profiles, internals, momentum systems, and tooling built for repeatable institutional logic.',
			primaryCTA: { text: 'View Indicators', href: '/indicators' },
			secondaryCTA: { text: 'See All Tools', href: '/store' },
			accentColor: 'rgba(139, 92, 246, 0.5)'
		}
	]);

	// ============================================================================
	// STATE
	// ============================================================================
	let chart: ReturnType<typeof import('lightweight-charts').createChart> | null = null;
	let series: any = null;
	let replayInterval: ReturnType<typeof setInterval> | null = null;
	let slideInterval: ReturnType<typeof setInterval> | null = null;
	let currentSlide = $state(0);
	let previousSlide = $state(-1);
	let isLooping = $state(false);
	let gsapLib: any = null;
	let timeline: any = null;
	let gsapLoaded = $state(false);
	let isTransitioning = $state(false);

	// DOM refs
	let heroSection = $state<HTMLElement | null>(null);
	let chartContainer = $state<HTMLElement | null>(null);
	let resizeObserver: ResizeObserver | null = null;
	let visibilityObserver: IntersectionObserver | null = null;
	let isVisible = $state(true);
	let chartReady = $state(false);
	let isAnimating = $state(false);

	// Progress for slide indicator
	let slideProgress = $state(0);
	let progressInterval: ReturnType<typeof setInterval> | null = null;

	// ============================================================================
	// CHART INITIALIZATION
	// ============================================================================
	async function initChart(): Promise<void> {
		if (!browser || !chartContainer || !heroSection || chartReady) return;

		// ICT 7: Wrap dynamic import in try-catch to prevent unhandled promise rejection
		let createChart, CandlestickSeries, CrosshairMode, ColorType;
		try {
			const chartModule = await import('lightweight-charts');
			createChart = chartModule.createChart;
			CandlestickSeries = chartModule.CandlestickSeries;
			CrosshairMode = chartModule.CrosshairMode;
			ColorType = chartModule.ColorType;
		} catch {
			// Chart library failed to load - hero will display without chart background
			return;
		}

		const width = heroSection.clientWidth || window.innerWidth;
		const height = heroSection.clientHeight || window.innerHeight;

		if (chart) {
			chart.remove();
			chart = null;
			series = null;
		}

		chart = createChart(chartContainer, {
			layout: {
				background: { type: ColorType.Solid, color: 'transparent' },
				textColor: 'rgba(255, 255, 255, 0.08)'
			},
			grid: {
				vertLines: { visible: false },
				horzLines: { visible: false }
			},
			width,
			height,
			handleScroll: false,
			handleScale: false,
			timeScale: {
				visible: false,
				rightOffset: 0,
				barSpacing: 5,
				minBarSpacing: 2,
				fixLeftEdge: false,
				fixRightEdge: true,
				lockVisibleTimeRangeOnResize: true,
				rightBarStaysOnScroll: true,
				shiftVisibleRangeOnNewBar: true,
				borderColor: 'transparent'
			},
			crosshair: { mode: CrosshairMode.Hidden },
			rightPriceScale: { visible: false, borderColor: 'transparent' }
		});

		series = chart.addSeries(CandlestickSeries, {
			upColor: 'rgba(52, 211, 153, 0.5)',
			downColor: 'rgba(239, 68, 68, 0.5)',
			borderVisible: false,
			wickVisible: true,
			wickUpColor: 'rgba(52, 211, 153, 0.35)',
			wickDownColor: 'rgba(239, 68, 68, 0.35)',
			priceLineVisible: false
		});

		const initialCount = 10;
		series.setData(CANDLE_DATA.slice(0, initialCount));

		if (chart.timeScale()) {
			chart.timeScale().setVisibleLogicalRange({ from: 0, to: initialCount });
		}

		chartReady = true;
		startReplay(initialCount);
	}

	// ============================================================================
	// REPLAY ANIMATION
	// ============================================================================
	function startReplay(startIndex: number = 10): void {
		if (!browser || !series || !chart || !heroSection) return;

		if (replayInterval) {
			clearInterval(replayInterval);
			replayInterval = null;
		}

		let currentIndex = Math.max(startIndex, 1);
		const totalCandles = CANDLE_DATA.length;
		const chartWidth = heroSection?.clientWidth || window.innerWidth;
		const barsVisible = Math.floor(chartWidth / 5);

		replayInterval = setInterval(() => {
			if (!series || isLooping || !isVisible) return;

			currentIndex++;

			if (currentIndex >= totalCandles) {
				handleLoop();
				return;
			}

			series.setData(CANDLE_DATA.slice(0, currentIndex));

			if (chart?.timeScale()) {
				chart.timeScale().setVisibleLogicalRange({
					from: Math.max(0, currentIndex - barsVisible + 1),
					to: currentIndex + 1
				});
			}
		}, 600);
	}

	function handleLoop(): void {
		if (isLooping || !browser || !chartContainer || !gsapLib || !series) return;

		isLooping = true;
		isAnimating = true;

		const loopTL = gsapLib.timeline({
			onComplete: () => {
				isLooping = false;
				isAnimating = false;
				if (replayInterval) clearInterval(replayInterval);
				startReplay(10);
			}
		});

		loopTL
			.to(chartContainer, { opacity: 0, scale: 1.02, duration: 0.6, ease: 'power2.in' })
			.call(() => {
				series.setData(CANDLE_DATA.slice(0, 10));
				if (chart?.timeScale()) {
					chart.timeScale().setVisibleLogicalRange({ from: 0, to: 10 });
				}
			})
			.to(chartContainer, { opacity: 0.4, scale: 1, duration: 0.6, ease: 'power2.out' });
	}

	// ============================================================================
	// SLIDE ANIMATIONS - ULTRA CINEMATIC TRANSITIONS (Netflix/Apple Level)
	// ============================================================================

	// Pulse the ambient glow during transitions
	function pulseAmbientGlow(): void {
		const glowEl = document.querySelector('.ambient-glow');
		if (!glowEl || !gsapLib) return;

		gsapLib.to(glowEl, {
			opacity: 1.5,
			scale: 1.2,
			duration: 0.3,
			ease: 'power2.out',
			yoyo: true,
			repeat: 1
		});
	}

	// Flash effect on chart during transition
	function flashChart(): void {
		if (!chartContainer || !gsapLib) return;

		gsapLib.to(chartContainer, {
			opacity: 0.7,
			duration: 0.15,
			ease: 'power2.out',
			yoyo: true,
			repeat: 1,
			onComplete: () => {
				gsapLib.to(chartContainer, { opacity: 0.4, duration: 0.3 });
			}
		});
	}

	function animateSlideOut(slideIndex: number): Promise<void> {
		return new Promise((resolve) => {
			if (!gsapLib || slideIndex < 0) {
				resolve();
				return;
			}

			const slide = document.querySelector(`[data-slide="${slideIndex}"]`);
			if (!slide) {
				resolve();
				return;
			}

			const h1 = slide.querySelector('h1');
			const h2 = slide.querySelector('h2');
			const p = slide.querySelector('p');
			const buttons = slide.querySelectorAll('.cta');

			const exitTL = gsapLib.timeline({ onComplete: resolve });

			// Cinematic exit - elements scatter/dissolve
			exitTL
				.to(
					buttons,
					{
						opacity: 0,
						y: 20,
						scale: 0.8,
						stagger: 0.03,
						duration: 0.25,
						ease: 'power3.in'
					},
					0
				)
				.to(
					p,
					{
						opacity: 0,
						y: -20,
						filter: 'blur(4px)',
						duration: 0.3,
						ease: 'power2.in'
					},
					0.05
				)
				.to(
					h2,
					{
						opacity: 0,
						y: -30,
						scale: 0.95,
						filter: 'blur(6px)',
						duration: 0.35,
						ease: 'power2.in'
					},
					0.1
				)
				.to(
					h1,
					{
						opacity: 0,
						scale: 0.9,
						filter: 'blur(12px)',
						y: -40,
						duration: 0.4,
						ease: 'power3.in'
					},
					0.15
				);

			// Trigger ambient effects
			pulseAmbientGlow();
		});
	}

	function animateSlideIn(slideIndex: number): void {
		if (!browser) return;

		const slide = document.querySelector(`[data-slide="${slideIndex}"]`);
		if (!slide) return;

		const h1 = slide.querySelector('h1');
		const h2 = slide.querySelector('h2');
		const p = slide.querySelector('p');
		const buttons = slide.querySelectorAll('.cta');

		// Fallback: If no GSAP, just make everything visible
		if (!gsapLib) {
			[h1, h2, p, ...Array.from(buttons)].forEach((el) => {
				if (el instanceof HTMLElement) {
					el.style.opacity = '1';
					el.style.visibility = 'visible';
					el.style.transform = 'none';
				}
			});
			return;
		}

		if (timeline) timeline.kill();

		isAnimating = true;

		// Flash chart for dramatic effect
		flashChart();

		timeline = gsapLib.timeline({
			onComplete: () => {
				isAnimating = false;
				// Ensure buttons are visible after animation completes
				buttons.forEach((btn) => {
					if (btn instanceof HTMLElement) {
						btn.style.opacity = '1';
						btn.style.visibility = 'visible';
					}
				});
			}
		});

		// Reset all elements
		gsapLib.set([h1, h2, p, ...Array.from(buttons)], {
			opacity: 0,
			y: 0,
			x: 0,
			scale: 1,
			rotation: 0,
			rotationX: 0,
			rotationY: 0,
			skewX: 0,
			skewY: 0,
			filter: 'blur(0px)',
			transformPerspective: 1000
		});

		// Each slide has a unique CINEMATIC entrance
		switch (slideIndex) {
			case 0: // "EPIC REVEAL" - Title explodes from center with shockwave
				timeline
					// Title bursts from tiny with massive blur
					.fromTo(
						h1,
						{ opacity: 0, scale: 0.3, filter: 'blur(30px)', y: 50 },
						{ opacity: 1, scale: 1, filter: 'blur(0px)', y: 0, duration: 1.2, ease: 'expo.out' },
						0
					)
					// Subtitle slides up with golden shimmer
					.fromTo(
						h2,
						{ opacity: 0, y: 60, scale: 0.9, filter: 'blur(8px)' },
						{ opacity: 1, y: 0, scale: 1, filter: 'blur(0px)', duration: 0.9, ease: 'power3.out' },
						0.3
					)
					// Description fades in smoothly
					.fromTo(
						p,
						{ opacity: 0, y: 30 },
						{ opacity: 1, y: 0, duration: 0.7, ease: 'power2.out' },
						0.5
					)
					// Buttons pop in with spring physics
					.fromTo(
						buttons,
						{ opacity: 0, y: 40, scale: 0.7 },
						{ opacity: 1, y: 0, scale: 1, duration: 0.8, stagger: 0.12, ease: 'back.out(1.7)' },
						0.65
					)
					// Subtle pulse on primary button
					.to(
						buttons[0],
						{
							boxShadow: '0 0 40px rgba(250, 204, 21, 0.6)',
							duration: 0.3,
							yoyo: true,
							repeat: 1
						},
						1.2
					);
				break;

			case 1: // "METEOR DROP" - Elements crash in from above with bounce
				timeline
					// Title slams down from way above
					.fromTo(
						h1,
						{ opacity: 0, y: -200, scale: 1.3, rotationX: -45 },
						{ opacity: 1, y: 0, scale: 1, rotationX: 0, duration: 1.1, ease: 'bounce.out' },
						0
					)
					// Subtitle follows with lighter bounce
					.fromTo(
						h2,
						{ opacity: 0, y: -150, scale: 1.2 },
						{ opacity: 1, y: 0, scale: 1, duration: 1, ease: 'elastic.out(1, 0.4)' },
						0.15
					)
					// Description drops in
					.fromTo(
						p,
						{ opacity: 0, y: -80 },
						{ opacity: 1, y: 0, duration: 0.8, ease: 'power3.out' },
						0.35
					)
					// Buttons slam in from above
					.fromTo(
						buttons,
						{ opacity: 0, y: -60, rotationX: -30 },
						{ opacity: 1, y: 0, rotationX: 0, duration: 0.7, stagger: 0.1, ease: 'back.out(1.4)' },
						0.55
					);
				break;

			case 2: // "CINEMATIC ZOOM" - Dolly zoom effect like Hitchcock
				timeline
					// Title zooms from far away with dramatic blur
					.fromTo(
						h1,
						{ opacity: 0, scale: 3, filter: 'blur(40px)', z: -500 },
						{ opacity: 1, scale: 1, filter: 'blur(0px)', z: 0, duration: 1.4, ease: 'expo.out' },
						0
					)
					// Subtitle emerges from the blur
					.fromTo(
						h2,
						{ opacity: 0, scale: 2, filter: 'blur(20px)' },
						{ opacity: 1, scale: 1, filter: 'blur(0px)', duration: 1.1, ease: 'power4.out' },
						0.2
					)
					// Description materializes
					.fromTo(
						p,
						{ opacity: 0, scale: 1.5, filter: 'blur(10px)' },
						{ opacity: 1, scale: 1, filter: 'blur(0px)', duration: 0.9, ease: 'power3.out' },
						0.45
					)
					// Buttons zoom in from distance
					.fromTo(
						buttons,
						{ opacity: 0, scale: 0.3, filter: 'blur(5px)' },
						{
							opacity: 1,
							scale: 1,
							filter: 'blur(0px)',
							duration: 0.8,
							stagger: 0.15,
							ease: 'back.out(2)'
						},
						0.7
					);
				break;

			case 3: // "MATRIX SLIDE" - Dramatic side-to-side with skew distortion
				timeline
					// Title slides from left with heavy skew
					.fromTo(
						h1,
						{ opacity: 0, x: -300, skewX: -20, scale: 0.8 },
						{ opacity: 1, x: 0, skewX: 0, scale: 1, duration: 1, ease: 'power4.out' },
						0
					)
					// Subtitle slides from right with opposite skew
					.fromTo(
						h2,
						{ opacity: 0, x: 300, skewX: 20, scale: 0.8 },
						{ opacity: 1, x: 0, skewX: 0, scale: 1, duration: 1, ease: 'power4.out' },
						0.12
					)
					// Description slides up
					.fromTo(
						p,
						{ opacity: 0, y: 50, skewY: 3 },
						{ opacity: 1, y: 0, skewY: 0, duration: 0.8, ease: 'power3.out' },
						0.35
					)
					// Buttons slide in from left with stagger
					.fromTo(
						buttons,
						{ opacity: 0, x: -80, skewX: -10 },
						{ opacity: 1, x: 0, skewX: 0, duration: 0.7, stagger: 0.12, ease: 'power3.out' },
						0.55
					);
				break;
		}
	}

	async function showSlide(index: number): Promise<void> {
		if (isTransitioning) return;
		isTransitioning = true;

		// Animate out previous slide
		if (previousSlide >= 0 && gsapLoaded) {
			await animateSlideOut(previousSlide);
		}

		previousSlide = currentSlide;
		currentSlide = index;

		await tick();

		if (gsapLoaded) {
			animateSlideIn(index);
		}

		// Reset progress
		slideProgress = 0;

		isTransitioning = false;
	}

	function nextSlide(): void {
		const next = (currentSlide + 1) % SLIDES.length;
		showSlide(next);
	}

	function goToSlide(index: number): void {
		if (index !== currentSlide) {
			// Reset slide interval when manually clicking
			if (slideInterval) {
				clearInterval(slideInterval);
			}
			showSlide(index);
			// Restart interval
			slideInterval = setInterval(nextSlide, 7000);
		}
	}

	// ============================================================================
	// PROGRESS ANIMATION
	// ============================================================================
	function startProgressTimer(): void {
		if (progressInterval) clearInterval(progressInterval);
		slideProgress = 0;

		progressInterval = setInterval(() => {
			slideProgress += 100 / 70; // 7 seconds = 7000ms, update every 100ms
			if (slideProgress >= 100) {
				slideProgress = 0;
			}
		}, 100);
	}

	// ============================================================================
	// RESIZE & VISIBILITY
	// ============================================================================
	function handleResize(): void {
		if (!browser || !chart || !heroSection) return;

		chart.applyOptions({
			width: heroSection.clientWidth,
			height: heroSection.clientHeight
		});
	}

	function setupVisibilityObserver(): void {
		if (!browser || !heroSection) return;

		visibilityObserver = new IntersectionObserver(
			(entries) => {
				isVisible = entries[0]?.isIntersecting ?? true;
			},
			{ threshold: 0.1 }
		);

		visibilityObserver.observe(heroSection);
	}

	// ============================================================================
	// LIFECYCLE
	// ============================================================================
	onMount(async () => {
		if (!browser) return;

		currentSlide = 0;
		previousSlide = -1;

		// Wait for DOM bindings to be ready
		await tick();

		if (heroSection && typeof ResizeObserver !== 'undefined') {
			resizeObserver = new ResizeObserver(handleResize);
			resizeObserver.observe(heroSection);
		}

		if (heroSection) {
			setupVisibilityObserver();
		}

		if (chartContainer && heroSection) {
			// ICT 7: Await the async function to properly handle any rejections
			try {
				await initChart();
			} catch {
				// Chart initialization failed - continue without chart
			}
		}

		try {
			const gsapModule = await import('gsap');
			gsapLib = gsapModule?.gsap || gsapModule?.default || null;
			if (gsapLib) {
				gsapLoaded = true;
				// Wait another tick to ensure slide elements are in DOM
				await tick();
				animateSlideIn(currentSlide);
			}
		} catch (e) {
			console.warn('GSAP failed to load:', e);
		}

		// Start slide rotation
		slideInterval = setInterval(nextSlide, 7000);
		startProgressTimer();
	});

	onDestroy(() => {
		if (slideInterval) clearInterval(slideInterval);
		if (replayInterval) clearInterval(replayInterval);
		if (progressInterval) clearInterval(progressInterval);
		if (timeline) timeline.kill();
		if (chart) chart.remove();
		if (resizeObserver) resizeObserver.disconnect();
		if (visibilityObserver) visibilityObserver.disconnect();

		chart = null;
		series = null;
		timeline = null;
	});
</script>

<section bind:this={heroSection} id="hero" class="hero-section" class:hero-animating={isAnimating}>
	<!-- Ambient Background Layers -->
	<div class="hero-ambient" aria-hidden="true">
		<div class="ambient-gradient"></div>
		<div class="ambient-grid"></div>
		<div class="ambient-glow" style="--accent-color: {SLIDES[currentSlide]!.accentColor}"></div>
	</div>

	<!-- Chart Background -->
	<div id="chart-bg" bind:this={chartContainer} class="hero-chart" aria-hidden="true"></div>

	<!-- Vignette Overlay -->
	<div class="hero-vignette" aria-hidden="true"></div>

	<!-- Content Overlay -->
	<div class="hero-content">
		<div class="hero-slides">
			{#each SLIDES as slide, i (i)}
				<article
					data-slide={i}
					class="slide"
					class:slide--hidden={i !== currentSlide}
					class:slide--active={i === currentSlide}
					aria-hidden={i !== currentSlide}
				>
					<h1 class="slide__title">{slide.title}</h1>
					<h2 class="slide__subtitle">{slide.subtitle}</h2>
					<p class="slide__description">{slide.description}</p>
					<div class="slide__actions">
						<a href={slide.primaryCTA.href} class="cta cta--primary">
							<span class="cta__text">{slide.primaryCTA.text}</span>
							<span class="cta__glow"></span>
						</a>
						<a href={slide.secondaryCTA.href} class="cta cta--secondary">
							<span class="cta__text">{slide.secondaryCTA.text}</span>
						</a>
					</div>
				</article>
			{/each}
		</div>

		<!-- Slide Indicators -->
		<nav class="slide-indicators" aria-label="Slide navigation">
			{#each SLIDES as _, i (i)}
				<button
					type="button"
					class="slide-dot"
					class:slide-dot--active={i === currentSlide}
					aria-label="Go to slide {i + 1}"
					aria-current={i === currentSlide ? 'true' : 'false'}
					onclick={() => goToSlide(i)}
				>
					<span
						class="slide-dot__fill"
						style="--progress: {i === currentSlide ? slideProgress : 0}%"
					></span>
				</button>
			{/each}
		</nav>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CSS Custom Properties - ICT11+ Responsive Nav Height
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	:root {
		--hero-bg: #050a12;
		--hero-nav-height: 80px;
		--hero-chart-opacity: 0.4;
	}

	/* ICT11+ Fix: Match responsive navbar heights */
	@media (min-width: 1152px) {
		:root {
			--hero-nav-height: 88px;
		}
	}

	@media (min-width: 1280px) {
		:root {
			--hero-nav-height: 94px;
		}
	}

	@media (min-width: 1440px) {
		:root {
			--hero-nav-height: 104px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Hero Section Container
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-section {
		position: relative;
		min-height: 100vh;
		min-height: 100dvh;
		padding-block-start: var(--hero-nav-height);
		background: var(--hero-bg);
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		/* ICT11+ Fix: Removed content-visibility as it can break IntersectionObserver for subsequent sections */
	}

	.hero-animating .hero-chart,
	.hero-animating .slide {
		will-change: transform, opacity, filter;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Ambient Background Layers - CINEMATIC DEPTH
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-ambient {
		position: absolute;
		inset: 0;
		z-index: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.ambient-gradient {
		position: absolute;
		inset: 0;
		background:
			radial-gradient(ellipse 150% 100% at 50% 0%, rgba(99, 102, 241, 0.15) 0%, transparent 50%),
			radial-gradient(ellipse 100% 60% at 100% 100%, rgba(139, 92, 246, 0.12) 0%, transparent 50%),
			radial-gradient(ellipse 80% 50% at 0% 80%, rgba(52, 211, 153, 0.08) 0%, transparent 50%),
			radial-gradient(ellipse 60% 40% at 30% 30%, rgba(250, 204, 21, 0.05) 0%, transparent 50%);
		animation: ambientPulse 8s ease-in-out infinite;
	}

	@keyframes ambientPulse {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.8;
			transform: scale(1.05);
		}
	}

	.ambient-grid {
		position: absolute;
		inset: -50px;
		background-image:
			linear-gradient(rgba(99, 102, 241, 0.04) 1px, transparent 1px),
			linear-gradient(90deg, rgba(99, 102, 241, 0.04) 1px, transparent 1px);
		background-size: 60px 60px;
		mask-image: radial-gradient(ellipse 90% 80% at 50% 50%, black 10%, transparent 70%);
		-webkit-mask-image: radial-gradient(ellipse 90% 80% at 50% 50%, black 10%, transparent 70%);
		animation: gridFloat 20s linear infinite;
		transform: perspective(500px) rotateX(5deg);
	}

	@keyframes gridFloat {
		0% {
			transform: perspective(500px) rotateX(5deg) translateY(0);
		}
		100% {
			transform: perspective(500px) rotateX(5deg) translateY(60px);
		}
	}

	.ambient-glow {
		position: absolute;
		top: 50%;
		left: 50%;
		width: 120%;
		height: 120%;
		transform: translate(-50%, -50%) scale(1);
		background: radial-gradient(
			ellipse 50% 35% at 50% 50%,
			var(--accent-color, rgba(99, 102, 241, 0.2)) 0%,
			transparent 70%
		);
		transition:
			background 1.5s ease,
			transform 0.5s ease,
			opacity 0.5s ease;
		mix-blend-mode: screen;
		filter: blur(40px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Chart Background
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-chart {
		position: absolute;
		inset: 0;
		z-index: 1;
		opacity: var(--hero-chart-opacity);
		mask-image: linear-gradient(to right, transparent 0%, black 10%, black 90%, transparent 100%);
		-webkit-mask-image: linear-gradient(
			to right,
			transparent 0%,
			black 10%,
			black 90%,
			transparent 100%
		);
		contain: strict;
		transform: translateZ(0);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Vignette Overlay
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-vignette {
		position: absolute;
		inset: 0;
		z-index: 2;
		background: radial-gradient(ellipse at center, transparent 0%, var(--hero-bg) 85%);
		pointer-events: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Content Layer
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.hero-content {
		position: relative;
		z-index: 10;
		width: 100%;
		max-width: 80rem;
		margin-inline: auto;
		padding-inline: 1.5rem;
		padding-block: 4rem;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	@media (min-width: 640px) {
		.hero-content {
			padding-inline: 2rem;
		}
	}

	@media (min-width: 1024px) {
		.hero-content {
			padding-inline: 3rem;
			padding-block: 5rem;
		}
	}

	.hero-slides {
		position: relative;
		min-height: 22rem;
		width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Slides
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide {
		text-align: center;
		max-width: 56rem;
		margin-inline: auto;
		opacity: 1;
		transform: translateZ(0);
	}

	.slide--hidden {
		display: none !important;
		pointer-events: none;
	}

	.slide--active {
		display: block !important;
		pointer-events: auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Typography - CINEMATIC TEXT EFFECTS
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide__title {
		font-family: var(--font-heading, system-ui);
		font-size: clamp(2.75rem, 9vw, 5.5rem);
		font-weight: 800;
		line-height: 1;
		color: white;
		margin-block-end: 0.75rem;
		letter-spacing: -0.03em;
		text-shadow:
			0 0 40px rgba(255, 255, 255, 0.15),
			0 4px 30px rgba(0, 0, 0, 0.5),
			0 0 80px rgba(99, 102, 241, 0.2);
		transform: translateZ(0);
		transform-style: preserve-3d;
		perspective: 1000px;
	}

	.slide__subtitle {
		font-family: var(--font-heading, system-ui);
		font-size: clamp(1.35rem, 4.5vw, 2.25rem);
		font-weight: 700;
		background: linear-gradient(
			135deg,
			#fde047 0%,
			#facc15 25%,
			#f59e0b 50%,
			#facc15 75%,
			#fde047 100%
		);
		background-size: 200% 100%;
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		animation: shimmer 3s ease-in-out infinite;
		margin-block-end: 1.5rem;
		transform: translateZ(0);
		filter: drop-shadow(0 2px 10px rgba(250, 204, 21, 0.3));
	}

	@keyframes shimmer {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	.slide__description {
		font-size: clamp(1.05rem, 2.5vw, 1.3rem);
		line-height: 1.75;
		color: rgba(255, 255, 255, 0.85);
		max-width: 44rem;
		margin-inline: auto;
		margin-block-end: 2.5rem;
		transform: translateZ(0);
		text-shadow: 0 2px 20px rgba(0, 0, 0, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CTA Buttons - ULTRA-CINEMATIC 3D EFFECTS
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide__actions {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
		justify-content: center;
		margin-top: 0.5rem;
		perspective: 1000px;
		transform-style: preserve-3d;
	}

	/* Ensure buttons are visible by default before GSAP takes over */
	.slide--active .cta {
		opacity: 1;
		visibility: visible;
	}

	.cta {
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 1.15rem 3rem;
		font-family: var(--font-heading, system-ui);
		font-size: 1.05rem;
		font-weight: 700;
		border-radius: 9999px;
		text-decoration: none;
		cursor: pointer;
		letter-spacing: 0.02em;
		opacity: 1;
		visibility: visible;

		/* 3D Transform Setup */
		transform-style: preserve-3d;
		transform: perspective(800px) translateZ(0) rotateX(0deg) rotateY(0deg);
		transition:
			transform 0.5s cubic-bezier(0.23, 1, 0.32, 1),
			box-shadow 0.5s cubic-bezier(0.23, 1, 0.32, 1),
			filter 0.3s ease;
	}

	.cta__text {
		position: relative;
		z-index: 3;
		transform: translateZ(20px);
		text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Primary Button - 3D Golden Powerhouse with Holographic Depth
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.cta--primary {
		background:
			linear-gradient(180deg, rgba(255, 255, 255, 0.25) 0%, transparent 50%),
			linear-gradient(135deg, #fde047 0%, #facc15 25%, #f59e0b 50%, #facc15 75%, #fde047 100%);
		background-size:
			100% 100%,
			300% 100%;
		color: #1a1a1a;

		/* Multi-layer 3D shadow for floating effect */
		box-shadow:
			/* Outer glow */
			0 0 60px rgba(250, 204, 21, 0.3),
			0 0 100px rgba(250, 204, 21, 0.15),
			/* Floating shadow layers */ 0 4px 6px rgba(0, 0, 0, 0.1),
			0 8px 15px rgba(245, 158, 11, 0.2),
			0 15px 30px rgba(250, 204, 21, 0.15),
			0 25px 50px rgba(0, 0, 0, 0.1),
			/* Inner highlights for 3D depth */ inset 0 2px 0 rgba(255, 255, 255, 0.5),
			inset 0 -2px 4px rgba(245, 158, 11, 0.3),
			/* Edge light */ 0 0 0 1px rgba(255, 255, 255, 0.2);

		animation:
			primaryFloat 4s ease-in-out infinite,
			goldenShimmer 3s ease-in-out infinite;
	}

	@keyframes primaryFloat {
		0%,
		100% {
			transform: perspective(800px) translateZ(0) rotateX(0deg) rotateY(0deg) translateY(0);
		}
		50% {
			transform: perspective(800px) translateZ(5px) rotateX(1deg) rotateY(0deg) translateY(-3px);
		}
	}

	@keyframes goldenShimmer {
		0%,
		100% {
			background-position:
				0% 0%,
				0% 50%;
		}
		50% {
			background-position:
				0% 0%,
				100% 50%;
		}
	}

	/* Holographic glow layer */
	.cta--primary .cta__glow {
		position: absolute;
		inset: -4px;
		background: conic-gradient(
			from 0deg,
			rgba(250, 204, 21, 0.8),
			rgba(251, 191, 36, 0.6),
			rgba(245, 158, 11, 0.8),
			rgba(251, 191, 36, 0.6),
			rgba(250, 204, 21, 0.8)
		);
		border-radius: inherit;
		opacity: 0;
		z-index: -1;
		filter: blur(15px);
		transform: translateZ(-10px);
		transition:
			opacity 0.5s ease,
			filter 0.5s ease;
		animation: rotateGlow 4s linear infinite;
	}

	@keyframes rotateGlow {
		from {
			filter: blur(15px) hue-rotate(0deg);
		}
		to {
			filter: blur(15px) hue-rotate(360deg);
		}
	}

	/* Light sweep reflection */
	.cta--primary::before {
		content: '';
		position: absolute;
		top: 0;
		left: -150%;
		width: 100%;
		height: 100%;
		background: linear-gradient(
			105deg,
			transparent 20%,
			rgba(255, 255, 255, 0.5) 45%,
			rgba(255, 255, 255, 0.8) 50%,
			rgba(255, 255, 255, 0.5) 55%,
			transparent 80%
		);
		transform: skewX(-20deg);
		transition: left 0.75s cubic-bezier(0.23, 1, 0.32, 1);
		z-index: 2;
	}

	/* Bottom reflection for 3D depth */
	.cta--primary::after {
		content: '';
		position: absolute;
		bottom: -8px;
		left: 10%;
		width: 80%;
		height: 8px;
		background: radial-gradient(ellipse at center, rgba(250, 204, 21, 0.4) 0%, transparent 70%);
		filter: blur(4px);
		opacity: 0.7;
		transform: translateZ(-20px) scaleY(0.3);
		transition:
			opacity 0.5s ease,
			transform 0.5s ease;
	}

	.cta--primary:hover {
		transform: perspective(800px) translateZ(25px) rotateX(-5deg) rotateY(3deg) translateY(-8px);
		animation: none;

		box-shadow:
			/* Intense outer glow */
			0 0 80px rgba(250, 204, 21, 0.5),
			0 0 120px rgba(250, 204, 21, 0.3),
			0 0 180px rgba(250, 204, 21, 0.15),
			/* Deep floating shadows */ 0 10px 20px rgba(0, 0, 0, 0.15),
			0 20px 40px rgba(245, 158, 11, 0.25),
			0 35px 60px rgba(0, 0, 0, 0.15),
			/* Inner 3D highlights */ inset 0 3px 0 rgba(255, 255, 255, 0.6),
			inset 0 -3px 6px rgba(245, 158, 11, 0.4),
			/* Edge glow */ 0 0 0 2px rgba(250, 204, 21, 0.5);
	}

	.cta--primary:hover .cta__glow {
		opacity: 1;
		filter: blur(20px);
	}

	.cta--primary:hover::before {
		left: 150%;
	}

	.cta--primary:hover::after {
		opacity: 1;
		transform: translateZ(-20px) scaleY(0.5) translateY(10px);
	}

	.cta--primary:active {
		transform: perspective(800px) translateZ(10px) rotateX(-2deg) rotateY(1deg) translateY(-2px);
		transition-duration: 0.1s;

		box-shadow:
			0 0 40px rgba(250, 204, 21, 0.4),
			0 5px 15px rgba(0, 0, 0, 0.2),
			0 10px 25px rgba(245, 158, 11, 0.2),
			inset 0 2px 0 rgba(255, 255, 255, 0.4),
			inset 0 -2px 4px rgba(245, 158, 11, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Secondary Button - 3D Crystal Glass with Holographic Edge
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.cta--secondary {
		background: linear-gradient(
			180deg,
			rgba(255, 255, 255, 0.12) 0%,
			rgba(255, 255, 255, 0.02) 50%,
			rgba(255, 255, 255, 0.05) 100%
		);
		color: white;
		border: 1.5px solid transparent;
		background-clip: padding-box;

		/* Holographic border effect */
		border-image: linear-gradient(
				135deg,
				rgba(255, 255, 255, 0.4),
				rgba(139, 92, 246, 0.3),
				rgba(99, 102, 241, 0.3),
				rgba(255, 255, 255, 0.2)
			)
			1;
		border-radius: 9999px;

		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);

		/* 3D Glass shadow layers */
		box-shadow:
			/* Outer glow */
			0 0 30px rgba(139, 92, 246, 0.1),
			/* Floating shadows */ 0 4px 8px rgba(0, 0, 0, 0.15),
			0 8px 20px rgba(0, 0, 0, 0.1),
			0 15px 40px rgba(0, 0, 0, 0.08),
			/* Inner glass highlights */ inset 0 1px 0 rgba(255, 255, 255, 0.2),
			inset 0 -1px 0 rgba(255, 255, 255, 0.05);

		animation: secondaryFloat 5s ease-in-out infinite;
	}

	/* Fix border-radius with border-image */
	.cta--secondary {
		border: none;
		position: relative;
	}

	/* Holographic border overlay */
	.cta--secondary::after {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: inherit;
		padding: 1.5px;
		background: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.5),
			rgba(139, 92, 246, 0.4),
			rgba(99, 102, 241, 0.4),
			rgba(52, 211, 153, 0.3),
			rgba(255, 255, 255, 0.3)
		);
		background-size: 300% 300%;
		-webkit-mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		-webkit-mask-composite: xor;
		mask-composite: exclude;
		animation: borderShift 4s ease infinite;
		pointer-events: none;
	}

	@keyframes borderShift {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	@keyframes secondaryFloat {
		0%,
		100% {
			transform: perspective(800px) translateZ(0) rotateX(0deg) rotateY(0deg) translateY(0);
		}
		50% {
			transform: perspective(800px) translateZ(3px) rotateX(-1deg) rotateY(0deg) translateY(-2px);
		}
	}

	/* Glass reflection sweep */
	.cta--secondary::before {
		content: '';
		position: absolute;
		top: 0;
		left: -150%;
		width: 80%;
		height: 100%;
		background: linear-gradient(
			100deg,
			transparent 20%,
			rgba(255, 255, 255, 0.2) 45%,
			rgba(255, 255, 255, 0.4) 50%,
			rgba(255, 255, 255, 0.2) 55%,
			transparent 80%
		);
		transform: skewX(-25deg);
		transition: left 0.8s cubic-bezier(0.23, 1, 0.32, 1);
		z-index: 2;
		border-radius: inherit;
	}

	.cta--secondary:hover {
		background: linear-gradient(
			180deg,
			rgba(255, 255, 255, 0.18) 0%,
			rgba(255, 255, 255, 0.05) 50%,
			rgba(255, 255, 255, 0.08) 100%
		);
		transform: perspective(800px) translateZ(20px) rotateX(-4deg) rotateY(-3deg) translateY(-8px);
		animation: none;

		box-shadow:
			/* Outer glow */
			0 0 50px rgba(139, 92, 246, 0.2),
			0 0 80px rgba(99, 102, 241, 0.1),
			/* Deep floating shadows */ 0 10px 20px rgba(0, 0, 0, 0.2),
			0 20px 40px rgba(0, 0, 0, 0.15),
			0 30px 60px rgba(0, 0, 0, 0.1),
			/* Enhanced inner highlights */ inset 0 2px 0 rgba(255, 255, 255, 0.3),
			inset 0 -1px 0 rgba(255, 255, 255, 0.1);
	}

	.cta--secondary:hover::before {
		left: 150%;
	}

	.cta--secondary:hover::after {
		animation: borderShift 2s ease infinite;
	}

	.cta--secondary:active {
		transform: perspective(800px) translateZ(8px) rotateX(-2deg) rotateY(-1deg) translateY(-2px);
		transition-duration: 0.1s;

		box-shadow:
			0 0 30px rgba(139, 92, 246, 0.15),
			0 5px 15px rgba(0, 0, 0, 0.2),
			0 10px 25px rgba(0, 0, 0, 0.1),
			inset 0 1px 0 rgba(255, 255, 255, 0.2);
	}

	.cta:focus-visible {
		outline: 2px solid #facc15;
		outline-offset: 6px;
		animation: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Responsive 3D Adjustments
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 768px) {
		.cta {
			padding: 1rem 2.25rem;
			font-size: 0.95rem;
		}

		.cta--primary:hover,
		.cta--secondary:hover {
			transform: perspective(800px) translateZ(15px) rotateX(-3deg) rotateY(2deg) translateY(-5px);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Slide Indicators
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.slide-indicators {
		display: flex;
		gap: 0.75rem;
		margin-top: 3rem;
	}

	.slide-dot {
		position: relative;
		width: 40px;
		height: 4px;
		background: rgba(255, 255, 255, 0.2);
		border: none;
		border-radius: 4px;
		cursor: pointer;
		overflow: hidden;
		transition:
			background 0.3s ease,
			transform 0.2s ease;
	}

	.slide-dot:hover {
		background: rgba(255, 255, 255, 0.3);
		transform: scaleY(1.5);
	}

	.slide-dot--active {
		background: rgba(255, 255, 255, 0.15);
	}

	.slide-dot__fill {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		width: var(--progress, 0%);
		background: linear-gradient(90deg, #facc15, #f59e0b);
		border-radius: 4px;
		transition: width 0.1s linear;
	}

	.slide-dot:focus-visible {
		outline: 2px solid #facc15;
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Responsive
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 768px) {
		.cta {
			padding: 0.875rem 2rem;
			font-size: 0.9rem;
		}

		.hero-slides {
			min-height: 20rem;
		}
	}

	@media (max-width: 640px) {
		.hero-section {
			padding-block-start: 60px;
		}

		.hero-content {
			padding-block: 3rem;
		}

		.slide__actions {
			flex-direction: column;
			align-items: center;
		}

		.cta {
			width: 100%;
			max-width: 280px;
		}

		.hero-slides {
			min-height: 24rem;
		}

		.slide-indicators {
			margin-top: 2rem;
		}

		.slide-dot {
			width: 32px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.slide,
		.cta,
		.hero-chart,
		.ambient-glow,
		.slide-dot {
			transition: none;
		}

		.slide-dot__fill {
			transition: none;
		}
	}
</style>
