<!--
	RoomNotification Component
	===============================================================================
	Apple ICT 11+ Principal Engineer Implementation

	Real-time notification system for trading room start times:
	- Countdown timer to next room session
	- Push notification permission request
	- Browser notifications when room is starting
	- Integrates with room schedules API

	@version 1.0.0 - January 2026
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// ===============================================================================
	// PROPS
	// ===============================================================================

	interface Props {
		roomSlug: string;
		roomName: string;
		roomUrl?: string;
		notifyMinutesBefore?: number;
		showCountdown?: boolean;
	}

	let {
		roomSlug,
		roomName,
		roomUrl = '',
		notifyMinutesBefore = 5,
		showCountdown = true
	}: Props = $props();

	// ===============================================================================
	// STATE
	// ===============================================================================

	interface UpcomingEvent {
		id: number;
		title: string;
		trader_name: string | null;
		event_date: string;
		start_time: string;
		end_time: string;
		date_time: string;
		timezone: string;
		is_cancelled: boolean;
	}

	let nextEvent = $state<UpcomingEvent | null>(null);
	let countdown = $state('');
	let isRoomLive = $state(false);
	let hasNotificationPermission = $state(false);
	let showNotificationPrompt = $state(false);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	let countdownInterval: ReturnType<typeof setInterval> | null = null;
	let notificationSent = $state(false);

	// ===============================================================================
	// API FUNCTIONS
	// ===============================================================================

	async function fetchUpcomingEvents(): Promise<void> {
		if (!browser) return;

		try {
			const response = await fetch(`/api/schedules/${roomSlug}/upcoming?days=1`, {
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error('Failed to fetch schedule');
			}

			const data = await response.json();
			const events: UpcomingEvent[] = data.events || [];

			if (events.length > 0) {
				// Get the next upcoming event
				const now = new Date();
				nextEvent =
					events.find((event) => {
						const eventTime = new Date(event.date_time);
						return eventTime > now || isWithinSession(event);
					}) || null;
			} else {
				nextEvent = null;
			}

			isLoading = false;
		} catch (err) {
			console.error('[RoomNotification] Error fetching schedule:', err);
			error = 'Unable to load schedule';
			isLoading = false;
		}
	}

	function isWithinSession(event: UpcomingEvent): boolean {
		const now = new Date();
		const eventDate = event.event_date;
		const startTime = event.start_time;
		const endTime = event.end_time;

		const startDateTime = new Date(`${eventDate}T${startTime}`);
		const endDateTime = new Date(`${eventDate}T${endTime}`);

		return now >= startDateTime && now <= endDateTime;
	}

	// ===============================================================================
	// COUNTDOWN LOGIC
	// ===============================================================================

	function updateCountdown(): void {
		if (!nextEvent) {
			countdown = '';
			isRoomLive = false;
			return;
		}

		const now = new Date();
		const eventDateTime = new Date(nextEvent.date_time);
		const diff = eventDateTime.getTime() - now.getTime();

		// Check if currently in session
		if (isWithinSession(nextEvent)) {
			isRoomLive = true;
			countdown = 'LIVE NOW';
			return;
		}

		if (diff <= 0) {
			// Event has passed, fetch new schedule
			fetchUpcomingEvents();
			return;
		}

		isRoomLive = false;

		// Calculate countdown
		const hours = Math.floor(diff / (1000 * 60 * 60));
		const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
		const seconds = Math.floor((diff % (1000 * 60)) / 1000);

		if (hours > 0) {
			countdown = `${hours}h ${minutes}m`;
		} else if (minutes > 0) {
			countdown = `${minutes}m ${seconds}s`;
		} else {
			countdown = `${seconds}s`;
		}

		// Send notification when approaching start time
		const minutesUntilStart = diff / (1000 * 60);
		if (
			minutesUntilStart <= notifyMinutesBefore &&
			minutesUntilStart > 0 &&
			!notificationSent &&
			hasNotificationPermission
		) {
			sendNotification();
			notificationSent = true;
		}
	}

	// ===============================================================================
	// NOTIFICATION LOGIC
	// ===============================================================================

	function checkNotificationPermission(): void {
		if (!browser || !('Notification' in window)) return;

		hasNotificationPermission = Notification.permission === 'granted';
		showNotificationPrompt = Notification.permission === 'default';
	}

	async function requestNotificationPermission(): Promise<void> {
		if (!browser || !('Notification' in window)) return;

		try {
			const permission = await Notification.requestPermission();
			hasNotificationPermission = permission === 'granted';
			showNotificationPrompt = false;
		} catch (err) {
			console.error('[RoomNotification] Error requesting permission:', err);
		}
	}

	function sendNotification(): void {
		if (!browser || !hasNotificationPermission || !nextEvent) return;

		try {
			const notification = new Notification(`${roomName} Starting Soon!`, {
				body: `${nextEvent.title} is starting in ${notifyMinutesBefore} minutes.${nextEvent.trader_name ? ` With ${nextEvent.trader_name}` : ''}`,
				icon: '/favicon.png',
				badge: '/favicon.png',
				tag: `room-${roomSlug}-${nextEvent.id}`,
				requireInteraction: true
			});

			notification.onclick = () => {
				window.focus();
				if (roomUrl) {
					window.location.href = roomUrl;
				}
				notification.close();
			};

			// Auto-close after 30 seconds
			setTimeout(() => notification.close(), 30000);
		} catch (err) {
			console.error('[RoomNotification] Error sending notification:', err);
		}
	}

	// ===============================================================================
	// LIFECYCLE
	// ===============================================================================

	onMount(() => {
		checkNotificationPermission();
		fetchUpcomingEvents();

		// Update countdown every second
		countdownInterval = setInterval(updateCountdown, 1000);

		// Refresh schedule every 5 minutes
		const scheduleInterval = setInterval(fetchUpcomingEvents, 5 * 60 * 1000);

		return () => {
			if (countdownInterval) clearInterval(countdownInterval);
			clearInterval(scheduleInterval);
		};
	});

	onDestroy(() => {
		if (countdownInterval) clearInterval(countdownInterval);
	});

	// ===============================================================================
	// COMPUTED
	// ===============================================================================

	const formattedEventTime = $derived.by(() => {
		if (!nextEvent) return '';
		try {
			const date = new Date(nextEvent.date_time);
			return date.toLocaleTimeString('en-US', {
				hour: 'numeric',
				minute: '2-digit',
				timeZoneName: 'short'
			});
		} catch {
			return nextEvent.start_time;
		}
	});
</script>

{#if isLoading}
	<div class="room-notification room-notification--loading">
		<div class="notification-icon">
			<RtpIcon name="clock" size={20} />
		</div>
		<span class="notification-text">Loading schedule...</span>
	</div>
{:else if error}
	<div class="room-notification room-notification--error">
		<div class="notification-icon">
			<RtpIcon name="alert-circle" size={20} />
		</div>
		<span class="notification-text">{error}</span>
	</div>
{:else if nextEvent}
	<div
		class="room-notification"
		class:room-notification--live={isRoomLive}
		class:room-notification--soon={!isRoomLive && countdown}
	>
		{#if isRoomLive}
			<div class="notification-badge live-badge">
				<span class="pulse"></span>
				LIVE
			</div>
			<div class="notification-content">
				<span class="notification-title">{nextEvent.title}</span>
				{#if nextEvent.trader_name}
					<span class="notification-trader">with {nextEvent.trader_name}</span>
				{/if}
			</div>
			{#if roomUrl}
				<a href={roomUrl} class="btn btn-join">Join Now</a>
			{/if}
		{:else if showCountdown && countdown}
			<div class="notification-icon">
				<RtpIcon name="clock" size={20} />
			</div>
			<div class="notification-content">
				<span class="notification-label">Next Session</span>
				<span class="notification-title">{nextEvent.title}</span>
				<span class="notification-time">
					{formattedEventTime}
					{#if nextEvent.trader_name}
						with {nextEvent.trader_name}
					{/if}
				</span>
			</div>
			<div class="notification-countdown">
				<span class="countdown-label">Starts in</span>
				<span class="countdown-value">{countdown}</span>
			</div>
		{/if}
	</div>

	{#if showNotificationPrompt && !isRoomLive}
		<button class="notification-prompt" onclick={requestNotificationPermission}>
			<RtpIcon name="bell" size={16} />
			<span>Enable notifications to be reminded when the room starts</span>
		</button>
	{/if}
{:else}
	<div class="room-notification room-notification--empty">
		<div class="notification-icon">
			<RtpIcon name="calendar" size={20} />
		</div>
		<span class="notification-text">No upcoming sessions today</span>
	</div>
{/if}

<style>
	/* ===============================================================================
	 * ROOM NOTIFICATION - 2026 Mobile-First Design
	 * =============================================================================== */

	.room-notification {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		font-family: 'Montserrat', sans-serif;
		transition: all 0.2s ease;
	}

	.room-notification--live {
		background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
		border-color: #dc2626;
		color: #fff;
	}

	.room-notification--soon {
		background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
		border-color: #22c55e;
	}

	.room-notification--loading,
	.room-notification--error,
	.room-notification--empty {
		background: #f8fafc;
		color: #64748b;
	}

	.room-notification--error {
		border-color: #fecaca;
		color: #dc2626;
	}

	/* Icon */
	.notification-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		width: 40px;
		height: 40px;
		background: rgba(20, 62, 89, 0.1);
		border-radius: 10px;
		color: #143e59;
	}

	.room-notification--soon .notification-icon {
		background: rgba(34, 197, 94, 0.15);
		color: #16a34a;
	}

	/* Live Badge */
	.live-badge {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 20px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.pulse {
		width: 8px;
		height: 8px;
		background: #fff;
		border-radius: 50%;
		animation: pulse 1.5s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.7;
			transform: scale(1.2);
		}
	}

	/* Content */
	.notification-content {
		flex: 1;
		min-width: 0;
	}

	.notification-label {
		display: block;
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #64748b;
		margin-bottom: 2px;
	}

	.room-notification--soon .notification-label {
		color: #16a34a;
	}

	.notification-title {
		display: block;
		font-size: 14px;
		font-weight: 600;
		color: #1e293b;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.room-notification--live .notification-title {
		color: #fff;
	}

	.notification-trader,
	.notification-time {
		display: block;
		font-size: 12px;
		color: #64748b;
		margin-top: 2px;
	}

	.room-notification--live .notification-trader {
		color: rgba(255, 255, 255, 0.8);
	}

	.notification-text {
		font-size: 13px;
		color: inherit;
	}

	/* Countdown */
	.notification-countdown {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 8px 12px;
		background: #fff;
		border-radius: 8px;
		border: 1px solid #e2e8f0;
		min-width: 70px;
	}

	.countdown-label {
		font-size: 9px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #64748b;
		margin-bottom: 2px;
	}

	.countdown-value {
		font-size: 14px;
		font-weight: 700;
		color: #16a34a;
		font-variant-numeric: tabular-nums;
	}

	/* Join Button */
	.btn-join {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		padding: 10px 20px;
		background: #fff;
		color: #dc2626;
		font-size: 13px;
		font-weight: 700;
		text-decoration: none;
		border-radius: 8px;
		transition: all 0.2s ease;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.btn-join:hover {
		background: #fef2f2;
		transform: translateY(-1px);
	}

	.btn-join:focus-visible {
		outline: 2px solid #fff;
		outline-offset: 2px;
	}

	/* Notification Prompt */
	.notification-prompt {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		margin-top: 8px;
		padding: 10px 14px;
		background: #fffbeb;
		border: 1px solid #fcd34d;
		border-radius: 8px;
		font-size: 12px;
		color: #92400e;
		cursor: pointer;
		transition: all 0.2s ease;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.notification-prompt:hover {
		background: #fef3c7;
	}

	.notification-prompt:focus-visible {
		outline: 2px solid #f59e0b;
		outline-offset: 2px;
	}

	/* ===============================================================================
	 * RESPONSIVE
	 * =============================================================================== */

	@media (min-width: 640px) {
		.room-notification {
			padding: 14px 20px;
		}

		.notification-title {
			font-size: 15px;
		}

		.notification-countdown {
			padding: 10px 16px;
			min-width: 80px;
		}

		.countdown-value {
			font-size: 16px;
		}
	}

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.pulse {
			animation: none;
		}

		.room-notification,
		.btn-join,
		.notification-prompt {
			transition: none;
		}
	}
</style>
