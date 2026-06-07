<script lang="ts">
	/**
	 * Revolution Trading Pros - Active Sessions Management
	 * Microsoft-style single-session authentication management
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import {
		authStore,
		isAuthenticated,
		sessionInvalidated,
		type UserSession
	} from '$lib/stores/auth.svelte';
	import authService from '$lib/api/auth';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	let sessions = $state<UserSession[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let revoking = $state<string | null>(null);
	let revokingAll = $state(false);

	// Confirmation modal state
	let showRevokeCurrentModal = $state(false);
	let showLogoutAllModal = $state(false);
	let showLogoutOtherModal = $state(false);
	let pendingSessionId = $state<string | null>(null);

	// FIX-2026-04-26: comment-out, verify, delete in follow-up.
	// The old $effect below is the same legacy_pre_subscribe cascade pattern that was
	// fixed in admin/+layout.svelte — $effect re-fires on every reactive run that reads
	// $authStore / $isAuthenticated, which can trigger multiple loadSessions() calls and
	// redundant redirects during store hydration. Replaced with onMount so the auth guard
	// and initial data fetch fire exactly once after the component mounts.
	// $effect(() => {
	// 	if (!browser) return;
	//
	// 	// Auth guard - redirect if not authenticated (user interaction: page load)
	// 	if (!$isAuthenticated && !$authStore.isLoading && !$authStore.isInitializing) {
	// 		goto('/login?redirect=/account/sessions', { replaceState: true });
	// 		return;
	// 	}
	// 	loadSessions();
	// });
	onMount(() => {
		if (!browser) return;
		if (!$isAuthenticated && !$authStore.isLoading && !$authStore.isInitializing) {
			void goto('/login?redirect=/account/sessions', { replaceState: true });
			return;
		}
		void loadSessions();
	});

	async function loadSessions() {
		loading = true;
		error = null;
		try {
			const response = await authService.getSessions();
			sessions = response.sessions;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load sessions';
			console.error('Failed to load sessions:', e);
		} finally {
			loading = false;
		}
	}

	async function handleRevokeSession(sessionId: string) {
		if (revoking) return;

		const session = sessions.find((s) => s.session_id === sessionId);
		if (session?.is_current) {
			// Revoking current session means logging out - show modal
			pendingSessionId = sessionId;
			showRevokeCurrentModal = true;
			return;
		}

		await executeRevokeSession(sessionId);
	}

	async function executeRevokeSession(sessionId: string) {
		const session = sessions.find((s) => s.session_id === sessionId);
		revoking = sessionId;
		try {
			await authService.revokeSession(sessionId);
			sessions = sessions.filter((s) => s.session_id !== sessionId);

			// If we revoked current session, redirect to login
			if (session?.is_current) {
				authStore.clearAuth();
				goto('/login?reason=session_revoked', { replaceState: true });
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to revoke session';
		} finally {
			revoking = null;
		}
	}

	async function handleLogoutAllDevices() {
		if (revokingAll) return;
		showLogoutAllModal = true;
	}

	async function executeLogoutAllDevices() {
		showLogoutAllModal = false;
		revokingAll = true;
		try {
			await authService.logoutAllDevices(false);
			authStore.clearAuth();
			goto('/login?reason=logged_out_all', { replaceState: true });
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to logout from all devices';
			revokingAll = false;
		}
	}

	async function handleLogoutOtherDevices() {
		if (revokingAll) return;
		showLogoutOtherModal = true;
	}

	async function executeLogoutOtherDevices() {
		showLogoutOtherModal = false;
		revokingAll = true;
		try {
			await authService.logoutAllDevices(true);
			await loadSessions();
			// Successfully logged out from other devices
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to logout from other devices';
		} finally {
			revokingAll = false;
		}
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getDeviceIcon(deviceType: string): string {
		switch (deviceType) {
			case 'desktop':
				return 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z';
			case 'mobile':
				return 'M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z';
			case 'tablet':
				return 'M12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z';
			default:
				return 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z';
		}
	}
</script>

<svelte:head>
	<title>Active Sessions - Revolution Trading Pros</title>
	<meta name="description" content="Manage your active sessions and devices" />
</svelte:head>

{#if $authStore.user}
	<div class="account-sessions-page">
		<div class="account-sessions-container">
			<!-- Header -->
			<div class="sessions-header">
				<nav class="sessions-breadcrumb">
					<a href="/account" class="sessions-breadcrumb__link">Account</a>
					<span class="sessions-breadcrumb__separator">/</span>
					<span class="sessions-breadcrumb__current">Sessions</span>
				</nav>
				<h1 class="sessions-title">Active Sessions</h1>
				<p class="sessions-subtitle">
					Manage your active sessions across devices. If you notice any suspicious activity, revoke
					the session immediately.
				</p>
			</div>

			<!-- Session Invalidation Warning -->
			{#if $sessionInvalidated}
				<div class="session-notice">
					<svg class="session-notice__icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
						/>
					</svg>
					<div>
						<p class="session-notice__title">Session Notice</p>
						<p class="session-notice__text">
							Your previous session was ended because you signed in from another device.
						</p>
					</div>
				</div>
			{/if}

			<!-- Error Message -->
			{#if error}
				<div class="session-error">
					{error}
					<button class="session-error__retry" onclick={loadSessions}>Retry</button>
				</div>
			{/if}

			<!-- Actions -->
			<div class="session-actions">
				<button
					class="session-action session-action--primary"
					onclick={loadSessions}
					disabled={loading}
				>
					{#if loading}
						<span class="session-action__content">
							<svg aria-hidden="true" class="spinner spinner--sm" viewBox="0 0 24 24">
								<circle
									class="spinner__track"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
									fill="none"
								/>
								<path
									class="spinner__mark"
									fill="currentColor"
									d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
								/>
							</svg>
							Refreshing...
						</span>
					{:else}
						Refresh
					{/if}
				</button>
				<button
					class="session-action session-action--warning"
					onclick={handleLogoutOtherDevices}
					disabled={revokingAll || sessions.length <= 1}
				>
					{revokingAll ? 'Processing...' : 'Sign out other devices'}
				</button>
				<button
					class="session-action session-action--danger"
					onclick={handleLogoutAllDevices}
					disabled={revokingAll}
				>
					{revokingAll ? 'Processing...' : 'Sign out all devices'}
				</button>
			</div>

			<!-- Sessions List -->
			<div class="sessions-panel">
				{#if loading && sessions.length === 0}
					<!-- Skeleton Loading -->
					{#each Array(3) as _, i (i)}
						<div class="session-skeleton">
							<div class="session-skeleton__row">
								<div class="session-skeleton__device"></div>
								<div class="session-skeleton__content">
									<div class="session-skeleton__line session-skeleton__line--title"></div>
									<div class="session-skeleton__line session-skeleton__line--wide"></div>
									<div class="session-skeleton__line session-skeleton__line--short"></div>
								</div>
								<div class="session-skeleton__button"></div>
							</div>
						</div>
					{/each}
				{:else if sessions.length === 0}
					<div class="sessions-empty">
						<svg class="sessions-empty__icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
							/>
						</svg>
						<p class="sessions-empty__title">No active sessions</p>
						<p class="sessions-empty__text">You don't have any active sessions.</p>
					</div>
				{:else}
					{#each sessions as session (session.session_id)}
						<div class={['session-card', { 'session-card--current': session.is_current }]}>
							<div class="session-card__row">
								<!-- Device Icon -->
								<div class={['session-device', { 'session-device--current': session.is_current }]}>
									<svg
										aria-hidden="true"
										class="session-device__icon"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d={getDeviceIcon(session.device_type)}
										/>
									</svg>
								</div>

								<!-- Session Info -->
								<div class="session-main">
									<div class="session-heading">
										<h3 class="session-device-name">
											{session.device_description || session.device_name}
										</h3>
										{#if session.is_current}
											<span class="session-current-badge"> This device </span>
										{/if}
									</div>
									<div class="session-meta">
										<p>
											<span class="session-meta__label">IP:</span>
											{session.ip_address}
											{#if session.location}
												<span class="session-meta__separator">•</span>
												{session.location}
											{/if}
										</p>
										<p>
											<span class="session-meta__label">Last active:</span>
											{formatDate(session.last_activity_at)}
										</p>
										<p>
											<span class="session-meta__label">Signed in:</span>
											{formatDate(session.created_at)}
										</p>
									</div>
								</div>

								<!-- Actions -->
								<div>
									<button
										class={['session-revoke', { 'session-revoke--current': session.is_current }]}
										onclick={() => handleRevokeSession(session.session_id)}
										disabled={revoking === session.session_id}
									>
										{#if revoking === session.session_id}
											<span class="session-revoke__content">
												<svg aria-hidden="true" class="spinner spinner--xs" viewBox="0 0 24 24">
													<circle
														class="spinner__track"
														cx="12"
														cy="12"
														r="10"
														stroke="currentColor"
														stroke-width="4"
														fill="none"
													/>
													<path
														class="spinner__mark"
														fill="currentColor"
														d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
													/>
												</svg>
												Revoking...
											</span>
										{:else if session.is_current}
											Sign out
										{:else}
											Revoke
										{/if}
									</button>
								</div>
							</div>
						</div>
					{/each}
				{/if}
			</div>

			<!-- Security Info -->
			<div class="security-card">
				<h3 class="security-card__title">Security Information</h3>
				<ul class="security-list">
					<li class="security-list__item">
						<svg
							aria-hidden="true"
							class="security-list__icon"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>For security, only one active session is allowed at a time.</span>
					</li>
					<li class="security-list__item">
						<svg
							aria-hidden="true"
							class="security-list__icon"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Signing in on a new device will automatically sign you out from others.</span>
					</li>
					<li class="security-list__item">
						<svg
							aria-hidden="true"
							class="security-list__icon"
							fill="currentColor"
							viewBox="0 0 20 20"
						>
							<path
								fill-rule="evenodd"
								d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
								clip-rule="evenodd"
							/>
						</svg>
						<span
							>If you see a session you don't recognize, revoke it and change your password.</span
						>
					</li>
				</ul>
			</div>

			<!-- Back Link -->
			<div class="account-back">
				<a href="/account" class="account-back__link">
					<svg
						aria-hidden="true"
						class="account-back__icon"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M10 19l-7-7m0 0l7-7m-7 7h18"
						/>
					</svg>
					Back to Account
				</a>
			</div>
		</div>
	</div>
{:else}
	<div class="sessions-loading-page">
		<div class="sessions-loading-card">
			<div class="sessions-loading-spinner"></div>
			<p class="sessions-loading-text">Loading...</p>
		</div>
	</div>
{/if}

<!-- Confirmation Modals -->
<ConfirmationModal
	isOpen={showRevokeCurrentModal}
	title="End Current Session"
	message="This will log you out of the current session. Are you sure?"
	confirmText="Log Out"
	variant="warning"
	onConfirm={async () => {
		showRevokeCurrentModal = false;
		if (pendingSessionId) {
			await executeRevokeSession(pendingSessionId);
			pendingSessionId = null;
		}
	}}
	onCancel={() => {
		showRevokeCurrentModal = false;
		pendingSessionId = null;
	}}
/>

<ConfirmationModal
	isOpen={showLogoutAllModal}
	title="Log Out All Devices"
	message="This will log you out from all devices, including this one. Are you sure?"
	confirmText="Log Out All"
	variant="danger"
	onConfirm={executeLogoutAllDevices}
	onCancel={() => (showLogoutAllModal = false)}
/>

<ConfirmationModal
	isOpen={showLogoutOtherModal}
	title="Log Out Other Devices"
	message="This will log you out from all other devices except this one. Continue?"
	confirmText="Log Out Others"
	variant="warning"
	onConfirm={executeLogoutOtherDevices}
	onCancel={() => (showLogoutOtherModal = false)}
/>

<style>
	.account-sessions-page,
	.sessions-loading-page {
		min-height: calc(100vh - 120px);
		background: #f9fafb;
	}

	.account-sessions-page {
		padding: 3rem 1rem;
	}

	.account-sessions-container {
		width: min(100%, 56rem);
		margin-inline: auto;
	}

	.sessions-header {
		margin-bottom: 2rem;
	}

	.sessions-breadcrumb {
		margin-bottom: 1rem;
		color: #4b5563;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.sessions-breadcrumb__link,
	.account-back__link {
		color: #2563eb;
		text-decoration: none;
	}

	.sessions-breadcrumb__link:hover,
	.account-back__link:hover {
		text-decoration: underline;
	}

	.sessions-breadcrumb__separator {
		margin-inline: 0.5rem;
		color: #9ca3af;
	}

	.sessions-breadcrumb__current,
	.sessions-subtitle,
	.sessions-loading-text {
		color: #4b5563;
	}

	.sessions-title {
		margin: 0;
		color: #111827;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 2.25rem;
	}

	.sessions-subtitle {
		margin: 0.5rem 0 0;
	}

	.session-notice,
	.session-error,
	.security-card {
		margin-bottom: 1.5rem;
		border: 1px solid;
		border-radius: 0.5rem;
		padding: 1rem;
	}

	.session-notice {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		border-color: #fde68a;
		background: #fffbeb;
	}

	.session-notice__icon {
		width: 1.25rem;
		height: 1.25rem;
		flex-shrink: 0;
		margin-top: 0.125rem;
		color: #f59e0b;
	}

	.session-notice__title {
		margin: 0;
		color: #92400e;
		font-weight: 500;
	}

	.session-notice__text {
		margin: 0;
		color: #b45309;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.session-error {
		border-color: #fecaca;
		background: #fef2f2;
		color: #b91c1c;
	}

	.session-error__retry {
		margin-left: 0.5rem;
		color: inherit;
		text-decoration: underline;
	}

	.session-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.session-action,
	.session-revoke {
		border: 0;
		border-radius: 0.5rem;
		font-weight: 500;
		cursor: pointer;
		transition:
			background-color 0.18s ease,
			color 0.18s ease,
			opacity 0.18s ease;
	}

	.session-action {
		padding: 0.5rem 1rem;
		color: #ffffff;
	}

	.session-action:disabled,
	.session-revoke:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.session-action--primary {
		background: #2563eb;
	}

	.session-action--primary:hover:not(:disabled) {
		background: #1d4ed8;
	}

	.session-action--warning {
		background: #f59e0b;
	}

	.session-action--warning:hover:not(:disabled) {
		background: #d97706;
	}

	.session-action--danger {
		background: #dc2626;
	}

	.session-action--danger:hover:not(:disabled) {
		background: #b91c1c;
	}

	.session-action__content,
	.session-revoke__content,
	.security-list__item,
	.account-back__link {
		display: flex;
		align-items: center;
	}

	.session-action__content {
		gap: 0.5rem;
	}

	.sessions-panel {
		overflow: hidden;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #ffffff;
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.05);
	}

	.session-skeleton,
	.session-card {
		border-bottom: 1px solid #f3f4f6;
		padding: 1.5rem;
	}

	.session-skeleton:last-child,
	.session-card:last-child {
		border-bottom: 0;
	}

	.session-skeleton {
		animation: skeleton-pulse 1.6s ease-in-out infinite;
	}

	.session-skeleton__row,
	.session-card__row {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.session-skeleton__device,
	.session-device {
		width: 3rem;
		height: 3rem;
		border-radius: 0.5rem;
	}

	.session-skeleton__device,
	.session-skeleton__button,
	.session-skeleton__line {
		background: #e5e7eb;
	}

	.session-skeleton__content,
	.session-main {
		min-width: 0;
		flex: 1;
	}

	.session-skeleton__line {
		height: 1rem;
		border-radius: 0.25rem;
	}

	.session-skeleton__line--title {
		width: 12rem;
		height: 1.25rem;
		margin-bottom: 0.5rem;
	}

	.session-skeleton__line--wide {
		width: 16rem;
		margin-bottom: 0.25rem;
		background: #f3f4f6;
	}

	.session-skeleton__line--short {
		width: 8rem;
		background: #f3f4f6;
	}

	.session-skeleton__button {
		width: 5rem;
		height: 2rem;
		border-radius: 0.25rem;
	}

	.sessions-empty {
		padding: 3rem;
		color: #6b7280;
		text-align: center;
	}

	.sessions-empty__icon {
		width: 3rem;
		height: 3rem;
		margin: 0 auto 1rem;
		color: #d1d5db;
	}

	.sessions-empty__title {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 500;
		line-height: 1.75rem;
	}

	.sessions-empty__text {
		margin: 0;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.session-card {
		transition: background-color 0.18s ease;
	}

	.session-card:hover {
		background: #f9fafb;
	}

	.session-card--current {
		background: #eff6ff;
	}

	.session-card--current:hover {
		background: #dbeafe;
	}

	.session-device {
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f3f4f6;
		color: #4b5563;
	}

	.session-device--current {
		background: #dbeafe;
		color: #2563eb;
	}

	.session-device__icon {
		width: 1.5rem;
		height: 1.5rem;
	}

	.session-heading {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.25rem;
	}

	.session-device-name {
		margin: 0;
		overflow: hidden;
		color: #111827;
		font-weight: 600;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.session-current-badge {
		border-radius: 999px;
		background: #dbeafe;
		color: #1d4ed8;
		padding: 0.125rem 0.5rem;
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1rem;
	}

	.session-meta {
		color: #4b5563;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.session-meta p {
		margin: 0 0 0.125rem;
	}

	.session-meta p:last-child {
		margin-bottom: 0;
	}

	.session-meta__label {
		color: #6b7280;
	}

	.session-meta__separator {
		color: #9ca3af;
	}

	.session-revoke {
		border: 0;
		background: #fee2e2;
		color: #b91c1c;
		padding: 0.375rem 0.75rem;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.session-revoke:hover:not(:disabled) {
		background: #fecaca;
	}

	.session-revoke--current {
		background: #f3f4f6;
		color: #374151;
	}

	.session-revoke--current:hover:not(:disabled) {
		background: #e5e7eb;
	}

	.session-revoke__content {
		gap: 0.25rem;
	}

	.security-card {
		margin-top: 2rem;
		margin-bottom: 0;
		border-color: #bfdbfe;
		background: #eff6ff;
		color: #1e40af;
	}

	.security-card__title {
		margin: 0 0 0.5rem;
		color: #1e3a8a;
		font-weight: 500;
	}

	.security-list {
		display: grid;
		gap: 0.25rem;
		margin: 0;
		padding: 0;
		font-size: 0.875rem;
		line-height: 1.25rem;
		list-style: none;
	}

	.security-list__item {
		align-items: flex-start;
		gap: 0.5rem;
	}

	.security-list__icon {
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
		margin-top: 0.125rem;
	}

	.account-back {
		margin-top: 2rem;
	}

	.account-back__link {
		width: fit-content;
		gap: 0.5rem;
	}

	.account-back__icon {
		width: 1rem;
		height: 1rem;
	}

	.sessions-loading-page {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.sessions-loading-card {
		text-align: center;
	}

	.sessions-loading-spinner {
		width: 3rem;
		height: 3rem;
		margin-inline: auto;
		border: 2px solid transparent;
		border-bottom-color: #2563eb;
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.sessions-loading-text {
		margin: 1rem 0 0;
	}

	.spinner {
		animation: spin 0.8s linear infinite;
	}

	.spinner--sm {
		width: 1rem;
		height: 1rem;
	}

	.spinner--xs {
		width: 0.75rem;
		height: 0.75rem;
	}

	.spinner__track {
		opacity: 0.25;
	}

	.spinner__mark {
		opacity: 0.75;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes skeleton-pulse {
		50% {
			opacity: 0.55;
		}
	}

	@media (max-width: 640px) {
		.session-card__row,
		.session-skeleton__row {
			gap: 0.75rem;
		}

		.session-card {
			padding: 1rem;
		}

		.session-heading {
			align-items: flex-start;
			flex-direction: column;
		}
	}
</style>
