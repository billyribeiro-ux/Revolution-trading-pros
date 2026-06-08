<!--
/**
 * DamTabs - Library / Upload tab switcher
 * Extracted from AssetManager.svelte (R6-C). One bindable prop (activeTab).
 */
-->
<script lang="ts">
	interface Props {
		activeTab: 'library' | 'upload';
		uploadQueueCount: number;
	}

	let { activeTab = $bindable('library'), uploadQueueCount }: Props = $props();
</script>

<div class="dam-tabs">
	<button
		class={['tab', { active: activeTab === 'library' }]}
		onclick={() => (activeTab = 'library')}
	>
		Library
	</button>
	<button
		class={['tab', { active: activeTab === 'upload' }]}
		onclick={() => (activeTab = 'upload')}
	>
		Upload
		{#if uploadQueueCount > 0}
			<span class="badge">{uploadQueueCount}</span>
		{/if}
	</button>
</div>

<style>
	.dam-tabs {
		display: flex;
		padding: 0 1.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.02);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.25rem;
		background: none;
		border: none;
		font-size: 0.875rem;
		font-weight: 500;
		color: #64748b;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #94a3b8;
	}

	.tab.active {
		color: #3b82f6;
		border-bottom-color: #3b82f6;
	}

	.badge {
		background: #3b82f6;
		color: white;
		padding: 0.125rem 0.5rem;
		border-radius: 999px;
		font-size: 0.75rem;
		font-weight: 600;
	}
</style>
