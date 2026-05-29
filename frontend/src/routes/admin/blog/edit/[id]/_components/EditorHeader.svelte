<script lang="ts">
	import IconDeviceFloppy from '@tabler/icons-svelte-runes/icons/device-floppy';
	import IconEye from '@tabler/icons-svelte-runes/icons/eye';
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';

	type Props = {
		status: string;
		saveError: string;
		saveSuccess: string;
		saving: boolean;
		onBack: () => void;
		onSaveDraft: () => void;
		onPublish: () => void;
	};

	let { status, saveError, saveSuccess, saving, onBack, onSaveDraft, onPublish }: Props = $props();
</script>

<div class="editor-header">
	<div class="header-left">
		<button class="btn-back" onclick={onBack}>
			<IconArrowLeft size={20} />
		</button>
		<h1>Edit Post</h1>
		{#if status === 'published'}
			<span class="status-badge published">Published</span>
		{:else if status === 'draft'}
			<span class="status-badge draft">Draft</span>
		{:else if status === 'scheduled'}
			<span class="status-badge scheduled">Scheduled</span>
		{/if}
	</div>
	<div class="header-actions">
		{#if saveError}
			<span class="save-error">{saveError}</span>
		{/if}
		{#if saveSuccess}
			<span class="save-success">{saveSuccess}</span>
		{/if}
		<button class="btn-secondary" onclick={onBack}> Cancel </button>
		<button class="btn-secondary" onclick={onSaveDraft} disabled={saving}>
			<IconDeviceFloppy size={18} />
			{saving ? 'Saving...' : 'Save Draft'}
		</button>
		<button class="btn-primary" onclick={onPublish} disabled={saving}>
			<IconEye size={18} />
			{saving ? 'Publishing...' : 'Publish'}
		</button>
	</div>
</div>

<style>
	.editor-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem 2rem;
		background: white;
		border-bottom: 1px solid #e5e5e5;
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.btn-back {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: #f8f9fa;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		cursor: pointer;
		color: #666;
		transition: all 0.2s;
	}

	.btn-back:hover {
		background: #e5e5e5;
		color: #1a1a1a;
	}

	.editor-header h1 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.status-badge {
		padding: 0.375rem 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.status-badge.published {
		background: rgba(16, 185, 129, 0.1);
		color: #10b981;
	}

	.status-badge.draft {
		background: rgba(245, 158, 11, 0.1);
		color: #f59e0b;
	}

	.status-badge.scheduled {
		background: rgba(59, 130, 246, 0.1);
		color: #3b82f6;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.save-error {
		padding: 0.5rem 1rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 6px;
		color: #dc2626;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.save-success {
		padding: 0.5rem 1rem;
		background: #f0fdf4;
		border: 1px solid #bbf7d0;
		border-radius: 6px;
		color: #16a34a;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.btn-primary,
	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: white;
		color: #666;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover {
		background: #f8f9fa;
	}
</style>
