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
	import SEOHead from '$lib/components/SEOHead.svelte';

	// Alert Services Data
	const alertServices = [
		{
			id: 'explosive-swings',
			name: 'Explosive Swings',
			tagline: 'Multi-Day Swing Trading Alerts',
			description: 'Premium swing trading alerts for 3-7 day opportunities. Catch the big moves without staring at screens all day.',
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
			description: 'High-precision daily options alerts focused on SPX and SPY. Designed for active traders who want consistent opportunities.',
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

	// --- Apple ICT9+ Scroll Animations ---
	let mounted = $state(false);
	
	function reveal(node: HTMLElement, params: { delay?: number; y?: number } = {}) {
		const delay = params.delay ?? 0;
		const translateY = params.y ?? 30;
		
		if (typeof window !== 'undefined' && window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
			return;
		}
		
		node.style.opacity = '0';
		node.style.transform = `translateY(${translateY}px)`;
		node.style.transition = `opacity 0.8s cubic-bezier(0.25, 0.46, 0.45, 0.94), transform 0.8s cubic-bezier(0.25, 0.46, 0.45, 0.94)`;
		node.style.transitionDelay = `${delay}ms`;
		node.style.willChange = 'opacity, transform';
		
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						node.style.opacity = '1';
						node.style.transform = 'translateY(0)';
						setTimeout(() => { node.style.willChange = 'auto'; }, 800 + delay);
						observer.unobserve(node);
					}
				});
			},
			{ threshold: 0.1, rootMargin: '0px 0px -60px 0px' }
		);
		
		observer.observe(node);
		
		return {
			destroy() {
				observer.disconnect();
			}
		};
	}
	
	onMount(() => {
		mounted = true;
	});
</script>

<SEOHead
	title="Trading Alert Services | Revolution Trading Pros"
	description="Get premium trading alerts delivered to your phone. Swing trading and day trading alerts with precise entries, exits, and risk management."
	canonical="/alerts"
	ogType="website"
	keywords={['trading alerts', 'swing trading alerts', 'SPX alerts', 'options alerts', 'trade signals', 'stock alerts']}
/>

<main class="alerts-page">
	<!-- Hero Section -->
	<section class="hero">
		<div class="hero-bg"></div>
		<div class="hero-content">
			<span class="hero-badge" use:reveal={{ delay: 0 }}>
				<span class="badge-dot"></span>
				Alert Services
			</span>
			<h1 use:reveal={{ delay: 100 }}>
				Trade Alerts<br />
				<span class="gradient-text">Delivered.</span>
			</h1>
			<p use:reveal={{ delay: 200 }}>
				Stop guessing. Get high-probability trade alerts sent directly to your phone.
				Choose the service that matches your style.
			</p>
		</div>
	</section>

	<!-- Comparison Section -->
	<section class="comparison-section">
		<div class="comparison-header" use:reveal>
			<h2>Choose Your Alert Service</h2>
			<p>Both services include real-time alerts, risk management, and community access.</p>
		</div>

		<div class="services-grid">
			{#each alertServices as service, i}
				<article
					class="service-card service-card--{service.color}"
					use:reveal={{ delay: i * 150 }}
				>
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
						{#each service.features as feature}
							<li>
								<svg class="check-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
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
								or ${service.price.annual}/year (save ${service.price.monthly * 12 - service.price.annual})
							</p>
						{/if}
					</div>

					<a href={service.href} class="service-cta">
						View Details & Subscribe
						<svg class="arrow-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
						</svg>
					</a>
				</article>
			{/each}
		</div>
	</section>

	<!-- How It Works -->
	<section class="how-section">
		<div class="how-content">
			<h2 use:reveal>How It Works</h2>
			<div class="steps-grid">
				<div class="step" use:reveal={{ delay: 0 }}>
					<div class="step-number">1</div>
					<h3>Subscribe</h3>
					<p>Choose your alert service and complete checkout. Instant access upon payment.</p>
				</div>
				<div class="step" use:reveal={{ delay: 100 }}>
					<div class="step-number">2</div>
					<h3>Connect</h3>
					<p>Join our Discord, set up SMS alerts, and configure your notification preferences.</p>
				</div>
				<div class="step" use:reveal={{ delay: 200 }}>
					<div class="step-number">3</div>
					<h3>Trade</h3>
					<p>Receive alerts with entry, target, and stop levels. Execute and manage your trades.</p>
				</div>
				<div class="step" use:reveal={{ delay: 300 }}>
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
					<p>Alerts are sent via SMS, email, and Discord simultaneously so you never miss a trade opportunity.</p>
				</div>
				<div class="faq-item">
					<h3>Can I cancel anytime?</h3>
					<p>Yes! All subscriptions can be cancelled at any time. We also offer a 30-day money-back guarantee.</p>
				</div>
				<div class="faq-item">
					<h3>What markets do you trade?</h3>
					<p>We primarily trade US equities and options, focusing on liquid stocks like SPY, QQQ, and the Magnificent 7.</p>
				</div>
				<div class="faq-item">
					<h3>Is this financial advice?</h3>
					<p>No. Our alerts are educational in nature. Always do your own research and manage your own risk.</p>
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
</main>

<style>
	.alerts-page {
		background: #0f172a;
		color: #e2e8f0;
		min-height: 100vh;
	}

	/* Hero */
	.hero {
		position: relative;
		padding: 120px 24px 80px;
		text-align: center;
		overflow: hidden;
	}

	.hero-bg {
		position: absolute;
		inset: 0;
		background:
			radial-gradient(ellipse at 30% 20%, rgba(16, 185, 129, 0.15) 0%, transparent 50%),
			radial-gradient(ellipse at 70% 80%, rgba(59, 130, 246, 0.1) 0%, transparent 50%);
	}

	.hero-content {
		position: relative;
		max-width: 800px;
		margin: 0 auto;
	}

	.hero-badge {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		padding: 8px 16px;
		border-radius: 24px;
		font-size: 14px;
		font-weight: 600;
		color: #34d399;
		margin-bottom: 24px;
	}

	.badge-dot {
		width: 8px;
		height: 8px;
		background: #22c55e;
		border-radius: 50%;
		animation: pulse 2s infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}

	.hero h1 {
		font-size: clamp(2.5rem, 6vw, 4rem);
		font-weight: 800;
		line-height: 1.1;
		margin-bottom: 24px;
		color: #fff;
	}

	.gradient-text {
		background: linear-gradient(135deg, #34d399, #60a5fa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero p {
		font-size: 1.25rem;
		color: #94a3b8;
		max-width: 600px;
		margin: 0 auto;
		line-height: 1.7;
	}

	/* Comparison Section */
	.comparison-section {
		padding: 80px 24px;
		max-width: 1000px;
		margin: 0 auto;
	}

	.comparison-header {
		text-align: center;
		margin-bottom: 48px;
	}

	.comparison-header h2 {
		font-size: 2rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 12px;
	}

	.comparison-header p {
		color: #94a3b8;
	}

	.services-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
		gap: 32px;
	}

	/* Service Card */
	.service-card {
		background: #1e293b;
		border-radius: 24px;
		padding: 32px;
		border: 2px solid #334155;
		transition: all 0.3s ease;
	}

	.service-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 20px 40px -12px rgba(0, 0, 0, 0.3);
	}

	.service-card--emerald:hover { border-color: rgba(16, 185, 129, 0.5); }
	.service-card--blue:hover { border-color: rgba(59, 130, 246, 0.5); }

	.service-header {
		display: flex;
		align-items: flex-start;
		gap: 16px;
		margin-bottom: 20px;
	}

	.service-icon {
		font-size: 2.5rem;
		line-height: 1;
	}

	.service-header h3 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #fff;
		margin: 0;
	}

	.tagline {
		font-size: 14px;
		color: #94a3b8;
		margin: 4px 0 0;
	}

	.service-description {
		color: #94a3b8;
		line-height: 1.6;
		margin-bottom: 24px;
	}

	/* Stats */
	.service-stats {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 16px;
		background: #0f172a;
		border-radius: 12px;
		padding: 16px;
		margin-bottom: 24px;
	}

	.stat {
		text-align: center;
	}

	.stat-value {
		display: block;
		font-size: 1.25rem;
		font-weight: 700;
		color: #fff;
	}

	.stat-label {
		font-size: 12px;
		color: #64748b;
	}

	.service-card--emerald .stat-value { color: #34d399; }
	.service-card--blue .stat-value { color: #60a5fa; }

	/* Features */
	.service-features {
		list-style: none;
		padding: 0;
		margin: 0 0 24px;
	}

	.service-features li {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 8px 0;
		font-size: 14px;
		color: #cbd5e1;
	}

	.check-icon {
		width: 18px;
		height: 18px;
		flex-shrink: 0;
		color: #22c55e;
	}

	/* Pricing */
	.service-pricing {
		text-align: center;
		margin-bottom: 24px;
	}

	.price-main {
		display: flex;
		align-items: baseline;
		justify-content: center;
		gap: 4px;
	}

	.price-from {
		font-size: 14px;
		color: #64748b;
	}

	.price-amount {
		font-size: 2.5rem;
		font-weight: 800;
		color: #fff;
	}

	.price-period {
		font-size: 14px;
		color: #64748b;
	}

	.price-annual {
		font-size: 13px;
		color: #22c55e;
		margin-top: 8px;
	}

	/* CTA Button */
	.service-cta {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		padding: 16px 24px;
		background: linear-gradient(135deg, #10b981, #059669);
		color: #fff;
		font-weight: 600;
		font-size: 16px;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.service-card--blue .service-cta {
		background: linear-gradient(135deg, #3b82f6, #2563eb);
	}

	.service-cta:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px -4px rgba(16, 185, 129, 0.4);
	}

	.service-card--blue .service-cta:hover {
		box-shadow: 0 8px 20px -4px rgba(59, 130, 246, 0.4);
	}

	.arrow-icon {
		width: 20px;
		height: 20px;
		transition: transform 0.2s ease;
	}

	.service-cta:hover .arrow-icon {
		transform: translateX(4px);
	}

	/* How Section */
	.how-section {
		background: #1e293b;
		padding: 80px 24px;
	}

	.how-content {
		max-width: 1000px;
		margin: 0 auto;
		text-align: center;
	}

	.how-content h2 {
		font-size: 2rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 48px;
	}

	.steps-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 32px;
	}

	.step {
		padding: 24px;
	}

	.step-number {
		width: 48px;
		height: 48px;
		background: linear-gradient(135deg, #10b981, #059669);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.25rem;
		font-weight: 700;
		color: #fff;
		margin: 0 auto 16px;
	}

	.step h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 8px;
	}

	.step p {
		font-size: 14px;
		color: #94a3b8;
		line-height: 1.6;
	}

	/* FAQ Section */
	.faq-section {
		padding: 80px 24px;
	}

	.faq-content {
		max-width: 800px;
		margin: 0 auto;
	}

	.faq-content h2 {
		font-size: 2rem;
		font-weight: 700;
		color: #fff;
		text-align: center;
		margin-bottom: 48px;
	}

	.faq-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 24px;
	}

	.faq-item {
		background: #1e293b;
		border-radius: 12px;
		padding: 24px;
	}

	.faq-item h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 8px;
	}

	.faq-item p {
		font-size: 14px;
		color: #94a3b8;
		line-height: 1.6;
	}

	/* CTA Section */
	.cta-section {
		padding: 80px 24px;
		text-align: center;
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(59, 130, 246, 0.1));
	}

	.cta-content {
		max-width: 600px;
		margin: 0 auto;
	}

	.cta-content h2 {
		font-size: 2rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 16px;
	}

	.cta-content p {
		color: #94a3b8;
		margin-bottom: 32px;
	}

	.cta-buttons {
		display: flex;
		gap: 16px;
		justify-content: center;
		flex-wrap: wrap;
	}

	.cta-button {
		display: inline-block;
		padding: 16px 32px;
		font-weight: 600;
		font-size: 16px;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.cta-button--primary {
		background: linear-gradient(135deg, #10b981, #059669);
		color: #fff;
	}

	.cta-button--secondary {
		background: linear-gradient(135deg, #3b82f6, #2563eb);
		color: #fff;
	}

	.cta-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px -4px rgba(0, 0, 0, 0.3);
	}

	/* Responsive */
	@media (max-width: 768px) {
		.hero {
			padding: 80px 16px 60px;
		}

		.services-grid {
			grid-template-columns: 1fr;
		}

		.service-stats {
			grid-template-columns: 1fr;
			gap: 12px;
		}

		.stat {
			display: flex;
			justify-content: space-between;
			text-align: left;
		}
	}
</style>
