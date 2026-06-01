<script lang="ts">
	/**
	 * R19-C extraction (2026-05-20): course editor tab nav. Receives
	 * activeTab as $bindable so the parent's existing reactive switch keeps
	 * working without an event-bus dance.
	 */
	export type TabKey = 'details' | 'content' | 'downloads' | 'settings';

	interface Props {
		activeTab: TabKey;
	}

	let { activeTab = $bindable() }: Props = $props();
</script>

<nav class="tabs">
	<button class:active={activeTab === 'details'} onclick={() => (activeTab = 'details')}
		>Details</button
	>
	<button class:active={activeTab === 'content'} onclick={() => (activeTab = 'content')}
		>Content</button
	>
	<button class:active={activeTab === 'downloads'} onclick={() => (activeTab = 'downloads')}
		>Downloads</button
	>
	<button class:active={activeTab === 'settings'} onclick={() => (activeTab = 'settings')}
		>Settings</button
	>
</nav>

<style>
	.tabs {
		display: flex;
		gap: 4px;
		margin-bottom: 24px;
		border-bottom: 1px solid #e5e7eb;
	}
	.tabs button {
		padding: 12px 20px;
		background: none;
		border: none;
		color: #6b7280;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
	}
	.tabs button:hover {
		color: #1f2937;
	}
	.tabs button.active {
		color: #143e59;
		border-bottom-color: #143e59;
	}

	@media (max-width: 767.98px) {
		.tabs {
			overflow-x: auto;
		}
	}
</style>
