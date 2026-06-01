<script lang="ts">
	import { IconUsers, IconDownload, IconUserPlus } from '$lib/icons';

	interface Props {
		onexport: () => void;
		onadd: () => void;
	}

	let { onexport, onadd }: Props = $props();
</script>

<div class="page-header">
	<div class="header-content">
		<div class="header-title">
			<div class="title-icon">
				<IconUsers size={28} />
			</div>
			<div>
				<h1>Email Subscribers</h1>
				<p class="subtitle">Manage your email audience and subscriber lists</p>
			</div>
		</div>

		<div class="header-actions">
			<button class="btn-secondary" onclick={onexport}>
				<IconDownload size={18} />
				Export
			</button>
			<button class="btn-primary" onclick={onadd}>
				<IconUserPlus size={18} />
				Add Subscriber
			</button>
		</div>
	</div>
</div>

<style>
	.page-header {
		margin-bottom: 2rem;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	@media (max-width: 767.98px) {
		.header-content {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}
	}
</style>
