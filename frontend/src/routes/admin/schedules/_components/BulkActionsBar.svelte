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
		background: #143e59;
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
		background: #f1f5f9;
		color: #475569;
	}

	.btn-sm:hover {
		background: #e2e8f0;
	}

	.btn-danger {
		background: #fef2f2;
		color: #dc2626;
	}

	.btn-danger:hover {
		background: #fee2e2;
	}

	.btn-text {
		background: none;
		color: #fff;
	}

	.btn-text:hover {
		text-decoration: underline;
	}
</style>
