<script lang="ts">
	/**
	 * R25-C extraction (2026-05-20): success + validation error banners shown
	 * above the coupon-create form. Pure presentational — parent owns the
	 * state, this just renders.
	 */
	import { slide } from 'svelte/transition';
	import { IconCheck, IconAlertCircle } from '$lib/icons';

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
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.alert-success {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}
</style>
