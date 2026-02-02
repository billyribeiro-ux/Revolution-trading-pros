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
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconWallet from '@tabler/icons-svelte-runes/icons/wallet';
	import IconBell from '@tabler/icons-svelte-runes/icons/bell';
	import IconRocket from '@tabler/icons-svelte-runes/icons/rocket';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';
	import IconChartCandle from '@tabler/icons-svelte-runes/icons/chart-candle';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconBook from '@tabler/icons-svelte-runes/icons/book';
	import IconSchool from '@tabler/icons-svelte-runes/icons/school';
	import IconCertificate from '@tabler/icons-svelte-runes/icons/certificate';
	import IconReportAnalytics from '@tabler/icons-svelte-runes/icons/report-analytics';
	import IconChartPie from '@tabler/icons-svelte-runes/icons/chart-pie';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import IconChartDots from '@tabler/icons-svelte-runes/icons/chart-dots';
	import IconTarget from '@tabler/icons-svelte-runes/icons/target';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconFlame from '@tabler/icons-svelte-runes/icons/flame';
	import IconStar from '@tabler/icons-svelte-runes/icons/star';
	import IconDiamond from '@tabler/icons-svelte-runes/icons/diamond';
	import IconCrown from '@tabler/icons-svelte-runes/icons/crown';
	import IconAward from '@tabler/icons-svelte-runes/icons/award';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconMessageCircle from '@tabler/icons-svelte-runes/icons/message-circle';
	import IconVideo from '@tabler/icons-svelte-runes/icons/video';
	import IconLive from '@tabler/icons-svelte-runes/icons/live-photo';
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconSettings from '@tabler/icons-svelte-runes/icons/settings';
	import IconHelp from '@tabler/icons-svelte-runes/icons/help';
	import IconHome from '@tabler/icons-svelte-runes/icons/home';
	import IconUser from '@tabler/icons-svelte-runes/icons/user';

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
