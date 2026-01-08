/**
 * Trader Data Store
 * ═══════════════════════════════════════════════════════════════════════════
 * Centralized trader data with strong typing.
 * Single source of truth for all trader-related pages.
 * 
 * @version 1.0.0
 */

export interface Trader {
	id: string;
	name: string;
	slug: string;
	title: string;
	image: string;
	quote: string;
	whoIs: string;
	tradingApproach: string;
	specialties?: string[];
	socialLinks?: {
		twitter?: string;
		youtube?: string;
		instagram?: string;
	};
}

export interface TraderSubPage {
	label: string;
	path: string;
	description?: string;
}

export const traderSubPages: TraderSubPage[] = [
	{ label: 'Overview', path: '', description: 'Trader overview and bio' },
	{ label: 'Chart Setups', path: '/chart-setups', description: 'Trading chart configurations' },
	{ label: 'Trading Plan', path: '/trading-plan', description: 'Trading methodology and rules' },
	{ label: 'Trading Strategies', path: '/trading-strategies', description: 'Strategy breakdowns' },
	{ label: 'Trader Store', path: '/trader-store', description: 'Products and courses' }
];

export const traders: Trader[] = [
	{
		id: 'john-carter',
		name: 'John Carter',
		slug: 'john-carter',
		title: 'Founder of Simpler Trading®',
		image: 'https://cdn.simplertrading.com/2021/10/08113910/john-headshot-2020.png',
		quote: '"I think there are a lot of parallels between being a good trader and living a good life. The market truly is the ultimate psychologist."',
		whoIs: 'In 1999, tired of talking to his goldfish while trading alone in his office, John Carter launched the company that has evolved into Simpler Trading. He\'s the trading expert everyone turns to when the markets go awry (as they often do), and many of our traders have taken ideas and strategies from him and morphed them into their own strategies. He\'s truly one of the best with a level of charisma that draws people in, which has transformed him into the famous day trader he is today.',
		tradingApproach: 'As for his trading strategy, it combines expert technical analysis with an overall macro, fundamental view. However, what makes him a truly fantastic mentor on top of his vast knowledge, is his ability to make you feel as though you\'re talking to an old friend.',
		specialties: ['Options', 'Futures', 'Technical Analysis']
	},
	{
		id: 'henry-gambell',
		name: 'Henry Gambell',
		slug: 'henry-gambell',
		title: 'Director of Options',
		image: 'https://cdn.simplertrading.com/2021/10/08114059/henry-headshot-2020.png',
		quote: '"Every trade is an opportunity to learn something new about yourself and the markets."',
		whoIs: 'Henry Gambell is the Director of Options at Simpler Trading. He focuses on high-probability options setups using technical analysis and risk management principles. His methodical approach has helped countless traders improve their skills.',
		tradingApproach: 'Henry\'s trading style emphasizes patience and discipline, waiting for the right setups before entering trades. He is known for his ability to break down complex concepts for traders of all levels.',
		specialties: ['Options', 'Technical Analysis', 'Risk Management']
	},
	{
		id: 'taylor-horton',
		name: 'Taylor Horton',
		slug: 'taylor-horton',
		title: 'Senior Trader',
		image: 'https://cdn.simplertrading.com/2021/10/08114206/taylor-headshot-2020.png',
		quote: '"Success in trading comes from consistency and discipline, not from hitting home runs."',
		whoIs: 'Taylor Horton is a Senior Trader at Simpler Trading specializing in day trading and swing trading strategies. He focuses on momentum plays and breakout patterns that have proven successful over time.',
		tradingApproach: 'Taylor brings energy and enthusiasm to his trading sessions, helping members identify actionable setups in real-time with a focus on risk management.',
		specialties: ['Day Trading', 'Swing Trading', 'Momentum']
	},
	{
		id: 'bruce-marshall',
		name: 'Bruce Marshall',
		slug: 'bruce-marshall',
		title: 'Senior Trader',
		image: 'https://cdn.simplertrading.com/2021/10/08113753/bruce-headshot-2020.png',
		quote: '"Options give you leverage, but risk management keeps you in the game."',
		whoIs: 'Bruce Marshall is a Senior Trader with expertise in options strategies and portfolio management. He focuses on income-generating strategies and risk-defined trades that work in various market conditions.',
		tradingApproach: 'Bruce\'s approach emphasizes consistency and capital preservation, making him a favorite among traders looking for steady returns.',
		specialties: ['Options', 'Income Strategies', 'Portfolio Management']
	},
	{
		id: 'danielle-shay',
		name: 'Danielle Shay',
		slug: 'danielle-shay',
		title: 'Director of Options',
		image: 'https://cdn.simplertrading.com/2021/10/08113828/danielle-headshot-2020.png',
		quote: '"The best trades are the ones where you manage your risk before thinking about reward."',
		whoIs: 'Danielle Shay is the Director of Options at Simpler Trading. She specializes in momentum-based options strategies and volatility trading, helping traders navigate fast-moving markets.',
		tradingApproach: 'Danielle is known for her ability to identify high-probability setups in fast-moving markets. She focuses on helping traders understand options Greeks and position sizing.',
		specialties: ['Options', 'Momentum Trading', 'Volatility']
	},
	{
		id: 'allison-ostrander',
		name: 'Allison Ostrander',
		slug: 'allison-ostrander',
		title: 'Director of Risk Tolerance',
		image: 'https://cdn.simplertrading.com/2021/10/08113700/allison-headshot-2020.png',
		quote: '"Small accounts can grow into big accounts with the right mindset and strategy."',
		whoIs: 'Allison Ostrander is the Director of Risk Tolerance at Simpler Trading. She specializes in helping traders with smaller accounts grow their portfolios responsibly without taking excessive risks.',
		tradingApproach: 'Allison\'s approach emphasizes proper position sizing, risk management, and building a sustainable trading practice that can last for years.',
		specialties: ['Small Account Trading', 'Risk Management', 'Position Sizing']
	},
	{
		id: 'sam-shames',
		name: 'Sam Shames',
		slug: 'sam-shames',
		title: 'Senior Trader',
		image: 'https://cdn.simplertrading.com/2021/10/08114128/sam-headshot-2020.png',
		quote: '"The futures market rewards those who are prepared and punishes those who are not."',
		whoIs: 'Sam Shames is a Senior Trader specializing in futures and day trading. He focuses on ES and NQ futures with a momentum-based approach that has proven effective in various market conditions.',
		tradingApproach: 'Sam is known for his quick analysis and ability to adapt to changing market conditions, helping traders stay ahead of the curve.',
		specialties: ['Futures', 'Day Trading', 'ES/NQ']
	},
	{
		id: 'kody-ashmore',
		name: 'Kody Ashmore',
		slug: 'kody-ashmore',
		title: 'Senior Futures Trader',
		image: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
		quote: '"Every day in the futures market is a new opportunity to execute your edge."',
		whoIs: 'Kody Ashmore is a Senior Futures Trader at Simpler Trading. He specializes in ES and NQ futures with a focus on intraday momentum setups that capture quick, high-probability moves.',
		tradingApproach: 'Kody\'s trading style is aggressive yet disciplined, with strict risk management rules. He is known for his daily trading sessions where he shares live setups with members.',
		specialties: ['Futures', 'Intraday Trading', 'Momentum']
	},
	{
		id: 'raghee-horner',
		name: 'Raghee Horner',
		slug: 'raghee-horner',
		title: 'Senior Trader',
		image: 'https://cdn.simplertrading.com/2021/10/08114038/raghee-headshot-2020.png',
		quote: '"The trend is your friend, but only if you know how to find it."',
		whoIs: 'Raghee Horner is a Senior Trader with over 30 years of trading experience. She specializes in forex and futures trading with a focus on technical analysis that has stood the test of time.',
		tradingApproach: 'Raghee is known for her proprietary indicators and trading methodology. She has authored multiple books on trading and has helped thousands of traders improve their skills.',
		specialties: ['Forex', 'Futures', 'Technical Analysis', 'Proprietary Indicators']
	},
	{
		id: 'billy-ribeiro',
		name: 'Billy Ribeiro',
		slug: 'billy-ribeiro',
		title: 'Founder of Revolution Trading Pros',
		image: 'https://cdn.simplertrading.com/2021/10/08113910/john-headshot-2020.png',
		quote: '"Trading is not just about making money—it\'s about mastering yourself, understanding the markets, and building a sustainable edge that works in any condition."',
		whoIs: 'Billy Ribeiro is the Founder of Revolution Trading Pros. With a unique background combining software engineering and professional trading, Billy has built a reputation for creating innovative trading systems and educational content that helps traders at all levels improve their skills. His journey from developer to full-time trader has given him a distinctive perspective on market analysis and risk management.',
		tradingApproach: 'Billy\'s trading methodology blends technical analysis with systematic risk management. He emphasizes the importance of building repeatable processes, maintaining discipline, and continuously adapting to market conditions. His approach focuses on high-probability setups with clearly defined risk parameters, making complex strategies accessible to traders of all experience levels.',
		specialties: ['Options Trading', 'Technical Analysis', 'Trading Systems', 'Risk Management']
	}
];

/**
 * Get trader by slug
 */
export function getTraderBySlug(slug: string): Trader | undefined {
	return traders.find(t => t.slug === slug);
}

/**
 * Get trader by ID
 */
export function getTraderById(id: string): Trader | undefined {
	return traders.find(t => t.id === id);
}

/**
 * Get all trader slugs (useful for static generation)
 */
export function getAllTraderSlugs(): string[] {
	return traders.map(t => t.slug);
}

/**
 * Check if a trader exists
 */
export function traderExists(slug: string): boolean {
	return traders.some(t => t.slug === slug);
}
