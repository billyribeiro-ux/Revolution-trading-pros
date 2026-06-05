<!--
	URL: /disclaimer
-->
<script lang="ts">
	const lastUpdated = 'June 1, 2026';
</script>

<svelte:head>
	<title>Risk Disclaimer | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Important risk disclaimer and disclosure information for Revolution Trading Pros members and visitors."
	/>
</svelte:head>

<div class="legal-page">
	<div class="legal-inner">
		<div class="legal-header">
			<p class="legal-header__label">Legal</p>
			<h1 class="legal-header__title">Risk Disclaimer</h1>
			<p class="legal-header__meta">Last updated: {lastUpdated}</p>
		</div>

		<div class="legal-body">
			<div class="risk-warning">
				<strong>Trading involves substantial risk of loss.</strong> Please read this entire disclaimer
				carefully before using any content, tools, or services provided by Revolution Trading Pros.
			</div>

			<h2>General Risk Warning</h2>
			<p>
				Trading in financial markets, including but not limited to stocks, options, futures, forex,
				and cryptocurrencies, carries a high level of risk and may not be suitable for all
				investors. The possibility exists that you could sustain a loss of some or all of your
				initial investment. Therefore, you should not invest money that you cannot afford to lose.
			</p>

			<h2>No Financial Advice</h2>
			<p>
				The content provided by Revolution Trading Pros — including but not limited to trade alerts,
				watchlists, educational materials, live trading sessions, videos, charts, and commentary —
				is for <strong>educational and informational purposes only</strong>. Nothing published by
				Revolution Trading Pros should be construed as financial advice, investment advice, trading
				advice, or any other type of advice.
			</p>
			<p>
				Revolution Trading Pros is not a registered investment advisor, broker-dealer, or financial
				planner. We do not recommend specific securities or trading strategies for any individual.
			</p>

			<h2>Past Performance</h2>
			<p>
				Past performance is not necessarily indicative of future results. Any performance figures,
				trade examples, or backtested results shown are hypothetical and for illustrative purposes
				only. Actual results may vary significantly.
			</p>

			<h2>Hypothetical Performance Disclaimer</h2>
			<p>
				Hypothetical or simulated performance results have certain inherent limitations. Unlike
				actual performance records, simulated results do not represent actual trading. Also, since
				the trades have not actually been executed, the results may have under- or over-compensated
				for the impact, if any, of certain market factors, such as lack of liquidity. Simulated
				trading programs in general are also subject to the fact that they are designed with the
				benefit of hindsight.
			</p>

			<h2>Do Your Own Research</h2>
			<p>
				You are solely responsible for your own investment decisions. We strongly encourage all
				users to conduct their own research, consult with licensed financial professionals, and
				carefully consider their financial situation before making any investment decisions.
			</p>

			<h2>Third-Party Links</h2>
			<p>
				Our platform may contain links to third-party websites or resources. These links are
				provided for convenience only. We have no control over the contents of those sites and
				accept no responsibility for them or for any loss or damage that may arise from your use of
				them.
			</p>

			<h2>Affiliate Disclosure</h2>
			<p>
				Revolution Trading Pros may have affiliate relationships with certain platforms, brokers, or
				services mentioned on this site. We may receive compensation for referrals. This does not
				influence our educational content or recommendations.
			</p>

			<h2>Contact</h2>
			<p>
				If you have questions about this disclaimer, please contact us at <a
					href="mailto:legal@revolutiontradingpros.com">legal@revolutiontradingpros.com</a
				>.
			</p>
		</div>
	</div>
</div>

<style>
	.legal-page {
		background: var(--rtp-bg);
		color: var(--rtp-text);
		min-height: 100vh;
		padding: 5rem 1.5rem;
	}
	.legal-inner {
		max-width: 780px;
		margin: 0 auto;
	}
	.legal-header {
		margin-bottom: 3rem;
		padding-bottom: 2rem;
		border-bottom: 1px solid var(--rtp-border-soft);
	}
	.legal-header__label {
		font-size: 0.8125rem;
		font-weight: 600;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--rtp-amber);
		margin: 0 0 0.75rem;
	}
	.legal-header__title {
		font-size: clamp(1.75rem, 4vw, 2.5rem);
		font-weight: 800;
		color: var(--rtp-text);
		margin: 0 0 0.75rem;
		line-height: 1.2;
	}
	.legal-header__meta {
		font-size: 0.875rem;
		color: var(--rtp-text-subtle);
		margin: 0;
	}

	.risk-warning {
		background: rgba(245, 158, 11, 0.08);
		border: 1px solid rgba(245, 158, 11, 0.25);
		border-radius: 0.75rem;
		padding: 1.25rem 1.5rem;
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
		line-height: 1.6;
		margin-bottom: 2.5rem;
	}
	.risk-warning strong {
		color: var(--rtp-amber);
	}

	.legal-body :global(h2) {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin: 2.5rem 0 0.75rem;
	}
	.legal-body :global(p) {
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
		line-height: 1.8;
		margin: 0 0 1rem;
	}
	.legal-body :global(strong) {
		color: var(--rtp-text);
	}
	.legal-body :global(a) {
		color: var(--rtp-indigo);
		text-decoration: underline;
		text-underline-offset: 3px;
	}
	.legal-body :global(a:hover) {
		color: #818cf8;
	}

	h2 {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin: 2.5rem 0 0.75rem;
	}
	p {
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
		line-height: 1.8;
		margin: 0 0 1rem;
	}
	strong {
		color: var(--rtp-text);
	}
	a {
		color: var(--rtp-indigo);
		text-decoration: underline;
		text-underline-offset: 3px;
	}
	a:hover {
		color: #818cf8;
	}
</style>
