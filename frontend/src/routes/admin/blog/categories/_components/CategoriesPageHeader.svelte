<script lang="ts">
	/**
	 * R28-C extraction (2026-05-20): page header for the categories/tags
	 * admin page — title, subtitle, and a single refresh action button.
	 */
	import { IconRefresh } from '$lib/icons';

	interface Props {
		loading: boolean;
		onRefresh: () => void;
	}

	let { loading, onRefresh }: Props = $props();
</script>

<header class="page-header">
	<h1>Categories & Tags</h1>
	<p class="subtitle">Organize your blog posts with categories and tags</p>
	<div class="header-actions">
		<button class="btn-secondary" onclick={onRefresh} disabled={loading}>
			<IconRefresh size={18} class={loading ? 'spinning' : ''} />
			Refresh
		</button>
	</div>
</header>

<style>
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin-bottom: 1rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.5);
	}

	:global(.spinning) {
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
</style>
