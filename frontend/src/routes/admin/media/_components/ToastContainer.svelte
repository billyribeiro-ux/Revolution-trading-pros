<script lang="ts">
	/**
	 * ToastContainer — fixed-position stack of transient notification pills.
	 * Extracted from `admin/media/+page.svelte` (R10-C). 0 binds, 0 callbacks.
	 */
	import { fly } from 'svelte/transition';
	import IconCircleCheckFilled from '@tabler/icons-svelte-runes/icons/circle-check-filled';
	import IconCircleXFilled from '@tabler/icons-svelte-runes/icons/circle-x-filled';
	import IconInfoCircle from '@tabler/icons-svelte-runes/icons/info-circle';

	type Toast = { id: string; message: string; type: 'success' | 'error' | 'info' };

	let { toasts }: { toasts: Toast[] } = $props();
</script>

<div class="toast-container">
	{#each toasts as toast (toast.id)}
		<div class={['toast', `toast-${toast.type}`]} transition:fly={{ y: 20, duration: 300 }}>
			{#if toast.type === 'success'}
				<IconCircleCheckFilled size={20} aria-hidden="true" />
			{:else if toast.type === 'error'}
				<IconCircleXFilled size={20} aria-hidden="true" />
			{:else}
				<IconInfoCircle size={20} aria-hidden="true" />
			{/if}
			<span>{toast.message}</span>
		</div>
	{/each}
</div>

<style>
	.toast-container {
		position: fixed;
		bottom: 1.5rem;
		right: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		z-index: 1000;
	}

	.toast {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.95);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.8125rem;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
	}

	.toast :global(svg) {
		width: 18px;
		height: 18px;
	}

	.toast-success :global(svg) {
		color: #22c55e;
	}

	.toast-error :global(svg) {
		color: #ef4444;
	}

	.toast-info :global(svg) {
		color: var(--primary-500);
	}
</style>
