<script lang="ts">
	import { onMount } from 'svelte';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconMailCheck from '@tabler/icons-svelte/icons/mail-check';
	import IconShieldLock from '@tabler/icons-svelte/icons/shield-lock';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconShoppingCart from '@tabler/icons-svelte/icons/shopping-cart';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import { crmAPI } from '$lib/api/crm';
	import type { DoubleOptInSettings, EmailPreferencePage } from '$lib/crm/types';

	let activeTab = $state('double-optin');
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state('');
	let successMessage = $state('');

	// Double Opt-In Settings
	let doubleOptIn = $state<DoubleOptInSettings>({
		enabled: false,
		email_subject: 'Please confirm your subscription',
		email_body: '<p>Hi {{first_name}},</p><p>Please click the button below to confirm your email address.</p><p><a href="{{confirmation_link}}">Confirm Email</a></p>',
		confirmation_page_url: '',
		redirect_url: '',
		after_confirmation_status: 'subscribed',
		apply_tags_on_confirm: [],
		add_to_lists_on_confirm: []
	});

	// Email Preferences Settings
	let emailPreferences = $state<EmailPreferencePage>({
		enabled: false,
		title: 'Email Preferences',
		intro_text: 'Manage your email subscription preferences below.',
		show_lists: true,
		show_tags: false,
		show_communication_types: true,
		show_unsubscribe_all: true,
		custom_css: '',
		redirect_after_update: ''
	});

	async function loadSettings() {
		isLoading = true;
		error = '';

		try {
			const [doiResponse, epResponse] = await Promise.all([
				crmAPI.getDoubleOptInSettings(),
				crmAPI.getEmailPreferenceSettings()
			]);

			doubleOptIn = doiResponse;
			emailPreferences = epResponse;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load settings';
		} finally {
			isLoading = false;
		}
	}

	async function saveDoubleOptIn() {
		isSaving = true;
		error = '';
		successMessage = '';

		try {
			await crmAPI.saveDoubleOptInSettings(doubleOptIn);
			successMessage = 'Double opt-in settings saved successfully';
			setTimeout(() => (successMessage = ''), 3000);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save settings';
		} finally {
			isSaving = false;
		}
	}

	async function saveEmailPreferences() {
		isSaving = true;
		error = '';
		successMessage = '';

		try {
			await crmAPI.saveEmailPreferenceSettings(emailPreferences);
			successMessage = 'Email preferences settings saved successfully';
			setTimeout(() => (successMessage = ''), 3000);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save settings';
		} finally {
			isSaving = false;
		}
	}

	const tabs = [
		{ id: 'double-optin', name: 'Double Opt-In', icon: IconMailCheck },
		{ id: 'email-preferences', name: 'Email Preferences', icon: IconMail },
		{ id: 'managers', name: 'Managers', icon: IconUsers, href: '/admin/crm/managers' },
		{ id: 'abandoned-cart', name: 'Abandoned Cart', icon: IconShoppingCart, href: '/admin/crm/abandoned-carts/settings' }
	];

	onMount(() => {
		loadSettings();
	});
</script>

<svelte:head>
	<title>CRM Settings - FluentCRM Pro</title>
</svelte:head>

<div class="settings-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>CRM Settings</h1>
			<p class="page-description">Configure your CRM preferences and options</p>
		</div>
	</div>

	<!-- Success/Error Messages -->
	{#if successMessage}
		<div class="success-message">
			<IconCheck size={18} />
			<span>{successMessage}</span>
		</div>
	{/if}

	{#if error}
		<div class="error-message">
			<span>{error}</span>
		</div>
	{/if}

	<div class="settings-layout">
		<!-- Tabs -->
		<div class="settings-tabs">
			{#each tabs as tab}
				{#if tab.href}
					<a href={tab.href} class="tab">
						<svelte:component this={tab.icon} size={18} />
						<span>{tab.name}</span>
					</a>
				{:else}
					<button class="tab" class:active={activeTab === tab.id} onclick={() => (activeTab = tab.id)}>
						<svelte:component this={tab.icon} size={18} />
						<span>{tab.name}</span>
					</button>
				{/if}
			{/each}
		</div>

		<!-- Content -->
		<div class="settings-content">
			{#if isLoading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Loading settings...</p>
				</div>
			{:else if activeTab === 'double-optin'}
				<!-- Double Opt-In Settings -->
				<div class="settings-section">
					<div class="section-header">
						<h2>Double Opt-In</h2>
						<p>Require contacts to confirm their email before subscribing</p>
					</div>

					<div class="form-group">
						<label class="toggle-label">
							<input type="checkbox" bind:checked={doubleOptIn.enabled} />
							<span class="toggle-switch"></span>
							<span>Enable Double Opt-In</span>
						</label>
						<p class="form-help">When enabled, new subscribers will receive a confirmation email before being added to your list.</p>
					</div>

					{#if doubleOptIn.enabled}
						<div class="form-group">
							<label for="doi-subject">Confirmation Email Subject</label>
							<input
								type="text"
								id="doi-subject"
								bind:value={doubleOptIn.email_subject}
								placeholder="Please confirm your subscription"
							/>
						</div>

						<div class="form-group">
							<label for="doi-body">Confirmation Email Body</label>
							<textarea
								id="doi-body"
								bind:value={doubleOptIn.email_body}
								rows={8}
								placeholder="HTML email content..."
							></textarea>
							<p class="form-help">Available smart tags: {"{{first_name}}"}, {"{{last_name}}"}, {"{{email}}"}, {"{{confirmation_link}}"}</p>
						</div>

						<div class="form-group">
							<label for="doi-redirect">Redirect URL After Confirmation</label>
							<input
								type="text"
								id="doi-redirect"
								bind:value={doubleOptIn.redirect_url}
								placeholder="https://example.com/thank-you"
							/>
						</div>

						<div class="form-group">
							<label for="doi-status">Contact Status After Confirmation</label>
							<select id="doi-status" bind:value={doubleOptIn.after_confirmation_status}>
								<option value="subscribed">Subscribed</option>
								<option value="pending">Pending</option>
							</select>
						</div>
					{/if}

					<div class="form-actions">
						<button class="btn-primary" onclick={saveDoubleOptIn} disabled={isSaving}>
							{#if isSaving}
								<IconRefresh size={18} class="spinning" />
							{:else}
								<IconCheck size={18} />
							{/if}
							Save Settings
						</button>
					</div>
				</div>
			{:else if activeTab === 'email-preferences'}
				<!-- Email Preferences Page Settings -->
				<div class="settings-section">
					<div class="section-header">
						<h2>Email Preferences Page</h2>
						<p>Configure the public email preferences page for subscribers</p>
					</div>

					<div class="form-group">
						<label class="toggle-label">
							<input type="checkbox" bind:checked={emailPreferences.enabled} />
							<span class="toggle-switch"></span>
							<span>Enable Email Preferences Page</span>
						</label>
						<p class="form-help">Allow subscribers to manage their email preferences through a dedicated page.</p>
					</div>

					{#if emailPreferences.enabled}
						<div class="form-group">
							<label for="ep-title">Page Title</label>
							<input
								type="text"
								id="ep-title"
								bind:value={emailPreferences.title}
								placeholder="Email Preferences"
							/>
						</div>

						<div class="form-group">
							<label for="ep-intro">Introduction Text</label>
							<textarea
								id="ep-intro"
								bind:value={emailPreferences.intro_text}
								rows={3}
								placeholder="Manage your email subscription preferences..."
							></textarea>
						</div>

						<div class="form-group">
							<label>Display Options</label>
							<div class="checkbox-group">
								<label class="checkbox-label">
									<input type="checkbox" bind:checked={emailPreferences.show_lists} />
									<span>Show Lists</span>
								</label>
								<label class="checkbox-label">
									<input type="checkbox" bind:checked={emailPreferences.show_tags} />
									<span>Show Tags</span>
								</label>
								<label class="checkbox-label">
									<input type="checkbox" bind:checked={emailPreferences.show_communication_types} />
									<span>Show Communication Types</span>
								</label>
								<label class="checkbox-label">
									<input type="checkbox" bind:checked={emailPreferences.show_unsubscribe_all} />
									<span>Show Unsubscribe All Option</span>
								</label>
							</div>
						</div>

						<div class="form-group">
							<label for="ep-css">Custom CSS</label>
							<textarea
								id="ep-css"
								bind:value={emailPreferences.custom_css}
								rows={5}
								placeholder="/* Custom styles */"
							></textarea>
						</div>

						<div class="form-group">
							<label for="ep-redirect">Redirect After Update</label>
							<input
								type="text"
								id="ep-redirect"
								bind:value={emailPreferences.redirect_after_update}
								placeholder="https://example.com/preferences-saved"
							/>
						</div>
					{/if}

					<div class="form-actions">
						<button class="btn-primary" onclick={saveEmailPreferences} disabled={isSaving}>
							{#if isSaving}
								<IconRefresh size={18} class="spinning" />
							{:else}
								<IconCheck size={18} />
							{/if}
							Save Settings
						</button>
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.settings-page {
		max-width: 1200px;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	.success-message, .error-message {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
	}

	.success-message {
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		color: #4ade80;
	}

	.error-message {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.settings-layout {
		display: grid;
		grid-template-columns: 220px 1fr;
		gap: 2rem;
	}

	@media (max-width: 768px) {
		.settings-layout {
			grid-template-columns: 1fr;
		}
	}

	.settings-tabs {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 10px;
		color: #94a3b8;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		text-align: left;
	}

	.tab:hover {
		color: #e2e8f0;
		background: rgba(99, 102, 241, 0.1);
	}

	.tab.active {
		color: #818cf8;
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
	}

	.settings-content {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		padding: 2rem;
	}

	.settings-section {
		max-width: 600px;
	}

	.section-header {
		margin-bottom: 2rem;
	}

	.section-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.section-header p {
		color: #64748b;
		margin: 0;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group > label {
		display: block;
		font-size: 0.9rem;
		font-weight: 600;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
	}

	.form-group input[type="text"],
	.form-group textarea,
	.form-group select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.form-group textarea {
		font-family: monospace;
		resize: vertical;
	}

	.form-help {
		font-size: 0.8rem;
		color: #64748b;
		margin-top: 0.5rem;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.toggle-label input {
		display: none;
	}

	.toggle-switch {
		width: 44px;
		height: 24px;
		background: rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		position: relative;
		transition: background 0.2s;
	}

	.toggle-switch::after {
		content: '';
		position: absolute;
		top: 2px;
		left: 2px;
		width: 20px;
		height: 20px;
		background: white;
		border-radius: 50%;
		transition: transform 0.2s;
	}

	.toggle-label input:checked + .toggle-switch {
		background: #6366f1;
	}

	.toggle-label input:checked + .toggle-switch::after {
		transform: translateX(20px);
	}

	.checkbox-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		color: #e2e8f0;
	}

	.checkbox-label input {
		width: 18px;
		height: 18px;
		accent-color: #6366f1;
	}

	.form-actions {
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.btn-primary :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
</style>
