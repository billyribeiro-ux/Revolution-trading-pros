<script lang="ts">
	/**
	 * R25-C extraction (2026-05-20): success + validation error banners shown
	 * above the coupon-create form. Pure presentational — parent owns the
	 * state, this just renders.
	 */
	import { slide } from 'svelte/transition';
	import { IconCheck, IconAlertCircle } from '#lib/icons/index.js';

	interface Props {
		successMessage: string | null;
		errors: string[];
	}

	let { successMessage, errors }: Props = $props();
</script>

{#if successMessage}
	<div class="alert alert-success" transition:slide={{ duration: 200 }}>
		<IconCheck size={20} />
		<span>{successMessage}</span>
	</div>
{/if}

{#if errors.length > 0}
	<div class="alerts" transition:slide={{ duration: 200 }}>
		{#each errors as error (error)}
			<div class="alert alert-error">
				<IconAlertCircle size={20} />
				<span>{error}</span>
			</div>
		{/each}
	</div>
{/if}

<style>
	.alerts {
		margin-bottom: 1.5rem;
	}

	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-radius: 8px;
		margin-bottom: 0.75rem;
		font-weight: 500;
	}

	.alert-error {
		background: rgba(248, 81, 73, 0.1);
		border: 1px solid rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
	}

	.alert-success {
		background: rgba(63, 185, 80, 0.1);
		border: 1px solid rgba(63, 185, 80, 0.3);
		color: var(--success-emphasis, #3fb950);
	}
</style>
