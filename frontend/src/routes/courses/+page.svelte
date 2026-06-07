<script lang="ts">
	import { onMount } from 'svelte';
	import { cubicOut } from 'svelte/easing';
	import { slide } from 'svelte/transition';
	import { browser } from '$app/environment';
	import { Tween } from 'svelte/motion';
	import { expoOut } from 'svelte/easing';
	import type { Attachment } from 'svelte/attachments';

	// --- ICONS ---
	import IconSchool from '@tabler/icons-svelte-runes/icons/school';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconChartCandle from '@tabler/icons-svelte-runes/icons/chart-candle';
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconBrain from '@tabler/icons-svelte-runes/icons/brain';
	import IconShield from '@tabler/icons-svelte-runes/icons/shield';
	import IconRocket from '@tabler/icons-svelte-runes/icons/rocket';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';

	// Assumed existing component based on your snippet
	import type { IconComponent } from '$lib/icons';

	// --- TYPES ---

	type CourseVariant = 'blue' | 'emerald' | 'violet' | 'orange';

	interface Course {
		id: string;
		title: string;
		slug: string;
		description: string;
		targetAudience: string;
		level: string;
		duration: string;
		students: string;
		rating: number;
		price: string;
		icon: IconComponent;
		features: string[];
		/** Variant token — drives the .courses-card--<variant> CSS modifier. */
		variant: CourseVariant;
	}

	interface FaqItem {
		question: string;
		answer: string;
	}

	interface VisualBar {
		id: number;
		height: string;
		delay: string;
		opacity: number;
	}

	// --- DATA ---

	const courses: Course[] = [
		{
			id: '1',
			title: 'Day Trading Masterclass',
			slug: 'day-trading-masterclass',
			description:
				'Decode institutional order flow, Level 2 data, and volume analysis to execute sniper-like entries.',
			targetAudience: 'Active Cash Flow',
			level: 'Advanced',
			duration: '8 Weeks',
			students: '2.8k+',
			rating: 4.9,
			price: '$497',
			icon: IconChartCandle,
			features: ['Tape Reading', 'L2 Analysis', 'Gap Strategies'],
			variant: 'blue'
		},
		{
			id: '2',
			title: 'Swing Trading Pro',
			slug: 'swing-trading-pro',
			description:
				'Capture major market moves. Identify macro trends and execute low-stress setups.',
			targetAudience: 'Wealth Compounding',
			level: 'Beginner',
			duration: '6 Weeks',
			students: '3.4k+',
			rating: 4.8,
			price: '$397',
			icon: IconChartLine,
			features: ['Macro Trends', 'Supply/Demand', 'Portfolio Rotation'],
			variant: 'emerald'
		},
		{
			id: '3',
			title: 'Options Tactics',
			slug: 'options-trading',
			description:
				'Defined-risk spreads that profit even if the market goes nowhere. Handle leverage safely.',
			targetAudience: 'Income & Hedging',
			level: 'Intermediate',
			duration: '10 Weeks',
			students: '1.9k+',
			rating: 4.9,
			price: '$597',
			icon: IconBrain,
			features: ['The Greeks', 'Iron Condors', 'Volatility Crush'],
			variant: 'violet'
		},
		{
			id: '4',
			title: 'Risk Protocol',
			slug: 'risk-management',
			description:
				'The mathematical framework used by proprietary desks to ensure you never blow up.',
			targetAudience: 'Mandatory Foundation',
			level: 'All Levels',
			duration: '4 Weeks',
			students: '4.1k+',
			rating: 5.0,
			price: '$297',
			icon: IconShield,
			features: ['Position Sizing', 'R-Multiples', 'Drawdown Protocols'],
			variant: 'orange'
		}
	];

	const faqs: FaqItem[] = [
		{
			question: 'Where do I start as a beginner?',
			answer:
				"Start with 'Swing Trading Pro' combined with 'Risk Management Mastery'. This combination builds the foundation of capital preservation and trend identification without the stress of intraday execution."
		},
		{
			question: 'Is there live mentorship?',
			answer:
				"Yes. It's a hybrid ecosystem. Core curriculum is on-demand video, but our Discord provides daily mentor access, trade reviews, and weekly live Q&A sessions."
		},
		{
			question: 'Do I need a large account?',
			answer:
				'No. We enforce starting with a simulator or micro-account. Our Risk Protocol teaches you to scale based on percentage performance, not dollar amount.'
		},
		{
			question: 'Lifetime access?',
			answer:
				'Guaranteed. You get all future updates for version 2.0, 3.0, and beyond at no extra cost. Markets evolve, and so does our content.'
		}
	];

	const trapVisualBars: VisualBar[] = Array.from({ length: 20 }, (_, i) => ({
		id: i,
		height: `${30 + ((i * 37) % 61)}%`,
		delay: `${i * 0.1}s`,
		opacity: 0.3 + ((i * 17) % 70) / 100
	}));

	// --- SVELTE 5 RUNES STATE ---

	let scrollY = $state(0);
	let innerHeight = $state(0);
	let innerWidth = $state(0);

	// Accordion State
	let openFaqIndex = $state<number | null>(0);

	// Canvas attachment — Svelte 5.29+ best practice.
	// Uses ResizeObserver to size the canvas from its own CSS dimensions,
	// avoiding the 0×0 seed problem that occurs when innerWidth/innerHeight
	// are still 0 at mount time.
	const particleCanvas: Attachment<HTMLCanvasElement> = (node) => {
		const maybeCtx = node.getContext('2d');
		if (!maybeCtx) return;
		const ctx: CanvasRenderingContext2D = maybeCtx;

		let particles: ParticleData[] = [];
		let rafId: number;

		function seed(w: number, h: number) {
			const count = w < 768 ? 30 : 60;
			particles = Array.from({ length: count }, () => createParticle(w, h));
		}

		function animate() {
			const w = node.width;
			const h = node.height;
			ctx.clearRect(0, 0, w, h);
			ctx.strokeStyle = 'rgba(56, 189, 248, 0.05)';
			ctx.lineWidth = 1;
			for (let i = 0; i < particles.length; i++) {
				updateParticle(particles[i], w, h);
				drawParticle(ctx, particles[i]);
				for (let j = i + 1; j < particles.length; j++) {
					const dx = particles[i].x - particles[j].x;
					const dy = particles[i].y - particles[j].y;
					if (Math.sqrt(dx * dx + dy * dy) < 150) {
						ctx.beginPath();
						ctx.moveTo(particles[i].x, particles[i].y);
						ctx.lineTo(particles[j].x, particles[j].y);
						ctx.stroke();
					}
				}
			}
			rafId = requestAnimationFrame(animate);
		}

		const ro = new ResizeObserver((entries) => {
			const entry = entries[0];
			const w = entry.contentRect.width;
			const h = entry.contentRect.height;
			if (w === 0 || h === 0) return;
			node.width = w;
			node.height = h;
			seed(w, h);
		});
		ro.observe(node);
		animate();

		return () => {
			cancelAnimationFrame(rafId);
			ro.disconnect();
		};
	};

	// --- PARTICLE CLASS (moved to top level to avoid nested class warning) ---

	interface ParticleData {
		x: number;
		y: number;
		size: number;
		speedX: number;
		speedY: number;
		opacity: number;
	}

	function createParticle(width: number, height: number): ParticleData {
		return {
			x: Math.random() * width,
			y: Math.random() * height,
			size: Math.random() * 2,
			speedX: (Math.random() - 0.5) * 0.3,
			speedY: (Math.random() - 0.5) * 0.3,
			opacity: Math.random() * 0.5
		};
	}

	function updateParticle(p: ParticleData, width: number, height: number): void {
		p.x += p.speedX;
		p.y += p.speedY;
		if (p.x > width) p.x = 0;
		if (p.x < 0) p.x = width;
		if (p.y > height) p.y = 0;
		if (p.y < 0) p.y = height;
	}

	function drawParticle(ctx: CanvasRenderingContext2D, p: ParticleData): void {
		ctx.fillStyle = `rgba(100, 116, 139, ${p.opacity})`;
		ctx.beginPath();
		ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
		ctx.fill();
	}

	// --- ACTIONS & UTILS ---

	// Per-card spotlight attachment — avoids page-level querySelectorAll +
	// getBoundingClientRect on every mousemove event.
	const spotlightCard: Attachment<HTMLElement> = (node) => {
		function onMouseMove(e: MouseEvent) {
			const rect = node.getBoundingClientRect();
			node.style.setProperty('--mouse-x', `${e.clientX - rect.left}px`);
			node.style.setProperty('--mouse-y', `${e.clientY - rect.top}px`);
		}
		node.addEventListener('mousemove', onMouseMove);
		return () => node.removeEventListener('mousemove', onMouseMove);
	};

	function toggleFaq(index: number) {
		if (openFaqIndex === index) {
			openFaqIndex = null;
		} else {
			openFaqIndex = index;
		}
	}

	function getFaqChevronClass(index: number): string {
		return openFaqIndex === index
			? 'faq-item__chevron faq-item__chevron--open'
			: 'faq-item__chevron';
	}

	// --- ANIMATION EFFECTS ---

	// Single source of truth for student count — tween and CTA both derive from this
	const STUDENT_COUNT = 12_000;
	const studentCounter = new Tween(0, { duration: 2000, easing: expoOut });

	onMount(() => {
		if (!browser) return;

		// Start the student counter tween
		studentCounter.set(STUDENT_COUNT);

		// Load GSAP dynamically for SSR safety
		(async () => {
			const { gsap } = await import('gsap');

			// GSAP Entrance Timeline
			const tl = gsap.timeline();
			tl.from('.hero-badge', { y: 20, opacity: 0, duration: 0.8, ease: 'power3.out' })
				.from(
					'.hero-title-line',
					{ y: 50, opacity: 0, duration: 1, stagger: 0.15, ease: 'power4.out' },
					'-=0.4'
				)
				.from('.hero-desc', { y: 20, opacity: 0, duration: 0.8, ease: 'power2.out' }, '-=0.6')
				.from(
					'.hero-stats',
					{ scale: 0.95, opacity: 0, duration: 0.8, ease: 'back.out(1.7)' },
					'-=0.6'
				)
				.from('.ticker-bar', { opacity: 0, y: 100, duration: 1 }, '-=0.8');
		})();
	});

	// Course schemas now emitted from +page.ts (page.data.seo).
</script>

<svelte:window bind:scrollY bind:innerHeight bind:innerWidth />

<div class="courses">
	<section class="courses-hero">
		<div class="courses-hero__canvas" aria-hidden="true">
			<canvas {@attach particleCanvas} class="courses-hero__canvas-el"></canvas>
		</div>

		<div class="courses-hero__veil" aria-hidden="true"></div>
		<div class="courses-hero__radial" aria-hidden="true"></div>

		<div class="courses-hero__inner">
			<div class="hero-badge courses-hero__badge">
				<span class="courses-hero__pulse-wrap">
					<span class="courses-hero__pulse-ping" aria-hidden="true"></span>
					<span class="courses-hero__pulse-dot" aria-hidden="true"></span>
				</span>
				<span class="courses-hero__badge-label">Live Market Admissions Open</span>
			</div>

			<h1 class="courses-hero__title">
				<span class="hero-title-line courses-hero__title-shout">EXECUTION</span>
				<span class="hero-title-line courses-hero__title-accent">OVER OPINION</span>
			</h1>

			<p class="hero-desc courses-hero__desc">
				The market transfers money from the
				<span class="courses-hero__desc-impatient">impatient</span>
				to the
				<span class="courses-hero__desc-disciplined">disciplined</span>. Stop gambling on signals.
				Start trading with institutional edge.
			</p>

			<div class="hero-stats courses-hero__stats">
				<a href="#curriculum" class="courses-hero__cta">
					<span class="courses-hero__cta-fill" aria-hidden="true"></span>
					<span class="courses-hero__cta-label">
						View Curriculum
						<IconArrowRight size={20} class="courses-hero__cta-arrow" />
					</span>
				</a>

				<div class="courses-hero__meta">
					<div class="courses-hero__meta-item">
						<span class="courses-hero__meta-value">
							{Math.floor(studentCounter.current).toLocaleString()}+
						</span>
						<span class="courses-hero__meta-label">Traders</span>
					</div>
					<div class="courses-hero__meta-divider" aria-hidden="true"></div>
					<div class="courses-hero__meta-item">
						<span class="courses-hero__meta-value">4.9/5</span>
						<span class="courses-hero__meta-label">Rating</span>
					</div>
				</div>
			</div>
		</div>

		<div class="ticker-bar courses-hero__ticker">
			<div class="courses-hero__ticker-track">
				{#each Array(2) as _, _ti (_ti)}
					<div class="courses-hero__ticker-row">
						<span class="courses-hero__ticker-cell courses-hero__ticker-cell--up">
							<IconTrendingUp size={14} /> SPY $542.30 +1.2%
						</span>
						<span class="courses-hero__ticker-cell courses-hero__ticker-cell--down">
							<IconActivity size={14} /> VIX 13.45 -2.1%
						</span>
						<span class="courses-hero__ticker-cell courses-hero__ticker-cell--up">
							<IconBolt size={14} /> NVDA $135.20 +3.4%
						</span>
						<span class="courses-hero__ticker-cell courses-hero__ticker-cell--mute">
							BTC $92,430 +0.1%
						</span>
						<span class="courses-hero__ticker-cell courses-hero__ticker-cell--sep">|</span>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="trap">
		<div class="trap__inner">
			<div class="trap__grid">
				<div class="trap__copy">
					<h2 class="trap__heading">
						The <span class="trap__heading-accent">Retail</span> Trap.
					</h2>
					<p class="trap__lede">
						90% of traders fail because they treat the market like a casino. They chase alerts, lack
						a risk model, and trade their P&L instead of the chart.
					</p>

					<div class="trap__cards">
						<div class="trap-card trap-card--bad">
							<IconX class="trap-card__icon" size={32} />
							<h3 class="trap-card__heading">Gambling</h3>
							<p class="trap-card__desc">Entry based on "feeling" rather than statistical edge.</p>
						</div>
						<div class="trap-card trap-card--good">
							<IconCheck class="trap-card__icon" size={32} />
							<h3 class="trap-card__heading">Business</h3>
							<p class="trap-card__desc">Execution based on a pre-defined, backtested playbook.</p>
						</div>
					</div>
				</div>

				<div class="trap__visual">
					<div class="trap__visual-noise" aria-hidden="true"></div>
					<div class="trap__visual-frame">
						<div class="trap__visual-panel">
							<div class="trap__visual-chrome">
								<div class="trap__visual-dots">
									<span class="trap__visual-dot trap__visual-dot--red"></span>
									<span class="trap__visual-dot trap__visual-dot--yellow"></span>
									<span class="trap__visual-dot trap__visual-dot--green"></span>
								</div>
								<div class="trap__visual-label">System: Active</div>
							</div>
							<div class="trap__visual-bars">
								{#each trapVisualBars as bar (bar.id)}
									<div
										class="trap__visual-bar"
										style:height={bar.height}
										style:animation-delay={bar.delay}
										style:opacity={bar.opacity}
									></div>
								{/each}
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="curriculum" class="curriculum">
		<div class="curriculum__inner">
			<div class="curriculum__header">
				<span class="curriculum__eyebrow">Classified Intel</span>
				<h2 class="curriculum__heading">Master The Setup.</h2>
				<p class="curriculum__lede">
					Four specialized pathways designed to take you from unconscious incompetence to
					unconscious competence.
				</p>
			</div>

			<div class="curriculum__grid">
				{#each courses as course (course.id)}
					{@const Icon = course.icon}
					<div
						{@attach spotlightCard}
						class={['spotlight-card', 'courses-card', `courses-card--${course.variant}`]}
					>
						<div class="courses-card__spotlight" aria-hidden="true"></div>

						<div class="courses-card__body">
							<div class="courses-card__header">
								<div class="courses-card__icon">
									<Icon size={32} stroke={1.5} />
								</div>
								<span class="courses-card__level">{course.level}</span>
							</div>

							<h3 class="courses-card__title">{course.title}</h3>
							<p class="courses-card__audience">{course.targetAudience}</p>

							<div class="courses-card__rule" aria-hidden="true"></div>

							<p class="courses-card__desc">{course.description}</p>

							<div class="courses-card__features">
								{#each course.features as feature (feature)}
									<div class="courses-card__feature">
										<IconCheck size={16} class="courses-card__feature-icon" />
										{feature}
									</div>
								{/each}
							</div>

							<div class="courses-card__footer">
								<div class="courses-card__price-block">
									<div class="courses-card__price-label">Investment</div>
									<div class="courses-card__price-value">{course.price}</div>
								</div>
								<a href={`/courses/${course.slug}`} class="courses-card__cta">
									Start <IconArrowRight size={16} />
								</a>
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="faq">
		<div class="faq__inner">
			<h2 class="faq__heading">Protocol FAQ</h2>

			<div class="faq__list">
				{#each faqs as faq, i (faq.question)}
					<div class="faq-item">
						<button
							onclick={() => toggleFaq(i)}
							aria-expanded={openFaqIndex === i}
							aria-controls="faq-panel-{i}"
							class="faq-item__trigger"
						>
							<span class="faq-item__question">{faq.question}</span>
							<IconChevronDown size={20} class={getFaqChevronClass(i)} />
						</button>
						{#if openFaqIndex === i}
							<div
								id="faq-panel-{i}"
								role="region"
								transition:slide={{ duration: 300, easing: cubicOut }}
								class="faq-item__panel"
							>
								<p class="faq-item__answer">{faq.answer}</p>
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="final-cta">
		<div class="final-cta__halo" aria-hidden="true"></div>

		<div class="final-cta__inner">
			<IconRocket size={48} stroke={1} class="final-cta__rocket" />
			<h2 class="final-cta__heading">Market Opens In... Now.</h2>
			<p class="final-cta__lede">
				Join {STUDENT_COUNT.toLocaleString()}+ disciplined traders building wealth through logic,
				not luck.
			</p>

			<a href="#curriculum" class="final-cta__button">
				<IconSchool size={24} />
				Enroll Now
			</a>
		</div>
	</section>
</div>

<style>
	/* ═════════════════════════════════════════════════════════════════════════
	   Page-local tokens
	   Reusable color/spacing values come from --rtp-* (marketing.css).
	   ═════════════════════════════════════════════════════════════════════════ */
	.courses {
		--c-blue: var(--rtp-primary);
		--c-blue-bright: var(--rtp-blue-bright);
		--c-violet: #8b5cf6;
		--c-violet-bright: #a78bfa;
		--c-orange: #f97316;
		--c-orange-bright: #fb923c;

		--c-card-bg: #000;
		--c-card-border: var(--rtp-border);
		--c-card-border-hover: var(--rtp-border-strong);

		background: #000;
		color: var(--rtp-text-soft);
		font-family: var(--rtp-font-sans);
	}
	.courses ::selection {
		background: color-mix(in oklab, var(--rtp-primary) 30%, transparent);
		color: #fff;
	}

	/* ═════════════════════════════════════════════════════════════════════════
	   Hero
	   ═════════════════════════════════════════════════════════════════════════ */
	.courses-hero {
		position: relative;
		min-block-size: 95vh;
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		padding-block-start: 5rem;
	}

	.courses-hero__canvas {
		position: absolute;
		inset: 0;
		z-index: 0;
		opacity: 0.4;
		pointer-events: none;
	}
	.courses-hero__canvas-el {
		inline-size: 100%;
		block-size: 100%;
	}

	.courses-hero__veil {
		position: absolute;
		inset: 0;
		z-index: 0;
		pointer-events: none;
		background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.5), #000);
	}
	.courses-hero__radial {
		position: absolute;
		inset: 0;
		z-index: 0;
		pointer-events: none;
		background: radial-gradient(
			circle at center,
			color-mix(in oklab, #1e3a8a 10%, transparent),
			rgba(0, 0, 0, 0),
			#000
		);
	}

	.courses-hero__inner {
		position: relative;
		z-index: 10;
		max-inline-size: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1.5rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}

	.courses-hero__badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 1rem;
		border-radius: var(--rtp-radius-pill);
		background: color-mix(in oklab, #fff 5%, transparent);
		border: 1px solid var(--rtp-border);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		margin-block-end: 2rem;
		box-shadow: 0 0 15px color-mix(in oklab, #38bdf8 15%, transparent);
	}
	.courses-hero__pulse-wrap {
		position: relative;
		display: inline-flex;
		block-size: 0.5rem;
		inline-size: 0.5rem;
	}
	.courses-hero__pulse-ping {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: var(--rtp-emerald-bright);
		opacity: 0.75;
		animation: courses-ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
	}
	.courses-hero__pulse-dot {
		position: relative;
		display: inline-flex;
		block-size: 0.5rem;
		inline-size: 0.5rem;
		border-radius: 50%;
		background: var(--rtp-emerald);
	}
	.courses-hero__badge-label {
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: var(--rtp-text-soft);
	}

	@keyframes courses-ping {
		75%,
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}

	.courses-hero__title {
		max-inline-size: 64rem;
		margin-inline: auto;
		margin-block-end: 2rem;
		line-height: 0.9;
	}
	.courses-hero__title-shout {
		display: block;
		font-size: 13vw;
		font-weight: 900;
		letter-spacing: -0.04em;
		color: var(--rtp-text);
		mix-blend-mode: overlay;
		opacity: 0.9;
	}
	.courses-hero__title-accent {
		display: block;
		font-size: 2.25rem;
		font-weight: 700;
		letter-spacing: -0.025em;
		background: linear-gradient(
			to right,
			var(--rtp-blue-bright),
			var(--c-violet-bright),
			var(--rtp-text)
		);
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
		margin-block-start: -0.5rem;
	}
	@media (min-width: 768px) {
		.courses-hero__title-shout {
			font-size: 6rem;
		}
		.courses-hero__title-accent {
			font-size: 4.5rem;
			margin-block-start: -1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.courses-hero__title-shout {
			font-size: 7.5rem;
		}
	}

	.courses-hero__desc {
		max-inline-size: 42rem;
		font-size: 1.125rem;
		line-height: 1.625;
		color: var(--rtp-text-muted);
		font-weight: 300;
		margin-block-end: 3rem;
	}
	@media (min-width: 768px) {
		.courses-hero__desc {
			font-size: 1.25rem;
		}
	}
	.courses-hero__desc-impatient {
		color: var(--rtp-red-soft);
		font-weight: 500;
	}
	.courses-hero__desc-disciplined {
		color: var(--rtp-emerald-bright);
		font-weight: 500;
	}

	.courses-hero__stats {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
		padding: 0.5rem;
	}
	@media (min-width: 768px) {
		.courses-hero__stats {
			flex-direction: row;
			gap: 3rem;
		}
	}

	.courses-hero__cta {
		position: relative;
		display: inline-flex;
		align-items: center;
		padding: 1rem 2rem;
		background: #fff;
		color: #000;
		border-radius: var(--rtp-radius-pill);
		font-weight: 700;
		font-size: 1.125rem;
		letter-spacing: -0.025em;
		overflow: hidden;
		text-decoration: none;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.courses-hero__cta:hover {
		transform: scale(1.05);
	}
	.courses-hero__cta:active {
		transform: scale(0.95);
	}
	.courses-hero__cta-fill {
		position: absolute;
		inset: 0;
		background: linear-gradient(to right, #bfdbfe, #fff);
		opacity: 0;
		transition: opacity var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.courses-hero__cta:hover .courses-hero__cta-fill {
		opacity: 1;
	}
	.courses-hero__cta-label {
		position: relative;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}
	:global(.courses-hero__cta-arrow) {
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.courses-hero__cta:hover :global(.courses-hero__cta-arrow) {
		transform: translateX(0.25rem);
	}

	.courses-hero__meta {
		display: flex;
		align-items: center;
		gap: 2rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--rtp-text-subtle);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}
	.courses-hero__meta-item {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	.courses-hero__meta-value {
		color: var(--rtp-text);
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: -0.025em;
		text-transform: none;
	}
	.courses-hero__meta-label {
		text-transform: uppercase;
	}
	.courses-hero__meta-divider {
		inline-size: 1px;
		block-size: 2rem;
		background: var(--rtp-border);
	}

	/* Ticker */
	.courses-hero__ticker {
		position: absolute;
		inset-inline: 0;
		inset-block-end: 0;
		inline-size: 100%;
		border-block-start: 1px solid var(--rtp-border);
		background: color-mix(in oklab, #000 40%, transparent);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		overflow: hidden;
		padding-block: 0.75rem;
		z-index: 20;
	}
	.courses-hero__ticker-track {
		display: flex;
		white-space: nowrap;
		animation: courses-marquee 40s linear infinite;
	}
	.courses-hero__ticker-row {
		display: flex;
		align-items: center;
		gap: 2rem;
		padding-inline: 1rem;
	}
	.courses-hero__ticker-cell {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
	}
	.courses-hero__ticker-cell--up {
		color: var(--rtp-emerald-bright);
	}
	.courses-hero__ticker-cell--down {
		color: var(--rtp-red-soft);
	}
	.courses-hero__ticker-cell--mute {
		color: var(--rtp-text-muted);
	}
	.courses-hero__ticker-cell--sep {
		color: var(--rtp-text-faint);
	}

	@keyframes courses-marquee {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-50%);
		}
	}
	@media (prefers-reduced-motion: reduce) {
		.courses-hero__ticker-track {
			animation: none;
		}
	}

	/* ═════════════════════════════════════════════════════════════════════════
	   Trap section
	   ═════════════════════════════════════════════════════════════════════════ */
	.trap {
		position: relative;
		z-index: 10;
		padding-block: 8rem;
	}
	.trap__inner {
		max-inline-size: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1.5rem;
	}
	.trap__grid {
		display: grid;
		gap: 4rem;
		align-items: center;
	}
	@media (min-width: 1024px) {
		.trap__grid {
			grid-template-columns: 1fr 1fr;
		}
	}

	.trap__copy {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}
	.trap__heading {
		font-size: 2.25rem;
		font-weight: 700;
		color: var(--rtp-text);
		letter-spacing: -0.025em;
	}
	.trap__heading-accent {
		color: var(--rtp-primary);
	}
	@media (min-width: 768px) {
		.trap__heading {
			font-size: 3rem;
		}
	}
	.trap__lede {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
		line-height: 1.625;
	}

	.trap__cards {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
	}
	@media (min-width: 640px) {
		.trap__cards {
			grid-template-columns: 1fr 1fr;
		}
	}

	.trap-card {
		padding: 1.5rem;
		border-radius: 1rem;
	}
	.trap-card--bad {
		background: color-mix(in oklab, var(--rtp-red) 5%, transparent);
		border: 1px solid color-mix(in oklab, var(--rtp-red) 10%, transparent);
	}
	.trap-card--good {
		background: color-mix(in oklab, var(--rtp-emerald) 5%, transparent);
		border: 1px solid color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
	}
	:global(.trap-card--bad .trap-card__icon) {
		color: var(--rtp-red);
		margin-block-end: 1rem;
	}
	:global(.trap-card--good .trap-card__icon) {
		color: var(--rtp-emerald);
		margin-block-end: 1rem;
	}
	.trap-card__heading {
		color: var(--rtp-text);
		font-weight: 700;
		margin-block-end: 0.5rem;
	}
	.trap-card__desc {
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
	}

	.trap__visual {
		position: relative;
		aspect-ratio: 1 / 1;
		border-radius: 1.5rem;
		overflow: hidden;
		border: 1px solid var(--rtp-border);
		background: color-mix(in oklab, #fff 5%, transparent);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
	}
	@media (min-width: 768px) {
		.trap__visual {
			aspect-ratio: 16 / 9;
		}
	}
	@media (min-width: 1024px) {
		.trap__visual {
			aspect-ratio: 1 / 1;
		}
	}
	.trap__visual-noise {
		position: absolute;
		inset: 0;
		background-image: url('https://assets.codepen.io/907368/noise.svg');
		opacity: 0.2;
		pointer-events: none;
	}
	.trap__visual-frame {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.trap__visual-panel {
		position: relative;
		inline-size: 75%;
		block-size: 75%;
		border: 1px solid var(--rtp-border);
		background: color-mix(in oklab, #000 50%, transparent);
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
		border-radius: 0.75rem;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
	}
	.trap__visual-chrome {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: 2rem;
		border-block-end: 1px solid var(--rtp-border-soft);
		padding-block-end: 1rem;
	}
	.trap__visual-dots {
		display: flex;
		gap: 0.5rem;
	}
	.trap__visual-dot {
		inline-size: 0.75rem;
		block-size: 0.75rem;
		border-radius: 50%;
	}
	.trap__visual-dot--red {
		background: color-mix(in oklab, var(--rtp-red) 50%, transparent);
	}
	.trap__visual-dot--yellow {
		background: color-mix(in oklab, #eab308 50%, transparent);
	}
	.trap__visual-dot--green {
		background: color-mix(in oklab, #22c55e 50%, transparent);
	}
	.trap__visual-label {
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-text-subtle);
		text-transform: uppercase;
	}
	.trap__visual-bars {
		flex: 1;
		display: flex;
		align-items: flex-end;
		gap: 0.25rem;
	}
	.trap__visual-bar {
		flex: 1;
		background: color-mix(in oklab, var(--rtp-primary) 50%, transparent);
		border-start-start-radius: 0.125rem;
		border-start-end-radius: 0.125rem;
		animation: courses-pulse 2s ease-in-out infinite;
	}
	@keyframes courses-pulse {
		0%,
		100% {
			opacity: 0.5;
		}
		50% {
			opacity: 1;
		}
	}
	@media (prefers-reduced-motion: reduce) {
		.trap__visual-bar {
			animation: none;
		}
	}

	/* ═════════════════════════════════════════════════════════════════════════
	   Curriculum grid
	   ═════════════════════════════════════════════════════════════════════════ */
	.curriculum {
		position: relative;
		z-index: 10;
		padding-block: 8rem;
		background: color-mix(in oklab, #0f172a 50%, transparent);
		border-block: 1px solid var(--rtp-border-soft);
	}
	.curriculum__inner {
		max-inline-size: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1.5rem;
	}
	.curriculum__header {
		text-align: center;
		max-inline-size: 48rem;
		margin-inline: auto;
		margin-block-end: 5rem;
	}
	.curriculum__eyebrow {
		display: block;
		color: var(--rtp-primary);
		font-family: var(--rtp-font-mono);
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.3em;
		margin-block-end: 1rem;
	}
	.curriculum__heading {
		font-size: 2.25rem;
		font-weight: 700;
		letter-spacing: -0.05em;
		color: var(--rtp-text);
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.curriculum__heading {
			font-size: 3.75rem;
		}
	}
	.curriculum__lede {
		color: var(--rtp-text-muted);
		font-size: 1.125rem;
	}

	.curriculum__grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
		max-inline-size: 72rem;
		margin-inline: auto;
	}
	@media (min-width: 768px) {
		.curriculum__grid {
			grid-template-columns: 1fr 1fr;
		}
	}
	@media (min-width: 1024px) {
		.curriculum__grid {
			gap: 2rem;
		}
	}

	/* ── Course card ── */
	.courses-card {
		position: relative;
		block-size: 100%;
		background: var(--c-card-bg);
		border: 1px solid var(--c-card-border);
		border-radius: 1.5rem;
		overflow: hidden;
		transition: border-color 500ms;
	}
	.courses-card:hover {
		border-color: var(--c-card-border-hover);
	}
	.courses-card__spotlight {
		position: absolute;
		inset: -1px;
		pointer-events: none;
		opacity: 0;
		transition: opacity 300ms;
		background: radial-gradient(
			600px circle at var(--mouse-x) var(--mouse-y),
			rgba(255, 255, 255, 0.06),
			transparent 40%
		);
		z-index: 0;
	}
	.courses-card:hover .courses-card__spotlight {
		opacity: 1;
	}
	.courses-card__body {
		position: relative;
		z-index: 10;
		padding: 2rem;
		display: flex;
		flex-direction: column;
		block-size: 100%;
		background: color-mix(in oklab, #000 40%, transparent);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}
	@media (min-width: 768px) {
		.courses-card__body {
			padding: 2.5rem;
		}
	}

	.courses-card__header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-block-end: 2rem;
	}
	.courses-card__icon {
		padding: 0.75rem;
		border-radius: 1rem;
		border: 1px solid var(--card-accent-border, var(--rtp-border));
		background: var(--card-accent-bg, transparent);
		color: var(--card-accent-fg, var(--rtp-primary));
	}
	.courses-card__level {
		padding: 0.25rem 0.75rem;
		border-radius: var(--rtp-radius-pill);
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: color-mix(in oklab, #fff 5%, transparent);
		border: 1px solid var(--rtp-border);
		color: var(--rtp-text-soft);
	}

	.courses-card__title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.5rem;
		transition: color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.courses-card:hover .courses-card__title {
		color: var(--rtp-blue-bright);
	}
	.courses-card__audience {
		font-size: 0.875rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-text-subtle);
		margin-block-end: 1.5rem;
	}
	.courses-card__rule {
		block-size: 1px;
		inline-size: 100%;
		background: var(--rtp-border-soft);
		margin-block-end: 1.5rem;
	}
	.courses-card__desc {
		color: var(--rtp-text-muted);
		line-height: 1.625;
		margin-block-end: 2rem;
		flex-grow: 1;
	}

	.courses-card__features {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-block-end: 2rem;
	}
	.courses-card__feature {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.875rem;
		color: var(--rtp-text-soft);
	}
	:global(.courses-card__feature-icon) {
		color: var(--card-accent-fg, var(--rtp-primary));
	}

	.courses-card__footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-block-start: 1.5rem;
		border-block-start: 1px solid var(--rtp-border-soft);
	}
	.courses-card__price-label {
		font-size: 0.75rem;
		color: var(--rtp-text-subtle);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-block-end: 0.25rem;
	}
	.courses-card__price-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--rtp-text);
	}
	.courses-card__cta {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		border-radius: var(--rtp-radius-sm);
		background: #fff;
		color: #000;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		transition: background-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.courses-card__cta:hover {
		background: var(--rtp-text-soft);
	}

	/* ── Card variant accents ── */
	.courses-card--blue {
		--card-accent-bg: color-mix(in oklab, var(--c-blue) 10%, transparent);
		--card-accent-border: color-mix(in oklab, var(--c-blue) 20%, transparent);
		--card-accent-fg: var(--rtp-blue-bright);
	}
	.courses-card--emerald {
		--card-accent-bg: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		--card-accent-border: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		--card-accent-fg: var(--rtp-emerald-bright);
	}
	.courses-card--violet {
		--card-accent-bg: color-mix(in oklab, var(--c-violet) 10%, transparent);
		--card-accent-border: color-mix(in oklab, var(--c-violet) 20%, transparent);
		--card-accent-fg: var(--c-violet-bright);
	}
	.courses-card--orange {
		--card-accent-bg: color-mix(in oklab, var(--c-orange) 10%, transparent);
		--card-accent-border: color-mix(in oklab, var(--c-orange) 20%, transparent);
		--card-accent-fg: var(--c-orange-bright);
	}

	/* ═════════════════════════════════════════════════════════════════════════
	   FAQ
	   ═════════════════════════════════════════════════════════════════════════ */
	.faq {
		position: relative;
		z-index: 10;
		padding-block: 8rem;
		background: #000;
	}
	.faq__inner {
		max-inline-size: 48rem;
		margin-inline: auto;
		padding-inline: 1.5rem;
	}
	.faq__heading {
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		text-align: center;
		margin-block-end: 3rem;
	}
	.faq__list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.faq-item {
		border-radius: 0.75rem;
		border: 1px solid var(--rtp-border-soft);
		overflow: hidden;
	}
	.faq-item__trigger {
		inline-size: 100%;
		text-align: start;
		background: color-mix(in oklab, #fff 2%, transparent);
		padding: 1.5rem;
		display: flex;
		align-items: center;
		justify-content: space-between;
		border: none;
		cursor: pointer;
		color: var(--rtp-text-soft);
		transition: background-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.faq-item__trigger:hover {
		background: color-mix(in oklab, #fff 4%, transparent);
	}
	.faq-item__question {
		font-size: 1.125rem;
		font-weight: 500;
		color: var(--rtp-text-soft);
		transition: color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.faq-item__trigger:hover .faq-item__question {
		color: var(--rtp-text);
	}
	:global(.faq-item__chevron) {
		color: var(--rtp-text-subtle);
		transition: transform 300ms;
	}
	:global(.faq-item__chevron--open) {
		transform: rotate(180deg);
		color: var(--rtp-blue-bright);
	}
	.faq-item__panel {
		padding-inline: 1.5rem;
		padding-block-end: 1.5rem;
		background: color-mix(in oklab, #fff 2%, transparent);
	}
	.faq-item__answer {
		color: var(--rtp-text-muted);
		line-height: 1.625;
		padding-block-start: 0.5rem;
		border-block-start: 1px solid var(--rtp-border-soft);
	}

	/* ═════════════════════════════════════════════════════════════════════════
	   Final CTA
	   ═════════════════════════════════════════════════════════════════════════ */
	.final-cta {
		position: relative;
		padding-block: 10rem;
		overflow: hidden;
	}
	.final-cta__halo {
		position: absolute;
		inset: 0;
		background: color-mix(in oklab, #2563eb 5%, transparent);
		filter: blur(100px);
		pointer-events: none;
	}
	.final-cta__inner {
		position: relative;
		z-index: 10;
		max-inline-size: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1.5rem;
		text-align: center;
	}
	:global(.final-cta__rocket) {
		display: block;
		margin-inline: auto;
		color: var(--rtp-primary);
		margin-block-end: 2rem;
		animation: courses-bounce 1s infinite;
	}
	@keyframes courses-bounce {
		0%,
		100% {
			transform: translateY(-25%);
			animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
		}
		50% {
			transform: translateY(0);
			animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
		}
	}
	@media (prefers-reduced-motion: reduce) {
		:global(.final-cta__rocket) {
			animation: none;
		}
	}
	.final-cta__heading {
		font-size: 3rem;
		font-weight: 900;
		letter-spacing: -0.05em;
		color: var(--rtp-text);
		margin-block-end: 2rem;
	}
	@media (min-width: 768px) {
		.final-cta__heading {
			font-size: 4.5rem;
		}
	}
	.final-cta__lede {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
		margin-block-end: 3rem;
		max-inline-size: 36rem;
		margin-inline: auto;
	}
	.final-cta__button {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1.25rem 2.5rem;
		background: #2563eb;
		color: var(--rtp-text);
		border-radius: var(--rtp-radius-pill);
		font-weight: 700;
		font-size: 1.125rem;
		text-decoration: none;
		transition: all var(--rtp-dur-base) var(--rtp-ease-out);
		box-shadow: 0 0 40px color-mix(in oklab, #2563eb 30%, transparent);
	}
	.final-cta__button:hover {
		background: var(--rtp-primary);
		box-shadow: 0 0 60px color-mix(in oklab, #2563eb 50%, transparent);
	}

	/* ═════════════════════════════════════════════════════════════════════════
	   Mobile refinement (≤359px)
	   ═════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 359px) {
		.courses-hero__inner {
			padding-inline: 0.75rem;
		}
		.courses-hero__title-shout {
			font-size: 11vw;
		}
		.courses-card {
			border-radius: 1rem;
		}
		.courses-card__body {
			padding: 1rem;
		}
	}

	@media (min-width: 360px) and (max-width: 639px) {
		.courses-hero__inner {
			padding-inline: 1rem;
		}
		.courses-card {
			border-radius: 1.25rem;
		}
		.courses-hero__ticker {
			font-size: 0.7rem;
		}
	}

	@media (min-width: 768px) and (max-width: 1023px) {
		.courses-card__body {
			padding: 1.5rem;
		}
	}

	/* Touch targets — minimum 44x44px for interactive elements on coarse pointers */
	@media (hover: none) and (pointer: coarse) {
		.courses-hero__cta,
		.courses-card__cta,
		.final-cta__button,
		.faq-item__trigger {
			min-block-size: 44px;
			min-inline-size: 44px;
		}
	}
</style>
