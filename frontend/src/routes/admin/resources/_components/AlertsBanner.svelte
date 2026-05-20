<script lang="ts">
	import { IconCheck, IconX, IconAlertCircle } from '$lib/icons';

	type Props = {
		successMessage: string;
		error: string;
		onClearError: () => void;
	};

	const { successMessage, error, onClearError }: Props = $props();
</script>

{#if successMessage}
	<div class="alert alert-success">
		<IconCheck size={18} />
		{successMessage}
	</div>
{/if}

{#if error}
	<div class="alert alert-error">
		<IconAlertCircle size={18} />
		{error}
		<button class="alert-close" onclick={onClearError}>
			<IconX size={16} />
		</button>
	</div>
{/if}

<style>
	/* Alerts */
	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
	}

	.alert-success {
		background: rgba(34, 197, 94, 0.15);
		border: 1px solid rgba(34, 197, 94, 0.3);
		color: #4ade80;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.alert-close {
		margin-left: auto;
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		opacity: 0.7;
	}

	.alert-close:hover {
		opacity: 1;
	}
</style>
