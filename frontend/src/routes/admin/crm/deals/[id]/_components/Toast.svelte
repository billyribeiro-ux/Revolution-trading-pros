<script lang="ts">
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import type { ToastMessage } from './helpers';

	interface Props {
		toast: ToastMessage;
		onDismiss: () => void;
	}

	let { toast, onDismiss }: Props = $props();
</script>

<div class="toast toast-{toast.type}" role="alert" aria-live="polite">
	{#if toast.type === 'success'}
		<IconCheck size={18} />
	{:else}
		<IconX size={18} />
	{/if}
	<span>{toast.text}</span>
	<button class="toast-close" onclick={onDismiss} aria-label="Dismiss">
		<IconX size={14} />
	</button>
</div>

<style>
	.toast {
		position: fixed;
		bottom: 24px;
		right: 24px;
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 18px;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 500;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
		z-index: 2000;
		animation: slideIn 0.3s ease-out;
	}

	.toast-success {
		background: linear-gradient(135deg, #065f46, #047857);
		border: 1px solid #10b981;
		color: #ecfdf5;
	}

	.toast-success :global(svg) {
		color: #34d399;
	}

	.toast-error {
		background: linear-gradient(135deg, #7f1d1d, #991b1b);
		border: 1px solid #f87171;
		color: #fef2f2;
	}

	.toast-error :global(svg) {
		color: #fca5a5;
	}

	.toast-close {
		display: flex;
		padding: 4px;
		margin-left: 8px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 6px;
		color: inherit;
		cursor: pointer;
		transition: background 0.2s;
	}

	.toast-close:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@media (max-width: 767.98px) {
		.toast {
			left: 16px;
			right: 16px;
			bottom: 16px;
		}
	}
</style>
