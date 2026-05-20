<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * NewAlertToast Component - Toast Notification for New Alerts
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays a toast notification when new trading alerts arrive
	 * @version 1.0.0 - Initial Implementation
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import { fly } from 'svelte/transition';

	interface Props {
		alert: any;
		onDismiss: () => void;
		duration?: number;
	}

	const { alert, onDismiss, duration = 5000 }: Props = $props();

	let visible = $state(true);

	$effect(() => {
		const timer = setTimeout(() => {
			visible = false;
			setTimeout(onDismiss, 300);
		}, duration);

		return () => clearTimeout(timer);
	});

	const alertTypeConfig = $derived.by(() => {
		switch (alert.type || alert.alert_type) {
			case 'ENTRY':
				return {
					borderColor: 'var(--color-entry, #14b8a6)',
					label: 'Entry'
				};
			case 'UPDATE':
				return {
					borderColor: 'var(--color-watching, #f59e0b)',
					label: 'Update'
				};
			case 'EXIT':
				return {
					borderColor: 'var(--color-profit, #10b981)',
					label: 'Exit'
				};
			default:
				return {
					borderColor: 'var(--color-profit, #10b981)',
					label: alert.type || alert.alert_type || 'Alert'
				};
		}
	});
</script>

{#if visible}
	<div
		transition:fly={{ y: -20, duration: 300 }}
		class="toast"
		style="--toast-border-color: {alertTypeConfig.borderColor}"
		role="alert"
		aria-live="polite"
	>
		<div class="toast-content">
			<div class="pulse-indicator">
				<span class="pulse-ring"></span>
				<span class="pulse-dot"></span>
			</div>
			<div class="toast-body">
				<p class="toast-title">New {alertTypeConfig.label} Alert</p>
				<p class="toast-message">{alert.ticker} - {alert.title}</p>
			</div>
			<button onclick={onDismiss} class="dismiss-btn" aria-label="Dismiss notification">
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					width="16"
					height="16"
					aria-hidden="true"
				>
					<path d="M18 6L6 18M6 6l12 12" stroke-linecap="round" stroke-linejoin="round" />
				</svg>
			</button>
		</div>
	</div>
{/if}

<style>
	.toast {
		position: fixed;
		top: var(--space-4, 16px);
		right: var(--space-4, 16px);
		z-index: 9999;
		max-width: 360px;
		background: var(--color-bg-card, #ffffff);
		border-radius: var(--radius-lg, 12px);
		border-left: 4px solid var(--toast-border-color);
		box-shadow:
			0 10px 25px rgba(0, 0, 0, 0.1),
			0 4px 10px rgba(0, 0, 0, 0.05);
		overflow: hidden;
	}

	.toast-content {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3, 12px);
		padding: var(--space-4, 16px);
	}

	/* Pulse Indicator */
	.pulse-indicator {
		position: relative;
		flex-shrink: 0;
		width: 12px;
		height: 12px;
		margin-top: 4px;
	}

	.pulse-ring {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: var(--toast-border-color);
		opacity: 0.4;
		animation: pulse-ring 1.5s ease-out infinite;
	}

	.pulse-dot {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: var(--toast-border-color);
	}

	@keyframes pulse-ring {
		0% {
			transform: scale(1);
			opacity: 0.4;
		}
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}

	/* Toast Body */
	.toast-body {
		flex: 1;
		min-width: 0;
	}

	.toast-title {
		margin: 0 0 var(--space-1, 4px) 0;
		font-size: var(--text-sm, 14px);
		font-weight: var(--font-semibold, 600);
		color: var(--color-text-primary, #111827);
		line-height: var(--leading-tight, 1.25);
	}

	.toast-message {
		margin: 0;
		font-size: var(--text-sm, 14px);
		color: var(--color-text-tertiary, #6b7280);
		line-height: var(--leading-snug, 1.375);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	/* Dismiss Button */
	.dismiss-btn {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: var(--radius-sm, 6px);
		color: var(--color-text-muted, #9ca3af);
		cursor: pointer;
		transition: all 0.15s ease-out;
	}

	.dismiss-btn:hover {
		background: var(--color-bg-subtle, #f3f4f6);
		color: var(--color-text-secondary, #374151);
	}

	.dismiss-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary, #3b82f6);
		outline-offset: 2px;
	}

	/* Responsive */
	@media (max-width: 480px) {
		.toast {
			left: var(--space-4, 16px);
			right: var(--space-4, 16px);
			max-width: none;
		}
	}
</style>
