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
	// ICON MAPPING - RTP names to Phosphor iconify strings
	// Trading platform icons matching Simpler Trading aesthetic
	// ═══════════════════════════════════════════════════════════════════════════

	const iconMap: Record<string, string> = {
		// Navigation
		home: 'ph:gauge',
		dashboard: 'ph:gauge',
		menu: 'ph:list',
		close: 'ph:x',
		'chevron-left': 'ph:caret-left',
		'chevron-right': 'ph:caret-right',
		'chevron-down': 'ph:caret-down',
		external: 'ph:arrow-square-out',
		'layout-dashboard': 'ph:squares-four',
		archive: 'ph:archive-box',
		users: 'ph:users',
		'shopping-cart': 'ph:shopping-cart',

		// Education & Courses
		courses: 'ph:video-camera',
		classes: 'ph:video-camera',
		book: 'ph:book',
		school: 'ph:graduation-cap',
		video: 'ph:video-camera',

		// Charts & Indicators
		indicators: 'ph:chart-line-up',
		charts: 'ph:chart-line',
		'chart-line': 'ph:chart-line-up',
		'chart-bar': 'ph:chart-bar',
		'chart-candle': 'ph:chart-line',
		activity: 'ph:activity',

		// Memberships (Trading Products)
		'mastering-the-trade': 'ph:trend-up',
		'simpler-showcase': 'ph:target',
		'tr3ndy-spx-alerts': 'ph:lightning',
		'tr3ndy-spx-alerts-circle': 'ph:lightning',
		'consistent-growth': 'ph:chart-bar',
		'compounding-growth': 'ph:chart-bar',

		// Trading Icons
		'trending-up': 'ph:trend-up',
		'trending-down': 'ph:trend-down',
		target: 'ph:target',
		bolt: 'ph:lightning',
		lightning: 'ph:lightning',
		flame: 'ph:fire',
		rocket: 'ph:rocket-launch',
		crown: 'ph:crown',
		trophy: 'ph:trophy',
		award: 'ph:trophy',

		// Tools & Lists
		'weekly-watchlist': 'ph:list-checks',
		watchlist: 'ph:list-checks',
		list: 'ph:clipboard',
		checklist: 'ph:list-checks',
		file: 'ph:file-text',
		calendar: 'ph:calendar',

		// Account & Settings
		settings: 'ph:gear',
		account: 'ph:user-circle',
		user: 'ph:user',
		profile: 'ph:user-circle',
		logout: 'ph:sign-out',
		lock: 'ph:lock',
		shield: 'ph:shield',

		// Support & Communication
		support: 'ph:headset',
		help: 'ph:question',
		message: 'ph:chat',
		mail: 'ph:envelope',
		phone: 'ph:phone',
		bell: 'ph:bell',
		notifications: 'ph:bell',
		location: 'ph:map-pin',

		// Commerce
		cart: 'ph:shopping-cart',
		payment: 'ph:credit-card',
		coin: 'ph:coin',
		receipt: 'ph:receipt',

		// Status & Feedback
		check: 'ph:check-circle',
		success: 'ph:check-circle',
		alert: 'ph:warning-circle',
		warning: 'ph:warning',
		info: 'ph:info',
		heart: 'ph:heart',
		star: 'ph:star',

		// Actions
		search: 'ph:magnifying-glass',
		filter: 'ph:funnel',
		refresh: 'ph:arrow-counter-clockwise',
		download: 'ph:download-simple',
		upload: 'ph:upload-simple',
		plus: 'ph:plus',
		minus: 'ph:minus',
		edit: 'ph:pencil-simple',
		delete: 'ph:trash',
		copy: 'ph:copy',
		eye: 'ph:eye',
		'eye-off': 'ph:eye-slash',

		// Media
		play: 'ph:play',
		pause: 'ph:pause',
		volume: 'ph:speaker-high',
		mute: 'ph:speaker-slash',

		// Business
		building: 'ph:building',
		store: 'ph:building',

		// Time
		clock: 'ph:clock'
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

	const iconName = $derived(iconMap[name] ?? 'ph:warning-circle');
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
		<Icon icon="ph:warning-circle" {size} {color} />
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
