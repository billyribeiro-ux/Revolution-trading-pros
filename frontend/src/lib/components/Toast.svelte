<script lang="ts">
	/**
	 * Toast Notification Component - Apple ICT9+ Level
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Premium toast notifications with:
	 * - Smooth spring animations
	 * - Progress bar for auto-dismiss
	 * - Loading state with spinner
	 * - Apple-inspired glass morphism design
	 */

	import { fly, fade, scale } from 'svelte/transition';
	import { quintOut, backOut } from 'svelte/easing';
	import { flip } from 'svelte/animate';
	import { toasts, toastStore } from '$lib/stores/toast';
	import type { Toast } from '$lib/stores/toast';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import IconInfoCircle from '@tabler/icons-svelte/icons/info-circle';
	import IconLoader2 from '@tabler/icons-svelte/icons/loader-2';

	function getIcon(type: Toast['type']) {
		switch (type) {
			case 'success':
				return IconCheck;
			case 'error':
				return IconX;
			case 'warning':
				return IconAlertTriangle;
			case 'loading':
				return IconLoader2;
			default:
				return IconInfoCircle;
		}
	}
</script>

<div class="toast-container" aria-live="polite" aria-label="Notifications">
	{#each $toasts as toast, index (toast.id)}
		<div
			class="toast toast-{toast.type}"
			animate:flip={{ duration: 300, easing: quintOut }}
			in:fly={{ y: -30, x: 30, duration: 400, easing: backOut }}
			out:scale={{ duration: 200, start: 0.95, opacity: 0 }}
			role="alert"
			style="--delay: {index * 50}ms"
		>
			<!-- Progress Bar for auto-dismiss -->
			{#if toast.duration > 0}
				<div class="toast-progress">
					<div
						class="toast-progress-bar"
						style="animation-duration: {toast.duration}ms"
					></div>
				</div>
			{/if}

			<div class="toast-icon" class:spinning={toast.type === 'loading'}>
				<svelte:component this={getIcon(toast.type)} size={20} stroke={2} />
			</div>
			<div class="toast-content">
				<span class="toast-message">{toast.message}</span>
			</div>
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
		top: 1.5rem;
		right: 1.5rem;
		z-index: 9999;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		max-width: 420px;
		width: 100%;
		pointer-events: none;
	}

	.toast {
		position: relative;
		display: flex;
		align-items: center;
		gap: 0.875rem;
		padding: 1rem 1.25rem;
		background: rgba(15, 23, 42, 0.95);
		backdrop-filter: blur(20px);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		box-shadow:
			0 20px 50px rgba(0, 0, 0, 0.4),
			0 0 0 1px rgba(255, 255, 255, 0.05) inset;
		pointer-events: auto;
		overflow: hidden;
	}

	/* Progress Bar */
	.toast-progress {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: rgba(255, 255, 255, 0.1);
		overflow: hidden;
	}

	.toast-progress-bar {
		height: 100%;
		width: 100%;
		transform-origin: left;
		animation: progress-shrink linear forwards;
	}

	@keyframes progress-shrink {
		from { transform: scaleX(1); }
		to { transform: scaleX(0); }
	}

	/* Icons */
	.toast-icon {
		width: 36px;
		height: 36px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.toast-icon.spinning {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* Success */
	.toast-success {
		border-color: rgba(16, 185, 129, 0.3);
	}

	.toast-success .toast-icon {
		background: rgba(16, 185, 129, 0.15);
		color: #10b981;
	}

	.toast-success .toast-progress-bar {
		background: linear-gradient(90deg, #10b981, #34d399);
	}

	/* Error */
	.toast-error {
		border-color: rgba(239, 68, 68, 0.3);
	}

	.toast-error .toast-icon {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.toast-error .toast-progress-bar {
		background: linear-gradient(90deg, #ef4444, #f87171);
	}

	/* Warning */
	.toast-warning {
		border-color: rgba(245, 158, 11, 0.3);
	}

	.toast-warning .toast-icon {
		background: rgba(245, 158, 11, 0.15);
		color: #f59e0b;
	}

	.toast-warning .toast-progress-bar {
		background: linear-gradient(90deg, #f59e0b, #fbbf24);
	}

	/* Info */
	.toast-info {
		border-color: rgba(59, 130, 246, 0.3);
	}

	.toast-info .toast-icon {
		background: rgba(59, 130, 246, 0.15);
		color: #3b82f6;
	}

	.toast-info .toast-progress-bar {
		background: linear-gradient(90deg, #3b82f6, #60a5fa);
	}

	/* Loading */
	.toast-loading {
		border-color: rgba(148, 163, 184, 0.3);
	}

	.toast-loading .toast-icon {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}

	/* Content */
	.toast-content {
		flex: 1;
		min-width: 0;
	}

	.toast-message {
		font-size: 0.9375rem;
		font-weight: 500;
		color: #f1f5f9;
		line-height: 1.4;
	}

	/* Close Button */
	.toast-close {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		opacity: 0;
		transition: all 0.2s;
	}

	.toast:hover .toast-close {
		opacity: 1;
	}

	.toast-close:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #f1f5f9;
	}

	/* Mobile */
	@media (max-width: 480px) {
		.toast-container {
			top: auto;
			bottom: 1rem;
			left: 1rem;
			right: 1rem;
			max-width: none;
		}

		.toast {
			padding: 0.875rem 1rem;
		}
	}
</style>
