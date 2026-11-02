<script lang="ts">
	import { IconCheck, IconX, IconAlertCircle } from '#lib/icons/index.js';

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
		background: rgba(63, 185, 80, 0.15);
		border: 1px solid rgba(63, 185, 80, 0.3);
		color: var(--success-emphasis, #3fb950);
	}

	.alert-error {
		background: rgba(248, 81, 73, 0.15);
		border: 1px solid rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
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
