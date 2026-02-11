/**
 * Indicators Page Data & Types
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple Principal Engineer ICT 7 Grade - February 2026
 *
 * Extracted from +page.svelte for maintainability.
 * Static data that doesn't need reactivity - imported as constants.
 */

import IconActivity from '@tabler/icons-svelte-runes/icons/activity';
import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
import IconWaveSine from '@tabler/icons-svelte-runes/icons/wave-sine';
import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
import IconChartCandle from '@tabler/icons-svelte-runes/icons/chart-candle';
import IconClock from '@tabler/icons-svelte-runes/icons/clock';
import IconTarget from '@tabler/icons-svelte-runes/icons/target';
import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
import type { Component } from 'svelte';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface Indicator {
	id: string;
	name: string;
	slug: string;
	category: string;
	description: string;
	useCase: string;
	difficulty: string;
	icon: Component;
	color: string;
	gradient: string;
	features: string[];
}

export interface FaqItem {
	question: string;
	answer: string;
}

export interface SetupItem {
	title: string;
	value: string;
	detail: string;
	icon: Component;
}

// ═══════════════════════════════════════════════════════════════════════════
// DATA
// ═══════════════════════════════════════════════════════════════════════════

export const indicators: Indicator[] = [
	{
		id: '1',
		name: 'RSI - Relative Strength Index',
		slug: 'rsi',
		category: 'Momentum',
		description:
			"Don't just spot overbought conditions-identify divergence. The RSI is our primary tool for spotting exhaustion before the price turns.",
		useCase: 'Spotting reversals and confirming trend weakness (Divergence)',
		difficulty: 'Beginner',
		icon: IconActivity,
		color: '#2e8eff',
		gradient: 'linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%)',
		features: ['Bullish/Bearish Divergence', 'Trend Reset Zones (50 line)', 'Momentum Confirmation']
	},
	{
		id: '2',
		name: 'MACD - Moving Average Convergence Divergence',
		slug: 'macd',
		category: 'Trend Following',
		description:
			'The grandfather of momentum indicators. We use it to filter out noise and stay on the right side of the dominant market trend.',
		useCase: 'Filtering false breakouts and confirming trend direction',
		difficulty: 'Intermediate',
		icon: IconWaveSine,
		color: '#34d399',
		gradient: 'linear-gradient(135deg, #34d399 0%, #059669 100%)',
		features: ['Histogram Momentum', 'Zero-Line Rejection', 'Multi-Timeframe Alignment']
	},
	{
		id: '3',
		name: 'Moving Averages (SMA/EMA)',
		slug: 'moving-averages',
		category: 'Trend Following',
		description:
			'The dynamic spine of the market. Learn why price respects the 9 EMA and 200 SMA, and how to use them as dynamic support.',
		useCase: 'Trailing stops and identifying dynamic support/resistance zones',
		difficulty: 'Beginner',
		icon: IconChartLine,
		color: '#a78bfa',
		gradient: 'linear-gradient(135deg, #a78bfa 0%, #7c3aed 100%)',
		features: ['Golden Cross / Death Cross', 'The "9 EMA Ride"', 'Mean Reversion']
	},
	{
		id: '4',
		name: 'Bollinger Bands',
		slug: 'bollinger-bands',
		category: 'Volatility',
		description:
			'The market breathes in (squeeze) and breathes out (expansion). Bollinger Bands help you catch the explosive moves after the calm.',
		useCase: 'Trading "The Squeeze" and measuring volatility extremes',
		difficulty: 'Intermediate',
		icon: IconChartBar,
		color: '#fb923c',
		gradient: 'linear-gradient(135deg, #fb923c 0%, #ea580c 100%)',
		features: ['Volatility Squeezes', 'Band Walking', 'Mean Reversion Targets']
	},
	{
		id: '5',
		name: 'VWAP - Volume Weighted Average Price',
		slug: 'vwap',
		category: 'Volume',
		description:
			'The only indicator institutions care about. If you are day trading without VWAP, you are trading blind against the big banks.',
		useCase: 'Institutional entry points and "Fair Value" assessment',
		difficulty: 'Beginner',
		icon: IconChartCandle,
		color: '#f59e0b',
		gradient: 'linear-gradient(135deg, #f59e0b 0%, #d97706 100%)',
		features: ['Institutional Support', 'Intraday Trend Filter', 'Fair Value Gap']
	},
	{
		id: '6',
		name: 'Stochastic Oscillator',
		slug: 'stochastic',
		category: 'Momentum',
		description:
			'Faster than RSI, the Stochastic helps you time precision entries in chopping markets or pullbacks within a trend.',
		useCase: 'Timing entries on pullbacks and range trading',
		difficulty: 'Intermediate',
		icon: IconActivity,
		color: '#ec4899',
		gradient: 'linear-gradient(135deg, #ec4899 0%, #be185d 100%)',
		features: ['Fast vs Slow Lines', 'Hidden Divergence', 'Crossover Signals']
	}
];

export const goldenSetup: SetupItem[] = [
	{
		title: 'Primary Trend',
		value: '1 Hour Chart',
		detail: 'Above 200 SMA = Bullish',
		icon: IconClock
	},
	{
		title: 'Entry Trigger',
		value: '5 Min Chart',
		detail: 'VWAP Reclaim + RSI > 50',
		icon: IconTarget
	},
	{
		title: 'Risk Management',
		value: '9 EMA Trailing',
		detail: 'Close below 9 EMA = Exit',
		icon: IconAlertTriangle
	},
	{ title: 'Momentum', value: 'RSI (14)', detail: 'Look for Divergence', icon: IconActivity }
];

export const faqs: FaqItem[] = [
	{
		question: 'Do I need all these indicators to be profitable?',
		answer:
			"Absolutely not. In fact, using too many causes 'analysis paralysis.' In our trading room, we teach you to master 2-3 core tools (usually VWAP, one momentum indicator, and moving averages) to build a clean, actionable chart."
	},
	{
		question: 'Are these indicators lagging?',
		answer:
			'Most indicators are lagging because they rely on past price data. However, we teach specific techniques - like divergence and multi-timeframe analysis - that turn these lagging tools into leading signals for future price action. Divergence often precedes price turns.'
	},
	{
		question: 'Which indicator is best for beginners?',
		answer:
			'We recommend starting with Moving Averages and VWAP. They provide an immediate visual guide to the trend and institutional value without complex calculations. They are the foundation of market structure.'
	},
	{
		question: 'What is the best RSI setting?',
		answer:
			'Stick to the default 14-period RSI. This is what the algorithms and institutional programs use. Changing it to 7 or 21 often results in curve-fitting. We want to see what the rest of the market sees.'
	},
	{
		question: 'Do you teach how to set these up?',
		answer:
			"Yes. When you join Revolution Trading Pros, you get our exact chart templates and settings. We don't just tell you what to use; we show you how to configure your platform (TradingView or TOS) to look exactly like ours."
	},
	{
		question: 'Can I use these for Crypto and Forex?',
		answer:
			'Yes. Technical analysis is universal. RSI divergence works on Bitcoin just as well as it works on Apple. However, VWAP is most effective in markets with centralized volume (Stocks/Futures).'
	}
];

export const categories = ['All', 'Momentum', 'Trend Following', 'Volatility', 'Volume'];

// ═══════════════════════════════════════════════════════════════════════════
// STRUCTURED DATA (SEO)
// ═══════════════════════════════════════════════════════════════════════════

export const indicatorsSchema = {
	'@context': 'https://schema.org',
	'@graph': [
		{
			'@type': 'ItemList',
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
					applicationCategory: 'FinanceApplication',
					operatingSystem: 'Web Browser'
				}
			}))
		},
		{
			'@type': 'FAQPage',
			mainEntity: faqs.map((faq) => ({
				'@type': 'Question',
				name: faq.question,
				acceptedAnswer: {
					'@type': 'Answer',
					text: faq.answer
				}
			}))
		}
	]
};
