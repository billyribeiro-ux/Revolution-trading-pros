<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import {
		IconWaveSine,
		IconTrendingUp,
		IconCheck,
		IconAlertTriangle,
		IconBolt,
		IconChartLine,
		IconTarget
	} from '@tabler/icons-svelte';

	let heroVisible = false;

	onMount(() => {
		if (!browser) return;
		const heroObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) heroVisible = true;
				});
			},
			{ threshold: 0.2 }
		);

		const heroElement = document.querySelector('.hero-section');
		if (heroElement) heroObserver.observe(heroElement);

		return () => heroObserver.disconnect();
	});
</script>

<svelte:head>
	<title>MACD - Moving Average Convergence Divergence | Revolution Trading</title>
	<meta
		name="description"
		content="Master the MACD indicator to identify trend changes and momentum shifts with this powerful trend-following tool."
	/>
	<meta property="og:title" content="MACD Indicator | Revolution Trading" />
	<meta
		property="og:description"
		content="Master the MACD indicator to identify trend changes and momentum shifts with this powerful trend-following tool."
	/>
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="MACD Indicator | Revolution Trading" />
	<meta
		name="twitter:description"
		content="Master the MACD indicator to identify trend changes and momentum shifts with this powerful trend-following tool."
	/>
</svelte:head>

<div class="indicator-page">
	<section class="hero-section" class:visible={heroVisible}>
		<div class="hero-background">
			<div class="glow-orb"></div>
			<div class="wave-pattern"></div>
		</div>

		<div class="hero-content">
			<div class="hero-badge">
				<IconWaveSine size={20} stroke={2} />
				<span>Trend Following Indicator</span>
			</div>

			<h1 class="hero-title">
				MACD<br />
				<span class="gradient-text">Indicator</span>
			</h1>

			<p class="hero-description">
				The MACD (Moving Average Convergence Divergence) is a trend-following momentum indicator
				that shows the relationship between two moving averages of price.
			</p>

			<div class="hero-meta">
				<div class="meta-item">
					<IconChartLine size={24} stroke={2} />
					<div>
						<div class="meta-label">Type</div>
						<div class="meta-value">Trend + Momentum</div>
					</div>
				</div>
				<div class="meta-item">
					<IconTarget size={24} stroke={2} />
					<div>
						<div class="meta-label">Components</div>
						<div class="meta-value">3 Lines</div>
					</div>
				</div>
				<div class="meta-item">
					<IconBolt size={24} stroke={2} />
					<div>
						<div class="meta-label">Best For</div>
						<div class="meta-value">Trend Changes</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="content-section">
		<div class="section-container">
			<h2 class="section-title">MACD Components</h2>

			<div class="components-grid">
				<div class="component-card">
					<div class="component-number">1</div>
					<h3>MACD Line</h3>
					<p>
						12-period EMA minus 26-period EMA. This is the faster moving line that reacts to price
						changes.
					</p>
				</div>

				<div class="component-card">
					<div class="component-number">2</div>
					<h3>Signal Line</h3>
					<p>
						9-period EMA of the MACD line. Crossovers with the MACD line generate trading signals.
					</p>
				</div>

				<div class="component-card">
					<div class="component-number">3</div>
					<h3>Histogram</h3>
					<p>
						Difference between MACD line and signal line. Shows momentum strength and potential
						reversals.
					</p>
				</div>
			</div>
		</div>
	</section>

	<section class="strategies-section">
		<div class="section-container">
			<h2 class="section-title">MACD Trading Signals</h2>

			<div class="signals-grid">
				<div class="signal-card bullish">
					<div class="signal-icon">
						<IconTrendingUp size={32} stroke={1.5} />
					</div>
					<h3>Bullish Crossover</h3>
					<p>MACD line crosses above signal line - potential buy signal</p>
					<div class="signal-tip">
						<IconCheck size={16} stroke={2} />
						<span>Confirm with price action</span>
					</div>
				</div>

				<div class="signal-card bearish">
					<div class="signal-icon">
						<IconTrendingUp size={32} stroke={1.5} style="transform: rotate(180deg)" />
					</div>
					<h3>Bearish Crossover</h3>
					<p>MACD line crosses below signal line - potential sell signal</p>
					<div class="signal-tip">
						<IconCheck size={16} stroke={2} />
						<span>Watch for volume confirmation</span>
					</div>
				</div>

				<div class="signal-card divergence">
					<div class="signal-icon">
						<IconWaveSine size={32} stroke={1.5} />
					</div>
					<h3>Divergence</h3>
					<p>Price makes new high/low but MACD doesn't - reversal signal</p>
					<div class="signal-tip">
						<IconAlertTriangle size={16} stroke={2} />
						<span>High probability setup</span>
					</div>
				</div>

				<div class="signal-card histogram">
					<div class="signal-icon">
						<IconChartLine size={32} stroke={1.5} />
					</div>
					<h3>Histogram Reversal</h3>
					<p>Histogram peaks and starts declining - momentum shift</p>
					<div class="signal-tip">
						<IconCheck size={16} stroke={2} />
						<span>Early reversal warning</span>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="cta-section">
		<div class="cta-card">
			<h2>Master MACD Trading</h2>
			<p>Learn advanced MACD strategies in our comprehensive courses</p>
			<a href="/courses" class="cta-button">
				View Courses
				<IconBolt size={20} stroke={2} />
			</a>
		</div>
	</section>
</div>

<style>
	.indicator-page {
		min-height: 100vh;
		background: linear-gradient(to bottom, #0a0f1e 0%, #050812 100%);
		color: white;
	}

	.hero-section {
		position: relative;
		min-height: 80vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 8rem 2rem 4rem;
		overflow: hidden;
		opacity: 0;
		transform: translateY(30px);
		transition:
			opacity 0.8s ease,
			transform 0.8s ease;
	}

	.hero-section.visible {
		opacity: 1;
		transform: translateY(0);
	}

	.hero-background {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		overflow: hidden;
	}

	.glow-orb {
		position: absolute;
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, #34d399 0%, transparent 70%);
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.3;
		top: -200px;
		right: -200px;
		animation: pulse 4s ease-in-out infinite;
	}

	.wave-pattern {
		position: absolute;
		top: 50%;
		left: 0;
		right: 0;
		height: 100px;
		background: linear-gradient(
			90deg,
			transparent 0%,
			rgba(52, 211, 153, 0.2) 50%,
			transparent 100%
		);
		opacity: 0.3;
		animation: wave 4s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			transform: scale(1);
			opacity: 0.3;
		}
		50% {
			transform: scale(1.1);
			opacity: 0.5;
		}
	}

	@keyframes wave {
		0%,
		100% {
			transform: translateY(-20px) scaleY(1);
		}
		50% {
			transform: translateY(20px) scaleY(0.8);
		}
	}

	.hero-content {
		position: relative;
		z-index: 1;
		max-width: 900px;
		text-align: center;
	}

	.hero-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1.25rem;
		background: rgba(52, 211, 153, 0.15);
		border: 1px solid rgba(52, 211, 153, 0.3);
		border-radius: 50px;
		font-size: 0.875rem;
		color: #34d399;
		margin-bottom: 2rem;
	}

	.hero-title {
		font-size: clamp(2.5rem, 5vw, 4.5rem);
		font-weight: 800;
		margin-bottom: 1.5rem;
	}

	.gradient-text {
		background: linear-gradient(135deg, #34d399 0%, #059669 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero-description {
		font-size: clamp(1rem, 2vw, 1.25rem);
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 3rem;
	}

	.hero-meta {
		display: flex;
		justify-content: center;
		gap: 2rem;
		flex-wrap: wrap;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.meta-item :global(svg) {
		color: #34d399;
	}

	.meta-label {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
	}

	.meta-value {
		font-size: 1rem;
		font-weight: 700;
	}

	.content-section {
		padding: 6rem 2rem;
	}

	.section-container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.section-title {
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 800;
		text-align: center;
		margin-bottom: 4rem;
	}

	.components-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 2rem;
	}

	.component-card {
		padding: 2rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		position: relative;
		transition: all 0.3s ease;
	}

	.component-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(52, 211, 153, 0.3);
		transform: translateY(-4px);
	}

	.component-number {
		position: absolute;
		top: 1rem;
		right: 1rem;
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #34d399 0%, #059669 100%);
		border-radius: 50%;
		font-size: 1.25rem;
		font-weight: 700;
	}

	.component-card h3 {
		font-size: 1.5rem;
		margin-bottom: 1rem;
	}

	.component-card p {
		color: rgba(255, 255, 255, 0.7);
		line-height: 1.6;
	}

	.strategies-section {
		padding: 6rem 2rem;
		background: rgba(255, 255, 255, 0.02);
	}

	.signals-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 2rem;
	}

	.signal-card {
		padding: 2rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		transition: all 0.3s ease;
	}

	.signal-card:hover {
		transform: translateY(-4px);
	}

	.signal-card.bullish {
		border-left: 4px solid #34d399;
	}

	.signal-card.bullish:hover {
		border-color: rgba(52, 211, 153, 0.5);
		background: rgba(52, 211, 153, 0.05);
	}

	.signal-card.bearish {
		border-left: 4px solid #ef4444;
	}

	.signal-card.bearish:hover {
		border-color: rgba(239, 68, 68, 0.5);
		background: rgba(239, 68, 68, 0.05);
	}

	.signal-card.divergence {
		border-left: 4px solid #a78bfa;
	}

	.signal-card.divergence:hover {
		border-color: rgba(167, 139, 250, 0.5);
		background: rgba(167, 139, 250, 0.05);
	}

	.signal-card.histogram {
		border-left: 4px solid #2e8eff;
	}

	.signal-card.histogram:hover {
		border-color: rgba(46, 142, 255, 0.5);
		background: rgba(46, 142, 255, 0.05);
	}

	.signal-icon {
		width: 64px;
		height: 64px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #34d399 0%, #059669 100%);
		border-radius: 12px;
		margin-bottom: 1.5rem;
		color: white;
	}

	.signal-card h3 {
		font-size: 1.25rem;
		margin-bottom: 0.75rem;
	}

	.signal-card p {
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 1rem;
	}

	.signal-tip {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(52, 211, 153, 0.1);
		border-radius: 8px;
		font-size: 0.875rem;
		color: #34d399;
	}

	.cta-section {
		padding: 6rem 2rem;
	}

	.cta-card {
		max-width: 800px;
		margin: 0 auto;
		padding: 4rem;
		background: linear-gradient(135deg, rgba(52, 211, 153, 0.1) 0%, rgba(5, 150, 105, 0.1) 100%);
		border: 2px solid rgba(52, 211, 153, 0.3);
		border-radius: 24px;
		text-align: center;
	}

	.cta-card h2 {
		font-size: 2.5rem;
		font-weight: 800;
		margin-bottom: 1rem;
	}

	.cta-card p {
		font-size: 1.125rem;
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 2rem;
	}

	.cta-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1.25rem 2.5rem;
		background: linear-gradient(135deg, #34d399 0%, #059669 100%);
		color: white;
		text-decoration: none;
		border-radius: 12px;
		font-weight: 700;
		font-size: 1.125rem;
		transition: all 0.3s ease;
	}

	.cta-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 15px 40px rgba(52, 211, 153, 0.5);
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.glow-orb {
			width: 500px;
			height: 500px;
		}
	}

	@media (max-width: 1024px) {
		.components-grid {
			grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		}

		.glow-orb {
			width: 400px;
			height: 400px;
		}
	}

	@media (max-width: 768px) {
		.hero-section {
			padding: 6rem 1.5rem 4rem;
			min-height: auto;
		}

		.hero-meta {
			flex-direction: column;
			gap: 1rem;
		}

		.components-grid,
		.signals-grid {
			grid-template-columns: 1fr;
		}

		.cta-card {
			padding: 2rem;
		}

		.cta-card h2 {
			font-size: 2rem;
		}

		.glow-orb {
			width: 300px;
			height: 300px;
		}
	}

	@media (max-width: 640px) {
		.content-section {
			padding: 4rem 1rem;
		}

		.strategies-section {
			padding: 4rem 1rem;
		}

		.cta-section {
			padding: 4rem 1rem;
		}

		.section-container {
			padding: 0 1rem;
		}

		.cta-card {
			padding: 1.5rem;
		}
	}
</style>
