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
	import { authStore, isAuthenticated, sessionInvalidated, type UserSession } from '$lib/stores/auth';
	import authService from '$lib/api/auth';

	// Redirect if not authenticated - use replaceState to prevent history pollution
	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isLoading && !$authStore.isInitializing) {
			goto('/login?redirect=/account/sessions', { replaceState: true });
		}
	});

	let sessions = $state<UserSession[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let revoking = $state<string | null>(null);
	let revokingAll = $state(false);

	onMount(async () => {
		await loadSessions();
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
			// Revoking current session means logging out
			if (!confirm('This will log you out. Are you sure?')) return;
		}

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

		if (!confirm('This will log you out from all devices. Are you sure?')) return;

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

		if (!confirm('This will log you out from all other devices. Continue?')) return;

		revokingAll = true;
		try {
			const response = await authService.logoutAllDevices(true);
			await loadSessions();
			alert(`Logged out from ${response.revoked_count} device(s)`);
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
	<div class="min-h-[calc(100vh-120px)] bg-gray-50 py-12 px-4">
		<div class="max-w-4xl mx-auto">
			<!-- Header -->
			<div class="mb-8">
				<nav class="text-sm mb-4">
					<a href="/account" class="text-blue-600 hover:underline">Account</a>
					<span class="mx-2 text-gray-400">/</span>
					<span class="text-gray-600">Sessions</span>
				</nav>
				<h1 class="text-3xl font-bold text-gray-900">Active Sessions</h1>
				<p class="mt-2 text-gray-600">
					Manage your active sessions across devices. If you notice any suspicious activity, revoke
					the session immediately.
				</p>
			</div>

			<!-- Session Invalidation Warning -->
			{#if $sessionInvalidated}
				<div
					class="mb-6 p-4 bg-amber-50 border border-amber-200 rounded-lg flex items-start gap-3"
				>
					<svg
						class="w-5 h-5 text-amber-500 flex-shrink-0 mt-0.5"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
						/>
					</svg>
					<div>
						<p class="font-medium text-amber-800">Session Notice</p>
						<p class="text-sm text-amber-700">
							Your previous session was ended because you signed in from another device.
						</p>
					</div>
				</div>
			{/if}

			<!-- Error Message -->
			{#if error}
				<div class="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
					{error}
					<button class="ml-2 underline" onclick={loadSessions}>Retry</button>
				</div>
			{/if}

			<!-- Actions -->
			<div class="mb-6 flex flex-wrap gap-3">
				<button
					class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 transition-colors"
					onclick={loadSessions}
					disabled={loading}
				>
					{#if loading}
						<span class="flex items-center gap-2">
							<svg class="animate-spin h-4 w-4" viewBox="0 0 24 24">
								<circle
									class="opacity-25"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
									fill="none"
								/>
								<path
									class="opacity-75"
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
					class="px-4 py-2 bg-amber-500 text-white rounded-lg hover:bg-amber-600 disabled:opacity-50 transition-colors"
					onclick={handleLogoutOtherDevices}
					disabled={revokingAll || sessions.length <= 1}
				>
					{revokingAll ? 'Processing...' : 'Sign out other devices'}
				</button>
				<button
					class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 transition-colors"
					onclick={handleLogoutAllDevices}
					disabled={revokingAll}
				>
					{revokingAll ? 'Processing...' : 'Sign out all devices'}
				</button>
			</div>

			<!-- Sessions List -->
			<div class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
				{#if loading && sessions.length === 0}
					<!-- Skeleton Loading -->
					{#each Array(3) as _}
						<div class="p-6 border-b border-gray-100 last:border-b-0 animate-pulse">
							<div class="flex items-start gap-4">
								<div class="w-12 h-12 bg-gray-200 rounded-lg"></div>
								<div class="flex-1">
									<div class="h-5 bg-gray-200 rounded w-48 mb-2"></div>
									<div class="h-4 bg-gray-100 rounded w-64 mb-1"></div>
									<div class="h-4 bg-gray-100 rounded w-32"></div>
								</div>
								<div class="w-20 h-8 bg-gray-200 rounded"></div>
							</div>
						</div>
					{/each}
				{:else if sessions.length === 0}
					<div class="p-12 text-center text-gray-500">
						<svg
							class="w-12 h-12 mx-auto mb-4 text-gray-300"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
							/>
						</svg>
						<p class="text-lg font-medium">No active sessions</p>
						<p class="text-sm">You don't have any active sessions.</p>
					</div>
				{:else}
					{#each sessions as session (session.session_id)}
						<div
							class="p-6 border-b border-gray-100 last:border-b-0 hover:bg-gray-50 transition-colors"
							class:bg-blue-50={session.is_current}
						>
							<div class="flex items-start gap-4">
								<!-- Device Icon -->
								<div
									class="w-12 h-12 rounded-lg flex items-center justify-center"
									class:bg-blue-100={session.is_current}
									class:text-blue-600={session.is_current}
									class:bg-gray-100={!session.is_current}
									class:text-gray-600={!session.is_current}
								>
									<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d={getDeviceIcon(session.device_type)}
										/>
									</svg>
								</div>

								<!-- Session Info -->
								<div class="flex-1 min-w-0">
									<div class="flex items-center gap-2 mb-1">
										<h3 class="font-semibold text-gray-900 truncate">
											{session.device_description || session.device_name}
										</h3>
										{#if session.is_current}
											<span
												class="px-2 py-0.5 text-xs font-medium bg-blue-100 text-blue-700 rounded-full"
											>
												This device
											</span>
										{/if}
									</div>
									<div class="text-sm text-gray-600 space-y-0.5">
										<p>
											<span class="text-gray-500">IP:</span>
											{session.ip_address}
											{#if session.location}
												<span class="text-gray-400">•</span>
												{session.location}
											{/if}
										</p>
										<p>
											<span class="text-gray-500">Last active:</span>
											{formatDate(session.last_activity_at)}
										</p>
										<p>
											<span class="text-gray-500">Signed in:</span>
											{formatDate(session.created_at)}
										</p>
									</div>
								</div>

								<!-- Actions -->
								<div>
									<button
										class="px-3 py-1.5 text-sm font-medium rounded-lg transition-colors disabled:opacity-50"
										class:bg-red-100={!session.is_current}
										class:text-red-700={!session.is_current}
										class:hover:bg-red-200={!session.is_current}
										class:bg-gray-100={session.is_current}
										class:text-gray-700={session.is_current}
										class:hover:bg-gray-200={session.is_current}
										onclick={() => handleRevokeSession(session.session_id)}
										disabled={revoking === session.session_id}
									>
										{#if revoking === session.session_id}
											<span class="flex items-center gap-1">
												<svg class="animate-spin h-3 w-3" viewBox="0 0 24 24">
													<circle
														class="opacity-25"
														cx="12"
														cy="12"
														r="10"
														stroke="currentColor"
														stroke-width="4"
														fill="none"
													/>
													<path
														class="opacity-75"
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
			<div class="mt-8 p-4 bg-blue-50 border border-blue-200 rounded-lg">
				<h3 class="font-medium text-blue-900 mb-2">Security Information</h3>
				<ul class="text-sm text-blue-800 space-y-1">
					<li class="flex items-start gap-2">
						<svg class="w-4 h-4 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
							<path
								fill-rule="evenodd"
								d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>For security, only one active session is allowed at a time.</span>
					</li>
					<li class="flex items-start gap-2">
						<svg class="w-4 h-4 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
							<path
								fill-rule="evenodd"
								d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Signing in on a new device will automatically sign you out from others.</span>
					</li>
					<li class="flex items-start gap-2">
						<svg class="w-4 h-4 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
							<path
								fill-rule="evenodd"
								d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>If you see a session you don't recognize, revoke it and change your password.</span>
					</li>
				</ul>
			</div>

			<!-- Back Link -->
			<div class="mt-8">
				<a href="/account" class="text-blue-600 hover:underline flex items-center gap-2">
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
	<div class="min-h-[calc(100vh-120px)] flex items-center justify-center">
		<div class="text-center">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
			<p class="mt-4 text-gray-600">Loading...</p>
		</div>
	</div>
{/if}
