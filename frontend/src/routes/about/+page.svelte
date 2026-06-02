<!--
	URL: /about
-->

<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import { aboutFaqs } from './about-data';
	import { onMount } from 'svelte';
	import { cubicOut } from 'svelte/easing';
	import { fade, draw, slide } from 'svelte/transition';
	import { browser } from '$app/environment';

	import IconBuildingBank from '@tabler/icons-svelte-runes/icons/building-bank';
	import IconShieldLock from '@tabler/icons-svelte-runes/icons/shield-lock';
	import IconUsersGroup from '@tabler/icons-svelte-runes/icons/users-group';
	import IconScale from '@tabler/icons-svelte-runes/icons/scale';
	import IconId from '@tabler/icons-svelte-runes/icons/id';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconBroadcast from '@tabler/icons-svelte-runes/icons/broadcast';
	import IconSchool from '@tabler/icons-svelte-runes/icons/school';
	import IconMessageCircle from '@tabler/icons-svelte-runes/icons/message-circle';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';

	// CLS FIX (measured 0.40 on /about): SSR-render the gated sections (present at
	// first paint, no pop-in). The in:heavySlide reveal (opacity/transform) still
	// plays on client navigation.
	let isVisible = $state(true);
	let openFaqIndex = $state(-1);
	let openAnswerHeight = $state(0);
	const spacerHeight = $derived(Math.max(0, 140 - openAnswerHeight));

	const interactiveCard: Attachment<HTMLElement> = (node) => {
		function onMouseMove(e: MouseEvent) {
			const rect = node.getBoundingClientRect();
			node.style.setProperty('--card-x', `${e.clientX - rect.left}px`);
			node.style.setProperty('--card-y', `${e.clientY - rect.top}px`);
		}
		node.addEventListener('mousemove', onMouseMove);
		return () => node.removeEventListener('mousemove', onMouseMove);
	};

	let gsapContext: ReturnType<typeof import('gsap').gsap.context> | null = null;

	onMount(() => {
		if (!browser) return;
		initGSAP();
		return () => {
			gsapContext?.revert();
		};
	});

	async function initGSAP(): Promise<void> {
		try {
			const { gsap } = await import('gsap');
			const { ScrollTrigger } = await import('gsap/ScrollTrigger');
			gsap.registerPlugin(ScrollTrigger);

			gsapContext = gsap.context(() => {
				gsap.to('.parallax-layer', {
					yPercent: -20,
					ease: 'none',
					scrollTrigger: {
						trigger: '#about-content',
						start: 'top top',
						end: 'bottom top',
						scrub: true
					}
				});
				ScrollTrigger.refresh();
			});
		} catch (error) {
			console.error('[About] GSAP initialization failed:', error);
		}
	}

	function heavySlide(_node: Element, { delay = 0, duration = 1000 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = cubicOut(t);
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 20}px);`;
			}
		};
	}

	const stats = [
		{ value: '1,200+', label: 'Active Members', icon: IconUsersGroup },
		{ value: 'Daily', label: 'Live Guidance', icon: IconBroadcast },
		{ value: '100%', label: 'Transparency', icon: IconShieldLock },
		{ value: '24/7', label: 'Support Chat', icon: IconMessageCircle }
	];

	const features = [
		{
			icon: IconBroadcast,
			title: 'Live Trading Floor',
			subtitle: 'Not just alerts. Real-time education.',
			description: `We don't just post a ticker and disappear. We share our screens, talk through the setup via voice chat, and explain the "Why" before the trade happens. You see the wins, the losses, and the management in real-time.`
		},
		{
			icon: IconSchool,
			title: 'Institutional Education',
			subtitle: 'Master the mechanics of the market.',
			description:
				'Stop relying on lagging indicators. Our curriculum teaches you how to read Order Flow, Dark Pool data, and Gamma Exposure—the actual data institutions use to move price.'
		},
		{
			icon: IconUsersGroup,
			title: 'Mentorship & Community',
			subtitle: 'You are no longer trading alone.',
			description:
				'Trading is the loneliest profession in the world—until now. Join a desk of serious traders who support each other, share alpha, and pick you up when you have a red day.'
		}
	];

	const team = [
		{
			name: 'Billy Ribeiro',
			role: 'Founder & Head Trader',
			id: 'FOUNDER',
			specialties: ['Price Action', 'Market Calls'],
			bio: 'Billy Ribeiro is a globally recognized trader and market strategist, personally mentored by Mark McGoldrick—"Goldfinger"—Goldman Sachs\'s most successful investor in history. McGoldrick called Billy "The Future of Trading." Creator of the revolutionary "Move Prior to The Move" system, Billy famously called the Covid top and bottom, the 2022 market top and bottom, and the breakout to new highs in 2024 and 2025.'
		},
		{
			name: 'Freddie Ferber',
			role: 'Co-Founder & Chart Master',
			id: 'CO-FOUNDER',
			specialties: ['Options Strategies', 'Technical Analysis'],
			bio: "Freddie holds a distinction no one else can claim: he is Billy Ribeiro's only private student—ever. For three years, he trained one-on-one with Billy, mastering the art of price action and market structure. Today, Freddie is a stock and options expert known for turning low-cost options plays into outsized returns. He studies the market relentlessly, every single day."
		},
		{
			name: 'Shao Wen',
			role: 'Mentor & Chart Master',
			id: 'MENTOR',
			specialties: ['East-West Fusion', 'Precision Charting'],
			bio: 'Shao Wen is one of Revolution\'s best-kept secrets. After mastering "The Move Prior to The Move" system, she developed her own unique approach—blending Western price action with Chinese trading strategies. Mentored by Freddie Ferber, Shao Wen\'s knowledge, discipline, and passion have made her one of the most respected female traders in the world.'
		}
	];

	const faqs = aboutFaqs;

	const tickerItems = [
		'ES_F +0.25%',
		'NQ_F -0.10%',
		'RTY_F +1.20%',
		'GC_F +0.05%',
		'CL_F -0.50%',
		'DX_F +0.12%',
		'VIX -4.5%'
	];
</script>

<div role="main" class="about">
	<div class="about__atmosphere" aria-hidden="true">
		<div class="about__noise"></div>
		<div class="about__gradient"></div>
	</div>

	<div class="ticker" aria-label="Market ticker">
		<div class="ticker__inner">
			<div class="ticker__track">
				{#each [...tickerItems, ...tickerItems] as item, i (`ticker-${i}`)}
					<span class="ticker__cell">
						<span
							class={['ticker__dot', item.includes('+') ? 'ticker__dot--up' : 'ticker__dot--down']}
							aria-hidden="true"
						></span>
						{item}
					</span>
				{/each}
			</div>
		</div>
	</div>

	<section id="about-content" class="about__content">
		<!-- HERO -->
		<section class="hero">
			<div class="hero__floor parallax-layer" aria-hidden="true"></div>

			{#if isVisible}
				<div in:heavySlide={{ delay: 0, duration: 1200 }} class="hero__shell">
					<div class="hero__bracket hero__bracket--tl" aria-hidden="true"></div>
					<div class="hero__bracket hero__bracket--br" aria-hidden="true"></div>

					<div class="hero__copy">
						<div class="hero__pill">
							<span class="hero__pill-ping">
								<span class="hero__pill-ping-ring" aria-hidden="true"></span>
								<span class="hero__pill-ping-dot" aria-hidden="true"></span>
							</span>
							Live Trading Floor Open
						</div>

						<h1 class="hero__title">
							Trade Real <span class="hero__title-gradient hero__title-gradient--slate"
								>Capital.</span
							><br />
							<span class="hero__title-gradient hero__title-gradient--amber">Real Mentors.</span><br
							/>
							<span class="hero__title-mute">Real Time.</span>
						</h1>

						<p class="hero__lede">
							Stop guessing. Join a professional trading floor where we share screens, explain the
							'why' behind every move, and fight for profitability together. No hindsight. No
							hiding.
						</p>

						<div class="hero__actions">
							<a href="/join" class="cta cta--primary">
								<span class="cta__shimmer" aria-hidden="true"></span>
								<span class="cta__label">Join The Community</span>
								<IconArrowRight size={16} class="cta__arrow" />
							</a>
							<a href="#how-it-works" class="cta cta--ghost">
								<span class="cta__label">See How It Works</span>
							</a>
						</div>
					</div>

					<div class="hero__stats">
						{#each stats as stat (stat.label)}
							{@const Icon = stat.icon}
							<div {@attach interactiveCard} class="stat interactive-card">
								<div class="stat__sheen" aria-hidden="true"></div>
								<div class="stat__icon">
									<Icon size={24} stroke={1.5} />
								</div>
								<div class="stat__value">{stat.value}</div>
								<div class="stat__label">{stat.label}</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</section>

		<!-- TRADING ALONE / CHART CARD -->
		<section class="alone">
			{#if isVisible}
				<div in:heavySlide={{ delay: 200 }} class="alone__chart-wrap">
					<div class="chart-card">
						<div class="chart-card__grid" aria-hidden="true"></div>

						<svg
							class="chart-card__svg"
							viewBox="0 0 400 300"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								d="M0 280 L50 250 L100 260 L150 180 L200 200 L250 100 L300 120 L350 50 L400 20"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								vector-effect="non-scaling-stroke"
								in:draw={{ duration: 3000, easing: cubicOut }}
							/>
							<path
								d="M0 280 L50 250 L100 260 L150 180 L200 200 L250 100 L300 120 L350 50 L400 20 V 300 H 0 Z"
								fill="url(#grad)"
								opacity="0.1"
								in:fade={{ delay: 1000, duration: 2000 }}
							/>
							<defs>
								<linearGradient id="grad" x1="0" y1="0" x2="0" y2="1">
									<stop offset="0%" stop-color="#d97706" />
									<stop offset="100%" stop-color="transparent" />
								</linearGradient>
							</defs>
						</svg>

						<div class="chart-card__scanner" aria-hidden="true"></div>

						<div class="chart-card__quote">
							<div class="chart-card__quote-head">
								<IconActivity size={14} class="chart-card__pulse" />
								<div class="chart-card__quote-label">The Retail Trap</div>
							</div>
							<p>
								"Most traders fail not because they lack intelligence, but because they lack
								support, structure, and emotional control."
							</p>
						</div>
					</div>
				</div>

				<div in:heavySlide={{ delay: 300 }} class="alone__copy">
					<h2 class="alone__title">
						You Shouldn't Have to <br /><span class="alone__title-em">Trade Alone.</span>
					</h2>
					<div class="alone__prose">
						<p>
							The hardest part of trading isn't reading a chart. It's the psychological battle of
							sitting alone in a room, making high-stakes decisions with no one to guide you.
						</p>
						<p class="alone__pullquote">
							<strong>Revolution Trading Pros changed the model.</strong>
							We replaced the "Guru Alert" system with a true "Virtual Trading Floor."
						</p>
						<p>
							When you join us, you aren't just buying data. You are plugging into a collective
							intelligence. You get mentors who check your risk, a community that keeps you
							accountable, and a system that prioritizes long-term survival over gambling.
						</p>
						<ul class="alone__checks">
							{#each ['No pump and dumps', 'No fake P&L screenshots', 'Real-time voice commentary'] as item (item)}
								<li class="check">
									<div class="check__icon"><IconCheck size={14} stroke={3} /></div>
									<span class="check__text">{item}</span>
								</li>
							{/each}
						</ul>
					</div>
				</div>
			{/if}
		</section>

		<!-- ECOSYSTEM -->
		<section id="how-it-works" class="ecosystem">
			{#if isVisible}
				<div in:heavySlide={{ delay: 400 }} class="ecosystem__inner">
					<div class="ecosystem__head">
						<div>
							<span class="ecosystem__eyebrow">
								<span class="ecosystem__eyebrow-rule" aria-hidden="true"></span> The Ecosystem
							</span>
							<h2 class="ecosystem__title">Everything You Need<br />To Succeed.</h2>
						</div>
						<p class="ecosystem__caption">
							A complete operating system for your trading business, <br />from data to psychology.
						</p>
					</div>

					<div class="features">
						{#each features as feat, i (feat.title)}
							{@const Icon = feat.icon}
							<div {@attach interactiveCard} class="feature interactive-card">
								<div class="feature__ghost-icon" aria-hidden="true">
									<Icon size={200} />
								</div>

								<div class="feature__head">
									<div class="feature__icon">
										<Icon size={28} stroke={1.5} />
									</div>
									<span class="feature__index">0{i + 1}</span>
								</div>

								<h3 class="feature__title">{feat.title}</h3>
								<div class="feature__rule" aria-hidden="true"></div>
								<p class="feature__subtitle">{feat.subtitle}</p>
								<p class="feature__desc">{feat.description}</p>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</section>

		<!-- TEAM -->
		<section class="team">
			{#if isVisible}
				<div in:heavySlide={{ delay: 500 }} class="team__inner">
					<div class="team__head">
						<div>
							<h2 class="team__title">Meet Your Mentors.</h2>
							<p class="team__lede">
								We aren't anonymous admins hiding behind screens. We're real traders in the chat
								with you every single day—calling moves, answering questions, and helping you grow.
							</p>
						</div>
						<div class="team__pulses" aria-hidden="true">
							{#each team as _, i (i)}<div class="team__pulse"></div>{/each}
						</div>
					</div>

					<div class="team__list">
						{#each team as member (member.name)}
							<div {@attach interactiveCard} class="member interactive-card">
								<div class="member__rail" aria-hidden="true"></div>

								<div class="member__avatar-wrap">
									<div class="member__avatar">
										<IconId size={36} stroke={1} />
									</div>
									<div class="member__inline">
										<div class="member__name">{member.name}</div>
										<div class="member__role-line">{member.role}</div>
									</div>
								</div>

								<div class="member__id">
									<div class="member__name">{member.name}</div>
									<div class="member__role-line">
										{member.role} <span class="member__pipe">|</span>
										<span class="member__tag">{member.id}</span>
									</div>
								</div>

								<div class="member__bio-wrap">
									<p class="member__bio">{member.bio}</p>
								</div>

								<div class="member__specs">
									{#each member.specialties as spec (spec)}
										<span class="spec-chip">{spec}</span>
									{/each}
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</section>

		<!-- TESTIMONIALS -->
		<section class="testimonials">
			{#if isVisible}
				<div in:heavySlide={{ delay: 600 }} class="testimonials__head">
					<div class="testimonials__halo" aria-hidden="true"></div>
					<h2 class="testimonials__title">Why Traders Stay.</h2>
					<p class="testimonials__sub">Community Feedback</p>
				</div>

				<div in:heavySlide={{ delay: 700 }} class="testimonials__grid">
					<div {@attach interactiveCard} class="testimonial interactive-card">
						<div class="testimonial__quote-icon" aria-hidden="true">
							<IconMessageCircle size={80} />
						</div>
						<div class="testimonial__stars" aria-hidden="true">
							{#each Array(5) as _, i (i)}<IconScale size={12} class="testimonial__star" />{/each}
						</div>
						<p class="testimonial__quote">
							"I spent years jumping from one alert service to another, losing money. Revolution is
							different. They actually taught me how to fish. I finally feel in control of my risk."
						</p>
						<div class="testimonial__sig">
							<div class="testimonial__avatar">MK</div>
							<div>
								<div class="testimonial__sig-name">Verified Member</div>
								<div class="testimonial__sig-meta">Member since 2021</div>
							</div>
						</div>
					</div>

					<div {@attach interactiveCard} class="testimonial interactive-card">
						<div class="testimonial__quote-icon" aria-hidden="true">
							<IconMessageCircle size={80} />
						</div>
						<div class="testimonial__stars" aria-hidden="true">
							{#each Array(5) as _, i (i)}<IconScale size={12} class="testimonial__star" />{/each}
						</div>
						<p class="testimonial__quote">
							"The morning voice chat is a game changer. Hearing Billy explain his thought process
							in real-time kept me out of so many bad trades. It's like having a risk manager over
							your shoulder."
						</p>
						<div class="testimonial__sig">
							<div class="testimonial__avatar">JT</div>
							<div>
								<div class="testimonial__sig-name">Verified Member</div>
								<div class="testimonial__sig-meta">Member since 2023</div>
							</div>
						</div>
					</div>
				</div>
			{/if}
		</section>

		<!-- FAQ -->
		<section class="faq">
			{#if isVisible}
				<div in:heavySlide={{ delay: 800 }} class="faq__inner">
					<div class="faq__head">
						<h2 class="faq__title">Frequently Asked Questions</h2>
						<div class="faq__rule" aria-hidden="true"></div>
					</div>

					<div class="faq__list">
						{#each faqs as faq, i (faq.q)}
							<div class={['faq__item', openFaqIndex === i && 'faq__item--open']}>
								<button
									type="button"
									onclick={() => (openFaqIndex = openFaqIndex === i ? -1 : i)}
									aria-expanded={openFaqIndex === i}
									class="faq__trigger"
								>
									<span class="faq__q">{faq.q}</span>
									<span class="faq__chev">
										<IconChevronDown
											size={20}
											class="faq__chev-icon {openFaqIndex === i ? 'faq__chev-icon--open' : ''}"
										/>
									</span>
								</button>
								{#if openFaqIndex === i}
									<div
										transition:slide={{ duration: 300, easing: cubicOut }}
										bind:clientHeight={openAnswerHeight}
										onoutroend={() => (openAnswerHeight = 0)}
										class="faq__answer"
									>
										{faq.a}
									</div>
								{/if}
							</div>
						{/each}

						<!-- Spacer absorbs FAQ expansion to keep total section
							 height constant — answer grows N px → spacer shrinks N px. -->
						<div aria-hidden="true" class="faq__spacer" style:height="{spacerHeight}px"></div>
					</div>
				</div>
			{/if}
		</section>

		<!-- FINAL CTA -->
		<section class="final-cta">
			<div class="final-cta__halo" aria-hidden="true"></div>
			<div class="final-cta__rule" aria-hidden="true"></div>
			<div class="final-cta__grid" aria-hidden="true"></div>

			<div class="final-cta__inner">
				<IconBuildingBank size={64} class="final-cta__icon" stroke={0.8} />
				<h2 class="final-cta__title">
					Ready to <span class="final-cta__title-em">Turn Pro?</span>
				</h2>
				<p class="final-cta__lede">
					The market is open. The team is ready. <br />The only thing missing is you.
				</p>
				<div class="final-cta__actions">
					<a href="/join" class="cta cta--primary cta--large">
						<span class="cta__shimmer" aria-hidden="true"></span>
						<span class="cta__label">Start Your Membership</span>
						<IconArrowRight size={18} class="cta__arrow" />
					</a>
				</div>

				<div class="final-cta__trust">
					<span class="final-cta__trust-item">
						<IconShieldLock size={12} /> Secure Payment
					</span>
					<span class="final-cta__trust-sep" aria-hidden="true"></span>
					<span class="final-cta__trust-item final-cta__trust-item--muted">Cancel Anytime</span>
					<span class="final-cta__trust-sep" aria-hidden="true"></span>
					<span class="final-cta__trust-item final-cta__trust-item--muted">24/7 Support</span>
				</div>
			</div>
		</section>
	</section>
</div>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Page-local tokens (one-offs for /about only).
	   The amber palette here is more saturated than --rtp-amber because
	   the page is positioning itself as a metallic / institutional pitch.
	   ───────────────────────────────────────────────────────────────── */
	.about {
		--a-bg: #010203;
		--a-bg-card: #050505;
		--a-bg-card-elev: #080808;
		--a-bg-fixed-strip: #020202;
		--a-bg-grad-top: #020408;
		--a-bg-grad-mid: #050810;
		--a-bg-grad-bottom: #010203;

		--a-amber-400: #fbbf24;
		--a-amber-500: var(--rtp-amber);
		--a-amber-600: var(--rtp-amber-deep);
		--a-amber-700: #b45309;
		--a-amber-800: #92400e;
		--a-amber-900: #78350f;

		--a-amber-glow-30: color-mix(in oklab, var(--a-amber-600) 30%, transparent);
		--a-amber-glow-20: color-mix(in oklab, var(--a-amber-600) 20%, transparent);
		--a-amber-glow-15: color-mix(in oklab, var(--a-amber-600) 15%, transparent);
		--a-amber-glow-10: color-mix(in oklab, var(--a-amber-600) 10%, transparent);
		--a-amber-glow-5: color-mix(in oklab, var(--a-amber-600) 5%, transparent);

		--a-emerald-glow-15: color-mix(in oklab, var(--rtp-emerald) 15%, transparent);

		--a-red-glow: rgba(244, 63, 94, 0.5);
		--a-emerald-shadow: rgba(16, 185, 129, 0.5);
		--a-amber-shadow: rgba(217, 119, 6, 0.4);
		--a-amber-shadow-strong: rgba(180, 83, 9, 0.4);
		--a-amber-shadow-soft: rgba(180, 83, 9, 0.2);
		--a-amber-text-shadow: rgba(217, 119, 6, 0.3);
		--a-amber-shadow-hero: rgba(245, 158, 11, 0.3);

		position: relative;
		background: var(--a-bg);
		color: var(--rtp-text-muted);
		font-family: var(--rtp-font-sans);
		overflow-x: clip;
	}
	.about ::selection {
		background: color-mix(in oklab, var(--a-amber-700) 50%, transparent);
		color: #fff;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Atmosphere (fixed background noise + gradient)
	   ───────────────────────────────────────────────────────────────── */
	.about__atmosphere {
		position: fixed;
		inset: 0;
		pointer-events: none;
		z-index: 0;
	}
	.about__noise {
		position: absolute;
		inset: 0;
		opacity: 0.04;
		mix-blend-mode: overlay;
		z-index: 2;
		background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E");
	}
	.about__gradient {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to bottom,
			var(--a-bg-grad-top),
			var(--a-bg-grad-mid),
			var(--a-bg-grad-bottom)
		);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Top market ticker strip
	   ───────────────────────────────────────────────────────────────── */
	.ticker {
		position: fixed;
		inset-block-start: 0;
		inset-inline: 0;
		z-index: 50;
		border-block-end: 1px solid var(--rtp-border-soft);
		background: color-mix(in oklab, var(--a-bg-fixed-strip) 90%, transparent);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		height: 2.5rem;
		display: flex;
		align-items: center;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.4);
	}
	.ticker__inner {
		width: 100%;
		overflow: hidden;
		white-space: nowrap;
		mask-image: linear-gradient(to right, transparent, #000 5%, #000 95%, transparent);
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		color: var(--rtp-text-subtle);
	}
	.ticker__track {
		display: inline-block;
		animation: a-ticker 40s linear infinite;
	}
	.ticker__track:hover {
		animation-play-state: paused;
	}
	@keyframes a-ticker {
		0% {
			transform: translate3d(0, 0, 0);
		}
		100% {
			transform: translate3d(-50%, 0, 0);
		}
	}
	.ticker__cell {
		display: inline-block;
		padding-inline: 2rem;
		cursor: default;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.ticker__cell:hover {
		color: #fff;
	}
	.ticker__dot {
		display: inline-block;
		width: 0.375rem;
		height: 0.375rem;
		border-radius: 50%;
		margin-inline-end: 0.5rem;
	}
	.ticker__dot--up {
		background: var(--rtp-emerald);
		box-shadow: 0 0 8px var(--a-emerald-shadow);
	}
	.ticker__dot--down {
		background: #f43f5e;
		box-shadow: 0 0 8px var(--a-red-glow);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Page content positioning (above atmosphere, below ticker)
	   ───────────────────────────────────────────────────────────────── */
	.about__content {
		position: relative;
		z-index: 10;
		padding-block: 8rem 6rem;
		padding-inline: 1.5rem;
	}
	@media (min-width: 1024px) {
		.about__content {
			padding-inline: 2rem;
		}
	}

	/* ─────────────────────────────────────────────────────────────────
	   Hero
	   ───────────────────────────────────────────────────────────────── */
	.hero {
		max-width: var(--rtp-content-wide);
		margin-inline: auto;
		margin-block-end: 8rem;
		position: relative;
	}
	@media (min-width: 1024px) {
		.hero {
			margin-block-end: 12rem;
		}
	}

	.hero__floor {
		position: absolute;
		inset-block-start: 5rem;
		inset-inline: -20%;
		height: 500px;
		pointer-events: none;
		z-index: 0;
		background-image:
			linear-gradient(var(--a-amber-glow-10) 1px, transparent 1px),
			linear-gradient(90deg, var(--a-amber-glow-10) 1px, transparent 1px);
		background-size: 60px 60px;
		transform: perspective(500px) rotateX(60deg);
		transform-origin: top center;
		opacity: 0.2;
		mask-image: linear-gradient(to bottom, #000, transparent);
	}

	.hero__shell {
		display: flex;
		flex-direction: column;
		gap: 4rem;
		border-block-end: 1px solid var(--rtp-border);
		padding-block-end: 4rem;
		position: relative;
		z-index: 10;
	}
	@media (min-width: 1024px) {
		.hero__shell {
			flex-direction: row;
			align-items: flex-end;
		}
		.hero__copy {
			width: 66.666%;
		}
		.hero__stats {
			width: 33.333%;
		}
	}

	.hero__bracket {
		position: absolute;
		width: 3rem;
		height: 3rem;
	}
	.hero__bracket--tl {
		inset-block-start: -1rem;
		inset-inline-start: -1rem;
		border-block-start: 2px solid var(--a-amber-glow-30);
		border-inline-start: 2px solid var(--a-amber-glow-30);
	}
	.hero__bracket--br {
		inset-block-end: -1rem;
		inset-inline-end: -1rem;
		border-block-end: 2px solid var(--a-amber-glow-30);
		border-inline-end: 2px solid var(--a-amber-glow-30);
	}

	.hero__copy {
		position: relative;
		z-index: 10;
	}

	.hero__pill {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.375rem 1rem;
		border: 1px solid color-mix(in oklab, #064e3b 30%, transparent);
		background: color-mix(in oklab, #022c22 20%, transparent);
		color: var(--rtp-emerald-bright);
		font-size: 0.625rem;
		font-weight: 700;
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 2rem;
		border-radius: var(--rtp-radius-pill);
		box-shadow: 0 0 20px var(--a-emerald-glow-15);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}
	.hero__pill-ping {
		position: relative;
		display: flex;
		height: 0.5rem;
		width: 0.5rem;
	}
	.hero__pill-ping-ring {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: var(--rtp-emerald-bright);
		opacity: 0.75;
		animation: a-ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
	}
	.hero__pill-ping-dot {
		position: relative;
		display: inline-flex;
		border-radius: 50%;
		height: 0.5rem;
		width: 0.5rem;
		background: var(--rtp-emerald);
	}
	@keyframes a-ping {
		75%,
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}

	.hero__title {
		font-family: var(--rtp-font-serif);
		color: #fff;
		font-size: 3.75rem;
		letter-spacing: -0.025em;
		line-height: 0.9;
		margin-block-end: 2.5rem;
		mix-blend-mode: screen;
		filter: drop-shadow(0 25px 25px rgba(0, 0, 0, 0.15));
	}
	@media (min-width: 640px) {
		.hero__title {
			font-size: 4.5rem;
		}
	}
	@media (min-width: 1024px) {
		.hero__title {
			font-size: 8rem;
		}
	}
	.hero__title-gradient {
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}
	.hero__title-gradient--slate {
		background-image: linear-gradient(to right, #f1f5f9, var(--rtp-text-faint));
	}
	.hero__title-gradient--amber {
		background-image: linear-gradient(to right, var(--a-amber-400), var(--a-amber-700));
		filter: drop-shadow(0 0 30px var(--a-amber-text-shadow));
	}
	.hero__title-mute {
		color: var(--rtp-text-faint);
	}

	.hero__lede {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
		font-weight: 300;
		line-height: 1.625;
		max-width: 42rem;
		border-inline-start: 2px solid color-mix(in oklab, var(--a-amber-600) 50%, transparent);
		padding-inline-start: 1.5rem;
		margin-block-end: 3rem;
		background: linear-gradient(to right, color-mix(in oklab, #fff 5%, transparent), transparent);
		padding-block: 1rem;
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		border-start-end-radius: 0.5rem;
		border-end-end-radius: 0.5rem;
	}

	.hero__actions {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	@media (min-width: 640px) {
		.hero__actions {
			flex-direction: row;
		}
	}

	/* ─────────────────────────────────────────────────────────────────
	   CTA buttons (shared by hero + final CTA)
	   ───────────────────────────────────────────────────────────────── */
	.cta {
		position: relative;
		display: inline-flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		padding: 1.25rem 2.5rem;
		font-weight: 700;
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.2em;
		text-decoration: none;
		overflow: hidden;
		transition: all 300ms var(--rtp-ease-out);
		cursor: pointer;
	}
	.cta--primary {
		background: linear-gradient(to bottom right, var(--a-amber-700), var(--a-amber-900));
		color: #fff;
		border: 1px solid color-mix(in oklab, var(--a-amber-600) 50%, transparent);
	}
	.cta--primary:hover {
		border-color: var(--a-amber-400);
		box-shadow: 0 0 40px var(--a-amber-shadow-hero);
	}
	.cta--ghost {
		border: 1px solid var(--rtp-border);
		color: var(--rtp-text-soft);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}
	.cta--ghost:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}
	.cta--ghost:hover .cta__label {
		color: #fff;
	}
	.cta--large {
		padding: 1.5rem 3rem;
		font-size: 0.875rem;
		box-shadow: 0 0 40px var(--a-amber-shadow-soft);
		border-radius: 0.125rem;
	}
	.cta--large:hover {
		box-shadow: 0 0 60px var(--a-amber-shadow-strong);
	}
	.cta__shimmer {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to right,
			transparent,
			color-mix(in oklab, #fff 20%, transparent),
			transparent
		);
		translate: -100% 0;
		transition: translate 500ms cubic-bezier(0.4, 0, 0.2, 1);
	}
	.cta:hover .cta__shimmer {
		translate: 100% 0;
	}
	.cta__label {
		position: relative;
		z-index: 10;
	}
	.cta :global(.cta__arrow) {
		position: relative;
		z-index: 10;
		transition: translate var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.cta:hover :global(.cta__arrow) {
		translate: 0.25rem 0;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Hero stats grid (2x2)
	   ───────────────────────────────────────────────────────────────── */
	.hero__stats {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1px;
		background: var(--rtp-border);
		border: 1px solid var(--rtp-border);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.4);
		border-radius: 0.125rem;
		overflow: hidden;
	}

	.stat {
		background: color-mix(in oklab, var(--a-bg-card) 90%, transparent);
		padding: 1.5rem;
		position: relative;
		overflow: hidden;
		transition: background var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.stat:hover {
		background: #0a0a0a;
	}
	.stat__sheen {
		position: absolute;
		inset-block-start: 0;
		inset-inline-start: 0;
		width: 100%;
		height: 2px;
		background: linear-gradient(to right, transparent, var(--a-amber-500), transparent);
		translate: -100% 0;
		transition: translate 1s cubic-bezier(0.4, 0, 0.2, 1);
	}
	.stat:hover .stat__sheen {
		translate: 100% 0;
	}
	.stat__icon {
		color: var(--a-amber-600);
		margin-block-end: 0.75rem;
		opacity: 0.6;
		transition:
			opacity 300ms var(--rtp-ease-out),
			scale 300ms var(--rtp-ease-out);
		transform-origin: left;
	}
	.stat:hover .stat__icon {
		opacity: 1;
		scale: 1.1;
	}
	.stat__value {
		font-size: 1.875rem;
		font-family: var(--rtp-font-serif);
		color: #fff;
		margin-block-end: 0.25rem;
		font-variant-numeric: tabular-nums;
		letter-spacing: -0.025em;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.stat:hover .stat__value {
		color: var(--a-amber-500);
	}
	.stat__label {
		font-size: 0.5625rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		color: var(--rtp-text-subtle);
		letter-spacing: 0.2em;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Alone section + chart card
	   ───────────────────────────────────────────────────────────────── */
	.alone {
		max-width: 87.5rem; /* 1400px */
		margin-inline: auto;
		margin-block-end: 8rem;
		display: grid;
		gap: 6rem;
		align-items: center;
	}
	@media (min-width: 1024px) {
		.alone {
			margin-block-end: 12rem;
			grid-template-columns: 1fr 1fr;
		}
		.alone__chart-wrap {
			order: 1;
		}
		.alone__copy {
			order: 2;
		}
	}
	.alone__chart-wrap {
		order: 2;
		position: relative;
		perspective: 1000px;
	}
	@media (max-width: 1023.98px) {
		.alone__chart-wrap {
			order: 2;
		}
		.alone__copy {
			order: 1;
		}
	}

	.chart-card {
		position: relative;
		width: 100%;
		aspect-ratio: 4 / 3;
		background: var(--a-bg-card);
		border: 1px solid var(--rtp-border);
		border-radius: 0.5rem;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.4);
		transition: rotate 1s var(--rtp-ease-out);
		transform-style: preserve-3d;
	}
	.alone__chart-wrap:hover .chart-card {
		rotate: x 2deg y 3deg;
	}
	.chart-card__grid {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(0deg, color-mix(in oklab, #fff 3%, transparent) 1px, transparent 1px),
			linear-gradient(90deg, color-mix(in oklab, #fff 3%, transparent) 1px, transparent 1px);
		background-size: 3rem 3rem;
	}
	.chart-card__svg {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		color: var(--a-amber-500);
		filter: drop-shadow(0 0 10px var(--a-emerald-shadow));
	}
	.chart-card__scanner {
		position: absolute;
		inset-block: 0;
		width: 2px;
		background: linear-gradient(to bottom, transparent, rgba(255, 255, 255, 0.8), transparent);
		box-shadow: 0 0 20px rgba(255, 255, 255, 0.8);
		z-index: 20;
		animation: a-scan 4s ease-in-out infinite;
		transform-origin: bottom;
	}
	@keyframes a-scan {
		0% {
			translate: 0 0;
			opacity: 0;
		}
		10% {
			opacity: 1;
		}
		90% {
			opacity: 1;
		}
		100% {
			translate: 350px 0;
			opacity: 0;
		}
	}
	.chart-card__quote {
		position: absolute;
		inset-block-start: 2.5rem;
		inset-inline-start: 2.5rem;
		background: color-mix(in oklab, var(--a-bg-card-elev) 90%, transparent);
		border-inline-start: 4px solid var(--rtp-red);
		padding: 1.5rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.4);
		max-width: 20rem;
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
		transition: translate 500ms var(--rtp-ease-out);
	}
	.alone__chart-wrap:hover .chart-card__quote {
		translate: 0 0 5px;
	}
	.chart-card__quote-head {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-block-end: 0.75rem;
	}
	.chart-card :global(.chart-card__pulse) {
		color: var(--rtp-red);
		animation: a-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	@keyframes a-pulse {
		50% {
			opacity: 0.5;
		}
	}
	.chart-card__quote-label {
		font-size: 0.5625rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-red);
		text-transform: uppercase;
		letter-spacing: 0.2em;
	}
	.chart-card__quote p {
		font-size: 0.875rem;
		color: var(--rtp-text-soft);
		font-weight: 300;
		font-style: italic;
		line-height: 1.625;
	}

	.alone__title {
		font-size: 2.25rem;
		font-family: var(--rtp-font-serif);
		color: #fff;
		margin-block-end: 2rem;
		letter-spacing: -0.025em;
	}
	@media (min-width: 1024px) {
		.alone__title {
			font-size: 3rem;
		}
	}
	.alone__title-em {
		color: var(--a-amber-600);
		font-style: italic;
	}
	.alone__prose {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		font-size: 1.125rem;
		font-weight: 300;
		line-height: 1.625;
		color: var(--rtp-text-muted);
	}
	.alone__pullquote {
		padding-inline-start: 1rem;
		border-inline-start: 2px solid var(--rtp-border-strong);
	}
	.alone__pullquote strong {
		color: #fff;
		font-weight: 500;
	}
	.alone__checks {
		list-style: none;
		margin: 2rem 0 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.check {
		display: flex;
		align-items: center;
		gap: 1rem;
		font-size: 0.875rem;
		color: var(--a-amber-500);
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		cursor: default;
	}
	.check__icon {
		padding: 0.375rem;
		border-radius: 0.25rem;
		background: color-mix(in oklab, var(--a-amber-500) 10%, transparent);
		border: 1px solid color-mix(in oklab, var(--a-amber-500) 20%, transparent);
		transition:
			background var(--rtp-dur-base) var(--rtp-ease-out),
			color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.check:hover .check__icon {
		background: var(--a-amber-500);
		color: #000;
	}
	.check__text {
		transition: translate var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.check:hover .check__text {
		translate: 0.5rem 0;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Ecosystem / Features
	   ───────────────────────────────────────────────────────────────── */
	.ecosystem {
		max-width: var(--rtp-content-wide);
		margin-inline: auto;
		margin-block-end: 8rem;
	}
	@media (min-width: 1024px) {
		.ecosystem {
			margin-block-end: 12rem;
		}
	}
	.ecosystem__inner {
		border-block-start: 1px solid var(--rtp-border);
		padding-block-start: 5rem;
	}
	.ecosystem__head {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: flex-end;
		margin-block-end: 5rem;
		gap: 1.5rem;
	}
	@media (min-width: 768px) {
		.ecosystem__head {
			flex-direction: row;
		}
	}
	.ecosystem__eyebrow {
		color: var(--a-amber-600);
		font-family: var(--rtp-font-mono);
		font-size: 0.625rem;
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 1rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.ecosystem__eyebrow-rule {
		width: 2rem;
		height: 1px;
		background: var(--a-amber-600);
	}
	.ecosystem__title {
		font-size: 2.25rem;
		font-family: var(--rtp-font-serif);
		color: #fff;
		letter-spacing: -0.025em;
	}
	@media (min-width: 768px) {
		.ecosystem__title {
			font-size: 3.75rem;
		}
	}
	.ecosystem__caption {
		display: none;
		color: var(--rtp-text-subtle);
		font-size: 0.875rem;
		max-width: 28rem;
		text-align: end;
		line-height: 1.625;
		border-inline-end: 2px solid color-mix(in oklab, var(--a-amber-600) 50%, transparent);
		padding-inline-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.ecosystem__caption {
			display: block;
		}
	}

	.features {
		display: grid;
		gap: 1.5rem;
	}
	@media (min-width: 768px) {
		.features {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	.feature {
		background: var(--a-bg-card);
		border: 1px solid var(--rtp-border);
		padding: 2.5rem;
		transition: border-color 500ms var(--rtp-ease-out);
		position: relative;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		border-radius: 0.75rem;
	}
	.feature:hover {
		border-color: color-mix(in oklab, var(--a-amber-600) 40%, transparent);
	}
	.feature__ghost-icon {
		position: absolute;
		inset-block-start: -1.5rem;
		inset-inline-end: -1.5rem;
		color: #fff;
		opacity: 0.02;
		transition:
			opacity 700ms var(--rtp-ease-out),
			scale 700ms var(--rtp-ease-out),
			rotate 700ms var(--rtp-ease-out);
		pointer-events: none;
	}
	.feature:hover .feature__ghost-icon {
		opacity: 0.05;
		scale: 1.1;
		rotate: -12deg;
	}
	.feature__head {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-block-end: 2rem;
		position: relative;
		z-index: 10;
	}
	.feature__icon {
		padding: 1rem;
		background: color-mix(in oklab, #fff 5%, transparent);
		border: 1px solid var(--rtp-border-soft);
		color: var(--a-amber-600);
		border-radius: 0.5rem;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
		transition:
			background var(--rtp-dur-base) var(--rtp-ease-out),
			color var(--rtp-dur-base) var(--rtp-ease-out),
			box-shadow var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.feature:hover .feature__icon {
		background: var(--a-amber-600);
		color: #000;
		box-shadow: 0 10px 15px -3px var(--a-amber-glow-20);
	}
	.feature__index {
		font-family: var(--rtp-font-mono);
		font-size: 0.625rem;
		color: var(--rtp-text-faint);
	}
	.feature__title {
		font-size: 1.5rem;
		font-family: var(--rtp-font-serif);
		color: #fff;
		margin-block-end: 0.75rem;
		position: relative;
		z-index: 10;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.feature:hover .feature__title {
		color: var(--a-amber-500);
	}
	.feature__rule {
		width: 3rem;
		height: 1px;
		background: color-mix(in oklab, var(--a-amber-600) 50%, transparent);
		margin-block-end: 1rem;
		transition: width 700ms var(--rtp-ease-out);
	}
	.feature:hover .feature__rule {
		width: 100%;
	}
	.feature__subtitle {
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		color: color-mix(in oklab, var(--a-amber-600) 80%, transparent);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		margin-block-end: 1.5rem;
		min-block-size: 1.5em;
		position: relative;
		z-index: 10;
	}
	.feature__desc {
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
		line-height: 1.625;
		font-weight: 300;
		position: relative;
		z-index: 10;
		margin-block-start: auto;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Team
	   ───────────────────────────────────────────────────────────────── */
	.team {
		max-width: var(--rtp-content-wide);
		margin-inline: auto;
		margin-block-end: 8rem;
	}
	@media (min-width: 1024px) {
		.team {
			margin-block-end: 12rem;
		}
	}
	.team__inner {
		border-block-start: 1px solid var(--rtp-border);
		padding-block-start: 5rem;
	}
	.team__head {
		margin-block-end: 4rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		gap: 2rem;
	}
	@media (min-width: 768px) {
		.team__head {
			flex-direction: row;
			align-items: flex-end;
		}
	}
	.team__title {
		font-size: 2.25rem;
		font-family: var(--rtp-font-serif);
		color: #fff;
		margin-block-end: 1rem;
	}
	.team__lede {
		color: var(--rtp-text-muted);
		max-width: 42rem;
		font-weight: 300;
		font-size: 1.125rem;
	}
	.team__pulses {
		display: none;
		gap: 0.5rem;
	}
	@media (min-width: 768px) {
		.team__pulses {
			display: flex;
		}
	}
	.team__pulse {
		width: 0.375rem;
		height: 0.375rem;
		border-radius: 50%;
		background: color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
		animation: a-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	.team__list {
		display: grid;
		gap: 1px;
		background: var(--rtp-border-soft);
		border: 1px solid var(--rtp-border);
		border-radius: 0.5rem;
		overflow: hidden;
	}

	.member {
		background: var(--a-bg-card);
		padding: 2rem;
		display: grid;
		gap: 2rem;
		align-items: center;
		position: relative;
		overflow: hidden;
		transition: background var(--rtp-dur-base) var(--rtp-ease-out);
	}
	@media (min-width: 768px) {
		.member {
			padding: 3rem;
			grid-template-columns: repeat(12, minmax(0, 1fr));
		}
	}
	.member:hover {
		background: var(--a-bg-card-elev);
	}
	.member__rail {
		position: absolute;
		inset-block: 0;
		inset-inline-start: 0;
		width: 0.375rem;
		background: var(--a-amber-600);
		translate: -100% 0;
		transition: translate var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.member:hover .member__rail {
		translate: 0 0;
	}
	.member__avatar-wrap {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		position: relative;
		z-index: 10;
	}
	@media (min-width: 768px) {
		.member__avatar-wrap {
			grid-column: span 2 / span 2;
		}
	}
	.member__avatar {
		width: 5rem;
		height: 5rem;
		border-radius: 50%;
		background: linear-gradient(
			to bottom right,
			color-mix(in oklab, #fff 10%, transparent),
			transparent
		);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--rtp-text-muted);
		border: 1px solid var(--rtp-border-soft);
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.4);
		transition:
			border-color var(--rtp-dur-base) var(--rtp-ease-out),
			color var(--rtp-dur-base) var(--rtp-ease-out),
			scale var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.member:hover .member__avatar {
		border-color: color-mix(in oklab, var(--a-amber-600) 50%, transparent);
		color: var(--a-amber-500);
		scale: 1.05;
	}
	.member__inline {
		display: block;
	}
	@media (min-width: 768px) {
		.member__inline {
			display: none;
		}
	}
	.member__id {
		display: none;
		position: relative;
		z-index: 10;
	}
	@media (min-width: 768px) {
		.member__id {
			display: block;
			grid-column: span 3 / span 3;
		}
	}
	.member__name {
		font-size: 1.25rem;
		font-family: var(--rtp-font-serif);
		color: #fff;
		letter-spacing: -0.025em;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	@media (min-width: 768px) {
		.member__id .member__name {
			font-size: 1.5rem;
		}
	}
	.member:hover .member__id .member__name {
		color: var(--a-amber-500);
	}
	.member__role-line {
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-text-subtle);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		margin-block-start: 0.25rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.member__pipe {
		color: color-mix(in oklab, #fff 10%, transparent);
	}
	.member__tag {
		color: color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
		font-weight: 700;
	}
	.member__bio-wrap {
		position: relative;
		z-index: 10;
	}
	@media (min-width: 768px) {
		.member__bio-wrap {
			grid-column: span 5 / span 5;
		}
	}
	.member__bio {
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
		line-height: 1.625;
		font-weight: 300;
		border-inline-start: 1px solid var(--rtp-border);
		padding-inline-start: 1rem;
		transition: border-color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.member:hover .member__bio {
		border-color: color-mix(in oklab, var(--a-amber-600) 30%, transparent);
	}
	.member__specs {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 0.75rem;
		position: relative;
		z-index: 10;
	}
	@media (min-width: 768px) {
		.member__specs {
			grid-column: span 2 / span 2;
		}
	}
	.spec-chip {
		padding: 0.375rem 0.75rem;
		background: color-mix(in oklab, #fff 5%, transparent);
		border: 1px solid var(--rtp-border-soft);
		font-size: 0.5625rem;
		color: var(--rtp-text-soft);
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-radius: 0.125rem;
		transition:
			border-color var(--rtp-dur-fast) var(--rtp-ease-out),
			color var(--rtp-dur-fast) var(--rtp-ease-out),
			background var(--rtp-dur-fast) var(--rtp-ease-out);
		cursor: default;
		white-space: nowrap;
	}
	.spec-chip:hover {
		border-color: color-mix(in oklab, var(--a-amber-600) 50%, transparent);
		color: var(--a-amber-500);
		background: color-mix(in oklab, var(--a-amber-900) 10%, transparent);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Testimonials
	   ───────────────────────────────────────────────────────────────── */
	.testimonials {
		max-width: 75rem; /* 1200px */
		margin-inline: auto;
		margin-block-end: 8rem;
		padding-inline: 1rem;
	}
	@media (min-width: 1024px) {
		.testimonials {
			margin-block-end: 12rem;
		}
	}
	.testimonials__head {
		text-align: center;
		margin-block-end: 5rem;
		position: relative;
	}
	.testimonials__halo {
		position: absolute;
		inset-block-start: 50%;
		inset-inline-start: 50%;
		translate: -50% -50%;
		width: 24rem;
		height: 24rem;
		background: var(--a-amber-glow-10);
		filter: blur(120px);
		border-radius: 50%;
		pointer-events: none;
		mix-blend-mode: screen;
	}
	.testimonials__title {
		font-size: 1.875rem;
		font-family: var(--rtp-font-serif);
		color: #fff;
		margin-block-end: 1rem;
		position: relative;
		z-index: 10;
	}
	.testimonials__sub {
		color: var(--rtp-text-subtle);
		font-size: 0.625rem;
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
		letter-spacing: 0.3em;
		position: relative;
		z-index: 10;
	}

	.testimonials__grid {
		display: grid;
		gap: 2rem;
	}
	@media (min-width: 768px) {
		.testimonials__grid {
			grid-template-columns: 1fr 1fr;
		}
	}
	.testimonial {
		background: color-mix(in oklab, var(--a-bg-card-elev) 80%, transparent);
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
		padding: 2.5rem;
		border: 1px solid var(--rtp-border-soft);
		border-radius: 0.75rem;
		position: relative;
		overflow: hidden;
		transition: border-color 500ms var(--rtp-ease-out);
	}
	.testimonial:hover {
		border-color: color-mix(in oklab, var(--a-amber-600) 30%, transparent);
	}
	.testimonial__quote-icon {
		position: absolute;
		inset-block-start: 1rem;
		inset-inline-start: 1rem;
		color: color-mix(in oklab, var(--a-amber-900) 20%, transparent);
	}
	.testimonial__stars {
		display: flex;
		gap: 0.25rem;
		margin-block-end: 1.5rem;
		color: var(--a-amber-600);
		position: relative;
		z-index: 10;
	}
	.testimonial :global(.testimonial__star) {
		fill: currentColor;
	}
	.testimonial__quote {
		font-size: 1.125rem;
		color: var(--rtp-text-soft);
		font-weight: 300;
		font-style: italic;
		margin-block-end: 2rem;
		line-height: 1.625;
		position: relative;
		z-index: 10;
	}
	.testimonial__sig {
		display: flex;
		align-items: center;
		gap: 1rem;
		border-block-start: 1px solid var(--rtp-border-soft);
		padding-block-start: 1.5rem;
		position: relative;
		z-index: 10;
	}
	.testimonial__avatar {
		width: 2.5rem;
		height: 2.5rem;
		background: linear-gradient(to top right, #1e293b, #334155);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.75rem;
		font-weight: 700;
		color: #fff;
		border: 1px solid var(--rtp-border);
	}
	.testimonial__sig-name {
		font-size: 0.875rem;
		color: #fff;
		font-weight: 500;
	}
	.testimonial__sig-meta {
		font-size: 0.625rem;
		color: var(--rtp-text-subtle);
		font-family: var(--rtp-font-mono);
		text-transform: uppercase;
	}

	/* ─────────────────────────────────────────────────────────────────
	   FAQ
	   ───────────────────────────────────────────────────────────────── */
	.faq {
		max-width: 56.25rem; /* 900px */
		margin-inline: auto;
		margin-block-end: 8rem;
	}
	.faq__inner {
		border-block-start: 1px solid var(--rtp-border);
		padding-block-start: 5rem;
	}
	.faq__head {
		text-align: center;
		margin-block-end: 4rem;
	}
	.faq__title {
		font-size: 2.25rem;
		font-family: var(--rtp-font-serif);
		color: #fff;
		margin-block-end: 0.5rem;
	}
	.faq__rule {
		height: 0.25rem;
		width: 5rem;
		background: var(--a-amber-600);
		margin-inline: auto;
		border-radius: var(--rtp-radius-pill);
		margin-block-start: 1.5rem;
		box-shadow: 0 0 15px rgba(217, 119, 6, 0.5);
	}
	.faq__list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.faq__item {
		background: var(--a-bg-card);
		border: 1px solid var(--rtp-border);
		border-radius: 0.5rem;
		overflow: hidden;
		transition: border-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.faq__item--open {
		border-color: color-mix(in oklab, var(--a-amber-600) 30%, transparent);
	}
	.faq__trigger {
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		padding: 1.5rem;
		cursor: pointer;
		background: color-mix(in oklab, #fff 2%, transparent);
		border: 0;
		text-align: start;
		color: inherit;
		font: inherit;
		transition: background var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.faq__trigger:hover {
		background: color-mix(in oklab, #fff 4%, transparent);
	}
	.faq__trigger:focus-visible {
		outline: 2px solid var(--a-amber-500);
		outline-offset: -2px;
	}
	.faq__q {
		font-size: 1.125rem;
		color: #e2e8f0;
		font-weight: 300;
		padding-inline-end: 2rem;
	}
	.faq__chev {
		color: var(--a-amber-600);
	}
	.faq :global(.faq__chev-icon) {
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.faq :global(.faq__chev-icon--open) {
		transform: rotate(180deg);
	}
	.faq__answer {
		padding: 1.5rem 1.5rem 2rem;
		color: var(--rtp-text-muted);
		font-weight: 300;
		line-height: 1.625;
		border-block-start: 1px solid var(--rtp-border-soft);
		background: var(--rtp-bg-darker);
	}
	.faq__spacer {
		overflow: hidden;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Final CTA
	   ───────────────────────────────────────────────────────────────── */
	.final-cta {
		margin-block-start: 8rem;
		border-block-start: 1px solid var(--rtp-border);
		padding-block: 8rem;
		text-align: center;
		position: relative;
		overflow: hidden;
	}
	.final-cta__halo {
		position: absolute;
		inset: 0;
		background: var(--a-amber-glow-5);
		filter: blur(120px);
		pointer-events: none;
	}
	.final-cta__rule {
		position: absolute;
		inset-block-start: 0;
		inset-inline: 0;
		height: 1px;
		background: linear-gradient(
			to right,
			transparent,
			color-mix(in oklab, var(--a-amber-600) 50%, transparent),
			transparent
		);
	}
	.final-cta__grid {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(color-mix(in oklab, #fff 2%, transparent) 1px, transparent 1px),
			linear-gradient(90deg, color-mix(in oklab, #fff 2%, transparent) 1px, transparent 1px);
		background-size: 40px 40px;
		opacity: 0.2;
	}
	.final-cta__inner {
		position: relative;
		z-index: 10;
		padding-inline: 1rem;
	}
	.final-cta :global(.final-cta__icon) {
		display: block;
		margin-inline: auto;
		margin-block-end: 2rem;
		color: var(--a-amber-600);
		opacity: 0.8;
		filter: drop-shadow(0 0 20px var(--a-amber-shadow));
	}
	.final-cta__title {
		font-size: 3rem;
		font-family: var(--rtp-font-serif);
		color: #fff;
		margin-block-end: 2rem;
		letter-spacing: -0.025em;
	}
	@media (min-width: 768px) {
		.final-cta__title {
			font-size: 6rem;
		}
	}
	.final-cta__title-em {
		font-style: italic;
		background: linear-gradient(to right, var(--a-amber-400), var(--a-amber-700));
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}
	.final-cta__lede {
		color: var(--rtp-text-muted);
		font-size: 1.25rem;
		margin-block-end: 3rem;
		max-width: 36rem;
		margin-inline: auto;
		font-weight: 300;
		line-height: 1.625;
	}
	.final-cta__actions {
		display: flex;
		flex-direction: column;
		justify-content: center;
		gap: 1.5rem;
	}
	@media (min-width: 640px) {
		.final-cta__actions {
			flex-direction: row;
		}
	}
	.final-cta__trust {
		margin-block-start: 3rem;
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 1.5rem;
		font-size: 0.625rem;
		color: var(--rtp-text-faint);
		text-transform: uppercase;
		letter-spacing: 0.2em;
		font-family: var(--rtp-font-mono);
	}
	.final-cta__trust-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--rtp-text-subtle);
	}
	.final-cta__trust-item--muted {
		color: var(--rtp-text-subtle);
	}
	.final-cta__trust-sep {
		width: 1px;
		height: 0.75rem;
		background: #1e293b;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Interactive card spotlight (shared)
	   ───────────────────────────────────────────────────────────────── */
	.interactive-card {
		position: relative;
	}
	.interactive-card::before {
		content: '';
		position: absolute;
		inset: 0;
		background: radial-gradient(
			800px circle at var(--card-x, 50%) var(--card-y, 50%),
			rgba(255, 255, 255, 0.08),
			transparent 40%
		);
		z-index: 2;
		pointer-events: none;
		opacity: 0;
		transition: opacity 500ms var(--rtp-ease-out);
		mix-blend-mode: overlay;
	}
	.interactive-card:hover::before {
		opacity: 1;
	}
</style>
