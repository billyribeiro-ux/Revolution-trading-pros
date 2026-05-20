<script lang="ts">
	import { IconRefresh, IconDownload } from '$lib/icons';

	interface Props {
		onrefresh: () => void;
		onexport: () => void;
	}

	let { onrefresh, onexport }: Props = $props();
</script>

<header class="page-header">
	<h1>Orders Management</h1>
	<p class="subtitle">View and manage all customer transactions</p>
	<div class="header-actions">
		<button class="btn-secondary" onclick={onrefresh}>
			<IconRefresh size={18} />
			Refresh
		</button>
		<button class="btn-secondary" onclick={onexport}>
			<IconDownload size={18} />
			Export
		</button>
	</div>
</header>

<style>
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 800;
		color: var(--admin-text-primary, #f8fafc);
		margin: 0 0 0.5rem;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1.5rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		border: 1px solid rgba(100, 116, 139, 0.3);
		background: rgba(100, 116, 139, 0.2);
		color: var(--text-primary);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
	}
</style>
