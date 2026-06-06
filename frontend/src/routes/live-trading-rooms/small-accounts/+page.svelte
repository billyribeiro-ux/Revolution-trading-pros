<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { browser } from '$app/environment';

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
					// Only set initial hidden state for elements NOT yet in viewport
					const elements = document.querySelectorAll('[data-gsap]');
					elements.forEach((el) => {
						const rect = el.getBoundingClientRect();
						const isInViewport = rect.top < window.innerHeight * 0.85;
						if (!isInViewport) {
							gsap.set(el, { opacity: 0, y: 30 });
						}
					});

					ScrollTrigger.batch('[data-gsap]', {
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

					ScrollTrigger.refresh();
				});
			} catch (error) {
				console.error('[SmallAccounts] GSAP initialization failed:', error);
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
			question: 'Do I need $25,000 to join this room?',
			answer:
				'No. This room is specifically designed for accounts under $25k. We teach you how to use Cash Accounts (which settle T+1 for options) to trade daily without triggering the Pattern Day Trader (PDT) rule.'
		},
		{
			question: 'What is the minimum recommended capital?',
			answer:
				'We strictly recommend a minimum of $2,000. While you can technically start with less, $2,000 allows you to trade micro-contracts or single option contracts while adhering to proper risk management rules (risking only 1-2% per trade).'
		},
		{
			question: 'How do you avoid the PDT rule?',
			answer:
				'By using a Cash Account rather than a Margin Account. In a Cash Account, options trades settle the next business day (T+1). This means you can trade your entire account balance every single day without restriction, as long as the cash has settled.'
		},
		{
			question: 'What specific instruments do you trade?',
			answer:
				'We focus on SPX (S&P 500 Index) 0DTE options. SPX offers favorable tax treatment (Section 1256), cash settlement (no assignment risk), and massive liquidity, making it ideal for scalping small accounts.'
		},
		{
			question: 'Is this room beginner friendly?',
			answer:
				'Yes. We assume you are learning. Our "New Member" onboarding covers how to set up your platform, how to read an options chain, and how to execute orders quickly. We focus heavily on risk habits.'
		},
		{
			question: 'Do you trade Futures (ES/NQ)?',
			answer:
				'Yes. For members who prefer Futures, we analyze ES and NQ. Futures accounts also do not have the PDT rule, making them another excellent vehicle for small account growth.'
		},
		{
			question: 'What is the win rate?',
			answer:
				'Win rates fluctuate with market conditions. Our strategy targets a 70%+ win rate with a 1:2 risk/reward ratio. However, in small account trading, risk management (keeping losers small) is more important than win rate.'
		},
		{
			question: 'Can I trade on mobile?',
			answer:
				"Yes, but we recommend a desktop or laptop for the best experience. You can listen to the live voice commentary and see the screen share on the Discord mobile app while executing trades on your broker's app."
		},
		{
			question: 'How many trades do you take per day?',
			answer:
				'Quality over quantity. We typically take 2-4 high-conviction trades per day, usually within the first 2 hours of the market open (9:30 AM - 11:30 AM EST).'
		},
		{
			question: 'What happens if I blow up my account?',
			answer:
				'We aim to prevent this. Our strict risk rules (hard stops) are designed to keep you in the game. If you are struggling, we offer mentorship channels to review your trades and correct bad habits before capital is lost.'
		}
	];

	// SEO schemas emitted via +page.ts (page.data.seo).
</script>

<div class="small-accounts">
	<section class="hero">
		<div class="hero__bg" aria-hidden="true">
			<div class="hero__bg-grid"></div>
			<div class="hero__halo hero__halo--primary"></div>
			<div class="hero__halo hero__halo--indigo"></div>
		</div>

		<div class="hero__inner">
			<div class="hero__copy">
				<div data-gsap class="hero__pill">
					<span class="hero__pill-pulse">
						<span class="hero__pill-ping"></span>
						<span class="hero__pill-dot"></span>
					</span>
					<span class="hero__pill-label">Market Active &bull; Live Commentary On</span>
				</div>

				<h1 data-gsap class="hero__title">
					Master <span class="hero__title-accent">0DTE Options</span> <br />Without The PDT Rule.
				</h1>

				<p data-gsap class="hero__lede">
					Don't just get alerts—learn the execution. Join our live voice &amp; screen-share room
					where we hunt high-probability setups on SPX optimized for <strong
						>accounts under $25,000.</strong
					>
				</p>

				<div data-gsap class="hero__cta-row">
					<a href="#pricing" class="hero__cta hero__cta--primary">
						Join the Small Account Room
						<svg
							aria-hidden="true"
							class="hero__cta-arrow"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 7l5 5m0 0l-5 5m5-5H6"
							/></svg
						>
					</a>
					<a href="#daily-routine" class="hero__cta hero__cta--ghost"> See Daily Routine </a>
				</div>

				<div data-gsap class="hero__social">
					<div class="hero__avatars">
						<div class="hero__avatar hero__avatar--img hero__avatar--trader-one"></div>
						<div class="hero__avatar hero__avatar--img hero__avatar--trader-two"></div>
						<div class="hero__avatar hero__avatar--more">+500</div>
					</div>
					<p>Traders currently active</p>
				</div>
			</div>

			<div class="hero__chrome">
				<div class="hero__chrome-glow" aria-hidden="true"></div>
				<div class="hero__chrome-tilt">
					<div class="hero__chrome-card">
						<div class="hero__chrome-head">
							<div class="hero__chrome-brand">
								<div class="hero__chrome-icon">
									<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
										><path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M13 10V3L4 14h7v7l9-11h-7z"
										/></svg
									>
								</div>
								<div>
									<div class="hero__chrome-title">Small Account Mastery</div>
									<div class="hero__chrome-live">&bull; Live Trading Room</div>
								</div>
							</div>
							<div class="hero__chrome-time">10:32:45 EST</div>
						</div>

						<div class="hero__chrome-body">
							<div class="hero__signal hero__signal--new">
								<div class="hero__signal-meta">
									<span class="hero__signal-tag">New Signal</span>
									<span class="hero__signal-time">Just now</span>
								</div>
								<div class="hero__signal-line">
									BTO <span class="hero__signal-emph">SPX 4580 CALL</span> @ $3.50
								</div>
								<div class="hero__signal-stops">
									<span>&#x1f6d1; Stop: $2.10</span>
									<span>&#x1f3af; Target: $5.00+</span>
								</div>
							</div>
							<div class="hero__signal hero__signal--update">
								<div class="hero__signal-meta">
									<span class="hero__signal-tag hero__signal-tag--update">Update</span>
									<span class="hero__signal-time">15m ago</span>
								</div>
								<div class="hero__signal-text">
									Approaching VWAP support. Watching for bounce to add to runners.
								</div>
							</div>
						</div>

						<div class="hero__chrome-bubble">
							<span class="hero__chrome-bubble-emoji">&#x1f680;</span>
							<div>
								<div class="hero__chrome-bubble-label">Last Trade</div>
								<div class="hero__chrome-bubble-value">+85% Profit</div>
							</div>
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
				<div class="stats__cell stats__cell--divided group">
					<div class="stats__value stats__value--emerald">9:30AM</div>
					<div class="stats__label">Live Bell-to-Bell</div>
				</div>
				<div class="stats__cell stats__cell--divided group">
					<div class="stats__value stats__value--indigo">1000+</div>
					<div class="stats__label">Setups Called</div>
				</div>
				<div class="stats__cell stats__cell--divided group">
					<div class="stats__value stats__value--blue">24/7</div>
					<div class="stats__label">Discord Access</div>
				</div>
			</div>
		</div>
	</section>

	<section class="methodology">
		<div class="methodology__inner">
			<div class="methodology__grid">
				<div>
					<span class="methodology__eyebrow">The Methodology</span>
					<h2 data-gsap class="methodology__title">How We Trade Under $25k</h2>
					<p class="methodology__lede">
						The Pattern Day Trader (PDT) rule restricts margin accounts under $25k to only 3 day
						trades per week. <strong>We solve this.</strong>
					</p>
					<ul class="methodology__list">
						<li class="methodology__item">
							<div class="methodology__num">
								<span>01</span>
							</div>
							<div>
								<h3 class="methodology__item-title">Cash Account Mastery</h3>
								<p class="methodology__item-desc">
									We teach you how to utilize Cash Accounts. Options settle T+1 (next day), meaning
									you can reuse your cash daily without hitting PDT limits.
								</p>
							</div>
						</li>
						<li class="methodology__item">
							<div class="methodology__num">
								<span>02</span>
							</div>
							<div>
								<h3 class="methodology__item-title">SPX 0DTE Leverage</h3>
								<p class="methodology__item-desc">
									The S&P 500 index offers massive liquidity. We scalp quick moves (10-30 minutes)
									using 0DTE options that offer high percentage returns on small capital.
								</p>
							</div>
						</li>
						<li class="methodology__item">
							<div class="methodology__num">
								<span>03</span>
							</div>
							<div>
								<h3 class="methodology__item-title">Strict Position Sizing</h3>
								<p class="methodology__item-desc">
									We never bet the farm. We teach you to risk strictly 1-2% of your account per
									trade, ensuring you survive the learning curve.
								</p>
							</div>
						</li>
					</ul>
				</div>
				<div class="methodology__media">
					<div class="methodology__media-glow" aria-hidden="true"></div>
					<div class="methodology__media-card">
						[Image of trading chart with support and resistance lines]
						<div class="methodology__media-stats">
							<div>
								<div class="methodology__media-label">Account Balance</div>
								<div class="methodology__media-value">$2,450.00</div>
							</div>
							<div class="methodology__media-rhs">
								<div class="methodology__media-label">Today's P&amp;L</div>
								<div class="methodology__media-value methodology__media-value--up">
									+$185.00 (+7.5%)
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="inside-the-room" class="inside">
		<div class="inside__inner">
			<div class="inside__header">
				<h2 data-gsap class="inside__title">Inside The Room</h2>
				<p data-gsap class="inside__lede">
					We don't just give signals. We teach you how to fish with institutional-grade tools.
				</p>
			</div>

			<div class="inside__grid">
				<div data-gsap class="inside-card inside-card--primary group">
					<div class="inside-card__icon">
						<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
							/></svg
						>
					</div>
					<h3 class="inside-card__title">Live Screen Sharing</h3>
					<p class="inside-card__desc">
						Watch our charts in real-time. See exactly where we draw support, resistance, and supply
						zones before the trade happens. No "after the fact" hindsight.
					</p>
				</div>

				<div data-gsap class="inside-card inside-card--emerald group">
					<div class="inside-card__icon">
						<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"
							/></svg
						>
					</div>
					<h3 class="inside-card__title">Instant Push Alerts</h3>
					<p class="inside-card__desc">
						Can't be at your desk? Get instant notifications via our mobile app for every Entry,
						Trim, and Exit. Trade from anywhere with confidence.
					</p>
				</div>

				<div data-gsap class="inside-card inside-card--indigo group">
					<div class="inside-card__icon">
						<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
							/></svg
						>
					</div>
					<h3 class="inside-card__title">Institutional Flow</h3>
					<p class="inside-card__desc">
						We track Dark Pool data and Gamma Exposure (GEX) levels to understand where the "Smart
						Money" is positioning. We don't guess; we follow the volume.
					</p>
				</div>
			</div>
		</div>
	</section>

	<section id="daily-routine" class="routine">
		<div class="routine__inner">
			<div class="routine__header">
				<span class="routine__eyebrow">Structure</span>
				<h2 data-gsap class="routine__title">Your Daily Routine</h2>
				<p class="routine__lede">Consistency is the key to longevity. Here is the plan.</p>
			</div>

			<div class="timeline">
				<div data-gsap class="timeline__row group">
					<div class="timeline__pin timeline__pin--primary">
						<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
					</div>
					<div class="timeline__card">
						<div class="timeline__card-head">
							<span class="timeline__time timeline__time--primary">8:30 AM EST</span>
							<span class="timeline__chip timeline__chip--primary">PRE-MARKET</span>
						</div>
						<h3 class="timeline__card-title">Game Plan &amp; Levels</h3>
						<p class="timeline__card-desc">
							We review overnight futures, check the economic calendar (CPI, FOMC), and map out key
							Support/Resistance zones on SPX.
						</p>
					</div>
				</div>

				<div data-gsap class="timeline__row group">
					<div class="timeline__pin timeline__pin--emerald">
						<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 10V3L4 14h7v7l9-11h-7z"
							/></svg
						>
					</div>
					<div class="timeline__card timeline__card--featured">
						<div class="timeline__card-head">
							<span class="timeline__time timeline__time--emerald">9:30 AM EST</span>
							<span class="timeline__chip timeline__chip--emerald">MARKET OPEN</span>
						</div>
						<h3 class="timeline__card-title">The Opening Bell</h3>
						<p class="timeline__card-desc">
							100% Focus. We look for the Opening Range Breakout or Rejection. Voice commentary is
							live and rapid-fire.
						</p>
					</div>
				</div>

				<div data-gsap class="timeline__row group">
					<div class="timeline__pin timeline__pin--indigo">
						<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
							/></svg
						>
					</div>
					<div class="timeline__card">
						<div class="timeline__card-head">
							<span class="timeline__time timeline__time--indigo">11:00 AM - 2:00 PM</span>
							<span class="timeline__chip timeline__chip--indigo">EDUCATION</span>
						</div>
						<h3 class="timeline__card-title">Review &amp; Analysis</h3>
						<p class="timeline__card-desc">
							Volume slows. We review morning trades, answer member Q&amp;A, and teach risk
							management strategies.
						</p>
					</div>
				</div>

				<div data-gsap class="timeline__row group">
					<div class="timeline__pin timeline__pin--blue">
						<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
							/></svg
						>
					</div>
					<div class="timeline__card">
						<div class="timeline__card-head">
							<span class="timeline__time timeline__time--blue">3:00 PM EST</span>
							<span class="timeline__chip timeline__chip--blue">THE CLOSE</span>
						</div>
						<h3 class="timeline__card-title">Power Hour</h3>
						<p class="timeline__card-desc">
							Institutional volume returns. We look for End-of-Day squeezes or hedge our positions
							into the close.
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
				<h2 class="pricing__title">Simple Membership</h2>
				<p class="pricing__lede">
					Invest in your education. One good trade covers your monthly access.
				</p>
			</div>

			<div class="pricing__toggle-row">
				<div class="pricing-toggle">
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
						class={['pricing-toggle__indicator', `pricing-toggle__indicator--${selectedPlan}`]}
					></div>
				</div>
			</div>

			<div class="pricing__grid">
				<div class={['plan plan--simple', selectedPlan === 'monthly' && 'plan--active']}>
					<h3 class="plan__name">Monthly</h3>
					<div class="plan__price-row">
						<span class="plan__price">$197</span>
						<span class="plan__period">/mo</span>
					</div>
					<div class="plan__perday">$9.85 / trading day</div>
					<ul class="plan__features">
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span> Cash-account execution plan
						</li>
						<li class="plan__feat"><span class="plan__tick">&#x2713;</span> PDT-free SPX setups</li>
						<li class="plan__feat">
							<span class="plan__tick">&#x2713;</span> Risk-per-trade guardrails
						</li>
						<li class="plan__feat"><span class="plan__tick">&#x2713;</span> Small account guide</li>
					</ul>
					<a href="/checkout/monthly-small-accounts" class="plan__cta plan__cta--ghost"
						>Select Monthly</a
					>
				</div>

				<div class={['plan plan--featured', selectedPlan === 'quarterly' && 'plan--active']}>
					<div class="plan__ribbon">Most Popular</div>
					<h3 class="plan__name plan__name--lg">Quarterly</h3>
					<div class="plan__price-row">
						<span class="plan__price plan__price--lg">$497</span>
						<span class="plan__period">/qtr</span>
					</div>
					<div class="plan__perday plan__perday--emerald">Save 15% ($8.20 / trading day)</div>
					<ul class="plan__features plan__features--featured">
						<li class="plan__feat">
							<span class="plan__tick plan__tick--bold">&#x2713;</span>
							<span class="plan__feat-bold">Priority Support</span>
						</li>
						<li class="plan__feat">
							<span class="plan__tick plan__tick--bold">&#x2713;</span> Cash-account execution plan
						</li>
						<li class="plan__feat">
							<span class="plan__tick plan__tick--bold">&#x2713;</span> PDT-free SPX setups
						</li>
						<li class="plan__feat">
							<span class="plan__tick plan__tick--bold">&#x2713;</span> Small account guide
						</li>
					</ul>
					<a href="/checkout/quarterly-small-accounts" class="plan__cta plan__cta--primary"
						>Join Quarterly</a
					>
				</div>

				<div class={['plan plan--highlight', selectedPlan === 'annual' && 'plan--active']}>
					<h3 class="plan__name">Annual</h3>
					<div class="plan__price-row">
						<span class="plan__price">$1,647</span>
						<span class="plan__period">/yr</span>
					</div>
					<div class="plan__perday plan__perday--indigo">Save 30% ($6.50 / trading day)</div>
					<ul class="plan__features">
						<li class="plan__feat">
							<span class="plan__tick plan__tick--indigo">&#x2713;</span>
							<span class="plan__feat-bold">1-on-1 Coaching Call</span>
						</li>
						<li class="plan__feat">
							<span class="plan__tick plan__tick--indigo">&#x2713;</span> Cash-account execution plan
						</li>
						<li class="plan__feat">
							<span class="plan__tick plan__tick--indigo">&#x2713;</span> PDT-free SPX setups
						</li>
						<li class="plan__feat">
							<span class="plan__tick plan__tick--indigo">&#x2713;</span> Account growth review
						</li>
					</ul>
					<a href="/checkout/annual-small-accounts" class="plan__cta plan__cta--indigo"
						>Select Annual</a
					>
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
								class={['faq__chevron', { 'faq__chevron--open': openFaq === i }]}
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
			<h2 class="bell-cta__title">Market Opens at 9:30 AM ET.</h2>
			<p class="bell-cta__lede">
				Don't trade a small account with large-account habits. Join a room built around PDT-free
				execution, tight risk, and repeatable account growth.
			</p>
			<a href="#pricing" class="bell-cta__btn">Secure Your Spot</a>
			<p class="bell-cta__fine">30-Day Money Back Guarantee &bull; Cancel Anytime</p>
		</div>
	</section>
</div>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Page-local tokens — small-accounts leans on primary blue + emerald
	   accents (mirrors small account / market open palette).
	   ───────────────────────────────────────────────────────────────── */
	.small-accounts {
		--sa-indigo-900: #312e81;

		width: 100%;
		background: var(--rtp-bg);
		color: var(--rtp-text);
		font-family: var(--rtp-font-sans);
	}
	.small-accounts ::selection {
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
			linear-gradient(rgba(255, 255, 255, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(255, 255, 255, 0.03) 1px, transparent 1px);
		background-size: 40px 40px;
		mask-image: radial-gradient(ellipse 60% 50% at 50% 0%, #000 70%, transparent 100%);
	}
	.hero__halo {
		position: absolute;
		border-radius: 50%;
	}
	.hero__halo--primary {
		top: 0;
		right: 0;
		width: 600px;
		height: 600px;
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
		filter: blur(120px);
		animation: hero-pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	.hero__halo--indigo {
		bottom: 0;
		left: 0;
		width: 500px;
		height: 500px;
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
		gap: 3rem;
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
		border: 1px solid var(--rtp-border);
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
		letter-spacing: 0.025em;
		text-transform: uppercase;
		color: var(--rtp-emerald-bright);
	}

	.hero__title {
		font-family: var(--rtp-font-display);
		font-size: 2.25rem;
		font-weight: 800;
		margin-block-end: 1.5rem;
		line-height: 1.1;
		letter-spacing: -0.025em;
	}
	@media (min-width: 768px) {
		.hero__title {
			font-size: 3.75rem;
		}
	}
	.hero__title-accent {
		background: linear-gradient(to right, var(--rtp-primary), var(--rtp-emerald-bright));
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}

	.hero__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
		margin-block-end: 2rem;
		max-width: 36rem;
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
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem 2rem;
		border-radius: var(--rtp-radius-md);
		font-weight: 700;
		font-size: 1.125rem;
		text-decoration: none;
		transition: all 0.3s var(--rtp-ease-out);
	}
	.hero__cta--primary {
		background: var(--rtp-primary);
		color: #fff;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-primary) 25%, transparent);
	}
	.hero__cta--primary:hover {
		background: #2563eb;
		transform: translateY(-0.25rem);
	}
	.hero__cta-arrow {
		width: 1.25rem;
		height: 1.25rem;
	}
	.hero__cta--ghost {
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border);
		color: var(--rtp-text);
	}
	.hero__cta--ghost:hover {
		background: var(--rtp-bg);
		border-color: color-mix(in oklab, var(--rtp-primary) 30%, transparent);
	}

	.hero__social {
		margin-block-start: 2rem;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		font-size: 0.875rem;
		color: var(--rtp-muted);
		font-weight: 500;
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
		margin-inline-start: -0.5rem;
	}
	.hero__avatar {
		width: 2rem;
		height: 2rem;
		border-radius: 50%;
		border: 2px solid var(--rtp-bg);
	}
	.hero__avatar--img {
		background-color: #1f2937;
		background-size: cover;
	}
	.hero__avatar--trader-one {
		background-image: url('/avatars/1.svg');
	}
	.hero__avatar--trader-two {
		background-image: url('/avatars/2.svg');
	}
	.hero__avatar--more {
		background: var(--rtp-primary);
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		font-size: 0.75rem;
		font-weight: 700;
	}

	/* ── Hero chrome — small-account-mastery card ── */
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
		transform: translateY(2.5rem);
	}
	.hero__chrome-tilt {
		position: relative;
		transform: rotateY(-5deg);
		transition: transform 700ms cubic-bezier(0.23, 1, 0.32, 1);
	}
	.hero__chrome-tilt:hover {
		transform: rotateY(0);
	}
	.hero__chrome-card {
		position: relative;
		background: color-mix(in oklab, var(--rtp-surface) 80%, transparent);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
		border-radius: 1.5rem;
		padding: 1.5rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		transform: rotateY(-10deg) rotateX(5deg);
		transition: transform 700ms var(--rtp-ease-out);
	}
	.hero__chrome-card:hover {
		transform: rotateY(0) rotateX(0);
	}
	.hero__chrome-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: 1.5rem;
		border-block-end: 1px solid color-mix(in oklab, var(--rtp-border) 30%, transparent);
		padding-block-end: 1rem;
	}
	.hero__chrome-brand {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}
	.hero__chrome-icon {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		background: linear-gradient(to bottom right, var(--rtp-primary), var(--rtp-blue));
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		font-weight: 700;
		box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
	}
	.hero__chrome-icon svg {
		width: 1.5rem;
		height: 1.5rem;
	}
	.hero__chrome-title {
		font-weight: 700;
		color: var(--rtp-text);
	}
	.hero__chrome-live {
		font-size: 0.75rem;
		color: var(--rtp-emerald);
	}
	.hero__chrome-time {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-muted);
		background: var(--rtp-bg);
		padding: 0.25rem 0.5rem;
		border-radius: var(--rtp-radius-sm);
	}

	.hero__chrome-body {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.hero__signal {
		background: color-mix(in oklab, var(--rtp-bg) 50%, transparent);
		padding: 1rem;
		border-radius: var(--rtp-radius-md);
	}
	.hero__signal--new {
		border-left: 4px solid var(--rtp-emerald);
	}
	.hero__signal--update {
		border-left: 4px solid var(--rtp-blue);
		opacity: 0.6;
	}
	.hero__signal-meta {
		display: flex;
		justify-content: space-between;
		font-size: 0.75rem;
		margin-block-end: 0.5rem;
	}
	.hero__signal-tag {
		color: var(--rtp-emerald);
		font-weight: 700;
		text-transform: uppercase;
	}
	.hero__signal-tag--update {
		color: var(--rtp-blue);
	}
	.hero__signal-time {
		color: var(--rtp-muted);
	}
	.hero__signal-line {
		font-size: 0.875rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-text);
		margin-block-end: 0.25rem;
	}
	.hero__signal-emph {
		font-weight: 700;
		color: #fff;
	}
	.hero__signal-stops {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: var(--rtp-muted);
	}
	.hero__signal-text {
		font-size: 0.875rem;
		color: var(--rtp-text);
	}

	.hero__chrome-bubble {
		position: absolute;
		bottom: -1.5rem;
		right: -1.5rem;
		background: #fff;
		color: var(--rtp-bg);
		padding: 0.75rem 1.5rem;
		border-radius: var(--rtp-radius-md);
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
		font-weight: 700;
		border: 2px solid var(--rtp-bg);
		display: flex;
		align-items: center;
		gap: 0.5rem;
		animation: bounce 1.5s ease-in-out infinite;
	}
	@keyframes bounce {
		0%,
		100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-0.5rem);
		}
	}
	.hero__chrome-bubble-emoji {
		font-size: 1.5rem;
	}
	.hero__chrome-bubble-label {
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		opacity: 0.7;
	}
	.hero__chrome-bubble-value {
		color: #059669;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Stats
	   ───────────────────────────────────────────────────────────────── */
	.stats {
		background: var(--rtp-surface);
		border-block: 1px solid var(--rtp-border);
		padding-block: 3rem;
	}
	.stats__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
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
		.stats__cell--divided {
			border-inline-start: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
		}
	}
	.stats__cell {
		text-align: center;
		cursor: default;
	}
	.stats__value {
		font-size: 1.875rem;
		font-weight: 700;
		margin-block-end: 0.5rem;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	@media (min-width: 768px) {
		.stats__value {
			font-size: 2.25rem;
		}
	}
	.stats__cell:hover .stats__value {
		transform: scale(1.1);
	}
	.stats__value--primary {
		color: var(--rtp-primary);
	}
	.stats__value--emerald {
		color: var(--rtp-emerald);
	}
	.stats__value--indigo {
		color: var(--rtp-indigo);
	}
	.stats__value--blue {
		color: var(--rtp-blue);
	}
	.stats__label {
		color: var(--rtp-muted);
		font-weight: 500;
		text-transform: uppercase;
		font-size: 0.75rem;
		letter-spacing: 0.05em;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Methodology
	   ───────────────────────────────────────────────────────────────── */
	.methodology {
		padding-block: 6rem;
		background: var(--rtp-bg);
	}
	.methodology__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.methodology__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.methodology__inner {
			padding-inline: 2rem;
		}
	}
	.methodology__grid {
		display: grid;
		gap: 4rem;
		align-items: center;
	}
	@media (min-width: 1024px) {
		.methodology__grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
	.methodology__eyebrow {
		color: var(--rtp-emerald);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.875rem;
		margin-block-end: 0.5rem;
		display: block;
	}
	.methodology__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.methodology__title {
			font-size: 3rem;
		}
	}
	.methodology__lede {
		font-size: 1.125rem;
		color: var(--rtp-muted);
		margin-block-end: 1.5rem;
		line-height: 1.625;
	}
	.methodology__list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}
	.methodology__item {
		display: flex;
		gap: 1rem;
	}
	.methodology__num {
		width: 3rem;
		height: 3rem;
		border-radius: 50%;
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}
	.methodology__num span {
		color: var(--rtp-emerald);
		font-weight: 700;
	}
	.methodology__item-title {
		color: #fff;
		font-weight: 700;
		font-size: 1.125rem;
	}
	.methodology__item-desc {
		color: var(--rtp-muted);
		font-size: 0.875rem;
		margin-block-start: 0.25rem;
	}

	.methodology__media {
		position: relative;
	}
	.methodology__media-glow {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to right,
			color-mix(in oklab, var(--rtp-emerald) 20%, transparent),
			color-mix(in oklab, var(--rtp-blue) 20%, transparent)
		);
		border-radius: 1.5rem;
		filter: blur(32px);
	}
	.methodology__media-card {
		position: relative;
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border);
		border-radius: 1rem;
		padding: 2rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		color: var(--rtp-muted);
	}
	.methodology__media-stats {
		margin-block-start: 1rem;
		background: var(--rtp-bg);
		padding: 1rem;
		border-radius: var(--rtp-radius-md);
		border: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.methodology__media-rhs {
		text-align: end;
	}
	.methodology__media-label {
		font-size: 0.75rem;
		color: var(--rtp-muted);
		text-transform: uppercase;
	}
	.methodology__media-value {
		font-size: 1.25rem;
		font-family: var(--rtp-font-mono);
		color: #fff;
	}
	.methodology__media-value--up {
		color: var(--rtp-emerald-bright);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Inside The Room
	   ───────────────────────────────────────────────────────────────── */
	.inside {
		padding-block: 6rem;
		background: var(--rtp-surface);
		border-block: 1px solid var(--rtp-border);
	}
	.inside__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.inside__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.inside__inner {
			padding-inline: 2rem;
		}
	}
	.inside__header {
		text-align: center;
		margin-block-end: 4rem;
	}
	.inside__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.inside__title {
			font-size: 3rem;
		}
	}
	.inside__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
		max-width: 48rem;
		margin-inline: auto;
	}
	.inside__grid {
		display: grid;
		gap: 2rem;
	}
	@media (min-width: 768px) {
		.inside__grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
	@media (min-width: 1024px) {
		.inside__grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
	.inside-card {
		background: var(--rtp-bg);
		padding: 2rem;
		border-radius: 1rem;
		box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
		border: 1px solid var(--rtp-border);
		transition:
			border-color var(--rtp-dur-base) var(--rtp-ease-out),
			transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.inside-card:hover {
		transform: translateY(-0.25rem);
	}
	.inside-card--primary:hover {
		border-color: var(--rtp-primary);
	}
	.inside-card--emerald:hover {
		border-color: var(--rtp-emerald);
	}
	.inside-card--indigo:hover {
		border-color: var(--rtp-indigo);
	}
	.inside-card__icon {
		width: 3rem;
		height: 3rem;
		border-radius: var(--rtp-radius-md);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-block-end: 1.5rem;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.inside-card__icon svg {
		width: 1.5rem;
		height: 1.5rem;
	}
	.inside-card:hover .inside-card__icon {
		transform: scale(1.1);
	}
	.inside-card--primary .inside-card__icon {
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
		color: var(--rtp-primary);
	}
	.inside-card--emerald .inside-card__icon {
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		color: var(--rtp-emerald);
	}
	.inside-card--indigo .inside-card__icon {
		background: color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
		color: var(--rtp-indigo);
	}
	.inside-card__title {
		font-family: var(--rtp-font-display);
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.75rem;
	}
	.inside-card__desc {
		color: var(--rtp-muted);
		line-height: 1.625;
		margin-block-end: 1.5rem;
		font-size: 0.875rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Routine (Timeline)
	   ───────────────────────────────────────────────────────────────── */
	.routine {
		padding-block: 6rem;
		background: var(--rtp-bg);
		border-block: 1px solid var(--rtp-border);
	}
	.routine__inner {
		max-width: 64rem;
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.routine__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.routine__inner {
			padding-inline: 2rem;
		}
	}
	.routine__header {
		text-align: center;
		margin-block-end: 4rem;
	}
	.routine__eyebrow {
		color: var(--rtp-primary);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.875rem;
		margin-block-end: 0.5rem;
		display: block;
	}
	.routine__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1rem;
	}
	@media (min-width: 768px) {
		.routine__title {
			font-size: 3rem;
		}
	}
	.routine__lede {
		font-size: 1.25rem;
		color: var(--rtp-muted);
	}

	.timeline {
		display: flex;
		flex-direction: column;
		gap: 2rem;
		position: relative;
	}
	.timeline::before {
		content: '';
		position: absolute;
		inset: 0;
		margin-inline-start: 1.25rem;
		transform: translateX(-1px);
		height: 100%;
		width: 2px;
		background: linear-gradient(to bottom, transparent, var(--rtp-border), transparent);
	}
	@media (min-width: 768px) {
		.timeline::before {
			margin-inline: auto;
			transform: translateX(0);
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
		flex-shrink: 0;
		background: var(--rtp-surface);
		box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
		z-index: 10;
		transition: all var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.timeline__pin svg {
		width: 1.25rem;
		height: 1.25rem;
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
		border: 1px solid var(--rtp-primary);
		color: var(--rtp-primary);
	}
	.timeline__row:hover .timeline__pin--primary {
		background: var(--rtp-primary);
		color: #fff;
	}
	.timeline__pin--emerald {
		border: 1px solid var(--rtp-emerald);
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		color: var(--rtp-emerald);
	}
	.timeline__row:hover .timeline__pin--emerald {
		background: var(--rtp-emerald);
		color: #fff;
	}
	.timeline__pin--indigo {
		border: 1px solid var(--rtp-indigo);
		color: var(--rtp-indigo);
	}
	.timeline__row:hover .timeline__pin--indigo {
		background: var(--rtp-indigo);
		color: #fff;
	}
	.timeline__pin--blue {
		border: 1px solid var(--rtp-blue);
		color: var(--rtp-blue);
	}
	.timeline__row:hover .timeline__pin--blue {
		background: var(--rtp-blue);
		color: #fff;
	}

	.timeline__card {
		width: calc(100% - 4rem);
		padding: 1.5rem;
		background: var(--rtp-surface);
		border-radius: var(--rtp-radius-md);
		border: 1px solid var(--rtp-border);
		box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
	}
	@media (min-width: 768px) {
		.timeline__card {
			width: calc(50% - 2.5rem);
		}
	}
	.timeline__card--featured {
		background: var(--rtp-bg);
		border-left: 4px solid var(--rtp-emerald);
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
	}
	.timeline__card-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: 0.5rem;
	}
	.timeline__time {
		font-weight: 700;
	}
	.timeline__time--primary {
		color: var(--rtp-primary);
	}
	.timeline__time--emerald {
		color: var(--rtp-emerald);
	}
	.timeline__time--indigo {
		color: var(--rtp-indigo);
	}
	.timeline__time--blue {
		color: var(--rtp-blue);
	}
	.timeline__chip {
		font-size: 0.75rem;
		padding: 0.125rem 0.5rem;
		border-radius: var(--rtp-radius-sm);
		font-weight: 700;
	}
	.timeline__chip--primary {
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
		color: var(--rtp-primary);
	}
	.timeline__chip--emerald {
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		color: var(--rtp-emerald);
	}
	.timeline__chip--indigo {
		background: color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
		color: var(--rtp-indigo);
	}
	.timeline__chip--blue {
		background: color-mix(in oklab, var(--rtp-blue) 10%, transparent);
		color: var(--rtp-blue);
	}
	.timeline__card-title {
		font-weight: 700;
		color: var(--rtp-text);
	}
	.timeline__card-desc {
		font-size: 0.875rem;
		color: var(--rtp-muted);
		margin-block-start: 0.5rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Pricing
	   ───────────────────────────────────────────────────────────────── */
	.pricing {
		padding-block: 6rem;
		background: var(--rtp-bg);
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
		background: var(--rtp-surface);
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
		width: calc(33.33% - 0.4rem);
		background: var(--rtp-primary);
		border-radius: var(--rtp-radius-sm);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
		transition: all 300ms cubic-bezier(0.25, 0.1, 0.25, 1);
	}
	.pricing-toggle__indicator--monthly {
		left: 0.375rem;
	}
	.pricing-toggle__indicator--quarterly {
		left: calc(33.33% + 0.2rem);
	}
	.pricing-toggle__indicator--annual {
		left: calc(66.66% + 0.1rem);
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
		background: var(--rtp-surface);
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
		background: var(--rtp-bg);
		padding: 2.5rem;
		border-radius: 1.5rem;
		border-width: 2px;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		z-index: 10;
	}
	.plan--simple.plan--active {
		border-color: var(--rtp-primary);
		opacity: 1;
		scale: 1.05;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-primary) 10%, transparent);
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
	.plan--highlight.plan--active {
		border-color: var(--rtp-indigo);
		opacity: 1;
		scale: 1.05;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
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
		background: var(--rtp-bg);
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
	.plan__perday--indigo {
		color: var(--rtp-indigo);
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
		color: var(--rtp-primary);
	}
	.plan__tick--bold {
		font-weight: 700;
		color: var(--rtp-emerald);
	}
	.plan__tick--indigo {
		color: var(--rtp-indigo);
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
		background: var(--rtp-bg);
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
	.plan__cta--indigo {
		background: var(--rtp-bg);
		border: 1px solid var(--rtp-indigo);
		color: var(--rtp-indigo);
	}
	.plan__cta--indigo:hover {
		background: var(--rtp-indigo);
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
		background: var(--rtp-surface);
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
		background: var(--rtp-surface);
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
		background: var(--rtp-bg);
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
		color: var(--rtp-muted);
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
		background: linear-gradient(to bottom right, var(--rtp-primary), var(--sa-indigo-900));
		color: #fff;
		text-align: center;
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
		font-size: 1.875rem;
		font-weight: 800;
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.bell-cta__title {
			font-size: 3rem;
		}
	}
	.bell-cta__lede {
		font-size: 1.25rem;
		color: #dbeafe;
		max-width: 42rem;
		margin-inline: auto;
		margin-block-end: 2.5rem;
	}
	.bell-cta__btn {
		display: inline-block;
		background: #fff;
		color: var(--rtp-primary);
		padding: 1rem 2.5rem;
		border-radius: var(--rtp-radius-md);
		font-weight: 700;
		font-size: 1.125rem;
		text-decoration: none;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
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
