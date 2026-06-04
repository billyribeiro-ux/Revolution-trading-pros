<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { browser } from '$app/environment';
	import IconMenu2 from '@tabler/icons-svelte-runes/icons/menu-2';
	import IconLock from '@tabler/icons-svelte-runes/icons/lock';

	// --- Pricing State (Svelte 5 Runes) ---
	let selectedPlan: 'monthly' | 'quarterly' | 'annual' = $state('quarterly');

	// SSOT: Single source of truth for all pricing data
	interface Plan {
		id: 'monthly' | 'quarterly' | 'annual';
		label: string;
		price: number;
		period: string;
		perDay: string;
		savingsCopy: string;
		checkoutHref: string;
		featured: boolean;
		variant: 'simple' | 'featured' | 'highlight';
		features: string[];
	}

	const plans: Plan[] = [
		{
			id: 'monthly',
			label: 'Monthly',
			price: 197,
			period: '/mo',
			perDay: '$6.56/day',
			savingsCopy: '',
			checkoutHref: '/checkout/monthly-room',
			featured: false,
			variant: 'simple',
			features: [
				'Daily Live Trading',
				'Discord Community',
				'Watchlists & Alerts',
				'Onboarding Course'
			]
		},
		{
			id: 'quarterly',
			label: 'Quarterly',
			price: 497,
			period: '/qtr',
			perDay: '$5.52/day',
			savingsCopy: 'Most Popular — Save $94',
			checkoutHref: '/checkout/quarterly-room',
			featured: true,
			variant: 'featured',
			features: [
				'Everything in Monthly',
				'Options Masterclass Access',
				'Small Account Strategy',
				'Priority Support'
			]
		},
		{
			id: 'annual',
			label: 'Annual',
			price: 1647,
			period: '/yr',
			perDay: '$4.51/day',
			savingsCopy: 'Best Value — Save $717',
			checkoutHref: '/checkout/annual-room',
			featured: false,
			variant: 'highlight',
			features: [
				'Everything in Quarterly',
				'1-on-1 Strategy Session',
				'Annual Members-Only Events',
				'Direct DM Access'
			]
		}
	];

	// --- FAQ Logic (Svelte 5 Runes) ---
	let openFaq: number | null = $state(null);
	const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

	// --- GSAP ScrollTrigger Animations (PE7 Svelte 5 Pattern) ---
	let gsapContext: ReturnType<typeof import('gsap').gsap.context> | null = null;
	let isComponentMounted = false;
	let scrollRefreshRaf = 0;

	onMount(() => {
		if (!browser) return;
		isComponentMounted = true;
		initGSAP();
		return () => {
			isComponentMounted = false;
			cancelAnimationFrame(scrollRefreshRaf);
			if (gsapContext) gsapContext.revert();
		};
	});

	// PE7: Separate async function for GSAP initialization
	async function initGSAP(): Promise<void> {
		try {
			const { gsap } = await import('gsap');
			const { ScrollTrigger } = await import('gsap/ScrollTrigger');

			if (!isComponentMounted) return;

			gsap.registerPlugin(ScrollTrigger);

			const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

			if (prefersReducedMotion) {
				gsap.set('[data-gsap]', { opacity: 1, y: 0 });
				return;
			}

			// Use gsap.context() for scoped cleanup - prevents global ScrollTrigger destruction
			gsapContext = gsap.context(() => {
				const all = Array.from(document.querySelectorAll('[data-gsap]'));
				gsap.set(all, { opacity: 0, y: 30 });

				// Elements visible in viewport on mount get an immediate
				// staggered entrance — ScrollTrigger.batch's onEnter does not
				// fire for elements already past the trigger position at
				// registration time, which would otherwise leave hero
				// elements stuck invisible.
				const visible: Element[] = [];
				const hidden: Element[] = [];
				all.forEach((el) => {
					const r = el.getBoundingClientRect();
					if (r.top < window.innerHeight && r.bottom > 0) visible.push(el);
					else hidden.push(el);
				});

				if (visible.length > 0) {
					gsap.to(visible, {
						opacity: 1,
						y: 0,
						duration: 0.9,
						ease: 'power3.out',
						stagger: 0.1
					});
				}

				if (hidden.length > 0) {
					ScrollTrigger.batch(hidden, {
						onEnter: (batch) => {
							gsap.to(batch, {
								opacity: 1,
								y: 0,
								duration: 0.8,
								ease: 'power3.out',
								stagger: 0.1,
								overwrite: true
							});
						},
						start: 'top 85%',
						once: true
					});
				}
			});

			// PE7: Refresh ScrollTrigger after layout settles (fonts, images loaded)
			const refreshTrigger = document.fonts?.ready ?? Promise.resolve();
			refreshTrigger.then(() => {
				if (!isComponentMounted) return;
				scrollRefreshRaf = requestAnimationFrame(() => {
					scrollRefreshRaf = 0;
					ScrollTrigger.refresh();
				});
			});
		} catch (error) {
			console.error('[DayTrading] GSAP initialization failed:', error);
		}
	}

	// SEO schemas now emitted via +page.ts (page.data.seo).
	// FAQ list kept locally for the on-page accordion UI.
	const faqList = [
		{
			question: 'Do I need to be an expert to join?',
			answer:
				"No. While the trading is fast-paced, we prioritize education. We provide a comprehensive 'New Member' onboarding video series to help you understand our levels, terminology, and platform execution before you take your first trade."
		},
		{
			question: 'What platform is the room hosted on?',
			answer:
				'We host our live room on a private, boosted Discord server. This allows for low-latency Voice Channels (sub-200ms) and crisp 1080p Screen Share, accessible from desktop or mobile.'
		},
		{
			question: 'What specific instruments do you trade?',
			answer:
				'We specialize in SPX (S&P 500 Index) options, specifically 0DTE (Zero Days to Expiration). We occasionally trade SPY, QQQ, and futures (ES/NQ) when setups present high probability, but SPX is our primary focus due to tax benefits and liquidity.'
		},
		{
			question: 'How much capital do I need to start?',
			answer:
				'We recommend a starting account size of at least $2,000 to trade comfortably with proper risk management. However, many members start with less using cash accounts to avoid PDT (Pattern Day Trader) rules.'
		},
		{
			question: 'Do you offer a free trial?',
			answer:
				"We do not offer free trials to maintain the integrity and quality of our community. We focus on dedicated traders who are ready to commit to their education. We do offer a 30-day money-back guarantee if you feel the value wasn't delivered."
		},
		{
			question: 'Is this just copy-trading or signals?',
			answer:
				'This is an educational room first. While we call out every trade we take live (Entry, Stop, Target), the goal is for you to learn the "Why" behind the trade so you can eventually become self-sufficient.'
		},
		{
			question: 'What happens if I miss the live session?',
			answer:
				'All key levels, trade recaps, and educational lessons are posted in text channels. We also record major educational webinars. However, the live voice commentary is exclusive to the live session hours.'
		},
		{
			question: 'Can I trade on my phone?',
			answer:
				"Yes. Discord has an excellent mobile app. You can listen to the voice commentary and see the screen share on your phone while executing trades on your broker's mobile app."
		},
		{
			question: 'What is the win rate of the room?',
			answer:
				'Win rates vary by market conditions. Historically, our strategy targets a 70%+ win rate with a strict 1:2 or 1:3 risk-to-reward ratio. We prioritize risk management over "calling tops and bottoms."'
		},
		{
			question: 'How do I cancel my subscription?',
			answer:
				'You have full control. You can cancel anytime via your user dashboard or the Stripe portal link provided in your welcome email. Access continues until the end of your billing cycle.'
		}
	];
</script>

<div class="day-trading">
	<section class="hero">
		<div class="hero__bg" aria-hidden="true">
			<div class="hero__bg-grid"></div>
			<div class="hero__halo hero__halo--blue"></div>
			<div class="hero__halo hero__halo--indigo"></div>
		</div>

		<div class="hero__inner">
			<div class="hero__copy">
				<div data-gsap class="hero__live-pill">
					<span class="hero__live-pulse">
						<span class="hero__live-ping"></span>
						<span class="hero__live-dot"></span>
					</span>
					<span class="hero__live-label">Live Trading Active &bull; Pre-Market Prep 08:30 EST</span>
				</div>

				<h1 data-gsap class="hero__title">
					Never Trade <br />
					<span class="hero__title-accent">Alone Again.</span>
				</h1>

				<p data-gsap class="hero__lede">
					Join the digital trading floor. Watch our screens, hear our professional voice commentary,
					and execute
					<strong>SPX 0DTE trades</strong> alongside 500+ serious traders every single morning.
				</p>

				<div data-gsap class="hero__cta-row">
					<a href="#pricing" class="hero__cta hero__cta--primary group">
						Join the Room
						<svg class="hero__cta-arrow" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 7l5 5m0 0l-5 5m5-5H6"
							/></svg
						>
					</a>
					<a href="#schedule" class="hero__cta hero__cta--ghost"> View Schedule </a>
				</div>

				<div data-gsap class="hero__social">
					<div class="hero__avatars">
						<div class="hero__avatar hero__avatar--blue">JD</div>
						<div class="hero__avatar hero__avatar--emerald">MK</div>
						<div class="hero__avatar hero__avatar--purple">SR</div>
						<div class="hero__avatar hero__avatar--more">+500</div>
					</div>
					<div class="hero__social-meta">
						<span class="hero__social-count">500+ Traders</span>
						<span>Active in Discord now</span>
					</div>
				</div>
			</div>

			<div class="hero__chrome">
				<div class="hero__chrome-glow" aria-hidden="true"></div>
				<div class="hero__chrome-card">
					<div class="hero__chrome-tab">
						<div class="hero__chrome-menu">
							<IconMenu2 size={20} stroke={2} aria-hidden="true" />
							<div class="hero__chrome-channel"># &#x1f534;-live-trading-floor</div>
						</div>
						<div class="hero__chrome-live">
							<span class="hero__chrome-live-dot"></span> LIVE VOICE
						</div>
					</div>

					<div class="hero__chrome-body">
						<div class="hero__chart-pane">
							<div class="hero__chart-label">SCREEN SHARE: HEAD TRADER</div>
							<svg class="hero__chart-svg" viewBox="0 0 400 200" fill="none" stroke="currentColor">
								<path
									stroke-width="2"
									d="M0 150 C 50 150, 50 100, 100 100 C 150 100, 150 180, 200 180 C 250 180, 250 40, 300 40 C 350 40, 350 90, 400 90"
								/>
								<path
									stroke-width="1"
									stroke-dasharray="4 4"
									class="hero__chart-grid"
									d="M0 100 H400"
								/>
								<circle cx="400" cy="90" r="4" class="hero__chart-blip" />
							</svg>
							<div class="hero__chart-quote">SPX 4580.50 (+1.2%)</div>
						</div>

						<div class="hero__chat-pane">
							<div class="hero__chat-heading">Chat Stream</div>

							<div class="hero__chat-row hero__chat-row--dim">
								<div class="hero__chat-avatar hero__chat-avatar--blue"></div>
								<div>
									<div class="hero__chat-skel hero__chat-skel--name"></div>
									<div class="hero__chat-skel hero__chat-skel--msg"></div>
								</div>
							</div>
							<div class="hero__chat-row">
								<div class="hero__chat-avatar hero__chat-avatar--emerald">M</div>
								<div class="hero__chat-text">
									<span class="hero__chat-author hero__chat-author--mod">Mod:</span>
									<span class="hero__chat-msg"
										>Approaching VWAP support at 4500. Watch for the bounce.</span
									>
								</div>
							</div>
							<div class="hero__chat-row">
								<div class="hero__chat-avatar hero__chat-avatar--indigo"></div>
								<div class="hero__chat-text">
									<span class="hero__chat-author">Trader88:</span>
									<span class="hero__chat-msg">Calls loaded. Volume looking massive!</span>
								</div>
							</div>
							<div class="hero__chat-row">
								<div class="hero__chat-avatar hero__chat-avatar--orange"></div>
								<div class="hero__chat-text">
									<span class="hero__chat-author">SarahK:</span>
									<span class="hero__chat-msg">Took 20% on that leg. Thanks!</span>
								</div>
							</div>
							<div class="hero__chat-input">Message #live-trading...</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="stats">
		<div class="stats__inner">
			<div class="stats__grid">
				<div class="stats__cell group">
					<div class="stats__value stats__value--primary">500+</div>
					<div class="stats__label">Active Members</div>
				</div>
				<div class="stats__cell group">
					<div class="stats__value stats__value--emerald">5+ Years</div>
					<div class="stats__label">Live History</div>
				</div>
				<div class="stats__cell group">
					<div class="stats__value stats__value--indigo">1,000+</div>
					<div class="stats__label">Trades Verified</div>
				</div>
				<div class="stats__cell group">
					<div class="stats__value stats__value--blue">Daily</div>
					<div class="stats__label">Live Voice Sessions</div>
				</div>
			</div>
		</div>
	</section>

	<section class="features">
		<div class="features__pattern" aria-hidden="true"></div>
		<div class="features__inner">
			<div class="features__header">
				<span class="features__eyebrow">The Experience</span>
				<h2 data-gsap class="features__title">
					A Professional Trading Floor <br /> On Your Screen
				</h2>
				<p data-gsap class="features__lede">
					Trading is lonely. It doesn't have to be. Surround yourself with winners and professional
					guidance every single day.
				</p>
			</div>

			<div class="features__grid">
				<div data-gsap class="feature-card feature-card--primary group">
					<div class="feature-card__icon">
						<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
							/></svg
						>
					</div>
					<h3 class="feature-card__title">Low-Latency Voice</h3>
					<p class="feature-card__desc">
						Hear the moderator's thought process in real-time. "I'm looking for a bounce here at
						4500." No typing delays. Just pure execution speed when seconds count.
					</p>
				</div>

				<div data-gsap class="feature-card feature-card--emerald group">
					<div class="feature-card__icon">
						<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
							/></svg
						>
					</div>
					<h3 class="feature-card__title">1080p Screen Share</h3>
					<p class="feature-card__desc">
						Watch our charts live. See exactly what we are seeing—Key Gamma Levels, VWAP, Order
						Flow, and Fibonacci levels—as the market prints them.
					</p>
				</div>

				<div data-gsap class="feature-card feature-card--indigo group">
					<div class="feature-card__icon">
						<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
							/></svg
						>
					</div>
					<h3 class="feature-card__title">Pro Community</h3>
					<p class="feature-card__desc">
						No toxicity. No pump and dump. Just a group of serious, funded traders sharing
						high-quality ideas, news, and psychological support throughout the session.
					</p>
				</div>
			</div>
		</div>
	</section>

	<section class="why">
		<div class="why__inner">
			<div class="why__grid">
				<div>
					<h2 data-gsap class="why__title">Why We Trade SPX 0DTE</h2>
					<p data-gsap class="why__lede">
						The S&P 500 Index (SPX) is the premier instrument for professional day traders. Unlike
						individual stocks like TSLA or NVDA, the SPX offers unique advantages that align with
						our institutional trading approach.
					</p>
					<ul class="why__list">
						<li class="why__item" data-gsap>
							<div class="why__check">&#x2713;</div>
							<div>
								<h4 class="why__item-title">Favorable Tax Treatment</h4>
								<p class="why__item-desc">
									SPX options fall under Section 1256, meaning 60% of gains are taxed at the lower
									long-term capital gains rate, regardless of holding period.
								</p>
							</div>
						</li>
						<li class="why__item" data-gsap>
							<div class="why__check">&#x2713;</div>
							<div>
								<h4 class="why__item-title">Cash Settled &amp; No Early Assignment</h4>
								<p class="why__item-desc">
									You can never be assigned shares. The index settles to cash, eliminating the risk
									of overnight gap risk in physical shares.
								</p>
							</div>
						</li>
						<li class="why__item" data-gsap>
							<div class="why__check">&#x2713;</div>
							<div>
								<h4 class="why__item-title">Institutional Liquidity</h4>
								<p class="why__item-desc">
									Massive volume ensures tight bid/ask spreads, allowing for rapid entries and exits
									even with large position sizes.
								</p>
							</div>
						</li>
					</ul>
				</div>
				<div class="why__media">
					<div class="why__media-overlay" aria-hidden="true"></div>
					<div class="why__media-inner">
						<div class="why__media-grid" aria-hidden="true"></div>
						<div class="why__media-glyph">
							<span class="why__media-text">SPX</span>
						</div>
						<svg class="why__media-curves" viewBox="0 0 100 100" preserveAspectRatio="none">
							<path
								d="M0,50 Q25,30 50,50 T100,50"
								fill="none"
								stroke="currentColor"
								stroke-width="0.5"
							/>
							<path
								d="M0,50 Q25,20 50,50 T100,50"
								fill="none"
								stroke="currentColor"
								stroke-width="0.5"
							/>
							<path
								d="M0,50 Q25,80 50,50 T100,50"
								fill="none"
								stroke="currentColor"
								stroke-width="0.5"
							/>
						</svg>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="schedule" class="schedule">
		<div class="schedule__inner">
			<div class="schedule__header">
				<span class="schedule__eyebrow">Routine</span>
				<h2 data-gsap class="schedule__title">The Daily Trading Routine</h2>
				<p class="schedule__lede">
					Consistency is the hallmark of a professional. This is exactly how we approach every
					trading day.
				</p>
			</div>

			<div class="timeline">
				<div data-gsap class="timeline__row group">
					<div class="timeline__pin timeline__pin--primary">08:30</div>
					<div class="timeline__card">
						<div class="timeline__card-head">
							<h3 class="timeline__card-title">Pre-Market Prep</h3>
							<span class="timeline__chip timeline__chip--primary">AM EST</span>
						</div>
						<p class="timeline__card-desc">
							We review overnight action, analyze key economic data drops (CPI/PPI/FOMC), and map
							out Critical Gamma Levels on the SPX. This prepares us mentally for the scenarios
							ahead.
						</p>
					</div>
				</div>

				<div data-gsap class="timeline__row timeline__row--featured group">
					<div class="timeline__pin timeline__pin--emerald">09:30</div>
					<div class="timeline__card timeline__card--featured">
						<div class="timeline__card-head">
							<h3 class="timeline__card-title">Market Open (The Bell)</h3>
							<span class="timeline__chip timeline__chip--emerald">AM EST</span>
						</div>
						<p class="timeline__card-desc">
							High focus. Voice commentary is active. We execute our primary strategies: Opening
							Range Breakout or Rejection. <span class="timeline__emph"
								>This is the most profitable window of the day.</span
							>
						</p>
					</div>
				</div>

				<div data-gsap class="timeline__row group">
					<div class="timeline__pin timeline__pin--muted">12:00</div>
					<div class="timeline__card">
						<div class="timeline__card-head">
							<h3 class="timeline__card-title">Lunch / Mid-Day</h3>
							<span class="timeline__chip timeline__chip--muted">PM EST</span>
						</div>
						<p class="timeline__card-desc">
							Institutional volume slows. We review morning trades, calculate P&L, answer member
							questions in voice, and scan for afternoon setups. Preservation of morning capital is
							the goal.
						</p>
					</div>
				</div>

				<div data-gsap class="timeline__row group">
					<div class="timeline__pin timeline__pin--indigo">15:00</div>
					<div class="timeline__card">
						<div class="timeline__card-head">
							<h3 class="timeline__card-title">Power Hour</h3>
							<span class="timeline__chip timeline__chip--indigo">PM EST</span>
						</div>
						<p class="timeline__card-desc">
							Final hour volatility injection. We look for end-of-day squeezes or flushes (MOC
							imbalances). All 0DTE positions are closed flat before the closing bell.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="pricing" class="pricing">
		<div class="pricing__inner">
			<div class="pricing__header">
				<span class="pricing__eyebrow">Investment</span>
				<h2 class="pricing__title">Membership Access</h2>
				<p class="pricing__lede">
					Invest in your education. One disciplined trade can pay for the whole year. Cancel
					anytime.
				</p>
			</div>

			<div class="pricing__toggle-row">
				<div class="pricing-toggle" role="group">
					<button
						onclick={() => (selectedPlan = 'monthly')}
						class={[
							'pricing-toggle__btn',
							selectedPlan === 'monthly' && 'pricing-toggle__btn--active'
						]}>Monthly</button
					>
					<button
						onclick={() => (selectedPlan = 'quarterly')}
						class={[
							'pricing-toggle__btn',
							selectedPlan === 'quarterly' && 'pricing-toggle__btn--active'
						]}>Quarterly</button
					>
					<button
						onclick={() => (selectedPlan = 'annual')}
						class={[
							'pricing-toggle__btn',
							selectedPlan === 'annual' && 'pricing-toggle__btn--active'
						]}>Annual</button
					>

					<div
						class="pricing-toggle__indicator"
						style="left: {selectedPlan === 'monthly'
							? '0.375rem'
							: selectedPlan === 'quarterly'
								? 'calc(33.33% + 0.2rem)'
								: 'calc(66.66% + 0.1rem)'}; width: calc(33.33% - 0.4rem);"
					></div>
				</div>
			</div>

			<div class="pricing__grid">
				{#each plans as plan (plan.id)}
					{#if plan.variant === 'simple'}
						<div class={['plan plan--simple', selectedPlan === plan.id && 'plan--active']}>
							<h3 class="plan__name">{plan.label}</h3>
							<div class="plan__price-row">
								<span class="plan__price">${plan.price}</span>
								<span class="plan__period">{plan.period}</span>
							</div>
							<div class="plan__perday">
								{plan.perDay}
							</div>
							<ul class="plan__features">
								{#each plan.features as feature, i (i)}
									<li class="plan__feat"><span class="plan__tick">&#x2713;</span> {feature}</li>
								{/each}
							</ul>
							<a href={plan.checkoutHref} class="plan__cta plan__cta--ghost">Select {plan.label}</a>
						</div>
					{:else if plan.variant === 'featured'}
						<div class={['plan plan--featured', selectedPlan === plan.id && 'plan--active']}>
							<div class="plan__ribbon plan__ribbon--primary">Most Popular</div>
							<h3 class="plan__name plan__name--lg">{plan.label}</h3>
							<div class="plan__price-row">
								<span class="plan__price plan__price--lg">${plan.price}</span>
								<span class="plan__period">{plan.period}</span>
							</div>
							<div class="plan__perday plan__perday--emerald">
								{plan.savingsCopy}
							</div>
							<ul class="plan__features plan__features--featured">
								{#each plan.features as feature, i (i)}
									<li class="plan__feat">
										<span class="plan__tick plan__tick--bold">&#x2713;</span>
										<span class="plan__feat-text--bold">{feature}</span>
									</li>
								{/each}
							</ul>
							<a href={plan.checkoutHref} class="plan__cta plan__cta--primary">Join {plan.label}</a>
						</div>
					{:else if plan.variant === 'highlight'}
						<div class={['plan plan--highlight', selectedPlan === plan.id && 'plan--active']}>
							<div class="plan__ribbon plan__ribbon--emerald">Best Deal</div>
							<h3 class="plan__name">{plan.label}</h3>
							<div class="plan__price-row">
								<span class="plan__price">${plan.price}</span>
								<span class="plan__period">{plan.period}</span>
							</div>
							<div class="plan__perday plan__perday--emerald-soft">
								{plan.savingsCopy}
							</div>
							<ul class="plan__features">
								{#each plan.features as feature, i (i)}
									<li class="plan__feat">
										<span class="plan__tick">&#x2713;</span>
										<span class="plan__feat-text--bold">{feature}</span>
									</li>
								{/each}
							</ul>
							<a href={plan.checkoutHref} class="plan__cta plan__cta--emerald"
								>Select {plan.label}</a
							>
						</div>
					{/if}
				{/each}
			</div>

			<div class="pricing__footer">
				<p class="pricing__secure">
					<IconLock size={16} stroke={2} aria-hidden="true" />
					Secure checkout powered by Stripe. Cancel anytime.
				</p>
			</div>
		</div>
	</section>

	<section class="faq">
		<div class="faq__inner">
			<h2 class="faq__title">Frequently Asked Questions</h2>
			<div class="faq__list">
				{#each faqList as faq, i (faq.question)}
					<div class="faq__item">
						<button class="faq__trigger" onclick={() => toggleFaq(i)} aria-expanded={openFaq === i}>
							<span class="faq__q">{faq.question}</span>
							<svg
								class="faq__chevron {openFaq === i ? 'faq__chevron--open' : ''}"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M19 9l-7 7-7-7"
								/></svg
							>
						</button>
						{#if openFaq === i}
							<div transition:slide={{ duration: 300, easing: cubicOut }} class="faq__panel">
								{faq.answer}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="bell-cta">
		<div class="bell-cta__pattern" aria-hidden="true"></div>
		<div class="bell-cta__inner">
			<h2 class="bell-cta__title">Markets Open at 9:30 AM.</h2>
			<p class="bell-cta__lede">
				Don't miss the next opening bell. Join the room today and be ready for tomorrow's session.
			</p>
			<div class="bell-cta__actions">
				<a href="#pricing" class="bell-cta__btn"> Get Access Now </a>
			</div>
			<p class="bell-cta__fine">30-Day Money Back Guarantee &bull; Cancel Anytime</p>
		</div>
	</section>
</div>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Page-local tokens. Two-color-rail page (blue primary, emerald
	   accents on the featured plan + market-open row).
	   ───────────────────────────────────────────────────────────────── */
	.day-trading {
		--dt-discord-bg: #1e1e24;
		--dt-discord-bar: #2b2b36;

		width: 100%;
		background: var(--rtp-bg);
		color: var(--rtp-text);
		font-family: var(--rtp-font-sans);
	}
	.day-trading ::selection {
		background: var(--rtp-primary);
		color: #fff;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Hero
	   ───────────────────────────────────────────────────────────────── */
	.hero {
		position: relative;
		min-height: 90vh;
		display: flex;
		align-items: center;
		overflow: hidden;
		padding-block: 6rem;
	}
	@media (min-width: 1024px) {
		.hero {
			padding-block: 0;
		}
	}
	.hero__bg {
		position: absolute;
		inset: 0;
		background: var(--rtp-bg);
		z-index: 0;
		pointer-events: none;
	}
	.hero__bg-grid {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(rgba(59, 130, 246, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(59, 130, 246, 0.03) 1px, transparent 1px);
		background-size: 40px 40px;
		opacity: 0.3;
	}
	.hero__halo {
		position: absolute;
		border-radius: 50%;
		filter: blur(120px);
	}
	.hero__halo--blue {
		top: -10%;
		right: -5%;
		width: 800px;
		height: 800px;
		background: color-mix(in oklab, var(--rtp-blue) 10%, transparent);
		animation: hero-pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	.hero__halo--indigo {
		bottom: -10%;
		left: -10%;
		width: 600px;
		height: 600px;
		background: color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
		filter: blur(100px);
	}
	@keyframes hero-pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.hero__inner {
		position: relative;
		z-index: 10;
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
		display: grid;
		gap: 4rem;
		align-items: center;
	}
	@media (min-width: 640px) {
		.hero__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.hero__inner {
			padding-inline: 2rem;
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.hero__copy {
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}
	@media (min-width: 1024px) {
		.hero__copy {
			text-align: start;
		}
	}

	.hero__live-pill {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		background: color-mix(in oklab, var(--rtp-red) 10%, transparent);
		border: 1px solid color-mix(in oklab, var(--rtp-red) 20%, transparent);
		padding: 0.375rem 1rem;
		border-radius: var(--rtp-radius-pill);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		align-self: center;
		width: max-content;
	}
	@media (min-width: 1024px) {
		.hero__live-pill {
			align-self: flex-start;
		}
	}
	.hero__live-pulse {
		position: relative;
		display: inline-flex;
		width: 0.625rem;
		height: 0.625rem;
	}
	.hero__live-ping {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: var(--rtp-red);
		opacity: 0.75;
		animation: hero-live-ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite;
	}
	.hero__live-dot {
		position: relative;
		display: inline-flex;
		width: 0.625rem;
		height: 0.625rem;
		border-radius: 50%;
		background: #dc2626;
	}
	@keyframes hero-live-ping {
		75%,
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}
	.hero__live-label {
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--rtp-red-soft);
	}

	.hero__title {
		font-family: var(--rtp-font-display);
		font-size: 3rem;
		font-weight: 800;
		line-height: 1.1;
		letter-spacing: -0.025em;
	}
	@media (min-width: 768px) {
		.hero__title {
			font-size: 4.5rem;
		}
	}
	.hero__title-accent {
		background: linear-gradient(to right, var(--rtp-blue), #818cf8, #fff);
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}

	.hero__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
		max-width: 42rem;
		margin-inline: auto;
		line-height: 1.625;
	}
	@media (min-width: 1024px) {
		.hero__lede {
			margin-inline: 0;
		}
	}

	.hero__cta-row {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		justify-content: center;
	}
	@media (min-width: 640px) {
		.hero__cta-row {
			flex-direction: row;
		}
	}
	@media (min-width: 1024px) {
		.hero__cta-row {
			justify-content: flex-start;
		}
	}

	.hero__cta {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 1rem 2rem;
		font-size: 1.125rem;
		font-weight: 700;
		border-radius: var(--rtp-radius-md);
		text-decoration: none;
		transition: all 0.2s var(--rtp-ease-out);
	}
	.hero__cta--primary {
		background: var(--rtp-primary);
		color: #fff;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, #000 30%, transparent);
	}
	.hero__cta--primary:hover {
		background: #2563eb;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-primary) 25%, transparent);
		transform: translateY(-0.25rem);
	}
	.hero__cta--primary:focus {
		outline: none;
		box-shadow:
			0 0 0 2px var(--rtp-bg),
			0 0 0 4px var(--rtp-primary);
	}
	.hero__cta-arrow {
		width: 1.25rem;
		height: 1.25rem;
		margin-inline-start: 0.5rem;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.hero__cta--primary:hover .hero__cta-arrow {
		transform: translateX(0.25rem);
	}
	.hero__cta--ghost {
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border);
		color: var(--rtp-text);
	}
	.hero__cta--ghost:hover {
		background: color-mix(in oklab, var(--rtp-surface) 80%, transparent);
		border-color: color-mix(in oklab, var(--rtp-primary) 30%, transparent);
	}

	.hero__social {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		font-size: 0.875rem;
		color: var(--rtp-muted);
		padding-block-start: 1rem;
	}
	@media (min-width: 1024px) {
		.hero__social {
			justify-content: flex-start;
		}
	}
	.hero__avatars {
		display: flex;
	}
	.hero__avatars > * + * {
		margin-inline-start: -0.75rem;
	}
	.hero__avatar {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		border: 2px solid var(--rtp-bg);
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		font-size: 0.75rem;
		font-weight: 700;
		box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
	}
	.hero__avatar--blue {
		background: linear-gradient(to bottom right, #3b82f6, #2563eb);
	}
	.hero__avatar--emerald {
		background: linear-gradient(to bottom right, #10b981, #059669);
	}
	.hero__avatar--purple {
		background: linear-gradient(to bottom right, #a855f7, #7e22ce);
	}
	.hero__avatar--more {
		background: var(--rtp-surface);
	}
	.hero__social-meta {
		display: flex;
		flex-direction: column;
		font-size: 0.75rem;
	}
	.hero__social-count {
		color: #fff;
		font-weight: 700;
	}

	/* ── Hero chrome (Discord-mock visual) ── */
	.hero__chrome {
		display: none;
		position: relative;
	}
	@media (min-width: 1024px) {
		.hero__chrome {
			display: block;
			perspective: 1000px;
		}
	}
	.hero__chrome-glow {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to top right,
			color-mix(in oklab, var(--rtp-primary) 20%, transparent),
			transparent
		);
		border-radius: 50%;
		filter: blur(48px);
		transform: translate(2.5rem, 2.5rem);
	}
	.hero__chrome-card {
		position: relative;
		background: var(--dt-discord-bg);
		border: 1px solid var(--rtp-border);
		border-radius: var(--rtp-radius-md);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		overflow: hidden;
		transform: rotateY(-5deg);
		transition: transform 500ms var(--rtp-ease-out);
	}
	.hero__chrome-card:hover {
		transform: rotateY(0);
	}
	.hero__chrome-tab {
		background: var(--dt-discord-bar);
		padding: 1rem;
		border-block-end: 1px solid #374151;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.hero__chrome-menu {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #9ca3af;
	}
	.hero__chrome-channel {
		font-weight: 700;
		color: #fff;
		letter-spacing: 0.025em;
		font-size: 0.875rem;
	}
	.hero__chrome-live {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: color-mix(in oklab, var(--rtp-red) 20%, transparent);
		padding: 0.25rem 0.75rem;
		border-radius: var(--rtp-radius-sm);
		color: var(--rtp-red-soft);
		font-size: 0.75rem;
		font-weight: 700;
	}
	.hero__chrome-live-dot {
		width: 0.5rem;
		height: 0.5rem;
		background: var(--rtp-red);
		border-radius: 50%;
		animation: hero-live-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	@keyframes hero-live-pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.hero__chrome-body {
		display: grid;
		grid-template-columns: 2fr 1fr;
		height: 400px;
	}
	.hero__chart-pane {
		background: #111827;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		border-inline-end: 1px solid #374151;
		position: relative;
	}
	.hero__chart-label {
		position: absolute;
		top: 1rem;
		left: 1rem;
		font-size: 0.75rem;
		color: #6b7280;
		font-family: var(--rtp-font-mono);
	}
	.hero__chart-svg {
		width: 100%;
		height: 16rem;
		color: var(--rtp-primary);
		opacity: 0.8;
	}
	.hero__chart-grid {
		color: #4b5563;
	}
	.hero__chart-blip {
		fill: currentColor;
		animation: hero-blip 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	@keyframes hero-blip {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}
	.hero__chart-quote {
		position: absolute;
		bottom: 1rem;
		right: 1rem;
		background: color-mix(in oklab, #000 60%, transparent);
		backdrop-filter: blur(4px);
		padding: 0.375rem 0.75rem;
		border-radius: var(--rtp-radius-sm);
		border: 1px solid color-mix(in oklab, #fff 10%, transparent);
		color: var(--rtp-emerald-bright);
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
	}

	.hero__chat-pane {
		background: var(--dt-discord-bar);
		padding: 0.75rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		overflow: hidden;
	}
	.hero__chat-heading {
		font-size: 0.75rem;
		color: #6b7280;
		font-weight: 700;
		margin-block-end: 0.5rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}
	.hero__chat-row {
		display: flex;
		gap: 0.5rem;
		align-items: flex-start;
	}
	.hero__chat-row--dim {
		opacity: 0.6;
	}
	.hero__chat-avatar {
		width: 1.5rem;
		height: 1.5rem;
		border-radius: 50%;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.625rem;
		color: #fff;
		font-weight: 700;
	}
	.hero__chat-avatar--blue {
		background: #3b82f6;
	}
	.hero__chat-avatar--emerald {
		background: #10b981;
	}
	.hero__chat-avatar--indigo {
		background: #6366f1;
	}
	.hero__chat-avatar--orange {
		background: #f97316;
	}
	.hero__chat-skel {
		height: 0.5rem;
		border-radius: var(--rtp-radius-sm);
	}
	.hero__chat-skel--name {
		width: 4rem;
		background: #4b5563;
		margin-block-end: 0.25rem;
	}
	.hero__chat-skel--msg {
		width: 6rem;
		background: #374151;
	}
	.hero__chat-text {
		font-size: 0.75rem;
	}
	.hero__chat-author {
		color: #9ca3af;
		font-weight: 700;
	}
	.hero__chat-author--mod {
		color: var(--rtp-emerald-bright);
	}
	.hero__chat-msg {
		color: #d1d5db;
	}
	.hero__chat-input {
		margin-block-start: auto;
		background: color-mix(in oklab, #374151 50%, transparent);
		padding: 0.5rem;
		border-radius: var(--rtp-radius-sm);
		font-size: 0.75rem;
		color: #9ca3af;
		border: 1px solid color-mix(in oklab, #4b5563 30%, transparent);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Stats bar
	   ───────────────────────────────────────────────────────────────── */
	.stats {
		background: var(--rtp-surface);
		border-block: 1px solid var(--rtp-border);
		position: relative;
		z-index: 20;
	}
	.stats__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding: 3rem 1rem;
	}
	@media (min-width: 640px) {
		.stats__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.stats__inner {
			padding-inline: 2rem;
		}
	}
	.stats__grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 2rem;
	}
	@media (min-width: 768px) {
		.stats__grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}
	.stats__cell {
		text-align: center;
		cursor: default;
	}
	.stats__value {
		font-size: 1.875rem;
		font-weight: 800;
		color: #fff;
		margin-block-end: 0.25rem;
		transition: color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	@media (min-width: 768px) {
		.stats__value {
			font-size: 2.25rem;
		}
	}
	.stats__cell:hover .stats__value--primary {
		color: var(--rtp-primary);
	}
	.stats__cell:hover .stats__value--emerald {
		color: var(--rtp-emerald);
	}
	.stats__cell:hover .stats__value--indigo {
		color: var(--rtp-indigo);
	}
	.stats__cell:hover .stats__value--blue {
		color: var(--rtp-blue);
	}
	.stats__label {
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: var(--rtp-muted);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Feature cards
	   ───────────────────────────────────────────────────────────────── */
	.features {
		padding-block: 6rem;
		background: var(--rtp-bg);
		position: relative;
		overflow: hidden;
	}
	.features__pattern {
		position: absolute;
		inset: 0;
		background-image: url('/grid-pattern.svg');
		opacity: 0.05;
		pointer-events: none;
	}
	.features__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
		position: relative;
		z-index: 10;
	}
	@media (min-width: 640px) {
		.features__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.features__inner {
			padding-inline: 2rem;
		}
	}
	.features__header {
		text-align: center;
		max-width: 48rem;
		margin-inline: auto;
		margin-block-end: 5rem;
	}
	.features__eyebrow {
		color: var(--rtp-primary);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.875rem;
		margin-block-end: 0.5rem;
		display: block;
	}
	.features__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.features__title {
			font-size: 3rem;
		}
	}
	.features__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
	}
	.features__grid {
		display: grid;
		gap: 2rem;
	}
	@media (min-width: 768px) {
		.features__grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
	.feature-card {
		background: var(--rtp-surface);
		padding: 2rem;
		border-radius: 1rem;
		border: 1px solid var(--rtp-border);
		transition:
			border-color var(--rtp-dur-base) var(--rtp-ease-out),
			transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.feature-card:hover {
		transform: translateY(-0.5rem);
	}
	.feature-card--primary:hover {
		border-color: color-mix(in oklab, var(--rtp-primary) 50%, transparent);
	}
	.feature-card--emerald:hover {
		border-color: color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
	}
	.feature-card--indigo:hover {
		border-color: color-mix(in oklab, var(--rtp-indigo) 50%, transparent);
	}
	.feature-card__icon {
		width: 3.5rem;
		height: 3.5rem;
		border-radius: var(--rtp-radius-md);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-block-end: 1.5rem;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.feature-card__icon svg {
		width: 1.75rem;
		height: 1.75rem;
	}
	.feature-card:hover .feature-card__icon {
		transform: scale(1.1);
	}
	.feature-card--primary .feature-card__icon {
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
		color: var(--rtp-primary);
	}
	.feature-card--emerald .feature-card__icon {
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		color: var(--rtp-emerald);
	}
	.feature-card--indigo .feature-card__icon {
		background: color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
		color: var(--rtp-indigo);
	}
	.feature-card__title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 0.75rem;
	}
	.feature-card__desc {
		color: var(--rtp-muted);
		font-size: 0.875rem;
		line-height: 1.625;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Why SPX
	   ───────────────────────────────────────────────────────────────── */
	.why {
		padding-block: 5rem;
		background: var(--rtp-surface);
		border-block: 1px solid var(--rtp-border);
	}
	.why__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.why__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.why__inner {
			padding-inline: 2rem;
		}
	}
	.why__grid {
		display: grid;
		gap: 3rem;
		align-items: center;
	}
	@media (min-width: 1024px) {
		.why__grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
	.why__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.why__title {
			font-size: 2.25rem;
		}
	}
	.why__lede {
		font-size: 1.125rem;
		color: var(--rtp-muted);
		margin-block-end: 1.5rem;
	}
	.why__list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		list-style: none;
		margin: 0;
		padding: 0;
	}
	.why__item {
		display: flex;
		gap: 1rem;
		align-items: flex-start;
	}
	.why__check {
		width: 1.5rem;
		height: 1.5rem;
		border-radius: var(--rtp-radius-sm);
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--rtp-emerald);
		margin-block-start: 0.25rem;
		flex-shrink: 0;
	}
	.why__item-title {
		color: #fff;
		font-weight: 700;
	}
	.why__item-desc {
		font-size: 0.875rem;
		color: var(--rtp-muted);
	}

	.why__media {
		position: relative;
		border-radius: 1rem;
		overflow: hidden;
		border: 1px solid var(--rtp-border);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
	}
	.why__media-overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(to top, var(--rtp-bg), transparent);
		opacity: 0.6;
		z-index: 10;
	}
	.why__media-inner {
		background: #111827;
		aspect-ratio: 16 / 9;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
	}
	.why__media-grid {
		position: absolute;
		inset: 0;
		background-image: url('/grid-pattern.svg');
		opacity: 0.2;
	}
	.why__media-glyph {
		z-index: 20;
		text-align: center;
	}
	.why__media-text {
		font-size: 3.75rem;
		font-weight: 900;
		color: color-mix(in oklab, #fff 10%, transparent);
		letter-spacing: 0.1em;
	}
	.why__media-curves {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		color: var(--rtp-primary);
		opacity: 0.3;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Schedule / Timeline
	   ───────────────────────────────────────────────────────────────── */
	.schedule {
		padding-block: 6rem;
		background: var(--rtp-bg);
		position: relative;
	}
	.schedule__inner {
		max-width: 64rem;
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.schedule__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.schedule__inner {
			padding-inline: 2rem;
		}
	}
	.schedule__header {
		text-align: center;
		margin-block-end: 4rem;
	}
	.schedule__eyebrow {
		color: var(--rtp-primary);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.875rem;
		margin-block-end: 0.5rem;
		display: block;
	}
	.schedule__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1rem;
	}
	@media (min-width: 768px) {
		.schedule__title {
			font-size: 3rem;
		}
	}
	.schedule__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
	}

	.timeline {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 3rem;
	}
	.timeline::before {
		content: '';
		position: absolute;
		top: 0;
		bottom: 0;
		left: 1.25rem;
		transform: translateX(-50%);
		width: 2px;
		height: 100%;
		background: linear-gradient(
			to bottom,
			transparent,
			color-mix(in oklab, var(--rtp-border) 60%, transparent),
			transparent
		);
	}
	@media (min-width: 768px) {
		.timeline::before {
			left: 50%;
		}
	}
	.timeline__row {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	@media (min-width: 768px) {
		.timeline__row {
			justify-content: normal;
		}
		.timeline__row:nth-child(odd) {
			flex-direction: row-reverse;
		}
	}

	.timeline__pin {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		background: var(--rtp-bg);
		flex-shrink: 0;
		font-weight: 700;
		font-size: 0.75rem;
		z-index: 10;
		border: 1px solid var(--rtp-border);
		box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
	}
	@media (min-width: 768px) {
		.timeline__pin {
			order: 1;
		}
		.timeline__row:nth-child(odd) .timeline__pin {
			transform: translateX(-50%);
		}
		.timeline__row:nth-child(even) .timeline__pin {
			transform: translateX(50%);
		}
	}
	.timeline__pin--primary {
		color: var(--rtp-primary);
	}
	.timeline__pin--emerald {
		border-color: var(--rtp-emerald);
		background: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		color: var(--rtp-emerald);
	}
	.timeline__pin--muted {
		color: var(--rtp-muted);
	}
	.timeline__pin--indigo {
		color: var(--rtp-indigo);
	}

	.timeline__card {
		width: calc(100% - 4rem);
		background: var(--rtp-surface);
		padding: 1.5rem;
		border-radius: var(--rtp-radius-md);
		border: 1px solid var(--rtp-border);
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
	}
	@media (min-width: 768px) {
		.timeline__card {
			width: calc(50% - 2.5rem);
		}
	}
	.timeline__card--featured {
		background: var(--rtp-bg);
		border-left: 4px solid var(--rtp-emerald);
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.5),
			0 0 0 1px color-mix(in oklab, #fff 5%, transparent);
	}
	.timeline__card-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: 0.5rem;
	}
	.timeline__card-title {
		font-weight: 700;
		color: #fff;
		font-size: 1.125rem;
	}
	.timeline__chip {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		padding: 0.25rem 0.5rem;
		border-radius: var(--rtp-radius-sm);
	}
	.timeline__chip--primary {
		color: var(--rtp-primary);
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
	}
	.timeline__chip--emerald {
		color: var(--rtp-emerald);
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
	}
	.timeline__chip--muted {
		color: var(--rtp-muted);
		background: color-mix(in oklab, var(--rtp-border) 30%, transparent);
	}
	.timeline__chip--indigo {
		color: var(--rtp-indigo);
		background: color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
	}
	.timeline__card-desc {
		font-size: 0.875rem;
		color: var(--rtp-muted);
		line-height: 1.625;
	}
	.timeline__emph {
		color: #fff;
		font-weight: 700;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Pricing
	   ───────────────────────────────────────────────────────────────── */
	.pricing {
		padding-block: 6rem;
		background: var(--rtp-surface);
		position: relative;
	}
	.pricing__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.pricing__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.pricing__inner {
			padding-inline: 2rem;
		}
	}
	.pricing__header {
		text-align: center;
		margin-block-end: 4rem;
	}
	.pricing__eyebrow {
		color: var(--rtp-primary);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.875rem;
		margin-block-end: 0.5rem;
		display: block;
	}
	.pricing__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1rem;
	}
	@media (min-width: 768px) {
		.pricing__title {
			font-size: 3rem;
		}
	}
	.pricing__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
		max-width: 42rem;
		margin-inline: auto;
	}

	.pricing__toggle-row {
		display: flex;
		justify-content: center;
		margin-block-end: 4rem;
	}
	.pricing-toggle {
		background: var(--rtp-bg);
		padding: 0.375rem;
		border-radius: var(--rtp-radius-md);
		border: 1px solid var(--rtp-border);
		display: inline-flex;
		position: relative;
		box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
	}
	.pricing-toggle__btn {
		position: relative;
		z-index: 10;
		padding: 0.5rem 1.5rem;
		border-radius: var(--rtp-radius-sm);
		font-weight: 700;
		font-size: 0.875rem;
		background: transparent;
		border: 0;
		color: var(--rtp-muted);
		cursor: pointer;
		transition: color 0.2s;
	}
	.pricing-toggle__btn:hover {
		color: #fff;
	}
	.pricing-toggle__btn--active {
		color: #fff;
	}
	.pricing-toggle__indicator {
		position: absolute;
		top: 0.375rem;
		bottom: 0.375rem;
		background: var(--rtp-primary);
		border-radius: var(--rtp-radius-sm);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
		transition: all 300ms cubic-bezier(0.25, 0.1, 0.25, 1);
	}

	.pricing__grid {
		display: grid;
		gap: 2rem;
		max-width: 72rem;
		margin-inline: auto;
		align-items: center;
		margin-block-start: 2rem;
	}
	@media (min-width: 768px) {
		.pricing__grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.plan {
		background: var(--rtp-bg);
		padding: 2rem;
		border-radius: 1rem;
		border: 1px solid var(--rtp-border);
		transition: all 0.3s;
		opacity: 0.7;
		position: relative;
	}
	.plan:hover {
		opacity: 1;
	}
	.plan--featured {
		padding: 2.5rem;
		border-radius: 0.75rem;
		border-width: 2px;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		z-index: 10;
		opacity: 0.7;
	}
	.plan--simple.plan--active {
		border-color: var(--rtp-primary);
		opacity: 1;
		scale: 1.05;
		box-shadow: 0 20px 25px -5px color-mix(in oklab, var(--rtp-primary) 10%, transparent);
	}
	.plan--featured.plan--active {
		border-color: var(--rtp-primary);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--rtp-primary) 20%, transparent);
		opacity: 1;
	}
	@media (min-width: 768px) {
		.plan--featured.plan--active {
			scale: 1.1;
		}
	}
	.plan--highlight.plan--active {
		border-color: var(--rtp-emerald);
		opacity: 1;
		scale: 1.05;
		box-shadow: 0 20px 25px -5px color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
	}

	.plan__ribbon {
		position: absolute;
		top: 0;
		left: 50%;
		transform: translate(-50%, -50%);
		padding: 0.25rem 1rem;
		border-radius: var(--rtp-radius-pill);
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #fff;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
		white-space: nowrap;
	}
	.plan__ribbon--primary {
		background: var(--rtp-primary);
	}
	.plan__ribbon--emerald {
		background: var(--rtp-emerald);
		top: -0.75rem;
		transform: translateX(-50%);
		z-index: 10;
	}

	.plan__name {
		font-size: 1.25rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 1rem;
	}
	.plan__name--lg {
		font-size: 1.5rem;
	}
	.plan__price-row {
		display: flex;
		align-items: baseline;
		gap: 0.25rem;
		margin-block-end: 1.5rem;
	}
	.plan__price {
		font-size: 2.25rem;
		font-weight: 700;
		color: #fff;
	}
	.plan__price--lg {
		font-size: 3rem;
		font-weight: 800;
	}
	.plan__period {
		color: var(--rtp-muted);
	}
	.plan__perday {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-muted);
		background: var(--rtp-surface);
		padding: 0.5rem;
		border-radius: var(--rtp-radius-sm);
		margin-block-end: 1.5rem;
		text-align: center;
		border: 1px solid var(--rtp-border);
	}
	.plan__perday--emerald {
		color: var(--rtp-emerald-bright);
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
	}
	.plan__perday--emerald-soft {
		color: var(--rtp-emerald);
	}
	.plan__features {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-block-end: 2rem;
		font-size: 0.875rem;
		color: var(--rtp-muted);
		list-style: none;
		margin-inline: 0;
		padding: 0;
	}
	.plan__features--featured {
		color: #fff;
	}
	.plan__feat {
		display: flex;
		gap: 0.75rem;
	}
	.plan__tick {
		color: var(--rtp-primary);
	}
	.plan__tick--bold {
		font-weight: 700;
	}
	.plan__feat-text--bold {
		font-weight: 700;
	}
	.plan__cta {
		display: block;
		width: 100%;
		padding-block: 0.75rem;
		font-weight: 700;
		border-radius: var(--rtp-radius-sm);
		text-align: center;
		text-decoration: none;
		transition:
			background var(--rtp-dur-base) var(--rtp-ease-out),
			color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.plan__cta--ghost {
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border);
		color: #fff;
	}
	.plan__cta--ghost:hover {
		background: #fff;
		color: #000;
	}
	.plan__cta--primary {
		padding-block: 1rem;
		background: var(--rtp-primary);
		color: #fff;
		border-radius: var(--rtp-radius-md);
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
	}
	.plan__cta--primary:hover {
		background: #2563eb;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-primary) 50%, transparent);
	}
	.plan__cta--emerald {
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-emerald);
		color: var(--rtp-emerald);
	}
	.plan__cta--emerald:hover {
		background: var(--rtp-emerald);
		color: #fff;
	}

	.pricing__footer {
		margin-block-start: 3rem;
		text-align: center;
	}
	.pricing__secure {
		color: var(--rtp-muted);
		font-size: 0.875rem;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   FAQ
	   ───────────────────────────────────────────────────────────────── */
	.faq {
		padding-block: 6rem;
		background: var(--rtp-bg);
		border-block-start: 1px solid var(--rtp-border);
	}
	.faq__inner {
		max-width: 48rem;
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.faq__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.faq__inner {
			padding-inline: 2rem;
		}
	}
	.faq__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		text-align: center;
		color: #fff;
		margin-block-end: 3rem;
	}
	.faq__list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.faq__item {
		border: 1px solid var(--rtp-border);
		border-radius: var(--rtp-radius-md);
		background: var(--rtp-surface);
		overflow: hidden;
		transition: border-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.faq__item:hover {
		border-color: color-mix(in oklab, var(--rtp-primary) 30%, transparent);
	}
	.faq__trigger {
		width: 100%;
		text-align: start;
		padding: 1.25rem 1.5rem;
		font-weight: 700;
		display: flex;
		justify-content: space-between;
		align-items: center;
		background: transparent;
		border: 0;
		color: #fff;
		cursor: pointer;
		font: inherit;
		font-weight: 700;
		transition: background var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.faq__trigger:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}
	.faq__trigger:focus {
		outline: none;
	}
	.faq__q {
		font-size: 1rem;
		padding-inline-end: 1rem;
	}
	.faq__chevron {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--rtp-primary);
		flex-shrink: 0;
		transition: transform 300ms;
	}
	.faq__chevron--open {
		transform: rotate(180deg);
	}
	.faq__panel {
		padding: 1rem 1.5rem 1.5rem;
		color: var(--rtp-muted);
		font-size: 0.875rem;
		line-height: 1.625;
		border-block-start: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Bell CTA
	   ───────────────────────────────────────────────────────────────── */
	.bell-cta {
		padding-block: 6rem;
		background: linear-gradient(to bottom right, var(--rtp-primary), var(--rtp-indigo));
		color: #fff;
		position: relative;
		overflow: hidden;
	}
	.bell-cta__pattern {
		position: absolute;
		inset: 0;
		background-image: url('/grid-pattern.svg');
		opacity: 0.1;
		pointer-events: none;
	}
	.bell-cta__inner {
		max-width: 56rem;
		margin-inline: auto;
		padding-inline: 1rem;
		text-align: center;
		position: relative;
		z-index: 10;
	}
	@media (min-width: 640px) {
		.bell-cta__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.bell-cta__inner {
			padding-inline: 2rem;
		}
	}
	.bell-cta__title {
		font-family: var(--rtp-font-display);
		font-size: 2.25rem;
		font-weight: 800;
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.bell-cta__title {
			font-size: 3.75rem;
		}
	}
	.bell-cta__lede {
		font-size: 1.25rem;
		color: #dbeafe;
		max-width: 42rem;
		margin-inline: auto;
		margin-block-end: 2.5rem;
	}
	.bell-cta__actions {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		justify-content: center;
	}
	@media (min-width: 640px) {
		.bell-cta__actions {
			flex-direction: row;
		}
	}
	.bell-cta__btn {
		background: #fff;
		color: var(--rtp-primary);
		padding: 1rem 2.5rem;
		border-radius: var(--rtp-radius-md);
		font-weight: 700;
		font-size: 1.125rem;
		text-decoration: none;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		transition: all 0.3s var(--rtp-ease-out);
	}
	.bell-cta__btn:hover {
		background: #eff6ff;
		transform: translateY(-0.25rem);
	}
	.bell-cta__fine {
		margin-block-start: 2rem;
		font-size: 0.875rem;
		color: color-mix(in oklab, #fff 60%, transparent);
	}
</style>
