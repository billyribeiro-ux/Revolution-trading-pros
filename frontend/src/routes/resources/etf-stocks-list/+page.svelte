<!--
	URL: /resources/etf-stocks-list
-->

<script lang="ts">
	/**
	 * ETF Stock List Page - Google L11 Enterprise Standard
	 * Comprehensive ETF education and reference resource
	 */
	import SEOHead from '$lib/components/SEOHead.svelte';
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import {
		IconChartLine,
		IconCoin,
		IconBuildingBank,
		IconHeartbeat,
		IconBolt,
		IconCpu,
		IconShoppingCart,
		IconHome,
		IconTools,
		IconLeaf,
		IconArrowRight,
		IconCheck,
		IconExternalLink
	} from '$lib/icons';

	// Featured ETFs with full descriptions
	const featuredETFs = [
		{
			symbol: 'SPY',
			name: 'SPDR S&P 500 ETF',
			description:
				'One of the oldest and most well-known ETFs, designed to track the performance of the S&P 500 index, which includes 500 of the largest U.S. companies.',
			features: ['High liquidity', 'Broad market exposure', 'Long track record']
		},
		{
			symbol: 'QQQ',
			name: 'Invesco QQQ Trust',
			description:
				'Tracks the performance of the Nasdaq-100 Index, which includes 100 of the largest non-financial companies listed on the Nasdaq Stock Market.',
			features: ['Technology focus', 'Innovation-driven companies', 'Strong historical performance']
		},
		{
			symbol: 'VTI',
			name: 'Vanguard Total Stock Market ETF',
			description:
				'Aims to track the performance of the CRSP US Total Market Index, providing exposure to the entire U.S. stock market, including small-, mid-, and large-cap stocks.',
			features: ['Comprehensive coverage', 'Low expense ratio', 'High liquidity']
		},
		{
			symbol: 'EEM',
			name: 'iShares MSCI Emerging Markets ETF',
			description:
				'Seeks to track the performance of the MSCI Emerging Markets Index, offering exposure to companies in emerging markets such as China, Brazil, and India.',
			features: [
				'High-growth potential',
				'Diversified emerging markets',
				'Geographic diversification'
			]
		},
		{
			symbol: 'IWM',
			name: 'iShares Russell 2000 ETF',
			description:
				'Tracks the performance of the Russell 2000 Index, providing exposure to small-cap U.S. companies.',
			features: ['Small-cap focus', 'Growth-oriented', 'Sector diversification']
		},
		{
			symbol: 'VEA',
			name: 'Vanguard FTSE Developed Markets ETF',
			description:
				'Aims to track the performance of the FTSE Developed All Cap ex US Index, offering exposure to developed markets outside of the U.S., including Europe, Japan, and Canada.',
			features: ['International exposure', 'Low expense ratio', 'Developed markets']
		},
		{
			symbol: 'AGG',
			name: 'iShares Core U.S. Aggregate Bond ETF',
			description:
				'Seeks to track the performance of the Bloomberg Barclays U.S. Aggregate Bond Index, providing broad exposure to U.S. investment-grade bonds.',
			features: ['Income generation', 'Fixed income diversification', 'Low expense ratio']
		},
		{
			symbol: 'VIG',
			name: 'Vanguard Dividend Appreciation ETF',
			description:
				'Focuses on U.S. companies that have a history of increasing dividends over time, tracking the performance of the NASDAQ US Dividend Achievers Select Index.',
			features: ['Dividend income', 'Financially stable companies', 'Low expense ratio']
		},
		{
			symbol: 'GLD',
			name: 'SPDR Gold Shares',
			description:
				'Aims to track the performance of the price of gold bullion, providing a simple and cost-effective way to invest in gold.',
			features: ['Inflation hedge', 'Physical gold backing', 'High liquidity']
		},
		{
			symbol: 'ARKK',
			name: 'ARK Innovation ETF',
			description:
				'Managed by ARK Invest, this actively managed ETF focuses on disruptive innovation across various sectors, including technology, healthcare, and industrials.',
			features: ['High growth potential', 'Cutting-edge technologies', 'Actively managed']
		},
		{
			symbol: 'EFA',
			name: 'iShares MSCI EAFE ETF',
			description:
				'Aims to track the performance of the MSCI EAFE Index, which includes large- and mid-cap stocks across developed markets outside of the U.S. and Canada.',
			features: ['International diversification', 'Established global companies', 'High liquidity']
		},
		{
			symbol: 'SCHB',
			name: 'Schwab U.S. Broad Market ETF',
			description:
				'Seeks to track the performance of the Dow Jones U.S. Broad Stock Market Index, providing exposure to the entire U.S. equity market.',
			features: ['Comprehensive exposure', 'Low expense ratio', 'High liquidity']
		}
	];

	// Sector ETFs
	const sectorETFs = [
		{
			sector: 'Technology',
			symbol: 'XLK',
			icon: IconCpu,
			color: 'from-blue-500 to-cyan-500',
			etfs: ['XLK', 'VGT', 'SMH', 'SOXX', 'PSI']
		},
		{
			sector: 'Healthcare',
			symbol: 'XLV',
			icon: IconHeartbeat,
			color: 'from-pink-500 to-rose-500',
			etfs: ['XLV', 'VHT', 'IHF', 'IBB', 'XBI', 'BBH']
		},
		{
			sector: 'Energy',
			symbol: 'XLE',
			icon: IconBolt,
			color: 'from-orange-500 to-amber-500',
			etfs: ['XLE', 'VDE', 'IYE', 'ICLN', 'TAN', 'PBW']
		},
		{
			sector: 'Financial',
			symbol: 'XLF',
			icon: IconBuildingBank,
			color: 'from-green-500 to-emerald-500',
			etfs: ['XLF', 'VFH', 'IYF', 'KRE', 'KBE']
		},
		{
			sector: 'Materials',
			symbol: 'XLB',
			icon: IconLeaf,
			color: 'from-teal-500 to-green-500',
			etfs: ['XLB', 'VAW', 'IYM', 'XME', 'PICK', 'GDX']
		},
		{
			sector: 'Utilities',
			symbol: 'XLU',
			icon: IconHome,
			color: 'from-yellow-500 to-orange-500',
			etfs: ['XLU', 'VPU', 'IDU']
		},
		{
			sector: 'Industrials',
			symbol: 'XLI',
			icon: IconTools,
			color: 'from-slate-500 to-gray-500',
			etfs: ['XLI', 'VIS', 'IYJ', 'ITA', 'XAR', 'PPA']
		},
		{
			sector: 'Consumer Staples',
			symbol: 'XLP',
			icon: IconShoppingCart,
			color: 'from-purple-500 to-violet-500',
			etfs: ['XLP', 'VDC', 'KXI']
		},
		{
			sector: 'Consumer Discretionary',
			symbol: 'XLY',
			icon: IconCoin,
			color: 'from-indigo-500 to-blue-500',
			etfs: ['XLY', 'VCR', 'IYC', 'ITB']
		}
	];

	// ETF Benefits
	const etfBenefits = [
		{
			title: 'Diverse Holdings',
			description:
				'ETFs often track an index, such as the S&P 500, meaning they include a broad selection of stocks or other securities from that index. This inherent diversity helps spread risk across various assets.'
		},
		{
			title: 'Tradable Like Stocks',
			description:
				'ETFs are bought and sold on stock exchanges throughout the trading day at market prices. This allows investors to trade ETFs just like individual stocks, with the flexibility to enter and exit positions as needed during market hours.'
		},
		{
			title: 'Lower Costs',
			description:
				'Compared to mutual funds, ETFs typically have lower expense ratios. This is because most ETFs are passively managed, meaning they aim to replicate the performance of an index rather than actively selecting securities.'
		},
		{
			title: 'Transparency',
			description:
				'ETFs disclose their holdings daily, providing investors with a clear and up-to-date view of the assets they are investing in. This level of transparency helps investors make informed decisions.'
		},
		{
			title: 'Tax Efficiency',
			description:
				'ETFs are generally more tax-efficient than mutual funds. Due to their unique structure, ETFs can minimize capital gains distributions, which can result in lower tax liabilities for investors.'
		}
	];

	// ETF Types
	const etfTypes = [
		{
			type: 'Stock ETFs',
			description:
				'Track a specific index of stocks, providing exposure to a wide range of companies within that index.'
		},
		{
			type: 'Bond ETFs',
			description:
				'Hold a collection of bonds, offering exposure to various types of debt securities including government and corporate bonds.'
		},
		{
			type: 'Commodity ETFs',
			description:
				'Invest in physical commodities such as gold, silver, oil, or agricultural products without having to physically purchase and store them.'
		},
		{
			type: 'Sector ETFs',
			description:
				'Focus on specific sectors or industries, such as technology, healthcare, or energy, allowing targeted investment.'
		},
		{
			type: 'International ETFs',
			description:
				"Provide exposure to markets outside of the investor's home country, helping to diversify geographically."
		}
	];

	// ETF vs Mutual Fund comparison
	const comparisonData = [
		{
			feature: 'Trading',
			etf: 'Traded throughout the day like stocks',
			mutualFund: 'Bought/sold at end of day at NAV price',
			etfBetter: true
		},
		{
			feature: 'Management',
			etf: 'Generally passive, tracking an index',
			mutualFund: 'Can be actively or passively managed',
			etfBetter: null
		},
		{
			feature: 'Expense Ratios',
			etf: 'Often lower due to passive nature',
			mutualFund: 'May be higher, especially if actively managed',
			etfBetter: true
		},
		{
			feature: 'Minimum Investment',
			etf: 'No minimum - buy as little as one share',
			mutualFund: 'Often have minimum requirements ($500-$3000+)',
			etfBetter: true
		},
		{
			feature: 'Tax Efficiency',
			etf: 'More tax-efficient structure',
			mutualFund: 'Less tax-efficient, more capital gains distributions',
			etfBetter: true
		},
		{
			feature: 'Flexibility',
			etf: 'Intraday trading, short selling, limit orders',
			mutualFund: 'End of day trading only',
			etfBetter: true
		},
		{
			feature: 'Best For',
			etf: 'Cost-conscious, active traders',
			mutualFund: 'Hands-off investors, professional management',
			etfBetter: null
		}
	];
</script>

<SEOHead
	title="ETF Stock List"
	description="Explore our complete ETF stock list with detailed information on ETFs and their underlying stocks. Perfect for traders and investors looking to diversify."
	canonical="/resources/etf-stocks-list"
	ogType="website"
	keywords={[
		'ETF list',
		'exchange traded funds',
		'SPY',
		'QQQ',
		'sector ETFs',
		'ETF investing',
		'stock market ETFs'
	]}
/>

<div class="etf-page">
	<!-- Hero Section -->
	<section class="hero">
		<div class="hero__bg">
			<div class="hero__gradient"></div>
			<div class="hero__grid"></div>
		</div>
		<div class="hero__content" in:fly={{ y: 30, duration: 600, easing: cubicOut }}>
			<div class="hero__badge">
				<IconChartLine size={20} />
				<span>Investment Resource</span>
			</div>
			<h1 class="hero__title">ETF Stock List</h1>
			<p class="hero__subtitle">
				Your comprehensive guide to Exchange-Traded Funds. Explore top ETFs, understand their
				benefits, and discover sector-specific opportunities for portfolio diversification.
			</p>
		</div>
	</section>

	<!-- What is an ETF Section -->
	<section class="section" in:fade={{ delay: 200, duration: 400 }}>
		<div class="container">
			<div class="section__header">
				<h2 class="section__title">What Is an ETF?</h2>
				<p class="section__description">
					Exchange-Traded Funds (ETFs) are investment vehicles that trade on stock exchanges, much
					like individual stocks. These funds are designed to track the performance of a specific
					index, sector, commodity, or asset class, thereby offering investors a way to gain
					exposure to a broad range of underlying assets through a single investment.
				</p>
			</div>

			<!-- Benefits Grid -->
			<div class="benefits-grid">
				{#each etfBenefits as benefit, i}
					<div class="benefit-card" in:fly={{ y: 20, delay: 100 * i, duration: 400 }}>
						<div class="benefit-card__icon">
							<IconCheck size={24} />
						</div>
						<h3 class="benefit-card__title">{benefit.title}</h3>
						<p class="benefit-card__text">{benefit.description}</p>
					</div>
				{/each}
			</div>

			<!-- ETF Types -->
			<div class="types-section">
				<h3 class="types-title">Types of ETFs</h3>
				<div class="types-grid">
					{#each etfTypes as type, i}
						<div class="type-card" in:fly={{ x: -20, delay: 100 * i, duration: 400 }}>
							<h4 class="type-card__title">{type.type}</h4>
							<p class="type-card__text">{type.description}</p>
						</div>
					{/each}
				</div>
			</div>
		</div>
	</section>

	<!-- Featured ETFs Section -->
	<section class="section section--dark">
		<div class="container">
			<div class="section__header">
				<h2 class="section__title">Popular ETFs</h2>
				<p class="section__description">
					Explore the most widely traded and recognized ETFs in the market, each offering unique
					exposure and benefits.
				</p>
			</div>

			<div class="etf-grid">
				{#each featuredETFs as etf, i}
					<div class="etf-card" in:fly={{ y: 20, delay: 50 * i, duration: 400 }}>
						<div class="etf-card__header">
							<span class="etf-card__symbol">{etf.symbol}</span>
							<a
								href="https://www.tradingview.com/symbols/AMEX-{etf.symbol}/"
								target="_blank"
								rel="noopener noreferrer"
								class="etf-card__chart-link"
								aria-label="View {etf.symbol} chart on TradingView"
							>
								<IconExternalLink size={16} />
							</a>
						</div>
						<h3 class="etf-card__name">{etf.name}</h3>
						<p class="etf-card__description">{etf.description}</p>
						<div class="etf-card__features">
							{#each etf.features as feature}
								<span class="etf-card__feature">{feature}</span>
							{/each}
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Sector ETFs Section -->
	<section class="section">
		<div class="container">
			<div class="section__header">
				<h2 class="section__title">ETFs By Sector</h2>
				<p class="section__description">
					Sector ETFs allow you to target specific segments of the economy. Click on any sector to
					explore available ETFs.
				</p>
			</div>

			<div class="sector-grid">
				{#each sectorETFs as sector, i}
					{@const Icon = sector.icon}
					<div class="sector-card" in:fly={{ y: 20, delay: 50 * i, duration: 400 }}>
						<div class="sector-card__icon bg-gradient-to-br {sector.color}">
							<Icon size={28} />
						</div>
						<h3 class="sector-card__title">{sector.sector}</h3>
						<div class="sector-card__etfs">
							{#each sector.etfs as etf}
								<a
									href="https://www.tradingview.com/symbols/AMEX-{etf}/"
									target="_blank"
									rel="noopener noreferrer"
									class="sector-card__etf"
								>
									{etf}
								</a>
							{/each}
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- ETFs vs Mutual Funds Section -->
	<section class="section section--dark">
		<div class="container">
			<div class="section__header">
				<h2 class="section__title">ETFs vs Mutual Funds</h2>
				<p class="section__description">
					Both ETFs and mutual funds are popular investment vehicles, but they have key differences
					that make them suitable for different types of investors.
				</p>
			</div>

			<div class="comparison-table-wrapper">
				<table class="comparison-table">
					<thead>
						<tr>
							<th>Feature</th>
							<th>ETFs</th>
							<th>Mutual Funds</th>
						</tr>
					</thead>
					<tbody>
						{#each comparisonData as row}
							<tr>
								<td class="comparison-table__feature">{row.feature}</td>
								<td
									class="comparison-table__cell"
									class:comparison-table__cell--better={row.etfBetter === true}
								>
									{#if row.etfBetter === true}
										<IconCheck
											size={16}
											class="comparison-table__icon comparison-table__icon--check"
										/>
									{/if}
									{row.etf}
								</td>
								<td
									class="comparison-table__cell"
									class:comparison-table__cell--better={row.etfBetter === false}
								>
									{#if row.etfBetter === false}
										<IconCheck
											size={16}
											class="comparison-table__icon comparison-table__icon--check"
										/>
									{/if}
									{row.mutualFund}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			<div class="comparison-summary">
				<div class="comparison-summary__item">
					<h4>ETFs are ideal for:</h4>
					<p>
						Cost-conscious investors who want to trade throughout the day and seek tax efficiency.
					</p>
				</div>
				<div class="comparison-summary__item">
					<h4>Mutual Funds are better for:</h4>
					<p>
						Those looking for professional management and who are comfortable with end-of-day
						trading.
					</p>
				</div>
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
					<a href="/resources/stock-indexes-list" class="cta-btn cta-btn--secondary">
						View Stock Indexes List
					</a>
				</div>
			</div>
		</div>
	</section>
</div>

<style>
	.etf-page {
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
		right: -20%;
		width: 800px;
		height: 800px;
		background: radial-gradient(circle, rgba(16, 185, 129, 0.15) 0%, transparent 70%);
		border-radius: 50%;
	}

	.hero__grid {
		position: absolute;
		inset: 0;
		background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%2310b981' fill-opacity='0.03'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
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
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 9999px;
		color: #10b981;
		font-size: 0.875rem;
		font-weight: 600;
		margin-bottom: 1.5rem;
	}

	.hero__title {
		font-size: clamp(2.5rem, 6vw, 4.5rem);
		font-weight: 800;
		background: linear-gradient(135deg, #fff 0%, #10b981 50%, #fff 100%);
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
		background: rgba(16, 185, 129, 0.02);
		border-top: 1px solid rgba(16, 185, 129, 0.1);
		border-bottom: 1px solid rgba(16, 185, 129, 0.1);
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

	/* Benefits Grid */
	.benefits-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
		margin-bottom: 4rem;
	}

	.benefit-card {
		background: rgba(16, 185, 129, 0.05);
		border: 1px solid rgba(16, 185, 129, 0.1);
		border-radius: 1rem;
		padding: 1.5rem;
		transition: all 0.3s ease;
	}

	.benefit-card:hover {
		border-color: rgba(16, 185, 129, 0.3);
		transform: translateY(-2px);
	}

	.benefit-card__icon {
		width: 48px;
		height: 48px;
		background: rgba(16, 185, 129, 0.1);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #10b981;
		margin-bottom: 1rem;
	}

	.benefit-card__title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 0.5rem;
	}

	.benefit-card__text {
		font-size: 0.9rem;
		color: #94a3b8;
		line-height: 1.6;
	}

	/* Types Section */
	.types-section {
		margin-top: 3rem;
	}

	.types-title {
		font-size: 1.5rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 1.5rem;
		text-align: center;
	}

	.types-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1rem;
	}

	.type-card {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 0.75rem;
		padding: 1.25rem;
		transition: all 0.3s ease;
	}

	.type-card:hover {
		border-color: rgba(16, 185, 129, 0.2);
	}

	.type-card__title {
		font-size: 1rem;
		font-weight: 600;
		color: #10b981;
		margin-bottom: 0.5rem;
	}

	.type-card__text {
		font-size: 0.875rem;
		color: #94a3b8;
		line-height: 1.5;
	}

	/* ETF Grid */
	.etf-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.etf-card {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		padding: 1.5rem;
		transition: all 0.3s ease;
	}

	.etf-card:hover {
		border-color: rgba(16, 185, 129, 0.3);
		transform: translateY(-2px);
		box-shadow: 0 10px 40px rgba(16, 185, 129, 0.1);
	}

	.etf-card__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.75rem;
	}

	.etf-card__symbol {
		font-size: 1.5rem;
		font-weight: 800;
		color: #10b981;
	}

	.etf-card__chart-link {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: rgba(16, 185, 129, 0.1);
		border-radius: 8px;
		color: #10b981;
		transition: all 0.2s ease;
	}

	.etf-card__chart-link:hover {
		background: rgba(16, 185, 129, 0.2);
	}

	.etf-card__name {
		font-size: 1rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 0.75rem;
	}

	.etf-card__description {
		font-size: 0.875rem;
		color: #94a3b8;
		line-height: 1.6;
		margin-bottom: 1rem;
	}

	.etf-card__features {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.etf-card__feature {
		font-size: 0.75rem;
		padding: 0.25rem 0.75rem;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 9999px;
		color: #10b981;
	}

	/* Sector Grid */
	.sector-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	.sector-card {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		padding: 1.5rem;
		transition: all 0.3s ease;
	}

	.sector-card:hover {
		border-color: rgba(255, 255, 255, 0.15);
		transform: translateY(-2px);
	}

	.sector-card__icon {
		width: 56px;
		height: 56px;
		border-radius: 14px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		margin-bottom: 1rem;
	}

	.sector-card__title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 1rem;
	}

	.sector-card__etfs {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.sector-card__etf {
		font-size: 0.8rem;
		font-weight: 600;
		padding: 0.375rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		color: #e5e7eb;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.sector-card__etf:hover {
		background: rgba(16, 185, 129, 0.1);
		border-color: rgba(16, 185, 129, 0.3);
		color: #10b981;
	}

	/* Comparison Table */
	.comparison-table-wrapper {
		overflow-x: auto;
		margin-bottom: 2rem;
	}

	.comparison-table {
		width: 100%;
		border-collapse: collapse;
		background: rgba(255, 255, 255, 0.02);
		border-radius: 1rem;
		overflow: hidden;
	}

	.comparison-table th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-weight: 600;
		color: #fff;
		background: rgba(16, 185, 129, 0.1);
		border-bottom: 1px solid rgba(16, 185, 129, 0.2);
	}

	.comparison-table td {
		padding: 1rem 1.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
		color: #94a3b8;
		font-size: 0.9rem;
	}

	.comparison-table__feature {
		font-weight: 600;
		color: #fff;
	}

	.comparison-table__cell--better {
		color: #10b981;
	}

	.comparison-table__icon {
		display: inline-block;
		margin-right: 0.5rem;
		vertical-align: middle;
	}

	.comparison-table__icon--check {
		color: #10b981;
	}

	/* Comparison Summary */
	.comparison-summary {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	.comparison-summary__item {
		background: rgba(16, 185, 129, 0.05);
		border: 1px solid rgba(16, 185, 129, 0.1);
		border-radius: 1rem;
		padding: 1.5rem;
	}

	.comparison-summary__item h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #10b981;
		margin-bottom: 0.5rem;
	}

	.comparison-summary__item p {
		font-size: 0.9rem;
		color: #94a3b8;
		line-height: 1.6;
	}

	/* CTA Section */
	.cta-section {
		padding: 5rem 1.5rem;
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, transparent 100%);
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
		background: linear-gradient(135deg, #10b981, #059669);
		color: #fff;
		box-shadow: 0 4px 20px rgba(16, 185, 129, 0.3);
	}

	.cta-btn--primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 30px rgba(16, 185, 129, 0.4);
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
	   2026 Mobile-First Responsive - ETF Stocks List
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

	/* Benefits Grid - Mobile Single Column */
	.benefits-grid {
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.benefit-card {
		padding: 1.25rem;
	}

	.benefit-card__icon {
		width: 40px;
		height: 40px;
	}

	.benefit-card__title {
		font-size: 1rem;
	}

	.benefit-card__text {
		font-size: 0.875rem;
	}

	/* Types Grid - Mobile Single Column */
	.types-grid {
		grid-template-columns: 1fr;
		gap: 0.75rem;
	}

	.type-card {
		padding: 1rem;
	}

	/* ETF Grid - Mobile Single Column */
	.etf-grid {
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.etf-card {
		padding: 1.25rem;
	}

	.etf-card__symbol {
		font-size: 1.25rem;
	}

	.etf-card__chart-link {
		/* 44px touch target */
		width: 44px;
		height: 44px;
	}

	.etf-card__name {
		font-size: 0.9375rem;
	}

	.etf-card__description {
		font-size: 0.8125rem;
	}

	.etf-card__feature {
		font-size: 0.6875rem;
		padding: 0.25rem 0.625rem;
	}

	/* Sector Grid - Mobile Single Column */
	.sector-grid {
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.sector-card {
		padding: 1.25rem;
	}

	.sector-card__icon {
		width: 48px;
		height: 48px;
	}

	.sector-card__etf {
		/* 44px touch target */
		min-height: 44px;
		display: inline-flex;
		align-items: center;
		padding: 0.5rem 0.875rem;
	}

	/* Comparison Table - Mobile Horizontal Scroll */
	.comparison-table-wrapper {
		margin: 0 -1rem;
		padding: 0 1rem;
	}

	.comparison-table {
		min-width: 600px;
	}

	.comparison-table th,
	.comparison-table td {
		padding: 0.625rem 0.75rem;
		font-size: 0.75rem;
	}

	/* Comparison Summary - Mobile Stack */
	.comparison-summary {
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.comparison-summary__item {
		padding: 1.25rem;
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

		.benefits-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.25rem;
		}

		.types-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1rem;
		}

		.etf-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.25rem;
		}

		.sector-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.25rem;
		}

		.comparison-summary {
			grid-template-columns: repeat(2, 1fr);
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

		.benefits-grid {
			grid-template-columns: repeat(3, 1fr);
		}

		.etf-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.5rem;
		}

		.etf-card {
			padding: 1.5rem;
		}

		.sector-grid {
			grid-template-columns: repeat(3, 1fr);
		}

		.comparison-table-wrapper {
			margin: 0;
			padding: 0;
		}

		.comparison-table th,
		.comparison-table td {
			padding: 1rem 1.5rem;
			font-size: 0.9rem;
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

		.benefits-grid {
			grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		}

		.types-grid {
			grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		}

		.etf-grid {
			grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		}

		.sector-grid {
			grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		}

		.cta-section {
			padding: 5rem 2rem;
		}
	}

	/* xl: 1280px+ */
	@media (min-width: 1280px) {
		.etf-grid {
			grid-template-columns: repeat(3, 1fr);
		}

		.sector-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>
