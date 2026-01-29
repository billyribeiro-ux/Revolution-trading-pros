<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * AlertNotificationManager Component - Browser Notification & Sound System
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Manages browser notifications and audio alerts for new trading alerts
	 * @version 1.0.0 - Initial Implementation
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';

	interface Props {
		enabled?: boolean;
		soundEnabled?: boolean;
		onNewAlert?: (alert: any) => void;
	}

	const { enabled = true, soundEnabled = true, onNewAlert }: Props = $props();

	let hasPermission = $state(false);
	let audioContext: AudioContext | null = $state(null);

	onMount(() => {
		if (browser && 'Notification' in window) {
			hasPermission = Notification.permission === 'granted';
		}
	});

	const requestPermission = async () => {
		if (browser && 'Notification' in window) {
			const result = await Notification.requestPermission();
			hasPermission = result === 'granted';
		}
	};

	function showNotification(alert: any) {
		if (!enabled || !hasPermission) return;

		const notification = new Notification('New Trading Alert', {
			body: `${alert.alert_type}: ${alert.ticker} - ${alert.title}`,
			icon: '/favicon.png',
			tag: `alert-${alert.id}`,
			requireInteraction: true
		});

		notification.onclick = () => {
			window.focus();
			notification.close();
		};

		if (soundEnabled) {
			playAlertSound();
		}

		onNewAlert?.(alert);
	}

	function playAlertSound() {
		if (!audioContext) {
			audioContext = new AudioContext();
		}

		const oscillator = audioContext.createOscillator();
		const gainNode = audioContext.createGain();

		oscillator.connect(gainNode);
		gainNode.connect(audioContext.destination);

		oscillator.frequency.value = 800;
		oscillator.type = 'sine';

		gainNode.gain.setValueAtTime(0.3, audioContext.currentTime);
		gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.3);

		oscillator.start(audioContext.currentTime);
		oscillator.stop(audioContext.currentTime + 0.3);
	}

	// Expose for external use
	export { showNotification, requestPermission };
</script>

{#if !hasPermission && browser}
	<button
		onclick={requestPermission}
		class="notification-btn"
		aria-label="Enable browser notifications"
	>
		<svg
			class="bell-icon"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			width="16"
			height="16"
			aria-hidden="true"
		>
			<path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
			<path d="M13.73 21a2 2 0 0 1-3.46 0" />
		</svg>
		Enable notifications for new alerts
	</button>
{/if}

<style>
	.notification-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2, 8px);
		padding: var(--space-2, 8px) var(--space-3, 12px);
		background: transparent;
		border: 1px solid var(--color-border-default, #e5e7eb);
		border-radius: var(--radius-md, 8px);
		font-size: var(--text-sm, 14px);
		font-weight: var(--font-medium, 500);
		color: var(--color-brand-primary, #3b82f6);
		cursor: pointer;
		transition: all 0.2s ease-out;
	}

	.notification-btn:hover {
		background: var(--color-brand-primary, #3b82f6);
		border-color: var(--color-brand-primary, #3b82f6);
		color: var(--color-bg-card, #ffffff);
	}

	.notification-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary, #3b82f6);
		outline-offset: 2px;
	}

	.notification-btn:active {
		transform: scale(0.98);
	}

	.bell-icon {
		flex-shrink: 0;
	}
</style>
