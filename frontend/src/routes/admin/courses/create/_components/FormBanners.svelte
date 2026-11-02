<script lang="ts">
	import { IconAlertCircle, IconCheck, IconX } from '#lib/icons/index.js';

	// Top-of-form error / success banners. The error banner has a dismiss
	// button that clears the message via the `onDismissError` callback.
	// Behaviour preserved 1:1 from courses/create +page.svelte
	// (originally lines 1631-1644).
	interface Props {
		error: string;
		success: string;
		onDismissError: () => void;
	}

	let { error, success, onDismissError }: Props = $props();
</script>

{#if error}
	<div class="form-error-banner">
		<IconAlertCircle size={20} />
		<span>{error}</span>
		<button onclick={onDismissError}><IconX size={16} /></button>
	</div>
{/if}

{#if success}
	<div class="form-success-banner">
		<IconCheck size={20} />
		<span>{success}</span>
	</div>
{/if}

<style>
	/* Mirrors the original +page.svelte rules so the banners render identically. */
	.form-error-banner,
	.form-success-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-radius: 12px;
		margin-bottom: 1.5rem;
		font-size: 0.9375rem;
	}

	.form-error-banner {
		background: rgba(248, 81, 73, 0.15);
		border: 1px solid rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
	}

	.form-error-banner button {
		margin-left: auto;
		background: none;
		border: none;
		color: var(--error-emphasis, #f85149);
		cursor: pointer;
		padding: 0.25rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
		transition: background 0.2s;
	}

	.form-error-banner button:hover {
		background: rgba(248, 81, 73, 0.2);
	}

	.form-success-banner {
		background: rgba(63, 185, 80, 0.15);
		border: 1px solid rgba(63, 185, 80, 0.3);
		color: #86efac;
	}
</style>
