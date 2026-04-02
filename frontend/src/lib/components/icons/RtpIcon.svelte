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
							
	// Education & Courses
			
	// Charts & Trading
						
	// Trading Specific
							
	// Lists & Tools
				
	// Account & Settings
						
	// Support & Communication
							
	// Commerce
				
	// Status & Feedback
						
	// Actions
												
	// Media
				
	// Business
	
	// Additional Navigation & Layout
			
	// Time
	
	// ═══════════════════════════════════════════════════════════════════════════
	// ICON MAPPING - RTP names to Tabler components
	// Trading platform icons matching Simpler Trading aesthetic
	// ═══════════════════════════════════════════════════════════════════════════

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
		<Icon icon={IconAlertCircle} {size} {color} {stroke} />
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
