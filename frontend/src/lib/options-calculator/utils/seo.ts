// ============================================================
// SEO — Structured Data & Meta Generation
// JSON-LD, Open Graph, FAQ schema for search engines.
// ============================================================

import type { BSInputs } from '../engine/types.js';

/**
 * Generate JSON-LD WebApplication structured data.
 * Place inside <script type="application/ld+json"> in <svelte:head>.
 */
export function generateStructuredData(): object {
	return {
		'@context': 'https://schema.org',
		'@type': 'WebApplication',
		name: 'Black-Scholes Options Calculator',
		description:
			'The most advanced free options pricing calculator. Real-time Greeks, interactive payoff diagrams, volatility analysis, multi-leg strategy builder, Monte Carlo simulation, and live market data.',
		url: 'https://revolutiontradingpros.com/tools/options-calculator',
		applicationCategory: 'FinanceApplication',
		operatingSystem: 'Web Browser',
		offers: {
			'@type': 'Offer',
			price: '0',
			priceCurrency: 'USD'
		},
		creator: {
			'@type': 'Organization',
			name: 'Revolution Trading Pros',
			url: 'https://revolutiontradingpros.com'
		},
		featureList: [
			'Black-Scholes-Merton option pricing',
			'Complete Greeks: Delta, Gamma, Theta, Vega, Rho, Charm, Vanna, Volga, Speed, Color, Zomma',
			'Interactive payoff diagrams',
			'3D P&L surface visualization',
			'Monte Carlo simulation',
			'Multi-leg strategy builder with 12 presets',
			'Volatility smile analysis',
			'Implied volatility solver (Newton-Raphson)',
			'Probability of profit calculator',
			'Real-time market data integration',
			'Options chain with mispricing detection',
			'Scenario analysis and stress testing',
			'Export to PNG, CSV',
			'Shareable calculator links',
			'Embeddable widget',
			'Dark and light themes',
			'Keyboard shortcuts and command palette'
		]
	};
}

/**
 * Generate Open Graph meta tag key-value pairs.
 * Optionally personalizes description with current inputs.
 */
export function generateOGMeta(inputs?: BSInputs): Record<string, string> {
	const description = inputs
		? `Analyze ${inputs.spotPrice > 0 ? `$${inputs.spotPrice.toFixed(0)}` : ''} options with real-time Greeks and interactive charts. Free professional-grade calculator by Revolution Trading Pros.`
		: 'The most advanced free Black-Scholes options calculator. Real-time Greeks, payoff diagrams, strategy builder, Monte Carlo simulation, and live market data.';

	return {
		'og:title': 'Black-Scholes Options Calculator | Revolution Trading Pros',
		'og:description': description,
		'og:type': 'website',
		'og:url': 'https://revolutiontradingpros.com/tools/options-calculator',
		'og:image': 'https://revolutiontradingpros.com/images/options-calculator-og.png',
		'og:site_name': 'Revolution Trading Pros',
		'twitter:card': 'summary_large_image',
		'twitter:title': 'Black-Scholes Options Calculator',
		'twitter:description': description,
		'twitter:image': 'https://revolutiontradingpros.com/images/options-calculator-og.png'
	};
}

/**
 * Generate FAQ structured data for rich search results.
 * Place inside <script type="application/ld+json"> in <svelte:head>.
 */
export function generateFAQStructuredData(): object {
	return {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity: [
			{
				'@type': 'Question',
				name: 'What is the Black-Scholes model?',
				acceptedAnswer: {
					'@type': 'Answer',
					text: 'The Black-Scholes-Merton model is a mathematical framework for pricing European-style options. Published in 1973, it calculates theoretical option prices based on five inputs: stock price, strike price, time to expiration, risk-free interest rate, and implied volatility.'
				}
			},
			{
				'@type': 'Question',
				name: 'What are the Greeks in options trading?',
				acceptedAnswer: {
					'@type': 'Answer',
					text: "The Greeks are risk measures that describe how an option's price changes relative to various factors. Delta measures sensitivity to stock price, Gamma measures delta's rate of change, Theta measures time decay, Vega measures sensitivity to volatility, and Rho measures sensitivity to interest rates."
				}
			},
			{
				'@type': 'Question',
				name: 'How accurate is the Black-Scholes calculator?',
				acceptedAnswer: {
					'@type': 'Answer',
					text: 'This calculator implements the exact Black-Scholes-Merton formula with continuous dividend yield adjustment. It calculates all first-order Greeks (Delta, Gamma, Theta, Vega, Rho) and second-order Greeks (Charm, Vanna, Volga, Speed, Color, Zomma) with full floating-point precision.'
				}
			},
			{
				'@type': 'Question',
				name: 'Is this options calculator free?',
				acceptedAnswer: {
					'@type': 'Answer',
					text: 'Yes, the Black-Scholes Options Calculator by Revolution Trading Pros is completely free to use. It includes real-time Greeks, payoff diagrams, strategy builder, Monte Carlo simulation, and more \u2014 no account required.'
				}
			}
		]
	};
}
