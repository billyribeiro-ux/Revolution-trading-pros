<script lang="ts">
	/**
	 * Individual Lesson Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /learning-center/[slug]
	 * Displays a single lesson with video player and related content.
	 * Matches Simpler Trading's lesson page layout.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import {
		IconPlayerPlay,
		IconClock,
		IconUser,
		IconCalendar,
		IconChevronLeft,
		IconChevronRight,
		IconShare,
		IconBookmark,
		IconDownload,
		IconArrowLeft
	} from '@tabler/icons-svelte';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);

	// ═══════════════════════════════════════════════════════════════════════════
	// LESSONS DATABASE
	// ═══════════════════════════════════════════════════════════════════════════

	interface Lesson {
		id: string;
		slug: string;
		title: string;
		description: string;
		fullDescription?: string;
		trainer: string;
		trainerId: string;
		trainerImage: string;
		category: string;
		duration: string;
		thumbnail: string;
		videoUrl: string;
		posterUrl: string;
		publishedDate: string;
		relatedLessons?: string[];
	}

	const lessonsDb: Record<string, Lesson> = {
		'growing-a-small-account': {
			id: '1',
			slug: 'growing-a-small-account',
			title: 'Growing a Small Account',
			description: 'Allison Ostrander shares her strategies for growing a small trading account effectively.',
			fullDescription: `In this comprehensive session, Allison Ostrander breaks down her proven strategies for growing a small trading account. You'll learn how to manage risk appropriately when capital is limited, identify high-probability setups, and compound gains over time.

Key topics covered:
• Position sizing for small accounts
• Risk management fundamentals
• Building a trading plan
• Psychology of trading with limited capital
• When to scale up your position sizes`,
			trainer: 'Allison Ostrander',
			trainerId: 'allison-ostrander',
			trainerImage: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
			category: 'Trading Strategies',
			duration: '45:00',
			thumbnail: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/GrowingSmallAccount-AO.mp4',
			posterUrl: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
			publishedDate: '2022-11-14',
			relatedLessons: ['fomo-to-jomo-with-ao', 'basic-options-strategies-with-danielle-shay']
		},
		'fomo-to-jomo-with-ao': {
			id: '2',
			slug: 'fomo-to-jomo-with-ao',
			title: 'Psychology: Turning FOMO into JOMO',
			description: 'Learn how to transform Fear of Missing Out into Joy of Missing Out for better trading decisions.',
			fullDescription: `Fear of Missing Out (FOMO) is one of the most dangerous emotions in trading. In this session, Allison Ostrander teaches you how to transform that anxiety into Joy of Missing Out (JOMO) - the peace that comes from disciplined trading.

What you'll learn:
• Understanding the psychology behind FOMO
• How FOMO leads to poor trading decisions
• Techniques for emotional regulation
• Building a JOMO mindset
• Creating rules that eliminate emotional trading
• Real-world examples and case studies`,
			trainer: 'Allison Ostrander',
			trainerId: 'allison-ostrander',
			trainerImage: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
			category: 'Trading Psychology',
			duration: '38:00',
			thumbnail: 'https://cdn.simplertrading.com/2023/06/07133302/Allison-SmallAccountMentorship.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/FOMOtoJOMO-AO-11152022.mp4',
			posterUrl: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/Allison-SmallAccountMentorship.jpg',
			publishedDate: '2022-11-15',
			relatedLessons: ['growing-a-small-account', 'stock-trader-tips-volatile-market']
		},
		'chris-brecher-watchlist': {
			id: '3',
			slug: 'chris-brecher-watchlist',
			title: 'How Chris Brecher Builds His Watchlist',
			description: 'Chris Brecher walks through his process for building and maintaining a trading watchlist.',
			fullDescription: `A well-maintained watchlist is essential for any successful trader. In this lesson, Chris Brecher shares his systematic approach to building and maintaining a watchlist that consistently produces trading opportunities.

Topics covered:
• Criteria for adding stocks to your watchlist
• Fundamental vs technical screening
• Organizing your watchlist by sector and setup type
• Daily maintenance routine
• When to remove stocks from your watchlist
• Tools and resources for watchlist building`,
			trainer: 'Chris Brecher',
			trainerId: 'chris-brecher',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/09/27154605/MemberWebinar-ChrisBrecher.jpg',
			category: 'Trading Strategies',
			duration: '42:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/09/27154605/MemberWebinar-ChrisBrecher.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/Watchlist-CB.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/09/27154605/MemberWebinar-ChrisBrecher.jpg',
			publishedDate: '2022-10-20',
			relatedLessons: ['checklist-buy-stocks', 'stock-trader-tips-volatile-market']
		},
		'basic-options-strategies-with-danielle-shay': {
			id: '4',
			slug: 'basic-options-strategies-with-danielle-shay',
			title: 'Basic Options Strategies',
			description: 'Danielle Shay introduces fundamental options trading strategies for beginners.',
			fullDescription: `Options trading doesn't have to be complicated. In this beginner-friendly session, Danielle Shay breaks down the most essential options strategies you need to know to get started.

What you'll learn:
• Understanding calls and puts
• When to buy vs sell options
• The most common options strategies
• Risk and reward profiles
• Position sizing for options
• Common mistakes to avoid
• Live chart examples`,
			trainer: 'Danielle Shay',
			trainerId: 'danielle-shay',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg',
			category: 'Trading Strategies',
			duration: '55:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/BasicOptions-DS.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120545/MemberWebinar-DanielleShay.jpg',
			publishedDate: '2023-01-15',
			relatedLessons: ['growing-a-small-account', 'intro-divergences']
		},
		'thinkorswim101-eric-purdy': {
			id: '5',
			slug: 'thinkorswim101-eric-purdy',
			title: 'ThinkorSwim (TOS) 101',
			description: 'Eric Purdy provides a comprehensive introduction to the ThinkorSwim trading platform.',
			fullDescription: `The ThinkorSwim (TOS) platform is one of the most powerful trading platforms available. In this comprehensive walkthrough, Eric Purdy shows you everything you need to know to get started.

Course outline:
• Platform overview and layout
• Setting up your workspace
• Charting basics and customization
• Order entry and management
• Scanning for opportunities
• Using technical indicators
• Paper trading setup
• Tips and shortcuts`,
			trainer: 'Eric Purdy',
			trainerId: 'eric-purdy',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			category: 'Platform Tutorials',
			duration: '1:15:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/TOS101-EP.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			publishedDate: '2022-09-10',
			relatedLessons: ['tos-tips-tricks-tg', 'checklist-buy-stocks']
		},
		'quick-hits-review-advanced-version': {
			id: '6',
			slug: 'quick-hits-review-advanced-version',
			title: 'Quick Hits Review — Advanced Version',
			description: 'Kody Ashmore reviews the advanced version of the Quick Hits strategy.',
			fullDescription: `Take your Quick Hits trading to the next level with this advanced review session. Kody Ashmore dives deep into the more sophisticated applications of this powerful strategy.

Advanced topics covered:
• Multi-timeframe Quick Hits analysis
• Combining Quick Hits with other indicators
• Optimizing entries and exits
• Managing trades in different market conditions
• Advanced position sizing techniques
• Real-time trade examples`,
			trainer: 'Kody Ashmore',
			trainerId: 'kody-ashmore',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			category: 'Trading Strategies',
			duration: '48:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/QuickHitsAdvanced-KA.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			publishedDate: '2022-12-01',
			relatedLessons: ['quick-hits-review-james-version', 'intro-divergences']
		},
		'intro-divergences': {
			id: '7',
			slug: 'intro-divergences',
			title: 'An Intro to Divergences',
			description: 'Sam Shames explains how to identify and trade divergences in the market.',
			fullDescription: `Divergences are powerful signals that can help you identify potential trend reversals. In this session, Sam Shames provides a comprehensive introduction to recognizing and trading divergences.

What you'll learn:
• What is a divergence?
• Bullish vs bearish divergences
• Regular vs hidden divergences
• Best indicators for spotting divergences
• How to confirm divergence signals
• Entry and exit strategies
• Common pitfalls to avoid`,
			trainer: 'Sam Shames',
			trainerId: 'sam-shames',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			category: 'Technical Analysis',
			duration: '52:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/Divergences-SS.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			publishedDate: '2022-11-01',
			relatedLessons: ['introduction-yield-curve', 'trading-w-market-internals-breadth']
		},
		'quick-hits-review-james-version': {
			id: '8',
			slug: 'quick-hits-review-james-version',
			title: 'Quick Hits Review — James Version',
			description: 'Sam Shames reviews the Quick Hits strategy and indicators based on the simpler version that John Carter taught his son.',
			fullDescription: `The "James Version" of Quick Hits is a simplified, more accessible approach to this powerful strategy. Sam Shames breaks down this beginner-friendly version that John Carter developed to teach his son James.

Session highlights:
• Understanding the James Version simplifications
• Key indicators and settings
• Step-by-step setup guide
• Trade entry rules
• Managing your trades
• When this version works best
• Transitioning to the advanced version`,
			trainer: 'Sam Shames',
			trainerId: 'sam-shames',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			category: 'Trading Strategies',
			duration: '35:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/QuickHitsJames-SS.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			publishedDate: '2022-12-02',
			relatedLessons: ['quick-hits-review-advanced-version', 'checklist-buy-stocks']
		},
		'introduction-yield-curve': {
			id: '9',
			slug: 'introduction-yield-curve',
			title: 'Introduction to the Yield Curve',
			description: 'Sam reviews the yield curve from a beginner, intermediate, and advanced level.',
			fullDescription: `The yield curve is one of the most important economic indicators for traders to understand. Sam Shames provides a multi-level breakdown suitable for traders at any experience level.

Course structure:
• Beginner: What is the yield curve?
• Beginner: Why does it matter for trading?
• Intermediate: Reading yield curve signals
• Intermediate: Historical patterns and their implications
• Advanced: Trading strategies based on yield curve analysis
• Advanced: Combining yield curve analysis with technical setups`,
			trainer: 'Sam Shames',
			trainerId: 'sam-shames',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			category: 'Technical Analysis',
			duration: '1:05:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/YieldCurve-SS.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			publishedDate: '2023-02-10',
			relatedLessons: ['macro-overview-with-sam-shames', 'intro-divergences']
		},
		'macro-overview-with-sam-shames': {
			id: '10',
			slug: 'macro-overview-with-sam-shames',
			title: 'Macro Overview',
			description: 'Sam Shames provides a comprehensive macro overview of the markets.',
			fullDescription: `Understanding the big picture is essential for making informed trading decisions. In this comprehensive session, Sam Shames breaks down the key macro factors affecting the markets.

Topics covered:
• Current economic landscape
• Interest rates and their impact
• Inflation dynamics
• Sector rotation analysis
• Global market correlations
• Key economic indicators to watch
• How to incorporate macro analysis into your trading`,
			trainer: 'Sam Shames',
			trainerId: 'sam-shames',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			category: 'Technical Analysis',
			duration: '1:20:00',
			thumbnail: 'https://cdn.simplertrading.com/2023/09/19020326/8-1-1.png',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/MacroOverview-SS.mp4',
			posterUrl: 'https://cdn.simplertrading.com/2023/09/19020326/8-1-1.png',
			publishedDate: '2023-09-22',
			relatedLessons: ['introduction-yield-curve', 'trading-w-market-internals-breadth']
		},
		'checklist-buy-stocks': {
			id: '11',
			slug: 'checklist-buy-stocks',
			title: 'Checklist for New Traders Looking to Buy Stocks',
			description: 'TG introduces his checklist for new traders looking to buy stocks and walks through setting up your trading account.',
			fullDescription: `Starting your trading journey can be overwhelming. TG Watkins provides a systematic checklist approach to help new traders get started on the right foot.

Checklist items covered:
• Choosing the right broker
• Account setup essentials
• Funding your account
• Platform familiarization
• Building your first watchlist
• Pre-trade checklist
• Trade execution basics
• Post-trade analysis`,
			trainer: 'TG Watkins',
			trainerId: 'tg-watkins',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			category: 'Getting Started',
			duration: '40:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/Checklist-TG.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			publishedDate: '2023-01-20',
			relatedLessons: ['thinkorswim101-eric-purdy', 'chris-brecher-watchlist']
		},
		'tos-tips-tricks-tg': {
			id: '12',
			slug: 'tos-tips-tricks-tg',
			title: 'ThinkOrSwim: Tips & Tricks',
			description: 'TG Watkins shares some of his favorite features and tips & tricks when using the ThinkOrSwim platform.',
			fullDescription: `Take your ThinkorSwim skills to the next level with these power-user tips and tricks from TG Watkins. These are the features and shortcuts that professional traders use every day.

Tips covered:
• Hidden features most traders miss
• Keyboard shortcuts that save time
• Custom script basics
• Alert setup strategies
• Workspace optimization
• Chart customization tricks
• Order entry shortcuts
• Mobile app tips`,
			trainer: 'TG Watkins',
			trainerId: 'tg-watkins',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			category: 'Platform Tutorials',
			duration: '55:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/TOSTips-TG.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			publishedDate: '2022-10-27',
			relatedLessons: ['thinkorswim101-eric-purdy', 'checklist-buy-stocks']
		},
		'trading-w-market-internals-breadth': {
			id: '13',
			slug: 'trading-w-market-internals-breadth',
			title: 'Trading with Market Internals and Breadth',
			description: 'TG Watkins explains how to use market internals and breadth indicators in your trading.',
			fullDescription: `Market internals and breadth indicators give you insight into the underlying strength or weakness of the market. TG Watkins shows you how to incorporate these powerful tools into your trading.

What you'll learn:
• Understanding market internals
• Key breadth indicators explained
• Advance/decline analysis
• New highs vs new lows
• Volume analysis techniques
• Combining internals with price action
• Building a market dashboard
• Real-time examples`,
			trainer: 'TG Watkins',
			trainerId: 'tg-watkins',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			category: 'Technical Analysis',
			duration: '1:10:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/MarketInternals-TG.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			publishedDate: '2023-03-02',
			relatedLessons: ['intro-divergences', 'macro-overview-with-sam-shames']
		},
		'stock-trader-tips-volatile-market': {
			id: '14',
			slug: 'stock-trader-tips-volatile-market',
			title: 'Stock Trader Tips For Volatile Market',
			description: 'Essential tips and strategies for navigating volatile market conditions.',
			fullDescription: `Volatile markets can be both dangerous and full of opportunity. In this session, learn how to adapt your trading approach when volatility spikes.

Strategies covered:
• Identifying increased volatility
• Adjusting position sizes
• Modified entry techniques
• Using volatility to your advantage
• When to sit on the sidelines
• Protecting existing positions
• Psychology in volatile markets
• Recovery after volatile periods`,
			trainer: 'TG Watkins',
			trainerId: 'tg-watkins',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			category: 'Trading Strategies',
			duration: '45:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/VolatileMarket-TG.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/03/29114004/DS_19-03-29.jpg',
			publishedDate: '2023-02-15',
			relatedLessons: ['fomo-to-jomo-with-ao', 'chris-brecher-watchlist']
		},
		'intro-jonathan-mckeever-favorite-strategy': {
			id: '15',
			slug: 'intro-jonathan-mckeever-favorite-strategy',
			title: "An Introduction to Jonathan McKeever's Favorite Strategy",
			description: 'Jonathan McKeever introduces himself and his favorite strategy, explaining supply and demand and how to identify formations.',
			fullDescription: `Supply and demand zones are powerful areas on charts that can provide excellent trading opportunities. Jonathan McKeever introduces his approach to identifying and trading these zones.

What you'll learn:
• Introduction to supply and demand trading
• How to identify supply zones
• How to identify demand zones
• Formation characteristics
• Entry and exit strategies
• Risk management for S&D trading
• Live chart walkthrough
• Common mistakes to avoid`,
			trainer: 'Jonathan McKeever',
			trainerId: 'jonathan-mckeever',
			trainerImage: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			category: 'Trading Strategies',
			duration: '58:00',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/tutorials/SupplyDemand-JM.mp4',
			posterUrl: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			publishedDate: '2022-11-14',
			relatedLessons: ['intro-divergences', 'basic-options-strategies-with-danielle-shay']
		}
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const lesson = $derived(lessonsDb[slug]);
	const relatedLessons = $derived(
		lesson?.relatedLessons?.map(s => lessonsDb[s]).filter(Boolean) || []
	);

	// Get all lessons as array for navigation
	const allLessons = Object.values(lessonsDb);
	const currentIndex = $derived(allLessons.findIndex(l => l.slug === slug));
	const prevLesson = $derived(currentIndex > 0 ? allLessons[currentIndex - 1] : null);
	const nextLesson = $derived(currentIndex < allLessons.length - 1 ? allLessons[currentIndex + 1] : null);

	// Format date
	function formatDate(dateStr: string) {
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	{#if lesson}
		<title>{lesson.title} | Learning Center | Revolution Trading Pros</title>
		<meta name="description" content={lesson.description} />
		<meta name="robots" content="noindex, follow" />
		<meta property="og:title" content={lesson.title} />
		<meta property="og:description" content={lesson.description} />
		<meta property="og:image" content={lesson.thumbnail} />
		<meta property="og:type" content="article" />
		<meta name="twitter:card" content="summary_large_image" />
	{:else}
		<title>Lesson Not Found | Learning Center | Revolution Trading Pros</title>
		<meta name="description" content="The requested lesson could not be found." />
	{/if}
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     PAGE CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

{#if lesson}
<div class="lesson-page">
	<!-- Header -->
	<header class="lesson-header">
		<div class="lesson-header__inner">
			<nav class="lesson-breadcrumb" aria-label="Breadcrumb">
				<a href="/">Home</a>
				<span class="separator">/</span>
				<a href="/learning-center">Learning Center</a>
				<span class="separator">/</span>
				<span class="current">{lesson.title}</span>
			</nav>
			<a href="/learning-center" class="lesson-back-link">
				<IconArrowLeft size={18} />
				Back to Learning Center
			</a>
		</div>
	</header>

	<!-- Main Content -->
	<div class="lesson-content">
		<div class="lesson-content__inner">
			<!-- Video Section -->
			<div class="lesson-video-section">
				<div class="lesson-video-wrapper">
					<video
						controls
						width="100%"
						poster={lesson.posterUrl}
						preload="metadata"
					>
						<source src={lesson.videoUrl} type="video/mp4" />
						Your browser does not support HTML5 video.
					</video>
				</div>

				<!-- Video Meta -->
				<div class="lesson-video-meta">
					<span class="lesson-category">{lesson.category}</span>
					<div class="lesson-actions">
						<button class="lesson-action-btn" title="Share">
							<IconShare size={18} />
						</button>
						<button class="lesson-action-btn" title="Save for later">
							<IconBookmark size={18} />
						</button>
					</div>
				</div>

				<!-- Title & Info -->
				<h1 class="lesson-title">{lesson.title}</h1>

				<div class="lesson-meta">
					<div class="lesson-meta-item">
						<IconUser size={16} />
						<span>{lesson.trainer}</span>
					</div>
					<div class="lesson-meta-item">
						<IconClock size={16} />
						<span>{lesson.duration}</span>
					</div>
					<div class="lesson-meta-item">
						<IconCalendar size={16} />
						<span>{formatDate(lesson.publishedDate)}</span>
					</div>
				</div>

				<!-- Description -->
				<div class="lesson-description">
					{#each (lesson.fullDescription || lesson.description).split('\n') as paragraph}
						{#if paragraph.trim()}
							<p>{paragraph}</p>
						{/if}
					{/each}
				</div>

				<!-- Trainer Card -->
				<div class="trainer-card">
					<img src={lesson.trainerImage} alt={lesson.trainer} class="trainer-card__image" />
					<div class="trainer-card__info">
						<h3 class="trainer-card__name">{lesson.trainer}</h3>
						<p class="trainer-card__role">Professional Trader & Educator</p>
						<a href="/learning-center?trainer={lesson.trainerId}" class="trainer-card__link">
							View all lessons by {lesson.trainer.split(' ')[0]}
						</a>
					</div>
				</div>

				<!-- Navigation -->
				<div class="lesson-nav">
					{#if prevLesson}
						<a href="/learning-center/{prevLesson.slug}" class="lesson-nav__btn lesson-nav__btn--prev">
							<IconChevronLeft size={20} />
							<div class="lesson-nav__text">
								<span class="lesson-nav__label">Previous Lesson</span>
								<span class="lesson-nav__title">{prevLesson.title}</span>
							</div>
						</a>
					{:else}
						<div></div>
					{/if}

					{#if nextLesson}
						<a href="/learning-center/{nextLesson.slug}" class="lesson-nav__btn lesson-nav__btn--next">
							<div class="lesson-nav__text">
								<span class="lesson-nav__label">Next Lesson</span>
								<span class="lesson-nav__title">{nextLesson.title}</span>
							</div>
							<IconChevronRight size={20} />
						</a>
					{/if}
				</div>
			</div>

			<!-- Sidebar -->
			<aside class="lesson-sidebar">
				<h3 class="sidebar-title">Related Lessons</h3>
				<div class="related-lessons">
					{#each relatedLessons as related}
						<a href="/learning-center/{related.slug}" class="related-card">
							<div class="related-card__thumbnail">
								<img src={related.thumbnail} alt={related.title} />
								<span class="related-card__duration">{related.duration}</span>
							</div>
							<div class="related-card__info">
								<h4 class="related-card__title">{related.title}</h4>
								<span class="related-card__trainer">{related.trainer}</span>
							</div>
						</a>
					{/each}
				</div>

				<a href="/learning-center" class="sidebar-link">
					View All Lessons
					<IconChevronRight size={16} />
				</a>
			</aside>
		</div>
	</div>
</div>
{:else}
<!-- 404 State -->
<div class="lesson-not-found">
	<h1>Lesson Not Found</h1>
	<p>The lesson you're looking for doesn't exist or has been moved.</p>
	<a href="/learning-center" class="btn-primary">
		<IconArrowLeft size={18} />
		Back to Learning Center
	</a>
</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	.lesson-page {
		min-height: 100vh;
		background: #f8f9fa;
	}

	/* Header */
	.lesson-header {
		background: #0a101c;
		padding: 20px 0;
	}

	.lesson-header__inner {
		max-width: 1400px;
		margin: 0 auto;
		padding: 0 24px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: 16px;
	}

	.lesson-breadcrumb {
		font-size: 14px;
		color: rgba(255, 255, 255, 0.6);
	}

	.lesson-breadcrumb a {
		color: rgba(255, 255, 255, 0.6);
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.lesson-breadcrumb a:hover {
		color: #fff;
	}

	.lesson-breadcrumb .separator {
		margin: 0 8px;
	}

	.lesson-breadcrumb .current {
		color: rgba(255, 255, 255, 0.9);
	}

	.lesson-back-link {
		display: flex;
		align-items: center;
		gap: 8px;
		color: #0ea5e9;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		transition: color 0.15s ease;
	}

	.lesson-back-link:hover {
		color: #38bdf8;
	}

	/* Content */
	.lesson-content {
		padding: 40px 0 80px;
	}

	.lesson-content__inner {
		max-width: 1400px;
		margin: 0 auto;
		padding: 0 24px;
		display: grid;
		grid-template-columns: 1fr 350px;
		gap: 40px;
	}

	/* Video Section */
	.lesson-video-section {
		background: #fff;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
	}

	.lesson-video-wrapper {
		background: #000;
		aspect-ratio: 16 / 9;
	}

	.lesson-video-wrapper video {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.lesson-video-meta {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #f1f5f9;
	}

	.lesson-category {
		display: inline-block;
		padding: 6px 12px;
		background: #f0f9ff;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
		color: #0369a1;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.lesson-actions {
		display: flex;
		gap: 8px;
	}

	.lesson-action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border: 1px solid #e5e7eb;
		background: #fff;
		border-radius: 8px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.lesson-action-btn:hover {
		border-color: #0ea5e9;
		color: #0ea5e9;
	}

	.lesson-title {
		font-size: 32px;
		font-weight: 800;
		color: #1e293b;
		margin: 0;
		padding: 24px 24px 16px;
		line-height: 1.2;
	}

	.lesson-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 24px;
		padding: 0 24px 24px;
	}

	.lesson-meta-item {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 14px;
		color: #64748b;
	}

	.lesson-description {
		padding: 0 24px 24px;
		border-bottom: 1px solid #f1f5f9;
	}

	.lesson-description p {
		font-size: 15px;
		line-height: 1.7;
		color: #475569;
		margin: 0 0 16px;
	}

	.lesson-description p:last-child {
		margin-bottom: 0;
	}

	/* Trainer Card */
	.trainer-card {
		display: flex;
		gap: 16px;
		padding: 24px;
		border-bottom: 1px solid #f1f5f9;
	}

	.trainer-card__image {
		width: 72px;
		height: 72px;
		border-radius: 50%;
		object-fit: cover;
	}

	.trainer-card__info {
		flex: 1;
	}

	.trainer-card__name {
		font-size: 18px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 4px;
	}

	.trainer-card__role {
		font-size: 14px;
		color: #64748b;
		margin: 0 0 8px;
	}

	.trainer-card__link {
		font-size: 14px;
		color: #0ea5e9;
		text-decoration: none;
		font-weight: 500;
	}

	.trainer-card__link:hover {
		text-decoration: underline;
	}

	/* Navigation */
	.lesson-nav {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
		padding: 24px;
	}

	.lesson-nav__btn {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		background: #f8f9fa;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.lesson-nav__btn:hover {
		background: #f1f5f9;
	}

	.lesson-nav__btn--next {
		justify-content: flex-end;
		text-align: right;
	}

	.lesson-nav__text {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.lesson-nav__label {
		font-size: 12px;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.lesson-nav__title {
		font-size: 14px;
		font-weight: 600;
		color: #1e293b;
		display: -webkit-box;
		-webkit-line-clamp: 1;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Sidebar */
	.lesson-sidebar {
		background: #fff;
		border-radius: 16px;
		padding: 24px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
		height: fit-content;
		position: sticky;
		top: 24px;
	}

	.sidebar-title {
		font-size: 18px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 20px;
	}

	.related-lessons {
		display: flex;
		flex-direction: column;
		gap: 16px;
		margin-bottom: 24px;
	}

	.related-card {
		display: flex;
		gap: 12px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.related-card:hover {
		opacity: 0.8;
	}

	.related-card__thumbnail {
		position: relative;
		width: 120px;
		flex-shrink: 0;
		border-radius: 8px;
		overflow: hidden;
	}

	.related-card__thumbnail img {
		width: 100%;
		aspect-ratio: 16 / 9;
		object-fit: cover;
	}

	.related-card__duration {
		position: absolute;
		bottom: 4px;
		right: 4px;
		padding: 2px 6px;
		background: rgba(0, 0, 0, 0.8);
		border-radius: 4px;
		font-size: 10px;
		color: #fff;
	}

	.related-card__info {
		flex: 1;
		min-width: 0;
	}

	.related-card__title {
		font-size: 14px;
		font-weight: 600;
		color: #1e293b;
		margin: 0 0 4px;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.related-card__trainer {
		font-size: 12px;
		color: #64748b;
	}

	.sidebar-link {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px;
		background: #f8f9fa;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: #0ea5e9;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.sidebar-link:hover {
		background: #f0f9ff;
	}

	/* 404 State */
	.lesson-not-found {
		min-height: 60vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		padding: 40px 24px;
	}

	.lesson-not-found h1 {
		font-size: 32px;
		font-weight: 800;
		color: #1e293b;
		margin: 0 0 12px;
	}

	.lesson-not-found p {
		font-size: 16px;
		color: #64748b;
		margin: 0 0 24px;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 14px 24px;
		background: #0ea5e9;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: #fff;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.btn-primary:hover {
		background: #0284c7;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.lesson-content__inner {
			grid-template-columns: 1fr;
		}

		.lesson-sidebar {
			position: static;
		}
	}

	@media (max-width: 768px) {
		.lesson-header__inner {
			flex-direction: column;
			align-items: flex-start;
		}

		.lesson-title {
			font-size: 24px;
		}

		.lesson-nav {
			grid-template-columns: 1fr;
		}
	}
</style>
