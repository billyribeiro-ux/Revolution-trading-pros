<script lang="ts">
	/**
	 * Live Trading Rooms Index Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Lists all available live trading rooms with pricing comparison.
	 * Matches Simpler Trading's product listing style.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import SEOHead from '$lib/components/SEOHead.svelte';

	// Trading Rooms Data
	const tradingRooms = [
		{
			id: 'day-trading',
			name: 'Day Trading Room',
			tagline: 'Trade the Open with Professionals',
			description: 'Join our expert traders live every market day. Real-time analysis, trade calls, and market commentary.',
			features: [
				'Live Trading Sessions (M-F 9:30 AM - 12:00 PM ET)',
				'Real-time Trade Alerts',
				'Screen Share & Live Commentary',
				'Private Discord Community',
				'Trade Replay Archive'
			],
			price: { monthly: 247, quarterly: 597, annual: 1897 },
			color: 'blue',
			icon: '📈',
			href: '/live-trading-rooms/day-trading'
		},
		{
			id: 'swing-trading',
			name: 'Swing Trading Room',
			tagline: 'Catch Multi-Day Moves',
			description: 'Focus on 3-7 day swing trades with weekly analysis sessions and detailed trade plans.',
			features: [
				'Weekly Swing Trade Ideas',
				'Technical Analysis Sessions',
				'Position Management Guidance',
				'Private Discord Community',
				'Weekend Market Prep'
			],
			price: { monthly: 197, quarterly: 497, annual: 1497 },
			color: 'emerald',
			icon: '🎯',
			href: '/live-trading-rooms/swing-trading'
		},
		{
			id: 'small-accounts',
			name: 'Small Accounts Room',
			tagline: 'Grow Your Account Strategically',
			description: 'Specialized strategies for traders with accounts under $25K. Risk-focused approach to consistent growth.',
			features: [
				'Small Account Strategies',
				'Risk-First Trade Selection',
				'Live Trading Sessions',
				'Account Growth Plans',
				'Community Support'
			],
			price: { monthly: 147, quarterly: 397, annual: 1197 },
			color: 'amber',
			icon: '🚀',
			href: '/live-trading-rooms/small-accounts'
		}
	];

	// Animation observer
	let observer: IntersectionObserver;

	function reveal(node: HTMLElement, params: { delay?: number } = {}) {
		node.classList.add('opacity-100', 'translate-y-0');

		if (typeof window !== 'undefined' && observer) {
			node.classList.remove('opacity-100', 'translate-y-0');
			node.classList.add('opacity-0', 'translate-y-8');
			node.dataset.delay = (params.delay || 0).toString();
			observer.observe(node);
		}

		return {
			destroy() {
				if (observer) observer.unobserve(node);
			}
		};
	}

	onMount(() => {
		observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const el = entry.target as HTMLElement;
						const delay = parseInt(el.dataset.delay || '0');

						setTimeout(() => {
							el.classList.remove('opacity-0', 'translate-y-8');
							el.classList.add('opacity-100', 'translate-y-0', 'transition-all', 'duration-700', 'ease-out');
						}, delay);

						observer.unobserve(el);
					}
				});
			},
			{ threshold: 0.1, rootMargin: '0px 0px -50px 0px' }
		);
	});
</script>

<SEOHead
	title="Live Trading Rooms | Revolution Trading Pros"
	description="Join our live trading rooms and trade alongside professional traders. Day trading, swing trading, and small account strategies with real-time alerts and analysis."
	canonical="/live-trading-rooms"
	ogType="website"
	keywords={['live trading room', 'day trading', 'swing trading', 'trading community', 'real-time alerts', 'trading education']}
/>

<main class="live-trading-rooms">
	<!-- Hero Section -->
	<section class="hero">
		<div class="hero-bg"></div>
		<div class="hero-content">
			<span class="hero-badge" use:reveal={{ delay: 0 }}>
				<span class="badge-dot"></span>
				Live Trading Rooms
			</span>
			<h1 use:reveal={{ delay: 100 }}>
				Trade <span class="gradient-text">Together.</span><br />
				Win Together.
			</h1>
			<p use:reveal={{ delay: 200 }}>
				Join our community of traders and learn from professionals in real-time.
				Pick the room that matches your trading style.
			</p>
		</div>
	</section>

	<!-- Trading Rooms Grid -->
	<section class="rooms-section">
		<div class="rooms-grid">
			{#each tradingRooms as room, i}
				<article
					class="room-card room-card--{room.color}"
					use:reveal={{ delay: i * 100 }}
				>
					<div class="room-header">
						<span class="room-icon">{room.icon}</span>
						<div>
							<h2>{room.name}</h2>
							<p class="tagline">{room.tagline}</p>
						</div>
					</div>

					<p class="room-description">{room.description}</p>

					<ul class="room-features">
						{#each room.features as feature}
							<li>
								<svg class="check-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
								</svg>
								{feature}
							</li>
						{/each}
					</ul>

					<div class="room-pricing">
						<div class="price-row">
							<span>Monthly</span>
							<span class="price">${room.price.monthly}<span class="period">/mo</span></span>
						</div>
						<div class="price-row">
							<span>Quarterly</span>
							<span class="price">${room.price.quarterly}<span class="period">/qtr</span></span>
						</div>
						<div class="price-row highlight">
							<span>Annual <span class="savings">(Best Value)</span></span>
							<span class="price">${room.price.annual}<span class="period">/yr</span></span>
						</div>
					</div>

					<a href={room.href} class="room-cta">
						Learn More & Join
						<svg class="arrow-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
						</svg>
					</a>
				</article>
			{/each}
		</div>
	</section>

	<!-- Why Join Section -->
	<section class="why-section">
		<div class="why-content">
			<h2 use:reveal>Why Trade With Us?</h2>
			<div class="why-grid">
				<div class="why-item" use:reveal={{ delay: 0 }}>
					<div class="why-icon">👨‍🏫</div>
					<h3>Expert Traders</h3>
					<p>Learn from professionals with decades of combined experience in the markets.</p>
				</div>
				<div class="why-item" use:reveal={{ delay: 100 }}>
					<div class="why-icon">⚡</div>
					<h3>Real-Time Alerts</h3>
					<p>Get trade alerts as they happen via SMS, email, and Discord notifications.</p>
				</div>
				<div class="why-item" use:reveal={{ delay: 200 }}>
					<div class="why-icon">🎓</div>
					<h3>Continuous Learning</h3>
					<p>Access educational content, strategy breakdowns, and market analysis.</p>
				</div>
				<div class="why-item" use:reveal={{ delay: 300 }}>
					<div class="why-icon">🤝</div>
					<h3>Supportive Community</h3>
					<p>Join thousands of traders who help each other succeed every day.</p>
				</div>
			</div>
		</div>
	</section>

	<!-- CTA Section -->
	<section class="cta-section">
		<div class="cta-content">
			<h2>Ready to Level Up Your Trading?</h2>
			<p>Join thousands of traders who've transformed their results with our live rooms.</p>
			<a href="#rooms-section" class="cta-button">
				Choose Your Room
			</a>
		</div>
	</section>
</main>

<style>
	.live-trading-rooms {
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
			radial-gradient(ellipse at 30% 20%, rgba(59, 130, 246, 0.15) 0%, transparent 50%),
			radial-gradient(ellipse at 70% 80%, rgba(16, 185, 129, 0.1) 0%, transparent 50%);
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
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.3);
		padding: 8px 16px;
		border-radius: 24px;
		font-size: 14px;
		font-weight: 600;
		color: #60a5fa;
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
		background: linear-gradient(135deg, #60a5fa, #34d399);
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

	/* Rooms Section */
	.rooms-section {
		padding: 80px 24px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.rooms-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(340px, 1fr));
		gap: 32px;
	}

	/* Room Card */
	.room-card {
		background: #1e293b;
		border-radius: 24px;
		padding: 32px;
		border: 1px solid #334155;
		transition: all 0.3s ease;
	}

	.room-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 20px 40px -12px rgba(0, 0, 0, 0.3);
	}

	.room-card--blue:hover { border-color: rgba(59, 130, 246, 0.5); }
	.room-card--emerald:hover { border-color: rgba(16, 185, 129, 0.5); }
	.room-card--amber:hover { border-color: rgba(245, 158, 11, 0.5); }

	.room-header {
		display: flex;
		align-items: flex-start;
		gap: 16px;
		margin-bottom: 20px;
	}

	.room-icon {
		font-size: 2.5rem;
		line-height: 1;
	}

	.room-header h2 {
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

	.room-description {
		color: #94a3b8;
		line-height: 1.6;
		margin-bottom: 24px;
	}

	.room-features {
		list-style: none;
		padding: 0;
		margin: 0 0 24px;
	}

	.room-features li {
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
	.room-pricing {
		background: #0f172a;
		border-radius: 12px;
		padding: 16px;
		margin-bottom: 24px;
	}

	.price-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 8px 0;
		font-size: 14px;
		color: #94a3b8;
	}

	.price-row.highlight {
		background: rgba(16, 185, 129, 0.1);
		margin: 8px -8px -8px;
		padding: 12px 8px;
		border-radius: 8px;
	}

	.price {
		font-weight: 700;
		color: #fff;
		font-size: 16px;
	}

	.period {
		font-weight: 400;
		color: #64748b;
		font-size: 12px;
	}

	.savings {
		color: #22c55e;
		font-size: 12px;
	}

	/* CTA Button */
	.room-cta {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		padding: 16px 24px;
		background: linear-gradient(135deg, #3b82f6, #2563eb);
		color: #fff;
		font-weight: 600;
		font-size: 16px;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.room-card--emerald .room-cta {
		background: linear-gradient(135deg, #10b981, #059669);
	}

	.room-card--amber .room-cta {
		background: linear-gradient(135deg, #f59e0b, #d97706);
	}

	.room-cta:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px -4px rgba(59, 130, 246, 0.4);
	}

	.arrow-icon {
		width: 20px;
		height: 20px;
		transition: transform 0.2s ease;
	}

	.room-cta:hover .arrow-icon {
		transform: translateX(4px);
	}

	/* Why Section */
	.why-section {
		background: #1e293b;
		padding: 80px 24px;
	}

	.why-content {
		max-width: 1000px;
		margin: 0 auto;
		text-align: center;
	}

	.why-content h2 {
		font-size: 2rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 48px;
	}

	.why-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 32px;
	}

	.why-item {
		padding: 24px;
	}

	.why-icon {
		font-size: 2.5rem;
		margin-bottom: 16px;
	}

	.why-item h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 8px;
	}

	.why-item p {
		font-size: 14px;
		color: #94a3b8;
		line-height: 1.6;
	}

	/* CTA Section */
	.cta-section {
		padding: 80px 24px;
		text-align: center;
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.1), rgba(16, 185, 129, 0.1));
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

	.cta-button {
		display: inline-block;
		padding: 16px 32px;
		background: #fff;
		color: #0f172a;
		font-weight: 600;
		font-size: 16px;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.cta-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px -4px rgba(255, 255, 255, 0.2);
	}

	/* Responsive */
	@media (max-width: 768px) {
		.hero {
			padding: 80px 16px 60px;
		}

		.rooms-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
