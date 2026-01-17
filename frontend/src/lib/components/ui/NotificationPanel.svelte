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
		IconBell,
		IconX,
		IconAlertCircle,
		IconInfoCircle,
		IconCircleCheck,
		IconAlertTriangle,
		IconSettings
	} from '$lib/icons';
	import {
		notificationStore,
		notifications,
		unreadCount,
		type Notification,
		type NotificationType
	} from '$lib/stores/notifications.svelte';

	interface Props {
		position?: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
		onclick?: (notification: Notification) => void;
	}

	let { position = 'top-right', onclick }: Props = $props();

	let isOpen = $state(false);
	let bellRef: HTMLButtonElement | null = $state(null);
	let panelRef: HTMLDivElement | null = $state(null);

	const iconMap: Record<NotificationType, typeof IconInfoCircle> = {
		info: IconInfoCircle,
		success: IconCircleCheck,
		warning: IconAlertTriangle,
		error: IconAlertCircle,
		system: IconSettings
	};

	const colorMap: Record<NotificationType, string> = {
		info: 'text-blue-400 bg-blue-500/20',
		success: 'text-emerald-400 bg-emerald-500/20',
		warning: 'text-amber-400 bg-amber-500/20',
		error: 'text-red-400 bg-red-500/20',
		system: 'text-purple-400 bg-purple-500/20'
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

	let positionClasses = $derived(
		{
			'top-right': 'right-0 top-full mt-2',
			'top-left': 'left-0 top-full mt-2',
			'bottom-right': 'right-0 bottom-full mb-2',
			'bottom-left': 'left-0 bottom-full mb-2'
		}[position]
	);
</script>

<div class="relative">
	<!-- Bell Button -->
	<button
		bind:this={bellRef}
		onclick={toggle}
		class="relative p-2 rounded-lg hover:bg-slate-700/50 transition-colors duration-200"
		aria-label="Notifications"
		aria-expanded={isOpen}
	>
		<IconBell size={24} class="text-slate-400 hover:text-white transition-colors" />

		<!-- Unread Badge -->
		{#if $unreadCount > 0}
			<span
				class="absolute -top-1 -right-1 min-w-[20px] h-5 px-1.5 flex items-center justify-center
					bg-red-500 text-white text-xs font-bold rounded-full animate-pulse"
			>
				{$unreadCount > 99 ? '99+' : $unreadCount}
			</span>
		{/if}
	</button>

	<!-- Dropdown Panel -->
	{#if isOpen}
		<div
			bind:this={panelRef}
			transition:fly={{ y: -10, duration: 200 }}
			class="absolute {positionClasses} w-96 max-h-[70vh] bg-slate-900/95 backdrop-blur-xl
				border border-slate-700/50 rounded-2xl shadow-2xl overflow-hidden z-50"
		>
			<!-- Header -->
			<div
				class="flex items-center justify-between px-4 py-3 border-b border-slate-700/50 bg-slate-800/50"
			>
				<h3 class="font-semibold text-white">Notifications</h3>
				<div class="flex items-center gap-2">
					{#if $unreadCount > 0}
						<button
							onclick={() => notificationStore.markAllAsRead()}
							class="text-xs text-indigo-400 hover:text-indigo-300 transition-colors"
						>
							Mark all read
						</button>
					{/if}
					<button
						onclick={close}
						class="p-1 rounded hover:bg-slate-700/50 transition-colors"
						aria-label="Close"
					>
						<IconX size={18} class="text-slate-400" />
					</button>
				</div>
			</div>

			<!-- Notification List -->
			<div class="overflow-y-auto max-h-[calc(70vh-60px)]">
				{#if $notifications.length === 0}
					<div class="flex flex-col items-center justify-center py-12 text-slate-500">
						<IconBell size={48} class="opacity-50 mb-3" />
						<p class="text-sm">No notifications yet</p>
					</div>
				{:else}
					{#each $notifications.filter((n) => !n.dismissed) as notification (notification.id)}
						{@const NotifIcon = iconMap[notification.type]}
						<div
							transition:fade={{ duration: 150 }}
							class="group relative px-4 py-3 border-b border-slate-700/30
								hover:bg-slate-800/50 transition-colors cursor-pointer
								{notification.read ? 'opacity-70' : ''}"
							onclick={() => handleNotificationClick(notification)}
							onkeypress={(e: KeyboardEvent) =>
								e.key === 'Enter' && handleNotificationClick(notification)}
							role="button"
							tabindex="0"
						>
							<!-- Unread indicator -->
							{#if !notification.read}
								<div
									class="absolute left-1 top-1/2 -translate-y-1/2 w-2 h-2 bg-indigo-500 rounded-full"
								></div>
							{/if}

							<div class="flex gap-3 pl-2">
								<!-- Icon -->
								<div
									class="flex-shrink-0 w-10 h-10 rounded-lg {colorMap[
										notification.type
									]} flex items-center justify-center"
								>
									<NotifIcon size={20} />
								</div>

								<!-- Content -->
								<div class="flex-1 min-w-0">
									<div class="flex items-start justify-between gap-2">
										<p class="font-medium text-white text-sm truncate">{notification.title}</p>
										<span class="text-xs text-slate-500 whitespace-nowrap">
											{formatTime(notification.timestamp)}
										</span>
									</div>
									<p class="text-sm text-slate-400 mt-0.5 line-clamp-2">{notification.message}</p>

									{#if notification.action}
										<button
											onclick={(e: MouseEvent) => {
												e.stopPropagation();
												handleNotificationClick(notification);
											}}
											class="mt-2 text-xs text-indigo-400 hover:text-indigo-300 font-medium"
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
									class="flex-shrink-0 p-1 rounded opacity-0 group-hover:opacity-100
										hover:bg-slate-700/50 transition-all"
									aria-label="Dismiss"
								>
									<IconX size={16} class="text-slate-500" />
								</button>
							</div>

							<!-- Priority indicator for urgent -->
							{#if notification.priority === 'urgent'}
								<div
									class="absolute inset-x-0 bottom-0 h-0.5 bg-gradient-to-r from-red-500 to-orange-500"
								></div>
							{/if}
						</div>
					{/each}
				{/if}
			</div>

			<!-- Footer -->
			{#if $notifications.length > 0}
				<div class="px-4 py-2 border-t border-slate-700/50 bg-slate-800/50">
					<button
						onclick={() => notificationStore.clear()}
						class="w-full text-sm text-slate-400 hover:text-white transition-colors py-1"
					>
						Clear all notifications
					</button>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>
