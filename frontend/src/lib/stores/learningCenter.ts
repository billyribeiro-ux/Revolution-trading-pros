/**
 * Learning Center Store
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Centralized state management for the Learning Center system.
 * Handles data fetching, caching, and user progress tracking.
 *
 * @version 1.0.0 (December 2025)
 */

import { writable, derived, get } from 'svelte/store';
import type {
	TradingRoom,
	Trainer,
	LessonCategory,
	Lesson,
	LessonModule,
	LessonWithRelations,
	LearningCenterData,
	LessonFilter,
	UserLessonProgress,
	UserRoomProgress,
	CreateLessonInput,
	UpdateLessonInput
} from '$lib/types/learning-center';

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - Trading Rooms
// ═══════════════════════════════════════════════════════════════════════════

export const TRADING_ROOMS: TradingRoom[] = [
	{
		id: 'mtt',
		slug: 'day-trading-room',
		name: 'Day Trading Room',
		shortName: 'DTR',
		description: 'Live trading room with professional traders sharing real-time analysis and trade ideas.',
		type: 'trading-room',
		icon: 'st-icon-mastering-the-trade',
		color: '#0984ae',
		tradingRoomUrl: '/trading-room/day-trading-room',
		hasLearningCenter: true,
		hasArchive: true,
		hasDiscord: true,
		isActive: true,
		sortOrder: 1,
		createdAt: '2020-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'rs',
		slug: 'revolution-showcase',
		name: 'Revolution Showcase',
		shortName: 'RS',
		description: 'Exclusive showcase of trading strategies from our top educators.',
		type: 'trading-room',
		icon: 'st-icon-revolution-showcase',
		color: '#6366f1',
		tradingRoomUrl: '/trading-room/revolution-showcase',
		hasLearningCenter: true,
		hasArchive: true,
		hasDiscord: true,
		isActive: true,
		sortOrder: 2,
		createdAt: '2021-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'mm',
		slug: 'moxie-mastery',
		name: 'Moxie Indicator™ Mastery',
		shortName: 'MM',
		description: 'Master the Moxie Indicator with comprehensive training and live sessions.',
		type: 'mastery',
		icon: 'st-icon-moxie',
		color: '#f97316',
		tradingRoomUrl: '/trading-room/moxie-mastery',
		hasLearningCenter: true,
		hasArchive: true,
		hasDiscord: false,
		isActive: true,
		sortOrder: 3,
		createdAt: '2022-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'dt',
		slug: 'day-trading',
		name: 'Day Trading Room',
		shortName: 'DT',
		description: 'Real-time day trading with focus on intraday opportunities.',
		type: 'trading-room',
		icon: 'st-icon-day-trading',
		color: '#10b981',
		tradingRoomUrl: '/trading-room/day-trading',
		hasLearningCenter: true,
		hasArchive: true,
		hasDiscord: true,
		isActive: true,
		sortOrder: 4,
		createdAt: '2020-06-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'st',
		slug: 'swing-trading',
		name: 'Swing Trading Room',
		shortName: 'ST',
		description: 'Multi-day swing trade setups and analysis.',
		type: 'trading-room',
		icon: 'st-icon-swing-trading',
		color: '#8b5cf6',
		tradingRoomUrl: '/trading-room/swing-trading',
		hasLearningCenter: true,
		hasArchive: true,
		hasDiscord: true,
		isActive: true,
		sortOrder: 5,
		createdAt: '2020-06-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'sa',
		slug: 'small-accounts',
		name: 'Small Account Trading',
		shortName: 'SA',
		description: 'Strategies specifically designed for growing small accounts.',
		type: 'trading-room',
		icon: 'st-icon-small-accounts',
		color: '#ec4899',
		tradingRoomUrl: '/trading-room/small-accounts',
		hasLearningCenter: true,
		hasArchive: true,
		hasDiscord: true,
		isActive: true,
		sortOrder: 6,
		createdAt: '2021-03-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'ww',
		slug: 'weekly-watchlist',
		name: 'Weekly Watchlist',
		shortName: 'WW',
		description: 'Weekly curated stock watchlist with detailed analysis.',
		type: 'alert-service',
		icon: 'st-icon-trade-of-the-week',
		color: '#14b8a6',
		hasLearningCenter: true,
		hasArchive: true,
		hasDiscord: false,
		isActive: true,
		sortOrder: 7,
		createdAt: '2022-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'spx',
		slug: 'spx-profit-pulse',
		name: 'SPX Profit Pulse',
		shortName: 'SPX',
		description: 'SPX-focused trading alerts and strategies.',
		type: 'alert-service',
		icon: 'st-icon-spx-profit-pulse',
		color: '#f59e0b',
		hasLearningCenter: true,
		hasArchive: true,
		hasDiscord: false,
		isActive: true,
		sortOrder: 8,
		createdAt: '2023-01-01',
		updatedAt: '2025-12-01'
	}
];

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - Trainers
// ═══════════════════════════════════════════════════════════════════════════

export const TRAINERS: Trainer[] = [
	{
		id: 'ao',
		slug: 'allison-ostrander',
		name: 'Allison Ostrander',
		firstName: 'Allison',
		lastName: 'Ostrander',
		title: 'Director of Risk Tolerance & Small Account Specialist',
		bio: 'Allison Ostrander specializes in helping traders manage risk and grow small accounts. Her unique approach to position sizing and risk management has helped thousands of traders.',
		shortBio: 'Small Account Specialist',
		imageUrl: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
		specialties: ['Small Accounts', 'Risk Management', 'Psychology'],
		isActive: true,
		sortOrder: 1,
		createdAt: '2020-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'ds',
		slug: 'danielle-shay',
		name: 'Danielle Shay',
		firstName: 'Danielle',
		lastName: 'Shay',
		title: 'Director of Options',
		bio: 'Danielle Shay is the Director of Options with over 15 years of trading experience. She specializes in options strategies and helping beginners understand complex concepts.',
		shortBio: 'Options Expert',
		imageUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg',
		specialties: ['Options', 'Technical Analysis', 'Swing Trading'],
		isActive: true,
		sortOrder: 2,
		createdAt: '2020-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'ss',
		slug: 'sam-shames',
		name: 'Sam Shames',
		firstName: 'Sam',
		lastName: 'Shames',
		title: 'Options & Macro Analyst',
		bio: 'Sam Shames brings a unique perspective combining macro analysis with technical trading. His background in economics provides comprehensive market insights.',
		shortBio: 'Macro Analyst',
		imageUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
		specialties: ['Macro Analysis', 'Yield Curve', 'Divergences'],
		isActive: true,
		sortOrder: 3,
		createdAt: '2020-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'tg',
		slug: 'tg-watkins',
		name: 'TG Watkins',
		firstName: 'TG',
		lastName: 'Watkins',
		title: 'Director of Stocks',
		bio: 'TG Watkins is a veteran trader specializing in stocks and market internals. His systematic approach to trading has been refined over decades.',
		shortBio: 'Stocks Expert',
		imageUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
		specialties: ['Stocks', 'Market Internals', 'ThinkorSwim'],
		isActive: true,
		sortOrder: 4,
		createdAt: '2020-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'cb',
		slug: 'chris-brecher',
		name: 'Chris Brecher',
		firstName: 'Chris',
		lastName: 'Brecher',
		title: 'Watchlist & Scanning Expert',
		bio: 'Chris Brecher specializes in building effective watchlists and scanning for trading opportunities.',
		shortBio: 'Watchlist Expert',
		imageUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/09/27154605/MemberWebinar-ChrisBrecher.jpg',
		specialties: ['Watchlists', 'Scanning', 'Stock Selection'],
		isActive: true,
		sortOrder: 5,
		createdAt: '2020-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'ep',
		slug: 'eric-purdy',
		name: 'Eric Purdy',
		firstName: 'Eric',
		lastName: 'Purdy',
		title: 'Platform & Technology Specialist',
		bio: 'Eric Purdy is the go-to expert for trading platform setup and optimization, especially ThinkorSwim.',
		shortBio: 'Platform Specialist',
		imageUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
		specialties: ['ThinkorSwim', 'Platform Setup', 'Technology'],
		isActive: true,
		sortOrder: 6,
		createdAt: '2020-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'ka',
		slug: 'kody-ashmore',
		name: 'Kody Ashmore',
		firstName: 'Kody',
		lastName: 'Ashmore',
		title: 'Quick Hits Specialist',
		bio: 'Kody Ashmore specializes in the Quick Hits trading strategy with advanced applications.',
		shortBio: 'Quick Hits Expert',
		imageUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
		specialties: ['Quick Hits', 'Day Trading', 'Indicators'],
		isActive: true,
		sortOrder: 7,
		createdAt: '2020-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'jm',
		slug: 'jonathan-mckeever',
		name: 'Jonathan McKeever',
		firstName: 'Jonathan',
		lastName: 'McKeever',
		title: 'Supply & Demand Specialist',
		bio: 'Jonathan McKeever specializes in supply and demand zone trading strategies.',
		shortBio: 'Supply & Demand Expert',
		imageUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
		specialties: ['Supply & Demand', 'Price Action', 'Zone Trading'],
		isActive: true,
		sortOrder: 8,
		createdAt: '2020-01-01',
		updatedAt: '2025-12-01'
	}
];

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - Categories
// ═══════════════════════════════════════════════════════════════════════════

export const CATEGORIES: LessonCategory[] = [
	{ id: 'getting-started', slug: 'getting-started', name: 'Getting Started', icon: '🚀', sortOrder: 1, isActive: true },
	{ id: 'strategies', slug: 'strategies', name: 'Trading Strategies', icon: '📈', sortOrder: 2, isActive: true },
	{ id: 'technical', slug: 'technical', name: 'Technical Analysis', icon: '📊', sortOrder: 3, isActive: true },
	{ id: 'psychology', slug: 'psychology', name: 'Trading Psychology', icon: '🧠', sortOrder: 4, isActive: true },
	{ id: 'platform', slug: 'platform', name: 'Platform Tutorials', icon: '💻', sortOrder: 5, isActive: true },
	{ id: 'indicators', slug: 'indicators', name: 'Indicators', icon: '📉', sortOrder: 6, isActive: true },
	{ id: 'options', slug: 'options', name: 'Options Trading', icon: '🎯', sortOrder: 7, isActive: true },
	{ id: 'risk', slug: 'risk', name: 'Risk Management', icon: '🛡️', sortOrder: 8, isActive: true }
];

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - Lessons
// ═══════════════════════════════════════════════════════════════════════════

export const LESSONS: Lesson[] = [
	// Day Trading Room lessons
	{
		id: 'mtt-1',
		slug: 'welcome-to-dtr',
		title: 'Welcome to Day Trading Room',
		description: 'Introduction to the Day Trading Room and what to expect.',
		fullDescription: 'Get started with Day Trading Room. Learn how the room operates, meet the traders, and understand the trading methodology.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/Welcome-MTT.mp4',
		posterUrl: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
		duration: '15:00',
		tradingRoomIds: ['mtt'],
		trainerId: 'ao',
		categoryId: 'getting-started',
		tags: ['introduction', 'getting-started'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: true,
		isPinned: true,
		sortOrder: 1,
		moduleId: 'mtt-mod-1',
		moduleOrder: 1,
		publishedAt: '2024-01-01',
		createdAt: '2024-01-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'mtt-2',
		slug: 'fomo-to-jomo',
		title: 'Psychology: Turning FOMO into JOMO',
		description: 'Learn to transform Fear of Missing Out into Joy of Missing Out.',
		fullDescription: 'Master your trading psychology by understanding FOMO and developing the discipline to trade only your best setups.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/FOMOtoJOMO-AO-11152022.mp4',
		posterUrl: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
		duration: '38:00',
		tradingRoomIds: ['mtt', 'sa'],
		trainerId: 'ao',
		categoryId: 'psychology',
		tags: ['psychology', 'fomo', 'discipline'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: true,
		isPinned: false,
		sortOrder: 2,
		moduleId: 'mtt-mod-2',
		moduleOrder: 1,
		publishedAt: '2022-11-15',
		createdAt: '2022-11-15',
		updatedAt: '2025-12-01'
	},
	{
		id: 'mtt-3',
		slug: 'growing-small-account',
		title: 'Growing a Small Account',
		description: 'Strategies for effectively growing a small trading account.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/GrowingSmallAccount-AO.mp4',
		posterUrl: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
		duration: '45:00',
		tradingRoomIds: ['mtt', 'sa'],
		trainerId: 'ao',
		categoryId: 'strategies',
		tags: ['small-account', 'position-sizing', 'risk'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: false,
		isPinned: false,
		sortOrder: 3,
		moduleId: 'mtt-mod-2',
		moduleOrder: 2,
		publishedAt: '2022-11-14',
		createdAt: '2022-11-14',
		updatedAt: '2025-12-01'
	},
	{
		id: 'mtt-4',
		slug: 'basic-options-strategies',
		title: 'Basic Options Strategies',
		description: 'Introduction to fundamental options trading strategies.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/BasicOptions-DS.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg',
		duration: '55:00',
		tradingRoomIds: ['mtt'],
		trainerId: 'ds',
		categoryId: 'options',
		tags: ['options', 'calls', 'puts', 'basics'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: true,
		isPinned: false,
		sortOrder: 4,
		moduleId: 'mtt-mod-3',
		moduleOrder: 1,
		publishedAt: '2023-01-15',
		createdAt: '2023-01-15',
		updatedAt: '2025-12-01'
	},
	{
		id: 'mtt-5',
		slug: 'intro-divergences',
		title: 'An Intro to Divergences',
		description: 'Learn to identify and trade divergences in the market.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/Divergences-SS.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
		duration: '52:00',
		tradingRoomIds: ['mtt', 'st'],
		trainerId: 'ss',
		categoryId: 'technical',
		tags: ['divergences', 'technical-analysis', 'indicators'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: false,
		isPinned: false,
		sortOrder: 5,
		moduleId: 'mtt-mod-4',
		moduleOrder: 1,
		publishedAt: '2022-11-01',
		createdAt: '2022-11-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'mtt-6',
		slug: 'macro-overview',
		title: 'Macro Overview',
		description: 'Comprehensive macro analysis of the markets.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/MacroOverview-SS.mp4',
		posterUrl: 'https://cdn.simplertrading.com/2023/09/19020326/8-1-1.png',
		thumbnailUrl: 'https://cdn.simplertrading.com/2023/09/19020326/8-1-1.png',
		duration: '1:20:00',
		tradingRoomIds: ['mtt', 'st', 'dt'],
		trainerId: 'ss',
		categoryId: 'technical',
		tags: ['macro', 'economics', 'fed', 'interest-rates'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: true,
		isPinned: false,
		sortOrder: 6,
		moduleId: 'mtt-mod-4',
		moduleOrder: 2,
		publishedAt: '2023-09-22',
		createdAt: '2023-09-22',
		updatedAt: '2025-12-01'
	},
	{
		id: 'mtt-7',
		slug: 'yield-curve-intro',
		title: 'Introduction to the Yield Curve',
		description: 'Understanding the yield curve from beginner to advanced.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/YieldCurve-SS.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
		duration: '1:05:00',
		tradingRoomIds: ['mtt'],
		trainerId: 'ss',
		categoryId: 'technical',
		tags: ['yield-curve', 'bonds', 'macro'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: false,
		isPinned: false,
		sortOrder: 7,
		moduleId: 'mtt-mod-4',
		moduleOrder: 3,
		publishedAt: '2023-02-10',
		createdAt: '2023-02-10',
		updatedAt: '2025-12-01'
	},
	// Day Trading room lessons
	{
		id: 'dt-1',
		slug: 'checklist-buy-stocks',
		title: 'Checklist for New Traders',
		description: 'Essential checklist for new traders looking to buy stocks.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/Checklist-TG.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
		duration: '40:00',
		tradingRoomIds: ['dt', 'mtt'],
		trainerId: 'tg',
		categoryId: 'getting-started',
		tags: ['beginner', 'checklist', 'stocks'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: true,
		isPinned: true,
		sortOrder: 1,
		moduleId: 'dt-mod-1',
		moduleOrder: 1,
		publishedAt: '2023-01-20',
		createdAt: '2023-01-20',
		updatedAt: '2025-12-01'
	},
	{
		id: 'dt-2',
		slug: 'tos-101',
		title: 'ThinkorSwim (TOS) 101',
		description: 'Comprehensive introduction to the ThinkorSwim platform.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/TOS101-EP.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
		duration: '1:15:00',
		tradingRoomIds: ['dt', 'mtt', 'st'],
		trainerId: 'ep',
		categoryId: 'platform',
		tags: ['thinkorswim', 'platform', 'setup'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: false,
		isPinned: false,
		sortOrder: 2,
		moduleId: 'dt-mod-1',
		moduleOrder: 2,
		publishedAt: '2022-09-10',
		createdAt: '2022-09-10',
		updatedAt: '2025-12-01'
	},
	{
		id: 'dt-3',
		slug: 'tos-tips-tricks',
		title: 'ThinkOrSwim: Tips & Tricks',
		description: 'Advanced tips and shortcuts for ThinkorSwim users.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/TOSTips-TG.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
		duration: '55:00',
		tradingRoomIds: ['dt', 'mtt'],
		trainerId: 'tg',
		categoryId: 'platform',
		tags: ['thinkorswim', 'tips', 'advanced'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: false,
		isPinned: false,
		sortOrder: 3,
		moduleId: 'dt-mod-1',
		moduleOrder: 3,
		publishedAt: '2022-10-27',
		createdAt: '2022-10-27',
		updatedAt: '2025-12-01'
	},
	{
		id: 'dt-4',
		slug: 'market-internals-breadth',
		title: 'Trading with Market Internals and Breadth',
		description: 'How to use market internals and breadth indicators.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/MarketInternals-TG.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
		duration: '1:10:00',
		tradingRoomIds: ['dt'],
		trainerId: 'tg',
		categoryId: 'technical',
		tags: ['internals', 'breadth', 'advance-decline'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: true,
		isPinned: false,
		sortOrder: 4,
		moduleId: 'dt-mod-2',
		moduleOrder: 1,
		publishedAt: '2023-03-02',
		createdAt: '2023-03-02',
		updatedAt: '2025-12-01'
	},
	{
		id: 'dt-5',
		slug: 'volatile-market-tips',
		title: 'Stock Trader Tips For Volatile Markets',
		description: 'Essential strategies for navigating volatile conditions.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/VolatileMarket-TG.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
		duration: '45:00',
		tradingRoomIds: ['dt', 'st'],
		trainerId: 'tg',
		categoryId: 'strategies',
		tags: ['volatility', 'risk', 'adaptation'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: false,
		isPinned: false,
		sortOrder: 5,
		moduleId: 'dt-mod-2',
		moduleOrder: 2,
		publishedAt: '2023-02-15',
		createdAt: '2023-02-15',
		updatedAt: '2025-12-01'
	},
	// More lessons for various rooms...
	{
		id: 'cb-1',
		slug: 'building-watchlist',
		title: 'How to Build Your Watchlist',
		description: 'Chris Brecher walks through his watchlist building process.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/Watchlist-CB.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/09/27154605/MemberWebinar-ChrisBrecher.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/09/27154605/MemberWebinar-ChrisBrecher.jpg',
		duration: '42:00',
		tradingRoomIds: ['mtt', 'st', 'dt', 'ww'],
		trainerId: 'cb',
		categoryId: 'strategies',
		tags: ['watchlist', 'scanning', 'stock-selection'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: false,
		isPinned: false,
		sortOrder: 1,
		publishedAt: '2022-10-20',
		createdAt: '2022-10-20',
		updatedAt: '2025-12-01'
	},
	{
		id: 'ka-1',
		slug: 'quick-hits-advanced',
		title: 'Quick Hits Review — Advanced Version',
		description: 'Advanced applications of the Quick Hits strategy.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/QuickHitsAdvanced-KA.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
		duration: '48:00',
		tradingRoomIds: ['mtt', 'dt'],
		trainerId: 'ka',
		categoryId: 'strategies',
		tags: ['quick-hits', 'advanced', 'indicators'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: false,
		isPinned: false,
		sortOrder: 1,
		publishedAt: '2022-12-01',
		createdAt: '2022-12-01',
		updatedAt: '2025-12-01'
	},
	{
		id: 'jm-1',
		slug: 'supply-demand-intro',
		title: "Jonathan McKeever's Favorite Strategy",
		description: 'Introduction to supply and demand zone trading.',
		type: 'video',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/SupplyDemand-JM.mp4',
		posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
		thumbnailUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
		duration: '58:00',
		tradingRoomIds: ['mtt', 'st'],
		trainerId: 'jm',
		categoryId: 'strategies',
		tags: ['supply-demand', 'zones', 'price-action'],
		status: 'published',
		accessLevel: 'member',
		isFeatured: false,
		isPinned: false,
		sortOrder: 1,
		publishedAt: '2022-11-14',
		createdAt: '2022-11-14',
		updatedAt: '2025-12-01'
	}
];

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - Modules
// ═══════════════════════════════════════════════════════════════════════════

export const MODULES: LessonModule[] = [
	// Day Trading Room modules
	{ id: 'mtt-mod-1', slug: 'getting-started', tradingRoomId: 'mtt', name: 'Getting Started', description: 'Start here to get oriented', sortOrder: 1 },
	{ id: 'mtt-mod-2', slug: 'psychology-risk', tradingRoomId: 'mtt', name: 'Psychology & Risk', description: 'Master your mindset and risk management', sortOrder: 2 },
	{ id: 'mtt-mod-3', slug: 'options-basics', tradingRoomId: 'mtt', name: 'Options Basics', description: 'Learn options fundamentals', sortOrder: 3 },
	{ id: 'mtt-mod-4', slug: 'technical-analysis', tradingRoomId: 'mtt', name: 'Technical Analysis', description: 'Charts, indicators, and analysis', sortOrder: 4 },
	// Day Trading modules
	{ id: 'dt-mod-1', slug: 'platform-setup', tradingRoomId: 'dt', name: 'Platform & Setup', description: 'Get your platform ready', sortOrder: 1 },
	{ id: 'dt-mod-2', slug: 'day-trading-strategies', tradingRoomId: 'dt', name: 'Day Trading Strategies', description: 'Intraday trading techniques', sortOrder: 2 }
];

// ═══════════════════════════════════════════════════════════════════════════
// STORE STATE
// ═══════════════════════════════════════════════════════════════════════════

interface LearningCenterState {
	tradingRooms: TradingRoom[];
	trainers: Trainer[];
	categories: LessonCategory[];
	lessons: Lesson[];
	modules: LessonModule[];
	userProgress: Map<string, UserLessonProgress>;
	isLoading: boolean;
	error: string | null;
	lastFetch: Date | null;
}

function createLearningCenterStore() {
	const { subscribe, set, update } = writable<LearningCenterState>({
		tradingRooms: TRADING_ROOMS,
		trainers: TRAINERS,
		categories: CATEGORIES,
		lessons: LESSONS,
		modules: MODULES,
		userProgress: new Map(),
		isLoading: false,
		error: null,
		lastFetch: null
	});

	return {
		subscribe,

		// ═══════════════════════════════════════════════════════════════════════
		// GETTERS
		// ═══════════════════════════════════════════════════════════════════════

		getTradingRoom(slug: string): TradingRoom | undefined {
			const state = get({ subscribe });
			return state.tradingRooms.find(r => r.slug === slug);
		},

		getTradingRoomById(id: string): TradingRoom | undefined {
			const state = get({ subscribe });
			return state.tradingRooms.find(r => r.id === id);
		},

		getTrainer(id: string): Trainer | undefined {
			const state = get({ subscribe });
			return state.trainers.find(t => t.id === id);
		},

		getCategory(id: string): LessonCategory | undefined {
			const state = get({ subscribe });
			return state.categories.find(c => c.id === id);
		},

		getLessonsForRoom(tradingRoomId: string): Lesson[] {
			const state = get({ subscribe });
			return state.lessons.filter(l =>
				l.tradingRoomIds?.includes(tradingRoomId) && l.status === 'published'
			).sort((a, b) => (a.sortOrder ?? 0) - (b.sortOrder ?? 0));
		},

		getModulesForRoom(tradingRoomId: string): LessonModule[] {
			const state = get({ subscribe });
			return state.modules.filter(m => m.tradingRoomId === tradingRoomId)
				.sort((a, b) => (a.sortOrder ?? 0) - (b.sortOrder ?? 0));
		},

		getLessonsForModule(moduleId: string): Lesson[] {
			const state = get({ subscribe });
			return state.lessons.filter(l => l.moduleId === moduleId && l.status === 'published')
				.sort((a, b) => (a.moduleOrder ?? 0) - (b.moduleOrder ?? 0));
		},

		getLessonBySlug(slug: string): LessonWithRelations | undefined {
			const state = get({ subscribe });
			const lesson = state.lessons.find(l => l.slug === slug);
			if (!lesson) return undefined;

			return {
				...lesson,
				trainer: state.trainers.find(t => t.id === lesson.trainerId),
				category: state.categories.find(c => c.id === lesson.categoryId),
				tradingRooms: state.tradingRooms.filter(r => lesson.tradingRoomIds?.includes(r.id)),
				module: state.modules.find(m => m.id === lesson.moduleId)
			};
		},

		// ═══════════════════════════════════════════════════════════════════════
		// FILTERING
		// ═══════════════════════════════════════════════════════════════════════

		filterLessons(filter: LessonFilter): LessonWithRelations[] {
			const state = get({ subscribe });
			let filtered = state.lessons.filter(l => l.status === 'published');

			if (filter.tradingRoomId) {
				filtered = filtered.filter(l => l.tradingRoomIds?.includes(filter.tradingRoomId!));
			}
			if (filter.trainerId) {
				filtered = filtered.filter(l => l.trainerId === filter.trainerId);
			}
			if (filter.categoryId) {
				filtered = filtered.filter(l => l.categoryId === filter.categoryId);
			}
			if (filter.moduleId) {
				filtered = filtered.filter(l => l.moduleId === filter.moduleId);
			}
			if (filter.type) {
				filtered = filtered.filter(l => l.type === filter.type);
			}
			if (filter.isFeatured !== undefined) {
				filtered = filtered.filter(l => l.isFeatured === filter.isFeatured);
			}
			if (filter.search) {
				const searchLower = filter.search.toLowerCase();
				filtered = filtered.filter(l =>
					l.title.toLowerCase().includes(searchLower) ||
					l.description.toLowerCase().includes(searchLower) ||
					l.tags?.some(t => t.toLowerCase().includes(searchLower))
				);
			}
			if (filter.tags && filter.tags.length > 0) {
				filtered = filtered.filter(l =>
					l.tags && l.tags.some(tag => filter.tags!.includes(tag))
				);
			}

			// Add relations
			return filtered.map(lesson => ({
				...lesson,
				trainer: state.trainers.find(t => t.id === lesson.trainerId),
				category: state.categories.find(c => c.id === lesson.categoryId),
				tradingRooms: state.tradingRooms.filter(r => lesson.tradingRoomIds?.includes(r.id)),
				module: state.modules.find(m => m.id === lesson.moduleId)
			})).sort((a, b) => (a.sortOrder ?? 0) - (b.sortOrder ?? 0));
		},

		async createLesson(input: CreateLessonInput): Promise<Lesson> {
			const state = get({ subscribe });
			const newLesson: Lesson = {
				id: `lesson-${Date.now()}`,
				slug: input.title.toLowerCase().replace(/[^a-z0-9]+/g, '-'),
				...input,
				tags: input.tags || [],
				status: input.status || 'draft',
				accessLevel: input.accessLevel || 'member',
				isFeatured: false,
				isPinned: false,
				sortOrder: state.lessons.length + 1,
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString()
			};

			update(s => ({
				...s,
				lessons: [...s.lessons, newLesson]
			}));

			// TODO: API call to persist
			return newLesson;
		},

		async updateLesson(input: UpdateLessonInput): Promise<Lesson> {
			let updatedLesson: Lesson | undefined;

			update(s => {
				const lessons = s.lessons.map(l => {
					if (l.id === input.id) {
						updatedLesson = {
							...l,
							...input,
							updatedAt: new Date().toISOString()
						};
						return updatedLesson;
					}
					return l;
				});
				return { ...s, lessons };
			});

			// TODO: API call to persist
			return updatedLesson!;
		},

		async deleteLesson(id: string): Promise<void> {
			update(s => ({
				...s,
				lessons: s.lessons.filter(l => l.id !== id)
			}));
			// TODO: API call to persist
		},

		// ═══════════════════════════════════════════════════════════════════════
		// USER PROGRESS
		// ═══════════════════════════════════════════════════════════════════════

		async markLessonComplete(lessonId: string, tradingRoomId: string): Promise<void> {
			update(s => {
				const progress = new Map(s.userProgress);
				progress.set(lessonId, {
					id: `progress-${Date.now()}`,
					userId: 'current-user', // Would come from auth
					lessonId,
					tradingRoomId,
					isCompleted: true,
					completedAt: new Date().toISOString(),
					progressPercent: 100,
					isBookmarked: false,
					firstViewedAt: new Date().toISOString(),
					lastViewedAt: new Date().toISOString()
				});
				return { ...s, userProgress: progress };
			});
			// TODO: API call to persist
		},

		async updateLessonProgress(lessonId: string, progressPercent: number, lastPosition?: number): Promise<void> {
			update(s => {
				const progress = new Map(s.userProgress);
				const existing = progress.get(lessonId);
				progress.set(lessonId, {
					...existing,
					id: existing?.id || `progress-${Date.now()}`,
					userId: 'current-user',
					lessonId,
					tradingRoomId: existing?.tradingRoomId || '',
					isCompleted: progressPercent >= 95,
					completedAt: progressPercent >= 95 ? new Date().toISOString() : undefined,
					progressPercent,
					lastPosition,
					isBookmarked: existing?.isBookmarked || false,
					firstViewedAt: existing?.firstViewedAt || new Date().toISOString(),
					lastViewedAt: new Date().toISOString()
				});
				return { ...s, userProgress: progress };
			});
		},

		async toggleBookmark(lessonId: string): Promise<void> {
			update(s => {
				const progress = new Map(s.userProgress);
				const existing = progress.get(lessonId);
				if (existing) {
					progress.set(lessonId, {
						...existing,
						isBookmarked: !existing.isBookmarked
					});
				}
				return { ...s, userProgress: progress };
			});
		},

		// ═══════════════════════════════════════════════════════════════════════
		// DATA LOADING (future API integration)
		// ═══════════════════════════════════════════════════════════════════════

		async loadData(): Promise<void> {
			update(s => ({ ...s, isLoading: true, error: null }));

			try {
				// TODO: Replace with actual API calls
				// const [rooms, trainers, categories, lessons, modules] = await Promise.all([
				//   api.getTradingRooms(),
				//   api.getTrainers(),
				//   api.getCategories(),
				//   api.getLessons(),
				//   api.getModules()
				// ]);

				update(s => ({
					...s,
					// Use mock data for now
					tradingRooms: TRADING_ROOMS,
					trainers: TRAINERS,
					categories: CATEGORIES,
					lessons: LESSONS,
					modules: MODULES,
					isLoading: false,
					lastFetch: new Date()
				}));
			} catch (error: any) {
				update(s => ({
					...s,
					isLoading: false,
					error: error.message || 'Failed to load learning center data'
				}));
			}
		},

		reset() {
			set({
				tradingRooms: TRADING_ROOMS,
				trainers: TRAINERS,
				categories: CATEGORIES,
				lessons: LESSONS,
				modules: MODULES,
				userProgress: new Map(),
				isLoading: false,
				error: null,
				lastFetch: null
			});
		}
	};
}

export const learningCenterStore = createLearningCenterStore();

// ═══════════════════════════════════════════════════════════════════════════
// DERIVED STORES
// ═══════════════════════════════════════════════════════════════════════════

export const activeTradingRooms = derived(
	learningCenterStore,
	$store => $store.tradingRooms.filter(r => r.isActive && r.hasLearningCenter)
);

export const activeTrainers = derived(
	learningCenterStore,
	$store => $store.trainers.filter(t => t.isActive)
);

export const activeCategories = derived(
	learningCenterStore,
	$store => $store.categories.filter(c => c.isActive)
);

export const featuredLessons = derived(
	learningCenterStore,
	$store => $store.lessons.filter(l => l.isFeatured && l.status === 'published')
);
