<script lang="ts">
	import { IconChartBar, IconCloudUpload, IconRefresh, IconPlus } from '$lib/icons';

	interface Props {
		hasSelectedRoom: boolean;
		isLoading: boolean;
		onAnalytics: () => void;
		onBunnyUpload: () => void;
		onRefresh: () => void;
		onAddVideo: () => void;
	}

	let { hasSelectedRoom, isLoading, onAnalytics, onBunnyUpload, onRefresh, onAddVideo }: Props =
		$props();
</script>

<div class="page-header">
	<h1>Trading Room Videos</h1>
	<p class="subtitle">Manage daily videos with category tags for each room and service</p>
	<div class="header-actions">
		<button class="btn-secondary" onclick={onAnalytics} title="View Analytics">
			<IconChartBar size={18} />
			Analytics
		</button>
		<button
			class="btn-bunny"
			onclick={onBunnyUpload}
			disabled={!hasSelectedRoom}
			title="Upload to Bunny.net"
		>
			<IconCloudUpload size={18} />
			Bunny Upload
		</button>
		<button class="btn-refresh" onclick={onRefresh} disabled={isLoading}>
			<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
		</button>
		<button class="btn-primary" onclick={onAddVideo} disabled={!hasSelectedRoom}>
			<IconPlus size={18} />
			Add Video
		</button>
	</div>
</div>

<style>
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0 0 0.25rem 0;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0.25rem 0 1rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	/* Buttons (duplicated from parent — still used elsewhere there) */
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		border: none;
		border-radius: 6px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--text-secondary);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--text-primary);
	}

	.btn-bunny {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #ff9500, #ff7b00);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-bunny:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(255, 149, 0, 0.4);
	}

	.btn-bunny:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.btn-refresh {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(22, 27, 34, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 767.98px) {
		.header-actions {
			flex-wrap: wrap;
		}
	}
</style>
