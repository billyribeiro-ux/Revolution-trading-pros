export interface FaqItem {
	q: string;
	a: string;
}

export const faqData: FaqItem[] = [
	{
		q: 'How much capital do I need for swing trading?',
		a: 'We recommend a minimum of $2,000 to properly manage risk. Our position sizing model suggests risking only 1-2% of your account per trade. While the strategies work mathematically on smaller accounts, $2,000 allows you to take multiple positions simultaneously while maintaining a cash buffer.'
	},
	{
		q: 'Does this trigger the PDT (Pattern Day Trader) rule?',
		a: 'Generally, no. The PDT rule applies if you make 4 or more "day trades" (open and close same day) in a 5-day period on a margin account under $25k. Since our average hold time is 3-7 days, these are swing trades, not day trades. However, occasionally we may advise closing a trade same-day if it hits a target immediately or invalidates, but this is rare.'
	},
	{
		q: 'Do I need to sit at my computer all day?',
		a: 'Absolutely not. This service is specifically engineered for people with full-time jobs. We send alerts with clear entry zones. Because we trade multi-day moves, you typically have a generous window (often 30-60 minutes or more) to enter the trade after the alert is sent. No split-second scalping required.'
	},
	{
		q: 'What specific markets and instruments do you trade?',
		a: 'We focus on large-cap US equities (Magnificent 7: NVDA, TSLA, AAPL, etc.) and high-liquidity ETFs (SPY, QQQ, IWM). We primarily use Long Calls and Long Puts (buying options) to leverage these moves, but we also occasionally issue share-equity signals for those who prefer not to trade options.'
	},
	{
		q: 'How do I receive the alerts?',
		a: 'Redundancy is key. You will receive alerts via specific Discord channels (with push notifications), SMS text messages (US/Canada), and Email. We ensure you get the "ding" on your phone wherever you are.'
	},
	{
		q: 'What is your historical win rate and risk profile?',
		a: 'Our historical win rate hovers around 82% on valid setups. However, win rate matters less than Risk:Reward. We target a minimum 2:1 ratio, often seeking 3:1 or 4:1. This means one winner can pay for 2-3 small losers. We use strict "Hard Stops" on every trade to protect capital.'
	},
	{
		q: 'Is this suitable for beginners?',
		a: 'Yes, provided you understand the basics of how to buy and sell an option in your broker. Our alerts are "copy-paste" simple: Ticker, Strike Price, Expiration, and Price. We also provide a library of educational videos explaining our terminology and how to execute the orders.'
	},
	{
		q: 'What happens if I miss an entry price?',
		a: 'We always provide an "Entry Zone" (e.g., Enter NVDA Calls between $4.50 and $4.80). If price has already moved beyond this zone, we advise WAITING. Stocks often retrace. We strictly advise against "chasing" a trade that has left the station. There will always be another setup.'
	},
	{
		q: 'Do you trade through earnings reports?',
		a: 'Rarely. Holding directional options through earnings is considered gambling due to IV crush (Implied Volatility drop). We typically close positions or tighten stops significantly before a company reports earnings to preserve capital.'
	},
	{
		q: 'Can I use Robinhood, Webull, or ThinkOrSwim?',
		a: 'Yes. Our signals are platform-agnostic. As long as your broker allows you to trade Tier 2 Options (buying calls and puts), you can execute these trades. We recommend ThinkOrSwim or Interactive Brokers for the best execution, but mobile brokers work fine for swing trading.'
	},
	{
		q: 'What is the refund policy?',
		a: "We offer a 30-day money-back guarantee for your first month. If you feel the service isn't right for you, simply email support and we will refund your most recent payment. We want you to stay because you are profitable, not because you are locked in."
	},
	{
		q: 'Do you offer short selling signals?',
		a: 'We play both sides of the market. When the market is bearish, we buy PUT options to profit from the downside. We do not "short sell" shares (which requires margin and has infinite risk); we buy Puts (defined risk) instead.'
	}
];
