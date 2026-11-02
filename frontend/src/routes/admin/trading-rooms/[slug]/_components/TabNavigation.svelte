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
		border-bottom: 2px solid var(--border-default, #30363d);
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
		color: var(--text-tertiary, #6e7681);
		cursor: pointer;
		border-bottom: 3px solid transparent;
		margin-bottom: -2px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: var(--text-primary, #f0f6fc);
	}

	.tab.active {
		color: var(--primary-500, #e6b800);
		border-bottom-color: var(--primary-500, #e6b800);
	}

	.tab .badge {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--text-tertiary, #6e7681);
		padding: 2px 8px;
		border-radius: 10px;
		font-size: 12px;
	}

	.tab.active .badge {
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
	}

	.tab .badge.active {
		background: var(--success-base, #2ea043);
		color: #fff;
	}
</style>
