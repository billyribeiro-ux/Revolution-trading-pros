<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import PricingSection from './_sections/PricingSection.svelte';
	import { faqData } from './faq-data';

	// --- FAQ State ---
	let openFaq: number | null = $state(null);
	let pageRef = $state<HTMLElement | null>(null);
	const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

	const capturePage: Attachment<HTMLElement> = (node) => {
		pageRef = node;

		return () => {
			if (pageRef === node) {
				pageRef = null;
			}
		};
	};

	// --- Icon SVG ---

	/**
	 * GSAP ScrollTrigger initialization (Svelte 5 SSR-safe pattern)
	 * Apple-grade scroll animations with GPU acceleration
	 */
	onMount(() => {
		if (!browser) return;

		let ctx: ReturnType<typeof import('gsap').gsap.context> | null = null;
		let cancelled = false;

		void (async () => {
			const root = pageRef;
			if (!root) return;

			try {
				const { gsap } = await import('gsap');
				const { ScrollTrigger } = await import('gsap/ScrollTrigger');
				if (cancelled || pageRef !== root) return;

				gsap.registerPlugin(ScrollTrigger);

				// Respect reduced motion preference
				const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

				if (prefersReducedMotion) {
					gsap.set(root.querySelectorAll('[data-gsap]'), { opacity: 1, y: 0 });
					return;
				}

				// Use gsap.context() for scoped cleanup - prevents global ScrollTrigger destruction
				ctx = gsap.context(() => {
					// Only set initial hidden state for elements NOT yet in viewport
					const elements = root.querySelectorAll('[data-gsap]');
					elements.forEach((el) => {
						const rect = el.getBoundingClientRect();
						const isInViewport = rect.top < window.innerHeight * 0.85;
						if (!isInViewport) {
							gsap.set(el, { opacity: 0, y: 30 });
						}
					});

					// Create ScrollTrigger batch for optimal performance
					ScrollTrigger.batch('[data-gsap]', {
						onEnter: (batch) => {
							if (cancelled) return;

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
				}, root);

				if (cancelled) {
					ctx.revert();
					ctx = null;
				}
			} catch (error) {
				if (!cancelled) {
					console.error('[ExplosiveSwings] GSAP initialization failed:', error);
				}
			}
		})();

		return () => {
			cancelled = true;
			ctx?.revert();
			ctx = null;
		};
	});

	// FAQ data extracted to ./faq-data so +page.ts can share it.

	// Methodology cards — drive variant coloring from CSS modifier classes.
	const methodologySteps = [
		{
			variant: 'emerald' as const,
			title: '1. Dark Pool Analysis',
			body: 'Institutions trade billions off-exchange to hide their intent. We monitor these "Dark Pool" block trades to see where Smart Money is accumulating positions *before* the breakout happens.'
		},
		{
			variant: 'blue' as const,
			title: '2. Technical Confirmation',
			body: "Data isn't enough; timing is everything. We layer technical analysis (Supply/Demand Zones, Key Moving Averages, and Volume Profiles) to pinpoint the exact moment momentum shifts."
		},
		{
			variant: 'indigo' as const,
			title: '3. Risk-First Execution',
			body: 'We hate losing money. Every alert comes with a predefined "Hard Stop" level. We calculate position size so that even if a trade fails, it only scratches the paint, but when it hits, it pays for the month.'
		}
	];
</script>

<div class="es" {@attach capturePage}>
	<!-- ─────────────────────────────────────────────────────────────
	     Hero
	     ───────────────────────────────────────────────────────────── -->
	<section class="hero">
		<div class="hero__bg" aria-hidden="true">
			<div class="hero__bg-grid"></div>
			<div class="hero__bg-halo hero__bg-halo--emerald"></div>
			<div class="hero__bg-halo hero__bg-halo--blue"></div>
		</div>

		<div class="hero__grid">
			<div class="hero__copy">
				<div data-gsap class="hero__badge">
					<span class="hero__badge-pulse">
						<span class="hero__badge-pulse-ping"></span>
						<span class="hero__badge-pulse-dot"></span>
					</span>
					<span class="hero__badge-label">Live Signals Active</span>
				</div>

				<h1 data-gsap class="hero__title">
					Catch the <br />
					<span class="hero__title-accent">Institutional Moves.</span>
				</h1>

				<p data-gsap class="hero__lede">
					Stop staring at the 1-minute chart. Get high-precision <strong
						>multi-day swing alerts</strong
					>
					backed by Dark Pool data. Designed for traders who want financial freedom, not another 9-to-5
					job.
				</p>

				<div data-gsap class="hero__cta-row">
					<a href="#pricing" class="hero__cta-primary">
						Start Trading Swings
						<svg
							class="hero__cta-arrow"
							aria-hidden="true"
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
					<a href="#methodology" class="hero__cta-secondary">See The Strategy</a>
				</div>

				<div data-gsap class="hero__trust-row">
					<div class="hero__trust-item">
						<svg
							class="hero__trust-icon"
							aria-hidden="true"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>Precise Entries &amp; Exits</span>
					</div>
					<div class="hero__trust-item">
						<svg
							class="hero__trust-icon"
							aria-hidden="true"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>3-7 Day Hold Time</span>
					</div>
					<div class="hero__trust-item">
						<svg
							class="hero__trust-icon"
							aria-hidden="true"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 10V3L4 14h7v7l9-11h-7z"
							/>
						</svg>
						<span>High R:R Ratio</span>
					</div>
				</div>
			</div>

			<div class="hero__preview" style="contain: layout style paint;">
				<div class="hero__preview-halo" aria-hidden="true"></div>

				<div class="hero__preview-card">
					<div class="hero__preview-head">
						<div class="hero__preview-brand">
							<div class="hero__preview-logo">
								<svg
									class="hero__preview-logo-icon"
									aria-hidden="true"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M13 10V3L4 14h7v7l9-11h-7z"
									/></svg
								>
							</div>
							<div>
								<div class="hero__preview-name">Explosive Swings</div>
								<div class="hero__preview-status">● Multi-Day Alerts</div>
							</div>
						</div>
						<div class="hero__preview-time">10:30 AM</div>
					</div>

					<div class="hero__preview-body">
						<div class="hero__signal hero__signal--emerald">
							<div class="hero__signal-meta">
								<span class="hero__signal-label hero__signal-label--emerald">New Signal</span>
								<span class="hero__signal-time">Just now</span>
							</div>
							<div class="hero__signal-line">
								<span class="hero__signal-ticker">NVDA 480 CALLS</span>
								<span class="hero__signal-exp">Exp: Next Week</span>
							</div>
							<div class="hero__signal-targets">
								<span>Entry: $5.50 - $6.00</span>
								<span>🎯 Target: $8.50+</span>
							</div>
						</div>
						<div class="hero__signal hero__signal--red">
							<div class="hero__signal-meta">
								<span class="hero__signal-label hero__signal-label--red">Invalidation</span>
								<span class="hero__signal-stop">$4.20 (Close Below)</span>
							</div>
						</div>
					</div>

					<div class="hero__preview-badge">
						<span class="hero__preview-badge-emoji">🚀</span>
						<div>
							<div class="hero__preview-badge-label">Potential Return</div>
							<div class="hero__preview-badge-value">+45%</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Stat strip
	     ───────────────────────────────────────────────────────────── -->
	<section class="stat-strip">
		<div class="stat-strip__inner">
			<dl class="stat-strip__grid">
				<div class="stat-strip__item">
					<dt class="stat-strip__label">Historical Win Rate</dt>
					<dd class="stat-strip__value stat-strip__value--emerald">82%</dd>
				</div>
				<div class="stat-strip__item">
					<dt class="stat-strip__label">Avg Hold Time</dt>
					<dd class="stat-strip__value">
						3-7<span class="stat-strip__unit">days</span>
					</dd>
				</div>
				<div class="stat-strip__item">
					<dt class="stat-strip__label">Risk/Reward</dt>
					<dd class="stat-strip__value stat-strip__value--indigo">4:1</dd>
				</div>
				<div class="stat-strip__item">
					<dt class="stat-strip__label">Alerts Per Week</dt>
					<dd class="stat-strip__value stat-strip__value--blue">2-4</dd>
				</div>
			</dl>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Lifestyle comparison
	     ───────────────────────────────────────────────────────────── -->
	<section class="lifestyle">
		<div class="lifestyle__inner">
			<div class="lifestyle__header">
				<h2 data-gsap class="lifestyle__title">Choose Your Trading Lifestyle</h2>
				<p data-gsap class="lifestyle__lede">
					Most traders burn out trying to scalp 1-minute candles against high-frequency algorithms.
					We play the bigger timeframe where institutions move money.
				</p>
			</div>

			<div class="lifestyle__grid">
				<div data-gsap class="lifestyle__card lifestyle__card--bad">
					<div class="lifestyle__card-head">
						<div class="lifestyle__card-icon">😰</div>
						<h3 class="lifestyle__card-title lifestyle__card-title--bad">The "Day Scalper"</h3>
					</div>
					<ul class="lifestyle__list lifestyle__list--bad">
						<li><span class="lifestyle__x">✕</span> Glued to screen 6+ hours/day</li>
						<li><span class="lifestyle__x">✕</span> High stress, high cortisol spikes</li>
						<li><span class="lifestyle__x">✕</span> Intense competition with HFT algos</li>
						<li><span class="lifestyle__x">✕</span> "Did I miss the move?" anxiety</li>
						<li><span class="lifestyle__x">✕</span> Often overtrades and triggers PDT</li>
					</ul>
				</div>

				<div data-gsap class="lifestyle__card lifestyle__card--good">
					<div class="lifestyle__pill">RECOMMENDED</div>
					<div class="lifestyle__card-head">
						<div class="lifestyle__card-icon lifestyle__card-icon--good">🧘‍♂️</div>
						<h3 class="lifestyle__card-title">The "Swing Trader"</h3>
					</div>
					<ul class="lifestyle__list">
						<li>
							<svg
								class="lifestyle__check"
								aria-hidden="true"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Check charts once or twice a day
						</li>
						<li>
							<svg
								class="lifestyle__check"
								aria-hidden="true"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Calm, calculated decisions based on data
						</li>
						<li>
							<svg
								class="lifestyle__check"
								aria-hidden="true"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Catch the meat of the move (20% to 100%+)
						</li>
						<li>
							<svg
								class="lifestyle__check"
								aria-hidden="true"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Keep your day job and compounding wealth
						</li>
						<li>
							<svg
								class="lifestyle__check"
								aria-hidden="true"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Generous entry windows (no rush)
						</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Methodology
	     ───────────────────────────────────────────────────────────── -->
	<section id="methodology" class="methodology">
		<div class="methodology__inner">
			<div class="methodology__header">
				<span class="methodology__eyebrow">The Strategy</span>
				<h2 data-gsap class="methodology__title">How We Find the "Whale's Wake"</h2>
				<p class="methodology__lede">
					We don't guess. We track the flow of money. Institutional investors leave footprints
					called "Dark Pool Prints" and "Unusual Options Activity." We follow them.
				</p>
			</div>

			<div class="methodology__grid">
				{#each methodologySteps as step (step.title)}
					<article data-gsap class="method-card method-card--{step.variant}">
						<div class="method-card__icon">
							{#if step.variant === 'emerald'}
								<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
									/></svg
								>
							{:else if step.variant === 'blue'}
								<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
									/><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
									/></svg
								>
							{:else}
								<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
									/></svg
								>
							{/if}
						</div>
						<h3 class="method-card__title">{step.title}</h3>
						<p class="method-card__body">{step.body}</p>
					</article>
				{/each}
			</div>

			<div data-gsap class="delivery">
				<div class="delivery__copy">
					<h4 class="delivery__title">How You Receive Alerts</h4>
					<p class="delivery__sub">
						We ensure you never miss a signal. Notifications are instant and redundant.
					</p>
				</div>
				<div class="delivery__channels">
					<div class="delivery__channel">
						<span class="delivery__channel-dot delivery__channel-dot--emerald"></span>
						<span class="delivery__channel-label">Discord</span>
					</div>
					<div class="delivery__channel">
						<span class="delivery__channel-dot delivery__channel-dot--blue"></span>
						<span class="delivery__channel-label">SMS Text</span>
					</div>
					<div class="delivery__channel">
						<span class="delivery__channel-dot delivery__channel-dot--indigo"></span>
						<span class="delivery__channel-label">Email</span>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Recent performance table
	     ───────────────────────────────────────────────────────────── -->
	<section class="perf">
		<div class="perf__inner">
			<div class="perf__header">
				<div>
					<h2 class="perf__title">Recent Swing Performance</h2>
					<p class="perf__sub">Real trades. Real timestamps. Transparent results.</p>
				</div>
				<div class="perf__live-wrap">
					<span class="perf__live">Live Database Updated: Today</span>
				</div>
			</div>

			<div class="perf__table-wrap">
				<table class="perf__table">
					<thead>
						<tr>
							<th>Ticker</th>
							<th>Direction</th>
							<th>Time Held</th>
							<th class="perf__th-right">Net Return</th>
							<th class="perf__th-right perf__col-note">Catalyst / Note</th>
						</tr>
					</thead>
					<tbody>
						<tr class="perf__row-hover">
							<td class="perf__ticker">
								NVDA
								<span class="perf__sub-name">NVIDIA Corp</span>
							</td>
							<td class="perf__dir perf__dir--long">CALLS (Long)</td>
							<td class="perf__held">5 Days</td>
							<td class="perf__return perf__return--pos">+125%</td>
							<td class="perf__note perf__col-note">Breakout of $480 resistance level.</td>
						</tr>
						<tr class="perf__row-hover">
							<td class="perf__ticker">AMD</td>
							<td class="perf__dir perf__dir--long">CALLS (Long)</td>
							<td class="perf__held">3 Days</td>
							<td class="perf__return perf__return--pos">+45%</td>
							<td class="perf__note perf__col-note">Sector rotation &amp; AI strength.</td>
						</tr>
						<tr class="perf__row-hover perf__row-hover--loss">
							<td class="perf__ticker">TSLA</td>
							<td class="perf__dir perf__dir--short">PUTS (Short)</td>
							<td class="perf__held">1 Day</td>
							<td class="perf__return perf__return--neg">-15%</td>
							<td class="perf__note perf__col-note">Reversal against trend. Hard stop hit.</td>
						</tr>
						<tr class="perf__row-hover">
							<td class="perf__ticker">META</td>
							<td class="perf__dir perf__dir--long">CALLS (Long)</td>
							<td class="perf__held">7 Days</td>
							<td class="perf__return perf__return--pos">+82%</td>
							<td class="perf__note perf__col-note">Earnings run-up swing strategy.</td>
						</tr>
					</tbody>
				</table>
			</div>
			<p class="perf__disclaimer">
				*Results are illustrative of alert performance. Past performance does not guarantee future
				results.
			</p>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Personas
	     ───────────────────────────────────────────────────────────── -->
	<section class="personas">
		<div class="personas__inner">
			<h2 class="personas__title">Who Needs This Service?</h2>
			<div class="personas__grid">
				<div class="persona">
					<div class="persona__num">1</div>
					<div>
						<h4 class="persona__title">Professionals with Day Jobs</h4>
						<p class="persona__body">
							You can't watch the screen from 9:30 to 4:00. You need a strategy that lets you enter
							trades on your lunch break or from your phone without panic.
						</p>
					</div>
				</div>
				<div class="persona">
					<div class="persona__num">2</div>
					<div>
						<h4 class="persona__title">Small Account Builders</h4>
						<p class="persona__body">
							You want to grow a $2k - $10k account aggressively but safely, avoiding the "PDT Rule"
							that restricts day traders.
						</p>
					</div>
				</div>
				<div class="persona">
					<div class="persona__num">3</div>
					<div>
						<h4 class="persona__title">Failed Day Traders</h4>
						<p class="persona__body">
							You've tried scalping and lost money to churn and fees. You're ready for a calmer,
							more statistical approach to the markets.
						</p>
					</div>
				</div>
				<div class="persona">
					<div class="persona__num">4</div>
					<div>
						<h4 class="persona__title">Options Buyers</h4>
						<p class="persona__body">
							You understand the power of leverage. You want to turn a 5% stock move into a 50%
							option gain using safe, defined-risk structures.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<PricingSection />

	<!-- ─────────────────────────────────────────────────────────────
	     FAQ
	     ───────────────────────────────────────────────────────────── -->
	<section class="faq">
		<div class="faq__inner">
			<h2 class="faq__title">Frequently Asked Questions</h2>
			<p class="faq__sub">
				Everything you need to know about the service, capital requirements, and strategy.
			</p>

			<div class="faq__list">
				{#each faqData as item, i (item.q)}
					<div class="faq__item">
						<button
							type="button"
							class="faq__trigger"
							onclick={() => toggleFaq(i)}
							aria-expanded={openFaq === i}
						>
							<span class="faq__q">{item.q}</span>
							<svg
								class="faq__chevron {openFaq === i ? 'faq__chevron--open' : ''}"
								aria-hidden="true"
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
								{item.a}
							</div>
						{/if}
					</div>
				{/each}
			</div>

			<div class="faq__contact">
				<p class="faq__contact-prompt">Still have questions?</p>
				<a href="/contact" class="faq__contact-link">Contact Our Support Team</a>
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Bottom CTA
	     ───────────────────────────────────────────────────────────── -->
	<section class="bottom-cta">
		<div class="bottom-cta__inner">
			<h2 class="bottom-cta__title">Stop Overtrading. Start Swinging.</h2>
			<p class="bottom-cta__lede">
				Join the trading room that values your time as much as your capital. Instant alerts,
				verified results, and a community of winners.
			</p>
			<a href="#pricing" class="bottom-cta__button">Get Instant Access</a>
			<p class="bottom-cta__legal">
				<span>Secure Checkout</span>
				<span class="bottom-cta__dot"></span>
				<span>Cancel Anytime</span>
				<span class="bottom-cta__dot"></span>
				<span>30-Day Guarantee</span>
			</p>
		</div>
	</section>
</div>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Page-local tokens (one-offs for this page only).
	   Reusable color/spacing values come from --rtp-* (marketing.css).
	   Note on muted-text token choice: surrounding body copy used
	   `text-slate-400` (#94a3b8) — converging on --rtp-text-muted which
	   maps to that exact shade. The legacy --rtp-muted (#6b7280) is
	   dimmer and not used here.
	   ───────────────────────────────────────────────────────────────── */
	.es {
		--es-bg: #020617; /* slate-950 — page canvas, deeper than --rtp-bg */
		--es-surface: #0f172a; /* slate-900 — section backgrounds */
		--es-surface-elev: #1e293b; /* slate-800 alt for borders/dividers */
		--es-border: #1e293b; /* slate-800 */
		--es-border-soft: #334155; /* slate-700 */
		--es-emerald-bright: #34d399; /* emerald-400 */
		--es-emerald-deep: #047857; /* emerald-700, bottom CTA gradient */
		--es-teal-deep: #115e59; /* teal-800, bottom CTA gradient end */
		--es-emerald-light: #ecfdf5; /* emerald-50, bottom CTA hover */
		--es-red-soft: #f87171; /* red-400 */
		--es-red-mid: #ef4444; /* red-500 — × bullets */

		width: 100%;
		background: var(--es-bg);
		color: var(--rtp-text-soft);
		font-family: var(--rtp-font-sans);
	}
	.es ::selection {
		background: color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
		color: var(--es-emerald-bright);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Hero
	   ───────────────────────────────────────────────────────────────── */
	.hero {
		position: relative;
		min-height: 90vh;
		display: flex;
		align-items: center;
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
		z-index: 0;
		pointer-events: none;
	}
	.hero__bg-grid {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(color-mix(in oklab, var(--rtp-emerald) 3%, transparent) 1px, transparent 1px),
			linear-gradient(
				90deg,
				color-mix(in oklab, var(--rtp-emerald) 3%, transparent) 1px,
				transparent 1px
			);
		background-size: 48px 48px;
		opacity: 0.4;
	}
	.hero__bg-halo {
		position: absolute;
		border-radius: 50%;
		filter: blur(64px);
	}
	.hero__bg-halo--emerald {
		inset-block-start: 0;
		inset-inline-end: 0;
		width: 600px;
		height: 600px;
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
	}
	.hero__bg-halo--blue {
		inset-block-end: 0;
		inset-inline-start: 0;
		width: 500px;
		height: 500px;
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
	}

	.hero__grid {
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
		.hero__grid {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.hero__grid {
			padding-inline: 2rem;
			grid-template-columns: 1fr 1fr;
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

	.hero__badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: var(--es-surface);
		border: 1px solid color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
		padding: 0.375rem 1rem;
		border-radius: var(--rtp-radius-pill);
		margin-block-end: 2rem;
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		backdrop-filter: blur(12px);
	}
	.hero__badge-pulse {
		position: relative;
		display: flex;
		width: 0.625rem;
		height: 0.625rem;
	}
	.hero__badge-pulse-ping {
		position: absolute;
		inset: 0;
		display: inline-flex;
		border-radius: 50%;
		background: var(--es-emerald-bright);
		opacity: 0.75;
		animation: es-ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
	}
	.hero__badge-pulse-dot {
		position: relative;
		display: inline-flex;
		width: 100%;
		height: 100%;
		border-radius: 50%;
		background: var(--rtp-emerald);
	}
	@keyframes es-ping {
		75%,
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}
	.hero__badge-label {
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--es-emerald-bright);
	}

	.hero__title {
		font-family: var(--rtp-font-display);
		font-size: 3rem;
		font-weight: 800;
		line-height: 1.1;
		letter-spacing: -0.025em;
		color: var(--rtp-text);
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.hero__title {
			font-size: 4.5rem;
		}
	}
	.hero__title-accent {
		background: linear-gradient(
			to right,
			var(--es-emerald-bright),
			#6ee7b7,
			#99f6e4
		); /* emerald-400 → emerald-300 → teal-200 */
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}

	.hero__lede {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
		line-height: 1.625;
		max-width: 42rem;
		margin-inline: auto;
		margin-block-end: 2.5rem;
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

	.hero__cta-primary,
	.hero__cta-secondary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 1rem 2rem;
		font-size: 1.125rem;
		font-weight: 700;
		border-radius: var(--rtp-radius-md);
		text-decoration: none;
		transition: all 0.2s ease;
		width: 100%;
	}
	@media (min-width: 640px) {
		.hero__cta-primary,
		.hero__cta-secondary {
			width: auto;
		}
	}
	.hero__cta-primary {
		position: relative;
		background: var(--rtp-emerald);
		color: var(--es-bg);
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
	}
	.hero__cta-primary:hover {
		background: var(--es-emerald-bright);
		transform: translateY(-4px);
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-emerald) 25%, transparent);
	}
	.hero__cta-primary:focus-visible {
		outline: 2px solid var(--rtp-emerald);
		outline-offset: 2px;
	}
	.hero__cta-arrow {
		width: 1.25rem;
		height: 1.25rem;
		margin-inline-start: 0.5rem;
		transition: transform 0.2s ease;
	}
	.hero__cta-primary:hover .hero__cta-arrow {
		transform: translateX(0.25rem);
	}

	.hero__cta-secondary {
		background: var(--es-surface);
		color: var(--rtp-text-soft);
		border: 1px solid var(--es-border-soft);
	}
	.hero__cta-secondary:hover {
		background: var(--es-border);
		border-color: color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
	}

	.hero__trust-row {
		margin-block-start: 2.5rem;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: center;
		gap: 1.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--rtp-text-subtle);
	}
	@media (min-width: 1024px) {
		.hero__trust-row {
			justify-content: flex-start;
		}
	}
	.hero__trust-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.hero__trust-icon {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--rtp-emerald);
	}

	/* ── Hero preview card (right column) ─────────────────────────── */
	.hero__preview {
		display: none;
		position: relative;
		perspective: 1000px;
	}
	@media (min-width: 1024px) {
		.hero__preview {
			display: block;
		}
	}
	.hero__preview-halo {
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
	.hero__preview-card {
		position: relative;
		background: color-mix(in oklab, var(--rtp-surface) 80%, transparent);
		backdrop-filter: blur(16px);
		border: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
		border-radius: 1.5rem;
		padding: 1.5rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		transform: rotateY(-10deg) rotateX(5deg);
		transition: transform 0.7s var(--rtp-ease-out);
	}
	.hero__preview-card:hover {
		transform: rotate(0);
	}
	.hero__preview-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: 1.5rem;
		border-block-end: 1px solid color-mix(in oklab, var(--rtp-border) 30%, transparent);
		padding-block-end: 1rem;
	}
	.hero__preview-brand {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}
	.hero__preview-logo {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		background: linear-gradient(to bottom right, var(--rtp-primary), var(--rtp-blue));
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--rtp-text);
		font-weight: 700;
		box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
	}
	.hero__preview-logo-icon {
		width: 1.5rem;
		height: 1.5rem;
	}
	.hero__preview-name {
		font-weight: 700;
		color: var(--rtp-text);
	}
	.hero__preview-status {
		font-size: 0.75rem;
		color: var(--rtp-emerald);
	}
	.hero__preview-time {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-text-muted);
		background: var(--rtp-bg);
		padding: 0.25rem 0.5rem;
		border-radius: var(--rtp-radius-sm);
	}
	.hero__preview-body {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.hero__signal {
		padding: 1rem;
		border-radius: var(--rtp-radius-md);
		background: color-mix(in oklab, var(--rtp-bg) 50%, transparent);
		border-inline-start: 4px solid;
	}
	.hero__signal--emerald {
		border-inline-start-color: var(--rtp-emerald);
	}
	.hero__signal--red {
		border-inline-start-color: color-mix(in oklab, var(--es-red-mid) 50%, transparent);
	}
	.hero__signal-meta {
		display: flex;
		justify-content: space-between;
		font-size: 0.75rem;
		margin-block-end: 0.5rem;
	}
	.hero__signal-label {
		font-weight: 700;
		text-transform: uppercase;
	}
	.hero__signal-label--emerald {
		color: var(--rtp-emerald);
	}
	.hero__signal-label--red {
		color: var(--es-red-soft);
	}
	.hero__signal-time {
		color: var(--rtp-text-muted);
	}
	.hero__signal-line {
		font-family: var(--rtp-font-mono);
		font-size: 0.875rem;
		color: var(--rtp-text);
		margin-block-end: 0.25rem;
	}
	.hero__signal-ticker {
		font-weight: 700;
		color: var(--rtp-text);
	}
	.hero__signal-exp {
		color: var(--rtp-text-muted);
		margin-inline-start: 0.5rem;
	}
	.hero__signal-targets {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: var(--rtp-text-muted);
	}
	.hero__signal-stop {
		color: var(--es-red-soft);
		font-family: var(--rtp-font-mono);
	}
	.hero__preview-badge {
		position: absolute;
		inset-block-end: -1.5rem;
		inset-inline-end: -1.5rem;
		background: #fff;
		color: var(--rtp-bg);
		padding: 0.75rem 1.5rem;
		border-radius: var(--rtp-radius-md);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.4);
		font-weight: 700;
		border: 2px solid var(--rtp-bg);
		display: flex;
		align-items: center;
		gap: 0.5rem;
		animation: es-bounce 2s infinite;
	}
	@keyframes es-bounce {
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
	.hero__preview-badge-emoji {
		font-size: 1.5rem;
	}
	.hero__preview-badge-label {
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		opacity: 0.7;
	}
	.hero__preview-badge-value {
		color: #059669; /* emerald-600 */
	}

	/* ─────────────────────────────────────────────────────────────────
	   Stat strip
	   ───────────────────────────────────────────────────────────────── */
	.stat-strip {
		background: var(--es-surface);
		border-block: 1px solid var(--es-border);
		position: relative;
		z-index: 20;
	}
	.stat-strip__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-block: 3rem;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.stat-strip__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.stat-strip__inner {
			padding-inline: 2rem;
		}
	}
	.stat-strip__grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 2rem;
	}
	@media (min-width: 768px) {
		.stat-strip__grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}
	.stat-strip__item {
		text-align: center;
	}
	.stat-strip__label {
		color: var(--rtp-text-subtle);
		font-weight: 500;
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-block-end: 0.5rem;
	}
	.stat-strip__value {
		font-size: 2.25rem;
		font-weight: 800;
		color: var(--rtp-text);
	}
	@media (min-width: 768px) {
		.stat-strip__value {
			font-size: 3rem;
		}
	}
	.stat-strip__value--emerald {
		color: var(--rtp-emerald);
	}
	.stat-strip__value--indigo {
		color: #818cf8; /* indigo-400 */
	}
	.stat-strip__value--blue {
		color: var(--rtp-blue-bright);
	}
	.stat-strip__unit {
		font-size: 1.125rem;
		color: var(--rtp-text-subtle);
		font-weight: 400;
		margin-inline-start: 0.25rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Lifestyle comparison
	   ───────────────────────────────────────────────────────────────── */
	.lifestyle {
		padding-block: 6rem;
		background: var(--es-bg);
		position: relative;
		overflow: hidden;
	}
	.lifestyle__inner {
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.lifestyle__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.lifestyle__inner {
			padding-inline: 2rem;
		}
	}
	.lifestyle__header {
		text-align: center;
		margin-block-end: 4rem;
	}
	.lifestyle__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.lifestyle__title {
			font-size: 3rem;
		}
	}
	.lifestyle__lede {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
		max-width: 42rem;
		margin-inline: auto;
	}
	.lifestyle__grid {
		display: grid;
		gap: 2rem;
		max-width: 64rem;
		margin-inline: auto;
	}
	@media (min-width: 768px) {
		.lifestyle__grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	.lifestyle__card {
		border-radius: 1.5rem;
		padding: 2.5rem;
		position: relative;
		overflow: hidden;
	}
	.lifestyle__card--bad {
		background: color-mix(in oklab, var(--es-surface) 50%, transparent);
		border: 1px solid var(--es-border);
	}
	.lifestyle__card--good {
		background: var(--es-surface);
		border: 2px solid var(--rtp-emerald);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
	}
	.lifestyle__pill {
		position: absolute;
		inset-block-start: 0;
		inset-inline-end: 0;
		background: var(--rtp-emerald);
		color: var(--es-bg);
		font-size: 0.75rem;
		font-weight: 700;
		padding: 0.25rem 0.75rem;
		border-end-start-radius: var(--rtp-radius-md);
	}
	.lifestyle__card-head {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-block-end: 1.5rem;
	}
	.lifestyle__card-icon {
		width: 3rem;
		height: 3rem;
		border-radius: 50%;
		background: var(--es-border);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.5rem;
	}
	.lifestyle__card-icon--good {
		background: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
	}
	.lifestyle__card-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--rtp-text);
	}
	.lifestyle__card-title--bad {
		color: var(--rtp-text-muted);
	}
	.lifestyle__list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		color: var(--rtp-text-soft);
		font-weight: 500;
	}
	.lifestyle__list--bad {
		color: var(--rtp-text-subtle);
		font-weight: 400;
	}
	.lifestyle__list li {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}
	.lifestyle__x {
		color: var(--es-red-mid);
		font-weight: 700;
	}
	.lifestyle__check {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--rtp-emerald);
		flex-shrink: 0;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Methodology
	   ───────────────────────────────────────────────────────────────── */
	.methodology {
		padding-block: 6rem;
		background: var(--es-surface);
		border-block-start: 1px solid var(--es-border);
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
	.methodology__header {
		margin-block-end: 4rem;
		max-width: 48rem;
		margin-inline: auto;
	}
	@media (min-width: 768px) {
		.methodology__header {
			text-align: center;
		}
	}
	.methodology__eyebrow {
		color: var(--rtp-emerald);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-size: 0.875rem;
	}
	.methodology__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block: 0.5rem 1.5rem;
	}
	@media (min-width: 768px) {
		.methodology__title {
			font-size: 3rem;
		}
	}
	.methodology__lede {
		color: var(--rtp-text-muted);
		font-size: 1.125rem;
	}
	.methodology__grid {
		display: grid;
		gap: 2.5rem;
	}
	@media (min-width: 768px) {
		.methodology__grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	.method-card {
		background: var(--es-bg);
		padding: 2rem;
		border-radius: var(--rtp-radius-xl);
		border: 1px solid var(--es-border);
		transition: border-color var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.method-card--emerald:hover {
		border-color: color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
	}
	.method-card--blue:hover {
		border-color: color-mix(in oklab, var(--rtp-primary) 30%, transparent);
	}
	.method-card--indigo:hover {
		border-color: color-mix(in oklab, var(--rtp-indigo) 30%, transparent);
	}
	.method-card__icon {
		width: 3.5rem;
		height: 3.5rem;
		border-radius: var(--rtp-radius-xl);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-block-end: 1.5rem;
		border: 1px solid;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.method-card__icon svg {
		width: 1.75rem;
		height: 1.75rem;
	}
	.method-card:hover .method-card__icon {
		transform: scale(1.1);
	}
	.method-card--emerald .method-card__icon {
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		color: var(--rtp-emerald);
	}
	.method-card--blue .method-card__icon {
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-primary) 20%, transparent);
		color: var(--rtp-primary);
	}
	.method-card--indigo .method-card__icon {
		background: color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-indigo) 20%, transparent);
		color: var(--rtp-indigo);
	}
	.method-card__title {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.75rem;
	}
	.method-card__body {
		color: var(--rtp-text-muted);
		line-height: 1.625;
	}

	/* ── Delivery callout ─────────────────────────────────────────── */
	.delivery {
		margin-block-start: 4rem;
		background: var(--es-bg);
		padding: 1.5rem;
		border-radius: var(--rtp-radius-xl);
		border: 1px solid color-mix(in oklab, var(--es-border-soft) 50%, transparent);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
		justify-content: space-between;
	}
	@media (min-width: 768px) {
		.delivery {
			padding: 2rem;
			flex-direction: row;
		}
	}
	.delivery__title {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.5rem;
	}
	.delivery__sub {
		color: var(--rtp-text-muted);
		font-size: 0.875rem;
	}
	.delivery__channels {
		display: flex;
		gap: 1rem;
	}
	.delivery__channel {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: var(--es-surface);
		padding: 0.5rem 1rem;
		border-radius: var(--rtp-radius-md);
		border: 1px solid var(--es-border);
	}
	.delivery__channel-dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 50%;
		animation: es-pulse 2s infinite;
	}
	.delivery__channel-dot--emerald {
		background: var(--rtp-emerald);
	}
	.delivery__channel-dot--blue {
		background: var(--rtp-primary);
	}
	.delivery__channel-dot--indigo {
		background: var(--rtp-indigo);
	}
	@keyframes es-pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}
	.delivery__channel-label {
		font-size: 0.875rem;
		font-weight: 700;
		color: #cbd5e1; /* slate-300 — already token --rtp-text-soft */
	}

	/* ─────────────────────────────────────────────────────────────────
	   Performance table
	   ───────────────────────────────────────────────────────────────── */
	.perf {
		padding-block: 6rem;
		background: var(--es-bg);
	}
	.perf__inner {
		max-width: 72rem;
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.perf__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.perf__inner {
			padding-inline: 2rem;
		}
	}
	.perf__header {
		margin-block-end: 2.5rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: flex-end;
		gap: 1rem;
	}
	@media (min-width: 768px) {
		.perf__header {
			flex-direction: row;
		}
	}
	.perf__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.5rem;
	}
	.perf__sub {
		color: var(--rtp-text-subtle);
	}
	.perf__live-wrap {
		text-align: end;
		display: none;
	}
	@media (min-width: 768px) {
		.perf__live-wrap {
			display: block;
		}
	}
	.perf__live {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-emerald);
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		padding: 0.25rem 0.75rem;
		border-radius: var(--rtp-radius-sm);
		border: 1px solid color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
	}

	.perf__table-wrap {
		background: var(--es-surface);
		border-radius: 1rem;
		border: 1px solid var(--es-border);
		overflow: hidden;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
	}
	.perf__table {
		width: 100%;
		text-align: start;
		border-collapse: collapse;
	}
	.perf__table thead tr {
		border-block-end: 1px solid var(--es-border);
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		color: var(--rtp-text-subtle);
		letter-spacing: 0.05em;
	}
	.perf__table th,
	.perf__table td {
		padding: 1rem;
	}
	@media (min-width: 768px) {
		.perf__table th,
		.perf__table td {
			padding: 1.25rem;
		}
	}
	.perf__th-right {
		text-align: end;
	}
	.perf__col-note {
		display: none;
	}
	@media (min-width: 768px) {
		.perf__col-note {
			display: table-cell;
		}
	}
	.perf__table tbody {
		font-family: var(--rtp-font-mono);
		font-size: 0.875rem;
	}
	.perf__table tbody tr {
		border-block-end: 1px solid var(--es-border);
		transition: background-color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.perf__table tbody tr:last-child {
		border-block-end: 0;
	}
	.perf__row-hover:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}
	.perf__row-hover--loss {
		background: color-mix(in oklab, var(--es-red-mid) 5%, transparent);
	}
	.perf__ticker {
		font-weight: 700;
		color: var(--rtp-text);
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.perf__sub-name {
		display: none;
		font-size: 0.625rem;
		color: var(--rtp-text-subtle);
		font-weight: 400;
	}
	.perf__row-hover:hover .perf__sub-name {
		display: inline;
	}
	.perf__dir {
		font-weight: 700;
	}
	.perf__dir--long {
		color: var(--es-emerald-bright);
	}
	.perf__dir--short {
		color: var(--es-red-soft);
	}
	.perf__held {
		color: var(--rtp-text-muted);
	}
	.perf__return {
		text-align: end;
		font-weight: 700;
	}
	.perf__return--pos {
		color: var(--es-emerald-bright);
		background: color-mix(in oklab, var(--rtp-emerald) 5%, transparent);
	}
	.perf__return--neg {
		color: var(--es-red-soft);
	}
	.perf__note {
		text-align: end;
		color: var(--rtp-text-subtle);
		font-size: 0.75rem;
	}
	.perf__disclaimer {
		text-align: center;
		font-size: 0.75rem;
		color: #475569; /* slate-600 — --rtp-text-dim */
		margin-block-start: 1rem;
		font-style: italic;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Personas
	   ───────────────────────────────────────────────────────────────── */
	.personas {
		padding-block: 5rem;
		background: var(--es-surface);
		border-block-start: 1px solid var(--es-border);
	}
	.personas__inner {
		max-width: 56rem;
		margin-inline: auto;
		padding-inline: 1rem;
		text-align: center;
	}
	.personas__title {
		font-family: var(--rtp-font-display);
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 2rem;
	}
	@media (min-width: 768px) {
		.personas__title {
			font-size: 1.875rem;
		}
	}
	.personas__grid {
		display: grid;
		gap: 2rem;
		text-align: start;
	}
	@media (min-width: 768px) {
		.personas__grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}
	.persona {
		display: flex;
		gap: 1rem;
	}
	.persona__num {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--rtp-emerald);
		font-weight: 700;
	}
	.persona__title {
		color: var(--rtp-text);
		font-weight: 700;
		margin-block-end: 0.25rem;
	}
	.persona__body {
		color: var(--rtp-text-muted);
		font-size: 0.875rem;
		line-height: 1.625;
	}

	/* ─────────────────────────────────────────────────────────────────
	   FAQ
	   ───────────────────────────────────────────────────────────────── */
	.faq {
		padding-block: 5rem;
		background: var(--es-bg);
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
		margin-block-end: 1rem;
	}
	.faq__sub {
		color: var(--rtp-text-subtle);
		text-align: center;
		margin-block-end: 3rem;
	}
	.faq__list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
	.faq__item {
		border: 1px solid var(--es-border);
		border-radius: var(--rtp-radius-md);
		background: var(--es-surface);
		overflow: hidden;
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
		color: var(--rtp-text-soft);
		font: inherit;
		cursor: pointer;
		transition: background var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.faq__trigger:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}
	.faq__trigger:focus-visible {
		outline: 2px solid var(--rtp-emerald);
		outline-offset: -2px;
	}
	.faq__q {
		padding-inline-end: 2rem;
	}
	.faq__chevron {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--rtp-text-subtle);
		flex-shrink: 0;
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.faq__chevron--open {
		transform: rotate(180deg);
	}
	.faq__panel {
		padding: 1rem 1.5rem 1.5rem;
		color: var(--rtp-text-muted);
		font-size: 0.875rem;
		line-height: 1.625;
		border-block-start: 1px solid color-mix(in oklab, var(--es-border) 50%, transparent);
	}
	.faq__contact {
		text-align: center;
		margin-block-start: 3rem;
	}
	.faq__contact-prompt {
		color: var(--rtp-text-subtle);
	}
	.faq__contact-link {
		color: var(--rtp-emerald);
		font-weight: 700;
		margin-block-start: 0.5rem;
		display: inline-block;
		border-block-end: 1px solid color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
		padding-block-end: 0.125rem;
		text-decoration: none;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.faq__contact-link:hover {
		color: var(--es-emerald-bright);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Bottom CTA
	   ───────────────────────────────────────────────────────────────── */
	.bottom-cta {
		padding-block: 6rem;
		position: relative;
		overflow: hidden;
		background: linear-gradient(to bottom right, #059669, var(--es-teal-deep));
		color: var(--rtp-text);
	}
	.bottom-cta__inner {
		max-width: 56rem;
		margin-inline: auto;
		padding-inline: 1rem;
		text-align: center;
		position: relative;
		z-index: 10;
	}
	@media (min-width: 640px) {
		.bottom-cta__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.bottom-cta__inner {
			padding-inline: 2rem;
		}
	}
	.bottom-cta__title {
		font-family: var(--rtp-font-display);
		font-size: 2.25rem;
		font-weight: 800;
		letter-spacing: -0.025em;
		margin-block-end: 1.5rem;
	}
	@media (min-width: 768px) {
		.bottom-cta__title {
			font-size: 3.75rem;
		}
	}
	.bottom-cta__lede {
		font-size: 1.25rem;
		color: #d1fae5; /* emerald-100 */
		margin-block-end: 2.5rem;
		max-width: 42rem;
		margin-inline: auto;
	}
	.bottom-cta__button {
		display: inline-block;
		background: #fff;
		color: var(--es-emerald-deep);
		padding: 1.25rem 2.5rem;
		border-radius: var(--rtp-radius-md);
		font-weight: 700;
		font-size: 1.125rem;
		text-decoration: none;
		transition: all 0.2s ease;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}
	.bottom-cta__button:hover {
		background: var(--es-emerald-light);
		transform: translateY(-4px);
	}
	.bottom-cta__legal {
		margin-block-start: 1.5rem;
		font-size: 0.875rem;
		color: color-mix(in oklab, #d1fae5 70%, transparent);
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1rem;
	}
	.bottom-cta__dot {
		width: 0.25rem;
		height: 0.25rem;
		background: #6ee7b7; /* emerald-300 */
		border-radius: 50%;
	}
</style>
