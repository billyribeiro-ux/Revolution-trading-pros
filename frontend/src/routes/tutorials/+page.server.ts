/**
 * Platform Tutorials - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * SSR Configuration (SvelteKit Official Docs):
 * - ssr: true - Server-side render for SEO and fast initial load
 * - csr: true - Enable client-side hydration for interactivity
 * - prerender: false - Dynamic content from API
 *
 * @version 1.0.0
 */

import type { PageServerLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

// SSR/SSG Configuration - Per SvelteKit Official Docs
export const ssr = true;
export const csr = true;
export const prerender = false;

export interface Tutorial {
	id: string;
	title: string;
	slug: string;
	excerpt: string;
	thumbnail: string;
	platform: 'thinkorswim' | 'tradestation';
	url: string;
}

export interface PageData {
	tosTutorials: Tutorial[];
	tradestationTutorials: Tutorial[];
	tosPagination: {
		page: number;
		totalPages: number;
	};
	seo: SEOInput;
}

// ThinkorSwim tutorials data
const tosTutorials: Tutorial[] = [
	{
		id: '1',
		title: 'Set up Time Frames in TOS',
		slug: 'time-frames-in-tos',
		excerpt: 'Looking at the time frame setup and how to customize it.',
		thumbnail:
			'https://cdn.simplertrading.com/2021/06/27152614/video-content-bg-1-2048x1152-1-1024x576.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/time-frames-in-tos'
	},
	{
		id: '2',
		title: 'ThinkorSwim Tools',
		slug: 'thinkorswim-tools',
		excerpt: 'Learn how to set a default for a tool in ThinkorSwim.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2020/07/16175820/video-content-bg-1-1024x576.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/thinkorswim-tools'
	},
	{
		id: '3',
		title: 'Moving Averages: ThinkorSwim Setup Tutorial',
		slug: 'moving-averages-tos-setup',
		excerpt: 'A quick tutorial on setting up a Moving Average on ThinkorSwim.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/08/03142158/st_tos_movingaverages.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/moving-averages-tos-setup'
	},
	{
		id: '4',
		title: 'Using Thinkorswim for Order Execution',
		slug: 'tos-order-execution',
		excerpt: 'In this video we will walk-through how to set up an options trade order.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/06/05104630/7-TOS-for-order-execution1-1024x695.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/tos-order-execution'
	},
	{
		id: '5',
		title: 'Getting Familiar with the Options Chain in Thinkorswim',
		slug: 'tos-options-chain',
		excerpt:
			'In this video we will walk-through the Option Chain and how it is set up on the ThinkorSwim platform.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/06/05104633/6-getting-familiar-with-TOS1-1024x695.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/tos-options-chain'
	},
	{
		id: '6',
		title: 'Watchlists in Thinkorswim',
		slug: 'tos-watchlists',
		excerpt:
			'Watchlists help keep you organized and on top of changes in your key factors like price, volume, open interest, and more.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/06/05104638/5-Watchlists-in-TOS1-1024x695.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/tos-watchlists'
	},
	{
		id: '7',
		title: 'Indicator Tutorials for the Thinkorswim Platform',
		slug: 'tos-indicator-tutorials',
		excerpt:
			'In this video we talk about the most popular indicators in order to stay safe trading and give us the greatest edge possible.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/06/05104641/4-Indicators-Tutorials-for-TOS1-1024x695.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/tos-indicator-tutorials'
	},
	{
		id: '8',
		title: 'Introduction to Charting in Thinkorswim',
		slug: 'introduction-to-charting-in-thinkorswim',
		excerpt:
			'Understanding how to change symbols, time frames, zoom and scroll features as well as drawing tools are all part of the charting process.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/06/05104644/3-Intro-to-Charting-in-TOS1-1024x695.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/introduction-to-charting-in-thinkorswim'
	},
	{
		id: '9',
		title: 'Thinkorswim Platform Features and Overview',
		slug: 'tos-platform-features-overview',
		excerpt:
			'This video will walk you through the Thinkorswim Platform to help you build a comfort level with where things are.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/06/05104647/2-TOS-Features-and-Overview1-1024x695.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/tos-platform-features-overview'
	},
	{
		id: '10',
		title: 'ThinkorSwim: Stops, Targets, and OCO Brackets',
		slug: 'stops-targets-ocos-brackets-thinkorswim',
		excerpt:
			'Henry Gambell reviews the different ways to adjust your positions including stop losses, targets, and OCO brackets in ThinkorSwim.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/07/27114831/MemberWebinar-Henry.jpg',
		platform: 'thinkorswim',
		url: '/learning-center/stops-targets-ocos-brackets-thinkorswim'
	}
];

// TradeStation tutorials data
const tradestationTutorials: Tutorial[] = [
	{
		id: '101',
		title: 'Resize the 10X Volume Dots',
		slug: 'resize-10x-volume-dots',
		excerpt: 'Learn how to change the size of 10X Volume Dots in Tradestation.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2020/07/16175820/video-content-bg-1-1024x576.jpg',
		platform: 'tradestation',
		url: '/learning-center/resize-10x-volume-dots'
	},
	{
		id: '102',
		title: 'How to find TS Workspaces',
		slug: 'ts-workspaces',
		excerpt: 'Learn how to get to the workspaces that are on our computer.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2020/07/16175820/video-content-bg-1-1024x576.jpg',
		platform: 'tradestation',
		url: '/learning-center/ts-workspaces'
	},
	{
		id: '103',
		title: 'Un-install of TS indicator',
		slug: 'uninstall-ts-indicator',
		excerpt: 'A quick video on how to remove a Tradestation indicator.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2020/07/16175820/video-content-bg-1-1024x576.jpg',
		platform: 'tradestation',
		url: '/learning-center/uninstall-ts-indicator'
	},
	{
		id: '104',
		title: 'Tradestation Indicator Install',
		slug: 'tradestation-indicator-install',
		excerpt: 'Walk through A to Z on how to get a Tradestation indicator installed.',
		thumbnail:
			'https://cdn.simplertrading.com/dev/wp-content/uploads/2020/07/16175820/video-content-bg-1-1024x576.jpg',
		platform: 'tradestation',
		url: '/learning-center/tradestation-indicator-install'
	}
];

export const load: PageServerLoad = async ({ url }) => {
	const page = parseInt(url.searchParams.get('page') || '1');
	const perPage = 10;

	// Paginate TOS tutorials
	const startIndex = (page - 1) * perPage;
	const endIndex = startIndex + perPage;
	const paginatedTosTutorials = tosTutorials.slice(startIndex, endIndex);

	const seo: SEOInput = {
		title: 'Platform Tutorials - ThinkorSwim & TradeStation',
		description: 'Tutorials, Tips and Platform Features for ThinkorSwim and TradeStation trading platforms. Step-by-step guides for traders of all levels.',
		canonical: '/tutorials',
		og: { type: 'website' }
	};

	return {
		tosTutorials: paginatedTosTutorials,
		tradestationTutorials,
		tosPagination: {
			page,
			totalPages: Math.ceil(tosTutorials.length / perPage)
		},
		seo
	} satisfies PageData;
};
