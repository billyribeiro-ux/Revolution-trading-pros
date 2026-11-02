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
		background: var(--bg-elevated, #161b22);
		border-bottom: 1px solid var(--border-default, #30363d);
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		cursor: pointer;
		color: var(--text-secondary, #8b949e);
		transition: all 0.2s;
	}

	.btn-back:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}

	.editor-header h1 {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
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
		background: rgba(63, 185, 80, 0.1);
		color: var(--success-emphasis, #3fb950);
	}

	.status-badge.draft {
		background: rgba(210, 153, 34, 0.1);
		color: var(--warning-emphasis, #d29922);
	}

	.status-badge.scheduled {
		background: rgba(88, 166, 255, 0.1);
		color: var(--info-emphasis, #58a6ff);
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.save-error {
		padding: 0.5rem 1rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		color: var(--error-emphasis, #f85149);
		font-size: 0.875rem;
		font-weight: 500;
	}

	.save-success {
		padding: 0.5rem 1rem;
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		color: var(--success-emphasis, #3fb950);
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
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
	}

	.btn-primary:hover:not(:disabled) {
		background: var(--primary-500, #e6b800);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: var(--bg-elevated, #161b22);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary:hover {
		background: var(--bg-elevated, #161b22);
	}
</style>
