<!--
	BulkActionsBar — count + activate / deactivate / delete / clear buttons.
	Extracted from /admin/schedules in R21-C.
-->
<script lang="ts">
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';

	interface Props {
		selectedCount: number;
		onbulkToggleActive: (active: boolean) => void;
		onbulkDelete: () => void;
		onclear: () => void;
	}

	let { selectedCount, onbulkToggleActive, onbulkDelete, onclear }: Props = $props();
</script>

<div class="bulk-actions">
	<span class="bulk-count">{selectedCount} selected</span>
	<button class="btn btn-sm" onclick={() => onbulkToggleActive(true)}>
		<IconCheck size={16} />
		Activate
	</button>
	<button class="btn btn-sm" onclick={() => onbulkToggleActive(false)}>
		<IconX size={16} />
		Deactivate
	</button>
	<button class="btn btn-sm btn-danger" onclick={onbulkDelete}>
		<IconTrash size={16} />
		Delete
	</button>
	<button class="btn btn-sm btn-text" onclick={onclear}>Clear</button>
</div>

<style>
	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: var(--primary-500, #e6b800);
		border-radius: 8px;
		margin-bottom: 16px;
	}

	.bulk-count {
		color: #fff;
		font-size: 14px;
		font-weight: 500;
		margin-right: auto;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-sm {
		padding: 6px 12px;
		font-size: 13px;
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--text-tertiary, #6e7681);
	}

	.btn-sm:hover {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
	}

	.btn-danger {
		background: var(--bg-elevated, #161b22);
		color: var(--error-emphasis, #f85149);
	}

	.btn-danger:hover {
		background: var(--error-soft, rgba(218, 54, 51, 0.15));
	}

	.btn-text {
		background: none;
		color: #fff;
	}

	.btn-text:hover {
		text-decoration: underline;
	}
</style>
