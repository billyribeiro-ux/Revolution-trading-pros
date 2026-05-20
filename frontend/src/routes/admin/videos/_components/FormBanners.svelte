<script lang="ts">
	import { IconCheck, IconX, IconAlertCircle } from '$lib/icons';

	interface Props {
		successMessage: string;
		error: string;
		onDismissError: () => void;
	}

	let { successMessage, error, onDismissError }: Props = $props();
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
		<button class="alert-close" onclick={onDismissError}>
			<IconX size={16} />
		</button>
	</div>
{/if}

<style>
	/* Alert Messages */
	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
		font-size: 0.9rem;
	}

	.alert-success {
		background: rgba(34, 197, 94, 0.15);
		border: 1px solid rgba(34, 197, 94, 0.3);
		color: var(--success-emphasis);
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: var(--error-emphasis);
	}

	.alert-close {
		margin-left: auto;
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	.alert-close:hover {
		opacity: 1;
	}
</style>
