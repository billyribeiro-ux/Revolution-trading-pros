<script lang="ts">
	/**
	 * NotificationPanel - Enterprise Notification Center
	 * Dropdown panel showing all notifications with actions
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import { onMount, onDestroy } from 'svelte';
	import { fly, fade } from 'svelte/transition';
	import { gsap } from 'gsap';
	import {
		notificationStore,
		getNotifications,
		getUnreadCount,
		type Notification,
		type NotificationType
	} from '$lib/stores/notifications.svelte';
	import { Icon, IconAlertCircle, IconAlertTriangle, IconBell, IconCircleCheck, IconInfoCircle, IconSettings, IconX } from '$lib/icons';

	interface Props {
		position?: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
		onclick?: (notification: Notification) => void;
	}

	let props: Props = $props();
	let position = $derived(props.position ?? 'top-right');
	let onclick = $derived(props.onclick);

	let isOpen = $state(false);
	let bellRef: HTMLButtonElement | null = $state(null);
	let panelRef: HTMLDivElement | null = $state(null);

	// Local derived from getters
	const notifications = $derived(getNotifications());
	const unreadCount = $derived(getUnreadCount());

	const iconMap: Record<NotificationType, string> = {
		info: IconInfoCircle,
		success: IconCircleCheck,
		warning: IconAlertTriangle,
		error: IconAlertCircle,
		system: IconSettings
	};


	function toggle() {
		isOpen = !isOpen;
		if (isOpen) {
			// Animate bell
			gsap.to(bellRef, {
				rotation: 15,
				duration: 0.1,
				yoyo: true,
				repeat: 3,
				ease: 'power1.inOut',
				onComplete: () => {
					gsap.set(bellRef, { rotation: 0 });
				}
			});
		}
	}

	function close() {
		isOpen = false;
	}

	function handleClickOutside(event: MouseEvent) {
		if (
			isOpen &&
			panelRef &&
			bellRef &&
			!panelRef.contains(event.target as Node) &&
			!bellRef.contains(event.target as Node)
		) {
			close();
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && isOpen) {
			close();
		}
	}

	function formatTime(date: Date): string {
		const now = new Date();
		const diff = now.getTime() - date.getTime();
		const minutes = Math.floor(diff / 60000);
		const hours = Math.floor(diff / 3600000);
		const days = Math.floor(diff / 86400000);

		if (minutes < 1) return 'Just now';
		if (minutes < 60) return `${minutes}m ago`;
		if (hours < 24) return `${hours}h ago`;
		if (days < 7) return `${days}d ago`;
		return date.toLocaleDateString();
	}

	function handleNotificationClick(notification: Notification) {
		notificationStore.markAsRead(notification.id);

		if (notification.action?.onClick) {
			notification.action.onClick();
		} else if (notification.action?.href) {
			window.location.href = notification.action.href;
		}

		onclick?.(notification);
	}

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		document.addEventListener('keydown', handleKeydown);
		notificationStore.initWebSocket();
	});

	onDestroy(() => {
		document.removeEventListener('click', handleClickOutside);
		document.removeEventListener('keydown', handleKeydown);
	});

</script>

<div class="np-wrapper">
	<!-- Bell Button -->
	<button
		bind:this={bellRef}
		onclick={toggle}
		class="np-bell"
		aria-label="Notifications"
		aria-expanded={isOpen}
	>
		<Icon icon={IconBell} size={24} />

		<!-- Unread Badge -->
		{#if unreadCount > 0}
			<span class="np-badge">
				{unreadCount > 99 ? '99+' : unreadCount}
			</span>
		{/if}
	</button>

	<!-- Dropdown Panel -->
	{#if isOpen}
		<div
			bind:this={panelRef}
			transition:fly={{ y: -10, duration: 200 }}
			class="np-panel"
			data-position={position}
		>
			<!-- Header -->
			<div class="np-header">
				<h3 class="np-header-title">Notifications</h3>
				<div class="np-header-actions">
					{#if unreadCount > 0}
						<button
							onclick={() => notificationStore.markAllAsRead()}
							class="np-mark-read"
						>
							Mark all read
						</button>
					{/if}
					<button onclick={close} class="np-close" aria-label="Close">
						<Icon icon={IconX} size={18} />
					</button>
				</div>
			</div>

			<!-- Notification List -->
			<div class="np-list">
				{#if notifications.length === 0}
					<div class="np-empty">
						<Icon icon={IconBell} size={48} />
						<p>No notifications yet</p>
					</div>
				{:else}
					{#each notifications.filter((n) => !n.dismissed) as notification (notification.id)}
						{@const NotifIcon = iconMap[notification.type]}
						<div
							transition:fade={{ duration: 150 }}
							class="np-item"
							data-read={notification.read || undefined}
							onclick={() => handleNotificationClick(notification)}
							onkeypress={(e: KeyboardEvent) =>
								e.key === 'Enter' && handleNotificationClick(notification)}
							role="button"
							tabindex="0"
						>
							<!-- Unread indicator -->
							{#if !notification.read}
								<div class="np-unread-dot"></div>
							{/if}

							<div class="np-item-inner">
								<!-- Icon -->
								<div class="np-item-icon" data-type={notification.type}>
									<Icon icon={NotifIcon} size={20} />
								</div>

								<!-- Content -->
								<div class="np-item-content">
									<div class="np-item-top">
										<p class="np-item-title">{notification.title}</p>
										<span class="np-item-time">
											{formatTime(notification.timestamp)}
										</span>
									</div>
									<p class="np-item-message">{notification.message}</p>

									{#if notification.action}
										<button
											onclick={(e: MouseEvent) => {
												e.stopPropagation();
												handleNotificationClick(notification);
											}}
											class="np-item-action"
										>
											{notification.action.label}
										</button>
									{/if}
								</div>

								<!-- Dismiss button -->
								<button
									onclick={(e: MouseEvent) => {
										e.stopPropagation();
										notificationStore.dismiss(notification.id);
									}}
									class="np-dismiss"
									aria-label="Dismiss"
								>
									<Icon icon={IconX} size={16} />
								</button>
							</div>

							<!-- Priority indicator for urgent -->
							{#if notification.priority === 'urgent'}
								<div class="np-urgent-bar"></div>
							{/if}
						</div>
					{/each}
				{/if}
			</div>

			<!-- Footer -->
			{#if notifications.length > 0}
				<div class="np-footer">
					<button onclick={() => notificationStore.clear()} class="np-clear-all">
						Clear all notifications
					</button>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.np-wrapper {
		position: relative;
	}

	.np-bell {
		position: relative;
		padding: var(--space-2);
		border-radius: var(--radius-lg);
		background: none;
		border: none;
		color: oklch(0.65 0.01 250);
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease-default);

		&:hover {
			background-color: oklch(0.3 0.01 250 / 50%);
			color: oklch(1 0 0);
		}
	}

	.np-badge {
		position: absolute;
		inset-block-start: -0.25rem;
		inset-inline-end: -0.25rem;
		min-inline-size: 1.25rem;
		block-size: 1.25rem;
		padding-inline: 0.375rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: oklch(0.55 0.24 27);
		color: oklch(1 0 0);
		font-size: var(--text-xs);
		font-weight: var(--weight-bold);
		border-radius: 9999px;
		animation: np-pulse 2s ease-in-out infinite;
	}

	.np-panel {
		position: absolute;
		inline-size: 24rem;
		max-block-size: 70vh;
		background-color: oklch(0.15 0.01 250 / 95%);
		backdrop-filter: blur(16px);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-2xl);
		overflow: hidden;
		z-index: 50;

		&[data-position='top-right'] { inset-inline-end: 0; inset-block-start: 100%; margin-block-start: var(--space-2); }
		&[data-position='top-left'] { inset-inline-start: 0; inset-block-start: 100%; margin-block-start: var(--space-2); }
		&[data-position='bottom-right'] { inset-inline-end: 0; inset-block-end: 100%; margin-block-end: var(--space-2); }
		&[data-position='bottom-left'] { inset-inline-start: 0; inset-block-end: 100%; margin-block-end: var(--space-2); }
	}

	.np-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		border-block-end: 1px solid oklch(0.38 0.01 250 / 50%);
		background-color: oklch(0.25 0.01 250 / 50%);
	}

	.np-header-title {
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
	}

	.np-header-actions {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.np-mark-read {
		font-size: var(--text-xs);
		color: oklch(0.65 0.18 260);
		background: none;
		border: none;
		cursor: pointer;
		transition: color var(--duration-fast) var(--ease-default);
		&:hover { color: oklch(0.72 0.18 260); }
	}

	.np-close {
		padding: var(--space-1);
		border-radius: var(--radius-sm);
		background: none;
		border: none;
		color: oklch(0.65 0.01 250);
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover { background-color: oklch(0.3 0.01 250 / 50%); }
	}

	.np-list {
		overflow-y: auto;
		max-block-size: calc(70vh - 60px);
	}

	.np-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: var(--space-12);
		color: oklch(0.55 0.01 250);
		& :first-child { opacity: 0.5; margin-block-end: var(--space-3); }
		& p { font-size: var(--text-sm); }
	}

	.np-item {
		position: relative;
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		border-block-end: 1px solid oklch(0.38 0.01 250 / 30%);
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);

		&:hover { background-color: oklch(0.25 0.01 250 / 50%); }
		&[data-read] { opacity: 0.7; }
	}

	.np-unread-dot {
		position: absolute;
		inset-inline-start: var(--space-1);
		inset-block-start: 50%;
		transform: translateY(-50%);
		inline-size: 0.5rem;
		block-size: 0.5rem;
		background-color: oklch(0.55 0.2 260);
		border-radius: 9999px;
	}

	.np-item-inner {
		display: flex;
		gap: var(--space-3);
		padding-inline-start: var(--space-2);
	}

	.np-item-icon {
		flex-shrink: 0;
		inline-size: 2.5rem;
		block-size: 2.5rem;
		border-radius: var(--radius-lg);
		display: flex;
		align-items: center;
		justify-content: center;

		&[data-type='info'] { color: oklch(0.7 0.18 260); background-color: oklch(0.55 0.2 260 / 20%); }
		&[data-type='success'] { color: oklch(0.7 0.18 160); background-color: oklch(0.6 0.18 160 / 20%); }
		&[data-type='warning'] { color: oklch(0.75 0.16 80); background-color: oklch(0.7 0.16 80 / 20%); }
		&[data-type='error'] { color: oklch(0.7 0.2 25); background-color: oklch(0.55 0.22 25 / 20%); }
		&[data-type='system'] { color: oklch(0.7 0.18 300); background-color: oklch(0.55 0.2 300 / 20%); }
	}

	.np-item-content {
		flex: 1;
		min-inline-size: 0;
	}

	.np-item-top {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: var(--space-2);
	}

	.np-item-title {
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		font-size: var(--text-sm);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.np-item-time {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
		white-space: nowrap;
	}

	.np-item-message {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
		margin-block-start: 0.125rem;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.np-item-action {
		margin-block-start: var(--space-2);
		font-size: var(--text-xs);
		color: oklch(0.65 0.18 260);
		font-weight: var(--weight-medium);
		background: none;
		border: none;
		cursor: pointer;
		&:hover { color: oklch(0.72 0.18 260); }
	}

	.np-dismiss {
		flex-shrink: 0;
		padding: var(--space-1);
		border-radius: var(--radius-sm);
		background: none;
		border: none;
		color: oklch(0.55 0.01 250);
		opacity: 0;
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease-default);

		&:hover { background-color: oklch(0.3 0.01 250 / 50%); }
	}

	.np-item:hover .np-dismiss {
		opacity: 1;
	}

	.np-urgent-bar {
		position: absolute;
		inset-inline: 0;
		inset-block-end: 0;
		block-size: 0.125rem;
		background: linear-gradient(to right, oklch(0.55 0.24 27), oklch(0.65 0.2 55));
	}

	.np-footer {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		border-block-start: 1px solid oklch(0.38 0.01 250 / 50%);
		background-color: oklch(0.25 0.01 250 / 50%);
	}

	.np-clear-all {
		inline-size: 100%;
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
		background: none;
		border: none;
		cursor: pointer;
		padding-block: var(--space-1);
		transition: color var(--duration-fast) var(--ease-default);
		&:hover { color: oklch(1 0 0); }
	}

	@keyframes np-pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.6; }
	}
</style>
