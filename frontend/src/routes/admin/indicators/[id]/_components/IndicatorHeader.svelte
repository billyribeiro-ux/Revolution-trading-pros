<script lang="ts">
	interface Props {
		name: string;
		slug: string;
		isActive: boolean;
		saving: boolean;
		onToggle: () => void;
		onSave: () => void;
	}

	let { name, slug, isActive, saving, onToggle, onSave }: Props = $props();
</script>

<header class="page-header">
	<div class="header-left">
		<a href="/admin/indicators" class="back-link">← Back</a>
		<h1>{name}</h1>
		<span class={['status', { 'status--published': isActive, 'status--draft': !isActive }]}>
			{isActive ? 'Active' : 'Inactive'}
		</span>
	</div>
	<div class="header-actions">
		<a href="/indicators/{slug}" target="_blank" class="btn-secondary">Preview</a>
		<button class="btn-success" onclick={onToggle}>
			{isActive ? 'Deactivate' : 'Activate'}
		</button>
		<button class="btn-primary" onclick={onSave} disabled={saving}>
			{saving ? 'Saving...' : 'Save Changes'}
		</button>
	</div>
</header>

<style>
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 24px;
		flex-wrap: wrap;
		gap: 16px;
	}
	.header-left {
		display: flex;
		align-items: center;
		gap: 12px;
	}
	.back-link {
		color: var(--text-tertiary, #6e7681);
		text-decoration: none;
		font-size: 14px;
	}
	.back-link:hover {
		color: var(--primary-500, #e6b800);
	}
	h1 {
		font-size: 24px;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.status {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 500;
		text-transform: capitalize;
	}
	.status--draft {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		color: var(--warning-emphasis, #d29922);
	}
	.status--published {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	.header-actions {
		display: flex;
		gap: 12px;
	}
	.btn-primary,
	.btn-secondary,
	.btn-success {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		text-decoration: none;
	}
	.btn-primary {
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
	}
	.btn-primary:hover {
		background: var(--primary-500, #e6b800);
	}
	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
	.btn-secondary {
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
		border: 1px solid var(--border-default, #30363d);
	}
	.btn-secondary:hover {
		background: var(--bg-surface, #1c2128);
	}
	.btn-success {
		background: #10b981;
		color: #fff;
	}
	.btn-success:hover {
		background: #059669;
	}

	@media (max-width: 639px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 12px;
		}

		.header-left {
			flex-wrap: wrap;
		}

		.header-actions {
			width: 100%;
			flex-wrap: wrap;
		}

		.header-actions button,
		.header-actions a {
			flex: 1;
			min-width: 100px;
			justify-content: center;
		}

		h1 {
			font-size: 20px;
		}
	}

	@media (min-width: 640px) {
		.header-actions {
			flex-wrap: nowrap;
		}
	}

	@media (hover: none) and (pointer: coarse) {
		.btn-primary,
		.btn-secondary,
		.btn-success {
			padding: 12px 24px;
			min-height: 48px;
		}
	}
</style>
