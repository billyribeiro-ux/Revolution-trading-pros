<!--
	URL: /resources/stock-indexes-list
-->

<script lang="ts">
	/**
	 * Stock Indexes List Page - Google L11 Enterprise Standard
	 * Comprehensive stock index education and reference resource
	 */
	import SEOHead from '$lib/components/SEOHead.svelte';
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import {
		IconChartLine,
		IconWorld,
		IconArrowRight,
		IconQuestionMark,
		IconExternalLink,
		IconChartBar,
		IconBuildingSkyscraper
	} from '$lib/icons';

	// Major Stock Indexes with detailed info
	const majorIndexes = [
		{
			symbol: 'SPX',
			ticker: '$SPX',
			name: 'S&P 500',
			description:
				'Tracks the 500 largest companies listed on major U.S. exchanges. Market capitalization-weighted, offering comprehensive representation of the U.S. stock market.',
			etfs: ['SPY', 'VOO', 'IVV'],
			region: 'US',
			companies: 500,
			type: 'Large-Cap'
		},
		{
			symbol: 'DJI',
			ticker: '$DJI',
			name: 'Dow Jones Industrial Average',
			description:
				'Measures the top 30 U.S. large-cap stocks. One of the oldest and most widely followed indexes, representing blue-chip companies.',
			etfs: ['DIA'],
			region: 'US',
			companies: 30,
			type: 'Large-Cap'
		},
		{
			symbol: 'COMPQ',
			ticker: '$COMPQ',
			name: 'NASDAQ Composite',
			description:
				'Includes all stocks listed on the NASDAQ exchange. Heavily weighted toward technology and growth companies.',
			etfs: ['QQQ', 'ONEQ'],
			region: 'US',
			companies: 3000,
			type: 'All-Cap'
		},
		{
			symbol: 'NDX',
			ticker: '$NDX',
			name: 'NASDAQ 100',
			description:
				'Measures the top 100 largest non-financial companies listed on NASDAQ. Known for its tech-heavy composition.',
			etfs: ['QQQ', 'QQQM'],
			region: 'US',
			companies: 100,
			type: 'Large-Cap'
		},
		{
			symbol: 'RUT',
			ticker: '$RUT',
			name: 'Russell 2000',
			description:
				'Tracks 2,000 small-cap U.S. companies. A benchmark for small-cap stock performance and economic health.',
			etfs: ['IWM', 'VTWO'],
			region: 'US',
			companies: 2000,
			type: 'Small-Cap'
		},
		{
			symbol: 'VIX',
			ticker: '$VIX',
			name: 'CBOE Volatility Index',
			description:
				'Measures market expectations of near-term volatility. Known as the "fear gauge" of the market.',
			etfs: ['VXX', 'UVXY'],
			region: 'US',
			companies: 0,
			type: 'Volatility'
		},
		{
			symbol: 'FTSE',
			ticker: 'FTSE',
			name: 'FTSE 100',
			description:
				'Tracks the 100 largest companies listed on the London Stock Exchange by market capitalization.',
			etfs: ['ISF', 'VUKE'],
			region: 'UK',
			companies: 100,
			type: 'Large-Cap'
		},
		{
			symbol: 'DAX',
			ticker: 'DAX',
			name: 'DAX 40',
			description:
				"Germany's premier stock index, tracking the 40 largest and most liquid companies on the Frankfurt Stock Exchange.",
			etfs: ['EWG', 'DAX'],
			region: 'Germany',
			companies: 40,
			type: 'Large-Cap'
		},
		{
			symbol: 'N225',
			ticker: 'N225',
			name: 'Nikkei 225',
			description:
				"Japan's leading stock index, tracking 225 large, publicly owned companies from a wide range of industries.",
			etfs: ['EWJ', 'DXJ'],
			region: 'Japan',
			companies: 225,
			type: 'Large-Cap'
		},
		{
			symbol: 'HSI',
			ticker: 'HSI',
			name: 'Hang Seng Index',
			description:
				'Tracks the largest companies listed on the Hong Kong Stock Exchange, representing about 60% of total market cap.',
			etfs: ['EWH', 'FXI'],
			region: 'Hong Kong',
			companies: 82,
			type: 'Large-Cap'
		}
	];

	// FAQ Data
	const faqItems = [
		{
			question: 'What are the three major stock indexes?',
			answer:
				'1. S&P 500: measures 500 large-caps in U.S. stock market\n2. Dow Jones 30: measures the top 30 U.S. large-cap stocks\n3. Nasdaq 100: measures the top 100 largest non-financial companies'
		},
		{
			question: 'What is the difference between a stock and an index?',
			answer:
				'A stock allows investors partial ownership in a company. An index gives investors the option of investing in a basket of companies through products like ETFs, mutual funds, and derivatives.'
		},
		{
			question: 'Can you invest directly in an index?',
			answer:
				'An index cannot be traded or invested in directly. Many different products, such as ETFs, mutual funds, and derivatives, are available to investors to gain exposure to index performance.'
		},
		{
			question: 'What are the ticker symbols for major indexes?',
			answer:
				'1. S&P 500 – SPX or $SPX\n2. Dow Jones Industrial Average (DJIA) – $DJI\n3. Nasdaq Composite – COMPQ or $COMPQ\n4. NYSE Composite – (DJ)\n5. Russell 2000 – RUT or $RUT'
		},
		{
			question: 'What is the most widely cited US stock market index?',
			answer:
				'The most widely cited US stock market index is the S&P 500, which measures 500 large-cap stocks and is considered the best representation of the overall U.S. stock market.'
		}
	];

	let selectedRegion = $state('all');
	let expandedFaq = $state<number | null>(null);

	let regions = $derived(['all', ...new Set(majorIndexes.map((i) => i.region))]);

	let filteredIndexes = $derived(
		majorIndexes.filter((index) => {
			return selectedRegion === 'all' || index.region === selectedRegion;
		})
	);

	function toggleFaq(index: number) {
		expandedFaq = expandedFaq === index ? null : index;
	}
</script>

<SEOHead
	title="Stock Indexes List"
	description="The S&P 500 is a prominent stock index that tracks the 500 largest companies listed on major U.S. exchanges. Learn about major stock indexes and how to trade them."
	canonical="/resources/stock-indexes-list"
	ogType="website"
	keywords={[
		'stock indexes',
		'S&P 500',
		'Dow Jones',
		'NASDAQ',
		'Russell 2000',
		'market indexes',
		'stock market'
	]}
/>

<div class="indexes-page">
	<!-- Hero Section -->
	<section class="hero">
		<div class="hero__bg">
			<div class="hero__gradient"></div>
			<div class="hero__grid"></div>
		</div>
		<div class="hero__content" in:fly={{ y: 30, duration: 600, easing: cubicOut }}>
			<div class="hero__badge">
				<IconChartBar size={20} />
				<span>Market Reference</span>
			</div>
			<h1 class="hero__title">Stock Indexes List</h1>
			<p class="hero__subtitle">
				Your comprehensive guide to major stock market indexes. Understand how indexes work, track
				market performance, and discover the best ways to gain exposure.
			</p>
		</div>
	</section>

	<!-- S&P 500 Feature Section -->
	<section class="section" in:fade={{ delay: 200, duration: 400 }}>
		<div class="container">
			<div class="sp500-feature">
				<div class="sp500-feature__content">
					<div class="sp500-feature__badge">Featured Index</div>
					<h2 class="sp500-feature__title">S&P 500 Index</h2>
					<p class="sp500-feature__text">
						The S&P 500 is a prominent stock index that tracks the 500 largest companies listed on
						major U.S. exchanges, such as the NYSE and NASDAQ. This index is weighted based on the
						total market capitalization of each company's outstanding shares, which means it offers
						a comprehensive representation of the overall U.S. stock market, more so than other
						indexes like the Dow Jones Industrial Average (DJIA) or the Russell 2000.
					</p>
					<p class="sp500-feature__text">
						Recognized globally as one of the most influential stock indexes, the S&P 500 is market
						capitalization-weighted. This indicates that companies with larger market values have a
						more significant impact on the index's overall performance compared to smaller
						companies. In essence, bigger companies exert more influence over the market's overall
						movement.
					</p>
					<p class="sp500-feature__text sp500-feature__text--highlight">
						<strong>Pro Tip:</strong> Investors aiming for exposure to large-cap stocks (such as
						Apple or Microsoft) might find it beneficial to invest in ETFs like
						<span class="ticker">VOO</span>
						or <span class="ticker">IVV</span> rather than picking individual stocks for their portfolio.
					</p>
					<p class="sp500-feature__text">
						To closely monitor market trends, <span class="ticker">$SPY</span> and
						<span class="ticker">$ES</span> (S&P 500 futures) are commonly used instruments. Having a
						chart of either SPY or SPY futures is crucial when trading stocks, as the majority of stocks
						tend to move in tandem with the broader market.
					</p>
					<div class="sp500-feature__etfs">
						<span class="sp500-feature__etfs-label">Popular S&P 500 ETFs:</span>
						<div class="sp500-feature__etfs-list">
							<a
								href="https://www.tradingview.com/symbols/AMEX-SPY/"
								target="_blank"
								rel="noopener noreferrer"
								class="etf-chip">SPY</a
							>
							<a
								href="https://www.tradingview.com/symbols/AMEX-VOO/"
								target="_blank"
								rel="noopener noreferrer"
								class="etf-chip">VOO</a
							>
							<a
								href="https://www.tradingview.com/symbols/AMEX-IVV/"
								target="_blank"
								rel="noopener noreferrer"
								class="etf-chip">IVV</a
							>
						</div>
					</div>
				</div>
				<div class="sp500-feature__chart">
					<div class="chart-placeholder">
						<IconChartLine size={64} />
						<span>SPY Chart</span>
						<a
							href="https://www.tradingview.com/symbols/AMEX-SPY/"
							target="_blank"
							rel="noopener noreferrer"
							class="chart-link"
						>
							View on TradingView
							<IconExternalLink size={16} />
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- All Indexes Section -->
	<section class="section section--dark">
		<div class="container">
			<div class="section__header">
				<h2 class="section__title">Major Stock Indexes</h2>
				<p class="section__description">
					Explore the world's most important stock market indexes and learn how to track them.
				</p>
			</div>

			<!-- Region Filter -->
			<div class="filter-bar">
				<label for="region-filter">
					<IconWorld size={20} />
				</label>
				<select id="region-filter" bind:value={selectedRegion} class="filter-select">
					{#each regions as region}
						<option value={region}>{region === 'all' ? 'All Regions' : region}</option>
					{/each}
				</select>
			</div>

			<!-- Index Cards Grid -->
			<div class="index-grid">
				{#each filteredIndexes as index, i (index.symbol)}
					<div class="index-card" in:fly={{ y: 20, delay: 50 * i, duration: 400 }}>
						<div class="index-card__header">
							<div class="index-card__symbol">{index.symbol}</div>
							<span class="index-card__region">{index.region}</span>
						</div>
						<h3 class="index-card__name">{index.name}</h3>
						<p class="index-card__ticker">Ticker: <span>{index.ticker}</span></p>
						<p class="index-card__description">{index.description}</p>
						<div class="index-card__meta">
							<div class="index-card__meta-item">
								<IconBuildingSkyscraper size={16} />
								<span>{index.companies > 0 ? `${index.companies} Companies` : 'N/A'}</span>
							</div>
							<div class="index-card__meta-item">
								<IconChartBar size={16} />
								<span>{index.type}</span>
							</div>
						</div>
						<div class="index-card__etfs">
							<span class="index-card__etfs-label">Related ETFs:</span>
							<div class="index-card__etfs-list">
								{#each index.etfs as etf}
									<a
										href="https://www.tradingview.com/symbols/AMEX-{etf}/"
										target="_blank"
										rel="noopener noreferrer"
										class="etf-chip etf-chip--small"
									>
										{etf}
									</a>
								{/each}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- FAQ Section -->
	<section class="section">
		<div class="container">
			<div class="section__header">
				<h2 class="section__title">Frequently Asked Questions</h2>
				<p class="section__description">
					Common questions about stock indexes and how to invest in them.
				</p>
			</div>

			<div class="faq-list">
				{#each faqItems as faq, i}
					<div class="faq-item" class:faq-item--expanded={expandedFaq === i}>
						<button class="faq-item__question" onclick={() => toggleFaq(i)}>
							<IconQuestionMark size={20} class="faq-item__icon" />
							<span>{faq.question}</span>
							<svg
								class="faq-item__chevron"
								width="20"
								height="20"
								viewBox="0 0 20 20"
								fill="currentColor"
							>
								<path
									fill-rule="evenodd"
									d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
									clip-rule="evenodd"
								/>
							</svg>
						</button>
						{#if expandedFaq === i}
							<div class="faq-item__answer" in:fly={{ y: -10, duration: 200 }}>
								{#each faq.answer.split('\n') as line}
									<p>{line}</p>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- CTA Section -->
	<section class="cta-section">
		<div class="container">
			<div class="cta-content">
				<h2 class="cta-title">Ready to Start Trading?</h2>
				<p class="cta-text">
					Join Revolution Trading Pros and get access to live trading rooms, expert alerts, and
					professional education.
				</p>
				<div class="cta-buttons">
					<a href="/live-trading-rooms" class="cta-btn cta-btn--primary">
						Get Started
						<IconArrowRight size={20} />
					</a>
					<a href="/resources/etf-stocks-list" class="cta-btn cta-btn--secondary">
						View ETF Stock List
					</a>
				</div>
			</div>
		</div>
	</section>
</div>

<style>
	.indexes-page {
		background: var(--rtp-bg, #0a0f1a);
		color: var(--rtp-text, #e5e7eb);
	}

	.container {
		max-width: 1280px;
		margin: 0 auto;
		padding: 0 1.5rem;
	}

	/* Hero */
	.hero {
		position: relative;
		padding: 10rem 1.5rem 6rem;
		overflow: hidden;
	}

	.hero__bg {
		position: absolute;
		inset: 0;
	}

	.hero__gradient {
		position: absolute;
		top: -50%;
		left: -20%;
		width: 800px;
		height: 800px;
		background: radial-gradient(circle, rgba(59, 130, 246, 0.15) 0%, transparent 70%);
		border-radius: 50%;
	}

	.hero__grid {
		position: absolute;
		inset: 0;
		background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%233b82f6' fill-opacity='0.03'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
		opacity: 0.5;
	}

	.hero__content {
		position: relative;
		max-width: 900px;
		margin: 0 auto;
		text-align: center;
	}

	.hero__badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.2);
		border-radius: 9999px;
		color: #3b82f6;
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 1.5rem;
	}

	.hero__title {
		font-size: clamp(2.5rem, 6vw, 4.5rem);
		font-weight: 800;
		background: linear-gradient(135deg, #fff 0%, #3b82f6 50%, #fff 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin-bottom: 1.5rem;
		line-height: 1.1;
	}

	.hero__subtitle {
		font-size: 1.25rem;
		color: #94a3b8;
		line-height: 1.7;
		max-width: 700px;
		margin: 0 auto;
	}

	/* Sections */
	.section {
		padding: 5rem 1.5rem;
	}

	.section--dark {
		background: rgba(59, 130, 246, 0.02);
		border-top: 1px solid rgba(59, 130, 246, 0.1);
		border-bottom: 1px solid rgba(59, 130, 246, 0.1);
	}

	.section__header {
		text-align: center;
		margin-bottom: 3rem;
	}

	.section__title {
		font-size: 2.5rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 1rem;
	}

	.section__description {
		font-size: 1.125rem;
		color: #94a3b8;
		max-width: 800px;
		margin: 0 auto;
		line-height: 1.7;
	}

	/* S&P 500 Feature */
	.sp500-feature {
		display: grid;
		grid-template-columns: 1fr 400px;
		gap: 3rem;
		align-items: start;
	}

	@media (max-width: 1024px) {
		.sp500-feature {
			grid-template-columns: 1fr;
		}
	}

	.sp500-feature__badge {
		display: inline-block;
		padding: 0.375rem 1rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.2);
		border-radius: 9999px;
		color: #3b82f6;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 1rem;
	}

	.sp500-feature__title {
		font-size: 2.5rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 1.5rem;
	}

	.sp500-feature__text {
		font-size: 1rem;
		color: #94a3b8;
		line-height: 1.8;
		margin-bottom: 1.25rem;
	}

	.sp500-feature__text--highlight {
		background: rgba(59, 130, 246, 0.05);
		border-left: 3px solid #3b82f6;
		padding: 1rem 1.25rem;
		border-radius: 0 8px 8px 0;
	}

	.ticker {
		font-family: 'Monaco', 'Menlo', monospace;
		background: rgba(59, 130, 246, 0.1);
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
		color: #3b82f6;
		font-weight: 600;
	}

	.sp500-feature__etfs {
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.sp500-feature__etfs-label {
		display: block;
		font-size: 0.875rem;
		color: #94a3b8;
		margin-bottom: 0.75rem;
	}

	.sp500-feature__etfs-list {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.etf-chip {
		display: inline-flex;
		align-items: center;
		padding: 0.5rem 1rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.2);
		border-radius: 8px;
		color: #3b82f6;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.etf-chip:hover {
		background: rgba(59, 130, 246, 0.2);
		border-color: rgba(59, 130, 246, 0.4);
	}

	.etf-chip--small {
		padding: 0.25rem 0.625rem;
		font-size: 0.75rem;
	}

	.sp500-feature__chart {
		position: sticky;
		top: 140px;
	}

	.chart-placeholder {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		padding: 3rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		color: #64748b;
		min-height: 300px;
	}

	.chart-placeholder span {
		font-weight: 600;
		color: #94a3b8;
	}

	.chart-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.2);
		border-radius: 9999px;
		color: #3b82f6;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.chart-link:hover {
		background: rgba(59, 130, 246, 0.2);
	}

	/* Filter Bar */
	.filter-bar {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 2rem;
		padding: 1rem 1.5rem;
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		color: #3b82f6;
	}

	.filter-select {
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		color: #e5e7eb;
		font-size: 0.9rem;
		cursor: pointer;
	}

	.filter-select:focus {
		outline: none;
		border-color: #3b82f6;
	}

	/* Index Grid */
	.index-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
		gap: 1.5rem;
	}

	.index-card {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		padding: 1.5rem;
		transition: all 0.3s ease;
	}

	.index-card:hover {
		border-color: rgba(59, 130, 246, 0.3);
		transform: translateY(-2px);
		box-shadow: 0 10px 40px rgba(59, 130, 246, 0.1);
	}

	.index-card__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.75rem;
	}

	.index-card__symbol {
		font-size: 1.5rem;
		font-weight: 800;
		color: #3b82f6;
	}

	.index-card__region {
		font-size: 0.75rem;
		padding: 0.25rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 9999px;
		color: #94a3b8;
	}

	.index-card__name {
		font-size: 1.125rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 0.25rem;
	}

	.index-card__ticker {
		font-size: 0.8rem;
		color: #64748b;
		margin-bottom: 0.75rem;
	}

	.index-card__ticker span {
		font-family: 'Monaco', 'Menlo', monospace;
		color: #3b82f6;
	}

	.index-card__description {
		font-size: 0.875rem;
		color: #94a3b8;
		line-height: 1.6;
		margin-bottom: 1rem;
	}

	.index-card__meta {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.index-card__meta-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	.index-card__etfs {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.index-card__etfs-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	.index-card__etfs-list {
		display: flex;
		gap: 0.375rem;
		flex-wrap: wrap;
	}

	/* FAQ */
	.faq-list {
		max-width: 800px;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.faq-item {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		overflow: hidden;
		transition: all 0.3s ease;
	}

	.faq-item--expanded {
		border-color: rgba(59, 130, 246, 0.3);
	}

	.faq-item__question {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		background: transparent;
		border: none;
		color: #fff;
		font-size: 1rem;
		font-weight: 600;
		text-align: left;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.faq-item__question:hover {
		background: rgba(255, 255, 255, 0.02);
	}

	.faq-item__question :global(.faq-item__icon) {
		color: #3b82f6;
		flex-shrink: 0;
	}

	.faq-item__question span {
		flex: 1;
	}

	.faq-item__chevron {
		color: #64748b;
		transition: transform 0.3s ease;
		flex-shrink: 0;
	}

	.faq-item--expanded .faq-item__chevron {
		transform: rotate(180deg);
	}

	.faq-item__answer {
		padding: 0 1.5rem 1.5rem 3.75rem;
	}

	.faq-item__answer p {
		font-size: 0.9rem;
		color: #94a3b8;
		line-height: 1.7;
		margin-bottom: 0.5rem;
	}

	.faq-item__answer p:last-child {
		margin-bottom: 0;
	}

	/* CTA Section */
	.cta-section {
		padding: 5rem 1.5rem;
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, transparent 100%);
	}

	.cta-content {
		max-width: 700px;
		margin: 0 auto;
		text-align: center;
	}

	.cta-title {
		font-size: 2.5rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 1rem;
	}

	.cta-text {
		font-size: 1.125rem;
		color: #94a3b8;
		margin-bottom: 2rem;
		line-height: 1.7;
	}

	.cta-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		justify-content: center;
	}

	.cta-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem 2rem;
		border-radius: 9999px;
		font-weight: 600;
		font-size: 1rem;
		text-decoration: none;
		transition: all 0.3s ease;
	}

	.cta-btn--primary {
		background: linear-gradient(135deg, #3b82f6, #2563eb);
		color: #fff;
		box-shadow: 0 4px 20px rgba(59, 130, 246, 0.3);
	}

	.cta-btn--primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 30px rgba(59, 130, 246, 0.4);
	}

	.cta-btn--secondary {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		color: #e5e7eb;
	}

	.cta-btn--secondary:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.2);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   2026 Mobile-First Responsive - Stock Indexes List
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Base Mobile Styles (xs: 360px) */
	.hero {
		padding: 5rem 1rem 3rem;
		padding-top: calc(5rem + env(safe-area-inset-top, 0px));
		padding-left: calc(1rem + env(safe-area-inset-left, 0px));
		padding-right: calc(1rem + env(safe-area-inset-right, 0px));
	}

	.hero__title {
		font-size: clamp(1.75rem, 8vw, 4.5rem);
	}

	.hero__subtitle {
		font-size: clamp(1rem, 3vw, 1.25rem);
	}

	.section {
		padding: 2.5rem 1rem;
		padding-left: calc(1rem + env(safe-area-inset-left, 0px));
		padding-right: calc(1rem + env(safe-area-inset-right, 0px));
	}

	.section__title {
		font-size: clamp(1.5rem, 5vw, 2.5rem);
	}

	.section__description {
		font-size: clamp(0.9375rem, 2.5vw, 1.125rem);
	}

	/* S&P 500 Feature - Mobile Stack */
	.sp500-feature {
		grid-template-columns: 1fr;
		gap: 2rem;
	}

	.sp500-feature__badge {
		font-size: 0.6875rem;
		padding: 0.3125rem 0.875rem;
	}

	.sp500-feature__title {
		font-size: clamp(1.5rem, 5vw, 2.5rem);
		margin-bottom: 1rem;
	}

	.sp500-feature__text {
		font-size: 0.9375rem;
		line-height: 1.7;
	}

	.sp500-feature__text--highlight {
		padding: 0.875rem 1rem;
	}

	.sp500-feature__etfs-list {
		gap: 0.375rem;
	}

	.etf-chip {
		/* 44px touch target */
		min-height: 44px;
		display: inline-flex;
		align-items: center;
		padding: 0.5rem 0.875rem;
	}

	.sp500-feature__chart {
		position: static;
	}

	.chart-placeholder {
		min-height: 200px;
		padding: 2rem;
	}

	.chart-link {
		/* 44px touch target */
		min-height: 44px;
		display: inline-flex;
		align-items: center;
	}

	/* Filter Bar - Mobile */
	.filter-bar {
		flex-direction: column;
		gap: 0.75rem;
		padding: 1rem;
	}

	.filter-select {
		width: 100%;
		/* 44px touch target */
		min-height: 44px;
	}

	/* Index Grid - Mobile Single Column */
	.index-grid {
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.index-card {
		padding: 1.25rem;
	}

	.index-card__symbol {
		font-size: 1.25rem;
	}

	.index-card__name {
		font-size: 1rem;
	}

	.index-card__ticker {
		font-size: 0.75rem;
	}

	.index-card__description {
		font-size: 0.8125rem;
	}

	.index-card__meta {
		flex-direction: column;
		gap: 0.5rem;
	}

	.index-card__etfs-list {
		gap: 0.375rem;
	}

	.etf-chip--small {
		/* 44px touch target */
		min-height: 44px;
		display: inline-flex;
		align-items: center;
		padding: 0.375rem 0.75rem;
	}

	/* FAQ Section - Mobile */
	.faq-list {
		gap: 0.75rem;
	}

	.faq-item__question {
		padding: 1rem;
		gap: 0.75rem;
		/* 44px touch target */
		min-height: 44px;
	}

	.faq-item__question span {
		font-size: 0.9375rem;
	}

	.faq-item__answer {
		padding: 0 1rem 1rem 2.75rem;
	}

	.faq-item__answer p {
		font-size: 0.8125rem;
	}

	/* CTA Section - Mobile */
	.cta-section {
		padding: 3rem 1rem;
		padding-bottom: calc(3rem + env(safe-area-inset-bottom, 0px));
	}

	.cta-title {
		font-size: clamp(1.5rem, 5vw, 2.5rem);
	}

	.cta-text {
		font-size: 1rem;
	}

	.cta-buttons {
		flex-direction: column;
		gap: 0.75rem;
	}

	.cta-btn {
		width: 100%;
		justify-content: center;
		/* 44px touch target */
		min-height: 44px;
		padding: 0.875rem 1.5rem;
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		.hero {
			padding: 6rem 1.5rem 4rem;
		}

		.section {
			padding: 3.5rem 1.5rem;
		}

		.filter-bar {
			flex-direction: row;
			padding: 1rem 1.5rem;
		}

		.filter-select {
			width: auto;
		}

		.index-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.25rem;
		}

		.index-card__meta {
			flex-direction: row;
			gap: 1rem;
		}

		.cta-buttons {
			flex-direction: row;
			justify-content: center;
		}

		.cta-btn {
			width: auto;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.hero {
			padding: 8rem 2rem 5rem;
		}

		.section {
			padding: 4rem 2rem;
		}

		.sp500-feature__text {
			font-size: 1rem;
		}

		.chart-placeholder {
			min-height: 300px;
			padding: 3rem;
		}

		.index-grid {
			gap: 1.5rem;
		}

		.index-card {
			padding: 1.5rem;
		}

		.faq-item__question {
			padding: 1.25rem 1.5rem;
		}

		.faq-item__answer {
			padding: 0 1.5rem 1.5rem 3.75rem;
		}

		.cta-section {
			padding: 4rem 2rem;
		}
	}

	/* lg: 1024px+ */
	@media (min-width: 1024px) {
		.hero {
			padding: 10rem 2rem 6rem;
		}

		.section {
			padding: 5rem 2rem;
		}

		.sp500-feature {
			grid-template-columns: 1fr 400px;
			gap: 3rem;
		}

		.sp500-feature__chart {
			position: sticky;
			top: 140px;
		}

		.index-grid {
			grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
		}

		.cta-section {
			padding: 5rem 2rem;
		}
	}

	/* xl: 1280px+ */
	@media (min-width: 1280px) {
		.index-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>
