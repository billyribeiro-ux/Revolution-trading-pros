<script lang="ts">
	import { IconClick, IconEye, IconRefresh } from '@tabler/icons-svelte';

	let pages: any[] = [];
	let isLoading = false;
</script>

<svelte:head>
	<title>Heatmaps - Analytics</title>
</svelte:head>

<div class="heatmaps-page">
	<div class="page-header">
		<div>
			<h1>Click Heatmaps</h1>
			<p class="page-description">Visualize where users click on your pages</p>
		</div>
		<button class="btn-refresh" disabled={isLoading}>
			<IconRefresh size={18} />
		</button>
	</div>

	<div class="empty-state">
		<IconClick size={64} />
		<h3>No heatmap data yet</h3>
		<p>Heatmaps will appear once you have visitor activity</p>
	</div>
</div>

<style>
	.heatmaps-page { max-width: 1400px; }

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-description { color: #64748b; margin: 0; }

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 4rem 2rem;
		text-align: center;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
	}

	.empty-state :global(svg) { color: #475569; margin-bottom: 1rem; }
	.empty-state h3 { color: #e2e8f0; margin: 0 0 0.5rem 0; }
	.empty-state p { color: #64748b; margin: 0; }
</style>
