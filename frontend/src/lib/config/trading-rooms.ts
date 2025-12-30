/**
 * Trading Room Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Single source of truth for all trading room and alert service configurations
 * Used by dynamic [slug] routes to render room-specific content
 *
 * @version 1.0.0
 */

export interface TradingRoomConfig {
	slug: string;
	name: string;
	title: string; // Full title for dashboard header
	type: 'trading-room' | 'alert-service';
	description: string;
	calendarId?: string; // Google Calendar ID for schedule
	calendarApiKey?: string;
	calendarClientId?: string;
	tutorialVideo?: {
		src: string;
		poster: string;
	};
	watchlistImage?: string;
	quickLinks: Array<{ label: string; href: string; external?: boolean }>;
	traders: Array<{
		id: number;
		name: string;
		nickname: string;
		title: string;
		bio: string;
		image: string;
		specialties: string[];
	}>;
	// Feature flags for what pages are available
	features: {
		dailyVideos: boolean;
		learningCenter: boolean;
		tradingRoomArchive: boolean;
		traders: boolean;
		startHere: boolean;
		resources: boolean;
		traderStore: boolean;
	};
}

// Trading Room Configurations
export const tradingRooms: Record<string, TradingRoomConfig> = {
	'day-trading-room': {
		slug: 'day-trading-room',
		name: 'Day Trading Room',
		title: 'Day Trading Room Dashboard',
		type: 'trading-room',
		description: 'Master the art of day trading with expert guidance and real-time market analysis.',
		calendarId: 'simpleroptions.com_sabio00har0rd4odbrsa705904@group.calendar.google.com',
		calendarApiKey: 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw',
		calendarClientId: '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com',
		tutorialVideo: {
			src: 'https://simpler-options.s3.amazonaws.com/tutorials/MTT_tutorial2025.mp4',
			poster: 'https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg'
		},
		watchlistImage: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
		quickLinks: [
			{ label: 'Support', href: 'https://intercom.help/simpler-trading/en/', external: true },
			{ label: 'Platform Tutorials', href: '/tutorials', external: true },
			{ label: 'Trading Blog', href: '/blog', external: true }
		],
		traders: [
			{
				id: 1,
				name: 'Billy Ribeiro',
				nickname: 'Billy',
				title: 'Lead Day Trader',
				bio: 'Billy brings a disciplined approach to day trading with a focus on high-probability setups and risk management. His systematic methodology helps traders develop consistent profitability.',
				image: '/images/traders/billy-ribeiro.jpg',
				specialties: ['Day Trading', 'Risk Management', 'Technical Analysis']
			},
			{
				id: 2,
				name: 'Freddie Ferber',
				nickname: 'Freddie',
				title: 'Senior Options Trader',
				bio: 'Freddie specializes in options strategies and momentum trading. His expertise in reading market dynamics and identifying high-probability trades has helped countless traders succeed.',
				image: '/images/traders/freddie-ferber.jpg',
				specialties: ['Options Strategies', 'Momentum Trading', 'Swing Trading']
			},
			{
				id: 3,
				name: 'Shao Wan',
				nickname: 'Shao',
				title: 'Quantitative Trading Expert',
				bio: 'Shao combines technical analysis with quantitative methods to identify market opportunities. His data-driven approach provides unique insights into market behavior and trading patterns.',
				image: '/images/traders/shao-wan.jpg',
				specialties: ['Quantitative Analysis', 'Price Action', 'Futures Trading']
			}
		],
		features: {
			dailyVideos: true,
			learningCenter: true,
			tradingRoomArchive: true,
			traders: true,
			startHere: true,
			resources: true,
			traderStore: true
		}
	},
	'swing-trading-room': {
		slug: 'swing-trading-room',
		name: 'Swing Trading Room',
		title: 'Swing Trading Room Dashboard',
		type: 'trading-room',
		description: 'Capture multi-day moves with our swing trading strategies and expert analysis.',
		calendarId: 'simpleroptions.com_swing_trading@group.calendar.google.com',
		calendarApiKey: 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw',
		calendarClientId: '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com',
		tutorialVideo: {
			src: 'https://simpler-options.s3.amazonaws.com/tutorials/SwingTrading_tutorial2025.mp4',
			poster: 'https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg'
		},
		watchlistImage: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Swing-Watchlist.jpg',
		quickLinks: [
			{ label: 'Support', href: 'https://intercom.help/simpler-trading/en/', external: true },
			{ label: 'Platform Tutorials', href: '/tutorials', external: true },
			{ label: 'Trading Blog', href: '/blog', external: true }
		],
		traders: [
			{
				id: 1,
				name: 'Billy Ribeiro',
				nickname: 'Billy',
				title: 'Lead Swing Trader',
				bio: 'Billy brings a disciplined approach to swing trading with a focus on multi-day setups and position management.',
				image: '/images/traders/billy-ribeiro.jpg',
				specialties: ['Swing Trading', 'Position Sizing', 'Technical Analysis']
			},
			{
				id: 2,
				name: 'Freddie Ferber',
				nickname: 'Freddie',
				title: 'Senior Options Trader',
				bio: 'Freddie specializes in swing options strategies that capture multi-day momentum moves.',
				image: '/images/traders/freddie-ferber.jpg',
				specialties: ['Options Strategies', 'Momentum Trading', 'Swing Trading']
			}
		],
		features: {
			dailyVideos: true,
			learningCenter: true,
			tradingRoomArchive: true,
			traders: true,
			startHere: true,
			resources: true,
			traderStore: false
		}
	},
	'small-account-mentorship': {
		slug: 'small-account-mentorship',
		name: 'Small Account Mentorship',
		title: 'Small Account Mentorship Dashboard',
		type: 'trading-room',
		description: 'Grow your small account with proven strategies designed for capital preservation and steady growth.',
		calendarId: 'simpleroptions.com_small_account@group.calendar.google.com',
		calendarApiKey: 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw',
		calendarClientId: '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com',
		tutorialVideo: {
			src: 'https://simpler-options.s3.amazonaws.com/tutorials/SmallAccount_tutorial2025.mp4',
			poster: 'https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg'
		},
		quickLinks: [
			{ label: 'Support', href: 'https://intercom.help/simpler-trading/en/', external: true },
			{ label: 'Account Setup Guide', href: '/guides/small-account', external: true },
			{ label: 'Trading Blog', href: '/blog', external: true }
		],
		traders: [
			{
				id: 1,
				name: 'Billy Ribeiro',
				nickname: 'Billy',
				title: 'Small Account Specialist',
				bio: 'Billy has helped hundreds of traders grow their small accounts with disciplined risk management and smart position sizing.',
				image: '/images/traders/billy-ribeiro.jpg',
				specialties: ['Small Account Growth', 'Risk Management', 'Position Sizing']
			},
			{
				id: 3,
				name: 'Shao Wan',
				nickname: 'Shao',
				title: 'Quantitative Trading Expert',
				bio: 'Shao applies quantitative methods to maximize small account returns while minimizing drawdowns.',
				image: '/images/traders/shao-wan.jpg',
				specialties: ['Quantitative Analysis', 'Risk Management', 'Capital Preservation']
			}
		],
		features: {
			dailyVideos: true,
			learningCenter: true,
			tradingRoomArchive: true,
			traders: true,
			startHere: true,
			resources: true,
			traderStore: false
		}
	},
	'spx-profit-pulse': {
		slug: 'spx-profit-pulse',
		name: 'SPX Profit Pulse',
		title: 'SPX Profit Pulse Dashboard',
		type: 'alert-service',
		description: 'Real-time SPX trading alerts with precise entry and exit signals.',
		quickLinks: [
			{ label: 'Support', href: 'https://intercom.help/simpler-trading/en/', external: true },
			{ label: 'Alert Settings', href: '/account', external: false }
		],
		traders: [
			{
				id: 1,
				name: 'Billy Ribeiro',
				nickname: 'Billy',
				title: 'SPX Trading Specialist',
				bio: 'Billy delivers precise SPX trading alerts based on advanced technical analysis and market internals.',
				image: '/images/traders/billy-ribeiro.jpg',
				specialties: ['SPX Trading', 'Index Options', 'Technical Analysis']
			}
		],
		features: {
			dailyVideos: true,
			learningCenter: false,
			tradingRoomArchive: false,
			traders: true,
			startHere: true,
			resources: true,
			traderStore: false
		}
	},
	'explosive-swing': {
		slug: 'explosive-swing',
		name: 'Explosive Swing',
		title: 'Explosive Swing Dashboard',
		type: 'alert-service',
		description: 'High-probability swing trade alerts for explosive moves in stocks and options.',
		quickLinks: [
			{ label: 'Support', href: 'https://intercom.help/simpler-trading/en/', external: true },
			{ label: 'Alert Settings', href: '/account', external: false }
		],
		traders: [
			{
				id: 2,
				name: 'Freddie Ferber',
				nickname: 'Freddie',
				title: 'Explosive Moves Specialist',
				bio: 'Freddie identifies high-probability explosive swing trade setups with potential for significant gains.',
				image: '/images/traders/freddie-ferber.jpg',
				specialties: ['Swing Trading', 'Momentum Plays', 'Breakout Trading']
			}
		],
		features: {
			dailyVideos: true,
			learningCenter: false,
			tradingRoomArchive: false,
			traders: true,
			startHere: true,
			resources: true,
			traderStore: false
		}
	},
	'options-day-trading': {
		slug: 'options-day-trading',
		name: 'Options Day Trading',
		title: 'Options Day Trading Dashboard',
		type: 'trading-room',
		description: 'Master the art of day trading options with expert guidance.',
		calendarId: 'simpleroptions.com_options_day@group.calendar.google.com',
		calendarApiKey: 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw',
		calendarClientId: '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com',
		tutorialVideo: {
			src: 'https://simpler-options.s3.amazonaws.com/tutorials/OptionsDayTrading_tutorial2025.mp4',
			poster: 'https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg'
		},
		watchlistImage: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Options-Watchlist.jpg',
		quickLinks: [
			{ label: 'Support', href: 'https://intercom.help/simpler-trading/en/', external: true },
			{ label: 'Platform Tutorials', href: '/tutorials', external: true },
			{ label: 'Trading Blog', href: '/blog', external: true }
		],
		traders: [
			{
				id: 1,
				name: 'Billy Ribeiro',
				nickname: 'Billy',
				title: 'Lead Day Trader',
				bio: 'Billy brings a disciplined approach to day trading with a focus on high-probability setups and risk management.',
				image: '/images/traders/billy-ribeiro.jpg',
				specialties: ['Day Trading', 'Risk Management', 'Technical Analysis']
			},
			{
				id: 2,
				name: 'Freddie Ferber',
				nickname: 'Freddie',
				title: 'Senior Options Trader',
				bio: 'Freddie specializes in options strategies and momentum trading.',
				image: '/images/traders/freddie-ferber.jpg',
				specialties: ['Options Strategies', 'Momentum Trading', 'Swing Trading']
			},
			{
				id: 3,
				name: 'Shao Wan',
				nickname: 'Shao',
				title: 'Quantitative Trading Expert',
				bio: 'Shao combines technical analysis with quantitative methods to identify market opportunities.',
				image: '/images/traders/shao-wan.jpg',
				specialties: ['Quantitative Analysis', 'Price Action', 'Futures Trading']
			}
		],
		features: {
			dailyVideos: true,
			learningCenter: true,
			tradingRoomArchive: true,
			traders: true,
			startHere: true,
			resources: true,
			traderStore: true
		}
	},
	'simpler-showcase': {
		slug: 'simpler-showcase',
		name: 'Simpler Showcase',
		title: 'Simpler Showcase Dashboard',
		type: 'trading-room',
		description: 'Watch expert traders showcase their strategies in real-time.',
		calendarId: 'simpleroptions.com_showcase@group.calendar.google.com',
		calendarApiKey: 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw',
		calendarClientId: '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com',
		quickLinks: [
			{ label: 'Support', href: 'https://intercom.help/simpler-trading/en/', external: true },
			{ label: 'Platform Tutorials', href: '/tutorials', external: true }
		],
		traders: [
			{
				id: 1,
				name: 'Billy Ribeiro',
				nickname: 'Billy',
				title: 'Lead Day Trader',
				bio: 'Billy brings a disciplined approach to trading with a focus on high-probability setups.',
				image: '/images/traders/billy-ribeiro.jpg',
				specialties: ['Day Trading', 'Risk Management', 'Technical Analysis']
			}
		],
		features: {
			dailyVideos: true,
			learningCenter: true,
			tradingRoomArchive: true,
			traders: true,
			startHere: true,
			resources: false,
			traderStore: false
		}
	}
};

/**
 * Get trading room config by slug
 */
export function getTradingRoom(slug: string): TradingRoomConfig | undefined {
	return tradingRooms[slug];
}

/**
 * Get all valid trading room slugs
 */
export function getAllSlugs(): string[] {
	return Object.keys(tradingRooms);
}

/**
 * Check if a slug is a valid trading room
 */
export function isValidSlug(slug: string): boolean {
	return slug in tradingRooms;
}
