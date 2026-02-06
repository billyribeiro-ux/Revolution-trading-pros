<script lang="ts">
	import { onMount } from 'svelte';
	import { apiFetch } from '$lib/api/config';
	import { connections, getIsEmailConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';

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

	let loading = $state(false);
	let message = $state('');
	let messageType: 'success' | 'error' = $state('success');
	let testing = $state(false);

	onMount(async () => {
		// Load connection status
		await connections.load();
		connectionLoading = false;
		await loadSettings();
	});

	async function loadSettings() {
		try {
			const data = (await apiFetch('/admin/email/settings')) as any;
			settings = data;
		} catch (error) {
			console.error('Failed to load settings:', error);
		}
	}

	async function saveSettings() {
		loading = true;
		message = '';

		try {
			await apiFetch('/admin/email/settings', {
				method: 'POST',
				body: JSON.stringify(settings)
			});

			message = 'Settings saved successfully!';
			messageType = 'success';
		} catch (error) {
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
			const result: any = await apiFetch('/admin/email/settings/test', {
				method: 'POST',
				body: JSON.stringify(settings)
			});

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
		} catch (error: any) {
			// Handle different types of errors
			if (error.name === 'TypeError' && error.message === 'Failed to fetch') {
				message =
					'Network error: Unable to reach the server. Please check your internet connection and try again.';
			} else if (error.status === 401) {
				message = 'Authentication error: Please log in again and retry.';
			} else if (error.status === 403) {
				message = 'Permission denied: You do not have access to test email settings.';
			} else if (error.status === 500) {
				message =
					'Server error: The email test failed on the server. Please check your SMTP configuration.';
			} else if (error.message) {
				message = `Test failed: ${error.message}`;
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

<div class="smtp-settings">
	<div class="page-header">
		<div>
			<h1>SMTP Email Configuration</h1>
			<p>Configure your email server settings for sending emails</p>
		</div>
	</div>

	<!-- Connection Status Banner -->
	{#if !connectionLoading && !getIsEmailConnected}
		<ServiceConnectionStatus feature="email" variant="banner" />
	{/if}

	{#if message}
		<div class="alert alert-{messageType}">
			{message}
		</div>
	{/if}

	<div class="settings-card">
		<form onsubmit={saveSettings}>
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
					<label for="password">Password / App Password</label>
					<input
						id="password"
						name="password"
						autocomplete="current-password"
						type="password"
						bind:value={settings.password}
						placeholder="Enter password"
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
				<button type="button" onclick={testConnection} disabled={testing} class="btn-secondary">
					{testing ? 'Testing...' : 'Test Connection'}
				</button>
				<button type="submit" disabled={loading} class="btn-primary">
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
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #94a3b8;
		font-size: 1rem;
	}

	.alert {
		padding: 1rem 1.25rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
		font-weight: 500;
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

	.settings-card {
		background: rgba(30, 41, 59, 0.5);
		backdrop-filter: blur(10px);
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
		color: #e2e8f0;
	}

	input,
	select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: all 0.2s;
	}

	input:focus,
	select:focus {
		outline: none;
		border-color: var(--primary-500);
		background: rgba(15, 23, 42, 0.7);
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
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 20px rgba(230, 184, 0, 0.3);
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	@media (max-width: 640px) {
		.form-grid {
			grid-template-columns: 1fr;
		}

		.form-actions {
			flex-direction: column;
		}
	}
</style>
