// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

// Augment @tabler/icons-svelte to fix type resolution issues
declare module '@tabler/icons-svelte' {
	import type { SvelteComponent } from 'svelte';

	type IconProps = {
		size?: number | string;
		stroke?: number | string;
		color?: string;
		class?: string;
	};

	type IconComponent = typeof SvelteComponent<IconProps>;

	// Core icons
	export const IconUser: IconComponent;
	export const IconShield: IconComponent;
	export const IconLock: IconComponent;
	export const IconMail: IconComponent;
	export const IconPhone: IconComponent;
	export const IconBuilding: IconComponent;
	export const IconUsers: IconComponent;
	export const IconCalendar: IconComponent;
	export const IconDevices: IconComponent;
	export const IconAlertTriangle: IconComponent;
	export const IconInfoCircle: IconComponent;
	export const IconChevronRight: IconComponent;
	export const IconEye: IconComponent;
	export const IconEyeOff: IconComponent;
	export const IconShieldCheck: IconComponent;
	export const IconUserCheck: IconComponent;
	export const IconCreditCard: IconComponent;
	export const IconChartBar: IconComponent;
	export const IconFileText: IconComponent;
	export const IconBell: IconComponent;
	export const IconWorld: IconComponent;
	export const IconClock: IconComponent;
	export const IconUpload: IconComponent;
	export const IconDownload: IconComponent;
	export const IconRefresh: IconComponent;
	export const IconQrcode: IconComponent;
	export const IconKey: IconComponent;
	export const IconActivity: IconComponent;
	export const IconCheckupList: IconComponent;
	export const IconSchool: IconComponent;
	export const IconStar: IconComponent;
	export const IconStarFilled: IconComponent;
	export const IconSettings: IconComponent;
	export const IconX: IconComponent;
	export const IconCheck: IconComponent;
	export const IconPlus: IconComponent;
	export const IconMinus: IconComponent;
	export const IconEdit: IconComponent;
	export const IconTrash: IconComponent;
	export const IconSearch: IconComponent;
	export const IconFilter: IconComponent;
	export const IconSortAscending: IconComponent;
	export const IconSortDescending: IconComponent;
	export const IconMenu2: IconComponent;
	export const IconAlertCircle: IconComponent;
	export const IconPlayerPlay: IconComponent;
	export const IconPlayerPause: IconComponent;
	export const IconVolume: IconComponent;
	export const IconVolumeOff: IconComponent;
	export const IconBolt: IconComponent;
	export const IconGift: IconComponent;
	export const IconBrandGoogle: IconComponent;
	export const IconBrandFacebook: IconComponent;
	export const IconSparkles: IconComponent;
	export const IconList: IconComponent;
	export const IconTrendingUp: IconComponent;
	export const IconExternalLink: IconComponent;
	export const IconCopy: IconComponent;
	export const IconTicket: IconComponent;
	export const IconPercentage: IconComponent;
	export const IconCurrencyDollar: IconComponent;
	export const IconShoppingCart: IconComponent;
	export const IconTag: IconComponent;
	export const IconRobot: IconComponent;

	// Additional icons that were causing resolution issues
	export const IconMapPin: IconComponent;
	export const IconId: IconComponent;
	export const IconFingerprint: IconComponent;
	export const IconBriefcase: IconComponent;
	export const IconBuildingStore: IconComponent;
	export const IconTestPipe: IconComponent;
	export const IconLayoutGrid: IconComponent;
	export const IconSquareRounded: IconComponent;
	export const IconSquareRoundedCheckFilled: IconComponent;
	export const IconMaximize: IconComponent;
	export const IconMinimize: IconComponent;
	export const IconTextCaption: IconComponent;
	export const IconPictureInPictureOn: IconComponent;
	export const IconShare: IconComponent;
	export const IconBookmark: IconComponent;
}

/**
 * Consent State interface for server-side access.
 */
export interface ConsentState {
	necessary: boolean;
	analytics: boolean;
	marketing: boolean;
	preferences: boolean;
	updatedAt: string;
	hasInteracted: boolean;
	version: number;
}

/**
 * Meta Pixel (fbq) function type.
 */
interface Fbq {
	(method: 'init', pixelId: string): void;
	(method: 'track', eventName: string, params?: Record<string, unknown>): void;
	(method: 'trackCustom', eventName: string, params?: Record<string, unknown>): void;
	(method: 'consent', action: 'grant' | 'revoke'): void;
	(method: 'dataProcessingOptions', options: string[], country?: number, state?: number): void;
	callMethod?: (...args: unknown[]) => void;
	queue: unknown[];
	loaded: boolean;
	version: string;
	push: (...args: unknown[]) => void;
}

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			/** User's consent state from cookie */
			consent: ConsentState;
			/** Whether the user has interacted with the consent banner */
			hasConsentInteraction: boolean;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	interface Window {
		/** YouTube IFrame API */
		YT?: {
			Player: any;
			PlayerState: {
				UNSTARTED: number;
				ENDED: number;
				PLAYING: number;
				PAUSED: number;
				BUFFERING: number;
				CUED: number;
			};
			ready: (callback: () => void) => void;
		};
		onYouTubeIframeAPIReady?: () => void;

		/** Google Tag (gtag.js) */
		dataLayer?: unknown[];
		gtag?: (...args: unknown[]) => void;

		/** Meta Pixel (fbq) */
		fbq?: Fbq;
		_fbq?: Fbq;

		/** Payment Providers */
		Stripe?: any;
		Square?: any;
		Paddle?: any;
		paypal?: any;
		Razorpay?: any;
		PaystackPop?: any;
	}
}

export {};

declare global {
	namespace svelteHTML {
		type HTMLAttributes<T> = any;
		type SVGAttributes<T> = any;
	}
}
