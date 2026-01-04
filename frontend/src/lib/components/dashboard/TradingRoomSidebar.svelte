<!--
	Trading Room Sidebar - Schedule + Quick Links
	Pixel-perfect extraction from WordPress DASHMAIN
	
	Features:
	- Google Calendar API integration for live schedule
	- Quick Links section (Support, Platform Tutorials, Simpler Blog)
	- Montserrat font family
	- Responsive design
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	interface ScheduleEvent {
		summary: string;
		startDateTime: string;
	}

	let scheduleEvents = $state<ScheduleEvent[]>([]);
	let scheduleLoading = $state(true);
	let scheduleError = $state(false);

	// Google Calendar API configuration
	const CALENDAR_CONFIG = {
		CLIENT_ID: '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com',
		API_KEY: 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw',
		CALENDAR_ID: 'simpleroptions.com_sabio00har0rd4odbrsa705904@group.calendar.google.com',
		DISCOVERY_DOCS: ['https://www.googleapis.com/discovery/v1/apis/calendar/v3/rest'],
		SCOPES: 'https://www.googleapis.com/auth/calendar.readonly'
	};

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

	// Load Google Calendar events
	async function loadSchedule() {
		if (!browser) return;

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

			// Fetch calendar events
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
			scheduleEvents = items.map((item: any) => ({
				summary: item.summary,
				startDateTime: item.start.dateTime
			}));

			scheduleLoading = false;
		} catch (error) {
			console.error('Failed to load schedule:', error);
			scheduleError = true;
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
			<div class="room-sched">
				{#if scheduleLoading}
					<p class="schedule-loading">Loading schedule...</p>
				{:else if scheduleError}
					<p class="schedule-error">Unable to load schedule. Please try again later.</p>
				{:else if scheduleEvents.length === 0}
					<p class="schedule-empty">No upcoming events scheduled.</p>
				{:else}
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
