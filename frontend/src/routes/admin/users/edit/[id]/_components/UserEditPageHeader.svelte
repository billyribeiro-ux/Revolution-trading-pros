<script lang="ts">
	/**
	 * R27-D extraction (2026-05-20): page header for admin user-edit.
	 * Title + subtitle + cancel button. Parent owns the cancel handler
	 * so `goto()` stays at the page level.
	 */
	import { IconX } from '$lib/icons';

	interface Props {
		onCancel: () => void;
	}

	let { onCancel }: Props = $props();
</script>

<div class="page-header">
	<div>
		<h1>Edit User</h1>
		<p>Update user information and roles</p>
	</div>
	<button class="btn-secondary" onclick={onCancel}>
		<IconX size={18} />
		Cancel
	</button>
</div>

<style>
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #94a3b8;
		font-size: 0.95rem;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: 1px solid rgba(148, 163, 184, 0.2);
		background: rgba(148, 163, 184, 0.1);
		color: #cbd5e1;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}
	}
</style>
