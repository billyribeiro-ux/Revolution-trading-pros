<script lang="ts">
	type TabKey = 'details' | 'files' | 'videos' | 'seo';

	interface Props {
		activeTab: TabKey;
		filesCount: number;
		videosCount: number;
		onSelect: (tab: TabKey) => void;
	}

	let { activeTab, filesCount, videosCount, onSelect }: Props = $props();
</script>

<nav class="tabs">
	<button class={{ active: activeTab === 'details' }} onclick={() => onSelect('details')}>
		Details
	</button>
	<button class={{ active: activeTab === 'files' }} onclick={() => onSelect('files')}>
		Files <span class="badge">{filesCount}</span>
	</button>
	<button class={{ active: activeTab === 'videos' }} onclick={() => onSelect('videos')}>
		Videos <span class="badge">{videosCount}</span>
	</button>
	<button class={{ active: activeTab === 'seo' }} onclick={() => onSelect('seo')}>SEO</button>
</nav>

<style>
	.tabs {
		display: flex;
		gap: 4px;
		border-bottom: 1px solid #e5e7eb;
		margin-bottom: 24px;
	}
	.tabs button {
		padding: 12px 20px;
		background: none;
		border: none;
		font-size: 14px;
		font-weight: 500;
		color: #6b7280;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		display: flex;
		align-items: center;
		gap: 8px;
	}
	.tabs button:hover {
		color: #1f2937;
	}
	.tabs button.active {
		color: #143e59;
		border-bottom-color: #143e59;
	}
	.badge {
		background: #e5e7eb;
		padding: 2px 8px;
		border-radius: 10px;
		font-size: 12px;
	}

	@media (max-width: 639px) {
		.tabs {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
			gap: 0;
		}

		.tabs button {
			padding: 12px 16px;
			white-space: nowrap;
			min-height: 44px;
		}
	}

	@media (hover: none) and (pointer: coarse) {
		.tabs button {
			min-height: 48px;
		}
	}
</style>
