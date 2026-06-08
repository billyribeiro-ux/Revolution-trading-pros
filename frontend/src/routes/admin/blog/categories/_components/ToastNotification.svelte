<script lang="ts">
	/**
	 * R28-C extraction (2026-05-20): toast component for the categories/tags
	 * admin page. Parent owns the auto-dismiss timer; this component is
	 * presentational only and forwards a single `onClose` callback for the
	 * close-button.
	 */
	import { fade } from 'svelte/transition';
	import { IconCheck, IconAlertCircle, IconX } from '$lib/icons';
	import type { ToastType } from './types';

	interface Props {
		message: string;
		type: ToastType;
		onClose: () => void;
	}

	let { message, type, onClose }: Props = $props();
</script>

<div class={['toast', `toast-${type}`]} transition:fade>
	{#if type === 'success'}
		<IconCheck size={20} />
	{:else}
		<IconAlertCircle size={20} />
	{/if}
	<span>{message}</span>
	<button onclick={onClose} aria-label="Dismiss notification">
		<IconX size={16} />
	</button>
</div>

<style>
	.toast {
		position: fixed;
		top: 2rem;
		right: 2rem;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		background: rgba(30, 41, 59, 0.98);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
		z-index: 2000;
		min-width: 300px;
	}

	.toast-success {
		border-left: 4px solid #10b981;
		color: #10b981;
	}

	.toast-error {
		border-left: 4px solid #ef4444;
		color: #ef4444;
	}

	.toast span {
		flex: 1;
		color: #f1f5f9;
	}

	.toast button {
		background: none;
		border: none;
		color: #94a3b8;
		cursor: pointer;
		padding: 0;
		display: flex;
	}

	.toast button:hover {
		color: #f1f5f9;
	}
</style>
