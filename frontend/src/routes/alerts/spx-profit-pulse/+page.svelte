<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';

	// --- SSOT: Pricing Plans ---
	type Plan = {
		id: 'monthly' | 'quarterly' | 'annual';
		label: string;
		price: number;
		period: string;
		perDay: string;
		savingsCopy?: string;
		checkoutHref: string;
		featured: boolean;
		variant: 'simple' | 'featured' | 'highlight';
		features: string[];
	};

	let selectedPlan = $state<'monthly' | 'quarterly' | 'annual'>('quarterly');

	const plans: Plan[] = [
		{
			id: 'monthly',
			label: 'Monthly',
			price: 97,
			period: '/mo',
			perDay: '$3.23/day',
			checkoutHref: '/checkout/monthly-spx',
			featured: false,
			variant: 'simple',
			features: [
				'SMS & Discord Alerts',
				'Live Order Flow Data',
				'Daily Pre-Market Prep',
				'Cancel Anytime'
			]
		},
		{
			id: 'quarterly',
			label: 'Quarterly',
			price: 247,
			period: '/qtr',
			perDay: '$2.74/day',
			savingsCopy: 'Most Popular — Save $44',
			checkoutHref: '/checkout/quarterly-spx',
			featured: true,
			variant: 'featured',
			features: [
				'Everything in Monthly',
				'Weekly Video Breakdowns',
				'Market Context Reports',
				'Priority Support'
			]
		},
		{
			id: 'annual',
			label: 'Annual',
			price: 777,
			period: '/yr',
			perDay: '$2.13/day',
			savingsCopy: 'Best Value — Save $387',
			checkoutHref: '/checkout/annual-spx',
			featured: false,
			variant: 'highlight',
			features: [
				'Everything in Quarterly',
				'1-on-1 Strategy Session',
				'Annual Members Events',
				'Direct DM Access'
			]
		}
	];

	// --- FAQ Logic ---
	let openFaq: number | null = $state(null);
	const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

	// --- GSAP ScrollTrigger Animations ---
	import { browser } from '$app/environment';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconLock from '@tabler/icons-svelte-runes/icons/lock';
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

	async function initGSAP() {
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

			const refreshTrigger = document.fonts?.ready ?? Promise.resolve();
			refreshTrigger.then(() => {
				if (!isComponentMounted) return;
				scrollRefreshRaf = requestAnimationFrame(() => {
					scrollRefreshRaf = 0;
					ScrollTrigger.refresh();
				});
			});
		} catch (error) {
			console.error('[SPX] GSAP initialization failed:', error);
		}
	}

	// SEO schemas emitted via +page.ts (page.data.seo). FAQ list kept locally
	// for the on-page accordion UI (the JSON-LD is owned by the loader).
	const faqList = [
		{
			question: 'What is SPX 0DTE?',
			answer:
				"SPX 0DTE refers to 'Zero Days to Expiration' options on the S&P 500 index. These contracts expire the same day they are traded, offering high potential returns due to rapid gamma exposure."
		},
		{
			question: 'How fast are the alerts?',
			answer:
				'Our alerts are sent instantly via SMS text message and Discord webhooks. The average latency is under 5 seconds from the moment our trader executes the trade.'
		},
		{
			question: 'What account size do I need?',
			answer:
				'Since we trade SPX options, premiums can range from $2.00 to $10.00 ($200-$1,000 per contract). We recommend a starting account of at least $2,000 to manage risk properly.'
		}
	];

	// Feature card data — variant drives icon-tile color via CSS modifier classes.
	type FeatureVariant = 'primary' | 'indigo' | 'emerald' | 'blue' | 'red' | 'purple';
	const featureCards: Array<{ variant: FeatureVariant; title: string; body: string }> = [
		{
			variant: 'primary',
			title: 'Instant SMS & Push',
			body: "Don't miss a move because you stepped away. Alerts hit your phone via SMS and App notification instantly."
		},
		{
			variant: 'indigo',
			title: 'Detailed Strategy Logic',
			body: 'We don\'t just say "Buy". We tell you *why*. Flow, technicals, and gamma levels explained in every alert.'
		},
		{
			variant: 'emerald',
			title: 'Exact Entry & Exits',
			body: 'No guessing games. You get the specific strike, expiration, and limit price. "Buy SPX 4600 Call @ $4.20".'
		},
		{
			variant: 'blue',
			title: 'Runner Management',
			body: 'We scale out to lock in profits and leave "runners" for the big moves. Maximize upside, minimize stress.'
		},
		{
			variant: 'red',
			title: 'Hard Stops (No Bagley)',
			body: 'We never hope. Every trade has a predefined invalidation level. We cut losers fast to protect your capital.'
		},
		{
			variant: 'purple',
			title: 'Market Context',
			body: 'Receive pre-market plans and mid-day updates. Know when to be aggressive and when to sit on your hands.'
		}
	];
</script>

<main class="spx">
	<!-- ─────────────────────────────────────────────────────────────
	     Hero
	     ───────────────────────────────────────────────────────────── -->
	<section class="hero">
		<div class="hero__bg" aria-hidden="true">
			<div class="hero__grid"></div>
			<div class="hero__halo hero__halo--primary"></div>
			<div class="hero__halo hero__halo--indigo"></div>
		</div>

		<div class="hero__inner">
			<div class="hero__copy">
				<div data-gsap class="hero__badge">
					<span class="hero__pulse">
						<span class="hero__pulse-ping"></span>
						<span class="hero__pulse-dot"></span>
					</span>
					<span class="hero__badge-label">Market Active Now</span>
				</div>

				<h1 data-gsap class="hero__title">
					Conquer Volatility with
					<span class="hero__title-accent">SPX 0DTE</span>
				</h1>

				<p data-gsap class="hero__lede">
					Institutional-grade S&amp;P 500 options alerts delivered instantly via SMS &amp; Discord.
					Capture rapid moves with strict risk management.
				</p>

				<div data-gsap class="hero__cta-row">
					<a href="#pricing" class="hero__cta-primary">
						Start Your Trial
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
					<a href="#performance" class="hero__cta-secondary">View Results</a>
				</div>

				<div data-gsap class="hero__trust">
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
						<span>Verified Data</span>
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
						<span>&lt; 5s Latency</span>
					</div>
				</div>
			</div>

			<div class="hero__preview-wrap">
				<div class="hero__preview-halo" aria-hidden="true"></div>

				<div class="hero__preview-card">
					<div class="hero__preview-head">
						<div class="hero__preview-brand">
							<div class="hero__preview-logo">
								<IconBolt size={24} stroke={2} aria-hidden="true" />
							</div>
							<div>
								<div class="hero__preview-name">SPX Profit Pulse</div>
								<div class="hero__preview-status">● Live Trading Room</div>
							</div>
						</div>
						<div class="hero__preview-time">10:32:45 EST</div>
					</div>

					<div class="hero__preview-body">
						<div class="hero__signal hero__signal--emerald">
							<div class="hero__signal-meta">
								<span class="hero__signal-label hero__signal-label--emerald">New Signal</span>
								<span class="hero__signal-time">Just now</span>
							</div>
							<div class="hero__signal-line">
								BTO <span class="hero__signal-ticker">SPX 4580 CALL</span> @ $3.50
							</div>
							<div class="hero__signal-targets">
								<span>🛑 Stop: $2.10</span>
								<span>🎯 Target: $5.00+</span>
							</div>
						</div>
						<div class="hero__signal hero__signal--blue hero__signal--dim">
							<div class="hero__signal-meta">
								<span class="hero__signal-label hero__signal-label--blue">Update</span>
								<span class="hero__signal-time">15m ago</span>
							</div>
							<div class="hero__signal-update">
								Approaching VWAP support. Watching for bounce to add to runners.
							</div>
						</div>
					</div>

					<div class="hero__preview-badge">
						<span class="hero__preview-badge-emoji">🚀</span>
						<div>
							<div class="hero__preview-badge-label">Last Trade</div>
							<div class="hero__preview-badge-value">+85% Profit</div>
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
			<div class="stat-strip__grid">
				<div class="stat-strip__item">
					<div class="stat-strip__value stat-strip__value--emerald">78%</div>
					<div class="stat-strip__label">Historical Win Rate</div>
				</div>
				<div class="stat-strip__item">
					<div class="stat-strip__value">&lt;5s</div>
					<div class="stat-strip__label">Alert Latency</div>
				</div>
				<div class="stat-strip__item">
					<div class="stat-strip__value">1k+</div>
					<div class="stat-strip__label">Active Traders</div>
				</div>
				<div class="stat-strip__item">
					<div class="stat-strip__value">$35M+</div>
					<div class="stat-strip__label">Volume Traded</div>
				</div>
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Features
	     ───────────────────────────────────────────────────────────── -->
	<section class="features">
		<div class="features__bg" aria-hidden="true"></div>
		<div class="features__inner">
			<div class="features__header">
				<h2 data-gsap class="features__title">Institutional Edge, Retail Accessible.</h2>
				<p data-gsap class="features__lede">
					Most retail traders gamble. We operate like a fund. Data-driven entries, strict sizing,
					and emotionless execution.
				</p>
			</div>

			<div class="features__grid">
				{#each featureCards as card (card.title)}
					<div data-gsap class="feature-card feature-card--{card.variant}">
						<div class="feature-card__icon">
							{#if card.variant === 'primary'}
								<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"
									/></svg
								>
							{:else if card.variant === 'indigo'}
								<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z"
									/></svg
								>
							{:else if card.variant === 'emerald'}
								<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
									/></svg
								>
							{:else if card.variant === 'blue'}
								<IconTrendingUp size={28} stroke={2} aria-hidden="true" />
							{:else if card.variant === 'red'}
								<IconLock size={28} stroke={2} aria-hidden="true" />
							{:else}
								<svg aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"
									/></svg
								>
							{/if}
						</div>
						<h3 class="feature-card__title">{card.title}</h3>
						<p class="feature-card__body">{card.body}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Timeline / execution
	     ───────────────────────────────────────────────────────────── -->
	<section class="timeline">
		<div class="timeline__inner">
			<div class="timeline__header">
				<h2 data-gsap class="timeline__title">Crystal Clear Execution</h2>
				<p data-gsap class="timeline__lede">Follow the lifecycle of a typical trade.</p>
			</div>

			<div class="timeline__rail-wrap">
				<div class="timeline__rail" aria-hidden="true"></div>

				<div data-gsap class="timeline__row timeline__row--left">
					<div class="timeline__copy">
						<h3 class="timeline__step-title">1. The Setup &amp; Entry</h3>
						<p class="timeline__step-body">
							We identify a key gamma level holding. You get the alert instantly with strike, price,
							and risk parameters.
						</p>
					</div>
					<div class="timeline__node timeline__node--emerald"><span>1</span></div>
					<div class="timeline__card-wrap">
						<div class="timeline__card timeline__card--emerald">
							<div class="timeline__card-head">
								<span class="timeline__tag timeline__tag--emerald">Signal</span>
								<span class="timeline__time">09:42 AM</span>
							</div>
							<div class="timeline__card-body">
								<div class="timeline__order">BTO SPX 4580 CALL</div>
								<div class="timeline__order-grid">
									<div class="timeline__order-line">
										Entry: <span class="timeline__order-val">$3.50</span>
									</div>
									<div class="timeline__order-line">
										Stop: <span class="timeline__order-val timeline__order-val--red">$2.10</span>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>

				<div data-gsap class="timeline__row timeline__row--right">
					<div class="timeline__card-wrap">
						<div class="timeline__card timeline__card--blue">
							<div class="timeline__card-head">
								<span class="timeline__tag timeline__tag--blue">Update</span>
								<span class="timeline__time">10:05 AM</span>
							</div>
							<div class="timeline__card-body">
								<div class="timeline__order">TARGET 1 HIT 🎯</div>
								<p class="timeline__update">
									Price at $4.50 (+28%). Trim half size. Move stop on runners to Breakeven.
								</p>
							</div>
						</div>
					</div>
					<div class="timeline__node timeline__node--blue"><span>2</span></div>
					<div class="timeline__copy">
						<h3 class="timeline__step-title">2. Trade Management</h3>
						<p class="timeline__step-body">
							We don't leave you hanging. We send real-time updates to trim profits and protect your
							downside as the trade moves.
						</p>
					</div>
				</div>

				<div data-gsap class="timeline__row timeline__row--left">
					<div class="timeline__copy">
						<h3 class="timeline__step-title">3. Final Exit</h3>
						<p class="timeline__step-body">
							We squeeze the move for maximum gain, exiting runners into strength before reversal.
						</p>
					</div>
					<div class="timeline__node timeline__node--indigo"><span>3</span></div>
					<div class="timeline__card-wrap">
						<div class="timeline__card timeline__card--indigo">
							<div class="timeline__card-head">
								<span class="timeline__tag timeline__tag--indigo">Exit</span>
								<span class="timeline__time">10:45 AM</span>
							</div>
							<div class="timeline__card-body">
								<div class="timeline__order">ALL OUT</div>
								<p class="timeline__update">Sold runners at $7.00.</p>
								<p class="timeline__profit">Total Profit: +100% ✅</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Performance ledger
	     ───────────────────────────────────────────────────────────── -->
	<section id="performance" class="perf">
		<div class="perf__inner">
			<div class="perf__header">
				<div>
					<h2 class="perf__title">Recent Performance</h2>
					<p class="perf__sub">Transparency is our currency. Live log of recent calls.</p>
				</div>
				<a href="/performance" class="perf__view-all">
					View Full Ledger
					<svg
						aria-hidden="true"
						class="perf__arrow"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M17 8l4 4m0 0l-4 4m4-4H3"
						/></svg
					>
				</a>
			</div>

			<div class="perf__table-shell">
				<div class="perf__row perf__row--head">
					<div class="perf__cell perf__cell--date">Date</div>
					<div class="perf__cell perf__cell--ticker">Ticker / Strike</div>
					<div class="perf__cell perf__cell--result">Result</div>
					<div class="perf__cell perf__cell--note">Notes</div>
				</div>
				<div class="perf__row perf__row--body">
					<div class="perf__cell perf__cell--date perf__muted">Nov 15</div>
					<div class="perf__cell perf__cell--ticker perf__ticker">SPX 4560 CALL</div>
					<div class="perf__cell perf__cell--result perf__return--pos">+50%</div>
					<div class="perf__cell perf__cell--note perf__note">Held VWAP perfectly.</div>
				</div>
				<div class="perf__row perf__row--body">
					<div class="perf__cell perf__cell--date perf__muted">Nov 14</div>
					<div class="perf__cell perf__cell--ticker perf__ticker">SPX 4575 PUT</div>
					<div class="perf__cell perf__cell--result perf__return--pos">+50%</div>
					<div class="perf__cell perf__cell--note perf__note">Clean breakdown of 4580.</div>
				</div>
				<div class="perf__row perf__row--body perf__row--loss">
					<div class="perf__cell perf__cell--date perf__muted">Nov 13</div>
					<div class="perf__cell perf__cell--ticker perf__ticker">SPX 4590 CALL</div>
					<div class="perf__cell perf__cell--result perf__return--neg">-31%</div>
					<div class="perf__cell perf__cell--note perf__note">Stopped out. Choppy open.</div>
				</div>
				<div class="perf__row perf__row--body">
					<div class="perf__cell perf__cell--date perf__muted">Nov 12</div>
					<div class="perf__cell perf__cell--ticker perf__ticker">SPX 4555 PUT</div>
					<div class="perf__cell perf__cell--result perf__return--pos">+60%</div>
					<div class="perf__cell perf__cell--note perf__note">Trend day runner.</div>
				</div>
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Pricing
	     ───────────────────────────────────────────────────────────── -->
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

			<div class="pricing__toggle-wrap">
				<div class="pricing__toggle" role="group">
					<button
						type="button"
						onclick={() => (selectedPlan = 'monthly')}
						class="pricing__toggle-btn {selectedPlan === 'monthly'
							? 'pricing__toggle-btn--active'
							: ''}">Monthly</button
					>
					<button
						type="button"
						onclick={() => (selectedPlan = 'quarterly')}
						class="pricing__toggle-btn {selectedPlan === 'quarterly'
							? 'pricing__toggle-btn--active'
							: ''}">Quarterly</button
					>
					<button
						type="button"
						onclick={() => (selectedPlan = 'annual')}
						class="pricing__toggle-btn {selectedPlan === 'annual'
							? 'pricing__toggle-btn--active'
							: ''}">Annual</button
					>

					<div
						class="pricing__toggle-pill"
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
						<div
							class="plan plan--simple {selectedPlan === plan.id ? 'plan--active' : 'plan--dim'}"
						>
							<h3 class="plan__label">{plan.label}</h3>
							<div class="plan__price-row">
								<span class="plan__price">${plan.price}</span>
								<span class="plan__period">{plan.period}</span>
							</div>
							<div class="plan__perday">{plan.perDay}</div>
							<ul class="plan__features">
								{#each plan.features as feature, i (i)}
									<li><span class="plan__check">✓</span> {feature}</li>
								{/each}
							</ul>
							<a href={plan.checkoutHref} class="plan__cta plan__cta--ghost">
								Select {plan.label}
							</a>
						</div>
					{:else if plan.variant === 'featured'}
						<div
							class="plan plan--featured {selectedPlan === plan.id ? 'plan--active' : 'plan--dim'}"
						>
							<div class="plan__ribbon">Most Popular</div>
							<h3 class="plan__label plan__label--large">{plan.label}</h3>
							<div class="plan__price-row">
								<span class="plan__price plan__price--large">${plan.price}</span>
								<span class="plan__period">{plan.period}</span>
							</div>
							<div class="plan__savings plan__savings--emerald">{plan.savingsCopy}</div>
							<ul class="plan__features plan__features--bold">
								{#each plan.features as feature, i (i)}
									<li>
										<span class="plan__check plan__check--bold">✓</span>
										<span>{feature}</span>
									</li>
								{/each}
							</ul>
							<a href={plan.checkoutHref} class="plan__cta plan__cta--primary">
								Join {plan.label}
							</a>
						</div>
					{:else if plan.variant === 'highlight'}
						<div
							class="plan plan--highlight {selectedPlan === plan.id ? 'plan--active' : 'plan--dim'}"
						>
							<div class="plan__ribbon plan__ribbon--emerald">Best Deal</div>
							<h3 class="plan__label">{plan.label}</h3>
							<div class="plan__price-row">
								<span class="plan__price">${plan.price}</span>
								<span class="plan__period">{plan.period}</span>
							</div>
							<div class="plan__savings">{plan.savingsCopy}</div>
							<ul class="plan__features">
								{#each plan.features as feature, i (i)}
									<li>
										<span class="plan__check">✓</span>
										<span>{feature}</span>
									</li>
								{/each}
							</ul>
							<a href={plan.checkoutHref} class="plan__cta plan__cta--emerald">
								Select {plan.label}
							</a>
						</div>
					{/if}
				{/each}
			</div>

			<div class="pricing__secure">
				<p class="pricing__secure-line">
					<IconLock size={16} stroke={2} aria-hidden="true" />
					Secure checkout powered by Stripe. Cancel anytime.
				</p>
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     FAQ
	     ───────────────────────────────────────────────────────────── -->
	<section class="faq">
		<div class="faq__inner">
			<h2 class="faq__title">Frequently Asked Questions</h2>
			<div class="faq__list">
				{#each faqList as faq, i (i)}
					<div class="faq__item">
						<button
							type="button"
							class="faq__trigger"
							onclick={() => toggleFaq(i)}
							aria-expanded={openFaq === i}
						>
							{faq.question}
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
								{faq.answer}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- ─────────────────────────────────────────────────────────────
	     Bottom CTA
	     ───────────────────────────────────────────────────────────── -->
	<section class="bottom-cta">
		<div class="bottom-cta__bg" aria-hidden="true"></div>
		<div class="bottom-cta__inner">
			<h2 class="bottom-cta__title">Ready to Level Up?</h2>
			<p class="bottom-cta__lede">
				Join the ranks of professional traders capturing daily alpha in the SPX.
			</p>
			<div class="bottom-cta__row">
				<a href="#pricing" class="bottom-cta__button">Get Access Now</a>
			</div>
			<p class="bottom-cta__legal">30-Day Money Back Guarantee on Annual Plans</p>
		</div>
	</section>
</main>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Page-local tokens (one-offs for this page only).
	   Reusable color/spacing values come from --rtp-* (marketing.css).
	   Note on muted-text token choice: this page already imports
	   project-local .text-rtp-muted via the legacy --rtp-muted shade
	   (#6b7280) in places, but the dominant body color in the original
	   markup was the same project-local muted utility (used uniformly
	   across hero lede, signal meta, table rows). Page is being
	   converted to scoped CSS so we route everything through the more
	   widely used --rtp-text-muted (#94a3b8, slate-400) for visual
	   consistency with the rest of the marketing surface. The single
	   spot where --rtp-muted was intentional was the inactive plan
	   toggle text; kept on --rtp-text-muted because the new pill
	   indicator already conveys active/inactive contrast.
	   ───────────────────────────────────────────────────────────────── */
	.spx {
		--spx-emerald-bright: #34d399; /* emerald-400 */
		--spx-emerald-deep: #059669; /* emerald-600, badge value */
		--spx-blue-hover: #2563eb; /* blue-600 — CTA hover */
		--spx-blue-100: #dbeafe; /* blue-100 — bottom CTA lede */
		--spx-blue-50: #eff6ff; /* blue-50 — bottom CTA hover */
		--spx-red-soft: #f87171; /* red-400 */
		--spx-red-mid: #ef4444; /* red-500 */
		--spx-indigo-mid: #6366f1; /* indigo-500 */
		--spx-purple-bright: #c084fc; /* purple-400 */
		--spx-purple-mid: #a855f7; /* purple-500 */

		width: 100%;
		overflow-x: hidden;
		background: var(--rtp-bg);
		color: var(--rtp-text);
		font-family: var(--rtp-font-sans);
	}
	.spx ::selection {
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
	}
	.hero__grid {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(rgba(255, 255, 255, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(255, 255, 255, 0.03) 1px, transparent 1px);
		background-size: 64px 64px;
		mask-image: radial-gradient(ellipse 60% 50% at 50% 0%, #000 70%, transparent 100%);
	}
	.hero__halo {
		position: absolute;
		border-radius: 50%;
	}
	.hero__halo--primary {
		inset-block-start: -20%;
		inset-inline-end: -10%;
		width: 800px;
		height: 800px;
		background: color-mix(in oklab, var(--rtp-primary) 10%, transparent);
		filter: blur(120px);
		animation: spx-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
	.hero__halo--indigo {
		inset-block-end: -10%;
		inset-inline-start: -10%;
		width: 600px;
		height: 600px;
		background: color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
		filter: blur(100px);
	}
	@keyframes spx-pulse {
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
			grid-template-columns: 1fr 1fr;
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

	.hero__badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 1rem;
		border-radius: var(--rtp-radius-pill);
		background: var(--rtp-surface);
		border: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
		box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
		backdrop-filter: blur(4px);
		align-self: center;
	}
	@media (min-width: 1024px) {
		.hero__badge {
			align-self: flex-start;
		}
	}
	.hero__pulse {
		position: relative;
		display: flex;
		width: 0.625rem;
		height: 0.625rem;
	}
	.hero__pulse-ping {
		position: absolute;
		inset: 0;
		display: inline-flex;
		border-radius: 50%;
		background: var(--spx-emerald-bright);
		opacity: 0.75;
		animation: spx-ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
	}
	.hero__pulse-dot {
		position: relative;
		display: inline-flex;
		width: 100%;
		height: 100%;
		border-radius: 50%;
		background: var(--rtp-emerald);
	}
	@keyframes spx-ping {
		75%,
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}
	.hero__badge-label {
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: var(--rtp-text-muted);
	}

	.hero__title {
		font-family: var(--rtp-font-display);
		font-size: 3rem;
		font-weight: 800;
		line-height: 1.1;
		letter-spacing: -0.025em;
		color: var(--rtp-text);
	}
	@media (min-width: 768px) {
		.hero__title {
			font-size: 4.5rem;
		}
	}
	.hero__title-accent {
		background: linear-gradient(to right, var(--rtp-primary), var(--rtp-blue), var(--rtp-emerald));
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}

	.hero__lede {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
		max-width: 42rem;
		line-height: 1.625;
		margin-inline: auto;
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
		padding-block-start: 1rem;
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
	}
	.hero__cta-primary {
		position: relative;
		color: var(--rtp-text);
		background: var(--rtp-primary);
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
	}
	.hero__cta-primary:hover {
		background: var(--spx-blue-hover);
		transform: translateY(-4px);
		box-shadow: 0 10px 15px -3px color-mix(in oklab, var(--rtp-primary) 25%, transparent);
	}
	.hero__cta-primary:focus-visible {
		outline: 2px solid var(--rtp-primary);
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
		color: var(--rtp-text);
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border);
	}
	.hero__cta-secondary:hover {
		background: color-mix(in oklab, var(--rtp-surface) 80%, transparent);
		border-color: color-mix(in oklab, var(--rtp-primary) 30%, transparent);
	}

	.hero__trust {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1.5rem;
		padding-block-start: 1rem;
		font-size: 0.875rem;
		color: color-mix(in oklab, var(--rtp-text-muted) 60%, transparent);
		font-weight: 500;
	}
	@media (min-width: 1024px) {
		.hero__trust {
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
	.hero__preview-wrap {
		position: relative;
		display: none;
		perspective: 1000px;
	}
	@media (min-width: 1024px) {
		.hero__preview-wrap {
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
		color: #fff;
		font-weight: 700;
		box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
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
	.hero__signal--blue {
		border-inline-start-color: var(--rtp-blue);
	}
	.hero__signal--dim {
		opacity: 0.6;
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
	.hero__signal-label--blue {
		color: var(--rtp-blue);
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
		color: #fff;
	}
	.hero__signal-targets {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: var(--rtp-text-muted);
	}
	.hero__signal-update {
		font-size: 0.875rem;
		color: var(--rtp-text);
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
		animation: spx-bounce 2s infinite;
	}
	@keyframes spx-bounce {
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
		color: var(--spx-emerald-deep);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Stat strip
	   ───────────────────────────────────────────────────────────────── */
	.stat-strip {
		background: var(--rtp-surface);
		border-block: 1px solid var(--rtp-border);
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
		transition: transform var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.stat-strip__item:hover {
		transform: scale(1.1);
	}
	.stat-strip__value {
		font-size: 1.875rem;
		font-weight: 800;
		color: var(--rtp-text);
		margin-block-end: 0.5rem;
	}
	@media (min-width: 768px) {
		.stat-strip__value {
			font-size: 3rem;
		}
	}
	.stat-strip__value--emerald {
		background: linear-gradient(
			to bottom right,
			var(--spx-emerald-bright),
			var(--spx-emerald-deep)
		);
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
	}
	.stat-strip__label {
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: var(--rtp-text-muted);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Features grid
	   ───────────────────────────────────────────────────────────────── */
	.features {
		padding-block: 8rem;
		background: var(--rtp-bg);
		position: relative;
		overflow: hidden;
	}
	.features__bg {
		position: absolute;
		inset: 0;
		opacity: 0.02;
		background-image: url('/grid-pattern.svg');
	}
	.features__inner {
		position: relative;
		z-index: 10;
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding-inline: 1rem;
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
		color: var(--rtp-text-muted);
	}
	.features__grid {
		display: grid;
		gap: 2rem;
	}
	@media (min-width: 768px) {
		.features__grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}
	@media (min-width: 1024px) {
		.features__grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	.feature-card {
		background: var(--rtp-surface);
		padding: 2rem;
		border-radius: 1rem;
		border: 1px solid var(--rtp-border);
		transition:
			border-color var(--rtp-dur-base) var(--rtp-ease-out),
			transform var(--rtp-dur-base) var(--rtp-ease-out),
			box-shadow var(--rtp-dur-base) var(--rtp-ease-out);
	}
	.feature-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}
	.feature-card--primary:hover {
		border-color: color-mix(in oklab, var(--rtp-primary) 50%, transparent);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--rtp-primary) 10%, transparent);
	}
	.feature-card--indigo:hover {
		border-color: color-mix(in oklab, var(--rtp-indigo) 50%, transparent);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
	}
	.feature-card--emerald:hover {
		border-color: color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
	}
	.feature-card--blue:hover {
		border-color: color-mix(in oklab, var(--rtp-blue) 50%, transparent);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--rtp-blue) 10%, transparent);
	}
	.feature-card--red:hover {
		border-color: color-mix(in oklab, var(--spx-red-soft) 50%, transparent);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--spx-red-soft) 10%, transparent);
	}
	.feature-card--purple:hover {
		border-color: color-mix(in oklab, var(--spx-purple-bright) 50%, transparent);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--spx-purple-bright) 10%, transparent);
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
	.feature-card--indigo .feature-card__icon {
		background: color-mix(in oklab, var(--rtp-indigo) 10%, transparent);
		color: var(--rtp-indigo);
	}
	.feature-card--emerald .feature-card__icon {
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		color: var(--rtp-emerald);
	}
	.feature-card--blue .feature-card__icon {
		background: color-mix(in oklab, var(--rtp-blue) 10%, transparent);
		color: var(--rtp-blue);
	}
	.feature-card--red .feature-card__icon {
		background: color-mix(in oklab, var(--spx-red-mid) 10%, transparent);
		color: var(--spx-red-soft);
	}
	.feature-card--purple .feature-card__icon {
		background: color-mix(in oklab, var(--spx-purple-mid) 10%, transparent);
		color: var(--spx-purple-bright);
	}
	.feature-card__title {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.75rem;
	}
	.feature-card__body {
		color: var(--rtp-text-muted);
		line-height: 1.625;
		font-size: 0.875rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Execution timeline
	   ───────────────────────────────────────────────────────────────── */
	.timeline {
		padding-block: 6rem;
		background: var(--rtp-surface);
		border-block: 1px solid var(--rtp-border);
	}
	.timeline__inner {
		max-width: 64rem;
		margin-inline: auto;
		padding-inline: 1rem;
	}
	@media (min-width: 640px) {
		.timeline__inner {
			padding-inline: 1.5rem;
		}
	}
	@media (min-width: 1024px) {
		.timeline__inner {
			padding-inline: 2rem;
		}
	}
	.timeline__header {
		text-align: center;
		margin-block-end: 4rem;
	}
	.timeline__title {
		font-family: var(--rtp-font-display);
		font-size: 1.875rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1rem;
	}
	@media (min-width: 768px) {
		.timeline__title {
			font-size: 3rem;
		}
	}
	.timeline__lede {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
	}

	.timeline__rail-wrap {
		position: relative;
	}
	.timeline__rail {
		position: absolute;
		inset-block: 0;
		inset-inline-start: 2rem;
		width: 2px;
		background: linear-gradient(to bottom, var(--rtp-emerald), var(--rtp-blue), var(--rtp-border));
	}
	@media (min-width: 768px) {
		.timeline__rail {
			inset-inline-start: 50%;
		}
	}

	.timeline__row {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-between;
		margin-block-end: 4rem;
	}
	.timeline__row:last-child {
		margin-block-end: 0;
	}
	@media (min-width: 768px) {
		.timeline__row {
			flex-direction: row;
		}
	}

	.timeline__copy {
		width: 100%;
		padding-inline-start: 4rem;
		margin-block-end: 1rem;
		order: 1;
	}
	@media (min-width: 768px) {
		.timeline__copy {
			width: 45%;
			padding-inline-start: 0;
			margin-block-end: 0;
		}
		.timeline__row--left .timeline__copy {
			text-align: end;
			padding-inline-end: 2rem;
			order: 1;
		}
		.timeline__row--right .timeline__copy {
			padding-inline-start: 2rem;
			order: 2;
		}
	}

	.timeline__step-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 0.5rem;
	}
	.timeline__step-body {
		color: var(--rtp-text-muted);
	}

	.timeline__node {
		position: absolute;
		inset-block-start: 0;
		inset-inline-start: 2rem;
		translate: -50% 0;
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		background: var(--rtp-bg);
		border: 4px solid;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
	}
	@media (min-width: 768px) {
		.timeline__node {
			inset-inline-start: 50%;
		}
	}
	.timeline__node--emerald {
		border-color: var(--rtp-emerald);
		box-shadow: 0 0 20px color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
	}
	.timeline__node--emerald span {
		color: var(--rtp-emerald);
	}
	.timeline__node--blue {
		border-color: var(--rtp-blue);
		box-shadow: 0 0 20px color-mix(in oklab, var(--rtp-primary) 50%, transparent);
	}
	.timeline__node--blue span {
		color: var(--rtp-blue);
	}
	.timeline__node--indigo {
		border-color: var(--spx-indigo-mid);
		box-shadow: 0 0 20px color-mix(in oklab, var(--rtp-indigo) 50%, transparent);
	}
	.timeline__node--indigo span {
		color: var(--spx-indigo-mid);
	}

	.timeline__card-wrap {
		width: 100%;
		padding-inline-start: 4rem;
		order: 2;
	}
	@media (min-width: 768px) {
		.timeline__card-wrap {
			width: 45%;
			padding-inline-start: 0;
		}
		.timeline__row--left .timeline__card-wrap {
			padding-inline-start: 2rem;
			order: 2;
		}
		.timeline__row--right .timeline__card-wrap {
			padding-inline-end: 2rem;
			order: 1;
		}
	}

	.timeline__card {
		background: var(--rtp-bg);
		padding: 1.5rem;
		border-radius: var(--rtp-radius-md);
		border-inline-start: 4px solid;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
	}
	.timeline__card--emerald {
		border-inline-start-color: var(--rtp-emerald);
	}
	.timeline__card--blue {
		border-inline-start-color: var(--rtp-blue);
	}
	.timeline__card--indigo {
		border-inline-start-color: var(--spx-indigo-mid);
	}

	.timeline__card-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: 0.75rem;
	}
	.timeline__tag {
		font-size: 0.625rem;
		font-weight: 700;
		padding: 0.25rem 0.5rem;
		border-radius: var(--rtp-radius-sm);
		text-transform: uppercase;
	}
	.timeline__tag--emerald {
		background: color-mix(in oklab, var(--rtp-emerald) 20%, transparent);
		color: var(--spx-emerald-bright);
	}
	.timeline__tag--blue {
		background: color-mix(in oklab, var(--rtp-blue) 20%, transparent);
		color: var(--rtp-blue-bright);
	}
	.timeline__tag--indigo {
		background: color-mix(in oklab, var(--rtp-indigo) 20%, transparent);
		color: #818cf8; /* indigo-400 */
	}
	.timeline__time {
		font-family: var(--rtp-font-mono);
		font-size: 0.75rem;
		color: var(--rtp-text-muted);
	}
	.timeline__card-body {
		font-family: var(--rtp-font-mono);
		font-size: 0.875rem;
	}
	.timeline__order {
		font-weight: 700;
		font-size: 1.125rem;
		color: #fff;
	}
	.timeline__order-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.5rem;
		margin-block-start: 0.5rem;
		font-size: 0.75rem;
	}
	.timeline__order-line {
		color: var(--rtp-text-muted);
	}
	.timeline__order-val {
		color: #fff;
		font-weight: 700;
	}
	.timeline__order-val--red {
		color: var(--spx-red-soft);
	}
	.timeline__update {
		font-size: 0.75rem;
		color: var(--rtp-text-muted);
		margin-block-start: 0.25rem;
	}
	.timeline__profit {
		color: var(--spx-emerald-bright);
		font-weight: 700;
		margin-block-start: 0.5rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Performance ledger
	   ───────────────────────────────────────────────────────────────── */
	.perf {
		padding-block: 6rem;
		background: var(--rtp-bg);
		position: relative;
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
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: flex-end;
		margin-block-end: 3rem;
		gap: 1.5rem;
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
	@media (min-width: 768px) {
		.perf__title {
			font-size: 2.25rem;
		}
	}
	.perf__sub {
		color: var(--rtp-text-muted);
	}
	.perf__view-all {
		color: var(--rtp-primary);
		font-weight: 700;
		text-decoration: none;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.perf__view-all:hover {
		color: #fff;
	}
	.perf__arrow {
		width: 1rem;
		height: 1rem;
	}

	.perf__table-shell {
		background: var(--rtp-surface);
		border-radius: 1rem;
		border: 1px solid var(--rtp-border);
		overflow: hidden;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
	}
	.perf__row {
		display: grid;
		grid-template-columns: repeat(12, 1fr);
		align-items: center;
		padding: 1rem;
		border-block-end: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
		transition: background-color var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.perf__row:last-child {
		border-block-end: 0;
	}
	.perf__row--head {
		background: color-mix(in oklab, var(--rtp-bg) 50%, transparent);
		border-block-end: 1px solid var(--rtp-border);
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--rtp-text-muted);
	}
	.perf__row--body {
		font-family: var(--rtp-font-mono);
		font-size: 0.875rem;
	}
	.perf__row--body:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}
	.perf__row--loss {
		background: color-mix(in oklab, var(--spx-red-mid) 5%, transparent);
	}

	.perf__cell--date {
		grid-column: span 3 / span 3;
	}
	@media (min-width: 768px) {
		.perf__cell--date {
			grid-column: span 2 / span 2;
		}
	}
	.perf__cell--ticker {
		grid-column: span 5 / span 5;
	}
	@media (min-width: 768px) {
		.perf__cell--ticker {
			grid-column: span 4 / span 4;
		}
	}
	.perf__cell--result {
		grid-column: span 4 / span 4;
		text-align: end;
	}
	@media (min-width: 768px) {
		.perf__cell--result {
			grid-column: span 2 / span 2;
		}
	}
	.perf__cell--note {
		display: none;
	}
	@media (min-width: 768px) {
		.perf__cell--note {
			display: block;
			grid-column: span 4 / span 4;
			text-align: end;
		}
	}
	.perf__muted {
		color: var(--rtp-text-muted);
	}
	.perf__ticker {
		font-weight: 700;
		color: #fff;
	}
	.perf__return--pos {
		color: var(--spx-emerald-bright);
		font-weight: 700;
	}
	.perf__return--neg {
		color: var(--spx-red-soft);
		font-weight: 700;
	}
	.perf__note {
		color: var(--rtp-text-muted);
		font-size: 0.75rem;
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
		color: var(--rtp-text-muted);
		max-width: 42rem;
		margin-inline: auto;
	}

	.pricing__toggle-wrap {
		display: flex;
		justify-content: center;
		margin-block-end: 4rem;
	}
	.pricing__toggle {
		background: var(--rtp-bg);
		padding: 0.375rem;
		border-radius: var(--rtp-radius-md);
		border: 1px solid var(--rtp-border);
		display: inline-flex;
		position: relative;
		box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
	}
	.pricing__toggle-btn {
		position: relative;
		z-index: 10;
		padding: 0.5rem 1.5rem;
		border-radius: var(--rtp-radius-sm);
		font-weight: 700;
		font-size: 0.875rem;
		transition: color var(--rtp-dur-fast) var(--rtp-ease-out);
		background: transparent;
		border: 0;
		cursor: pointer;
		color: var(--rtp-text-muted);
		font-family: inherit;
	}
	.pricing__toggle-btn:hover {
		color: #fff;
	}
	.pricing__toggle-btn--active {
		color: #fff;
	}
	.pricing__toggle-pill {
		position: absolute;
		inset-block: 0.375rem;
		background: var(--rtp-primary);
		border-radius: var(--rtp-radius-sm);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
		transition: all 0.3s cubic-bezier(0.25, 0.1, 0.25, 1);
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
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	.plan {
		background: var(--rtp-bg);
		padding: 2rem;
		border-radius: 1rem;
		border: 1px solid var(--rtp-border);
		transition: all 0.3s ease;
		position: relative;
	}
	.plan--featured {
		padding: 2.5rem;
		border-width: 2px;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
		z-index: 10;
	}
	.plan--active {
		opacity: 1;
	}
	.plan--dim {
		opacity: 0.7;
	}
	.plan--dim:hover {
		opacity: 1;
	}
	.plan--simple.plan--active {
		border-color: var(--rtp-primary);
		transform: scale(1.05);
		box-shadow: 0 20px 25px -5px color-mix(in oklab, var(--rtp-primary) 10%, transparent);
	}
	.plan--featured.plan--active {
		border-color: var(--rtp-primary);
		box-shadow: 0 25px 50px -12px color-mix(in oklab, var(--rtp-primary) 20%, transparent);
	}
	@media (min-width: 768px) {
		.plan--featured.plan--active {
			transform: scale(1.1);
		}
	}
	.plan--highlight.plan--active {
		border-color: var(--rtp-emerald);
		transform: scale(1.05);
		box-shadow: 0 20px 25px -5px color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
	}

	.plan__ribbon {
		position: absolute;
		inset-block-start: 0;
		inset-inline-start: 50%;
		translate: -50% -50%;
		background: var(--rtp-primary);
		color: #fff;
		padding: 0.25rem 1rem;
		border-radius: var(--rtp-radius-pill);
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
		white-space: nowrap;
		z-index: 10;
	}
	.plan__ribbon--emerald {
		background: var(--rtp-emerald);
		inset-block-start: -0.75rem;
		translate: -50% 0;
	}

	.plan__label {
		font-size: 1.25rem;
		font-weight: 700;
		color: #fff;
		margin-block-end: 1rem;
	}
	.plan__label--large {
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
	.plan__price--large {
		font-size: 3rem;
		font-weight: 800;
	}
	.plan__period {
		color: var(--rtp-text-muted);
	}
	.plan__perday {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		color: var(--rtp-text-muted);
		background: var(--rtp-surface);
		padding: 0.5rem;
		border-radius: var(--rtp-radius-sm);
		margin-block-end: 1.5rem;
		text-align: center;
		border: 1px solid var(--rtp-border);
	}
	.plan__savings {
		font-size: 0.75rem;
		font-family: var(--rtp-font-mono);
		padding: 0.5rem;
		border-radius: var(--rtp-radius-sm);
		margin-block-end: 1.5rem;
		text-align: center;
		color: var(--rtp-emerald);
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border);
	}
	.plan__savings--emerald {
		color: var(--spx-emerald-bright);
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		border-color: color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
	}
	.plan__features {
		list-style: none;
		margin: 0 0 2rem;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
	}
	.plan__features--bold {
		color: #fff;
	}
	.plan__features li {
		display: flex;
		gap: 0.75rem;
	}
	.plan__features--bold li span:last-child {
		font-weight: 700;
	}
	.plan__check {
		color: var(--rtp-primary);
	}
	.plan__check--bold {
		font-weight: 700;
	}

	.plan__cta {
		display: block;
		width: 100%;
		padding-block: 0.75rem;
		text-align: center;
		font-weight: 700;
		text-decoration: none;
		border-radius: var(--rtp-radius-md);
		transition: all var(--rtp-dur-fast) var(--rtp-ease-out);
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
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
		border-radius: var(--rtp-radius-md);
	}
	.plan__cta--primary:hover {
		background: var(--spx-blue-hover);
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

	.pricing__secure {
		margin-block-start: 3rem;
		text-align: center;
	}
	.pricing__secure-line {
		color: var(--rtp-text-muted);
		font-size: 0.875rem;
		display: inline-flex;
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
		font: inherit;
		cursor: pointer;
		transition: background var(--rtp-dur-fast) var(--rtp-ease-out);
	}
	.faq__trigger:hover {
		background: color-mix(in oklab, #fff 5%, transparent);
	}
	.faq__trigger:focus-visible {
		outline: 2px solid var(--rtp-primary);
		outline-offset: -2px;
	}
	.faq__chevron {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--rtp-text-muted);
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
		border-block-start: 1px solid color-mix(in oklab, var(--rtp-border) 50%, transparent);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Bottom CTA
	   ───────────────────────────────────────────────────────────────── */
	.bottom-cta {
		padding-block: 6rem;
		position: relative;
		overflow: hidden;
	}
	.bottom-cta__bg {
		position: absolute;
		inset: 0;
		background: linear-gradient(to bottom right, var(--rtp-primary), var(--rtp-indigo));
		z-index: 0;
	}
	.bottom-cta__bg::after {
		content: '';
		position: absolute;
		inset: 0;
		background-image: url('/grid-pattern.svg');
		opacity: 0.1;
	}
	.bottom-cta__inner {
		position: relative;
		z-index: 10;
		max-width: 56rem;
		margin-inline: auto;
		padding-inline: 1rem;
		text-align: center;
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
		color: #fff;
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
		color: var(--spx-blue-100);
		margin-block-end: 2.5rem;
		max-width: 42rem;
		margin-inline: auto;
	}
	.bottom-cta__row {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		justify-content: center;
	}
	@media (min-width: 640px) {
		.bottom-cta__row {
			flex-direction: row;
		}
	}
	.bottom-cta__button {
		background: #fff;
		color: var(--rtp-primary);
		padding: 1rem 2.5rem;
		border-radius: var(--rtp-radius-md);
		font-weight: 700;
		font-size: 1.125rem;
		text-decoration: none;
		transition: all var(--rtp-dur-fast) var(--rtp-ease-out);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}
	.bottom-cta__button:hover {
		background: var(--spx-blue-50);
		transform: translateY(-4px);
	}
	.bottom-cta__legal {
		margin-block-start: 2rem;
		font-size: 0.875rem;
		color: color-mix(in oklab, #fff 60%, transparent);
	}
</style>
