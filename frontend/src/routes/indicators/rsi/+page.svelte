<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import {
		IconActivity,
		IconTrendingUp,
		IconTrendingDown,
		IconCheck,
		IconAlertTriangle,
		IconBolt,
		IconChartLine,
		IconTarget
	} from '$lib/icons';

	let heroVisible = false;
	let sectionsVisible: boolean[] = [false, false, false];

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

		const sectionObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const index = parseInt(entry.target.getAttribute('data-section') || '0');
						sectionsVisible[index] = true;
					}
				});
			},
			{ threshold: 0.1 }
		);

		const heroElement = document.querySelector('.hero-section');
		if (heroElement) heroObserver.observe(heroElement);

		const sectionElements = document.querySelectorAll('.animated-section');
		sectionElements.forEach((section) => sectionObserver.observe(section));

		return () => {
			heroObserver.disconnect();
			sectionObserver.disconnect();
		};
	});
</script>

<svelte:head>
	<title>RSI - Relative Strength Index | Revolution Trading</title>
	<meta
		name="description"
		content="Master the RSI indicator to identify overbought/oversold conditions and reversal points with precision."
	/>
	<meta property="og:title" content="RSI - Relative Strength Index | Revolution Trading" />
	<meta
		property="og:description"
		content="Master the RSI indicator to identify overbought/oversold conditions and reversal points with precision."
	/>
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="RSI Indicator | Revolution Trading" />
	<meta
		name="twitter:description"
		content="Master the RSI indicator to identify overbought/oversold conditions and reversal points with precision."
	/>
</svelte:head>

<div class="indicator-page">
	<!-- Hero Section -->
	<section class="hero-section" class:visible={heroVisible}>
		<div class="hero-background">
			<div class="glow-orb"></div>
			<div class="oscillator-line"></div>
		</div>

		<div class="hero-content">
			<div class="hero-badge">
				<IconActivity size={20} stroke={2} />
				<span>Momentum Indicator</span>
			</div>

			<h1 class="hero-title">
				RSI - Relative<br />
				<span class="gradient-text">Strength Index</span>
			</h1>

			<p class="hero-description">
				The RSI is a momentum oscillator that measures the speed and magnitude of price changes.
				Used by professional traders to identify overbought/oversold conditions and potential
				reversals.
			</p>

			<div class="hero-meta">
				<div class="meta-item">
					<IconChartLine size={24} stroke={2} />
					<div>
						<div class="meta-label">Type</div>
						<div class="meta-value">Oscillator</div>
					</div>
				</div>
				<div class="meta-item">
					<IconTarget size={24} stroke={2} />
					<div>
						<div class="meta-label">Range</div>
						<div class="meta-value">0-100</div>
					</div>
				</div>
				<div class="meta-item">
					<IconBolt size={24} stroke={2} />
					<div>
						<div class="meta-label">Best For</div>
						<div class="meta-value">Reversals</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- How It Works -->
	<section class="animated-section" class:visible={sectionsVisible[0]} data-section="0">
		<div class="section-container">
			<h2 class="section-title">How RSI Works</h2>

			<div class="content-grid">
				<div class="content-card">
					<div class="card-number">1</div>
					<h3>Calculation</h3>
					<p>
						RSI = 100 - (100 / (1 + RS)) where RS = Average Gain / Average Loss over 14 periods.
					</p>
				</div>

				<div class="content-card">
					<div class="card-number">2</div>
					<h3>Interpretation</h3>
					<p>
						Values above 70 indicate overbought conditions. Values below 30 indicate oversold
						conditions.
					</p>
				</div>

				<div class="content-card">
					<div class="card-number">3</div>
					<h3>Signals</h3>
					<p>
						Look for divergences between price and RSI, as well as crossovers of the 30 and 70
						levels.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Key Levels -->
	<section
		class="levels-section animated-section"
		class:visible={sectionsVisible[1]}
		data-section="1"
	>
		<div class="section-container">
			<h2 class="section-title">RSI Key Levels</h2>

			<div class="levels-visual">
				<div class="level-bar">
					<div class="level-zone overbought">
						<div class="zone-label">
							<IconTrendingUp size={20} stroke={2} />
							<span>Overbought (70-100)</span>
						</div>
						<p>Potential selling opportunity or trend continuation in strong uptrends</p>
					</div>

					<div class="level-zone neutral">
						<div class="zone-label">
							<IconActivity size={20} stroke={2} />
							<span>Neutral (30-70)</span>
						</div>
						<p>Normal trading range with no extreme conditions</p>
					</div>

					<div class="level-zone oversold">
						<div class="zone-label">
							<IconTrendingDown size={20} stroke={2} />
							<span>Oversold (0-30)</span>
						</div>
						<p>Potential buying opportunity or trend continuation in strong downtrends</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Trading Strategies -->
	<section
		class="strategies-section animated-section"
		class:visible={sectionsVisible[2]}
		data-section="2"
	>
		<div class="section-container">
			<h2 class="section-title">RSI Trading Strategies</h2>

			<div class="strategies-grid">
				<div class="strategy-card">
					<div class="strategy-icon">
						<IconCheck size={32} stroke={1.5} />
					</div>
					<h3>Overbought/Oversold</h3>
					<p>Classic strategy: Buy when RSI crosses above 30, sell when it crosses below 70.</p>
					<div class="strategy-tip">
						<IconAlertTriangle size={16} stroke={2} />
						<span>Works best in ranging markets</span>
					</div>
				</div>

				<div class="strategy-card">
					<div class="strategy-icon">
						<IconCheck size={32} stroke={1.5} />
					</div>
					<h3>Divergence Trading</h3>
					<p>Look for divergence between price action and RSI to identify potential reversals.</p>
					<div class="strategy-tip">
						<IconAlertTriangle size={16} stroke={2} />
						<span>High probability reversal signal</span>
					</div>
				</div>

				<div class="strategy-card">
					<div class="strategy-icon">
						<IconCheck size={32} stroke={1.5} />
					</div>
					<h3>Trend Confirmation</h3>
					<p>In uptrends, use RSI pullbacks to 40-50 as buying opportunities.</p>
					<div class="strategy-tip">
						<IconAlertTriangle size={16} stroke={2} />
						<span>Aligns with trend direction</span>
					</div>
				</div>

				<div class="strategy-card">
					<div class="strategy-icon">
						<IconCheck size={32} stroke={1.5} />
					</div>
					<h3>Failure Swings</h3>
					<p>Identify RSI failure swings for high-probability reversal signals.</p>
					<div class="strategy-tip">
						<IconAlertTriangle size={16} stroke={2} />
						<span>Advanced pattern recognition</span>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- CTA -->
	<section class="cta-section">
		<div class="cta-card">
			<h2>Master RSI Trading</h2>
			<p>Learn advanced RSI strategies in our professional trading courses</p>
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

	/* Hero Section */
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
		background: radial-gradient(circle, #2e8eff 0%, transparent 70%);
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.3;
		top: -200px;
		right: -200px;
		animation: pulse 4s ease-in-out infinite;
	}

	.oscillator-line {
		position: absolute;
		top: 50%;
		left: 0;
		right: 0;
		height: 2px;
		background: linear-gradient(90deg, transparent 0%, #2e8eff 50%, transparent 100%);
		opacity: 0.3;
		animation: oscillate 3s ease-in-out infinite;
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

	@keyframes oscillate {
		0%,
		100% {
			transform: translateX(-20%);
		}
		50% {
			transform: translateX(20%);
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
		background: rgba(46, 142, 255, 0.15);
		border: 1px solid rgba(46, 142, 255, 0.3);
		border-radius: 50px;
		font-size: 0.875rem;
		color: #2e8eff;
		margin-bottom: 2rem;
	}

	.hero-title {
		font-size: clamp(2.5rem, 5vw, 4.5rem);
		font-weight: 800;
		margin-bottom: 1.5rem;
	}

	.gradient-text {
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero-description {
		font-size: clamp(1rem, 2vw, 1.25rem);
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 3rem;
		max-width: 700px;
		margin-left: auto;
		margin-right: auto;
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
		color: #2e8eff;
	}

	.meta-label {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
	}

	.meta-value {
		font-size: 1rem;
		font-weight: 700;
	}

	/* Animated Sections */
	.animated-section {
		padding: 6rem 2rem;
		opacity: 0;
		transform: translateY(30px);
		transition:
			opacity 0.8s ease,
			transform 0.8s ease;
	}

	.animated-section.visible {
		opacity: 1;
		transform: translateY(0);
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

	/* Content Grid */
	.content-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 2rem;
	}

	.content-card {
		padding: 2rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		position: relative;
		transition: all 0.3s ease;
	}

	.content-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(46, 142, 255, 0.3);
		transform: translateY(-4px);
	}

	.card-number {
		position: absolute;
		top: 1rem;
		right: 1rem;
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		border-radius: 50%;
		font-size: 1.25rem;
		font-weight: 700;
	}

	.content-card h3 {
		font-size: 1.5rem;
		margin-bottom: 1rem;
	}

	.content-card p {
		color: rgba(255, 255, 255, 0.7);
		line-height: 1.6;
	}

	/* Levels Section */
	.levels-section {
		background: rgba(255, 255, 255, 0.02);
	}

	.levels-visual {
		max-width: 900px;
		margin: 0 auto;
	}

	.level-bar {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.level-zone {
		padding: 2rem;
		border-radius: 16px;
		border-left: 4px solid;
		transition: all 0.3s ease;
	}

	.level-zone:hover {
		transform: translateX(8px);
	}

	.level-zone.overbought {
		background: linear-gradient(90deg, rgba(239, 68, 68, 0.1) 0%, transparent 100%);
		border-left-color: #ef4444;
	}

	.level-zone.neutral {
		background: linear-gradient(90deg, rgba(46, 142, 255, 0.1) 0%, transparent 100%);
		border-left-color: #2e8eff;
	}

	.level-zone.oversold {
		background: linear-gradient(90deg, rgba(52, 211, 153, 0.1) 0%, transparent 100%);
		border-left-color: #34d399;
	}

	.zone-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 1.25rem;
		font-weight: 700;
		margin-bottom: 0.75rem;
	}

	.level-zone p {
		color: rgba(255, 255, 255, 0.7);
	}

	/* Strategies Section */
	.strategies-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 2rem;
	}

	.strategy-card {
		padding: 2rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		transition: all 0.3s ease;
	}

	.strategy-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(46, 142, 255, 0.3);
		transform: translateY(-4px);
	}

	.strategy-icon {
		width: 64px;
		height: 64px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		border-radius: 12px;
		margin-bottom: 1.5rem;
		color: white;
	}

	.strategy-card h3 {
		font-size: 1.25rem;
		margin-bottom: 0.75rem;
	}

	.strategy-card p {
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 1rem;
		line-height: 1.6;
	}

	.strategy-tip {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(46, 142, 255, 0.1);
		border-radius: 8px;
		font-size: 0.875rem;
		color: #2e8eff;
	}

	/* CTA Section */
	.cta-section {
		padding: 6rem 2rem;
	}

	.cta-card {
		max-width: 800px;
		margin: 0 auto;
		padding: 4rem;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.1) 0%, rgba(30, 92, 184, 0.1) 100%);
		border: 2px solid rgba(46, 142, 255, 0.3);
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
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		color: white;
		text-decoration: none;
		border-radius: 12px;
		font-weight: 700;
		font-size: 1.125rem;
		transition: all 0.3s ease;
	}

	.cta-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 15px 40px rgba(46, 142, 255, 0.5);
	}

	/* Responsive */
	/* Responsive */
	@media (max-width: 1200px) {
		.glow-orb {
			width: 500px;
			height: 500px;
		}
	}

	@media (max-width: 1024px) {
		.content-grid {
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

		.content-grid,
		.strategies-grid {
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
		.animated-section {
			padding: 4rem 1rem;
		}

		.section-container {
			padding: 0 1rem;
		}

		.cta-section {
			padding: 4rem 1rem;
		}

		.cta-card {
			padding: 1.5rem;
		}

		.level-zone {
			padding: 1.5rem;
		}
	}
</style>
