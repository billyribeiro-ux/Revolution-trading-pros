<!--
	Trading Room Sidebar - Schedule + Quick Links
	Pixel-perfect extraction from WordPress DASHMAIN
	
	ICT 11+ Apple Principal Engineer Implementation:
	- Google Calendar API integration (exact same as WordPress)
	- Exponential backoff retry logic (3 attempts)
	- Circuit breaker pattern (prevents API abuse)
	- localStorage caching with 5-minute TTL
	- Stale-while-revalidate pattern
	- Structured logging for monitoring
	- Performance metrics tracking
	- Graceful degradation with cached fallback
	- Error boundary with user-friendly messages
	- Accessibility improvements (ARIA live regions)
	
	@version 2.0.0 - ICT 11+ Enterprise Grade
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPE DEFINITIONS - ICT 11+ TypeScript Best Practice
	// ═══════════════════════════════════════════════════════════════════════════

	interface ScheduleEvent {
		summary: string;
		startDateTime: string;
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
	let lastFetchTime = $state<number>(0);

	// ═══════════════════════════════════════════════════════════════════════════
	// CONFIGURATION - ICT 11+ Enterprise Settings
	// ═══════════════════════════════════════════════════════════════════════════

	// Google Calendar API configuration (exact same as WordPress)
	const CALENDAR_CONFIG = {
		CLIENT_ID: '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com',
		API_KEY: 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw',
		CALENDAR_ID: 'simpleroptions.com_sabio00har0rd4odbrsa705904@group.calendar.google.com',
		DISCOVERY_DOCS: ['https://www.googleapis.com/discovery/v1/apis/calendar/v3/rest'],
		SCOPES: 'https://www.googleapis.com/auth/calendar.readonly'
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

	// Quick Links data
	const quickLinks = [
		{
			text: 'Support',
			href: 'https://intercom.help/simpler-trading/en/',
			external: true
		},
		{
			text: 'Platform Tutorials',
			href: '/tutorials',
			external: true
		},
		{
			text: 'Simpler Blog',
			href: '/blog',
			external: true
		}
	];

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
			console.log('[ICT11 Calendar API]', logEntry);
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
		} catch (error) {
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
		} catch (error) {
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
		return new Promise(resolve => setTimeout(resolve, ms));
	}

	function calculateBackoff(attempt: number): number {
		const delay = RETRY_CONFIG.baseDelay * Math.pow(RETRY_CONFIG.backoffMultiplier, attempt);
		return Math.min(delay, RETRY_CONFIG.maxDelay);
	}

	async function fetchCalendarWithRetry(attempt: number = 0): Promise<ScheduleEvent[]> {
		const startTime = Date.now();

		try {
			// Load Google API
			await loadGoogleAPI();
			
			// Initialize gapi client
			await (window as any).gapi.client.init({
				apiKey: CALENDAR_CONFIG.API_KEY,
				clientId: CALENDAR_CONFIG.CLIENT_ID,
				discoveryDocs: CALENDAR_CONFIG.DISCOVERY_DOCS,
				scope: CALENDAR_CONFIG.SCOPES
			});

			// Fetch calendar events (exact same as WordPress)
			const response = await (window as any).gapi.client.calendar.events.list({
				calendarId: CALENDAR_CONFIG.CALENDAR_ID,
				timeMin: new Date().toISOString(),
				showDeleted: false,
				singleEvents: true,
				maxResults: 10,
				orderBy: 'startTime',
				fields: 'items(summary,start/dateTime)'
			});

			const items = response.result.items || [];
			const events = items.map((item: any) => ({
				summary: item.summary,
				startDateTime: item.start.dateTime
			}));

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
				return fetchCalendarWithRetry(attempt + 1);
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
			const events = await fetchCalendarWithRetry();
			
			// Update state
			scheduleEvents = events;
			scheduleLoading = false;
			scheduleError = false;
			isStaleData = false;
			lastFetchTime = Date.now();

			// Update cache
			setCachedSchedule(events);
		} catch (error) {
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

	// Load Google API script
	function loadGoogleAPI(): Promise<void> {
		return new Promise((resolve, reject) => {
			if ((window as any).gapi) {
				resolve();
				return;
			}

			const script = document.createElement('script');
			script.src = 'https://apis.google.com/js/api.js';
			script.onload = () => {
				(window as any).gapi.load('client', () => resolve());
			};
			script.onerror = reject;
			document.head.appendChild(script);
		});
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
							<h4>{event.summary}</h4>
							<span>{formatEventDate(event.startDateTime)}</span>
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
	 * TRADING ROOM SIDEBAR - Pixel Perfect WordPress Match
	 * Font: Montserrat (as requested by user)
	 * Source: DASHMAIN lines 3199-3222
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-sidebar {
		flex: 0 0 320px;
		width: 320px;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	@media (max-width: 1199px) {
		.dashboard__content-sidebar {
			flex: 0 0 280px;
			width: 280px;
		}
	}

	@media (max-width: 991px) {
		.dashboard__content-sidebar {
			display: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SIDEBAR SECTIONS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.content-sidebar__section {
		padding: 24px;
		border-bottom: 1px solid #e5e5e5;
	}

	.content-sidebar__section:last-child {
		border-bottom: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECTION HEADINGS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.content-sidebar__heading {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 16px 0;
		font-family: 'Montserrat', var(--font-heading), sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.pssubject {
		font-size: 10px;
		margin-top: 8px;
		text-transform: initial;
		color: #666;
		font-weight: 400;
		letter-spacing: normal;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * TRADING ROOM SCHEDULE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.script-container {
		margin-top: 12px;
	}

	.room-sched {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.schedule-event {
		padding-bottom: 16px;
		border-bottom: 1px solid #f0f0f0;
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
	 * QUICK LINKS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.link-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.link-list li {
		margin: 0 0 12px 0;
	}

	.link-list li:last-child {
		margin-bottom: 0;
	}

	.link-list a {
		display: block;
		font-size: 14px;
		font-weight: 600;
		color: #143E59;
		text-decoration: none;
		font-family: 'Montserrat', var(--font-heading), sans-serif;
		transition: color 0.15s ease-in-out;
		padding: 8px 0;
	}

	.link-list a:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE DESIGN
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1199px) {
		.content-sidebar__section {
			padding: 20px;
		}

		.content-sidebar__heading {
			font-size: 16px;
		}

		.schedule-event h4 {
			font-size: 13px;
		}

		.schedule-event span {
			font-size: 11px;
		}

		.link-list a {
			font-size: 13px;
		}
	}
</style>
