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

	// Icon registry - maps icon names to Phosphor iconify strings
	// This is the single source of truth for available icons
	const iconRegistry: Record<string, string> = {
		// Chart/Trading icons
		'chart-line': 'ph:chart-line-up',
		'trending-up': 'ph:trend-up',
		'chart-candle': 'ph:chart-line',
		'chart-pie': 'ph:chart-pie',
		'chart-bar': 'ph:chart-bar',
		'chart-dots': 'ph:chart-scatter',
		'report-analytics': 'ph:chart-line-up',

		// Action icons
		bell: 'ph:bell',
		rocket: 'ph:rocket-launch',
		activity: 'ph:activity',
		bolt: 'ph:lightning',
		flame: 'ph:fire',
		target: 'ph:target',

		// Finance icons
		wallet: 'ph:wallet',

		// Media icons
		'player-play': 'ph:play',
		video: 'ph:video-camera',
		live: 'ph:broadcast',

		// Education icons
		book: 'ph:book',
		school: 'ph:graduation-cap',
		certificate: 'ph:certificate',

		// Status/Badge icons
		star: 'ph:star',
		diamond: 'ph:diamond',
		crown: 'ph:crown',
		award: 'ph:trophy',

		// Social icons
		users: 'ph:users',
		'message-circle': 'ph:chat-circle',

		// Utility icons
		calendar: 'ph:calendar',
		clock: 'ph:clock',
		settings: 'ph:gear',
		help: 'ph:question',
		home: 'ph:house',
		user: 'ph:user'
	};

	// Get the icon name, fallback to chart-line if not found
	const iconName = $derived(iconRegistry[name || ''] || 'ph:chart-line-up');
</script>

<Icon icon={iconName} {size} class={className} />
