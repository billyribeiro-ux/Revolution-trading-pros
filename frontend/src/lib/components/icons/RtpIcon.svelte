<!--
	RtpIcon - Revolution Trading Pros Icon System
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Custom icon wrapper using Tabler Icons with RTP naming convention.
	All dashboard icons use "rtp-icon-*" class names for consistency.

	Svelte 5 Features:
	- $props() for type-safe props
	- $derived() for computed icon lookup

	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	// Core icons for dashboard - using Tabler Icons
	import IconDashboard from '@tabler/icons-svelte-runes/icons/dashboard';
	import IconMenu2 from '@tabler/icons-svelte-runes/icons/menu-2';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import IconChevronRight from '@tabler/icons-svelte-runes/icons/chevron-right';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import IconExternalLink from '@tabler/icons-svelte-runes/icons/external-link';

	// Education & Courses
	import IconVideo from '@tabler/icons-svelte-runes/icons/video';
	import IconBook from '@tabler/icons-svelte-runes/icons/book';
	import IconSchool from '@tabler/icons-svelte-runes/icons/school';

	// Charts & Trading
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import IconChartCandle from '@tabler/icons-svelte-runes/icons/chart-candle';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconTrendingDown from '@tabler/icons-svelte-runes/icons/trending-down';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';

	// Trading Specific
	import IconTarget from '@tabler/icons-svelte-runes/icons/target';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconFlame from '@tabler/icons-svelte-runes/icons/flame';
	import IconRocket from '@tabler/icons-svelte-runes/icons/rocket';
	import IconCrown from '@tabler/icons-svelte-runes/icons/crown';
	import IconTrophy from '@tabler/icons-svelte-runes/icons/trophy';
	import IconAward from '@tabler/icons-svelte-runes/icons/award';

	// Lists & Tools
	import IconListCheck from '@tabler/icons-svelte-runes/icons/list-check';
	import IconClipboard from '@tabler/icons-svelte-runes/icons/clipboard';
	import IconFileText from '@tabler/icons-svelte-runes/icons/file-text';
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';

	// Account & Settings
	import IconUser from '@tabler/icons-svelte-runes/icons/user';
	import IconUserCircle from '@tabler/icons-svelte-runes/icons/user-circle';
	import IconSettings from '@tabler/icons-svelte-runes/icons/settings';
	import IconLogout from '@tabler/icons-svelte-runes/icons/logout';
	import IconLock from '@tabler/icons-svelte-runes/icons/lock';
	import IconShield from '@tabler/icons-svelte-runes/icons/shield';

	// Support & Communication
	import IconHeadset from '@tabler/icons-svelte-runes/icons/headset';
	import IconHelp from '@tabler/icons-svelte-runes/icons/help';
	import IconMessage from '@tabler/icons-svelte-runes/icons/message';
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconPhone from '@tabler/icons-svelte-runes/icons/phone';
	import IconBell from '@tabler/icons-svelte-runes/icons/bell';
	import IconMapPin from '@tabler/icons-svelte-runes/icons/map-pin';

	// Commerce
	import IconShoppingCart from '@tabler/icons-svelte-runes/icons/shopping-cart';
	import IconCreditCard from '@tabler/icons-svelte-runes/icons/credit-card';
	import IconCoin from '@tabler/icons-svelte-runes/icons/coin';
	import IconReceipt from '@tabler/icons-svelte-runes/icons/receipt';

	// Status & Feedback
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import IconInfoCircle from '@tabler/icons-svelte-runes/icons/info-circle';
	import IconHeart from '@tabler/icons-svelte-runes/icons/heart';
	import IconStar from '@tabler/icons-svelte-runes/icons/star';

	// Actions
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import IconFilter from '@tabler/icons-svelte-runes/icons/filter';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconMinus from '@tabler/icons-svelte-runes/icons/minus';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconCopy from '@tabler/icons-svelte-runes/icons/copy';
	import IconEye from '@tabler/icons-svelte-runes/icons/eye';
	import IconEyeOff from '@tabler/icons-svelte-runes/icons/eye-off';

	// Media
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconPlayerPause from '@tabler/icons-svelte-runes/icons/player-pause';
	import IconVolume from '@tabler/icons-svelte-runes/icons/volume';
	import IconVolumeOff from '@tabler/icons-svelte-runes/icons/volume-off';

	// Business
	import IconBuilding from '@tabler/icons-svelte-runes/icons/building';

	// Additional Navigation & Layout
	import IconLayoutDashboard from '@tabler/icons-svelte-runes/icons/layout-dashboard';
	import IconArchive from '@tabler/icons-svelte-runes/icons/archive';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';

	// Time
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';

	// ═══════════════════════════════════════════════════════════════════════════
	// ICON MAPPING - RTP names to Tabler components
	// Trading platform icons matching Simpler Trading aesthetic
	// ═══════════════════════════════════════════════════════════════════════════

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const iconMap: Record<string, any> = {
		// Navigation
		home: IconDashboard,
		dashboard: IconDashboard,
		menu: IconMenu2,
		close: IconX,
		'chevron-left': IconChevronLeft,
		'chevron-right': IconChevronRight,
		'chevron-down': IconChevronDown,
		external: IconExternalLink,
		'layout-dashboard': IconLayoutDashboard,
		archive: IconArchive,
		users: IconUsers,
		'shopping-cart': IconShoppingCart,

		// Education & Courses
		courses: IconVideo,
		classes: IconVideo,
		book: IconBook,
		school: IconSchool,
		video: IconVideo,

		// Charts & Indicators
		indicators: IconChartLine,
		charts: IconChartCandle,
		'chart-line': IconChartLine,
		'chart-bar': IconChartBar,
		'chart-candle': IconChartCandle,
		activity: IconActivity,

		// Memberships (Trading Products)
		'mastering-the-trade': IconTrendingUp,
		'simpler-showcase': IconTarget,
		'tr3ndy-spx-alerts': IconBolt,
		'tr3ndy-spx-alerts-circle': IconBolt,
		'consistent-growth': IconChartBar,
		'compounding-growth': IconChartBar,

		// Trading Icons
		'trending-up': IconTrendingUp,
		'trending-down': IconTrendingDown,
		target: IconTarget,
		bolt: IconBolt,
		lightning: IconBolt,
		flame: IconFlame,
		rocket: IconRocket,
		crown: IconCrown,
		trophy: IconTrophy,
		award: IconAward,

		// Tools & Lists
		'weekly-watchlist': IconListCheck,
		watchlist: IconListCheck,
		list: IconClipboard,
		checklist: IconListCheck,
		file: IconFileText,
		calendar: IconCalendar,

		// Account & Settings
		settings: IconSettings,
		account: IconUserCircle,
		user: IconUser,
		profile: IconUserCircle,
		logout: IconLogout,
		lock: IconLock,
		shield: IconShield,

		// Support & Communication
		support: IconHeadset,
		help: IconHelp,
		message: IconMessage,
		mail: IconMail,
		phone: IconPhone,
		bell: IconBell,
		notifications: IconBell,
		location: IconMapPin,

		// Commerce
		cart: IconShoppingCart,
		payment: IconCreditCard,
		coin: IconCoin,
		receipt: IconReceipt,

		// Status & Feedback
		check: IconCircleCheck,
		success: IconCircleCheck,
		alert: IconAlertCircle,
		warning: IconAlertTriangle,
		info: IconInfoCircle,
		heart: IconHeart,
		star: IconStar,

		// Actions
		search: IconSearch,
		filter: IconFilter,
		refresh: IconRefresh,
		download: IconDownload,
		upload: IconUpload,
		plus: IconPlus,
		minus: IconMinus,
		edit: IconEdit,
		delete: IconTrash,
		copy: IconCopy,
		eye: IconEye,
		'eye-off': IconEyeOff,

		// Media
		play: IconPlayerPlay,
		pause: IconPlayerPause,
		volume: IconVolume,
		mute: IconVolumeOff,

		// Business
		building: IconBuilding,
		store: IconBuilding,

		// Time
		clock: IconClock
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		/** Icon name from RTP icon system */
		name: string;
		/** Icon size in pixels (default: 24) */
		size?: number;
		/** Icon color (default: currentColor) */
		color?: string;
		/** Stroke width (default: 2) */
		stroke?: number;
		/** Additional CSS class */
		class?: string;
		/** Accessible label for screen readers */
		ariaLabel?: string;
	}

	let {
		name,
		size = 24,
		color = 'currentColor',
		stroke = 2,
		class: className = '',
		ariaLabel
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	// Get the icon component, fallback to alert if not found
	const IconComponent = $derived(iconMap[name] ?? IconAlertCircle);
	const isValidIcon = $derived(name in iconMap);
</script>

{#if !isValidIcon}
	<!-- Development warning for unknown icons -->
	<span
		class="rtp-icon rtp-icon--unknown {className}"
		title="Unknown icon: {name}"
		role="img"
		aria-label={ariaLabel ?? `Unknown icon: ${name}`}
	>
		<IconAlertCircle {size} {color} {stroke} />
	</span>
{:else}
	<span
		class="rtp-icon rtp-icon--{name} {className}"
		role="img"
		aria-label={ariaLabel ?? name}
		aria-hidden={!ariaLabel}
	>
		<IconComponent {size} {color} {stroke} />
	</span>
{/if}

<style>
	.rtp-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		line-height: 1;
	}

	.rtp-icon--unknown {
		color: #dc3545;
	}

	/* Allow color inheritance */
	.rtp-icon :global(svg) {
		display: block;
	}
</style>
