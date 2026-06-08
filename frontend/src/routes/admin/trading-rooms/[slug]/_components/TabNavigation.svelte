<!--
	TabNavigation — 5-tab admin selector for the trading-room content page.
	Extracted from +page.svelte (R8-C) — 1 $bindable enum, 0 callback props.
-->
<script lang="ts">
	import IconTable from '@tabler/icons-svelte-runes/icons/table';
	import IconBell from '@tabler/icons-svelte-runes/icons/bell';
	import IconVideo from '@tabler/icons-svelte-runes/icons/video';
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';

	export type Tab = 'trade-plan' | 'alerts' | 'weekly-video' | 'trades' | 'video-library';

	interface Props {
		activeTab: Tab;
		tradePlanCount: number;
		alertsCount: number;
		hasCurrentVideo: boolean;
		tradesCount: number;
		activeTradesCount: number;
		videosCount: number;
	}

	let {
		activeTab = $bindable(),
		tradePlanCount,
		alertsCount,
		hasCurrentVideo,
		tradesCount,
		activeTradesCount,
		videosCount
	}: Props = $props();
</script>

<nav class="tabs">
	<button
		class={['tab', { active: activeTab === 'trade-plan' }]}
		onclick={() => (activeTab = 'trade-plan')}
	>
		<IconTable size={20} />
		<span>Trade Plan</span>
		<span class="badge">{tradePlanCount}</span>
	</button>
	<button
		class={['tab', { active: activeTab === 'alerts' }]}
		onclick={() => (activeTab = 'alerts')}
	>
		<IconBell size={20} />
		<span>Alerts</span>
		<span class="badge">{alertsCount}</span>
	</button>
	<button
		class={['tab', { active: activeTab === 'weekly-video' }]}
		onclick={() => (activeTab = 'weekly-video')}
	>
		<IconVideo size={20} />
		<span>Weekly Video</span>
		{#if hasCurrentVideo}
			<span class="badge active">1</span>
		{/if}
	</button>
	<button
		class={['tab', { active: activeTab === 'trades' }]}
		onclick={() => (activeTab = 'trades')}
	>
		<IconChartLine size={20} />
		<span>Trade Tracker</span>
		{#if activeTradesCount > 0}
			<span class="badge active">{activeTradesCount}</span>
		{:else}
			<span class="badge">{tradesCount}</span>
		{/if}
	</button>
	<button
		class={['tab', { active: activeTab === 'video-library' }]}
		onclick={() => (activeTab = 'video-library')}
	>
		<IconPlayerPlay size={20} />
		<span>Video Library</span>
		<span class="badge">{videosCount}</span>
	</button>
</nav>

<style>
	/* Tabs */
	.tabs {
		display: flex;
		gap: 8px;
		border-bottom: 2px solid #e2e8f0;
		margin-bottom: 32px;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 14px 24px;
		background: none;
		border: none;
		font-size: 15px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		border-bottom: 3px solid transparent;
		margin-bottom: -2px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #1e293b;
	}

	.tab.active {
		color: #143e59;
		border-bottom-color: #143e59;
	}

	.tab .badge {
		background: #e2e8f0;
		color: #64748b;
		padding: 2px 8px;
		border-radius: 10px;
		font-size: 12px;
	}

	.tab.active .badge {
		background: #143e59;
		color: #fff;
	}

	.tab .badge.active {
		background: #22c55e;
		color: #fff;
	}
</style>
