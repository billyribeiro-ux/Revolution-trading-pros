<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { browser } from '$app/environment';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';

	// --- Pricing State (Svelte 5 Runes) ---
	let selectedPlan: 'monthly' | 'quarterly' | 'annual' = $state('quarterly');

	// --- FAQ Logic (Svelte 5 Runes) ---
	let openFaq: number | null = $state(null);
	const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

	// --- GSAP ScrollTrigger Animations (Svelte 5 SSR-safe pattern) ---
	onMount(() => {
		if (!browser) return;

		let isComponentMounted = true;
		let ctx: ReturnType<typeof import('gsap').gsap.context> | null = null;

		void (async () => {
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
				ctx = gsap.context(() => {
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

					ScrollTrigger.refresh();
				});
			} catch (error) {
				console.error('[SwingTrading] GSAP initialization failed:', error);
			}
		})();

		return () => {
			isComponentMounted = false;
			if (ctx) ctx.revert();
		};
	});

	// --- DATA SOURCE: FAQs (Single Source of Truth) ---
	const faqList = [
		{
			question: 'How much capital do I need for swing trading?',
			answer:
				'We recommend a minimum of $2,000 to properly manage risk. Unlike day trading, you do NOT need $25,000 since you are not subject to Pattern Day Trader (PDT) rules if using a cash account or limiting frequency. Swing trading is very capital efficient.'
		},
		{
			question: 'Are these day trades?',
			answer:
				'No. These are swing trades designed to be held for 3-7 days on average. The goal is to catch larger moves (5-10% on stock, 20-100% on options) without staring at the screen all day.'
		},
		{
			question: 'Do I need to be at my computer all day?',
			answer:
				'Absolutely not. This service is specifically built for people with day jobs. We send alerts via SMS, Email, and Push Notification. You can execute the trade on your phone in under 60 seconds.'
		},
		{
			question: 'What instruments do you trade?',
			answer:
				'We primarily trade equity options (Calls and Puts) on large-cap tech stocks (NVDA, AMD, TSLA, MSFT) and major indices (SPY, QQQ). We occasionally trade common shares if the setup implies a longer-term hold.'
		},
		{
			question: 'How are the alerts delivered?',
			answer:
				'Redundancy is key. You receive alerts through our private Discord server, instant Push Notifications via our mobile app integration, and optional SMS text messages. You will never miss a setup.'
		},
		{
			question: 'Do you trade both long and short?',
			answer:
				'Yes. We are market agnostic. We buy Calls when the trend is up, and we buy Puts when the trend breaks down. Making money in bear markets is a core part of our strategy.'
		},
		{
			question: 'What is your risk management strategy?',
			answer:
				'We are risk-first. Every alert comes with a predefined Stop Loss level. We generally look for a 1:3 Risk/Reward ratio. If a trade hits our invalidation point, we cut it immediately to preserve capital.'
		},
		{
			question: 'Is this suitable for beginners?',
			answer:
				'Yes. Because the pace is slower than day trading, beginners have more time to analyze the entry, check the chart, and execute the order without panic. We also provide a "Swing Trading Bootcamp" video course to get you started.'
		},
		{
			question: 'Can I cancel my subscription?',
			answer:
				'Yes, anytime. You can manage your subscription directly from the dashboard. There are no contracts or hidden fees.'
		},
		{
			question: 'What happens if I miss an entry?',
			answer:
				'We always provide an "Entry Zone" rather than a specific penny. If the price is still within the zone, you are good to enter. If it has run too far, we advise waiting for a pullback or skipping the trade. Never chase.'
		}
	];

	// SEO schemas emitted via +page.ts (page.data.seo).
</script>

<div class="swing-trading">
	<section class="hero">
		<div class="hero__bg" aria-hidden="true">
			<div class="hero__bg-grid"></div>
			<div class="hero__halo hero__halo--emerald"></div>
			<div class="hero__halo hero__halo--blue"></div>
		</div>

		<div class="hero__inner">
			<div class="hero__copy">
				<div data-gsap class="hero__pill">
					<span class="hero__pill-pulse">
						<span class="hero__pill-ping"></span>
						<span class="hero__pill-dot"></span>
					</span>
					<span class="hero__pill-label">Signals Active &bull; Last Alert: +45% (AMD)</span>
				</div>

				<h1 data-gsap class="hero__title">
					Catch the <br />
					<span class="hero__title-accent">Big Moves.</span>
				</h1>

				<p data-gsap class="hero__lede">
					Stop staring at the 1-minute chart. Get high-precision <strong
						>multi-day swing alerts</strong
					>
					designed for traders who want freedom, not another 9-to-5 job.
				</p>

				<div data-gsap class="hero__cta-row">
					<a href="#pricing" class="hero__cta hero__cta--primary group">
						Start Trading Swings
						<svg class="hero__cta-arrow" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 7l5 5m0 0l-5 5m5-5H6"
							/></svg
						>
					</a>
					<a href="#process" class="hero__cta hero__cta--ghost"> See How It Works </a>
				</div>

				<div data-gsap class="hero__bullets">
					<div class="hero__bullet">
						<svg class="hero__bullet-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>Precise Entries</span>
					</div>
					<div class="hero__bullet">
						<svg class="hero__bullet-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>3-7 Day Holds</span>
					</div>
					<div class="hero__bullet">
						<svg class="hero__bullet-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"
							/></svg
						>
						<span>SMS Alerts</span>
					</div>
				</div>
			</div>

			<div class="hero__chrome">
				<div class="hero__chrome-glow" aria-hidden="true"></div>

				<div class="hero__alert-card">
					<div class="hero__alert-head">
						<div>
							<h3 class="hero__alert-title">Swing Alert &#x1f680;</h3>
							<p class="hero__alert-sub">High Probability Setup</p>
						</div>
						<div class="hero__alert-time">Sent: 10:30 AM</div>
					</div>

					<div class="hero__alert-body">
						<div class="hero__alert-row hero__alert-row--action">
							<div class="hero__alert-label">Action</div>
							<div class="hero__alert-value">
								<span class="hero__alert-badge">BUY</span>
								NVDA 480 CALLS
							</div>
						</div>

						<div class="hero__alert-pair">
							<div class="hero__alert-stat">
								<div class="hero__alert-label">Entry Zone</div>
								<div class="hero__alert-num">$5.50 - $6.00</div>
							</div>
							<div class="hero__alert-stat">
								<div class="hero__alert-label">Target</div>
								<div class="hero__alert-num hero__alert-num--emerald">$8.50+</div>
							</div>
						</div>

						<div class="hero__alert-stop">
							<div class="hero__alert-stop-row">
								<div class="hero__alert-label">Invalidation (Stop)</div>
								<div class="hero__alert-stop-val">$4.20 (Hard Stop)</div>
							</div>
						</div>
					</div>

					<div class="hero__alert-bonus">
						<div class="hero__alert-bonus-label">Potential Return</div>
						<div class="hero__alert-bonus-value">+45%</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="stats">
		<div class="stats__inner">
			<dl class="stats__grid">
				<div class="stats__cell group">
					<dt class="stats__label">Historical Win Rate</dt>
					<dd class="stats__value stats__value--emerald">82%</dd>
				</div>
				<div class="stats__cell group">
					<dt class="stats__label">Avg Hold Time</dt>
					<dd class="stats__value stats__value--primary">
						3-7<span class="stats__unit">days</span>
					</dd>
				</div>
				<div class="stats__cell group">
					<dt class="stats__label">Risk/Reward</dt>
					<dd class="stats__value stats__value--indigo">4:1</dd>
				</div>
				<div class="stats__cell group">
					<dt class="stats__label">Alerts Per Week</dt>
					<dd class="stats__value stats__value--blue">2-4</dd>
				</div>
			</dl>
		</div>
	</section>

	<section class="compare">
		<div class="compare__inner">
			<div class="compare__header">
				<span class="compare__eyebrow">Lifestyle First</span>
				<h2 data-gsap class="compare__title">Choose Your Battle</h2>
				<p data-gsap class="compare__lede">
					Most traders burn out trying to scalp 1-minute candles. We play the bigger timeframe for
					bigger peace of mind.
				</p>
			</div>

			<div class="compare__grid">
				<div data-gsap class="compare-card compare-card--bad">
					<div class="compare-card__head">
						<div class="compare-card__emoji compare-card__emoji--muted">&#x1f630;</div>
						<h3 class="compare-card__title compare-card__title--muted">Day Scalper</h3>
					</div>
					<ul class="compare-card__list">
						<li class="compare-card__item">
							<span class="compare-card__x">&#x2715;</span> Glued to screen 6 hours/day
						</li>
						<li class="compare-card__item">
							<span class="compare-card__x">&#x2715;</span> High stress, cortisol spikes
						</li>
						<li class="compare-card__item">
							<span class="compare-card__x">&#x2715;</span> Expensive commissions eat profits
						</li>
						<li class="compare-card__item">
							<span class="compare-card__x">&#x2715;</span> "Did I miss the move?" anxiety
						</li>
					</ul>
				</div>

				<div data-gsap class="compare-card compare-card--good">
					<div class="compare-card__ribbon">RECOMMENDED</div>
					<div class="compare-card__head">
						<div class="compare-card__emoji compare-card__emoji--good">
							&#x1f9d8;&#x200d;&#x2642;&#xfe0f;
						</div>
						<h3 class="compare-card__title">Swing Trader</h3>
					</div>
					<ul class="compare-card__list compare-card__list--bold">
						<li class="compare-card__item">
							<svg class="compare-card__check" fill="none" stroke="currentColor" viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Check charts once a day (evening)
						</li>
						<li class="compare-card__item">
							<svg class="compare-card__check" fill="none" stroke="currentColor" viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Calm, calculated decisions
						</li>
						<li class="compare-card__item">
							<svg class="compare-card__check" fill="none" stroke="currentColor" viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Catch the meat of the move (20%+)
						</li>
						<li class="compare-card__item">
							<svg class="compare-card__check" fill="none" stroke="currentColor" viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Keep your day job &amp; freedom
						</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<section id="process" class="process">
		<div class="process__inner">
			<div class="process__header">
				<span class="process__eyebrow">How It Works</span>
				<h2 class="process__title">The Anatomy of a Swing Trade</h2>
			</div>

			<div class="process__grid">
				<article data-gsap class="pill-card pill-card--emerald group">
					<div class="pill-card__icon">
						<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
							/></svg
						>
					</div>
					<h3 class="pill-card__title">Institutional Flow Analysis</h3>
					<p class="pill-card__desc">
						Retail traders guess. We track <strong>Dark Pool prints</strong> and unusual options flow
						to identify where institutions are positioning billions of dollars before the price moves.
						We ride the whale's wake.
					</p>
				</article>

				<article data-gsap class="pill-card pill-card--primary group">
					<div class="pill-card__icon">
						<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"
							/></svg
						>
					</div>
					<h3 class="pill-card__title">Multi-Channel Alerts</h3>
					<p class="pill-card__desc">
						You can't miss the entry. We send alerts via <strong
							>SMS Text, Email, Push Notification, and Discord</strong
						>
						immediately when our criteria are met. You have a full window to enter the trade.
					</p>
				</article>

				<article data-gsap class="pill-card pill-card--indigo group">
					<div class="pill-card__icon">
						<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
							/></svg
						>
					</div>
					<h3 class="pill-card__title">Risk-First Approach</h3>
					<p class="pill-card__desc">
						Capital preservation is our #1 job. Every trade comes with a predefined "Hard Stop"
						level. We cut losers fast and let winners run to target. No bag holding. No hope.
					</p>
				</article>
			</div>
		</div>
	</section>

	<section class="recent">
		<div class="recent__inner">
			<div class="recent__header">
				<div>
					<h2 class="recent__title">Recent Verified Swings</h2>
					<p class="recent__lede">Real trades. Real timestamps. Verified community results.</p>
				</div>
				<div>
					<a href="/performance" class="recent__link">
						View Full Performance Log
						<IconArrowRight size={16} stroke={2} aria-hidden="true" />
					</a>
				</div>
			</div>

			<div class="recent__table">
				<div class="recent__row recent__row--head">
					<div class="recent__col recent__col--ticker">Ticker</div>
					<div class="recent__col recent__col--type">Type</div>
					<div class="recent__col recent__col--days">Days Held</div>
					<div class="recent__col recent__col--return">Return</div>
					<div class="recent__col recent__col--notes">Notes</div>
				</div>

				<div class="recent__row">
					<div class="recent__col recent__col--ticker recent__ticker">
						<img
							src="/logos/nvda.svg"
							alt="NVIDIA logo"
							class="recent__logo"
							width="20"
							height="20"
							loading="lazy"
						/>
						NVDA
					</div>
					<div class="recent__col recent__col--type recent__type--call">CALLS</div>
					<div class="recent__col recent__col--days recent__days">5 Days</div>
					<div class="recent__col recent__col--return recent__return recent__return--up">+125%</div>
					<div class="recent__col recent__col--notes recent__notes">
						Breakout over $480 psych level.
					</div>
				</div>

				<div class="recent__row">
					<div class="recent__col recent__col--ticker recent__ticker">
						<img
							src="/logos/amd.svg"
							alt="AMD logo"
							class="recent__logo"
							width="20"
							height="20"
							loading="lazy"
						/>
						AMD
					</div>
					<div class="recent__col recent__col--type recent__type--call">CALLS</div>
					<div class="recent__col recent__col--days recent__days">3 Days</div>
					<div class="recent__col recent__col--return recent__return recent__return--up">+45%</div>
					<div class="recent__col recent__col--notes recent__notes">
						Sector rotation play / AI Lag.
					</div>
				</div>

				<div class="recent__row recent__row--loss">
					<div class="recent__col recent__col--ticker recent__ticker">
						<img
							src="/logos/tsla.svg"
							alt="Tesla logo"
							class="recent__logo"
							width="20"
							height="20"
							loading="lazy"
						/>
						TSLA
					</div>
					<div class="recent__col recent__col--type recent__type--put">PUTS</div>
					<div class="recent__col recent__col--days recent__days">1 Day</div>
					<div class="recent__col recent__col--return recent__return recent__return--down">
						-15%
					</div>
					<div class="recent__col recent__col--notes recent__notes">
						Hit stop loss on reversal. Rule disciplined.
					</div>
				</div>

				<div class="recent__row">
					<div class="recent__col recent__col--ticker recent__ticker">
						<img
							src="/logos/meta.svg"
							alt="Meta logo"
							class="recent__logo"
							width="20"
							height="20"
							loading="lazy"
						/>
						META
					</div>
					<div class="recent__col recent__col--type recent__type--call">CALLS</div>
					<div class="recent__col recent__col--days recent__days">7 Days</div>
					<div class="recent__col recent__col--return recent__return recent__return--up">+82%</div>
					<div class="recent__col recent__col--notes recent__notes">
						Earnings run-up swing strategy.
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="pricing" class="pricing">
		<div class="pricing__inner">
			<div class="pricing__header">
				<span class="pricing__eyebrow">Membership</span>
				<h2 class="pricing__title">Simple Pricing</h2>
				<p class="pricing__lede">
					Pay for the subscription with your first successful swing trade.
				</p>
			</div>

			<div class="pricing__toggle-row">
				<div class="pricing-toggle">
					<button
						type="button"
						onclick={() => (selectedPlan = 'monthly')}
						class={[
							'pricing-toggle__btn',
							selectedPlan === 'monthly' && 'pricing-toggle__btn--active'
						]}>Monthly</button
					>
					<button
						type="button"
						onclick={() => (selectedPlan = 'quarterly')}
						class={[
							'pricing-toggle__btn',
							selectedPlan === 'quarterly' && 'pricing-toggle__btn--active'
						]}>Quarterly</button
					>
					<button
						type="button"
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
				<div class={['plan plan--simple', selectedPlan === 'monthly' && 'plan--active']}>
					<h3 class="plan__name">Monthly</h3>
					<div class="plan__price-row">
						<span class="plan__price">$97</span>
						<span class="plan__period">/mo</span>
					</div>
					<div class="plan__perday">$3.20 per day</div>
					<ul class="plan__features">
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span> 2-4 Premium Swings / Week
						</li>
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span> Instant SMS &amp; Email Alerts
						</li>
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span> Private Discord Community
						</li>
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span> Swing Bootcamp Video
						</li>
					</ul>
					<a href="/checkout/monthly-swings" class="plan__cta plan__cta--ghost">Select Monthly</a>
				</div>

				<div class={['plan plan--featured', selectedPlan === 'quarterly' && 'plan--active']}>
					<div class="plan__ribbon">Most Popular</div>
					<h3 class="plan__name plan__name--lg">Quarterly</h3>
					<div class="plan__price-row">
						<span class="plan__price plan__price--lg">$247</span>
						<span class="plan__period">/qtr</span>
					</div>
					<div class="plan__perday plan__perday--emerald">Save 15% ($2.75 / day)</div>
					<ul class="plan__features plan__features--featured">
						<li class="plan__feat">
							<span class="plan__tick plan__tick--bold">&#x2713;</span>
							<span class="plan__feat-bold">Priority Support</span>
						</li>
						<li class="plan__feat">
							<span class="plan__tick plan__tick--bold">&#x2713;</span> 2-4 Premium Swings / Week
						</li>
						<li class="plan__feat">
							<span class="plan__tick plan__tick--bold">&#x2713;</span> Instant SMS &amp; Email Alerts
						</li>
						<li class="plan__feat">
							<span class="plan__tick plan__tick--bold">&#x2713;</span> Private Discord Community
						</li>
					</ul>
					<a href="/checkout/quarterly-swings" class="plan__cta plan__cta--primary"
						>Join Quarterly</a
					>
				</div>

				<div class={['plan plan--highlight', selectedPlan === 'annual' && 'plan--active']}>
					<h3 class="plan__name">Annual</h3>
					<div class="plan__price-row">
						<span class="plan__price">$927</span>
						<span class="plan__period">/yr</span>
					</div>
					<div class="plan__perday plan__perday--emerald-soft">Save 20% ($2.54 / day)</div>
					<ul class="plan__features">
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span>
							<span class="plan__feat-bold">Strategy Video Library</span>
						</li>
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span> 2-4 Premium Swings / Week
						</li>
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span> Instant SMS &amp; Email Alerts
						</li>
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span> Private Discord Community
						</li>
					</ul>
					<a href="/checkout/annual-swings" class="plan__cta plan__cta--emerald">Select Annual</a>
				</div>
			</div>

			<div class="pricing__footer">
				<p class="pricing__secure-line">Secure checkout powered by Stripe. Cancel anytime.</p>
				<div class="pricing__guarantee">
					<svg class="pricing__guarantee-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
						/></svg
					>
					<span>30-Day Money Back Guarantee. Zero risk to try.</span>
				</div>
			</div>
		</div>
	</section>

	<section class="faq">
		<div class="faq__inner">
			<h2 class="faq__title">Frequently Asked Questions</h2>
			<div class="faq__list">
				{#each faqList as faq, i (i)}
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
			<h2 class="bell-cta__title">
				Stop Overtrading. <br /> Start Swinging.
			</h2>
			<p class="bell-cta__lede">
				Join the trading room that values your time as much as your capital.
			</p>
			<a href="#pricing" class="bell-cta__btn">Get Instant Access</a>
			<p class="bell-cta__fine">Secure Checkout &bull; Cancel Anytime</p>
		</div>
	</section>
</div>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Page-local tokens — swing-trading leans emerald (primary accent),
	   with blue and indigo on supporting cards/stats.
	   ───────────────────────────────────────────────────────────────── */
	.swing-trading {
		--swing-teal-200: #99f6e4;
		--swing-teal-900: #134e4a;

		width: 100%;
		background: var(--rtp-bg);
		color: var(--rtp-text);
		font-family: var(--rtp-font-sans);
	}
	.swing-trading ::selection {
		background: var(--rtp-emerald);
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
		padding-block: 5rem;
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
			linear-gradient(rgba(16, 185, 129, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(16, 185, 129, 0.03) 1px, transparent 1px);
		background-size: 48px 48px;
		opacity: 0.5;
	}
	.hero__halo {
		position: absolute;
		border-radius: 50%;
	}
	.hero__halo--emerald {
		top: 0;
		right: 0;
		width: 600px;
		height: 600px;
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		filter: blur(100px);
		animation: hero-pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	.hero__halo--blue {
		bottom: 0;
		left: 0;
		width: 500px;
		height: 500px;
		background: color-mix(in oklab, var(--rtp-blue) 10%, transparent);
		filter: blur(120px);
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
	}
	@media (min-width: 1024px) {
		.hero__copy {
			text-align: start;
		}
	}

	.hero__pill {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: var(--rtp-surface);
		border: 1px solid color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
		padding: 0.375rem 1rem;
		border-radius: var(--rtp-radius-pill);
		margin-block-end: 2rem;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		cursor: default;
		transition: border-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.hero__pill:hover {
		border-color: color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
	}
	.hero__pill-pulse {
		position: relative;
		display: inline-flex;
		width: 0.625rem;
		height: 0.625rem;
	}
	.hero__pill-ping {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: var(--rtp-emerald-bright);
		opacity: 0.75;
		animation: hero-ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite;
	}
	.hero__pill-dot {
		position: relative;
		display: inline-flex;
		width: 0.625rem;
		height: 0.625rem;
		border-radius: 50%;
		background: var(--rtp-emerald);
	}
	@keyframes hero-ping {
		75%,
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}
	.hero__pill-label {
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.05em;
		text-transform: uppercase;
		color: var(--rtp-emerald-bright);
	}

	.hero__title {
		font-family: var(--rtp-font-display);
		font-size: 3rem;
		font-weight: 800;
		margin-block-end: 1.5rem;
		line-height: 1.1;
		letter-spacing: -0.025em;
	}
	@media (min-width: 768px) {
		.hero__title {
			font-size: 4.5rem;
		}
	}
	.hero__title-accent {
		background: linear-gradient(to right, var(--rtp-emerald), #6ee7b7, var(--swing-teal-200));
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}

	.hero__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
		margin-block-end: 2.5rem;
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
		align-items: center;
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
		width: 100%;
		transition: all 0.2s var(--rtp-ease-out);
	}
	@media (min-width: 640px) {
		.hero__cta {
			width: auto;
		}
	}
	.hero__cta--primary {
		background: var(--rtp-emerald);
		color: #fff;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
	}
	.hero__cta--primary:hover {
		background: #059669;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-emerald) 25%, transparent);
		transform: translateY(-0.25rem);
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
		border-color: color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
	}

	.hero__bullets {
		margin-block-start: 2.5rem;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: center;
		gap: 1.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: color-mix(in oklab, var(--rtp-muted) 60%, transparent);
	}
	@media (min-width: 1024px) {
		.hero__bullets {
			justify-content: flex-start;
		}
	}
	.hero__bullet {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.hero__bullet-icon {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--rtp-emerald);
	}

	/* ── Hero chrome (Alert card) ── */
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
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 600px;
		height: 600px;
		background: linear-gradient(
			to top right,
			color-mix(in oklab, var(--rtp-emerald) 20%, transparent),
			transparent
		);
		border-radius: 50%;
		filter: blur(48px);
	}

	.hero__alert-card {
		position: relative;
		background: color-mix(in oklab, var(--rtp-surface) 90%, transparent);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
		padding: 2rem;
		border-radius: 1.5rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		transform: rotateY(-12deg) rotateX(5deg);
		transition: transform 700ms cubic-bezier(0.25, 0.1, 0.25, 1);
	}
	.hero__alert-card:hover {
		transform: rotateY(0) rotateX(0);
	}

	.hero__alert-head {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-block-end: 2rem;
	}
	.hero__alert-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #fff;
	}
	.hero__alert-sub {
		color: var(--rtp-emerald);
		font-size: 0.875rem;
		font-weight: 700;
	}
	.hero__alert-time {
		background: var(--rtp-bg);
		padding: 0.25rem 0.75rem;
		border-radius: var(--rtp-radius-sm);
		border: 1px solid var(--rtp-border);
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-muted);
	}

	.hero__alert-body {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}
	.hero__alert-row {
		background: var(--rtp-bg);
		padding: 1rem;
		border-radius: var(--rtp-radius-md);
		border: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
	}
	.hero__alert-row--action {
		border: 0;
		border-left: 4px solid var(--rtp-emerald);
	}
	.hero__alert-label {
		font-size: 0.75rem;
		color: var(--rtp-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-block-end: 0.25rem;
	}
	.hero__alert-value {
		font-size: 1.125rem;
		font-weight: 700;
		color: #fff;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.hero__alert-badge {
		background: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		color: var(--rtp-emerald-bright);
		padding: 0 0.5rem;
		border-radius: var(--rtp-radius-sm);
		font-size: 0.875rem;
	}
	.hero__alert-pair {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}
	.hero__alert-stat {
		background: var(--rtp-bg);
		padding: 1rem;
		border-radius: var(--rtp-radius-md);
		border: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
	}
	.hero__alert-num {
		font-size: 1.25rem;
		font-family: var(--rtp-font-mono);
		font-weight: 700;
		color: #fff;
	}
	.hero__alert-num--emerald {
		color: var(--rtp-emerald-bright);
	}
	.hero__alert-stop {
		background: var(--rtp-bg);
		padding: 1rem;
		border-radius: var(--rtp-radius-md);
		border: 1px solid color-mix(in oklab, var(--rtp-red) 30%, transparent);
	}
	.hero__alert-stop-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.hero__alert-stop-val {
		color: var(--rtp-red-soft);
		font-family: var(--rtp-font-mono);
		font-weight: 700;
	}

	.hero__alert-bonus {
		position: absolute;
		right: -1.5rem;
		bottom: -1.5rem;
		background: var(--rtp-emerald);
		color: #fff;
		padding: 1rem;
		border-radius: 1rem;
		box-shadow: 0 20px 25px -5px color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		animation: bounce-slow 3s ease-in-out infinite;
	}
	@keyframes bounce-slow {
		0%,
		100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-0.5rem);
		}
	}
	.hero__alert-bonus-label {
		font-size: 0.75rem;
		font-weight: 700;
		opacity: 0.8;
		text-transform: uppercase;
	}
	.hero__alert-bonus-value {
		font-size: 1.5rem;
		font-weight: 800;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Stats
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
	.stats__label {
		color: var(--rtp-muted);
		font-weight: 500;
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-block-end: 0.5rem;
		transition: color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.stats__cell:hover .stats__label {
		color: var(--rtp-emerald-bright);
	}
	.stats__value {
		font-size: 2.25rem;
		font-weight: 800;
	}
	@media (min-width: 768px) {
		.stats__value {
			font-size: 3rem;
		}
	}
	.stats__value--emerald {
		color: var(--rtp-emerald);
	}
	.stats__value--primary {
		color: var(--rtp-primary);
	}
	.stats__value--indigo {
		color: var(--rtp-indigo);
	}
	.stats__value--blue {
		color: var(--rtp-blue);
	}
	.stats__unit {
		font-size: 1.125rem;
		color: var(--rtp-muted);
		font-weight: 400;
		margin-inline-start: 0.25rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Compare (Scalper vs Swing Trader)
	   ───────────────────────────────────────────────────────────────── */
	.compare {
		padding-block: 6rem;
		background: var(--rtp-bg);
		position: relative;
		overflow: hidden;
	}
	.compare__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.compare__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.compare__inner {
			padding-inline: 2rem;
		}
	}
	.compare__header {
		text-align: center;
		margin-block-end: 4rem;
	}
	.compare__eyebrow {
		color: var(--rtp-emerald);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.875rem;
		margin-block-end: 0.5rem;
		display: block;
	}
	.compare__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.compare__title {
			font-size: 3rem;
		}
	}
	.compare__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
		max-width: 42rem;
		margin-inline: auto;
	}
	.compare__grid {
		display: grid;
		gap: 2rem;
		max-width: 64rem;
		margin-inline: auto;
	}
	@media (min-width: 768px) {
		.compare__grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
	.compare-card {
		padding: 2.5rem;
		border-radius: 1.5rem;
		transition: opacity 0.3s var(--rtp-ease-out);
	}
	.compare-card--bad {
		background: color-mix(in oklab, var(--rtp-surface) 50%, transparent);
		border: 1px solid var(--rtp-border);
		opacity: 0.7;
	}
	.compare-card--bad:hover {
		opacity: 1;
	}
	.compare-card--good {
		background: var(--rtp-surface);
		border: 2px solid var(--rtp-emerald);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		position: relative;
		overflow: hidden;
		transform: none;
	}
	@media (min-width: 768px) {
		.compare-card--good {
			transform: scale(1.05);
		}
	}
	.compare-card__ribbon {
		position: absolute;
		top: 0;
		right: 0;
		background: var(--rtp-emerald);
		color: #fff;
		font-size: 0.75rem;
		font-weight: 700;
		padding: 0.25rem 0.75rem;
		border-bottom-left-radius: var(--rtp-radius-md);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
	}
	.compare-card__head {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-block-end: 1.5rem;
	}
	.compare-card__emoji {
		width: 3rem;
		height: 3rem;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.5rem;
		box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
	}
	.compare-card__emoji--muted {
		background: var(--rtp-border);
	}
	.compare-card__emoji--good {
		background: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		border: 1px solid color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
	}
	.compare-card__title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--rtp-text);
	}
	.compare-card__title--muted {
		color: var(--rtp-muted);
	}
	.compare-card__list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		color: var(--rtp-muted);
	}
	.compare-card__list--bold {
		color: var(--rtp-text);
		font-weight: 500;
	}
	.compare-card__item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}
	.compare-card__x {
		color: var(--rtp-red-soft);
		font-weight: 700;
		font-size: 1.125rem;
	}
	.compare-card__check {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--rtp-emerald);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Process / How It Works
	   ───────────────────────────────────────────────────────────────── */
	.process {
		padding-block: 6rem;
		background: var(--rtp-surface);
		border-block-start: 1px solid var(--rtp-border);
	}
	.process__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.process__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.process__inner {
			padding-inline: 2rem;
		}
	}
	.process__header {
		text-align: center;
		margin-block-end: 4rem;
	}
	.process__eyebrow {
		color: var(--rtp-emerald);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.875rem;
		margin-block-end: 0.5rem;
		display: block;
	}
	.process__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
	}
	@media (min-width: 768px) {
		.process__title {
			font-size: 3rem;
		}
	}
	.process__grid {
		display: grid;
		gap: 2.5rem;
	}
	@media (min-width: 768px) {
		.process__grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
	.pill-card__icon {
		width: 3.5rem;
		height: 3.5rem;
		border-radius: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-block-end: 1.5rem;
		border: 1px solid;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.pill-card__icon svg {
		width: 1.75rem;
		height: 1.75rem;
	}
	.pill-card:hover .pill-card__icon {
		transform: scale(1.1);
	}
	.pill-card--emerald .pill-card__icon {
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		color: var(--rtp-emerald);
	}
	.pill-card--primary .pill-card__icon {
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-primary) 20%, transparent);
		color: var(--rtp-primary);
	}
	.pill-card--indigo .pill-card__icon {
		background: color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-indigo) 20%, transparent);
		color: var(--rtp-indigo);
	}
	.pill-card__title {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.75rem;
	}
	.pill-card__desc {
		color: var(--rtp-muted);
		line-height: 1.625;
		font-size: 0.875rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Recent verified swings (table)
	   ───────────────────────────────────────────────────────────────── */
	.recent {
		padding-block: 6rem;
		background: var(--rtp-bg);
	}
	.recent__inner {
		max-width: 72rem;
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.recent__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.recent__inner {
			padding-inline: 2rem;
		}
	}
	.recent__header {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: flex-end;
		gap: 1rem;
		margin-block-end: 2.5rem;
	}
	@media (min-width: 768px) {
		.recent__header {
			flex-direction: row;
		}
	}
	.recent__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.5rem;
	}
	.recent__lede {
		color: var(--rtp-muted);
	}
	.recent__link {
		color: var(--rtp-emerald);
		font-weight: 700;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.875rem;
		text-decoration: none;
		transition: color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.recent__link:hover {
		color: var(--rtp-emerald-bright);
	}

	.recent__table {
		background: var(--rtp-surface);
		border-radius: 1rem;
		border: 1px solid var(--rtp-border);
		overflow: hidden;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
	}
	.recent__row {
		display: grid;
		grid-template-columns: repeat(12, 1fr);
		padding: 1.25rem 1rem;
		align-items: center;
		font-family: var(--rtp-font-mono);
		font-size: 0.875rem;
		transition: background var(--rtp-dur-base) var(--rtp-ease-out);
		border-block-start: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
	}
	.recent__row:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}
	.recent__row--head {
		background: var(--rtp-surface);
		border-block-end: 1px solid var(--rtp-border);
		border-block-start: 0;
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		color: var(--rtp-muted);
		letter-spacing: 0.05em;
		font-family: var(--rtp-font-sans);
	}
	.recent__row--head:hover {
		background: var(--rtp-surface);
	}
	.recent__row--loss {
		background: color-mix(in oklab, var(--rtp-red) 5%, transparent);
	}
	.recent__col--ticker {
		grid-column: span 3 / span 3;
	}
	.recent__col--type {
		grid-column: span 3 / span 3;
	}
	.recent__col--days {
		grid-column: span 3 / span 3;
	}
	.recent__col--return {
		grid-column: span 3 / span 3;
		text-align: end;
	}
	.recent__col--notes {
		display: none;
	}
	@media (min-width: 768px) {
		.recent__col--ticker,
		.recent__col--type,
		.recent__col--days,
		.recent__col--return {
			grid-column: span 2 / span 2;
		}
		.recent__col--notes {
			display: block;
			grid-column: span 4 / span 4;
			text-align: end;
		}
	}
	.recent__ticker {
		font-weight: 700;
		color: #fff;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.recent__logo {
		width: 1.25rem;
		height: 1.25rem;
		opacity: 0.7;
		display: none;
	}
	@media (min-width: 640px) {
		.recent__logo {
			display: block;
		}
	}
	.recent__type--call {
		color: var(--rtp-emerald-bright);
		font-weight: 700;
	}
	.recent__type--put {
		color: var(--rtp-red-soft);
		font-weight: 700;
	}
	.recent__days {
		color: var(--rtp-muted);
	}
	.recent__return {
		font-weight: 700;
	}
	.recent__return--up {
		color: var(--rtp-emerald-bright);
	}
	.recent__return--down {
		color: var(--rtp-red-soft);
	}
	.recent__notes {
		color: var(--rtp-muted);
		font-size: 0.75rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Pricing
	   ───────────────────────────────────────────────────────────────── */
	.pricing {
		padding-block: 6rem;
		background: var(--rtp-surface);
		border-block-start: 1px solid var(--rtp-border);
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
		color: var(--rtp-emerald);
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
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.pricing__title {
			font-size: 3rem;
		}
	}
	.pricing__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
		max-width: 48rem;
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
		background: var(--rtp-emerald);
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
		opacity: 0.7;
		position: relative;
		transition: all 0.3s;
	}
	.plan:hover {
		opacity: 1;
	}
	.plan--featured {
		padding: 2.5rem;
		border-radius: 1.5rem;
		border-width: 2px;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		z-index: 10;
	}
	.plan--simple.plan--active {
		border-color: var(--rtp-emerald);
		opacity: 1;
		scale: 1.05;
		box-shadow: 0 20px 25px -5px color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
	}
	.plan--featured.plan--active {
		border-color: var(--rtp-emerald);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		opacity: 1;
	}
	@media (min-width: 768px) {
		.plan--featured.plan--active {
			scale: 1.1;
		}
	}
	.plan--highlight {
		background: var(--rtp-surface);
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
		background: var(--rtp-emerald);
		color: #fff;
		padding: 0.25rem 1rem;
		border-radius: var(--rtp-radius-pill);
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
		white-space: nowrap;
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
		background: var(--rtp-bg);
	}
	.plan__features {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-block-end: 2rem;
		font-size: 0.875rem;
		color: var(--rtp-muted);
	}
	.plan__features--featured {
		color: #fff;
	}
	.plan__feat {
		display: flex;
		gap: 0.75rem;
	}
	.plan__tick {
		color: var(--rtp-emerald);
	}
	.plan__tick--bold {
		font-weight: 700;
	}
	.plan__feat-bold {
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
		transition: all var(--rtp-dur-base) var(--rtp-ease-out);
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
		background: var(--rtp-emerald);
		color: #fff;
		border-radius: var(--rtp-radius-md);
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
	}
	.plan__cta--primary:hover {
		background: #059669;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
	}
	.plan__cta--emerald {
		background: var(--rtp-bg);
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
	.pricing__secure-line {
		color: var(--rtp-muted);
		font-size: 0.875rem;
		margin-block-end: 1rem;
	}
	.pricing__guarantee {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		color: var(--rtp-muted);
		font-size: 0.875rem;
		background: var(--rtp-bg);
		padding: 0.5rem 1rem;
		border-radius: var(--rtp-radius-pill);
		border: 1px solid var(--rtp-border);
	}
	.pricing__guarantee-icon {
		width: 1rem;
		height: 1rem;
		color: var(--rtp-emerald);
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
		color: var(--rtp-text);
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
		border-color: color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
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
		color: var(--rtp-text);
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
		color: var(--rtp-emerald);
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
		background: linear-gradient(to bottom right, var(--rtp-emerald), var(--swing-teal-900));
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
		letter-spacing: -0.025em;
	}
	@media (min-width: 768px) {
		.bell-cta__title {
			font-size: 3.75rem;
		}
	}
	.bell-cta__lede {
		font-size: 1.25rem;
		color: #d1fae5;
		max-width: 42rem;
		margin-inline: auto;
		margin-block-end: 2.5rem;
	}
	.bell-cta__btn {
		display: inline-block;
		background: #fff;
		color: #047857;
		padding: 1.25rem 2.5rem;
		border-radius: var(--rtp-radius-md);
		font-weight: 700;
		font-size: 1.125rem;
		text-decoration: none;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		transition: all 0.3s var(--rtp-ease-out);
	}
	.bell-cta__btn:hover {
		background: #ecfdf5;
		transform: translateY(-0.25rem);
	}
	.bell-cta__fine {
		margin-block-start: 1.5rem;
		font-size: 0.875rem;
		color: color-mix(in oklab, #d1fae5 70%, transparent);
	}
</style>
