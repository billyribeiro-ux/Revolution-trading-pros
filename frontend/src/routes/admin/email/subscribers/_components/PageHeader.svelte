<script lang="ts">
	import { IconUsers, IconDownload, IconUserPlus } from '#lib/icons/index.js';

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
		background: var(--primary-500);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--bg-base, #0d1117);
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.subtitle {
		color: var(--text-tertiary, #6e7681);
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
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.btn-secondary {
		background: var(--bg-surface, #1c2128);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	@media (max-width: 767.98px) {
		.header-content {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}
	}
</style>
