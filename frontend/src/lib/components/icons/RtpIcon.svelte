<!--
	RtpIcon - Revolution Trading Pros Icon System
	═══════════════════════════════════════════════════════════════════════════
	Custom icon wrapper using @iconify/svelte with RTP naming convention.
	All dashboard icons use "rtp-icon-*" class names for consistency.

	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { Icon } from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// ICON MAPPING - RTP names to Tabler iconify strings
	// Trading platform icons matching Simpler Trading aesthetic
	// ═══════════════════════════════════════════════════════════════════════════

	const iconMap: Record<string, string> = {
		// Navigation
		home: 'tabler:dashboard',
		dashboard: 'tabler:dashboard',
		menu: 'tabler:menu-2',
		close: 'tabler:x',
		'chevron-left': 'tabler:chevron-left',
		'chevron-right': 'tabler:chevron-right',
		'chevron-down': 'tabler:chevron-down',
		external: 'tabler:external-link',
		'layout-dashboard': 'tabler:layout-dashboard',
		archive: 'tabler:archive',
		users: 'tabler:users',
		'shopping-cart': 'tabler:shopping-cart',

		// Education & Courses
		courses: 'tabler:video',
		classes: 'tabler:video',
		book: 'tabler:book',
		school: 'tabler:school',
		video: 'tabler:video',

		// Charts & Indicators
		indicators: 'tabler:chart-line',
		charts: 'tabler:chart-candle',
		'chart-line': 'tabler:chart-line',
		'chart-bar': 'tabler:chart-bar',
		'chart-candle': 'tabler:chart-candle',
		activity: 'tabler:activity',

		// Memberships (Trading Products)
		'mastering-the-trade': 'tabler:trending-up',
		'simpler-showcase': 'tabler:target',
		'tr3ndy-spx-alerts': 'tabler:bolt',
		'tr3ndy-spx-alerts-circle': 'tabler:bolt',
		'consistent-growth': 'tabler:chart-bar',
		'compounding-growth': 'tabler:chart-bar',

		// Trading Icons
		'trending-up': 'tabler:trending-up',
		'trending-down': 'tabler:trending-down',
		target: 'tabler:target',
		bolt: 'tabler:bolt',
		lightning: 'tabler:bolt',
		flame: 'tabler:flame',
		rocket: 'tabler:rocket',
		crown: 'tabler:crown',
		trophy: 'tabler:trophy',
		award: 'tabler:award',

		// Tools & Lists
		'weekly-watchlist': 'tabler:list-check',
		watchlist: 'tabler:list-check',
		list: 'tabler:clipboard',
		checklist: 'tabler:list-check',
		file: 'tabler:file-text',
		calendar: 'tabler:calendar',

		// Account & Settings
		settings: 'tabler:settings',
		account: 'tabler:user-circle',
		user: 'tabler:user',
		profile: 'tabler:user-circle',
		logout: 'tabler:logout',
		lock: 'tabler:lock',
		shield: 'tabler:shield',

		// Support & Communication
		support: 'tabler:headset',
		help: 'tabler:help',
		message: 'tabler:message',
		mail: 'tabler:mail',
		phone: 'tabler:phone',
		bell: 'tabler:bell',
		notifications: 'tabler:bell',
		location: 'tabler:map-pin',

		// Commerce
		cart: 'tabler:shopping-cart',
		payment: 'tabler:credit-card',
		coin: 'tabler:coin',
		receipt: 'tabler:receipt',

		// Status & Feedback
		check: 'tabler:circle-check',
		success: 'tabler:circle-check',
		alert: 'tabler:alert-circle',
		warning: 'tabler:alert-triangle',
		info: 'tabler:info-circle',
		heart: 'tabler:heart',
		star: 'tabler:star',

		// Actions
		search: 'tabler:search',
		filter: 'tabler:filter',
		refresh: 'tabler:refresh',
		download: 'tabler:download',
		upload: 'tabler:upload',
		plus: 'tabler:plus',
		minus: 'tabler:minus',
		edit: 'tabler:edit',
		delete: 'tabler:trash',
		copy: 'tabler:copy',
		eye: 'tabler:eye',
		'eye-off': 'tabler:eye-off',

		// Media
		play: 'tabler:player-play',
		pause: 'tabler:player-pause',
		volume: 'tabler:volume',
		mute: 'tabler:volume-off',

		// Business
		building: 'tabler:building',
		store: 'tabler:building',

		// Time
		clock: 'tabler:clock'
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
		stroke: _stroke = 2,
		class: className = '',
		ariaLabel
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	const iconName = $derived(iconMap[name] ?? 'tabler:alert-circle');
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
		<Icon icon="tabler:alert-circle" {size} {color} />
	</span>
{:else}
	<span
		class="rtp-icon rtp-icon--{name} {className}"
		role="img"
		aria-label={ariaLabel ?? name}
		aria-hidden={!ariaLabel}
	>
		<Icon icon={iconName} {size} {color} />
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
