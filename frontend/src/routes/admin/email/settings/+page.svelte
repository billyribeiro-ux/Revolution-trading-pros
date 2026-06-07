<script lang="ts">
	/**
	 * Email Settings - Apple ICT7 Principal Engineer Grade
	 *
	 * Svelte 5 runes implementation with:
	 * - $state for reactive state management
	 * - $effect for lifecycle and data loading
	 * - Proper TypeScript types
	 * - Enhanced error handling
	 *
	 */

	import { onMount } from 'svelte';
	import { apiFetch } from '$lib/api/config';
	import { connections, getIsEmailConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';

	// State - Svelte 5 Runes

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

	// FIX-2026-04-26 (07-marketing P0-2): track whether the backend already
	// has a stored password. The GET response carries `has_password: boolean`
	// (never the plaintext). When `has_password` is true and the admin leaves
	// the field blank, we send NO `password` key to the server — keeping the
	// stored value. When `has_password` is false the field is the only way
	// to seed credentials.
	let hasStoredPassword = $state(false);

	let loading = $state(false);
	let message = $state('');
	let messageType: 'success' | 'error' = $state('success');
	let testing = $state(false);

	// Lifecycle

	onMount(async () => {
		// Load connection status
		await connections.load();
		connectionLoading = false;
		await loadSettings();
	});

	// Data Loading

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
		try {
			const data = (await apiFetch('/admin/email/settings')) as EmailSettingsResponse;
			// FIX-2026-04-26 (07-marketing P0-2): never store plaintext password
			// in client state. Server returns `has_password: boolean`; we treat
			// the password field as a write-only secondary input.
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
		} catch (error) {
			console.error('Failed to load settings:', error);
		}
	}

	// Actions

	async function saveSettings() {
		loading = true;
		message = '';

		try {
			// FIX-2026-04-26 (07-marketing P0-2): omit `password` when blank so
			// the backend keeps the stored value. Only send a password when the
			// admin explicitly types one.
			const payload: Record<string, unknown> = { ...settings };
			if (!settings.password) {
				delete payload.password;
			}

			await apiFetch('/admin/email/settings', {
				method: 'POST',
				body: JSON.stringify(payload)
			});

			message = 'Settings saved successfully!';
			messageType = 'success';
			// Clear the password field after a successful save so it isn't
			// kept in DOM state across navigations.
			settings.password = '';
			hasStoredPassword = hasStoredPassword || Boolean(payload.password);
		} catch (_error) {
			message = 'Failed to save settings';
			messageType = 'error';
		} finally {
			loading = false;
		}
	}

	async function testConnection() {
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
			// FIX-2026-04-26 (07-marketing P0-2): never send the literal mask
			// or empty placeholder as a password to the SMTP test endpoint.
			// When the admin hasn't retyped, omit `password` so the backend
			// uses the stored credential to test.
			const testPayload: Record<string, unknown> = { ...settings };
			if (!settings.password) {
				delete testPayload.password;
			}

			const result = (await apiFetch('/admin/email/settings/test', {
				method: 'POST',
				body: JSON.stringify(testPayload)
			})) as { success?: boolean; message?: string; error?: string } | null;

			if (result && result.success === true) {
				message = result.message || 'Connection test successful! Email delivery is working.';
				messageType = 'success';
			} else if (result && result.success === false) {
				message =
					result.error ||
					result.message ||
					'Connection test failed. Please check your SMTP credentials.';
				messageType = 'error';
			} else {
				message = result?.message || 'Test completed. Check your inbox for a test email.';
				messageType = 'success';
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
			console.error('Email test connection error:', error);
		} finally {
			testing = false;
		}
	}
</script>

<svelte:head>
	<title>Email Settings | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-email-settings">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<header class="page-header">
			<h1>Email Settings</h1>
			<p class="subtitle">Configure your SMTP server settings for sending emails</p>
			<div class="header-actions">
				<button class="btn-secondary" onclick={testConnection} disabled={testing}>
					{testing ? 'Testing...' : 'Test Connection'}
				</button>
				<button class="btn-primary" onclick={saveSettings} disabled={loading}>
					{loading ? 'Saving...' : 'Save Settings'}
				</button>
			</div>
		</header>
	</div>

	<!-- Connection Status Banner -->
	{#if !connectionLoading && !getIsEmailConnected}
		<ServiceConnectionStatus feature="email" variant="banner" />
	{/if}

	{#if message}
		<div
			class={{
				alert: true,
				'alert-success': messageType === 'success',
				'alert-error': messageType === 'error'
			}}
		>
			{message}
		</div>
	{/if}

	<div class="settings-card">
		<h2 class="card-title">SMTP Configuration</h2>
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
					<input id="port" name="port" type="number" bind:value={settings.port} required />
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
			</div>
		</form>
	</div>

	<div class="settings-card">
		<h2 class="card-title">Sender Information</h2>
		<div class="form-grid">
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
					id="from_name"
					name="from_name"
					type="text"
					bind:value={settings.from_name}
					placeholder="Revolution Trading Pros"
					required
				/>
			</div>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	   Page Layout - Matching Email Templates Style
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.admin-email-settings {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Header - CENTERED
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Buttons
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		padding: 0.625rem 1.25rem;
		border-radius: 6px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 8px 16px rgba(99, 102, 241, 0.25);
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
		padding: 0.625rem 1.25rem;
		border-radius: 6px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Alerts
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.alert {
		padding: 0.75rem 1rem;
		border-radius: 8px;
		margin-bottom: 1.5rem;
		font-weight: 500;
		font-size: 0.875rem;
	}

	.alert-success {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Settings Cards
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.settings-card {
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		border: 1px solid rgba(148, 163, 184, 0.1);
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.card-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.25rem 0;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Form Elements
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.25rem;
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
		font-size: 0.8125rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.hint {
		font-weight: 400;
		font-size: 0.75rem;
		color: #64748b;
		margin-left: 0.5rem;
	}

	input,
	select {
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	input:focus,
	select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
		background: rgba(15, 23, 42, 0.8);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	input::placeholder {
		color: #475569;
	}

	select {
		cursor: pointer;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Responsive
	   ═══════════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 639.98px) {
		.admin-email-settings {
			padding: 1rem;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}

		.header-actions {
			flex-direction: column;
		}
	}
</style>
