<script lang="ts">
	/**
	 * R27-D extraction (2026-05-20): success / error alert banner used at
	 * the top of the admin user-edit form. Modeled as a discriminated
	 * `kind` prop so the parent renders ONE component instead of two
	 * near-identical {#if} blocks; error variant gets the Retry button.
	 */
	import { IconCheck, IconX } from '$lib/icons';

	type Props =
		| { kind: 'success'; message: string }
		| { kind: 'error'; message: string; onRetry: () => void };

	let props: Props = $props();
</script>

{#if props.kind === 'success'}
	<div class="alert success">
		<IconCheck size={20} />
		{props.message}
	</div>
{:else}
	<div class="alert error">
		<IconX size={20} />
		<span class="error-message">{props.message}</span>
		<button class="btn-retry" onclick={props.onRetry}>Retry</button>
	</div>
{/if}

<style>
	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-radius: 8px;
		margin-bottom: 1.5rem;
		font-weight: 500;
	}

	.alert.success {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.alert.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
		justify-content: flex-start;
	}

	.error-message {
		flex: 1;
	}

	.btn-retry {
		padding: 0.5rem 1rem;
		background: rgba(239, 68, 68, 0.2);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #f87171;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.btn-retry:hover {
		background: rgba(239, 68, 68, 0.3);
	}
</style>
