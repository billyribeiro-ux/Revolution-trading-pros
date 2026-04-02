<script lang="ts">
	/**
	 * DynamicIcon Component - ICT 11+ Principal Engineer Pattern
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Renders Tabler icons dynamically based on icon name from data.
	 * No hardcoding - icon names come from database/API.
	 *
	 * Usage: <DynamicIcon name="chart-line" size={24} />
	 *
	 * @version 1.0.0
	 */

	// Import all icons we might need - loaded lazily by Vite
																																
	// Type for Tabler icon components
	type IconComponent = typeof IconChartLine;

	interface Props {
		name: string | null | undefined;
		size?: number;
		class?: string;
	}

	let { name, size = 24, class: className = '' }: Props = $props();

	// Icon registry - maps icon names to components
	// This is the single source of truth for available icons
	const iconRegistry: Record<string, IconComponent> = {
		// Chart/Trading icons
		'chart-line': IconChartLine,
		'trending-up': IconTrendingUp,
		'chart-candle': IconChartCandle,
		'chart-pie': IconChartPie,
		'chart-bar': IconChartBar,
		'chart-dots': IconChartDots,
		'report-analytics': IconReportAnalytics,

		// Action icons
		bell: IconBell,
		rocket: IconRocket,
		activity: IconActivity,
		bolt: IconBolt,
		flame: IconFlame,
		target: IconTarget,

		// Finance icons
		wallet: IconWallet,

		// Media icons
		'player-play': IconPlayerPlay,
		video: IconVideo,
		live: IconLive,

		// Education icons
		book: IconBook,
		school: IconSchool,
		certificate: IconCertificate,

		// Status/Badge icons
		star: IconStar,
		diamond: IconDiamond,
		crown: IconCrown,
		award: IconAward,

		// Social icons
		users: IconUsers,
		'message-circle': IconMessageCircle,

		// Utility icons
		calendar: IconCalendar,
		clock: IconClock,
		settings: IconSettings,
		help: IconHelp,
		home: IconHome,
		user: IconUser
	};

	// Get the icon component, fallback to chart-line if not found
	const IconComponent = $derived(iconRegistry[name || ''] || IconChartLine);
</script>

<IconComponent {size} class={className} />
