<script lang="ts">
	import { fly, fade } from 'svelte/transition';
	import { toasts, toastStore } from '$lib/stores/toast';
	import type { Toast } from '$lib/stores/toast';
	import { IconCheck, IconX, IconAlertTriangle, IconInfoCircle } from '@tabler/icons-svelte';

	function getIcon(type: Toast['type']) {
		switch (type) {
			case 'success':
				return IconCheck;
			case 'error':
				return IconX;
			case 'warning':
				return IconAlertTriangle;
			default:
				return IconInfoCircle;
		}
	}
</script>

<div class="toast-container" aria-live="polite">
	{#each $toasts as toast (toast.id)}
		<div
			class="toast toast-{toast.type}"
			in:fly={{ y: 50, duration: 300 }}
			out:fade={{ duration: 200 }}
			role="alert"
		>
			<div class="toast-icon">
				<svelte:component this={getIcon(toast.type)} size={20} />
			</div>
			<div class="toast-message">{toast.message}</div>
			{#if toast.dismissible}
				<button class="toast-close" onclick={() => toastStore.dismiss(toast.id)} aria-label="Dismiss">
					<IconX size={16} />
				</button>
			{/if}
		</div>
	{/each}
</div>

<style>
	.toast-container {
		position: fixed;
		bottom: 1.5rem;
		right: 1.5rem;
		z-index: 9999;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		max-width: 400px;
		pointer-events: none;
	}

	.toast {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.3), 0 8px 10px -6px rgba(0, 0, 0, 0.2);
		pointer-events: auto;
		animation: slideIn 0.3s ease-out;
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}

	.toast-icon {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.toast-success .toast-icon {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.toast-error .toast-icon {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.toast-warning .toast-icon {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}

	.toast-info .toast-icon {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}

	.toast-message {
		flex: 1;
		font-size: 0.875rem;
		color: #f1f5f9;
		line-height: 1.4;
	}

	.toast-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.toast-close:hover {
		background: rgba(148, 163, 184, 0.1);
		color: #f1f5f9;
	}

	.toast-success {
		border-left: 3px solid #34d399;
	}

	.toast-error {
		border-left: 3px solid #f87171;
	}

	.toast-warning {
		border-left: 3px solid #fbbf24;
	}

	.toast-info {
		border-left: 3px solid #60a5fa;
	}

	@media (max-width: 640px) {
		.toast-container {
			left: 1rem;
			right: 1rem;
			bottom: 1rem;
			max-width: none;
		}
	}
</style>
