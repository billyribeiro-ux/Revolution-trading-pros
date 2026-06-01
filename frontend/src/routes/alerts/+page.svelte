<script lang="ts">
	/**
	 * Alert Services Index Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Lists all available alert services with comparison.
	 * Matches Revolution Trading's product listing style.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// --- GSAP ScrollTrigger Animations (Svelte 5 SSR-safe pattern) ---
	onMount(() => {
		if (!browser) return;

		let ctx: ReturnType<typeof import('gsap').gsap.context> | null = null;

		(async () => {
			const { gsap } = await import('gsap');
			const { ScrollTrigger } = await import('gsap/ScrollTrigger');
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
		})();

		return () => ctx?.revert();
	});

	// Alert Services Data
	const alertServices = [
		{
			id: 'explosive-swings',
			name: 'Explosive Swings',
			tagline: 'Multi-Day Swing Trading Alerts',
			description:
				'Premium swing trading alerts for 3-7 day opportunities. Catch the big moves without staring at screens all day.',
			features: [
				'2-4 Premium Swing Alerts Weekly',
				'Instant SMS & Email Delivery',
				'Private Discord Community',
				'Detailed Technical Analysis',
				'Risk Management Guidelines'
			],
			stats: { winRate: '82%', avgHold: '3-7 days', riskReward: '4:1' },
			price: { monthly: 97, annual: 927 },
			color: 'emerald',
			icon: '🎯',
			href: '/alerts/explosive-swings'
		},
		{
			id: 'spx-profit-pulse',
			name: 'SPX Profit Pulse',
			tagline: 'Daily SPX/SPY Options Alerts',
			description:
				'High-precision daily options alerts focused on SPX and SPY. Designed for active traders who want consistent opportunities.',
			features: [
				'Daily SPX/SPY Options Alerts',
				'Real-time Entry & Exit Signals',
				'Market Open Analysis',
				'Private Discord Access',
				'Weekly Strategy Sessions'
			],
			stats: { winRate: '76%', avgHold: '0-1 day', riskReward: '3:1' },
			price: { monthly: 147, quarterly: 397, annual: 1397 },
			color: 'blue',
			icon: '⚡',
			href: '/alerts/spx-profit-pulse'
		}
	];
</script>

<div class="alerts-page">
	<!-- Hero Section -->
	<section class="hero">
		<div class="hero-bg"></div>
		<div class="hero-content">
			<span class="hero-badge" data-gsap>
				<span class="badge-dot"></span>
				Alert Services
			</span>
			<h1 data-gsap>
				Trade Alerts<br />
				<span class="gradient-text">Delivered.</span>
			</h1>
			<p data-gsap>
				Stop guessing. Get high-probability trade alerts sent directly to your phone. Choose the
				service that matches your style.
			</p>
		</div>
	</section>

	<!-- Comparison Section -->
	<section class="comparison-section">
		<div class="comparison-header" data-gsap>
			<h2>Choose Your Alert Service</h2>
			<p>Both services include real-time alerts, risk management, and community access.</p>
		</div>

		<div class="services-grid">
			{#each alertServices as service, i (service.id)}
				<article class="service-card service-card--{service.color}" data-gsap={{ delay: i * 150 }}>
					<div class="service-header">
						<span class="service-icon">{service.icon}</span>
						<div>
							<h3>{service.name}</h3>
							<p class="tagline">{service.tagline}</p>
						</div>
					</div>

					<p class="service-description">{service.description}</p>

					<!-- Stats -->
					<div class="service-stats">
						<div class="stat">
							<span class="stat-value">{service.stats.winRate}</span>
							<span class="stat-label">Win Rate</span>
						</div>
						<div class="stat">
							<span class="stat-value">{service.stats.avgHold}</span>
							<span class="stat-label">Hold Time</span>
						</div>
						<div class="stat">
							<span class="stat-value">{service.stats.riskReward}</span>
							<span class="stat-label">Risk/Reward</span>
						</div>
					</div>

					<!-- Features -->
					<ul class="service-features">
						{#each service.features as feature (feature)}
							<li>
								<svg
									aria-hidden="true"
									class="check-icon"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M5 13l4 4L19 7"
									/>
								</svg>
								{feature}
							</li>
						{/each}
					</ul>

					<!-- Pricing -->
					<div class="service-pricing">
						<div class="price-main">
							<span class="price-from">Starting at</span>
							<span class="price-amount">${service.price.monthly}</span>
							<span class="price-period">/month</span>
						</div>
						{#if service.price.annual}
							<p class="price-annual">
								or ${service.price.annual}/year (save ${service.price.monthly * 12 -
									service.price.annual})
							</p>
						{/if}
					</div>

					<a href={service.href} class="service-cta">
						View Details & Subscribe
						<svg
							aria-hidden="true"
							class="arrow-icon"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 7l5 5m0 0l-5 5m5-5H6"
							/>
						</svg>
					</a>
				</article>
			{/each}
		</div>
	</section>

	<!-- How It Works -->
	<section class="how-section">
		<div class="how-content">
			<h2 data-gsap>How It Works</h2>
			<div class="steps-grid">
				<div class="step" data-gsap>
					<div class="step-number">1</div>
					<h3>Subscribe</h3>
					<p>Choose your alert service and complete checkout. Instant access upon payment.</p>
				</div>
				<div class="step" data-gsap>
					<div class="step-number">2</div>
					<h3>Connect</h3>
					<p>Join our Discord, set up SMS alerts, and configure your notification preferences.</p>
				</div>
				<div class="step" data-gsap>
					<div class="step-number">3</div>
					<h3>Trade</h3>
					<p>Receive alerts with entry, target, and stop levels. Execute and manage your trades.</p>
				</div>
				<div class="step" data-gsap>
					<div class="step-number">4</div>
					<h3>Learn</h3>
					<p>Review trade breakdowns, attend live sessions, and improve your skills over time.</p>
				</div>
			</div>
		</div>
	</section>

	<!-- FAQ Section -->
	<section class="faq-section">
		<div class="faq-content">
			<h2>Frequently Asked Questions</h2>
			<div class="faq-grid">
				<div class="faq-item">
					<h3>How are alerts delivered?</h3>
					<p>
						Alerts are sent via SMS, email, and Discord simultaneously so you never miss a trade
						opportunity.
					</p>
				</div>
				<div class="faq-item">
					<h3>Can I cancel anytime?</h3>
					<p>
						Yes! All subscriptions can be cancelled at any time. We also offer a 30-day money-back
						guarantee.
					</p>
				</div>
				<div class="faq-item">
					<h3>What markets do you trade?</h3>
					<p>
						We primarily trade US equities and options, focusing on liquid stocks like SPY, QQQ, and
						the Magnificent 7.
					</p>
				</div>
				<div class="faq-item">
					<h3>Is this financial advice?</h3>
					<p>
						No. Our alerts are educational in nature. Always do your own research and manage your
						own risk.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- CTA Section -->
	<section class="cta-section">
		<div class="cta-content">
			<h2>Ready to Trade Smarter?</h2>
			<p>Join thousands of traders receiving our alerts every day.</p>
			<div class="cta-buttons">
				<a href="/alerts/explosive-swings" class="cta-button cta-button--primary">
					Swing Trading Alerts
				</a>
				<a href="/alerts/spx-profit-pulse" class="cta-button cta-button--secondary">
					SPX Daily Alerts
				</a>
			</div>
		</div>
	</section>
</div>

<style>
	/* ─────────────────────────────────────────────────────────────────
	   Page-local tokens — colors only used inside this hub page.
	   Reusable values (canvas, surfaces, text) come from --rtp-*.
	   Brand emerald/blue used as service variants live on --rtp-emerald/
	   --rtp-blue; lighter accent shades (emerald-300, blue-bright) are
	   already token-defined as well.
	   ───────────────────────────────────────────────────────────────── */
	.alerts-page {
		--alerts-emerald-deep: #059669; /* emerald-600, gradient pair */
		--alerts-blue-deep: #2563eb; /* blue-600, gradient pair */
		--alerts-success-bright: #22c55e; /* badge dot pulse, check icon */

		background: var(--rtp-bg);
		color: var(--rtp-text-soft);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Hero
	   ───────────────────────────────────────────────────────────────── */
	.hero {
		position: relative;
		padding-block: 7.5rem 5rem;
		padding-inline: 1.5rem;
		text-align: center;
		overflow: hidden;
	}

	.hero-bg {
		position: absolute;
		inset: 0;
		background:
			radial-gradient(
				ellipse at 30% 20%,
				color-mix(in oklab, var(--rtp-emerald) 15%, transparent) 0%,
				transparent 50%
			),
			radial-gradient(
				ellipse at 70% 80%,
				color-mix(in oklab, var(--rtp-primary) 10%, transparent) 0%,
				transparent 50%
			);
	}

	.hero-content {
		position: relative;
		max-width: 800px;
		margin-inline: auto;
	}

	.hero-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: color-mix(in oklab, var(--rtp-emerald) 10%, transparent);
		border: 1px solid color-mix(in oklab, var(--rtp-emerald) 30%, transparent);
		padding: 0.5rem 1rem;
		border-radius: var(--rtp-radius-pill);
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--rtp-emerald-bright);
		margin-block-end: 1.5rem;
	}

	.badge-dot {
		width: 0.5rem;
		height: 0.5rem;
		background: var(--alerts-success-bright);
		border-radius: 50%;
		animation: alerts-pulse 2s infinite;
	}

	@keyframes alerts-pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.hero h1 {
		font-size: clamp(2.5rem, 6vw, 4rem);
		font-weight: 800;
		line-height: 1.1;
		margin-block-end: 1.5rem;
		color: var(--rtp-text);
	}

	.gradient-text {
		background: linear-gradient(135deg, var(--rtp-emerald-bright), var(--rtp-blue-bright));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero p {
		font-size: 1.25rem;
		color: var(--rtp-text-muted);
		max-width: 600px;
		margin-inline: auto;
		line-height: 1.7;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Comparison
	   ───────────────────────────────────────────────────────────────── */
	.comparison-section {
		padding-block: 5rem;
		padding-inline: 1.5rem;
		max-width: 1000px;
		margin-inline: auto;
	}

	.comparison-header {
		text-align: center;
		margin-block-end: 3rem;
	}

	.comparison-header h2 {
		font-size: 2rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 0.75rem;
	}

	.comparison-header p {
		color: var(--rtp-text-muted);
	}

	.services-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 400px), 1fr));
		gap: 2rem;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Service card (variant: emerald | blue)
	   ───────────────────────────────────────────────────────────────── */
	.service-card {
		background: var(--rtp-surface);
		border-radius: var(--rtp-radius-xl);
		padding: 2rem;
		border: 2px solid var(--rtp-text-faint);
		transition: all 0.3s ease;
	}

	.service-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 20px 40px -12px rgba(0, 0, 0, 0.3);
	}

	.service-card--emerald:hover {
		border-color: color-mix(in oklab, var(--rtp-emerald) 50%, transparent);
	}
	.service-card--blue:hover {
		border-color: color-mix(in oklab, var(--rtp-primary) 50%, transparent);
	}

	.service-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		margin-block-end: 1.25rem;
	}

	.service-icon {
		font-size: 2.5rem;
		line-height: 1;
	}

	.service-header h3 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin: 0;
	}

	.tagline {
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
		margin: 0.25rem 0 0;
	}

	.service-description {
		color: var(--rtp-text-muted);
		line-height: 1.6;
		margin-block-end: 1.5rem;
	}

	/* ── Stats ────────────────────────────────────────────────────── */
	.service-stats {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
		background: var(--rtp-bg);
		border-radius: var(--rtp-radius-md);
		padding: 1rem;
		margin-block-end: 1.5rem;
	}

	.stat {
		text-align: center;
	}

	.stat-value {
		display: block;
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--rtp-text);
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--rtp-text-subtle);
	}

	.service-card--emerald .stat-value {
		color: var(--rtp-emerald-bright);
	}
	.service-card--blue .stat-value {
		color: var(--rtp-blue-bright);
	}

	/* ── Features ─────────────────────────────────────────────────── */
	.service-features {
		list-style: none;
		padding: 0;
		margin: 0 0 1.5rem;
	}

	.service-features li {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.5rem 0;
		font-size: 0.875rem;
		color: var(--rtp-text-soft);
	}

	.check-icon {
		width: 1.125rem;
		height: 1.125rem;
		flex-shrink: 0;
		color: var(--alerts-success-bright);
	}

	/* ── Pricing ──────────────────────────────────────────────────── */
	.service-pricing {
		text-align: center;
		margin-block-end: 1.5rem;
	}

	.price-main {
		display: flex;
		align-items: baseline;
		justify-content: center;
		gap: 0.25rem;
	}

	.price-from {
		font-size: 0.875rem;
		color: var(--rtp-text-subtle);
	}

	.price-amount {
		font-size: 2.5rem;
		font-weight: 800;
		color: var(--rtp-text);
	}

	.price-period {
		font-size: 0.875rem;
		color: var(--rtp-text-subtle);
	}

	.price-annual {
		font-size: 0.8125rem;
		color: var(--alerts-success-bright);
		margin-block-start: 0.5rem;
	}

	/* ── CTA button ───────────────────────────────────────────────── */
	.service-cta {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 1rem 1.5rem;
		background: linear-gradient(135deg, var(--rtp-emerald), var(--alerts-emerald-deep));
		color: var(--rtp-text);
		font-weight: 600;
		font-size: 1rem;
		border-radius: var(--rtp-radius-md);
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.service-card--blue .service-cta {
		background: linear-gradient(135deg, var(--rtp-primary), var(--alerts-blue-deep));
	}

	.service-cta:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px -4px color-mix(in oklab, var(--rtp-emerald) 40%, transparent);
	}

	.service-card--blue .service-cta:hover {
		box-shadow: 0 8px 20px -4px color-mix(in oklab, var(--rtp-primary) 40%, transparent);
	}

	.arrow-icon {
		width: 1.25rem;
		height: 1.25rem;
		transition: transform 0.2s ease;
	}

	.service-cta:hover .arrow-icon {
		transform: translateX(4px);
	}

	/* ─────────────────────────────────────────────────────────────────
	   How it works
	   ───────────────────────────────────────────────────────────────── */
	.how-section {
		background: var(--rtp-surface);
		padding-block: 5rem;
		padding-inline: 1.5rem;
	}

	.how-content {
		max-width: 1000px;
		margin-inline: auto;
		text-align: center;
	}

	.how-content h2 {
		font-size: 2rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 3rem;
	}

	.steps-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 2rem;
	}

	.step {
		padding: 1.5rem;
	}

	.step-number {
		width: 3rem;
		height: 3rem;
		background: linear-gradient(135deg, var(--rtp-emerald), var(--alerts-emerald-deep));
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin: 0 auto 1rem;
	}

	.step h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--rtp-text);
		margin-block-end: 0.5rem;
	}

	.step p {
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
		line-height: 1.6;
	}

	/* ─────────────────────────────────────────────────────────────────
	   FAQ
	   ───────────────────────────────────────────────────────────────── */
	.faq-section {
		padding-block: 5rem;
		padding-inline: 1.5rem;
	}

	.faq-content {
		max-width: 800px;
		margin-inline: auto;
	}

	.faq-content h2 {
		font-size: 2rem;
		font-weight: 700;
		color: var(--rtp-text);
		text-align: center;
		margin-block-end: 3rem;
	}

	.faq-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.faq-item {
		background: var(--rtp-surface);
		border-radius: var(--rtp-radius-md);
		padding: 1.5rem;
	}

	.faq-item h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--rtp-text);
		margin-block-end: 0.5rem;
	}

	.faq-item p {
		font-size: 0.875rem;
		color: var(--rtp-text-muted);
		line-height: 1.6;
	}

	/* ─────────────────────────────────────────────────────────────────
	   Bottom CTA
	   ───────────────────────────────────────────────────────────────── */
	.cta-section {
		padding-block: 5rem;
		padding-inline: 1.5rem;
		text-align: center;
		background: linear-gradient(
			135deg,
			color-mix(in oklab, var(--rtp-emerald) 10%, transparent),
			color-mix(in oklab, var(--rtp-primary) 10%, transparent)
		);
	}

	.cta-content {
		max-width: 600px;
		margin-inline: auto;
	}

	.cta-content h2 {
		font-size: 2rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin-block-end: 1rem;
	}

	.cta-content p {
		color: var(--rtp-text-muted);
		margin-block-end: 2rem;
	}

	.cta-buttons {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.cta-button {
		display: inline-block;
		padding: 1rem 2rem;
		font-weight: 600;
		font-size: 1rem;
		border-radius: var(--rtp-radius-md);
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.cta-button--primary {
		background: linear-gradient(135deg, var(--rtp-emerald), var(--alerts-emerald-deep));
		color: var(--rtp-text);
	}

	.cta-button--secondary {
		background: linear-gradient(135deg, var(--rtp-primary), var(--alerts-blue-deep));
		color: var(--rtp-text);
	}

	.cta-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px -4px rgba(0, 0, 0, 0.3);
	}

	/* ─────────────────────────────────────────────────────────────────
	   Mobile-first responsive (small-screen overrides).
	   The defaults above already work mobile-first; the previous
	   max-width queries shrunk things further on small phones.
	   ───────────────────────────────────────────────────────────────── */
	@media (max-width: 767.98px) {
		.hero {
			padding-block: 5rem 3.75rem;
			padding-inline: 1rem;
		}

		.services-grid {
			grid-template-columns: 1fr;
		}

		.service-stats {
			grid-template-columns: 1fr;
			gap: 0.75rem;
		}

		.stat {
			display: flex;
			justify-content: space-between;
			text-align: start;
		}

		.service-card {
			padding: 1.5rem;
		}

		.service-features li {
			font-size: 0.9375rem;
		}
	}

	@media (max-width: 479.98px) {
		.hero {
			padding-block: 3.75rem 2.5rem;
			padding-inline: 0.75rem;
		}

		.service-card {
			padding: 1.25rem;
			border-radius: var(--rtp-radius-lg);
		}

		.service-header h3 {
			font-size: 1.25rem;
		}
	}
</style>
