/**
 * Tabler Icons - Proper Enterprise Export Barrel
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * CRITICAL: All icons MUST be imported from '@tabler/icons-svelte' main export.
 * DO NOT import from '@tabler/icons-svelte/icons/*' - Vite cannot resolve those
 * paths due to the package.json exports configuration.
 * 
 * This follows Google/Microsoft/Meta standards for package exports:
 * - Single source of truth (barrel export)
 * - Type-safe imports
 * - Works with all bundlers (Vite, Webpack, Rollup)
 * 
 * Usage:
 * import { IconMapPin, IconUser, IconMail } from '$lib/icons';
 */

// Re-export all icons from main package (the ONLY way that works with Vite)
export {
	// Core icons
	IconUser,
	IconShield,
	IconLock,
	IconMail,
	IconPhone,
	IconBuilding,
	IconUsers,
	IconCalendar,
	IconDevices,
	IconAlertTriangle,
	IconInfoCircle,
	IconChevronRight,
	IconEye,
	IconEyeOff,
	IconShieldCheck,
	IconUserCheck,
	IconCreditCard,
	IconChartBar,
	IconFileText,
	IconBell,
	IconWorld,
	IconClock,
	IconUpload,
	IconDownload,
	IconRefresh,
	IconQrcode,
	IconKey,
	IconActivity,
	IconCheckupList,
	IconSchool,
	IconStar,
	IconStarFilled,
	IconSettings,
	IconX,
	IconCheck,
	IconPlus,
	IconMinus,
	IconEdit,
	IconTrash,
	IconSearch,
	IconFilter,
	IconSortAscending,
	IconSortDescending,
	IconMenu2,
	IconAlertCircle,
	IconPlayerPlay,
	IconPlayerPause,
	IconVolume,
	IconVolumeOff,
	IconBolt,
	IconGift,
	IconBrandGoogle,
	IconBrandFacebook,
	IconSparkles,
	IconList,
	IconTrendingUp,
	IconExternalLink,
	IconCopy,
	IconTicket,
	IconPercentage,
	IconCurrencyDollar,
	IconShoppingCart,
	IconTag,
	IconRobot,
	// Additional icons
	IconMapPin,
	IconId,
	IconFingerprint,
	IconBriefcase,
	IconBuildingStore,
	IconTestPipe,
	IconLayoutGrid,
	IconSquareRounded,
	IconSquareRoundedCheckFilled,
	IconMaximize,
	IconMinimize,
	IconTextCaption,
	IconPictureInPictureOn,
	IconShare,
	IconBookmark
} from '@tabler/icons-svelte';

/**
 * Type-safe icon component type
 * Use this for props that accept icon components
 */
export type IconComponent = typeof IconMapPin;
