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
		color: #6b7280;
		text-decoration: none;
		font-size: 14px;
	}
	.back-link:hover {
		color: #143e59;
	}
	h1 {
		font-size: 24px;
		font-weight: 600;
		color: #1f2937;
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
		background: #fef3c7;
		color: #92400e;
	}
	.status--published {
		background: #d1fae5;
		color: #065f46;
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
		background: #143e59;
		color: #fff;
	}
	.btn-primary:hover {
		background: #0f2d42;
	}
	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
	.btn-secondary {
		background: #f3f4f6;
		color: #374151;
		border: 1px solid #e5e7eb;
	}
	.btn-secondary:hover {
		background: #e5e7eb;
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
