<!--
	RoomIcon - Tabler icon for a trading room, keyed by room id.
	Replaces the emoji `Room.icon` field in admin UI.
-->
<script lang="ts">
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconChartCandle from '@tabler/icons-svelte-runes/icons/chart-candle';
	import IconTarget from '@tabler/icons-svelte-runes/icons/target';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconCoin from '@tabler/icons-svelte-runes/icons/coin';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';

	let { id, size = 18 }: { id: string; size?: number } = $props();

	const icons = {
		'day-trading-room': IconChartLine,
		'swing-trading-room': IconChartCandle,
		'small-account-mentorship': IconTarget,
		'explosive-swings': IconBolt,
		'spx-profit-pulse': IconCoin
	} as const;

	const Icon = $derived(icons[id as keyof typeof icons] ?? IconChartBar);
</script>

<Icon {size} aria-hidden="true" />
