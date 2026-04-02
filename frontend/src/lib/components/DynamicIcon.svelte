<script lang="ts">
	import { Icon } from '$lib/icons';

	/**
	 * DynamicIcon Component
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Renders icons dynamically based on icon name from data via @iconify/svelte.
	 * No hardcoding - icon names come from database/API.
	 *
	 * Usage: <DynamicIcon name="chart-line" size={24} />
	 *
	 * @version 2.0.0
	 */

	interface Props {
		name: string | null | undefined;
		size?: number;
		class?: string;
	}

	let { name, size = 24, class: className = '' }: Props = $props();

	// Icon registry - maps icon names to Tabler iconify strings
	// This is the single source of truth for available icons
	const iconRegistry: Record<string, string> = {
		// Chart/Trading icons
		'chart-line': 'tabler:chart-line',
		'trending-up': 'tabler:trending-up',
		'chart-candle': 'tabler:chart-candle',
		'chart-pie': 'tabler:chart-pie',
		'chart-bar': 'tabler:chart-bar',
		'chart-dots': 'tabler:chart-dots',
		'report-analytics': 'tabler:report-analytics',

		// Action icons
		bell: 'tabler:bell',
		rocket: 'tabler:rocket',
		activity: 'tabler:activity',
		bolt: 'tabler:bolt',
		flame: 'tabler:flame',
		target: 'tabler:target',

		// Finance icons
		wallet: 'tabler:wallet',

		// Media icons
		'player-play': 'tabler:player-play',
		video: 'tabler:video',
		live: 'tabler:live-photo',

		// Education icons
		book: 'tabler:book',
		school: 'tabler:school',
		certificate: 'tabler:certificate',

		// Status/Badge icons
		star: 'tabler:star',
		diamond: 'tabler:diamond',
		crown: 'tabler:crown',
		award: 'tabler:award',

		// Social icons
		users: 'tabler:users',
		'message-circle': 'tabler:message-circle',

		// Utility icons
		calendar: 'tabler:calendar',
		clock: 'tabler:clock',
		settings: 'tabler:settings',
		help: 'tabler:help',
		home: 'tabler:home',
		user: 'tabler:user'
	};

	// Get the icon name, fallback to chart-line if not found
	const iconName = $derived(iconRegistry[name || ''] || 'tabler:chart-line');
</script>

<Icon icon={iconName} {size} class={className} />
