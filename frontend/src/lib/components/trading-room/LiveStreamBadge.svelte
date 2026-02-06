<!--
	LiveStreamBadge Component
	===============================================================================
	Apple ICT 11+ Principal Engineer Implementation

	Visual indicator showing when a trading room is currently live.
	- Pulsing animation for live status
	- Viewer count display (optional)
	- Click to join functionality
	- Integrates with WebSocket for real-time updates

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
		roomName?: string;
		roomUrl?: string;
		variant?: 'default' | 'compact' | 'minimal';
		showViewerCount?: boolean;
		showJoinButton?: boolean;
		class?: string;
	}

	let {
		roomSlug,
		roomName = '',
		roomUrl = '',
		variant = 'default',
		showViewerCount = false,
		showJoinButton = true,
		class: className = ''
	}: Props = $props();

	// ===============================================================================
	// STATE
	// ===============================================================================

	let isLive = $state(false);
	let viewerCount = $state(0);
	let isLoading = $state(true);
	let ws: WebSocket | null = null;

	// ===============================================================================
	// WEBSOCKET CONNECTION
	// ===============================================================================

	function connectWebSocket(): void {
		if (!browser) return;

		try {
			const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
			const wsUrl = `${protocol}//${window.location.host}/api/realtime/ws?rooms=${roomSlug}`;

			ws = new WebSocket(wsUrl);

			ws.onopen = () => {
				console.log(`[LiveStreamBadge] Connected to ${roomSlug}`);
				isLoading = false;
			};

			ws.onmessage = (event) => {
				try {
					const message = JSON.parse(event.data);

					if (message.type === 'Connected') {
						// Connection established
						isLoading = false;
					} else if (message.type === 'StatsUpdated' && message.payload?.stats) {
						// Update viewer count if available
						if (message.payload.stats.active_connections !== undefined) {
							viewerCount = message.payload.stats.active_connections;
						}
					} else if (message.type === 'RoomStatus') {
						// Room status update
						isLive = message.payload?.is_live ?? false;
						if (message.payload?.viewer_count !== undefined) {
							viewerCount = message.payload.viewer_count;
						}
					}
				} catch (err) {
					console.error('[LiveStreamBadge] Error parsing message:', err);
				}
			};

			ws.onclose = () => {
				console.log(`[LiveStreamBadge] Disconnected from ${roomSlug}`);
				// Attempt to reconnect after 5 seconds
				setTimeout(connectWebSocket, 5000);
			};

			ws.onerror = (err) => {
				console.error('[LiveStreamBadge] WebSocket error:', err);
				isLoading = false;
			};
		} catch (err) {
			console.error('[LiveStreamBadge] Error connecting:', err);
			isLoading = false;
		}
	}

	// Fallback: Check schedule API for live status
	async function checkSchedule(): Promise<void> {
		if (!browser) return;

		try {
			const response = await fetch(`/api/schedules/${roomSlug}/upcoming?days=1`, {
				credentials: 'include'
			});

			if (!response.ok) return;

			const data = await response.json();
			const events = data.events || [];

			// Check if any event is currently in session
			const now = new Date();
			isLive = events.some(
				(event: { event_date: string; start_time: string; end_time: string }) => {
					const startDateTime = new Date(`${event.event_date}T${event.start_time}`);
					const endDateTime = new Date(`${event.event_date}T${event.end_time}`);
					return now >= startDateTime && now <= endDateTime;
				}
			);

			isLoading = false;
		} catch (err) {
			console.error('[LiveStreamBadge] Error checking schedule:', err);
			isLoading = false;
		}
	}

	// ===============================================================================
	// LIFECYCLE
	// ===============================================================================

	onMount(() => {
		// First check schedule, then connect WebSocket for real-time updates
		checkSchedule();

		// Only connect WebSocket if we want real-time viewer count
		if (showViewerCount) {
			connectWebSocket();
		}

		// Refresh schedule check every minute
		const scheduleInterval = setInterval(checkSchedule, 60 * 1000);

		return () => {
			clearInterval(scheduleInterval);
		};
	});

	onDestroy(() => {
		if (ws) {
			ws.close();
			ws = null;
		}
	});
</script>

{#if isLoading}
	<!-- Loading state - show nothing or skeleton -->
	{#if variant !== 'minimal'}
		<div class="live-badge live-badge--loading {className}" data-variant={variant}>
			<span class="badge-skeleton"></span>
		</div>
	{/if}
{:else if isLive}
	{#if variant === 'minimal'}
		<span class="live-indicator {className}" title="{roomName || 'Room'} is LIVE">
			<span class="pulse-dot"></span>
		</span>
	{:else if variant === 'compact'}
		<div class="live-badge live-badge--compact live-badge--live {className}">
			<span class="pulse-dot"></span>
			<span class="badge-text">LIVE</span>
			{#if showViewerCount && viewerCount > 0}
				<span class="viewer-count">{viewerCount}</span>
			{/if}
		</div>
	{:else}
		<div class="live-badge live-badge--live {className}">
			<div class="badge-content">
				<span class="pulse-dot"></span>
				<span class="badge-text">LIVE NOW</span>
				{#if showViewerCount && viewerCount > 0}
					<span class="viewer-count">
						<RtpIcon name="users" size={12} />
						{viewerCount}
					</span>
				{/if}
			</div>
			{#if showJoinButton && roomUrl}
				<a href={roomUrl} class="join-btn">
					<RtpIcon name="play" size={14} />
					Join
				</a>
			{/if}
		</div>
	{/if}
{:else if variant !== 'minimal'}
	<!-- Offline state - only show for non-minimal variants -->
	<div class="live-badge live-badge--offline {className}" data-variant={variant}>
		<span class="badge-text">Offline</span>
	</div>
{/if}

<style>
	/* ===============================================================================
	 * LIVE STREAM BADGE - 2026 Mobile-First Design
	 * =============================================================================== */

	/* Base badge */
	.live-badge {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 14px;
		background: #f1f5f9;
		border-radius: 20px;
		font-family: 'Montserrat', sans-serif;
		font-size: 12px;
		font-weight: 600;
		transition: all 0.2s ease;
	}

	/* Live state */
	.live-badge--live {
		background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
		color: #fff;
		box-shadow: 0 4px 12px rgba(220, 38, 38, 0.3);
	}

	/* Compact variant */
	.live-badge--compact {
		padding: 6px 10px;
		font-size: 10px;
		gap: 6px;
	}

	/* Offline state */
	.live-badge--offline {
		background: #e2e8f0;
		color: #64748b;
	}

	/* Loading state */
	.live-badge--loading {
		background: #f1f5f9;
		min-width: 80px;
	}

	.badge-skeleton {
		display: block;
		width: 100%;
		height: 14px;
		background: linear-gradient(90deg, #e2e8f0 25%, #f1f5f9 50%, #e2e8f0 75%);
		background-size: 200% 100%;
		border-radius: 4px;
		animation: skeleton-shimmer 1.5s infinite;
	}

	@keyframes skeleton-shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Badge content wrapper */
	.badge-content {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.badge-text {
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	/* Pulsing dot indicator */
	.pulse-dot {
		width: 8px;
		height: 8px;
		background: #fff;
		border-radius: 50%;
		animation: pulse-live 1.5s ease-in-out infinite;
		flex-shrink: 0;
	}

	.live-badge--offline .pulse-dot {
		display: none;
	}

	@keyframes pulse-live {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
			box-shadow: 0 0 0 0 rgba(255, 255, 255, 0.4);
		}
		50% {
			opacity: 0.8;
			transform: scale(1.1);
			box-shadow: 0 0 0 4px rgba(255, 255, 255, 0);
		}
	}

	/* Minimal indicator (just a dot) */
	.live-indicator {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 12px;
		height: 12px;
		cursor: default;
	}

	.live-indicator .pulse-dot {
		width: 10px;
		height: 10px;
		background: #dc2626;
		box-shadow: 0 0 0 0 rgba(220, 38, 38, 0.4);
		animation: pulse-minimal 1.5s ease-in-out infinite;
	}

	@keyframes pulse-minimal {
		0%,
		100% {
			box-shadow: 0 0 0 0 rgba(220, 38, 38, 0.4);
		}
		50% {
			box-shadow: 0 0 0 4px rgba(220, 38, 38, 0);
		}
	}

	/* Viewer count */
	.viewer-count {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 2px 8px;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 10px;
		font-size: 11px;
		font-weight: 500;
	}

	.live-badge--compact .viewer-count {
		padding: 2px 6px;
		font-size: 10px;
	}

	/* Join button */
	.join-btn {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 6px 12px;
		background: #fff;
		color: #dc2626;
		font-size: 12px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 14px;
		margin-left: 4px;
		transition: all 0.2s ease;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.join-btn:hover {
		background: #fef2f2;
		transform: translateY(-1px);
	}

	.join-btn:focus-visible {
		outline: 2px solid #fff;
		outline-offset: 2px;
	}

	/* ===============================================================================
	 * RESPONSIVE
	 * =============================================================================== */

	@media (min-width: 640px) {
		.live-badge {
			padding: 10px 16px;
			font-size: 13px;
		}

		.live-badge--compact {
			padding: 6px 12px;
			font-size: 11px;
		}

		.viewer-count {
			font-size: 12px;
		}
	}

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.pulse-dot,
		.live-indicator .pulse-dot {
			animation: none;
		}

		.badge-skeleton {
			animation: none;
		}

		.live-badge,
		.join-btn {
			transition: none;
		}
	}
</style>
