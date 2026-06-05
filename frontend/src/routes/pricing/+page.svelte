<!--
	URL: /pricing
-->
<script lang="ts">
	import { IconArrowRight, IconCheck } from '$lib/icons';

	const plans = [
		{
			name: 'Starter',
			price: 97,
			period: 'month',
			description: 'Perfect for traders getting started with professional education.',
			features: [
				'1 Trading Room Access',
				'Weekly Watchlist',
				'Daily Alerts',
				'Community Access',
				'Mobile App Access'
			],
			cta: 'Get Started',
			href: '/signup',
			highlighted: false
		},
		{
			name: 'Pro',
			price: 197,
			period: 'month',
			description: 'For serious traders who want every edge available.',
			features: [
				'All 6 Trading Rooms',
				'Weekly Watchlist',
				'Daily Alerts',
				'Premium Indicators (3)',
				'Live Trading Sessions',
				'1-on-1 Mentorship (2/mo)',
				'Community Access',
				'Mobile App Access'
			],
			cta: 'Go Pro',
			href: '/signup?plan=pro',
			highlighted: true
		},
		{
			name: 'Elite',
			price: 397,
			period: 'month',
			description: 'The complete package for traders who want it all.',
			features: [
				'All 6 Trading Rooms',
				'Weekly Watchlist',
				'Daily Alerts',
				'All Premium Indicators',
				'Live Trading Sessions',
				'Unlimited Mentorship',
				'Private Elite Community',
				'Early Access to New Tools',
				'Direct Trader Line'
			],
			cta: 'Go Elite',
			href: '/signup?plan=elite',
			highlighted: false
		}
	];

	const faqs = [
		{
			q: 'Can I cancel anytime?',
			a: 'Yes. Cancel anytime from your dashboard — no contracts, no cancellation fees.'
		},
		{
			q: 'Is there a free trial?',
			a: "We offer a 7-day money-back guarantee on all plans. If you're not satisfied, contact support within 7 days for a full refund."
		},
		{
			q: 'What payment methods do you accept?',
			a: 'We accept all major credit and debit cards (Visa, Mastercard, Amex, Discover) as well as Apple Pay and Google Pay.'
		},
		{
			q: 'Can I upgrade or downgrade my plan?',
			a: 'Absolutely. You can change your plan at any time from your dashboard. Upgrades take effect immediately; downgrades apply at the next billing cycle.'
		},
		{
			q: 'Are the trading rooms live every day?',
			a: 'Rooms are live during US market hours (9:30 AM – 4:00 PM ET) Monday through Friday, excluding major holidays.'
		}
	];
</script>

<svelte:head>
	<title>Pricing | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Simple, transparent pricing for every level of trader. Start with Starter, grow with Pro, or unlock everything with Elite."
	/>
</svelte:head>

<div class="pricing-page">
	<!-- Hero -->
	<section class="pricing-hero">
		<div class="pricing-hero__inner">
			<p class="pricing-hero__eyebrow">Transparent Pricing</p>
			<h1 class="pricing-hero__title">Invest in Your Trading Edge</h1>
			<p class="pricing-hero__subtitle">No hidden fees. No long-term contracts. Cancel anytime.</p>
		</div>
	</section>

	<!-- Plans -->
	<section class="plans-section">
		<div class="plans-grid">
			{#each plans as plan (plan.name)}
				<div class="plan-card" class:plan-card--highlighted={plan.highlighted}>
					{#if plan.highlighted}
						<div class="plan-card__badge">★ Most Popular</div>
					{/if}
					<div class="plan-card__header">
						<h2 class="plan-card__name">{plan.name}</h2>
						<div class="plan-card__price">
							<span class="plan-card__currency">$</span>
							<span class="plan-card__amount">{plan.price}</span>
							<span class="plan-card__period">/ {plan.period}</span>
						</div>
						<p class="plan-card__desc">{plan.description}</p>
					</div>
					<ul class="plan-card__features">
						{#each plan.features as feature (feature)}
							<li class="plan-card__feature">
								<IconCheck size={16} />
								{feature}
							</li>
						{/each}
					</ul>
					<a
						href={plan.href}
						class="plan-card__cta"
						class:plan-card__cta--primary={plan.highlighted}
					>
						{plan.cta}
						<IconArrowRight size={16} />
					</a>
				</div>
			{/each}
		</div>
	</section>

	<!-- FAQ -->
	<section class="faq-section">
		<div class="faq-inner">
			<h2 class="faq-title">Frequently Asked Questions</h2>
			<div class="faq-list">
				{#each faqs as faq (faq.q)}
					<div class="faq-item">
						<h3 class="faq-q">{faq.q}</h3>
						<p class="faq-a">{faq.a}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Final CTA -->
	<section class="pricing-cta">
		<div class="pricing-cta__inner">
			<h2 class="pricing-cta__title">Ready to trade with an edge?</h2>
			<p class="pricing-cta__sub">
				Join thousands of traders already using Revolution Trading Pros.
			</p>
			<a href="/signup" class="pricing-cta__btn">Start Today <IconArrowRight size={18} /></a>
		</div>
	</section>
</div>

<style>
	.pricing-page {
		background: var(--rtp-bg);
		color: var(--rtp-text);
		min-height: 100vh;
	}

	/* ── Hero ── */
	.pricing-hero {
		padding: 6rem 1.5rem 4rem;
		text-align: center;
		background: radial-gradient(
			ellipse 80% 50% at 50% 0%,
			rgba(99, 102, 241, 0.12) 0%,
			transparent 70%
		);
		border-bottom: 1px solid var(--rtp-border-soft);
	}
	.pricing-hero__inner {
		max-width: 640px;
		margin: 0 auto;
	}
	.pricing-hero__eyebrow {
		font-size: 0.8125rem;
		font-weight: 600;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--rtp-indigo);
		margin: 0 0 1rem;
	}
	.pricing-hero__title {
		font-size: clamp(2rem, 5vw, 3rem);
		font-weight: 800;
		color: var(--rtp-text);
		margin: 0 0 1rem;
		line-height: 1.15;
	}
	.pricing-hero__subtitle {
		font-size: 1.125rem;
		color: var(--rtp-text-muted);
		margin: 0;
		line-height: 1.6;
	}

	/* ── Plans ── */
	.plans-section {
		padding: 5rem 1.5rem;
		max-width: 1200px;
		margin: 0 auto;
	}
	.plans-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1.5rem;
		align-items: start;
	}
	.plan-card {
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border-soft);
		border-radius: 1rem;
		padding: 2rem;
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		transition: border-color 0.2s ease;
	}
	.plan-card:hover {
		border-color: var(--rtp-border);
	}
	.plan-card--highlighted {
		border-color: var(--rtp-indigo);
		background: linear-gradient(160deg, rgba(99, 102, 241, 0.08) 0%, var(--rtp-surface) 60%);
		box-shadow: 0 0 40px rgba(99, 102, 241, 0.15);
	}
	.plan-card__badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		background: var(--rtp-indigo);
		color: #fff;
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.05em;
		padding: 0.3rem 0.75rem;
		border-radius: 9999px;
		position: absolute;
		top: -0.75rem;
		left: 50%;
		transform: translateX(-50%);
		white-space: nowrap;
	}
	.plan-card__name {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin: 0 0 0.75rem;
	}
	.plan-card__price {
		display: flex;
		align-items: baseline;
		gap: 0.125rem;
		margin-bottom: 0.75rem;
	}
	.plan-card__currency {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--rtp-text-muted);
	}
	.plan-card__amount {
		font-size: 3rem;
		font-weight: 800;
		color: var(--rtp-text);
		line-height: 1;
	}
	.plan-card__period {
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
	}
	.plan-card__desc {
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
		margin: 0;
		line-height: 1.5;
	}
	.plan-card__features {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
		flex: 1;
	}
	.plan-card__feature {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
	}
	.plan-card__feature :global(svg) {
		color: var(--rtp-emerald);
		flex-shrink: 0;
	}
	.plan-card--highlighted .plan-card__feature :global(svg) {
		color: var(--rtp-indigo);
	}
	.plan-card__cta {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		font-weight: 600;
		text-decoration: none;
		background: var(--rtp-surface-elev);
		border: 1px solid var(--rtp-border);
		color: var(--rtp-text);
		transition:
			background 0.2s ease,
			border-color 0.2s ease;
	}
	.plan-card__cta:hover {
		background: rgba(255, 255, 255, 0.08);
		border-color: var(--rtp-border-strong);
	}
	.plan-card__cta--primary {
		background: var(--rtp-indigo);
		border-color: var(--rtp-indigo);
		color: #fff;
	}
	.plan-card__cta--primary:hover {
		background: #4f52e0;
		border-color: #4f52e0;
	}

	/* ── FAQ ── */
	.faq-section {
		padding: 5rem 1.5rem;
		border-top: 1px solid var(--rtp-border-soft);
		background: var(--rtp-bg-deeper);
	}
	.faq-inner {
		max-width: 720px;
		margin: 0 auto;
	}
	.faq-title {
		font-size: 2rem;
		font-weight: 800;
		color: var(--rtp-text);
		margin: 0 0 3rem;
		text-align: center;
	}
	.faq-list {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}
	.faq-item {
		padding: 1.5rem;
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border-soft);
		border-radius: 0.75rem;
	}
	.faq-q {
		font-size: 1rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin: 0 0 0.5rem;
	}
	.faq-a {
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
		margin: 0;
		line-height: 1.6;
	}

	/* ── Final CTA ── */
	.pricing-cta {
		padding: 6rem 1.5rem;
		text-align: center;
		background: radial-gradient(
			ellipse 60% 50% at 50% 50%,
			rgba(99, 102, 241, 0.1) 0%,
			transparent 70%
		);
		border-top: 1px solid var(--rtp-border-soft);
	}
	.pricing-cta__inner {
		max-width: 560px;
		margin: 0 auto;
	}
	.pricing-cta__title {
		font-size: clamp(1.75rem, 4vw, 2.5rem);
		font-weight: 800;
		color: var(--rtp-text);
		margin: 0 0 1rem;
	}
	.pricing-cta__sub {
		font-size: 1.0625rem;
		color: var(--rtp-text-muted);
		margin: 0 0 2rem;
	}
	.pricing-cta__btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem 2rem;
		background: var(--rtp-indigo);
		color: #fff;
		font-size: 1rem;
		font-weight: 700;
		border-radius: 0.5rem;
		text-decoration: none;
		transition: background 0.2s ease;
	}
	.pricing-cta__btn:hover {
		background: #4f52e0;
	}

	@media (max-width: 900px) {
		.plans-grid {
			grid-template-columns: 1fr;
			max-width: 440px;
			margin: 0 auto;
		}
	}
</style>
