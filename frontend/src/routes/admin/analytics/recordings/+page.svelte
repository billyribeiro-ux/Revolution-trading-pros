<script lang="ts">
	import { IconPlayerPlay, IconRefresh } from '$lib/icons';

	let isLoading = false;
</script>

<svelte:head>
	<title>Session Recordings - Analytics</title>
</svelte:head>

<div class="recordings-page">
	<div class="page-header">
		<div>
			<h1>Session Recordings</h1>
			<p class="page-description">Watch how users interact with your site</p>
		</div>
		<button class="btn-refresh" disabled={isLoading}>
			<IconRefresh size={18} />
		</button>
	</div>

	<div class="empty-state">
		<IconPlayerPlay size={64} />
		<h3>No recordings available</h3>
		<p>Session recordings will appear once enabled and visitors interact with your site</p>
	</div>
</div>

<style>
	.recordings-page { max-width: 1400px; }

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
