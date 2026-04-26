// ============================================================
// EDUCATIONAL CONTENT — Every Concept Explained
// Written for traders, not academics.
// Tone: authoritative, practical, actionable.
// ============================================================

import type { EducationEntry } from '../../engine/types.js';

export const GREEK_EDUCATION: Record<string, EducationEntry> = {
	delta: {
		id: 'delta',
		term: 'Delta',
		symbol: 'Δ',
		shortDescription: 'How much the option price moves per $1 move in the stock.',
		fullExplanation:
			'Delta measures the rate of change of the option price with respect to a $1 change in the underlying stock price. A delta of 0.50 means the option gains $0.50 for every $1 the stock moves up. Call deltas range from 0 to 1. Put deltas range from -1 to 0. Delta also roughly approximates the probability the option expires in-the-money.',
		proTip:
			'Use delta as a quick proxy for probability of profit. A 0.30 delta call has roughly a 30% chance of expiring ITM. Market makers hedge by trading delta × 100 shares per contract.',
		whenIncreases: 'Option becomes more sensitive to stock movement. Behaves more like stock.',
		whenDecreases: 'Option becomes less sensitive. More likely to expire worthless.',
		relatedTerms: ['gamma', 'charm']
	},
	gamma: {
		id: 'gamma',
		term: 'Gamma',
		symbol: 'Γ',
		shortDescription: 'How fast delta changes per $1 move in the stock.',
		fullExplanation:
			"Gamma is the rate of change of delta. It tells you how much your delta will shift for a $1 move in the stock. High gamma means your position's risk profile is changing rapidly. Gamma is highest for at-the-money options near expiration — this is why the last week before expiry is so volatile for option sellers.",
		proTip:
			'Gamma is the enemy of option sellers near expiration. A short ATM option with 3 DTE has extreme gamma risk — a $2 move can flip your position from profitable to deep underwater. This is why many premium sellers close at 21 DTE.',
		whenIncreases: 'Delta changes faster. Position is more volatile and harder to hedge.',
		whenDecreases: 'Delta is more stable. Position behaves more predictably.',
		relatedTerms: ['delta', 'speed', 'color']
	},
	theta: {
		id: 'theta',
		term: 'Theta',
		symbol: 'Θ',
		shortDescription: 'How much value the option loses per day from time decay.',
		fullExplanation:
			"Theta represents the daily time decay of an option. A theta of -$0.05 means the option loses $0.05 per day, all else equal. Theta accelerates as expiration approaches — the decay curve is not linear, it's exponential. Most theta decay happens in the final 30 days, with the steepest acceleration in the last 7-14 days.",
		proTip:
			'Theta is your best friend when selling options and your worst enemy when buying them. The 45 DTE sweet spot gives premium sellers good theta decay without excessive gamma risk. The "theta burn" really kicks in around 21 DTE.',
		whenIncreases: 'Option loses value faster each day. Sellers profit more from holding.',
		whenDecreases: 'Time decay slows. Far-dated options decay very slowly.',
		relatedTerms: ['charm', 'color']
	},
	vega: {
		id: 'vega',
		term: 'Vega',
		symbol: 'ν',
		shortDescription: 'How much the option price changes per 1% change in implied volatility.',
		fullExplanation:
			'Vega measures sensitivity to changes in implied volatility. A vega of $0.15 means the option gains $0.15 for every 1 percentage point increase in IV. Vega is highest for at-the-money, longer-dated options. This is why buying options before earnings (IV expansion) and selling after (IV crush) is a popular strategy.',
		proTip:
			'IV crush after earnings can destroy a long option position even if you got the direction right. If you buy a call before earnings and the stock moves up 3%, but IV drops 15 points, you can still lose money. Always check vega exposure before earnings.',
		whenIncreases:
			'Option is more sensitive to vol changes. Bigger swings from IV expansion/crush.',
		whenDecreases: 'Vol changes have less impact on option price.',
		relatedTerms: ['volga', 'vanna']
	},
	rho: {
		id: 'rho',
		term: 'Rho',
		symbol: 'ρ',
		shortDescription: 'How much the option price changes per 1% change in interest rates.',
		fullExplanation:
			'Rho measures sensitivity to interest rate changes. In most market conditions, rho has a minimal impact on short-dated options. However, for LEAPS (1-2 year options), rho can be significant. Higher rates increase call values and decrease put values because the present value of the strike price changes.',
		proTip:
			'Rho matters most for deep ITM LEAPS. In a rising rate environment, deep ITM calls get a small tailwind while deep ITM puts face a headwind. For most short-term trading, you can safely ignore rho.',
		whenIncreases: 'Calls become more expensive, puts become cheaper (higher rates favor calls).',
		whenDecreases: 'Calls become cheaper, puts become more expensive.',
		relatedTerms: ['delta', 'vega']
	},
	charm: {
		id: 'charm',
		term: 'Charm',
		symbol: '∂Δ/∂t',
		shortDescription: 'How fast delta decays over time (delta bleed).',
		fullExplanation:
			"Charm, also called delta decay, tells you how much your delta changes each day. This is critical for delta-hedging portfolios. An OTM call's delta decreases over time (charm is negative for OTM calls), meaning it becomes less likely to end up ITM as time passes.",
		proTip:
			'Charm explains why your OTM options feel like they "give up" as expiration approaches. Even without a stock move, your delta is bleeding away. This is the second-order manifestation of time decay that most retail traders overlook.',
		relatedTerms: ['delta', 'theta']
	},
	vanna: {
		id: 'vanna',
		term: 'Vanna',
		symbol: '∂Δ/∂σ',
		shortDescription: 'How delta changes when implied volatility changes.',
		fullExplanation:
			'Vanna measures the sensitivity of delta to changes in implied volatility. When IV rises, OTM options gain more delta (become more likely to be ITM from a probabilistic standpoint). Vanna is a key driver of "vol-driven rallies" — when market makers are short OTM puts and IV spikes, they must buy more stock to hedge increasing delta, creating a feedback loop.',
		proTip:
			'Vanna flows are behind many mysterious market moves. When dealers are short puts and IV spikes, they hedge by buying stock (positive vanna effect). Conversely, when IV collapses, dealers sell stock. Understanding vanna flows gives you edge in reading market structure.',
		relatedTerms: ['delta', 'vega', 'volga']
	},
	volga: {
		id: 'volga',
		term: 'Volga (Vomma)',
		symbol: '∂²V/∂σ²',
		shortDescription: 'How vega changes when implied volatility changes (vol of vol).',
		fullExplanation:
			'Volga, also called vomma, measures the convexity of vega — how much your vega exposure changes as IV moves. High volga means your position benefits disproportionately from large vol moves. OTM options have higher volga, which is why they get "bid up" during fear events — they have convex payoffs in volatility.',
		proTip:
			'Volga is why far OTM puts seem "expensive" — the market prices in volga premium because these options have convex vol exposure. When VIX spikes, the highest volga options see the most inflated premiums.',
		relatedTerms: ['vega', 'vanna']
	},
	speed: {
		id: 'speed',
		term: 'Speed',
		symbol: '∂Γ/∂S',
		shortDescription: 'How fast gamma changes as the stock moves.',
		fullExplanation:
			'Speed measures the rate of change of gamma with respect to the underlying price. It tells you how unstable your gamma exposure is. High speed means your risk profile is changing very rapidly as the stock moves.',
		proTip:
			"Speed matters most for large portfolios doing dynamic hedging. If you're managing a book of short options, speed tells you how quickly your gamma hedge needs to be adjusted as the market moves.",
		relatedTerms: ['gamma', 'color']
	},
	color: {
		id: 'color',
		term: 'Color',
		symbol: '∂Γ/∂t',
		shortDescription: 'How gamma changes over time (gamma decay rate).',
		fullExplanation:
			"Color, or gamma decay, shows how your gamma exposure changes as time passes. Near expiration, gamma for ATM options skyrockets — color quantifies this acceleration. Understanding color helps you anticipate when your position's risk profile will shift dramatically.",
		proTip:
			'Color screams "danger" in the final days before expiration. If you\'re short ATM options approaching expiry, color tells you that your gamma risk is about to explode. This is precisely why the "pin risk" window (final 3-5 days) is so dangerous.',
		relatedTerms: ['gamma', 'charm']
	},
	zomma: {
		id: 'zomma',
		term: 'Zomma',
		symbol: '∂Γ/∂σ',
		shortDescription: 'How gamma changes when implied volatility changes.',
		fullExplanation:
			'Zomma measures the sensitivity of gamma to changes in implied volatility. When IV increases, gamma for ATM options decreases slightly (the gamma peak "flattens" and spreads out), while OTM options gain gamma. Zomma helps explain how a vol spike changes the risk dynamics of your entire options book.',
		proTip:
			'Zomma is an advanced concept used primarily by market makers managing large books. For most traders, understanding that "higher IV flattens the gamma curve" is the practical takeaway.',
		relatedTerms: ['gamma', 'vega', 'volga']
	}
};

export const INPUT_EDUCATION: Record<string, EducationEntry> = {
	spotPrice: {
		id: 'spotPrice',
		term: 'Spot Price (S)',
		symbol: 'S',
		shortDescription: 'The current market price of the underlying stock or ETF.',
		fullExplanation:
			'The spot price is the current trading price of the underlying asset. This is the most important input to the Black-Scholes model. In live mode, this auto-populates from real-time market data.',
		proTip:
			'For options analysis, always use the mid-price of the current bid-ask spread, not the last traded price. This gives you the most accurate theoretical pricing.',
		relatedTerms: ['strikePrice']
	},
	strikePrice: {
		id: 'strikePrice',
		term: 'Strike Price (K)',
		symbol: 'K',
		shortDescription: 'The price at which the option can be exercised.',
		fullExplanation:
			'The strike price determines the price at which the option holder can buy (call) or sell (put) the underlying asset. The relationship between strike and spot determines moneyness: ITM (intrinsic value > 0), ATM (strike \u2248 spot), OTM (no intrinsic value).',
		proTip:
			'ATM options have the highest gamma and theta. OTM options are cheaper but have lower probability of profit. The strike you choose defines your risk/reward tradeoff.',
		relatedTerms: ['spotPrice']
	},
	volatility: {
		id: 'volatility',
		term: 'Implied Volatility (\u03c3)',
		symbol: '\u03c3',
		shortDescription:
			"The market's expectation of future price movement, expressed as annualized percentage.",
		fullExplanation:
			'Implied volatility is the market\'s forecast of the stock\'s annualized standard deviation. An IV of 30% means the market expects the stock to move within \u00b130% over the next year (one standard deviation). IV is the only input to Black-Scholes that is forward-looking \u2014 all others are known quantities. IV is "implied" from the market price of options using reverse engineering.',
		proTip:
			'IV Percentile and IV Rank tell you if current IV is high or low relative to its history. Sell premium when IV is high (>70th percentile), buy premium when IV is low (<30th percentile). This single concept separates profitable options traders from the rest.',
		whenIncreases: 'All options become more expensive. Sellers benefit when it drops (IV crush).',
		whenDecreases: 'All options become cheaper. Buyers get hurt (IV crush after earnings).',
		relatedTerms: ['vega', 'volga']
	},
	timeToExpiry: {
		id: 'timeToExpiry',
		term: 'Time to Expiry (T)',
		symbol: 'T',
		shortDescription: 'Time remaining until the option expires, measured in years.',
		fullExplanation:
			'Time to expiry is the remaining life of the option. More time = more expensive (more time for the stock to move). Time decay is not linear \u2014 it accelerates as expiration approaches. The "square root of time" rule means a 4x increase in time only doubles the option\'s time value.',
		proTip:
			'The 45 DTE sweet spot is popular for premium sellers: enough theta decay to profit, but far enough out to avoid gamma explosion. For buyers, consider 60-90 DTE to give your thesis time to play out without excessive theta burn.',
		whenIncreases: 'Options become more expensive. Theta decay per day is lower.',
		whenDecreases: 'Options lose extrinsic value. Theta per day accelerates sharply below 21 DTE.',
		relatedTerms: ['theta', 'charm', 'color']
	},
	riskFreeRate: {
		id: 'riskFreeRate',
		term: 'Risk-Free Rate (r)',
		symbol: 'r',
		shortDescription: 'The annual return on a risk-free investment, typically the Treasury yield.',
		fullExplanation:
			"The risk-free rate represents the theoretical return of an investment with zero risk, used as a benchmark in Black-Scholes. In practice, we use the Treasury yield that matches the option's time to expiry (e.g., 3-month T-bill for 3-month options). In live mode, this auto-populates from FRED Treasury data.",
		proTip:
			"The risk-free rate has minimal impact on short-dated options. For LEAPS (1-2 years), it matters more. In high-rate environments, calls are slightly more expensive and puts are slightly cheaper. Use the Treasury yield matching your option's expiration.",
		relatedTerms: ['rho']
	},
	dividendYield: {
		id: 'dividendYield',
		term: 'Dividend Yield (q)',
		symbol: 'q',
		shortDescription: 'Annual dividend payments as a percentage of the stock price.',
		fullExplanation:
			"The continuous dividend yield adjusts for expected dividends during the option's life. Dividends reduce call values and increase put values because the stock drops by the dividend amount on ex-date. For non-dividend stocks, set this to 0.",
		proTip:
			'Watch out for ex-dividend dates when trading short-dated calls. Early assignment risk increases for ITM calls just before ex-dividend. If the extrinsic value of your short call is less than the dividend, you may get assigned early.',
		whenIncreases: 'Calls become cheaper, puts become more expensive.',
		whenDecreases: 'Calls become more expensive, puts become cheaper.',
		relatedTerms: ['spotPrice']
	}
};

export const STRATEGY_EDUCATION: Record<string, EducationEntry> = {
	'long-call': {
		id: 'long-call',
		term: 'Long Call',
		shortDescription: 'Buy a call option \u2014 bullish bet with limited risk.',
		fullExplanation:
			"A long call gives you the right to buy shares at the strike price. Max risk is the premium paid. Max profit is unlimited. Break-even at expiry is strike + premium. Best used when you're bullish on direction AND expect IV to remain stable or increase.",
		proTip:
			"Don't buy calls just because you're bullish \u2014 check IV first. Buying calls when IV is elevated means you're overpaying. Consider a bull call spread instead to reduce vega exposure.",
		relatedTerms: ['long-put']
	},
	'long-put': {
		id: 'long-put',
		term: 'Long Put',
		shortDescription: 'Buy a put option \u2014 bearish bet or portfolio hedge.',
		fullExplanation:
			'A long put gives you the right to sell shares at the strike price. Max risk is the premium paid. Max profit is strike \u2212 premium (stock goes to zero). Break-even at expiry is strike \u2212 premium. Commonly used as portfolio insurance.',
		proTip:
			'Long puts are expensive insurance. Consider put spreads to reduce cost, or sell puts against existing short positions. Protective puts on a stock position create a "married put" with defined downside.',
		relatedTerms: ['long-call']
	},
	'covered-call': {
		id: 'covered-call',
		term: 'Covered Call',
		shortDescription: 'Own stock + sell a call \u2014 generate income, cap upside.',
		fullExplanation:
			'A covered call involves owning 100 shares and selling a call against them. You collect premium but cap your upside at the strike price. If the stock stays below the strike, you keep the premium and the shares. This is the most popular options strategy among retail investors.',
		proTip:
			'Sell covered calls at the 30-delta strike for a good balance of premium income vs. upside participation. Sell when IV is elevated. Avoid selling covered calls right before earnings unless you want to risk assignment.',
		relatedTerms: ['long-call']
	},
	'iron-condor': {
		id: 'iron-condor',
		term: 'Iron Condor',
		shortDescription:
			'Sell OTM call spread + OTM put spread \u2014 profit from range-bound movement.',
		fullExplanation:
			'An iron condor combines a bull put spread and a bear call spread. You collect premium from both sides and profit if the stock stays between your short strikes. Max profit is the net premium received. Max loss is the spread width minus premium. This is a defined-risk, neutral strategy.',
		proTip:
			'Place iron condors when IV is high (>70th percentile) and you expect the stock to stay range-bound. Set your short strikes at the expected move (1 SD) to get ~68% probability of profit. Manage at 50% of max profit or 21 DTE, whichever comes first.',
		relatedTerms: ['long-call', 'long-put']
	},
	straddle: {
		id: 'straddle',
		term: 'Straddle',
		shortDescription: 'Buy ATM call + ATM put \u2014 profit from a big move in either direction.',
		fullExplanation:
			'A long straddle involves buying both a call and a put at the same strike (usually ATM). You profit if the stock moves more than the combined premium in either direction. Max risk is the total premium paid. This is a pure volatility play.',
		proTip:
			'Straddles are expensive because you pay for two ATM options. The stock needs to move MORE than the expected move to profit. Buy straddles when you think the market is underpricing a catalyst. Sell straddles when IV is extremely elevated.',
		relatedTerms: ['iron-condor']
	},
	'bull-call-spread': {
		id: 'bull-call-spread',
		term: 'Bull Call Spread',
		shortDescription:
			'Buy a lower-strike call + sell a higher-strike call \u2014 bullish with capped risk.',
		fullExplanation:
			'A bull call spread (debit spread) involves buying a call at a lower strike and selling a call at a higher strike, both with the same expiration. Max profit is the spread width minus the debit paid. Max loss is the debit paid. This reduces cost vs. a naked long call but caps upside.',
		proTip:
			"Bull call spreads are ideal when you're moderately bullish and IV is high. The short call offsets vega, protecting you from IV crush. Choose spread width based on your target: narrow spreads for high probability, wide spreads for bigger payoff.",
		relatedTerms: ['long-call', 'iron-condor']
	}
};

export const CONCEPT_EDUCATION: Record<string, EducationEntry> = {
	'black-scholes': {
		id: 'black-scholes',
		term: 'Black-Scholes Model',
		shortDescription: 'The foundational mathematical model for pricing European-style options.',
		fullExplanation:
			'The Black-Scholes-Merton model, published in 1973, revolutionized finance by providing a closed-form solution for pricing European options. It assumes: constant volatility, log-normal price distribution, no dividends (original version), continuous trading, and no arbitrage. While real markets violate these assumptions, BSM remains the industry-standard benchmark for options pricing.',
		proTip:
			'Black-Scholes is a model, not reality. The market systematically prices options differently than BS predicts \u2014 this is why the volatility smile exists. Understanding WHERE and WHY BS diverges from market prices is where you find trading edge.',
		relatedTerms: ['put-call-parity', 'iv-crush']
	},
	'put-call-parity': {
		id: 'put-call-parity',
		term: 'Put-Call Parity',
		shortDescription:
			'The relationship that must hold between call and put prices to prevent arbitrage.',
		fullExplanation:
			'Put-call parity states: C \u2212 P = S\u00b7e^(\u2212qT) \u2212 K\u00b7e^(\u2212rT). If this relationship breaks, an arbitrage opportunity exists. In practice, slight deviations occur due to bid-ask spreads, transaction costs, and early exercise premium for American options.',
		proTip:
			'Check put-call parity as a sanity check. If the calculator shows a parity violation > $0.05, something is wrong with your inputs. In the real market, persistent parity violations signal data issues or extreme market stress.',
		relatedTerms: ['black-scholes']
	},
	'iv-crush': {
		id: 'iv-crush',
		term: 'IV Crush',
		shortDescription: 'The sharp drop in implied volatility after a known event (like earnings).',
		fullExplanation:
			'Before earnings announcements, options become expensive because of uncertainty. Implied volatility rises ("IV expansion"). After earnings are released, the uncertainty is resolved and IV drops sharply \u2014 often 20-50% overnight. This IV crush can devastate long option positions even if the stock moves in the predicted direction.',
		proTip:
			"The #1 mistake new options traders make: buying calls before earnings, being right on direction, and still losing money because of IV crush. If you're buying options into earnings, you need the stock to move MORE than the expected move to profit. Consider selling premium instead.",
		relatedTerms: ['vega', 'volatility']
	},
	'expected-move': {
		id: 'expected-move',
		term: 'Expected Move',
		shortDescription:
			"The market's predicted price range for a stock by expiration (1 standard deviation).",
		fullExplanation:
			"The expected move is calculated from the ATM straddle price and represents a 1-standard-deviation range. There's approximately a 68% probability the stock stays within this range by expiration. The formula: Expected Move = Stock Price \u00d7 IV \u00d7 \u221a(DTE/365).",
		proTip:
			'The expected move is your baseline for every options trade. Selling options inside the expected move gives you a statistical edge. The ATM straddle price roughly equals the expected move \u2014 this is the "free money" the market is pricing in for uncertainty.',
		relatedTerms: ['volatility']
	},
	moneyness: {
		id: 'moneyness',
		term: 'Moneyness',
		shortDescription:
			'Whether an option has intrinsic value (ITM), is at the strike (ATM), or has no intrinsic value (OTM).',
		fullExplanation:
			'Moneyness describes the relationship between the strike price and the current stock price. In-the-money (ITM): the option has intrinsic value. At-the-money (ATM): strike \u2248 stock price. Out-of-the-money (OTM): no intrinsic value, only extrinsic/time value. Deep ITM options behave like stock; deep OTM options are cheap lottery tickets.',
		proTip:
			'Most professional options traders focus on ATM and slightly OTM options. ATM has the highest gamma and theta. OTM has the highest percentage returns but lowest probability of profit. Choose moneyness based on your conviction level and risk tolerance.',
		relatedTerms: ['delta', 'spotPrice', 'strikePrice']
	}
};

/**
 * Look up an education entry by ID from any category.
 * Returns null if not found.
 */
export function getEducation(id: string): EducationEntry | null {
	return (
		GREEK_EDUCATION[id] ??
		INPUT_EDUCATION[id] ??
		STRATEGY_EDUCATION[id] ??
		CONCEPT_EDUCATION[id] ??
		null
	);
}

/**
 * Get all education entries as a flat array for search/filtering.
 */
export function getAllEducation(): EducationEntry[] {
	return [
		...Object.values(GREEK_EDUCATION),
		...Object.values(INPUT_EDUCATION),
		...Object.values(STRATEGY_EDUCATION),
		...Object.values(CONCEPT_EDUCATION)
	];
}
