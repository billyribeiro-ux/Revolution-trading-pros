<script lang="ts">
	/**
	 * R23-C extraction (2026-05-20): top-of-form validation alerts list.
	 */
	import { slide } from 'svelte/transition';
	import { IconAlertCircle } from '$lib/icons';
	import type { ValidationError } from './types';

	interface Props {
		errors: ValidationError[];
	}

	let { errors }: Props = $props();
</script>

{#if errors.length > 0}
	<div class="alerts" transition:slide={{ duration: 300 }}>
		{#each errors as error (error.message)}
			<div class={['alert', `alert-${error.severity}`]}>
				<IconAlertCircle size={20} />
				<span>{error.message}</span>
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
		padding: 1rem 1.5rem;
		border-radius: 8px;
		margin-bottom: 0.75rem;
		font-weight: 500;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.alert-warning {
		background: rgba(245, 158, 11, 0.1);
		border: 1px solid rgba(245, 158, 11, 0.3);
		color: #fbbf24;
	}
</style>
