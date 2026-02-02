// Type declarations for SvelteKit modules and external icon library
// This file silences TypeScript errors when the IDE cannot locate module typings.

declare module '$app/navigation' {
	/** Navigate to a new URL */
	export function goto(
		href: string,
		opts?: { replaceState?: boolean; noscroll?: boolean; keepfocus?: boolean; state?: any }
	): Promise<void>;
}

declare module '$app/stores' {
	import type { Readable } from 'svelte/store';
	export const page: Readable<any>;
	export const navigating: Readable<any>;
	export const session: Readable<any>;
	export const updated: Readable<any>;
}

declare module '@tabler/icons-svelte-runes' {
	// Export all icons as Svelte components with a generic props interface
	export const IconEdit: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconTrash: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconEye: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSave: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconX: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconMail: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSearch: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconPlus: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowLeft: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconLoader: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCheck: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconChevronLeft: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconChevronRight: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconLogout: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconUser: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSettings: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconMenu2: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconChartLine: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBolt: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBellRinging: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconShoppingCart: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconTrendingUp: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconMoon: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCalendar: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCircleCheck: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconAlertCircle: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSeedling: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconRocket: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconTarget: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconShieldCheck: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconLock: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandGoogle: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandFacebook: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandTwitter: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandGithub: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconGift: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconUserPlus: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSparkles: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowRight: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconInbox: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconChartCandle: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconWorld: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconUsers: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconStar: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconClock: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconVideo: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSchool: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconTrophy: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBulb: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconHeart: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconLogin: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconDeviceAnalytics: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowUpRight: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowDownRight: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconRefresh: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconDownload: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconFilter: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowNarrowRight: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowNarrowLeft: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandLinkedin: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandInstagram: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandYoutube: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconMenu: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconChevronDown: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconQuote: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandTelegram: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandDiscord: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandReddit: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandTiktok: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandMedium: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandX: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowRight: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowLeft: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCheck: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconX: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconChevronRight: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconChevronLeft: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSearch: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconUser: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconLock: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconShieldCheck: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconAlertCircle: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconLoader: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconActivity: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconAlertTriangle: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconTrendingDown: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandFacebook: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandTwitter: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandGithub: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrandGoogle: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconDashboard: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconForms: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSeo: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconLink: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconError404: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBold: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconItalic: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconUnderline: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconStrikethrough: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconAlignLeft: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconAlignCenter: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconAlignRight: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconAlignJustified: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconList: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconListNumbers: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconPhoto: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconTable: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCode: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconH1: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconH2: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconH3: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowBackUp: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowForwardUp: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconDeviceFloppy: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCircleX: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconFileText: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconRobot: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconAward: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconReceipt: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconTicket: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconNews: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCurrencyDollar: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCopy: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconTag: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconFolder: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBook: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCrown: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconEyeOff: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconChartBar: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconClick: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconDevices: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconArrowForward: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconExternalLink: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconKey: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSitemap: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconMinus: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconToggleLeft: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconToggleRight: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconUpload: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconPlugConnected: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconPlugConnectedX: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCreditCard: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCertificate: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBrain: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconShield: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconHeadset: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCalendarEvent: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBell: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconFlame: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSend: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconWaveSine: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconInfoCircle: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconUserCircle: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconPhone: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconGripVertical: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconPercentage: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconPlayerPlay: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconPlayerPause: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconVolume: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconVolumeOff: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconCheckupList: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconQrcode: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconUserCheck: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconBuilding: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconQuestionMark: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconPencil: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconStarFilled: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSortAscending: typeof import('svelte').SvelteComponentTyped<any>;
	export const IconSortDescending: typeof import('svelte').SvelteComponentTyped<any>;
	// Add any other icons you use as needed
}

// Note: We don't need to declare 'svelte' module as it's provided by the package itself
// The svelte/internal import is deprecated in Svelte 5 and will be removed in Svelte 6
