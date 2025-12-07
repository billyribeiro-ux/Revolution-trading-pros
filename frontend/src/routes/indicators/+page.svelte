<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import {
		IconChartLine,
		IconTrendingUp,
		IconTrendingDown,
		IconActivity,
		IconTarget,
		IconBolt,
		IconStar,
		IconCheck,
		IconArrowRight,
		IconChartCandle,
		IconWaveSine,
		IconChartBar
	} from '@tabler/icons-svelte';
	import SEOHead from '$lib/components/SEOHead.svelte';

	interface Indicator {
		id: string;
		name: string;
		slug: string;
		category: string;
		description: string;
		useCase: string;
		difficulty: string;
		icon: any;
		color: string;
		gradient: string;
		features: string[];
	}

	const indicators: Indicator[] = [
		{
			id: '1',
			name: 'RSI - Relative Strength Index',
			slug: 'rsi',
			category: 'Momentum',
			description: 'Measure momentum and identify overbought/oversold conditions with precision.',
			useCase: 'Perfect for finding reversal points and confirming trend strength',
			difficulty: 'Beginner',
			icon: IconActivity,
			color: '#2e8eff',
			gradient: 'linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%)',
			features: ['Overbought/Oversold signals', 'Divergence detection', 'Trend confirmation']
		},
		{
			id: '2',
			name: 'MACD - Moving Average Convergence Divergence',
			slug: 'macd',
			category: 'Trend Following',
			description: 'Identify trend changes and momentum shifts with this powerful indicator.',
			useCase: 'Essential for catching trend reversals and momentum confirmation',
			difficulty: 'Intermediate',
			icon: IconWaveSine,
			color: '#34d399',
			gradient: 'linear-gradient(135deg, #34d399 0%, #059669 100%)',
			features: ['Signal line crossovers', 'Histogram analysis', 'Divergence patterns']
		},
		{
			id: '3',
			name: 'Moving Averages (SMA/EMA)',
			slug: 'moving-averages',
			category: 'Trend Following',
			description: 'Smooth price action and identify trend direction with moving averages.',
			useCase: 'Foundation for trend trading and dynamic support/resistance',
			difficulty: 'Beginner',
			icon: IconChartLine,
			color: '#a78bfa',
			gradient: 'linear-gradient(135deg, #a78bfa 0%, #7c3aed 100%)',
			features: ['Golden/Death cross', 'Dynamic support/resistance', 'Trend identification']
		},
		{
			id: '4',
			name: 'Bollinger Bands',
			slug: 'bollinger-bands',
			category: 'Volatility',
			description: 'Measure volatility and identify potential breakouts or reversals.',
			useCase: 'Excellent for volatility-based trading and squeeze setups',
			difficulty: 'Intermediate',
			icon: IconChartBar,
			color: '#fb923c',
			gradient: 'linear-gradient(135deg, #fb923c 0%, #ea580c 100%)',
			features: ['Volatility measurement', 'Squeeze patterns', 'Breakout signals']
		},
		{
			id: '5',
			name: 'VWAP - Volume Weighted Average Price',
			slug: 'vwap',
			category: 'Volume',
			description: 'Institutional-level price benchmark weighted by volume.',
			useCase: 'Used by institutions for optimal execution and fair value',
			difficulty: 'Beginner',
			icon: IconChartCandle,
			color: '#f59e0b',
			gradient: 'linear-gradient(135deg, #f59e0b 0%, #d97706 100%)',
			features: ['Fair value benchmark', 'Intraday support/resistance', 'Institutional levels']
		},
		{
			id: '6',
			name: 'Stochastic Oscillator',
			slug: 'stochastic',
			category: 'Momentum',
			description: 'Compare closing price to price range over time for momentum insights.',
			useCase: 'Identify overbought/oversold extremes and potential reversals',
			difficulty: 'Intermediate',
			icon: IconActivity,
			color: '#ec4899',
			gradient: 'linear-gradient(135deg, #ec4899 0%, #be185d 100%)',
			features: ['%K and %D lines', 'Overbought/Oversold zones', 'Crossover signals']
		}
	];

	let heroVisible = false;
	let cardsVisible: boolean[] = new Array(indicators.length).fill(false);
	let selectedCategory = 'All';

	const categories = ['All', 'Momentum', 'Trend Following', 'Volatility', 'Volume'];

	let filteredIndicators = $derived(
		selectedCategory === 'All'
			? indicators
			: indicators.filter((ind) => ind.category === selectedCategory)
	);

	// Indicators structured data for rich snippets
	const indicatorsSchema = [
		{
			'@context': 'https://schema.org',
			'@type': 'ItemList',
			'@id': 'https://revolutiontradingpros.com/indicators/#indicatorlist',
			name: 'Professional Trading Indicators',
			description: 'Technical analysis indicators used by professional traders',
			numberOfItems: indicators.length,
			itemListElement: indicators.map((indicator, index) => ({
				'@type': 'ListItem',
				position: index + 1,
				item: {
					'@type': 'SoftwareApplication',
					name: indicator.name,
					description: indicator.description,
					url: `https://revolutiontradingpros.com/indicators/${indicator.slug}`,
					applicationCategory: 'FinanceApplication',
					operatingSystem: 'Web Browser',
					offers: {
						'@type': 'Offer',
						price: '0',
						priceCurrency: 'USD'
					}
				}
			}))
		},
		{
			'@context': 'https://schema.org',
			'@type': 'HowTo',
			'@id': 'https://revolutiontradingpros.com/indicators/#howto',
			name: 'How to Use Technical Trading Indicators',
			description:
				'Learn to use professional-grade technical indicators to identify high-probability setups, confirm trends, and time your entries with precision.',
			step: [
				{
					'@type': 'HowToStep',
					name: 'Identify Trends',
					text: 'Use trend-following indicators like Moving Averages and MACD to spot market direction.'
				},
				{
					'@type': 'HowToStep',
					name: 'Measure Momentum',
					text: 'Use momentum oscillators like RSI and Stochastic to gauge strength of moves.'
				},
				{
					'@type': 'HowToStep',
					name: 'Time Entries',
					text: 'Combine multiple indicators to confirm setups and optimize entry timing.'
				}
			]
		}
	];

	onMount(() => {
		if (!browser) return;

		const heroObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						heroVisible = true;
					}
				});
			},
			{ threshold: 0.2 }
		);

		const cardObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const index = parseInt(entry.target.getAttribute('data-index') || '0');
						cardsVisible[index] = true;
					}
				});
			},
			{ threshold: 0.1 }
		);

		const heroElement = document.querySelector('.hero-section');
		if (heroElement) heroObserver.observe(heroElement);

		const cardElements = document.querySelectorAll('.indicator-card');
		cardElements.forEach((card) => cardObserver.observe(card));

		return () => {
			heroObserver.disconnect();
			cardObserver.disconnect();
		};
	});
</script>

<SEOHead
	title="Professional Trading Indicators"
	description="Master technical indicators used by professional traders. RSI, MACD, Moving Averages, Bollinger Bands, VWAP, and more. Free guides and proven strategies."
	canonical="/indicators"
	ogType="website"
	ogImage="/og-image.webp"
	ogImageAlt="Revolution Trading Pros Technical Indicators"
	keywords={[
		'trading indicators',
		'technical indicators',
		'RSI indicator',
		'MACD indicator',
		'moving averages',
		'bollinger bands',
		'VWAP',
		'momentum indicators',
		'trend indicators',
		'technical analysis'
	]}
	schema={indicatorsSchema}
/>

<div class="indicators-page">
	<!-- Hero Section -->
	<section class="hero-section" class:visible={heroVisible}>
		<div class="hero-background">
			<div class="glow-orb glow-orb-1"></div>
			<div class="glow-orb glow-orb-2"></div>
			<div class="chart-lines">
				<div class="chart-line line-1"></div>
				<div class="chart-line line-2"></div>
				<div class="chart-line line-3"></div>
			</div>
		</div>

		<div class="hero-content">
			<div class="hero-badge">
				<IconChartLine size={20} stroke={2} />
				<span>Professional Trading Indicators</span>
			</div>

			<h1 class="hero-title">
				Master Technical<br />
				<span class="gradient-text">Indicators</span>
			</h1>

			<p class="hero-description">
				Learn to use professional-grade technical indicators to identify high-probability setups,
				confirm trends, and time your entries with precision. From momentum oscillators to
				trend-following tools, master the indicators used by institutional traders.
			</p>

			<div class="hero-stats">
				<div class="stat-item">
					<IconTarget size={32} stroke={1.5} class="stat-icon" />
					<div class="stat-content">
						<div class="stat-value">15+</div>
						<div class="stat-label">Pro Indicators</div>
					</div>
				</div>
				<div class="stat-item">
					<IconBolt size={32} stroke={1.5} class="stat-icon" />
					<div class="stat-content">
						<div class="stat-value">Real-Time</div>
						<div class="stat-label">Market Analysis</div>
					</div>
				</div>
				<div class="stat-item">
					<IconStar size={32} stroke={1.5} class="stat-icon" />
					<div class="stat-content">
						<div class="stat-value">Proven</div>
						<div class="stat-label">Strategies</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Category Filter -->
	<section class="filter-section">
		<div class="filter-container">
			<h3>Filter by Category</h3>
			<div class="filter-buttons">
				{#each categories as category}
					<button
						class="filter-button"
						class:active={selectedCategory === category}
						onclick={() => (selectedCategory = category)}
					>
						{category}
					</button>
				{/each}
			</div>
		</div>
	</section>

	<!-- Indicators Grid -->
	<section class="indicators-section">
		<div class="section-container">
			<div class="indicators-grid">
				{#each filteredIndicators as indicator, index}
					<article
						class="indicator-card"
						class:visible={cardsVisible[index]}
						data-index={index}
						style="--delay: {index * 0.1}s; --card-color: {indicator.color}"
					>
						<div class="card-header" style="background: {indicator.gradient}">
							<div class="card-icon">
								<svelte:component this={indicator.icon} size={48} stroke={1.5} />
							</div>
							<div class="card-category">{indicator.category}</div>
						</div>

						<div class="card-content">
							<h3 class="card-title">{indicator.name}</h3>
							<p class="card-description">{indicator.description}</p>

							<div class="card-use-case">
								<IconTarget size={18} stroke={2} />
								<span>{indicator.useCase}</span>
							</div>

							<div class="card-meta">
								<div class="meta-badge difficulty">
									<IconStar size={16} stroke={2} />
									<span>{indicator.difficulty}</span>
								</div>
							</div>

							<ul class="card-features">
								{#each indicator.features as feature}
									<li>
										<IconCheck size={16} stroke={2} />
										<span>{feature}</span>
									</li>
								{/each}
							</ul>

							<a href="/indicators/{indicator.slug}" class="card-button">
								Learn More
								<IconArrowRight size={18} stroke={2} />
							</a>
						</div>
					</article>
				{/each}
			</div>
		</div>
	</section>

	<!-- Why Use Indicators -->
	<section class="benefits-section">
		<div class="section-container">
			<h2 class="section-title">Why Use Technical Indicators?</h2>

			<div class="benefits-grid">
				<div class="benefit-card">
					<div class="benefit-icon">
						<IconTrendingUp size={32} stroke={1.5} />
					</div>
					<h3>Identify Trends</h3>
					<p>Spot trends early and ride them for maximum profit with trend-following indicators.</p>
				</div>

				<div class="benefit-card">
					<div class="benefit-icon">
						<IconActivity size={32} stroke={1.5} />
					</div>
					<h3>Measure Momentum</h3>
					<p>Gauge market momentum to identify strong moves and potential exhaustion points.</p>
				</div>

				<div class="benefit-card">
					<div class="benefit-icon">
						<IconTarget size={32} stroke={1.5} />
					</div>
					<h3>Time Entries</h3>
					<p>Improve entry and exit timing with precise signals from oscillators and crossovers.</p>
				</div>

				<div class="benefit-card">
					<div class="benefit-icon">
						<IconTrendingDown size={32} stroke={1.5} />
					</div>
					<h3>Confirm Setups</h3>
					<p>Validate trade ideas with multiple indicators for higher probability setups.</p>
				</div>
			</div>
		</div>
	</section>
</div>

<style>
	.indicators-page {
		min-height: 100vh;
		background: linear-gradient(to bottom, #0a0f1e 0%, #050812 100%);
		color: white;
	}

	/* Hero Section */
	.hero-section {
		position: relative;
		min-height: 85vh;
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
		z-index: 0;
	}

	.glow-orb {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.3;
		animation: pulse 4s ease-in-out infinite;
	}

	.glow-orb-1 {
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, #2e8eff 0%, transparent 70%);
		top: -200px;
		right: -200px;
	}

	.glow-orb-2 {
		width: 500px;
		height: 500px;
		background: radial-gradient(circle, #34d399 0%, transparent 70%);
		bottom: -150px;
		left: -150px;
		animation-delay: 2s;
	}

	.chart-lines {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 100%;
		max-width: 1000px;
		height: 400px;
	}

	.chart-line {
		position: absolute;
		width: 100%;
		height: 2px;
		background: linear-gradient(90deg, transparent 0%, var(--line-color) 50%, transparent 100%);
		opacity: 0.2;
		animation: slideLine 3s ease-in-out infinite;
	}

	.line-1 {
		--line-color: #2e8eff;
		top: 30%;
		animation-delay: 0s;
	}

	.line-2 {
		--line-color: #34d399;
		top: 50%;
		animation-delay: 1s;
	}

	.line-3 {
		--line-color: #a78bfa;
		top: 70%;
		animation-delay: 2s;
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

	@keyframes slideLine {
		0%,
		100% {
			transform: translateX(-10%);
			opacity: 0.1;
		}
		50% {
			transform: translateX(10%);
			opacity: 0.3;
		}
	}

	.hero-content {
		position: relative;
		z-index: 1;
		max-width: 1000px;
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
		font-weight: 500;
		color: #2e8eff;
		margin-bottom: 2rem;
		backdrop-filter: blur(10px);
	}

	.hero-title {
		font-size: clamp(2.5rem, 5vw, 4.5rem);
		font-weight: 800;
		line-height: 1.1;
		margin-bottom: 1.5rem;
		letter-spacing: -0.02em;
	}

	.gradient-text {
		background: linear-gradient(135deg, #2e8eff 0%, #34d399 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero-description {
		font-size: clamp(1rem, 2vw, 1.25rem);
		line-height: 1.7;
		color: rgba(255, 255, 255, 0.7);
		max-width: 850px;
		margin: 0 auto 3rem;
	}

	.hero-stats {
		display: flex;
		justify-content: center;
		gap: 3rem;
		flex-wrap: wrap;
	}

	.stat-item {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.stat-content {
		text-align: left;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
	}

	.stat-label {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.6);
	}

	/* Filter Section */
	.filter-section {
		padding: 2rem 2rem 4rem;
		background: rgba(255, 255, 255, 0.02);
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.filter-container {
		max-width: 1400px;
		margin: 0 auto;
		text-align: center;
	}

	.filter-container h3 {
		font-size: 1.25rem;
		font-weight: 700;
		margin-bottom: 1.5rem;
	}

	.filter-buttons {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.filter-button {
		padding: 0.75rem 1.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 50px;
		color: rgba(255, 255, 255, 0.7);
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.filter-button:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(46, 142, 255, 0.3);
		color: white;
	}

	.filter-button.active {
		background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
		border-color: #2e8eff;
		color: white;
	}

	/* Indicators Section */
	.indicators-section {
		padding: 6rem 2rem;
	}

	.section-container {
		max-width: 1400px;
		margin: 0 auto;
	}

	.section-title {
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 800;
		text-align: center;
		margin-bottom: 4rem;
	}

	.indicators-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
		gap: 2rem;
	}

	.indicator-card {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 20px;
		overflow: hidden;
		transition: all 0.4s ease;
		opacity: 0;
		transform: translateY(30px);
	}

	.indicator-card.visible {
		opacity: 1;
		transform: translateY(0);
		transition-delay: var(--delay);
	}

	.indicator-card:hover {
		transform: translateY(-8px);
		border-color: var(--card-color);
		box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
	}

	.card-header {
		position: relative;
		padding: 3rem 2rem;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.card-icon {
		color: white;
		filter: drop-shadow(0 0 10px rgba(255, 255, 255, 0.3));
	}

	.card-category {
		position: absolute;
		top: 1rem;
		right: 1rem;
		padding: 0.4rem 1rem;
		background: rgba(0, 0, 0, 0.3);
		backdrop-filter: blur(10px);
		border-radius: 50px;
		font-size: 0.75rem;
		font-weight: 600;
		color: white;
	}

	.card-content {
		padding: 2rem;
	}

	.card-title {
		font-size: 1.5rem;
		font-weight: 700;
		margin-bottom: 1rem;
	}

	.card-description {
		font-size: 0.9375rem;
		line-height: 1.6;
		color: rgba(255, 255, 255, 0.7);
		margin-bottom: 1.5rem;
	}

	.card-use-case {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.05);
		border-left: 3px solid var(--card-color);
		border-radius: 8px;
		margin-bottom: 1.5rem;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.8);
	}

	.card-use-case :global(svg) {
		color: var(--card-color);
		flex-shrink: 0;
		margin-top: 2px;
	}

	.card-meta {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.meta-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 50px;
		font-size: 0.8125rem;
		font-weight: 600;
	}

	.meta-badge.difficulty :global(svg) {
		color: #f59e0b;
	}

	.card-features {
		list-style: none;
		padding: 0;
		margin: 0 0 2rem 0;
	}

	.card-features li {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 0;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.8);
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.card-features li:last-child {
		border-bottom: none;
	}

	.card-features li :global(svg) {
		color: var(--card-color);
		flex-shrink: 0;
	}

	.card-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		justify-content: center;
		padding: 1rem 1.5rem;
		background: rgba(255, 255, 255, 0.05);
		color: white;
		text-decoration: none;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9375rem;
		transition: all 0.3s ease;
	}

	.card-button:hover {
		background: var(--card-color);
		border-color: var(--card-color);
		transform: translateX(4px);
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.3);
	}

	/* Benefits Section */
	.benefits-section {
		padding: 6rem 2rem;
		background: rgba(255, 255, 255, 0.02);
	}

	.benefits-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 2rem;
		margin-top: 3rem;
	}

	.benefit-card {
		padding: 2rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		transition: all 0.3s ease;
	}

	.benefit-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(46, 142, 255, 0.3);
		transform: translateY(-4px);
	}

	.benefit-icon {
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

	.benefit-card h3 {
		font-size: 1.25rem;
		font-weight: 700;
		margin-bottom: 0.75rem;
	}

	.benefit-card p {
		font-size: 0.9375rem;
		line-height: 1.6;
		color: rgba(255, 255, 255, 0.7);
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.glow-orb-1 {
			width: 500px;
			height: 500px;
		}

		.glow-orb-2 {
			width: 450px;
			height: 450px;
		}
	}

	@media (max-width: 1024px) {
		.indicators-grid {
			grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		}

		.glow-orb-1 {
			width: 400px;
			height: 400px;
		}

		.glow-orb-2 {
			width: 350px;
			height: 350px;
		}
	}

	@media (max-width: 768px) {
		.hero-section {
			padding: 6rem 1.5rem 4rem;
			min-height: auto;
		}

		.indicators-grid {
			grid-template-columns: 1fr;
		}

		.hero-stats {
			flex-direction: column;
			gap: 1.5rem;
		}

		.stat-item {
			justify-content: center;
		}

		.filter-section {
			padding: 2rem 1rem;
		}

		.glow-orb-1 {
			width: 300px;
			height: 300px;
		}

		.glow-orb-2 {
			width: 250px;
			height: 250px;
		}
	}

	@media (max-width: 640px) {
		.section-container {
			padding: 0 1rem;
		}

		.indicators-section {
			padding: 4rem 1rem;
		}

		.benefits-section {
			padding: 4rem 1rem;
		}

		.benefits-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
