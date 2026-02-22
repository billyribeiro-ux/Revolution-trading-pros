<!--
	Trading Room Sidebar - Schedule + Quick Links
	Pixel-perfect extraction from WordPress DASHMAIN
	
	ICT 11+ Apple Principal Engineer Implementation:
	- Backend API integration (database-backed schedules)
	- Per-room schedule support (Day Trading, Swing Trading, Small Account Mentorship)
	- Exponential backoff retry logic (3 attempts)
	- Circuit breaker pattern (prevents API abuse)
	- localStorage caching with 5-minute TTL
	- Stale-while-revalidate pattern
	- Structured logging for monitoring
	- Performance metrics tracking
	- Graceful degradation with cached fallback
	- Error boundary with user-friendly messages
	- Accessibility improvements (ARIA live regions)
	
	@version 3.0.0 - ICT 11+ Backend Integration
	@author Revolution Trading Pros
-->
<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPE DEFINITIONS - ICT 11+ TypeScript Best Practice
	// ═══════════════════════════════════════════════════════════════════════════

	interface ScheduleEvent {
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

	interface Props {
		planSlug?: string;
	}

	interface CacheEntry {
		data: ScheduleEvent[];
		timestamp: number;
		ttl: number;
	}

	interface CalendarAPILog {
		timestamp: string;
		action: 'api_call' | 'cache_hit' | 'cache_stale' | 'error' | 'retry' | 'circuit_open';
		duration?: number;
		error?: string;
		eventCount?: number;
		attempt?: number;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE MANAGEMENT - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let scheduleEvents = $state<ScheduleEvent[]>([]);
	let scheduleLoading = $state(true);
	let scheduleError = $state(false);
	let isStaleData = $state(false);
	let _lastFetchTime = $state<number>(0);

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Per-room schedule support
	// ═══════════════════════════════════════════════════════════════════════════

	let props: Props = $props();

	// Derived props with defaults
	let planSlug = $derived(props.planSlug ?? 'day-trading-room');

	// ═══════════════════════════════════════════════════════════════════════════
	// CONFIGURATION - ICT 11+ Enterprise Settings
	// ═══════════════════════════════════════════════════════════════════════════

	// Backend API configuration
	// ICT 11+ CORB Fix: Use same-origin SvelteKit proxy to prevent CORB
	const API_CONFIG = {
		baseUrl: '/api/schedules',
		daysAhead: 7
	};

	// ICT 11+ Retry configuration
	const RETRY_CONFIG = {
		maxAttempts: 3,
		baseDelay: 1000, // 1 second
		maxDelay: 10000, // 10 seconds
		backoffMultiplier: 2
	};

	// ICT 11+ Cache configuration
	const CACHE_CONFIG = {
		key: 'rtp_trading_room_schedule',
		ttl: 300000, // 5 minutes
		staleThreshold: 600000 // 10 minutes (serve stale if API fails)
	};

	// ICT 11+ Circuit breaker configuration
	const CIRCUIT_BREAKER_CONFIG = {
		failureThreshold: 3,
		resetTimeout: 60000 // 1 minute
	};

	// Circuit breaker state
	let circuitBreakerFailures = 0;
	let circuitBreakerLastFailure = 0;

	// Quick Links data - now room-specific
	interface QuickLink {
		text: string;
		href: string;
		external: boolean;
		download?: boolean;
	}

	const quickLinks = $derived<QuickLink[]>([
		{
			text: '⭐ My Favorites',
			href: `/dashboard/${planSlug}/favorites`,
			external: false
		},
		{
			text: '📥 Export Watchlist',
			href: `/api/export/watchlist?room_slug=${planSlug}&format=csv`,
			external: false,
			download: true
		},
		{
			text: 'Support',
			href: 'https://intercom.help/simpler-trading/en/',
			external: true
		},
		{
			text: 'Platform Tutorials',
			href: '/tutorials',
			external: false
		}
	]);

	// ═══════════════════════════════════════════════════════════════════════════
	// ICT 11+ LOGGING - Structured Logging for Monitoring
	// ═══════════════════════════════════════════════════════════════════════════

	function logCalendarAPI(log: CalendarAPILog): void {
		if (browser && typeof console !== 'undefined') {
			const logEntry = {
				...log,
				component: 'TradingRoomSidebar',
				version: '2.0.0-ICT11'
			};
			logger.info('[ICT11 Calendar API]', logEntry);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// ICT 11+ CACHING - localStorage with TTL
	// ═══════════════════════════════════════════════════════════════════════════

	function getCachedSchedule(): ScheduleEvent[] | null {
		if (!browser) return null;

		try {
			const cached = localStorage.getItem(CACHE_CONFIG.key);
			if (!cached) return null;

			const entry: CacheEntry = JSON.parse(cached);
			const age = Date.now() - entry.timestamp;

			// Fresh cache (within TTL)
			if (age < CACHE_CONFIG.ttl) {
				logCalendarAPI({
					timestamp: new Date().toISOString(),
					action: 'cache_hit',
					eventCount: entry.data.length
				});
				return entry.data;
			}

			// Stale cache (beyond TTL but within stale threshold)
			if (age < CACHE_CONFIG.staleThreshold) {
				logCalendarAPI({
					timestamp: new Date().toISOString(),
					action: 'cache_stale',
					eventCount: entry.data.length
				});
				isStaleData = true;
				return entry.data; // Serve stale while revalidating
			}

			return null; // Cache too old
		} catch (_error) {
			logCalendarAPI({
				timestamp: new Date().toISOString(),
				action: 'error',
				error: 'Cache read failed'
			});
			return null;
		}
	}

	function setCachedSchedule(events: ScheduleEvent[]): void {
		if (!browser) return;

		try {
			const entry: CacheEntry = {
				data: events,
				timestamp: Date.now(),
				ttl: CACHE_CONFIG.ttl
			};
			localStorage.setItem(CACHE_CONFIG.key, JSON.stringify(entry));
		} catch (_error) {
			logCalendarAPI({
				timestamp: new Date().toISOString(),
				action: 'error',
				error: 'Cache write failed'
			});
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// ICT 11+ CIRCUIT BREAKER - Prevent API Abuse
	// ═══════════════════════════════════════════════════════════════════════════

	function isCircuitOpen(): boolean {
		if (circuitBreakerFailures < CIRCUIT_BREAKER_CONFIG.failureThreshold) {
			return false;
		}

		const timeSinceLastFailure = Date.now() - circuitBreakerLastFailure;
		if (timeSinceLastFailure > CIRCUIT_BREAKER_CONFIG.resetTimeout) {
			// Reset circuit breaker
			circuitBreakerFailures = 0;
			return false;
		}

		return true;
	}

	function recordFailure(): void {
		circuitBreakerFailures++;
		circuitBreakerLastFailure = Date.now();
	}

	function recordSuccess(): void {
		circuitBreakerFailures = 0;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// ICT 11+ RETRY LOGIC - Exponential Backoff
	// ═══════════════════════════════════════════════════════════════════════════

	async function sleep(ms: number): Promise<void> {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	function calculateBackoff(attempt: number): number {
		const delay = RETRY_CONFIG.baseDelay * Math.pow(RETRY_CONFIG.backoffMultiplier, attempt);
		return Math.min(delay, RETRY_CONFIG.maxDelay);
	}

	async function fetchScheduleWithRetry(attempt: number = 0): Promise<ScheduleEvent[]> {
		const startTime = Date.now();

		try {
			// Fetch from backend API
			const response = await fetch(
				`${API_CONFIG.baseUrl}/${planSlug}/upcoming?days=${API_CONFIG.daysAhead}`,
				{ credentials: 'include' }
			);

			if (!response.ok) {
				throw new Error(`HTTP ${response.status}: ${response.statusText}`);
			}

			const data = await response.json();
			const events: ScheduleEvent[] = data.events || [];

			const duration = Date.now() - startTime;
			logCalendarAPI({
				timestamp: new Date().toISOString(),
				action: 'api_call',
				duration,
				eventCount: events.length,
				attempt: attempt + 1
			});

			recordSuccess();
			return events;
		} catch (error) {
			const duration = Date.now() - startTime;
			const errorMessage = error instanceof Error ? error.message : 'Unknown error';

			logCalendarAPI({
				timestamp: new Date().toISOString(),
				action: attempt < RETRY_CONFIG.maxAttempts - 1 ? 'retry' : 'error',
				duration,
				error: errorMessage,
				attempt: attempt + 1
			});

			// Retry with exponential backoff
			if (attempt < RETRY_CONFIG.maxAttempts - 1) {
				const backoffDelay = calculateBackoff(attempt);
				await sleep(backoffDelay);
				return fetchScheduleWithRetry(attempt + 1);
			}

			recordFailure();
			throw error;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// ICT 11+ MAIN LOAD FUNCTION - Stale-While-Revalidate Pattern
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadSchedule() {
		if (!browser) return;

		// Check cache first (stale-while-revalidate)
		const cached = getCachedSchedule();
		if (cached && cached.length > 0) {
			scheduleEvents = cached;
			scheduleLoading = false;
			// Continue to fetch fresh data in background
		}

		// Check circuit breaker
		if (isCircuitOpen()) {
			logCalendarAPI({
				timestamp: new Date().toISOString(),
				action: 'circuit_open',
				error: 'Circuit breaker open - using cached data'
			});

			if (!cached || cached.length === 0) {
				scheduleError = true;
				scheduleLoading = false;
			}
			return;
		}

		try {
			// Fetch with retry logic
			const events = await fetchScheduleWithRetry();

			// Update state
			scheduleEvents = events;
			scheduleLoading = false;
			scheduleError = false;
			isStaleData = false;
			_lastFetchTime = Date.now();

			// Update cache
			setCachedSchedule(events);
		} catch (_error) {
			// Graceful degradation: Use cached data if available
			if (cached && cached.length > 0) {
				scheduleEvents = cached;
				isStaleData = true;
				scheduleError = false; // Don't show error if we have cached data
			} else {
				scheduleError = true;
			}
			scheduleLoading = false;
		}
	}

	// Format date for display
	function formatEventDate(dateTimeString: string): string {
		const date = new Date(dateTimeString);
		const options: Intl.DateTimeFormatOptions = {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: 'numeric',
			minute: 'numeric',
			timeZoneName: 'short'
		};
		return date.toLocaleString('en-US', options);
	}

	onMount(() => {
		loadSchedule();
	});
</script>

<!-- Trading Room Sidebar -->
<aside class="dashboard__content-sidebar">
	<!-- Trading Room Schedule Section -->
	<section class="content-sidebar__section">
		<h4 class="content-sidebar__heading">
			Trading Room Schedule
			<p class="pssubject">Schedule is subject to change.</p>
		</h4>
		<div class="script-container">
			<div class="room-sched" role="region" aria-live="polite" aria-label="Trading room schedule">
				{#if scheduleLoading}
					<p class="schedule-loading">Loading schedule...</p>
				{:else if scheduleError}
					<p class="schedule-error">Unable to load schedule. Please try again later.</p>
				{:else if scheduleEvents.length === 0}
					<p class="schedule-empty">No upcoming events scheduled.</p>
				{:else}
					{#if isStaleData}
						<div class="schedule-stale-notice" role="status">
							<small>⚠️ Showing cached schedule (updating in background)</small>
						</div>
					{/if}
					{#each scheduleEvents as event}
						<div class="schedule-event">
							<h4>{event.title}</h4>
							<span>{formatEventDate(event.date_time)}</span>
							{#if event.trader_name}
								<span class="event-trader">with {event.trader_name}</span>
							{/if}
						</div>
					{/each}
				{/if}
			</div>
		</div>
	</section>

	<!-- Quick Links Section -->
	<section class="content-sidebar__section">
		<h4 class="content-sidebar__heading">Quick Links</h4>
		<ul class="link-list">
			{#each quickLinks as link}
				<li>
					<a
						href={link.href}
						target={link.external ? '_blank' : undefined}
						rel={link.external ? 'noopener noreferrer' : undefined}
						download={link.download ? '' : undefined}
					>
						{link.text}
					</a>
				</li>
			{/each}
		</ul>
	</section>
</aside>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * TRADING ROOM SIDEBAR - 2026 Mobile-First Responsive Design
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch Targets: 44x44px minimum
	 * Safe Areas: env(safe-area-inset-*) for notched devices
	 * Features: Slide-in panel on mobile, scrollable content
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-First Base - Full width, stacked below main content */
	.dashboard__content-sidebar {
		width: 100%;
		background: #fff;
		border-radius: 0;
		box-shadow: none;
		overflow: visible;
		padding-bottom: env(safe-area-inset-bottom, 16px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SIDEBAR SECTIONS - Platform Tutorials Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.content-sidebar__section {
		padding: 0;
		border-bottom: none;
	}

	.content-sidebar__section:last-child {
		border-bottom: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECTION HEADINGS - Platform Tutorials Style with Background
	 * Background: #143E59 (dark teal/navy) - matches new dashboard color scheme
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.content-sidebar__heading {
		font-size: 14px;
		font-weight: 700;
		color: #ffffff;
		margin: 0;
		padding: 12px 20px;
		font-family: 'Montserrat', var(--font-heading), sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		background-color: #143e59;
		border-bottom: 2px solid #0e2433;
	}

	.pssubject {
		font-size: 10px;
		margin-top: 8px;
		text-transform: initial;
		color: #fff;
		font-weight: 400;
		letter-spacing: normal;
		text-align: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * TRADING ROOM SCHEDULE - Platform Tutorials Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.script-container {
		padding: 20px;
		background: #ffffff;
	}

	.room-sched {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.schedule-event {
		padding-bottom: 16px;
		border-bottom: 1px solid #e5e5e5;
	}

	.schedule-event:last-child {
		padding-bottom: 0;
		border-bottom: none;
	}

	.schedule-event h4 {
		font-size: 14px;
		font-weight: 700;
		color: #333;
		margin: 0 0 6px 0;
		font-family: 'Montserrat', var(--font-heading), sans-serif;
		line-height: 1.4;
	}

	.schedule-event span {
		font-size: 12px;
		color: #666;
		font-family: 'Montserrat', var(--font-body), sans-serif;
		display: block;
		line-height: 1.5;
	}

	.schedule-event .event-trader {
		font-size: 11px;
		color: #888;
		font-style: italic;
		margin-top: 2px;
	}

	/* Schedule states */
	.schedule-loading,
	.schedule-error,
	.schedule-empty {
		font-size: 13px;
		color: #666;
		font-family: 'Montserrat', var(--font-body), sans-serif;
		text-align: center;
		padding: 20px 0;
		margin: 0;
	}

	.schedule-error {
		color: #d32f2f;
	}

	/* ICT 11+ Stale data notice */
	.schedule-stale-notice {
		background: #fff3cd;
		border: 1px solid #ffc107;
		border-radius: 4px;
		padding: 8px 12px;
		margin-bottom: 12px;
		text-align: center;
	}

	.schedule-stale-notice small {
		font-size: 11px;
		color: #856404;
		font-family: 'Montserrat', var(--font-body), sans-serif;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * QUICK LINKS - Platform Tutorials Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.link-list {
		list-style: none;
		padding: 20px;
		margin: 0;
		background: #ffffff;
	}

	.link-list li {
		margin-bottom: 12px;
	}

	.link-list li:last-child {
		margin-bottom: 0;
	}

	.link-list a {
		color: #143e59;
		text-decoration: none;
		font-size: 14px;
		font-family: 'Montserrat', var(--font-body), sans-serif;
		transition: color 0.2s ease;
		display: block;
		line-height: 1.6;
	}

	.link-list a:hover {
		color: #0e2433;
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Progressive Enhancement
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ - Small phones */
	@media (min-width: 360px) {
		.content-sidebar__heading {
			font-size: 13px;
			padding: 14px 20px;
		}

		.script-container,
		.link-list {
			padding: 16px 20px;
		}
	}

	/* sm: 640px+ - Large phones / small tablets - Two column layout */
	@media (min-width: 640px) {
		.dashboard__content-sidebar {
			display: grid;
			grid-template-columns: 1fr 1fr;
			gap: 0;
		}

		.content-sidebar__section {
			border-right: 1px solid #e5e5e5;
		}

		.content-sidebar__section:last-child {
			border-right: none;
		}
	}

	/* md: 768px+ - Tablets */
	@media (min-width: 768px) {
		.content-sidebar__heading {
			font-size: 14px;
		}

		.schedule-event h4 {
			font-size: 15px;
		}

		.link-list a {
			font-size: 14px;
			min-height: 44px;
			display: flex;
			align-items: center;
		}
	}

	/* lg: 1024px+ - Desktop - Sidebar column layout */
	@media (min-width: 1024px) {
		.dashboard__content-sidebar {
			display: block;
			flex: 0 0 280px;
			width: 280px;
			border-radius: 8px;
			box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
			max-height: calc(100dvh - 100px);
			overflow-y: auto;
			overflow-x: hidden;
			-webkit-overflow-scrolling: touch;
			position: sticky;
			top: 20px;
		}

		.content-sidebar__section {
			border-right: none;
			border-bottom: 1px solid #e5e5e5;
		}

		.content-sidebar__section:last-child {
			border-bottom: none;
		}

		.room-sched {
			max-height: 300px;
			overflow-y: auto;
			-webkit-overflow-scrolling: touch;
		}
	}

	/* xl: 1280px+ - Large desktop */
	@media (min-width: 1280px) {
		.dashboard__content-sidebar {
			flex: 0 0 300px;
			width: 300px;
		}

		.content-sidebar__heading {
			font-size: 14px;
			padding: 14px 24px;
		}

		.script-container,
		.link-list {
			padding: 20px 24px;
		}
	}

	/* xxl: 1440px+ - Extra large desktop */
	@media (min-width: 1440px) {
		.dashboard__content-sidebar {
			flex: 0 0 320px;
			width: 320px;
		}
	}

	/* Touch-friendly link list */
	.link-list a {
		min-height: 44px;
		padding: 10px 0;
		display: flex;
		align-items: center;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.link-list a:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
		border-radius: 4px;
	}

	/* Schedule scrollable on all devices */
	.room-sched {
		max-height: 400px;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
		scrollbar-width: thin;
		scrollbar-color: #ccc transparent;
	}

	.room-sched::-webkit-scrollbar {
		width: 6px;
	}

	.room-sched::-webkit-scrollbar-track {
		background: transparent;
	}

	.room-sched::-webkit-scrollbar-thumb {
		background: #ccc;
		border-radius: 3px;
	}

	/* High contrast / reduced motion preferences */
	@media (prefers-reduced-motion: reduce) {
		.dashboard__content-sidebar {
			transition: none;
		}
	}

	/* Landscape on mobile - condensed layout */
	@media (max-width: 1023px) and (orientation: landscape) {
		.room-sched {
			max-height: 200px;
		}

		.schedule-event {
			padding-bottom: 12px;
		}

		.link-list li {
			margin-bottom: 8px;
		}
	}
</style>
