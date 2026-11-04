<script lang="ts">
	import { onMount } from 'svelte';
	import { adminFetch } from '#lib/utils/adminFetch.js';
	import { connections, getIsEmailConnected } from '#lib/stores/connections.svelte.js';
	import ServiceConnectionStatus from '#lib/components/admin/ServiceConnectionStatus.svelte';

	let connectionLoading = $state(true);
	let settings = $state({
		provider: 'smtp',
		host: '',
		port: 587,
		username: '',
		password: '',
		encryption: 'tls',
		from_address: '',
		from_name: 'Revolution Trading Pros'
	});

	// FIX-2026-04-26 (07-marketing P0-2): mirrors /admin/email/settings.
	// See note in that file. TODO(2026-04-26-audit): this page duplicates
	// /admin/email/settings; pick one — see docs/audits/admin-2026-04-26/
	// 07-marketing-DEFERRED.md.
	let hasStoredPassword = $state(false);

	let loading = $state(false);
	let message = $state('');
	let messageType: 'success' | 'error' = $state('success');
	let testing = $state(false);
	let settingsLoading = $state(true);
	let settingsLoaded = $state(false);
	let settingsError = $state<string | null>(null);

	onMount(async () => {
		// Load connection status
		await connections.load();
		connectionLoading = false;
		await loadSettings();
	});

	interface EmailSettingsResponse {
		provider?: string;
		host?: string;
		port?: number;
		username?: string;
		encryption?: string;
		from_address?: string;
		from_name?: string;
		has_password?: boolean;
	}

	async function loadSettings() {
		settingsLoading = true;
		settingsError = null;
		try {
			const data = await adminFetch<EmailSettingsResponse>('/admin/email/settings', {
				signal: AbortSignal.timeout(25000)
			});
			if (
				data?.provider !== 'smtp' ||
				typeof data.host !== 'string' ||
				typeof data.port !== 'number' ||
				!Number.isInteger(data.port) ||
				data.port < 1 ||
				data.port > 65535 ||
				typeof data.username !== 'string' ||
				typeof data.has_password !== 'boolean' ||
				typeof data.encryption !== 'string' ||
				!['tls', 'ssl', 'null'].includes(data.encryption) ||
				typeof data.from_address !== 'string' ||
				typeof data.from_name !== 'string'
			)
				throw new Error('Invalid SMTP settings response');
			hasStoredPassword = Boolean(data?.has_password);
			settings = {
				provider: data?.provider ?? 'smtp',
				host: data?.host ?? '',
				port: data?.port ?? 587,
				username: data?.username ?? '',
				password: '',
				encryption: data?.encryption ?? 'tls',
				from_address: data?.from_address ?? '',
				from_name: data?.from_name ?? 'Revolution Trading Pros'
			};
			settingsLoaded = true;
		} catch (error) {
			settingsLoaded = false;
			settingsError = error instanceof Error ? error.message : 'Failed to load SMTP settings';
		} finally {
			settingsLoading = false;
		}
	}

	async function saveSettings() {
		if (!settingsLoaded) return;
		loading = true;
		message = '';

		try {
			const payload: Record<string, unknown> = { ...settings };
			if (!settings.password) {
				delete payload.password;
			}

			const saved = await adminFetch<{ success: boolean; settings: { has_password: boolean } }>(
				'/admin/email/settings',
				{
					method: 'POST',
					signal: AbortSignal.timeout(25000),
					body: JSON.stringify(payload)
				}
			);

			if (saved.success !== true || typeof saved.settings?.has_password !== 'boolean')
				throw new Error('Invalid SMTP save response; reload before retrying');
			message = 'SMTP settings saved. Test the connection to verify relay access.';
			messageType = 'success';
			settings.password = '';
			hasStoredPassword = saved.settings.has_password;
		} catch (error) {
			message = error instanceof Error ? error.message : 'Failed to save SMTP settings';
			messageType = 'error';
		} finally {
			loading = false;
		}
	}

	async function testConnection() {
		if (!settingsLoaded) return;
		testing = true;
		message = '';

		// Validate required fields before testing
		if (!settings.host || !settings.port || !settings.from_address) {
			message = 'Please fill in all required fields (Host, Port, From Email) before testing.';
			messageType = 'error';
			testing = false;
			return;
		}

		try {
			// FIX-2026-04-26 (07-marketing P0-2): omit blank password.
			const testPayload: Record<string, unknown> = { ...settings };
			if (!settings.password) {
				delete testPayload.password;
			}

			const result = (await adminFetch('/admin/email/settings/test', {
				method: 'POST',
				signal: AbortSignal.timeout(25000),
				body: JSON.stringify(testPayload)
			})) as { success?: boolean; message?: string; error?: string } | null;

			if (result && result.success === true) {
				message = result.message || 'SMTP connection check passed; no email was sent.';
				messageType = 'success';
			} else if (result && result.success === false) {
				message =
					result.error ||
					result.message ||
					'Connection test failed. Please check your SMTP credentials.';
				messageType = 'error';
			} else {
				message = 'Invalid SMTP verification response';
				messageType = 'error';
			}
		} catch (error) {
			const err = error as { name?: string; message?: string; status?: number };
			// Handle different types of errors
			if (err.name === 'TypeError' && err.message === 'Failed to fetch') {
				message =
					'Network error: Unable to reach the server. Please check your internet connection and try again.';
			} else if (err.status === 401) {
				message = 'Authentication error: Please log in again and retry.';
			} else if (err.status === 403) {
				message = 'Permission denied: You do not have access to test email settings.';
			} else if (err.status === 500) {
				message =
					'Server error: The email test failed on the server. Please check your SMTP configuration.';
			} else if (err.message) {
				message = `Test failed: ${err.message}`;
			} else {
				message = 'Test failed: An unexpected error occurred. Please try again.';
			}
			messageType = 'error';
		} finally {
			testing = false;
		}
	}
</script>

<svelte:head>
	<title>Email Settings | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="smtp-settings">
	<div class="page-header">
		<div>
			<h1>SMTP Email Configuration</h1>
			<p>Configure your email server settings for sending emails</p>
		</div>
	</div>

	{#if settingsError}
		<div role="alert" class="alert alert-error">
			{settingsError}
			<button
				type="button"
				class="btn-secondary"
				onclick={() => void loadSettings()}
				disabled={settingsLoading}>Retry SMTP settings</button
			>
		</div>
	{:else if settingsLoading}
		<p role="status">Loading SMTP settings...</p>
	{/if}

	<!-- Connection Status Banner -->
	{#if !connectionLoading && !getIsEmailConnected()}
		<ServiceConnectionStatus feature="email" variant="banner" />
	{/if}

	{#if message}
		<div class={['alert', `alert-${messageType}`]}>
			{message}
		</div>
	{/if}

	<div class="settings-card">
		<form
			onsubmit={(event) => {
				event.preventDefault();
				void saveSettings();
			}}
		>
			<div class="form-grid">
				<!-- Host -->
				<div class="form-group">
					<label for="host">SMTP Host</label>
					<input
						autocomplete="off"
						id="host"
						name="host"
						type="text"
						bind:value={settings.host}
						placeholder="smtp.gmail.com"
						required
					/>
				</div>

				<!-- Port -->
				<div class="form-group">
					<label for="port">Port</label>
					<input
						autocomplete="off"
						id="port"
						name="port"
						type="number"
						bind:value={settings.port}
						required
					/>
				</div>

				<!-- Encryption -->
				<div class="form-group">
					<label for="encryption">Encryption</label>
					<select id="encryption" bind:value={settings.encryption}>
						<option value="tls">TLS</option>
						<option value="ssl">SSL</option>
						<option value="null">None</option>
					</select>
				</div>

				<!-- Username -->
				<div class="form-group full-width">
					<label for="username">Username</label>
					<input
						id="username"
						name="username"
						type="text"
						autocomplete="username"
						bind:value={settings.username}
						placeholder="your-email@gmail.com"
					/>
				</div>

				<!-- Password -->
				<div class="form-group full-width">
					<label for="password">
						Password / App Password
						{#if hasStoredPassword}
							<span class="hint">(saved — leave blank to keep current)</span>
						{/if}
					</label>
					<input
						id="password"
						name="password"
						autocomplete="current-password"
						type="password"
						bind:value={settings.password}
						placeholder={hasStoredPassword
							? 'Leave blank to keep current password'
							: 'Enter password'}
					/>
				</div>

				<!-- From Address -->
				<div class="form-group">
					<label for="from_address">From Email</label>
					<input
						id="from_address"
						name="from_address"
						autocomplete="email"
						type="email"
						bind:value={settings.from_address}
						placeholder="noreply@revolutiontradingpros.com"
						required
					/>
				</div>

				<!-- From Name -->
				<div class="form-group">
					<label for="from_name">From Name</label>
					<input
						autocomplete="off"
						id="from_name"
						name="from_name"
						type="text"
						bind:value={settings.from_name}
						placeholder="Revolution Trading Pros"
						required
					/>
				</div>
			</div>

			<div class="form-actions">
				<button
					type="button"
					onclick={testConnection}
					disabled={testing || !settingsLoaded || settingsLoading}
					class="btn-secondary"
				>
					{testing ? 'Testing...' : 'Test Connection'}
				</button>
				<button
					type="submit"
					disabled={loading || !settingsLoaded || settingsLoading}
					class="btn-primary"
				>
					{loading ? 'Saving...' : 'Save Settings'}
				</button>
			</div>
		</form>
	</div>
</div>

<style>
	.smtp-settings {
		max-width: 900px;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: var(--text-secondary, #8b949e);
		font-size: 1rem;
	}

	.alert {
		padding: 1rem 1.25rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
		font-weight: 500;
	}

	.alert-success {
		background: rgba(63, 185, 80, 0.1);
		color: var(--success-emphasis, #3fb950);
		border: 1px solid rgba(63, 185, 80, 0.3);
	}

	.alert-error {
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		border: 1px solid rgba(248, 81, 73, 0.3);
	}

	.settings-card {
		background: var(--bg-elevated, #161b22);
		border-radius: 16px;
		border: 1px solid rgba(230, 184, 0, 0.1);
		padding: 2rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	label {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.hint {
		font-weight: 400;
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
		margin-left: 0.5rem;
	}

	input,
	select {
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9375rem;
		transition: all 0.2s;
	}

	input:focus,
	select:focus {
		outline: none;
		border-color: var(--primary-500);
		background: var(--bg-base, #0d1117);
	}

	.form-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
	}

	.btn-primary,
	.btn-secondary {
		padding: 0.875rem 1.75rem;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.btn-secondary {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary:hover:not(:disabled) {
		background: var(--bg-hover, #252b33);
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	@media (max-width: 639.98px) {
		.form-grid {
			grid-template-columns: 1fr;
		}

		.form-actions {
			flex-direction: column;
		}
	}
</style>
