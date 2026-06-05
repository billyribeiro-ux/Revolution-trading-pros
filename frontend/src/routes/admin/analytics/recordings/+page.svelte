<script lang="ts">
	/**
	 * Session Recordings - User Session Replay
	 * Apple ICT7 Grade Implementation
	 *
	 * Watch how users interact with your site
	 * through recorded session playback.
	 *
	 * ──────────────────────────────────────────────────────────────────────
	 * ⚠ PII REDACTION REQUIRED BEFORE INTEGRATING A REAL SESSION-REPLAY LIB
	 * ──────────────────────────────────────────────────────────────────────
	 *
	 * TODO(2026-04-26-audit-08-P0-3): Before wiring the placeholder player at
	 * `<!-- Player Area -->` to a real provider (rrweb, FullStory, Hotjar,
	 * OpenReplay, …) you MUST land an input-redaction policy first:
	 *
	 *   1. Mask all `<input>`, `<textarea>`, `[contenteditable]` content.
	 *      rrweb supports this via `maskAllInputs: true` or a per-field
	 *      `data-rr-mask` allowlist. NEVER ship `maskAllInputs: false`.
	 *   2. Block password / payment fields by attribute selector at the
	 *      recorder level — opt in, not opt out (autocomplete="cc-number",
	 *      type="password", `[data-payment]`, `[data-sensitive]`).
	 *   3. Strip `user_email`, full names, phone numbers, billing addresses,
	 *      and KYC fields from the page DOM before the recorder serializes
	 *      it. Today this page already exposes `recording.user_email` raw
	 *      (table line 325, modal line 442) — that surface needs masking
	 *      OR a role check (super-admin only) before any real recording
	 *      lands.
	 *   4. Honor the existing consent banner (`/api/admin/consent`) — do
	 *      not record sessions for users who have opted out.
	 *   5. Add a server-side TTL (≤ 90 days) and a "delete by user_id"
	 *      endpoint for GDPR/CCPA right-to-erasure.
	 *
	 * The audit (docs/audits/admin-2026-04-26/08-analytics.md §P0-3) flags
	 * this as P0 because the moment a recorder ships, it instantly starts
	 * capturing raw form inputs. Build the redaction policy FIRST.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconX from '@tabler/icons-svelte-runes/icons/x';

	interface Recording {
		id: string;
		session_id: string;
		user_id?: string;
		user_email?: string;
		duration: number;
		pages_viewed: number;
		events_count: number;
		device_type: 'desktop' | 'tablet' | 'mobile';
		browser: string;
		country: string;
		started_at: string;
		has_errors: boolean;
		has_rage_clicks: boolean;
	}

	// Svelte 5 Runes - State
	let recordings = $state<Recording[]>([]);
	let selectedRecording = $state<Recording | null>(null);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('7d');
	let activeFilter = $state<'all' | 'with_errors' | 'rage_clicks' | 'long'>('all');
	let page = $state(1);
	let totalPages = $state(1);

	async function loadRecordings() {
		loading = true;
		error = null;
		try {
			const params = new URLSearchParams({
				period: selectedPeriod,
				filter: activeFilter,
				page: String(page)
			});
			const response = await fetch(`/api/admin/analytics/recordings?${params.toString()}`);
			if (!response.ok) {
				// FIX-2026-04-26 (audit 08-analytics §P1-4): surface real upstream
				// status instead of silently zeroing the list. The previous
				// `recordings = []` masked the orphan-endpoint bug — admins saw
				// "No recordings" forever even when the backend was returning 404.
				throw new Error(`Failed to load recordings (HTTP ${response.status})`);
			}
			const data = await response.json();
			recordings = data.recordings || [];
			totalPages = data.pagination?.total_pages || 1;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load recordings';
			recordings = [];
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		page = 1;
		loadRecordings();
	}

	function handleFilterChange(filter: typeof activeFilter) {
		activeFilter = filter;
		page = 1;
		loadRecordings();
	}

	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}

	function formatDate(dateStr: string): string {
		// FIX-2026-04-26 (audit 08-analytics §P2-5): render in UTC so a backend
		// timestamp like `2026-04-26T00:00:00Z` doesn't read as `Apr 25` in
		// PST (the classic off-by-one). Calendar-day intent is UTC.
		const date = new Date(dateStr);
		return date.toLocaleString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			timeZone: 'UTC',
			timeZoneName: 'short'
		});
	}

	function getDeviceIcon(device: string): string {
		switch (device) {
			case 'mobile':
				return 'M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z';
			case 'tablet':
				return 'M12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z';
			default:
				return 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z';
		}
	}

	// FIX-2026-04-26 (P1-3): $derived(...) installs tracking at the proxy
	// boundary BEFORE the helper's `untrack` runs, so this stays reactive when
	// connectionsState mutates after mount. Bare `getIsAnalyticsConnected()`
	// calls in templates do not — they read the helper's untracked return.
	let isAnalyticsConnected = $derived(getIsAnalyticsConnected());

	// FIX-2026-04-26 (P1-1): use `onMount` instead of `$effect` for one-shot
	// init. The previous `$effect` read `getIsAnalyticsConnected()` then
	// synchronously called `connections.load()` which mutates the same rune —
	// the classic write-while-reading-tracked-dep cascade.
	onMount(() => {
		if (!browser) return;

		(async () => {
			try {
				await connections.load();
			} catch (e) {
				if (import.meta.env.DEV) {
					console.error('[Recordings] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadRecordings();
			} else {
				loading = false;
			}
		})();
	});

	// Derived stats
	const stats = $derived({
		total: recordings.length,
		avgDuration:
			recordings.length > 0
				? Math.round(recordings.reduce((sum, r) => sum + r.duration, 0) / recordings.length)
				: 0,
		withErrors: recordings.filter((r) => r.has_errors).length,
		rageClicks: recordings.filter((r) => r.has_rage_clicks).length
	});
</script>

<svelte:head>
	<title>Session Recordings | Analytics</title>
</svelte:head>

<div class="recordings-page">
	<div class="recordings-page__container">
		<!-- Apple ICT7 Grade Header -->
		<header class="recordings-header">
			<div class="recordings-header__title-group">
				<div class="recordings-header__icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: player-play -->
					<IconPlayerPlay size={24} aria-hidden="true" />
				</div>
				<div>
					<h1>Session Recordings</h1>
					<p>Watch how users interact with your site</p>
				</div>
			</div>
			{#if isAnalyticsConnected}
				<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="loading-state">
				<div class="loading-spinner loading-spinner--lg"></div>
			</div>
		{:else if !isAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<!-- Stats Grid -->
			<div class="recordings-stats">
				<div class="recordings-stat">
					<div class="recordings-stat__value">{stats.total}</div>
					<div class="recordings-stat__label">Total Recordings</div>
				</div>
				<div class="recordings-stat">
					<div class="recordings-stat__value" data-tone="indigo">
						{formatDuration(stats.avgDuration)}
					</div>
					<div class="recordings-stat__label">Avg Duration</div>
				</div>
				<div class="recordings-stat">
					<div class="recordings-stat__value" data-tone="red">{stats.withErrors}</div>
					<div class="recordings-stat__label">With Errors</div>
				</div>
				<div class="recordings-stat">
					<div class="recordings-stat__value" data-tone="amber">{stats.rageClicks}</div>
					<div class="recordings-stat__label">Rage Clicks</div>
				</div>
			</div>

			<!-- Filters -->
			<div class="recordings-filters">
				{#each [{ value: 'all', label: 'All Sessions' }, { value: 'with_errors', label: 'With Errors' }, { value: 'rage_clicks', label: 'Rage Clicks' }, { value: 'long', label: 'Long Sessions' }] as filter (filter.value)}
					<button
						onclick={() => handleFilterChange(filter.value as typeof activeFilter)}
						class={{ 'recordings-filter': true, 'is-active': activeFilter === filter.value }}
					>
						{filter.label}
					</button>
				{/each}
			</div>

			{#if loading}
				<div class="loading-state">
					<div class="loading-spinner"></div>
				</div>
			{:else if error}
				<div class="state-card state-card--error">
					<div class="state-card__icon">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: alert-circle error -->
						<IconAlertCircle size={32} aria-hidden="true" />
					</div>
					<p>{error}</p>
					<button onclick={loadRecordings} class="state-card__button state-card__button--error">
						Retry
					</button>
				</div>
			{:else if recordings.length === 0}
				<div class="state-card">
					<div class="state-card__icon state-card__icon--indigo">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: player-play-circle (no recordings empty state) -->
						<IconPlayerPlay size={32} aria-hidden="true" />
					</div>
					<h3>No Recordings Available</h3>
					<p>Session recordings will appear once enabled and visitors interact with your site</p>
					<a href="/admin/settings/tracking" class="state-card__button state-card__button--primary">
						Enable Recordings
					</a>
				</div>
			{:else}
				<!-- Recordings List -->
				<div class="recordings-table-card">
					<div class="recordings-table-scroll">
						<table class="recordings-table">
							<thead>
								<tr>
									<th>Session</th>
									<th>User</th>
									<th>Device</th>
									<th>Duration</th>
									<th>Pages</th>
									<th>Flags</th>
									<th class="align-right">Date</th>
									<th class="align-right"></th>
								</tr>
							</thead>
							<tbody>
								{#each recordings as recording (recording.session_id)}
									<tr>
										<td>
											<span class="recording-session">
												{recording.session_id.slice(0, 8)}...
											</span>
										</td>
										<td>
											{#if recording.user_email}
												<span class="recording-user">{recording.user_email}</span>
											{:else}
												<span class="recording-anonymous">Anonymous</span>
											{/if}
										</td>
										<td>
											<div class="recording-device">
												<svg
													class="recording-device__icon"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
												>
													<path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="2"
														d={getDeviceIcon(recording.device_type)}
													/>
												</svg>
												<span>{recording.device_type}</span>
											</div>
										</td>
										<td>
											<span class="recording-value">{formatDuration(recording.duration)}</span>
										</td>
										<td>
											<span class="recording-muted">{recording.pages_viewed} pages</span>
										</td>
										<td>
											<div class="recording-flags">
												{#if recording.has_errors}
													<span class="flag-badge" data-tone="red">Error</span>
												{/if}
												{#if recording.has_rage_clicks}
													<span class="flag-badge" data-tone="amber">Rage</span>
												{/if}
												{#if !recording.has_errors && !recording.has_rage_clicks}
													<span class="recording-empty-flag">-</span>
												{/if}
											</div>
										</td>
										<td class="align-right recording-date">
											{formatDate(recording.started_at)}
										</td>
										<td class="align-right">
											<button onclick={() => (selectedRecording = recording)} class="play-button">
												Play
											</button>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>

					<!-- Pagination -->
					{#if totalPages > 1}
						<div class="recordings-pagination">
							<p>
								Page {page} of {totalPages}
							</p>
							<div class="recordings-pagination__controls">
								<button
									onclick={() => {
										page = Math.max(1, page - 1);
										loadRecordings();
									}}
									disabled={page === 1}
									class="pagination-button"
								>
									Previous
								</button>
								<button
									onclick={() => {
										page = Math.min(totalPages, page + 1);
										loadRecordings();
									}}
									disabled={page === totalPages}
									class="pagination-button"
								>
									Next
								</button>
							</div>
						</div>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Recording Player Modal -->
{#if selectedRecording}
	<div class="recording-modal-backdrop">
		<div
			class="recording-modal"
			role="dialog"
			aria-modal="true"
			aria-labelledby="recording-modal-title"
		>
			<div class="recording-modal__header">
				<div class="recording-modal__title-group">
					<div class="recording-modal__icon">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: player-play (modal header) -->
						<IconPlayerPlay size={20} aria-hidden="true" />
					</div>
					<div>
						<h3 id="recording-modal-title">Session Recording</h3>
						<p>{selectedRecording.session_id}</p>
					</div>
				</div>
				<button
					onclick={() => (selectedRecording = null)}
					class="recording-modal__close"
					aria-label="Close recording"
				>
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (close) -->
					<IconX size={20} aria-hidden="true" />
				</button>
			</div>

			<!-- Player Area -->
			<div class="recording-player">
				<div class="recording-player__content">
					<div class="recording-player__icon">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: player-play-circle (player area) -->
						<IconPlayerPlay size={40} aria-hidden="true" />
					</div>
					<p>Session recording player</p>
					<span>Requires session replay library integration</span>
				</div>
			</div>

			<!-- Session Info -->
			<div class="recording-modal__meta">
				<div>
					<span>Duration</span>
					<p>{formatDuration(selectedRecording.duration)}</p>
				</div>
				<div>
					<span>Pages</span>
					<p>{selectedRecording.pages_viewed} viewed</p>
				</div>
				<div>
					<span>Device</span>
					<p class="recording-modal__device">{selectedRecording.device_type}</p>
				</div>
				<div>
					<span>Browser</span>
					<p>{selectedRecording.browser}</p>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.recordings-page {
		min-height: 100%;
		background: linear-gradient(135deg, #020617 0%, #0f172a 50%, #020617 100%);
		color: #ffffff;
	}

	.recordings-page__container {
		max-width: 80rem;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	.recordings-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.recordings-header__title-group,
	.recording-modal__title-group,
	.recording-device,
	.recording-flags,
	.recordings-pagination__controls {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.recordings-header__title-group {
		gap: 1rem;
	}

	.recordings-header__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 3rem;
		height: 3rem;
		border-radius: 1rem;
		background: linear-gradient(135deg, #6366f1 0%, #2563eb 100%);
		box-shadow: 0 10px 15px -3px rgb(99 102 241 / 0.2);
	}

	.recordings-header h1 {
		margin: 0;
		color: #ffffff;
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: 0;
		line-height: 2rem;
	}

	.recordings-header p,
	.recordings-stat__label,
	.recording-muted,
	.recording-date,
	.recording-device,
	.recording-session,
	.recordings-pagination p,
	.recording-modal__header p,
	.recording-player p,
	.recording-modal__meta span {
		margin: 0;
		color: #94a3b8;
	}

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
	}

	.loading-spinner {
		width: 2.5rem;
		height: 2.5rem;
		border: 4px solid rgb(99 102 241 / 0.2);
		border-top-color: #6366f1;
		border-radius: 999px;
		animation: spin 700ms linear infinite;
	}

	.loading-spinner--lg {
		width: 3rem;
		height: 3rem;
	}

	.recordings-stats {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.recordings-stat,
	.state-card,
	.recordings-table-card {
		border: 1px solid rgb(255 255 255 / 0.1);
		background: rgb(255 255 255 / 0.05);
		backdrop-filter: blur(24px);
	}

	.recordings-stat {
		border-radius: 1rem;
		padding: 1.25rem;
	}

	.recordings-stat__value {
		margin-bottom: 0.25rem;
		color: #ffffff;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 2.25rem;
	}

	.recordings-stat__value[data-tone='indigo'] {
		color: #818cf8;
	}

	.recordings-stat__value[data-tone='red'] {
		color: #f87171;
	}

	.recordings-stat__value[data-tone='amber'] {
		color: #fbbf24;
	}

	.recordings-stat__label,
	.recordings-pagination p {
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.recordings-filters {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
	}

	.recordings-filter,
	.pagination-button,
	.play-button,
	.state-card__button,
	.recording-modal__close {
		border: 0;
		cursor: pointer;
		font: inherit;
		transition:
			background 150ms ease,
			border-color 150ms ease,
			color 150ms ease,
			opacity 150ms ease;
	}

	.recordings-filter {
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 0.75rem;
		background: rgb(255 255 255 / 0.05);
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
		padding: 0.5rem 1rem;
	}

	.recordings-filter:hover {
		background: rgb(255 255 255 / 0.1);
		color: #ffffff;
	}

	.recordings-filter.is-active {
		border-color: #ffffff;
		background: #ffffff;
		color: #0f172a;
	}

	.state-card {
		border-radius: 1rem;
		padding: 3rem;
		text-align: center;
	}

	.state-card--error {
		border-color: rgb(239 68 68 / 0.2);
		background: rgb(239 68 68 / 0.1);
	}

	.state-card__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 4rem;
		height: 4rem;
		border-radius: 1rem;
		margin: 0 auto 1rem;
		background: rgb(239 68 68 / 0.1);
		color: #f87171;
	}

	.state-card__icon--indigo {
		background: rgb(99 102 241 / 0.1);
		color: #818cf8;
	}

	.state-card h3 {
		margin: 0 0 0.5rem;
		color: #ffffff;
		font-size: 1.125rem;
		font-weight: 500;
		line-height: 1.75rem;
	}

	.state-card p {
		margin: 0 0 1.5rem;
		color: #94a3b8;
	}

	.state-card--error p {
		color: #f87171;
	}

	.state-card__button {
		display: inline-block;
		border-radius: 0.75rem;
		color: #ffffff;
		font-weight: 600;
		padding: 0.75rem 1.5rem;
		text-decoration: none;
	}

	.state-card__button--primary {
		background: linear-gradient(90deg, #6366f1 0%, #2563eb 100%);
		box-shadow: 0 10px 15px -3px rgb(99 102 241 / 0.25);
	}

	.state-card__button--primary:hover {
		background: linear-gradient(90deg, #818cf8 0%, #3b82f6 100%);
	}

	.state-card__button--error {
		border: 1px solid rgb(239 68 68 / 0.3);
		background: rgb(239 68 68 / 0.2);
		color: #f87171;
		padding: 0.625rem 1.25rem;
	}

	.state-card__button--error:hover {
		background: rgb(239 68 68 / 0.3);
	}

	.recordings-table-card {
		overflow: hidden;
		border-radius: 1rem;
	}

	.recordings-table-scroll {
		overflow-x: auto;
	}

	.recordings-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.recordings-table thead {
		background: rgb(30 41 59 / 0.5);
	}

	.recordings-table th,
	.recordings-table td {
		padding: 1rem 1.25rem;
		text-align: left;
		vertical-align: middle;
	}

	.recordings-table th {
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 500;
		letter-spacing: 0.05em;
		line-height: 1rem;
		text-transform: uppercase;
		white-space: nowrap;
	}

	.recordings-table tbody tr + tr {
		border-top: 1px solid rgb(255 255 255 / 0.05);
	}

	.recordings-table tbody tr {
		transition: background 150ms ease;
	}

	.recordings-table tbody tr:hover {
		background: rgb(255 255 255 / 0.05);
	}

	.align-right {
		text-align: right;
	}

	.recording-session {
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', monospace;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.recording-user,
	.recording-value {
		color: #ffffff;
	}

	.recording-value {
		font-weight: 500;
	}

	.recording-anonymous {
		color: #64748b;
		font-style: italic;
	}

	.recording-device {
		gap: 0.5rem;
		text-transform: capitalize;
	}

	.recording-device__icon {
		width: 1rem;
		height: 1rem;
		color: #94a3b8;
	}

	.recording-flags {
		gap: 0.5rem;
	}

	.flag-badge {
		border-radius: 0.5rem;
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1rem;
		padding: 0.25rem 0.5rem;
	}

	.flag-badge[data-tone='red'] {
		background: rgb(239 68 68 / 0.2);
		color: #f87171;
	}

	.flag-badge[data-tone='amber'] {
		background: rgb(245 158 11 / 0.2);
		color: #fbbf24;
	}

	.recording-empty-flag {
		color: #64748b;
	}

	.recording-date {
		white-space: nowrap;
	}

	.play-button {
		border-radius: 0.5rem;
		background: rgb(99 102 241 / 0.2);
		color: #818cf8;
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1rem;
		padding: 0.375rem 0.75rem;
	}

	.play-button:hover {
		background: rgb(99 102 241 / 0.3);
	}

	.recordings-pagination {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		border-top: 1px solid rgb(255 255 255 / 0.1);
		padding: 1.25rem;
	}

	.recordings-pagination__controls {
		gap: 0.5rem;
	}

	.pagination-button {
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 0.75rem;
		background: transparent;
		color: #cbd5e1;
		font-size: 0.875rem;
		line-height: 1.25rem;
		padding: 0.5rem 1rem;
	}

	.pagination-button:hover:not(:disabled) {
		background: rgb(255 255 255 / 0.05);
	}

	.pagination-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.recording-modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgb(0 0 0 / 0.8);
		backdrop-filter: blur(8px);
		padding: 1rem;
	}

	.recording-modal {
		width: 100%;
		max-width: 64rem;
		max-height: 90vh;
		overflow: hidden;
		border: 1px solid rgb(255 255 255 / 0.1);
		border-radius: 1rem;
		background: #0f172a;
		box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.6);
	}

	.recording-modal__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		border-bottom: 1px solid rgb(255 255 255 / 0.1);
		padding: 1.25rem;
	}

	.recording-modal__title-group {
		gap: 1rem;
	}

	.recording-modal__icon,
	.recording-modal__close,
	.recording-player__icon {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.recording-modal__icon {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.75rem;
		background: rgb(99 102 241 / 0.2);
		color: #818cf8;
	}

	.recording-modal h3 {
		margin: 0;
		color: #ffffff;
		font-size: 1rem;
		font-weight: 600;
		line-height: 1.5rem;
	}

	.recording-modal__header p {
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.recording-modal__close {
		width: 2rem;
		height: 2rem;
		border-radius: 0.5rem;
		background: rgb(255 255 255 / 0.05);
		color: #94a3b8;
	}

	.recording-modal__close:hover {
		background: rgb(255 255 255 / 0.1);
		color: #ffffff;
	}

	.recording-player {
		display: flex;
		align-items: center;
		justify-content: center;
		aspect-ratio: 16 / 9;
		background: rgb(30 41 59 / 0.5);
	}

	.recording-player__content {
		text-align: center;
	}

	.recording-player__icon {
		width: 5rem;
		height: 5rem;
		border-radius: 1rem;
		margin: 0 auto 1rem;
		background: rgb(99 102 241 / 0.1);
		color: #818cf8;
	}

	.recording-player span {
		display: block;
		margin-top: 0.25rem;
		color: #64748b;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.recording-modal__meta {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
		border-top: 1px solid rgb(255 255 255 / 0.1);
		padding: 1.25rem;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.recording-modal__meta p {
		margin: 0;
		color: #ffffff;
		font-weight: 500;
	}

	.recording-modal__device {
		text-transform: capitalize;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 640px) {
		.recordings-page__container {
			padding-inline: 1.5rem;
		}
	}

	@media (min-width: 768px) {
		.recordings-stats {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}

		.recording-modal__meta {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.recordings-page__container {
			padding-inline: 2rem;
		}
	}

	@media (max-width: 760px) {
		.recordings-header,
		.recordings-pagination {
			align-items: flex-start;
			flex-direction: column;
		}

		.recordings-header :global(.period-selector) {
			width: 100%;
		}
	}
</style>
