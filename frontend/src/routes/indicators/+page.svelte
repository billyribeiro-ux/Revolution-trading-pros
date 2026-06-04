<!--
	URL: /indicators
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { SvelteSet } from 'svelte/reactivity';
	import type { Attachment } from 'svelte/attachments';

	// Icons (only those used directly in template)
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconTarget from '@tabler/icons-svelte-runes/icons/target';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconStar from '@tabler/icons-svelte-runes/icons/star';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconSchool from '@tabler/icons-svelte-runes/icons/school';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';

	// Data & types extracted for maintainability
	import { indicators, goldenSetup, faqs, categories } from './data';

	// --- State Management (Svelte 5 Runes) ---
	let heroVisible = $state(false);
	let visibleCards = new SvelteSet<string>();
	let selectedCategory = $state('All');
	let openFaq = $state<number | null>(null);
	let pageRef = $state<HTMLElement | null>(null);

	let filteredIndicators = $derived(
		selectedCategory === 'All'
			? indicators
			: indicators.filter((ind) => ind.category === selectedCategory)
	);

	function toggleFaq(index: number) {
		openFaq = openFaq === index ? null : index;
	}

	// --- Mouse Spotlight Logic ---
	function handleMouseMove(e: MouseEvent) {
		if (!browser) return;
		const target = e.currentTarget as HTMLElement;
		const rect = target.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const y = e.clientY - rect.top;
		target.style.setProperty('--mouse-x', `${x}px`);
		target.style.setProperty('--mouse-y', `${y}px`);
	}

	const capturePage: Attachment<HTMLDivElement> = (node) => {
		pageRef = node;

		return () => {
			if (pageRef === node) {
				pageRef = null;
			}
		};
	};

	function revealIndicatorCard(indicatorId: string): Attachment<HTMLElement> {
		return (node) => {
			if (!browser) return;

			const observer = new IntersectionObserver(
				(entries) => {
					if (entries[0]?.isIntersecting) {
						visibleCards.add(indicatorId);
						observer.disconnect();
					}
				},
				{ threshold: 0.1 }
			);

			observer.observe(node);

			return () => {
				observer.disconnect();
			};
		};
	}

	onMount(() => {
		let heroObserver: IntersectionObserver | null = null;
		let gsapContext: ReturnType<typeof import('gsap').gsap.context> | null = null;
		let cancelled = false;

		// Use IIFE for async operations
		void (async () => {
			const root = pageRef;
			if (!root) return;

			const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
			if (prefersReducedMotion) return;

			try {
				// Dynamically import GSAP to avoid SSR issues
				const { gsap } = await import('gsap');
				const ScrollTrigger = (await import('gsap/ScrollTrigger')).default;
				if (cancelled || pageRef !== root) return;

				gsap.registerPlugin(ScrollTrigger);

				// Use gsap.context() for scoped cleanup - prevents global ScrollTrigger destruction
				gsapContext = gsap.context(() => {
					// --- Cinematic GSAP Animations ---

					// 1. Hero Parallax & Reveal
					const heroTitleAccent = root.querySelector('.hero-title span');
					if (heroTitleAccent) {
						const tlHero = gsap.timeline();
						tlHero.to(heroTitleAccent, {
							backgroundPosition: '200% center',
							duration: 2,
							ease: 'none',
							repeat: -1
						});
					}

					// 2. Truth Section - Animated Chart Drawing
					const truthSection = root.querySelector('.truth-section');
					const chartPath = root.querySelector('.mockup-price-path');
					const indicatorPath = root.querySelector('.mockup-indicator-path');
					const annotation = root.querySelector('.mockup-annotation');

					if (truthSection && chartPath && indicatorPath) {
						const chartLength = (chartPath as SVGPathElement).getTotalLength();
						const indLength = (indicatorPath as SVGPathElement).getTotalLength();

						gsap.set(chartPath, { strokeDasharray: chartLength, strokeDashoffset: chartLength });
						gsap.set(indicatorPath, { strokeDasharray: indLength, strokeDashoffset: indLength });
						gsap.set(annotation, { opacity: 0, scale: 0.8, y: 10 });

						ScrollTrigger.create({
							trigger: truthSection,
							start: 'top 60%',
							onEnter: () => {
								gsap.to(chartPath, { strokeDashoffset: 0, duration: 2, ease: 'power2.out' });
								gsap.to(indicatorPath, {
									strokeDashoffset: 0,
									duration: 2,
									delay: 0.2,
									ease: 'power2.out'
								});
								gsap.to(annotation, {
									opacity: 1,
									scale: 1,
									y: 0,
									duration: 0.5,
									delay: 1.8,
									ease: 'back.out(1.7)'
								});
							}
						});
					}

					// 3. Confluence Flow Animation
					const confluenceSection = root.querySelector('.confluence-section');
					const connectors = root.querySelectorAll('.confluence-connector');
					const steps = root.querySelectorAll('.confluence-step');

					if (confluenceSection && steps.length >= 3 && connectors.length >= 2) {
						const tlConfluence = gsap.timeline({
							scrollTrigger: {
								trigger: confluenceSection,
								start: 'top 70%'
							}
						});

						tlConfluence
							.from(steps[0], { y: 30, opacity: 0, duration: 0.5 })
							.from(connectors[0], { scale: 0, opacity: 0, duration: 0.3 }, '-=0.1')
							.from(steps[1], { y: 30, opacity: 0, duration: 0.5 }, '-=0.1')
							.from(connectors[1], { scale: 0, opacity: 0, duration: 0.3 }, '-=0.1')
							.from(steps[2], { y: 30, opacity: 0, duration: 0.5 }, '-=0.1');
					}

					// 4. Setup Section Animation
					const setupGrid = root.querySelector('.setup-grid');
					const setupItems = root.querySelectorAll('.setup-item');
					if (setupGrid && setupItems.length > 0) {
						gsap.from(setupItems, {
							scrollTrigger: {
								trigger: setupGrid,
								start: 'top 80%'
							},
							y: 20,
							opacity: 0,
							duration: 0.4,
							stagger: 0.1,
							ease: 'power2.out'
						});
					}
				}, root); // Close gsap.context()
			} catch (error) {
				if (!cancelled) {
					console.error('Failed to load indicators page animations', error);
				}
			}
		})();

		// --- Original Observer Logic (Preserved for compatibility) ---
		heroObserver = new IntersectionObserver(
			(entries) => {
				if (cancelled) return;

				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						heroVisible = true;
					}
				});
			},
			{ threshold: 0.2 }
		);

		const heroElement = pageRef?.querySelector('.hero-section');
		if (heroElement) heroObserver.observe(heroElement);

		return () => {
			cancelled = true;
			heroObserver?.disconnect();
			if (gsapContext) gsapContext.revert();
		};
	});
</script>

<div class="indicators-page" {@attach capturePage}>
	<div
		class="indicators-page__grain"
		aria-hidden="true"
		style="background-image: url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0IiBoZWlnaHQ9IjQiPgo8cmVjdCB3aWR0aD0iNCIgaGVpZ2h0PSI0IiBmaWxsPSIjZmZmIi8+CjxyZWN0IHdpZHRoPSIxIiBoZWlnaHQ9IjEiIGZpbGw9IiMwMDAiLz4KPC9zdmc+');"
	></div>

	<section class={['hero-section', { visible: heroVisible }]}>
		<div class="hero-background">
			<div class="glow-orb glow-orb-1"></div>
			<div class="glow-orb glow-orb-2"></div>
			<div class="glow-orb glow-orb-3"></div>

			<div class="perspective-grid"></div>

			<div class="chart-lines">
				<svg aria-hidden="true" class="chart-lines__svg" preserveAspectRatio="none">
					<path
						class="chart-line-svg line-1"
						d="M0,300 Q400,250 800,350 T1600,200"
						fill="none"
						stroke="#2e8eff"
						stroke-width="2"
					/>
					<path
						class="chart-line-svg line-2"
						d="M0,350 Q400,300 800,400 T1600,250"
						fill="none"
						stroke="#34d399"
						stroke-width="2"
					/>
				</svg>
			</div>
		</div>

		<div class="hero-content">
			<div class="hero-badge animate-float">
				<IconChartLine size={18} stroke={2} />
				<span class="hero-badge__label">The Professional Toolkit</span>
			</div>

			<h1 class="hero-title">
				Master the Tools<br />
				<span class="gradient-text gradient-text--animated">Pro Traders Use</span>
			</h1>

			<p class="hero-description">
				Stop looking for a "magic bullet." Successful trading isn't about finding the perfect
				indicator—it's about interpreting the data correctly. We teach you how to use
				institutional-grade tools like VWAP and RSI to read the market's narrative, not just its
				noise.
			</p>

			<div class="hero-cta-group">
				<a class="cta-button primary" href="/live-trading-rooms/day-trading">
					<span class="cta-button__label">
						Join the Live Room
						<IconArrowRight size={20} class="cta-button__arrow" />
					</span>
					<div class="cta-button__fill" aria-hidden="true"></div>
				</a>
				<a class="cta-button secondary" href="#indicators-list">Explore Indicators</a>
			</div>

			<div class="hero-stats">
				<div class="stat-item stat-item--blue">
					<div class="stat-item__icon-wrap">
						<IconTarget size={28} stroke={1.5} class="stat-icon stat-icon--blue" />
					</div>
					<div class="stat-content">
						<div class="stat-value">Precision</div>
						<div class="stat-label">Entries & Exits</div>
					</div>
				</div>
				<div class="stat-item stat-item--emerald">
					<div class="stat-item__icon-wrap">
						<IconBolt size={28} stroke={1.5} class="stat-icon stat-icon--emerald" />
					</div>
					<div class="stat-content">
						<div class="stat-value">Real-Time</div>
						<div class="stat-label">Live Application</div>
					</div>
				</div>
				<div class="stat-item stat-item--purple">
					<div class="stat-item__icon-wrap">
						<IconUsers size={28} stroke={1.5} class="stat-icon stat-icon--purple" />
					</div>
					<div class="stat-content">
						<div class="stat-value">Community</div>
						<div class="stat-label">Mentorship</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="setup-section">
		<div class="section-container">
			<div class="setup-header">
				<h2 class="section-title">The "Golden Setup"</h2>
				<p class="section-subtitle">
					You don't need 20 indicators. You need 3 that work. Here is the exact configuration we use
					every day.
				</p>
			</div>

			<div class="setup-grid">
				{#each goldenSetup as item (item.title)}
					{@const Icon = item.icon}
					<div class="setup-item">
						<div class="setup-icon-wrapper">
							<Icon size={24} class="setup-icon" />
						</div>
						<h3 class="setup-item__title">{item.title}</h3>
						<div class="setup-item__value">{item.value}</div>
						<div class="setup-item__detail">{item.detail}</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="truth-section">
		<div class="truth-section__fade" aria-hidden="true"></div>

		<div class="section-container truth-section__inner">
			<div class="truth-grid">
				<div class="truth-content">
					<h2 class="section-title left-align truth-section__title">
						The Truth About <span class="text-highlight">Technical Indicators</span>
					</h2>
					<p class="truth-text">
						Most new traders fail because they treat indicators as "Go/Stop" signals. They buy when
						the RSI crosses 30, or sell when lines cross, without understanding context.
					</p>
					<p class="truth-text">
						<strong>Here is the reality:</strong> Indicators are just derivatives of price. They are tools,
						not crystal balls. In our community, we teach you to use these tools to build a case—a "confluence"
						of evidence—that tilts the probabilities in your favor.
					</p>
					<ul class="truth-list">
						<li class="truth-list__item">
							<IconAlertTriangle size={24} class="warning-icon" />
							<span>Stop chasing "lagging" signals blindly.</span>
						</li>
						<li class="truth-list__item">
							<IconCheck size={24} class="check-icon" />
							<span>Start using indicators to confirm price action.</span>
						</li>
						<li class="truth-list__item">
							<IconCheck size={24} class="check-icon" />
							<span>Learn to spot what the institutions are doing.</span>
						</li>
					</ul>
				</div>
				<div class="truth-visual">
					<div class="chart-card">
						<div class="chart-header">
							<div class="chart-header__dots">
								<span class="dot red"></span>
								<span class="dot yellow"></span>
								<span class="dot green"></span>
							</div>
							<div class="chart-header__label">M15 • ES_F</div>
						</div>
						<div class="chart-mockup">
							<svg aria-hidden="true" class="chart-mockup__svg" viewBox="0 0 400 200">
								<defs>
									<linearGradient id="chartGrad" x1="0%" y1="0%" x2="0%" y2="100%">
										<stop offset="0%" stop-color="#2e8eff" stop-opacity="0.3" />
										<stop offset="100%" stop-color="#2e8eff" stop-opacity="0" />
									</linearGradient>
								</defs>
								<path
									class="mockup-price-path"
									d="M0,150 L50,140 L80,160 L120,100 L160,120 L220,50 L260,80 L350,20"
									fill="none"
									stroke="#2e8eff"
									stroke-width="3"
									stroke-linecap="round"
									stroke-linejoin="round"
								/>

								<path
									class="mockup-indicator-path"
									d="M0,180 L50,175 L80,185 L120,170 L160,175 L220,172 L260,178 L350,180"
									fill="none"
									stroke="#f59e0b"
									stroke-width="2"
									stroke-dasharray="4 4"
									opacity="0.8"
								/>
							</svg>

							<div class="mockup-annotation">
								<div class="mockup-annotation__row">
									<IconAlertTriangle size={14} />
									<span>Bearish Divergence</span>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="filter-section" id="indicators-list">
		<div class="filter-container">
			<h3 class="filter-section__title">Explore Our Core Indicators</h3>
			<p class="filter-subtitle">
				These are the exact tools active in our trading room charts right now.
			</p>
			<div class="filter-buttons">
				{#each categories as category (category)}
					<button
						class={['filter-button', { active: selectedCategory === category }]}
						onclick={() => (selectedCategory = category)}
					>
						{category}
					</button>
				{/each}
			</div>
		</div>
	</section>

	<section class="indicators-section">
		<div class="section-container">
			<div class="indicators-grid">
				{#each filteredIndicators as indicator, index (indicator.id)}
					{@const Icon = indicator.icon}
					<article
						{@attach revealIndicatorCard(indicator.id)}
						class={['indicator-card', { visible: visibleCards.has(indicator.id) }]}
						onmousemove={handleMouseMove}
						style="--delay: {index * 0.1}s; --card-color: {indicator.color};"
					>
						<div class="spotlight-overlay" aria-hidden="true"></div>

						<div class="card-header" style="background: {indicator.gradient}">
							<div class="bg-noise" aria-hidden="true"></div>

							<div class="card-icon">
								<Icon size={48} stroke={1.5} />
							</div>
							<div class="card-category">{indicator.category}</div>
						</div>

						<div class="card-content">
							<h3 class="card-title">
								{indicator.name}
							</h3>
							<p class="card-description">{indicator.description}</p>

							<div class="card-use-case">
								<IconTarget size={18} stroke={2} />
								<span>{indicator.useCase}</span>
							</div>

							<div class="card-meta">
								<div class="meta-badge difficulty">
									<IconStar size={16} stroke={2} />
									<span>{indicator.difficulty}</span>
								</div>
							</div>

							<ul class="card-features">
								{#each indicator.features as feature (feature)}
									<li>
										<IconCheck size={16} stroke={2} />
										<span>{feature}</span>
									</li>
								{/each}
							</ul>

							<a href="/indicators/{indicator.slug}" class="card-button">
								View Strategy Guide
								<IconArrowRight size={18} stroke={2} class="card-button__arrow" />
							</a>
						</div>
					</article>
				{/each}
			</div>
		</div>
	</section>

	<section class="confluence-section">
		<div class="confluence-section__fade" aria-hidden="true"></div>
		<div class="section-container confluence-section__inner">
			<h2 class="section-title">
				The Power of <span class="gradient-text">Confluence</span>
			</h2>
			<p class="section-subtitle">
				A single indicator can be wrong. Three indicators telling the same story are rarely wrong.
				This is how we find high-probability trades.
			</p>

			<div class="confluence-grid">
				<div class="confluence-step confluence-step--blue">
					<div class="step-number step-number--blue">1</div>
					<h3 class="confluence-step__title">Trend</h3>
					<p class="confluence-step__desc">
						We use <strong>Moving Averages</strong> to determine if the market is bullish or bearish.
						We never fight the trend.
					</p>
				</div>

				<div class="confluence-connector confluence-connector--horizontal">
					<IconArrowRight size={32} class="confluence-connector__icon" />
				</div>
				<div class="confluence-connector confluence-connector--vertical">
					<IconArrowRight size={32} class="confluence-connector__icon" />
				</div>

				<div class="confluence-step confluence-step--emerald">
					<div class="step-number step-number--emerald">2</div>
					<h3 class="confluence-step__title">Location</h3>
					<p class="confluence-step__desc">
						We wait for price to return to value areas like <strong>VWAP</strong> or Support zones. We
						don't chase extended moves.
					</p>
				</div>

				<div class="confluence-connector confluence-connector--horizontal">
					<IconArrowRight size={32} class="confluence-connector__icon" />
				</div>
				<div class="confluence-connector confluence-connector--vertical">
					<IconArrowRight size={32} class="confluence-connector__icon" />
				</div>

				<div class="confluence-step confluence-step--purple">
					<div class="step-number step-number--purple">3</div>
					<h3 class="confluence-step__title">Momentum</h3>
					<p class="confluence-step__desc">
						We execute when <strong>RSI or MACD</strong> confirms the buyers are stepping back in. This
						creates the "sniper" entry.
					</p>
				</div>
			</div>

			<div class="confluence-cta">
				<p class="confluence-cta__lede">Want to see this confluence strategy in action?</p>
				<a href="/live-trading-rooms/day-trading" class="text-link">
					Watch us trade this setup live tomorrow morning
					<IconArrowRight size={18} class="text-link__arrow" />
				</a>
			</div>
		</div>
	</section>

	<section class="faq-section">
		<div class="section-container">
			<h2 class="section-title">Common Questions</h2>
			<div class="faq-list">
				{#each faqs as faq, i (i)}
					<div class={['faq-item', { open: openFaq === i }]}>
						<button class="faq-question" onclick={() => toggleFaq(i)}>
							{faq.question}
							<div class={['faq-icon', { 'faq-icon--open': openFaq === i }]}>
								<IconChevronDown size={20} />
							</div>
						</button>
						<div class="faq-answer">
							<p class="faq-answer__text">{faq.answer}</p>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="final-cta">
		<div class="final-cta__halo" aria-hidden="true"></div>

		<div class="cta-content">
			<div class="final-cta__icon-wrap">
				<IconSchool size={48} class="cta-icon" />
			</div>
			<h2 class="final-cta__heading">Stop Learning Alone. Start Trading Together.</h2>
			<p class="final-cta__lede">
				You have the tools. Now get the guidance. Join a community of traders who genuinely care
				about your success.
			</p>
			<a class="cta-button primary large" href="/signup">Join Revolution Trading Pros</a>
			<p class="cta-subtext">No spam. No fake alerts. Just real trading.</p>
		</div>
	</section>
</div>

<style>
	/* ═════════════════════════════════════════════════════════════════════════
	   Page-local tokens (one-offs for /indicators only).
	   Reusable color/spacing values come from --rtp-* in marketing.css.
	   ═════════════════════════════════════════════════════════════════════════ */
	.indicators-page {
		--ind-bg-navy: #050812;
		--ind-bg-navy-2: #0a0f1e;
		--ind-bg-navy-3: #080c18;
		--ind-card-bg: #0b101e;
		--ind-blue: #2e8eff;
		--ind-blue-soft: var(--rtp-blue-bright);
		--ind-emerald: var(--rtp-emerald-bright);
		--ind-purple: #a78bfa;
		--ind-purple-strong: #8b5cf6;
		--ind-amber: var(--rtp-amber);

		background: var(--ind-bg-navy);
		color: var(--rtp-text);
		font-family: var(--rtp-font-sans);
		overflow-x: hidden;
		position: relative;
	}

	.indicators-page__grain {
		position: fixed;
		inset: 0;
		pointer-events: none;
		opacity: 0.03;
		z-index: 1;
		mix-blend-mode: overlay;
	}

	/* Cinematic Spotlight Effect */
	.spotlight-overlay {
		pointer-events: none;
		position: absolute;
		inset: 0;
		background: radial-gradient(
			600px circle at var(--mouse-x) var(--mouse-y),
			rgba(255, 255, 255, 0.06),
			transparent 40%
		);
		z-index: 2;
		opacity: 0;
		transition: opacity 500ms;
	}
	.indicator-card:hover .spotlight-overlay {
		opacity: 1;
	}

	.bg-noise {
		position: absolute;
		inset: 0;
		opacity: 0.2;
		pointer-events: none;
		background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)' opacity='1'/%3E%3C/svg%3E");
	}

	/* Common Utilities */
	.section-container {
		max-width: 1400px;
		margin: 0 auto;
		padding: 0 2rem;
	}

	.section-title {
		font-size: clamp(2rem, 4vw, 3.5rem);
		font-weight: 800;
		text-align: center;
		margin-bottom: 1.5rem;
		line-height: 1.1;
	}

	.section-subtitle {
		text-align: center;
		max-width: 800px;
		margin: 0 auto 4rem;
		color: #94a3b8;
		font-size: 1.125rem;
		line-height: 1.6;
	}

	.gradient-text {
		background: linear-gradient(135deg, #2e8eff 0%, #34d399 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.text-highlight {
		color: #2e8eff;
	}

	/* Hero Section */
	.hero-section {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 8rem 2rem 6rem;
		overflow: hidden;
		opacity: 0;
		transform: translateY(30px);
		transition:
			opacity 1s cubic-bezier(0.16, 1, 0.3, 1),
			transform 1s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.hero-section.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.hero-background {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		overflow: hidden;
		z-index: 0;
	}

	.perspective-grid {
		background-image:
			linear-gradient(rgba(46, 142, 255, 0.1) 1px, transparent 1px),
			linear-gradient(90deg, rgba(46, 142, 255, 0.1) 1px, transparent 1px);
		background-size: 40px 40px;
		transform: perspective(500px) rotateX(60deg) translateY(-100px) scale(2);
		animation: gridMove 20s linear infinite;
		mask-image: linear-gradient(to bottom, transparent, black 40%, transparent);
	}

	@keyframes gridMove {
		0% {
			transform: perspective(500px) rotateX(60deg) translateY(0) scale(2);
		}
		100% {
			transform: perspective(500px) rotateX(60deg) translateY(40px) scale(2);
		}
	}

	.glow-orb {
		position: absolute;
		border-radius: 50%;
		filter: blur(100px);
		opacity: 0.4;
		animation: pulse 8s ease-in-out infinite;
	}

	.glow-orb-1 {
		width: 800px;
		height: 800px;
		background: radial-gradient(circle, #2e8eff 0%, transparent 70%);
		top: -300px;
		right: -200px;
	}

	.glow-orb-2 {
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, #34d399 0%, transparent 70%);
		bottom: -250px;
		left: -150px;
		animation-delay: 2s;
	}

	.glow-orb-3 {
		width: 400px;
		height: 400px;
		background: radial-gradient(circle, #a78bfa 0%, transparent 70%);
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		animation-delay: 4s;
		opacity: 0.2;
	}

	.chart-lines {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		pointer-events: none;
	}

	.chart-line-svg {
		stroke-dasharray: 2000;
		stroke-dashoffset: 2000;
		animation: drawLine 4s ease-out forwards infinite;
		opacity: 0.3;
	}

	.line-2 {
		animation-delay: 2s;
	}

	@keyframes pulse {
		0%,
		100% {
			transform: scale(1);
			opacity: 0.3;
		}
		50% {
			transform: scale(1.1);
			opacity: 0.5;
		}
	}

	@keyframes drawLine {
		0% {
			stroke-dashoffset: 2000;
			opacity: 0;
		}
		20% {
			opacity: 0.4;
		}
		100% {
			stroke-dashoffset: 0;
			opacity: 0;
		}
	}

	.animate-float {
		animation: float 6s ease-in-out infinite;
	}

	@keyframes float {
		0%,
		100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-10px);
		}
	}

	.hero-content {
		max-width: 1000px;
		text-align: center;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.hero-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1.25rem;
		background: rgba(46, 142, 255, 0.1);
		border: 1px solid rgba(46, 142, 255, 0.2);
		border-radius: 50px;
		color: #60a5fa;
		margin-bottom: 2rem;
		backdrop-filter: blur(10px);
	}

	.hero-title {
		font-size: clamp(3rem, 6vw, 5.5rem);
		font-weight: 800;
		line-height: 1;
		margin-bottom: 1.5rem;
		letter-spacing: -0.03em;
	}

	/* CTA Buttons */
	.hero-cta-group {
		display: flex;
		gap: 1rem;
		margin-bottom: 4rem;
		flex-wrap: wrap;
		justify-content: center;
	}

	.cta-button {
		padding: 1rem 2.5rem;
		border-radius: 50px;
		font-weight: 600;
		font-size: 1rem;
		cursor: pointer;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		text-decoration: none;
		border: none;
		letter-spacing: -0.01em;
	}

	.cta-button.primary {
		background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
		color: white;
		box-shadow: 0 4px 15px rgba(37, 99, 235, 0.3);
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.cta-button.primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(37, 99, 235, 0.4);
	}

	.cta-button.secondary {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.08);
		color: white;
	}

	.cta-button.secondary:hover {
		background: rgba(255, 255, 255, 0.08);
		border-color: rgba(255, 255, 255, 0.2);
	}

	.hero-stats {
		display: flex;
		justify-content: center;
		gap: 4rem;
		flex-wrap: wrap;
		width: 100%;
	}

	.stat-item {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	/* Setup Section (New) */
	.setup-section {
		padding: 6rem 0;
		background: linear-gradient(to bottom, #050812, #080c18);
		border-top: 1px solid rgba(255, 255, 255, 0.03);
	}

	.setup-item {
		transition:
			transform 0.3s ease,
			border-color 0.3s ease;
	}

	.setup-item:hover {
		transform: translateY(-5px);
	}

	/* Truth Section */
	.truth-section {
		padding: 8rem 0;
		background: linear-gradient(to bottom, #050812, #0a0f1e);
	}

	.truth-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 6rem;
		align-items: center;
	}

	.left-align {
		text-align: left;
	}

	.truth-text {
		font-size: 1.125rem;
		line-height: 1.8;
		margin-bottom: 1.5rem;
	}

	.truth-list li {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	:global(.warning-icon) {
		color: #f59e0b;
	}
	:global(.check-icon) {
		color: #34d399;
	}

	.chart-card {
		background: rgba(13, 18, 30, 0.6);
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 24px;
		padding: 2rem;
		height: auto;
		position: relative;
	}

	.dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		display: block;
	}
	.red {
		background: #ef4444;
	}
	.yellow {
		background: #f59e0b;
	}
	.green {
		background: #22c55e;
	}

	/* Filter Section */
	.filter-section {
		padding: 6rem 2rem 2rem;
	}

	.filter-container {
		max-width: 1400px;
		margin: 0 auto;
		text-align: center;
	}

	.filter-buttons {
		display: flex;
		gap: 0.75rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.filter-button {
		padding: 0.6rem 1.5rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 50px;
		color: #94a3b8;
		font-weight: 500;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.filter-button:hover {
		background: rgba(255, 255, 255, 0.08);
		color: white;
	}

	.filter-button.active {
		background: #2e8eff;
		border-color: #2e8eff;
		color: white;
		box-shadow: 0 0 20px rgba(46, 142, 255, 0.3);
	}

	/* Indicators Grid */
	.indicators-section {
		padding: 2rem 2rem 8rem;
	}

	.indicators-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 320px), 1fr));
		gap: 1.5rem;
	}

	.indicator-card {
		background: #0b101e;
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 24px;
		overflow: hidden;
		transition:
			transform 0.4s cubic-bezier(0.2, 0.8, 0.2, 1),
			opacity 0.6s ease;
		opacity: 0;
		transform: translateY(30px);
		display: flex;
		flex-direction: column;
		box-shadow: 0 0 0 1px rgba(255, 255, 255, 0.02);
	}

	.indicator-card.visible {
		opacity: 1;
		transform: translateY(0);
		transition-delay: var(--delay);
	}

	.indicator-card:hover {
		transform: translateY(-8px) scale(1.01);
		border-color: rgba(255, 255, 255, 0.1);
		box-shadow: 0 20px 50px -10px rgba(0, 0, 0, 0.5);
	}

	.card-header {
		padding: 2.5rem 2rem;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.card-content {
		padding: 2rem;
		flex-grow: 1;
		display: flex;
		flex-direction: column;
	}

	.card-meta {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.meta-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.25rem 0.75rem;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.meta-badge.difficulty :global(svg) {
		color: #f59e0b;
	}

	.card-features {
		list-style: none;
		padding: 0;
		margin: 0 0 2rem 0;
		flex-grow: 1;
	}

	.card-features li {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 0;
		font-size: 0.9rem;
		color: #cbd5e1;
		border-bottom: 1px solid rgba(255, 255, 255, 0.03);
	}

	.card-features li:last-child {
		border-bottom: none;
	}

	.card-features li :global(svg) {
		color: var(--card-color);
		flex-shrink: 0;
	}

	.card-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		justify-content: center;
		padding: 1rem 1.5rem;
		background: rgba(255, 255, 255, 0.03);
		color: white;
		text-decoration: none;
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 12px;
		font-weight: 600;
		font-size: 0.9375rem;
		transition: all 0.3s ease;
		margin-top: auto;
	}

	.card-button:hover {
		background: var(--card-color);
		border-color: var(--card-color);
		color: #000;
	}

	/* Confluence Section */
	.confluence-section {
		padding: 8rem 2rem;
		background: #050812;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.confluence-grid {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 3rem;
		margin: 5rem 0;
		flex-wrap: wrap;
	}

	.confluence-step {
		background: linear-gradient(145deg, rgba(255, 255, 255, 0.03), rgba(255, 255, 255, 0.01));
		border: 1px solid rgba(255, 255, 255, 0.05);
		padding: 2.5rem 2rem;
		border-radius: 20px;
		flex: 1;
		min-width: 250px;
		max-width: 350px;
		text-align: center;
		position: relative;
		backdrop-filter: blur(10px);
	}

	.step-number {
		background: #2e8eff;
		width: 48px;
		height: 48px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 800;
		font-size: 1.25rem;
		margin: -3.5rem auto 1.5rem;
		border: 4px solid #050812;
		color: white;
	}

	.confluence-cta {
		text-align: center;
		margin-top: 4rem;
	}

	.text-link {
		color: #34d399;
		font-weight: 600;
		text-decoration: none;
		font-size: 1.125rem;
		border-bottom: 1px solid transparent;
		transition: border-color 0.2s;
	}
	.text-link:hover {
		border-bottom-color: #34d399;
	}

	/* FAQ Section */
	.faq-section {
		padding: 8rem 2rem;
		max-width: 900px;
		margin: 0 auto;
	}

	.faq-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.faq-item {
		background: rgba(255, 255, 255, 0.02);
		border-radius: 16px;
		overflow: hidden;
		border: 1px solid rgba(255, 255, 255, 0.05);
	}

	.faq-item.open {
		background: rgba(255, 255, 255, 0.04);
		border-color: rgba(46, 142, 255, 0.3);
		box-shadow: 0 0 30px rgba(0, 0, 0, 0.2);
	}

	.faq-question {
		width: 100%;
		text-align: left;
		padding: 1.5rem 2rem;
		background: none;
		border: none;
		color: white;
		font-weight: 600;
		font-size: 1.125rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
		cursor: pointer;
	}

	.faq-answer {
		max-height: 0;
		overflow: hidden;
		transition: max-height 0.4s cubic-bezier(0, 1, 0, 1);
		padding: 0 2rem;
		color: #94a3b8;
		line-height: 1.7;
	}

	.faq-item.open .faq-answer {
		max-height: 300px;
		padding-bottom: 2rem;
	}

	/* Final CTA */
	.final-cta {
		padding: 10rem 2rem;
		text-align: center;
		background: radial-gradient(circle at center, rgba(10, 15, 30, 0) 0%, #050812 100%);
	}

	.cta-content {
		max-width: 700px;
		margin: 0 auto;
	}

	.final-cta h2 {
		font-size: clamp(2.5rem, 5vw, 3.5rem);
		font-weight: 800;
		margin-bottom: 1.5rem;
		line-height: 1.1;
	}

	.final-cta p {
		color: #cbd5e1;
		font-size: 1.25rem;
		margin-bottom: 2.5rem;
	}

	.cta-button.large {
		padding: 1.25rem 3.5rem;
		font-size: 1.125rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 Mobile-First Responsive Design
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch targets: min 44x44px, Safe areas: env(safe-area-inset-*)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-first base improvements */
	.section-container {
		padding-left: max(1rem, env(safe-area-inset-left));
		padding-right: max(1rem, env(safe-area-inset-right));
	}

	/* Touch-friendly buttons on all devices */
	.cta-button,
	.filter-button,
	.card-button {
		min-height: 44px;
		min-width: 44px;
		-webkit-tap-highlight-color: transparent;
	}

	/* xs: Extra small devices (≥ 360px) */
	@media (min-width: 360px) {
		.hero-section {
			padding: 5rem 1rem 3rem;
		}

		.section-container {
			padding: 0 1rem;
		}
	}

	/* sm: Small devices (≥ 640px) - 2 columns for cards */
	@media (min-width: 640px) {
		.hero-section {
			padding: 6rem 1.5rem 4rem;
		}

		.indicators-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.hero-stats {
			flex-direction: row;
			gap: 2rem;
		}

		.filter-buttons {
			gap: 0.5rem;
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.hero-section {
			padding: 7rem 2rem 5rem;
		}

		.truth-grid {
			grid-template-columns: 1fr;
			gap: 4rem;
		}

		.setup-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.confluence-grid {
			flex-direction: row;
			flex-wrap: wrap;
		}

		.filter-section {
			padding: 5rem 2rem 2rem;
		}
	}

	/* lg: Large devices (≥ 1024px) - 3 columns for cards */
	@media (min-width: 1024px) {
		.hero-section {
			padding: 8rem 2rem 6rem;
		}

		.indicators-grid {
			grid-template-columns: repeat(3, 1fr);
		}

		.truth-grid {
			grid-template-columns: 1fr 1fr;
			gap: 6rem;
		}

		.setup-grid {
			grid-template-columns: repeat(4, 1fr);
		}

		.hero-stats {
			gap: 4rem;
		}

		.confluence-grid {
			flex-wrap: nowrap;
			gap: 3rem;
		}
	}

	/* xl: Extra large devices (≥ 1280px) */
	@media (min-width: 1280px) {
		.glow-orb-1 {
			width: 800px;
			height: 800px;
		}
		.glow-orb-2 {
			width: 600px;
			height: 600px;
		}
	}

	/* Smaller screens for orbs */
	@media (max-width: 1200px) {
		.glow-orb-1 {
			width: 500px;
			height: 500px;
		}
		.glow-orb-2 {
			width: 450px;
			height: 450px;
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.cta-button {
			padding: 1.125rem 2.5rem;
		}

		.filter-button {
			padding: 0.75rem 1.5rem;
		}

		.indicator-card:hover {
			transform: none;
		}

		.card-button {
			padding: 1.125rem 1.5rem;
		}

		/* Disable hover effects on touch */
		.indicator-card:hover {
			box-shadow: none;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.glow-orb,
		.chart-line-svg,
		.animate-float,
		.indicator-card,
		.confluence-connector :global(svg) {
			animation: none;
		}

		.indicator-card,
		.cta-button,
		.filter-button,
		.card-button {
			transition: none;
		}

		.perspective-grid {
			animation: none;
			transform: perspective(500px) rotateX(60deg) scale(2);
		}
	}

	/* High contrast mode */
	@media (prefers-contrast: high) {
		.indicator-card {
			border-width: 2px;
			border-color: #fff;
		}

		.cta-button.primary {
			border: 2px solid #fff;
		}

		.filter-button.active {
			border: 2px solid #fff;
		}
	}

	/* Dark mode forced colors */
	@media (forced-colors: active) {
		.gradient-text {
			background: none;
			-webkit-text-fill-color: currentColor;
			color: LinkText;
		}
	}

	/* Landscape mobile orientation */
	@media (max-height: 500px) and (orientation: landscape) {
		.hero-section {
			padding: 3rem 1rem 2rem;
			min-height: auto;
		}

		.hero-stats {
			margin-top: 1rem;
			padding-top: 1rem;
		}
	}

	/* Print styles */
	@media print {
		.glow-orb,
		.chart-lines,
		.perspective-grid,
		.spotlight-overlay {
			display: none;
		}

		.indicators-page {
			background: #fff;
			color: #000;
		}

		.indicator-card {
			break-inside: avoid;
			page-break-inside: avoid;
		}
	}

	/* ═════════════════════════════════════════════════════════════════════════
	   Additions for cleaned-up markup (Phase 2D, 2026-05-25).
	   New BEM modifiers/elements introduced when stripping Tailwind utilities
	   off the markup. Existing rules above are unchanged for visual parity.
	   ═════════════════════════════════════════════════════════════════════════ */

	/* Section positioning */
	.hero-section {
		position: relative;
	}
	.setup-section,
	.truth-section,
	.confluence-section,
	.final-cta {
		position: relative;
		overflow: hidden;
	}

	.hero-content {
		position: relative;
		z-index: 10;
		padding-inline: 1rem;
		max-width: 1000px;
		text-align: center;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	/* Hero badge label */
	.hero-badge__label {
		letter-spacing: 0.05em;
		font-size: 0.75rem;
		text-transform: uppercase;
		font-weight: 700;
	}

	/* Gradient text — animated horizontal pan (was the inline tailwind gradient) */
	.gradient-text--animated {
		background: linear-gradient(
			to right,
			var(--ind-blue-soft),
			var(--ind-emerald),
			var(--ind-blue-soft)
		);
		background-size: 200% auto;
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		color: transparent;
	}

	/* CTA buttons enhancements */
	.cta-button.primary {
		position: relative;
		overflow: hidden;
	}
	.cta-button__label {
		position: relative;
		z-index: 10;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}
	:global(.cta-button__arrow) {
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.cta-button.primary:hover :global(.cta-button__arrow) {
		transform: translateX(0.25rem);
	}
	.cta-button__fill {
		position: absolute;
		inset: 0;
		background: color-mix(in oklab, #fff 20%, transparent);
		transform: translateY(100%);
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
		pointer-events: none;
	}
	.cta-button.primary:hover .cta-button__fill {
		transform: translateY(0);
	}
	.cta-button.secondary {
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
	}

	.hero-cta-group {
		padding-block-start: 1rem;
	}

	/* Hero stats border + spacing (was inline tailwind) */
	.hero-stats {
		border-block-start: 1px solid var(--rtp-border);
		margin-block-start: 3rem;
		padding-block-start: 2rem;
	}
	.stat-item__icon-wrap {
		padding: 0.75rem;
		border-radius: var(--rtp-radius-pill);
		background: color-mix(in oklab, #fff 5%, transparent);
		transition: background-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.stat-item--blue:hover .stat-item__icon-wrap {
		background: color-mix(in oklab, var(--ind-blue) 10%, transparent);
	}
	.stat-item--emerald:hover .stat-item__icon-wrap {
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
	}
	.stat-item--purple:hover .stat-item__icon-wrap {
		background: color-mix(in oklab, var(--ind-purple-strong) 10%, transparent);
	}
	:global(.stat-icon--blue) {
		color: var(--ind-blue-soft);
	}
	:global(.stat-icon--emerald) {
		color: var(--rtp-emerald-bright);
	}
	:global(.stat-icon--purple) {
		color: var(--ind-purple);
	}
	.stat-content {
		text-align: start;
	}
	.stat-value {
		color: var(--rtp-text);
		font-family: var(--rtp-font-mono);
	}
	.stat-label {
		color: var(--rtp-text-muted);
	}

	/* Setup section */
	.setup-header {
		text-align: center;
		margin-block-end: 3rem;
	}
	.setup-header .section-title {
		margin-block-end: 1rem;
		letter-spacing: -0.025em;
	}
	.setup-header .section-subtitle {
		margin-block-end: 2rem;
	}
	.setup-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}
	@media (min-width: 768px) {
		.setup-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
	@media (min-width: 1024px) {
		.setup-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.setup-item {
		background: color-mix(in oklab, #fff 5%, transparent);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border: 1px solid var(--rtp-border);
		border-radius: 1rem;
		padding: 1.5rem;
		transition: all 300ms;
	}
	.setup-item:hover {
		border-color: color-mix(in oklab, var(--ind-blue) 30%, transparent);
	}
	.setup-icon-wrapper {
		margin-block-end: 1rem;
		inline-size: 3rem;
		block-size: 3rem;
		border-radius: 0.75rem;
		background: color-mix(in oklab, var(--ind-blue) 10%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.setup-item:hover .setup-icon-wrapper {
		transform: scale(1.1);
	}
	:global(.setup-icon) {
		color: var(--ind-blue-soft);
	}
	.setup-item__title {
		color: var(--rtp-text-muted);
		font-size: 0.875rem;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-block-end: 0.5rem;
	}
	.setup-item__value {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.25rem;
	}
	.setup-item__detail {
		font-size: 0.75rem;
		color: var(--rtp-emerald-bright);
		font-family: var(--rtp-font-mono);
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		display: inline-block;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		border: 1px solid color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
	}

	/* Truth section */
	.truth-section__fade {
		position: absolute;
		inset-block-start: 0;
		inset-inline-end: 0;
		inline-size: 33.333%;
		block-size: 100%;
		background: linear-gradient(
			to left,
			color-mix(in oklab, #1e3a8a 10%, transparent),
			transparent
		);
		pointer-events: none;
	}
	.truth-section__inner {
		position: relative;
		z-index: 10;
	}
	.truth-section__title {
		letter-spacing: -0.05em;
	}
	.truth-text {
		color: var(--rtp-text-soft);
	}
	.truth-list {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.truth-list__item {
		display: flex;
		align-items: center;
		gap: 1rem;
		background: color-mix(in oklab, #fff 5%, transparent);
		padding: 1rem;
		border-radius: 0.5rem;
		border: 1px solid var(--rtp-border-soft);
	}
	:global(.warning-icon),
	:global(.check-icon) {
		flex-shrink: 0;
	}

	/* Chart card / header / mockup */
	.chart-card {
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, #1e3a8a 20%, transparent);
	}
	.chart-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		border-block-end: 1px solid var(--rtp-border);
		padding-block-end: 1rem;
		margin-block-end: 1.5rem;
	}
	.chart-header__dots {
		display: flex;
		gap: 0.5rem;
	}
	.chart-header__label {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-text-subtle);
	}
	.chart-mockup {
		block-size: 220px;
		position: relative;
		inline-size: 100%;
	}
	.chart-mockup__svg {
		inline-size: 100%;
		block-size: 100%;
		overflow: visible;
	}

	.mockup-annotation {
		position: absolute;
		inset-block-start: 1rem;
		inset-inline-end: 2rem;
		background: var(--rtp-emerald);
		color: #000;
		font-weight: 700;
		padding: 0.25rem 0.75rem;
		border-radius: 0.25rem;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
		transform-origin: bottom left;
	}
	.mockup-annotation__row {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
	}

	/* Filter section heading */
	.filter-section__title {
		letter-spacing: -0.025em;
		font-size: clamp(1.5rem, 3vw, 2rem);
		font-weight: 800;
		margin-block-end: 1rem;
	}
	.filter-subtitle {
		font-size: 1.125rem;
		max-width: 600px;
		margin: 0 auto 2rem;
		color: var(--rtp-text-muted);
	}

	/* Indicator card additions */
	.indicator-card {
		position: relative;
	}
	.card-header {
		position: relative;
		overflow: hidden;
	}
	.card-icon {
		position: relative;
		z-index: 10;
		transform: translateZ(0);
		transition: transform 500ms;
	}
	.indicator-card:hover .card-icon {
		transform: scale(1.1) rotate(3deg);
	}
	.card-category {
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.3);
	}
	.card-content {
		background: var(--ind-card-bg);
	}
	.card-title {
		color: var(--rtp-text);
		transition: color 300ms;
	}
	.indicator-card:hover .card-title {
		color: var(--card-color);
	}
	.card-use-case {
		background: color-mix(in oklab, #fff 5%, transparent);
		border-inline-start: 2px solid var(--card-color);
	}
	.meta-badge {
		border: 1px solid var(--rtp-border);
	}
	.card-features li {
		transition: padding-inline-start 300ms;
	}
	.indicator-card:hover .card-features li {
		padding-inline-start: 0.25rem;
	}
	:global(.card-button__arrow) {
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.card-button:hover :global(.card-button__arrow) {
		transform: translateX(0.25rem);
	}

	/* Confluence section */
	.confluence-section__fade {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to bottom,
			transparent,
			color-mix(in oklab, #1e3a8a 5%, transparent),
			transparent
		);
		pointer-events: none;
	}
	.confluence-section__inner {
		position: relative;
		z-index: 10;
	}
	.confluence-step__title {
		font-size: 1.25rem;
		font-weight: 700;
		margin-block-end: 0.5rem;
	}
	.confluence-step__desc {
		color: var(--rtp-text-muted);
		font-size: 0.875rem;
	}
	.confluence-step {
		transition: border-color 300ms;
	}
	.confluence-step--blue:hover {
		border-color: color-mix(in oklab, var(--ind-blue) 50%, transparent);
	}
	.confluence-step--emerald:hover {
		border-color: color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
	}
	.confluence-step--purple:hover {
		border-color: color-mix(in oklab, var(--ind-purple-strong) 50%, transparent);
	}
	.step-number--blue {
		background: var(--ind-blue);
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--ind-blue) 30%, transparent);
	}
	.step-number--emerald {
		background: var(--rtp-emerald);
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
	}
	.step-number--purple {
		background: var(--ind-purple-strong);
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--ind-purple-strong) 30%, transparent);
	}

	.confluence-connector--horizontal {
		display: none;
	}
	.confluence-connector--vertical {
		display: block;
		transform: rotate(90deg);
	}
	@media (min-width: 768px) {
		.confluence-connector--horizontal {
			display: block;
		}
		.confluence-connector--vertical {
			display: none;
		}
	}
	:global(.confluence-connector__icon) {
		color: var(--rtp-text-dim);
		animation: indicators-pulse-icon 2s ease-in-out infinite;
	}
	@keyframes indicators-pulse-icon {
		0%,
		100% {
			opacity: 0.5;
		}
		50% {
			opacity: 1;
		}
	}
	@media (prefers-reduced-motion: reduce) {
		:global(.confluence-connector__icon) {
			animation: none;
		}
	}

	.confluence-cta {
		text-align: center;
		margin-block-start: 3rem;
	}
	.confluence-cta__lede {
		color: var(--rtp-text-muted);
		margin-block-end: 0.5rem;
	}
	.text-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}
	:global(.text-link__arrow) {
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.text-link:hover :global(.text-link__arrow) {
		transform: translateX(0.25rem);
	}

	/* FAQ extras */
	.faq-item {
		transition: all 300ms;
	}
	.faq-icon {
		transform: rotate(0deg);
		transition: transform 300ms;
	}
	.faq-icon--open {
		transform: rotate(180deg);
	}
	.faq-question {
		transition: color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.faq-question:hover {
		color: var(--ind-blue-soft);
	}
	.faq-answer__text {
		color: var(--rtp-text-soft);
	}

	/* Final CTA */
	.final-cta__halo {
		position: absolute;
		inset-block-start: 50%;
		inset-inline-start: 50%;
		transform: translate(-50%, -50%);
		inline-size: 800px;
		block-size: 400px;
		background: color-mix(in oklab, #2563eb 10%, transparent);
		filter: blur(120px);
		border-radius: 50%;
		pointer-events: none;
	}
	.cta-content {
		position: relative;
		z-index: 10;
	}
	.final-cta__icon-wrap {
		display: inline-block;
		padding: 1rem;
		border-radius: var(--rtp-radius-pill);
		background: color-mix(in oklab, var(--ind-blue) 10%, transparent);
		margin-block-end: 1.5rem;
	}
	:global(.cta-icon) {
		color: var(--ind-blue-soft);
		display: block;
		margin: 0;
	}
	.final-cta__heading {
		letter-spacing: -0.025em;
	}
	.cta-button.primary.large {
		transition:
			transform var(--rtp-dur-base) var(--rtp-ease-out),
			box-shadow var(--rtp-dur-base) var(--rtp-ease-out);
		box-shadow:
			0 20px 25px -5px rgba(0, 0, 0, 0.25),
			0 0 30px color-mix(in oklab, var(--ind-blue) 20%, transparent);
	}
	.cta-button.primary.large:hover {
		transform: translateY(-2px) scale(1.05);
	}

	/* Chart lines svg helper */
	.chart-lines__svg {
		position: absolute;
		inset: 0;
		inline-size: 100%;
		block-size: 100%;
	}

	.glow-orb {
		mix-blend-mode: screen;
	}
	.perspective-grid {
		position: absolute;
		inset: 0;
		opacity: 0.2;
	}
</style>
