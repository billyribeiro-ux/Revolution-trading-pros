<script lang="ts">
	/**
	 * Notification Center - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Centralized notification center with:
	 * - Real-time notification display
	 * - Read/unread tracking
	 * - Grouped by type and time
	 * - Action buttons
	 * - Mark all as read
	 *
	 * @version 1.0.0
	 */

	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import {
		notificationStore,
		notifications,
		unreadCount,
		type Notification
	} from '$lib/stores/notifications';
	import {
		IconBell,
		IconX,
		IconCheck,
		IconTrash,
		IconInfoCircle,
		IconCircleCheck,
		IconAlertTriangle,
		IconAlertCircle,
		IconSettings,
		IconChevronRight
	} from '$lib/icons';

	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let { isOpen = $bindable(false), onclose }: Props = $props();

	let activeFilter = $state<'all' | 'unread'>('all');

	// Derived filtered notifications
	let filteredNotifications = $derived(activeFilter === 'unread'
		? $notifications.filter(n => !n.read && !n.dismissed)
		: $notifications.filter(n => !n.dismissed));

	function groupByDate(notifs: Notification[]) {
		const groups: Record<string, Notification[]> = {
			Today: [],
			Yesterday: [],
			'This Week': [],
			Older: []
		};

		const now = new Date();
		const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
		const yesterday = new Date(today);
		yesterday.setDate(yesterday.getDate() - 1);
		const weekAgo = new Date(today);
		weekAgo.setDate(weekAgo.getDate() - 7);

		for (const notif of notifs) {
			const date = new Date(notif.timestamp);
			if (date >= today) {
				groups.Today.push(notif);
			} else if (date >= yesterday) {
				groups.Yesterday.push(notif);
			} else if (date >= weekAgo) {
				groups['This Week'].push(notif);
			} else {
				groups.Older.push(notif);
			}
		}

		return Object.entries(groups).filter(([_, items]) => items.length > 0);
	}

	// Derived grouped notifications
	let groupedNotifications = $derived(groupByDate(filteredNotifications));

	function getIcon(type: string) {
		switch (type) {
			case 'success': return IconCircleCheck;
			case 'warning': return IconAlertTriangle;
			case 'error': return IconAlertCircle;
			case 'system': return IconSettings;
			default: return IconInfoCircle;
		}
	}

	function getIconColor(type: string) {
		switch (type) {
			case 'success': return '#10b981';
			case 'warning': return '#f59e0b';
			case 'error': return '#ef4444';
			case 'system': return '#8b5cf6';
			default: return '#3b82f6';
		}
	}

	function formatTime(timestamp: Date) {
		const date = new Date(timestamp);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffMins = Math.floor(diffMs / 60000);
		const diffHours = Math.floor(diffMs / 3600000);

		if (diffMins < 1) return 'Just now';
		if (diffMins < 60) return `${diffMins}m ago`;
		if (diffHours < 24) return `${diffHours}h ago`;
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	function handleNotificationClick(notification: Notification) {
		notificationStore.markAsRead(notification.id);
		if (notification.action?.href) {
			window.location.href = notification.action.href;
		} else if (notification.action?.onClick) {
			notification.action.onClick();
		}
	}

	function close() {
		isOpen = false;
		onclose?.();
	}
</script>

{#if isOpen}
	<div
		class="notification-overlay"
		transition:fade={{ duration: 150 }}
		onclick={close}
		onkeydown={(e) => e.key === 'Enter' && close()}
		role="button"
		tabindex="0"
		aria-label="Close notification center"
	>
		<div
			class="notification-panel"
			transition:fly={{ x: 320, duration: 300, easing: quintOut }}
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="presentation"
		>
			<!-- Header -->
			<div class="panel-header">
				<div class="header-left">
					<IconBell size={20} />
					<h2>Notifications</h2>
					{#if $unreadCount > 0}
						<span class="unread-badge">{$unreadCount}</span>
					{/if}
				</div>
				<button class="close-btn" onclick={close}>
					<IconX size={20} />
				</button>
			</div>

			<!-- Filter Tabs -->
			<div class="filter-tabs">
				<button
					class="filter-tab"
					class:active={activeFilter === 'all'}
					onclick={() => activeFilter = 'all'}
				>
					All
				</button>
				<button
					class="filter-tab"
					class:active={activeFilter === 'unread'}
					onclick={() => activeFilter = 'unread'}
				>
					Unread
					{#if $unreadCount > 0}
						<span class="tab-count">{$unreadCount}</span>
					{/if}
				</button>
			</div>

			<!-- Actions Bar -->
			{#if $unreadCount > 0}
				<div class="actions-bar" in:fade={{ duration: 150 }}>
					<button class="action-btn" onclick={() => notificationStore.markAllAsRead()}>
						<IconCheck size={16} />
						Mark all as read
					</button>
				</div>
			{/if}

			<!-- Notifications List -->
			<div class="notifications-list">
				{#if filteredNotifications.length === 0}
					<div class="empty-state" in:scale={{ duration: 200 }}>
						<IconBell size={48} />
						<h3>No notifications</h3>
						<p>You're all caught up!</p>
					</div>
				{:else}
					{#each groupedNotifications as [group, items], groupIndex}
						<div class="notification-group" in:fly={{ y: 20, duration: 200, delay: groupIndex * 50 }}>
							<div class="group-label">{group}</div>
							{#each items as notification (notification.id)}
								{@const Icon = getIcon(notification.type)}
								<button
									class="notification-item"
									class:unread={!notification.read}
									onclick={() => handleNotificationClick(notification)}
								>
									<div class="item-icon" style="color: {getIconColor(notification.type)}">
										<Icon size={20} />
									</div>
									<div class="item-content">
										<div class="item-header">
											<span class="item-title">{notification.title}</span>
											<span class="item-time">{formatTime(notification.timestamp)}</span>
										</div>
										<p class="item-message">{notification.message}</p>
										{#if notification.action}
											<div class="item-action">
												<span>{notification.action.label}</span>
												<IconChevronRight size={14} />
											</div>
										{/if}
									</div>
									{#if !notification.read}
										<div class="unread-dot"></div>
									{/if}
								</button>
							{/each}
						</div>
					{/each}
				{/if}
			</div>

			<!-- Footer -->
			<div class="panel-footer">
				<button class="footer-btn" onclick={() => notificationStore.clear()}>
					<IconTrash size={16} />
					Clear all
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.notification-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		z-index: 9998;
	}

	.notification-panel {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		width: 100%;
		max-width: 400px;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border-left: 1px solid rgba(99, 102, 241, 0.2);
		display: flex;
		flex-direction: column;
		box-shadow: -20px 0 60px rgba(0, 0, 0, 0.4);
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #f1f5f9;
	}

	.header-left h2 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	.unread-badge {
		padding: 0.125rem 0.5rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border-radius: 10px;
		font-size: 0.75rem;
		font-weight: 600;
		color: white;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #f1f5f9;
	}

	.filter-tabs {
		display: flex;
		padding: 0 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.filter-tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1rem;
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-tab:hover {
		color: #94a3b8;
	}

	.filter-tab.active {
		color: #a5b4fc;
		border-bottom-color: #6366f1;
	}

	.tab-count {
		padding: 0.125rem 0.375rem;
		background: rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		font-size: 0.6875rem;
	}

	.actions-bar {
		display: flex;
		padding: 0.75rem 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
		background: rgba(99, 102, 241, 0.05);
	}

	.action-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #a5b4fc;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.notifications-list {
		flex: 1;
		overflow-y: auto;
		padding: 0.75rem;
	}

	.notifications-list::-webkit-scrollbar {
		width: 6px;
	}

	.notifications-list::-webkit-scrollbar-track {
		background: transparent;
	}

	.notifications-list::-webkit-scrollbar-thumb {
		background: rgba(99, 102, 241, 0.3);
		border-radius: 3px;
	}

	.notification-group {
		margin-bottom: 1rem;
	}

	.group-label {
		padding: 0.5rem 0.75rem;
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #64748b;
	}

	.notification-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		width: 100%;
		padding: 0.875rem;
		background: transparent;
		border: none;
		border-radius: 12px;
		text-align: left;
		cursor: pointer;
		transition: all 0.2s;
		position: relative;
	}

	.notification-item:hover {
		background: rgba(99, 102, 241, 0.1);
	}

	.notification-item.unread {
		background: rgba(99, 102, 241, 0.05);
	}

	.item-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		flex-shrink: 0;
	}

	.item-content {
		flex: 1;
		min-width: 0;
	}

	.item-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 0.5rem;
		margin-bottom: 0.25rem;
	}

	.item-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.item-time {
		font-size: 0.6875rem;
		color: #64748b;
		white-space: nowrap;
	}

	.item-message {
		font-size: 0.8125rem;
		color: #94a3b8;
		line-height: 1.4;
		margin: 0;
		display: -webkit-box;
		line-clamp: 2;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.item-action {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #818cf8;
	}

	.unread-dot {
		position: absolute;
		top: 0.875rem;
		right: 0.875rem;
		width: 8px;
		height: 8px;
		background: #6366f1;
		border-radius: 50%;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h3 {
		margin: 1rem 0 0.5rem;
		color: #f1f5f9;
	}

	.empty-state p {
		margin: 0;
	}

	.panel-footer {
		padding: 1rem 1.25rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.footer-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 10px;
		color: #f87171;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.footer-btn:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	@media (max-width: 480px) {
		.notification-panel {
			max-width: 100%;
		}
	}
</style>
