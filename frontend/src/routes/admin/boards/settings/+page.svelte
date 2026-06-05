<script lang="ts">
	import { onMount } from 'svelte';
	import { boardsAPI } from '$lib/api/boards';
	import type { BoardsSettings, StorageConfig } from '$lib/boards/types';
	// FIX-2026-04-26: replaced native alert() with toastStore.
	import { toastStore } from '$lib/stores/toast.svelte';
	import {
		IconLayoutKanban,
		IconArrowLeft,
		IconSettings,
		IconCloud,
		IconClock,
		IconPalette,
		IconPlus,
		IconTrash,
		IconCheck,
		IconAlertCircle,
		IconRefresh
	} from '$lib/icons';

	// State
	let settings = $state<BoardsSettings>({
		default_board_type: 'kanban',
		default_stages: [
			{ title: 'To Do', color: '#6b7280', auto_complete: false },
			{ title: 'In Progress', color: '#3b82f6', auto_complete: false },
			{ title: 'Done', color: '#22c55e', auto_complete: true }
		],
		default_labels: [
			{ title: 'Bug', color: '#ef4444' },
			{ title: 'Feature', color: '#B38F00' },
			{ title: 'Improvement', color: '#06b6d4' }
		],
		time_tracking_enabled: true,
		default_hourly_rate: 50,
		working_hours_per_day: 8,
		working_days_per_week: 5,
		webhooks_enabled: true,
		integrations: []
	});

	let storageConfig = $state<StorageConfig | null>(null);
	// FIX-2026-04-26 (P0-3): keep new credential entries in separate write-only buffers.
	// The backend never round-trips raw credentials; the page never binds the GET'd
	// (possibly redacted) value into a password field. Only send a credential field
	// on save when the user actually typed a new value into these buffers.
	let newAccessKeyBuffer = $state('');
	let newSecretKeyBuffer = $state('');
	let hasStoredAccessKey = $state(false);
	let hasStoredSecretKey = $state(false);
	let loading = $state(true);
	let saving = $state(false);
	let testingStorage = $state(false);
	let storageTestResult = $state<{ success: boolean; message: string } | null>(null);
	let activeTab = $state<'general' | 'stages' | 'labels' | 'storage' | 'time' | 'integrations'>(
		'general'
	);

	// New stage/label forms
	let newStage = $state({ title: '', color: '#6b7280', auto_complete: false });
	let newLabel = $state({ title: '', color: '#6b7280' });

	onMount(async () => {
		await loadSettings();
	});

	async function loadSettings() {
		loading = true;
		try {
			const [settingsRes, storageRes] = await Promise.all([
				boardsAPI.getSettings(),
				boardsAPI.getStorageConfig()
			]);
			settings = settingsRes;
			// FIX-2026-04-26 (P0-3): Note whether the backend reports stored credentials
			// (the proxy redacts secret_key/access_key to a placeholder string when set).
			// Then null those values out of the bound state so they NEVER live in the
			// browser's bound input. The user types into separate buffers to update.
			hasStoredAccessKey = !!storageRes?.access_key;
			hasStoredSecretKey = !!storageRes?.secret_key;
			storageConfig = storageRes
				? {
						...storageRes,
						access_key: '',
						secret_key: ''
					}
				: null;
			newAccessKeyBuffer = '';
			newSecretKeyBuffer = '';
		} catch (error) {
			console.error('Failed to load settings:', error);
		} finally {
			loading = false;
		}
	}

	async function saveSettings() {
		saving = true;
		try {
			await boardsAPI.updateSettings(settings);
			// FIX-2026-04-26: replaced native alert() with toastStore.success.
			// Old: alert('Settings saved successfully!');
			toastStore.success('Settings saved successfully!');
		} catch (error) {
			console.error('Failed to save settings:', error);
			// FIX-2026-04-26: replaced native alert() with toastStore.error.
			// Old: alert('Failed to save settings');
			toastStore.error('Failed to save settings');
		} finally {
			saving = false;
		}
	}

	async function saveStorageConfig() {
		if (!storageConfig) return;
		saving = true;
		try {
			// FIX-2026-04-26 (P0-3): Build the payload by merging only the buffers the
			// user actually populated. Empty buffer = "leave the stored credential
			// untouched" — the proxy strips the empty fields before forwarding.
			const payload: StorageConfig = {
				...storageConfig,
				...(newAccessKeyBuffer ? { access_key: newAccessKeyBuffer } : { access_key: '' }),
				...(newSecretKeyBuffer ? { secret_key: newSecretKeyBuffer } : { secret_key: '' })
			};
			await boardsAPI.updateStorageConfig(payload);
			// Clear write-only buffers and refresh stored-presence flags.
			newAccessKeyBuffer = '';
			newSecretKeyBuffer = '';
			hasStoredAccessKey = hasStoredAccessKey || !!payload.access_key;
			hasStoredSecretKey = hasStoredSecretKey || !!payload.secret_key;
			// FIX-2026-04-26: replaced native alert() with toastStore.success.
			// Old: alert('Storage configuration saved!');
			toastStore.success('Storage configuration saved!');
		} catch (error) {
			console.error('Failed to save storage config:', error);
			// FIX-2026-04-26: replaced native alert() with toastStore.error.
			// Old: alert('Failed to save storage configuration');
			toastStore.error('Failed to save storage configuration');
		} finally {
			saving = false;
		}
	}

	async function testStorageConnection() {
		if (!storageConfig) return;
		testingStorage = true;
		storageTestResult = null;
		try {
			storageTestResult = await boardsAPI.testStorageConfig(storageConfig);
		} catch (_error) {
			storageTestResult = { success: false, message: 'Connection test failed' };
		} finally {
			testingStorage = false;
		}
	}

	function addStage() {
		if (!newStage.title.trim()) return;
		settings.default_stages = [...settings.default_stages, { ...newStage }];
		newStage = { title: '', color: '#6b7280', auto_complete: false };
	}

	function removeStage(index: number) {
		settings.default_stages = settings.default_stages.filter((_, i) => i !== index);
	}

	function addLabel() {
		if (!newLabel.title.trim()) return;
		settings.default_labels = [...settings.default_labels, { ...newLabel }];
		newLabel = { title: '', color: '#6b7280' };
	}

	function removeLabel(index: number) {
		settings.default_labels = settings.default_labels.filter((_, i) => i !== index);
	}
</script>

<svelte:head>
	<title>Board Settings | Admin</title>
</svelte:head>

<div class="board-settings-page">
	<header class="settings-header">
		<div class="settings-container settings-container--header">
			<div class="header-row">
				<a href="/admin/boards" class="back-link" aria-label="Back to boards">
					<IconArrowLeft class="icon-md" />
				</a>
				<div class="title-row">
					<div class="title-icon" aria-hidden="true">
						<IconSettings class="icon-lg" />
					</div>
					<div>
						<h1>Board Settings</h1>
						<p>Configure default settings for project boards</p>
					</div>
				</div>
			</div>
		</div>
	</header>

	<main class="settings-container settings-container--main">
		{#if loading}
			<div class="loading-state" aria-live="polite" aria-label="Loading board settings">
				<span class="loading-spinner"></span>
			</div>
		{:else}
			<div class="settings-layout">
				<aside class="settings-sidebar" aria-label="Board settings sections">
					<nav class="settings-nav">
						<button
							type="button"
							onclick={() => (activeTab = 'general')}
							class={{ 'settings-tab': true, active: activeTab === 'general' }}
						>
							<IconLayoutKanban class="icon-sm" />
							<span>General</span>
						</button>
						<button
							type="button"
							onclick={() => (activeTab = 'stages')}
							class={{ 'settings-tab': true, active: activeTab === 'stages' }}
						>
							<IconPalette class="icon-sm" />
							<span>Default Stages</span>
						</button>
						<button
							type="button"
							onclick={() => (activeTab = 'labels')}
							class={{ 'settings-tab': true, active: activeTab === 'labels' }}
						>
							<IconPalette class="icon-sm" />
							<span>Default Labels</span>
						</button>
						<button
							type="button"
							onclick={() => (activeTab = 'time')}
							class={{ 'settings-tab': true, active: activeTab === 'time' }}
						>
							<IconClock class="icon-sm" />
							<span>Time Tracking</span>
						</button>
						<button
							type="button"
							onclick={() => (activeTab = 'storage')}
							class={{ 'settings-tab': true, active: activeTab === 'storage' }}
						>
							<IconCloud class="icon-sm" />
							<span>Cloud Storage</span>
						</button>
					</nav>
				</aside>

				<section class="settings-content">
					<div class="settings-card">
						{#if activeTab === 'general'}
							<h2>General Settings</h2>
							<div class="form-stack">
								<div class="field-group">
									<label for="default-board-type">Default Board Type</label>
									<select id="default-board-type" bind:value={settings.default_board_type}>
										<option value="kanban">Kanban Board</option>
										<option value="list">List View</option>
										<option value="calendar">Calendar View</option>
										<option value="table">Table View</option>
									</select>
								</div>

								<div class="setting-row">
									<div>
										<div id="webhooks-toggle-label" class="setting-label">Enable Webhooks</div>
										<div class="setting-description">
											Allow boards to send webhook notifications
										</div>
									</div>
									<label class="toggle-control">
										<input
											id="page-settings-webhooks-enabled"
											name="page-settings-webhooks-enabled"
											type="checkbox"
											bind:checked={settings.webhooks_enabled}
											aria-labelledby="webhooks-toggle-label"
										/>
										<span class="toggle-track"></span>
									</label>
								</div>
							</div>
						{:else if activeTab === 'stages'}
							<h2>Default Stages</h2>
							<p class="section-description">
								These stages will be created automatically for new boards.
							</p>

							<div class="item-list">
								{#each settings.default_stages as stage, index (index)}
									<div class="config-row">
										<input
											type="color"
											bind:value={stage.color}
											class="color-input"
											aria-label={`Stage ${index + 1} color`}
										/>
										<input
											type="text"
											bind:value={stage.title}
											class="compact-input"
											aria-label={`Stage ${index + 1} title`}
										/>
										<label class="checkbox-option">
											<input type="checkbox" bind:checked={stage.auto_complete} />
											<span>Auto-complete</span>
										</label>
										<button
											type="button"
											onclick={() => removeStage(index)}
											class="icon-button icon-button--danger"
											aria-label={`Remove ${stage.title || 'stage'}`}
										>
											<IconTrash class="icon-sm" />
										</button>
									</div>
								{/each}
							</div>

							<div class="config-row config-row--new">
								<input
									type="color"
									bind:value={newStage.color}
									class="color-input"
									aria-label="New stage color"
								/>
								<input
									type="text"
									bind:value={newStage.title}
									placeholder="Stage name..."
									class="compact-input"
									aria-label="New stage name"
								/>
								<label class="checkbox-option">
									<input type="checkbox" bind:checked={newStage.auto_complete} />
									<span>Auto-complete</span>
								</label>
								<button
									type="button"
									onclick={addStage}
									disabled={!newStage.title.trim()}
									class="icon-button icon-button--primary"
									aria-label="Add stage"
								>
									<IconPlus class="icon-sm" />
								</button>
							</div>
						{:else if activeTab === 'labels'}
							<h2>Default Labels</h2>
							<p class="section-description">These labels will be available for new boards.</p>

							<div class="item-list">
								{#each settings.default_labels as label, index (index)}
									<div class="config-row">
										<input
											type="color"
											bind:value={label.color}
											class="color-input"
											aria-label={`Label ${index + 1} color`}
										/>
										<input
											type="text"
											bind:value={label.title}
											class="compact-input"
											aria-label={`Label ${index + 1} title`}
										/>
										<span class="label-preview" style:background-color={label.color}>
											{label.title}
										</span>
										<button
											type="button"
											onclick={() => removeLabel(index)}
											class="icon-button icon-button--danger"
											aria-label={`Remove ${label.title || 'label'}`}
										>
											<IconTrash class="icon-sm" />
										</button>
									</div>
								{/each}
							</div>

							<div class="config-row config-row--new">
								<input
									type="color"
									bind:value={newLabel.color}
									class="color-input"
									aria-label="New label color"
								/>
								<input
									type="text"
									bind:value={newLabel.title}
									placeholder="Label name..."
									class="compact-input"
									aria-label="New label name"
								/>
								<button
									type="button"
									onclick={addLabel}
									disabled={!newLabel.title.trim()}
									class="icon-button icon-button--primary"
									aria-label="Add label"
								>
									<IconPlus class="icon-sm" />
								</button>
							</div>
						{:else if activeTab === 'time'}
							<h2>Time Tracking Settings</h2>

							<div class="form-stack">
								<div class="setting-row">
									<div>
										<div id="time-tracking-toggle-label" class="setting-label">
											Enable Time Tracking
										</div>
										<div class="setting-description">Allow users to log time on tasks</div>
									</div>
									<label class="toggle-control">
										<input
											id="page-settings-time-tracking-enabled"
											name="page-settings-time-tracking-enabled"
											type="checkbox"
											bind:checked={settings.time_tracking_enabled}
											aria-labelledby="time-tracking-toggle-label"
										/>
										<span class="toggle-track"></span>
									</label>
								</div>

								<div class="field-group">
									<label for="default-hourly-rate">Default Hourly Rate ($)</label>
									<input
										type="number"
										id="default-hourly-rate"
										name="default-hourly-rate"
										bind:value={settings.default_hourly_rate}
										min="0"
										step="0.01"
									/>
								</div>

								<div class="field-grid">
									<div class="field-group">
										<label for="working-hours">Working Hours/Day</label>
										<input
											type="number"
											id="working-hours"
											name="working-hours"
											bind:value={settings.working_hours_per_day}
											min="1"
											max="24"
										/>
									</div>
									<div class="field-group">
										<label for="working-days">Working Days/Week</label>
										<input
											type="number"
											id="working-days"
											name="working-days"
											bind:value={settings.working_days_per_week}
											min="1"
											max="7"
										/>
									</div>
								</div>
							</div>
						{:else if activeTab === 'storage'}
							<h2>Cloud Storage Configuration</h2>
							<p class="section-description">
								Configure where attachments are stored. Leave empty to use local storage.
							</p>

							<div class="form-stack">
								{#if storageConfig}
									<div class="field-group">
										<label for="storage-driver">Storage Driver</label>
										<select id="storage-driver" bind:value={storageConfig.driver}>
											<option value="local">Local Storage</option>
											<option value="s3">Amazon S3</option>
											<option value="r2">Cloudflare R2</option>
											<option value="digitalocean">DigitalOcean Spaces</option>
											<option value="backblaze">Backblaze B2</option>
										</select>
									</div>
								{/if}

								{#if storageConfig && storageConfig.driver !== 'local'}
									<div class="field-group">
										<label for="bucket-name">Bucket Name</label>
										<input
											type="text"
											id="bucket-name"
											name="bucket-name"
											bind:value={storageConfig.bucket}
										/>
									</div>

									<div class="field-group">
										<label for="region">Region</label>
										<input
											type="text"
											id="region"
											name="region"
											bind:value={storageConfig.region}
											placeholder="us-east-1"
										/>
									</div>

									<div class="field-group">
										<label for="endpoint">Endpoint (optional)</label>
										<input
											type="text"
											id="endpoint"
											name="endpoint"
											bind:value={storageConfig.endpoint}
											placeholder="https://..."
										/>
									</div>

									<div class="field-group">
										<label for="access-key">
											Access Key
											{#if hasStoredAccessKey}
												<span class="stored-note">
													(stored - leave blank to keep current value)
												</span>
											{/if}
										</label>
										<input
											type="password"
											id="access-key"
											name="access-key"
											autocomplete="new-password"
											bind:value={newAccessKeyBuffer}
											placeholder={hasStoredAccessKey ? '••••••••' : 'AKIA...'}
										/>
									</div>

									<div class="field-group">
										<label for="secret-key">
											Secret Key
											{#if hasStoredSecretKey}
												<span class="stored-note">
													(stored - leave blank to keep current value)
												</span>
											{/if}
										</label>
										<input
											type="password"
											id="secret-key"
											name="secret-key"
											autocomplete="new-password"
											bind:value={newSecretKeyBuffer}
											placeholder={hasStoredSecretKey ? '••••••••' : 'Enter secret key'}
										/>
									</div>

									<div class="storage-actions">
										<button
											type="button"
											onclick={testStorageConnection}
											disabled={testingStorage}
											class="secondary-button"
										>
											<IconRefresh class={testingStorage ? 'icon-sm spinning' : 'icon-sm'} />
											<span>Test Connection</span>
										</button>
										{#if storageTestResult}
											<span
												class={{
													'test-result': true,
													success: storageTestResult.success,
													error: !storageTestResult.success
												}}
											>
												{#if storageTestResult.success}
													<IconCheck class="icon-sm" />
												{:else}
													<IconAlertCircle class="icon-sm" />
												{/if}
												<span>{storageTestResult.message}</span>
											</span>
										{/if}
									</div>

									<button
										type="button"
										onclick={saveStorageConfig}
										disabled={saving}
										class="primary-button"
									>
										Save Storage Config
									</button>
								{/if}
							</div>
						{/if}

						{#if activeTab !== 'storage'}
							<div class="save-panel">
								<button
									type="button"
									onclick={saveSettings}
									disabled={saving}
									class="primary-button primary-button--with-icon"
								>
									<IconRefresh class={saving ? 'icon-sm spinning' : 'icon-sm hidden'} />
									<IconCheck class={saving ? 'icon-sm hidden' : 'icon-sm'} />
									<span>Save Settings</span>
								</button>
							</div>
						{/if}
					</div>
				</section>
			</div>
		{/if}
	</main>
</div>

<style>
	.board-settings-page {
		min-height: 100%;
		background: #f9fafb;
		color: #111827;
	}

	:global(.dark) .board-settings-page {
		background: #111827;
		color: #f9fafb;
	}

	.settings-header {
		border-bottom: 1px solid #e5e7eb;
		background: #ffffff;
	}

	:global(.dark) .settings-header {
		border-color: #374151;
		background: #1f2937;
	}

	.settings-container {
		width: min(100%, 64rem);
		margin-inline: auto;
		padding-inline: 1rem;
	}

	.settings-container--header {
		padding-block: 1.5rem;
	}

	.settings-container--main {
		padding-block: 2rem;
	}

	.header-row,
	.title-row,
	.settings-tab,
	.setting-row,
	.config-row,
	.checkbox-option,
	.storage-actions,
	.primary-button,
	.secondary-button,
	.test-result {
		display: flex;
		align-items: center;
	}

	.header-row {
		gap: 1rem;
	}

	.title-row {
		gap: 0.75rem;
		min-width: 0;
	}

	.title-icon,
	.back-link,
	.icon-button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex: 0 0 auto;
	}

	.title-icon {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.5rem;
		background: rgb(230 184 0 / 10%);
		color: #e6b800;
	}

	:global(.dark) .title-icon {
		background: rgb(179 143 0 / 30%);
		color: #ffd11a;
	}

	.back-link {
		width: 2.25rem;
		height: 2.25rem;
		border-radius: 0.5rem;
		color: #6b7280;
		transition:
			background-color 160ms ease,
			color 160ms ease;
	}

	.back-link:hover {
		background: #f3f4f6;
		color: #374151;
	}

	:global(.dark) .back-link {
		color: #9ca3af;
	}

	:global(.dark) .back-link:hover {
		background: #374151;
		color: #e5e7eb;
	}

	h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.25;
		color: #111827;
	}

	h2 {
		margin: 0 0 1.5rem;
		font-size: 1.125rem;
		font-weight: 650;
		line-height: 1.4;
		color: #111827;
	}

	:global(.dark) h1,
	:global(.dark) h2 {
		color: #ffffff;
	}

	.title-row p,
	.section-description,
	.setting-description {
		margin: 0;
		color: #6b7280;
	}

	.title-row p,
	.section-description {
		font-size: 0.875rem;
	}

	.section-description {
		margin-bottom: 1rem;
	}

	.setting-description {
		margin-top: 0.125rem;
		font-size: 0.75rem;
	}

	:global(.dark) .title-row p,
	:global(.dark) .section-description,
	:global(.dark) .setting-description {
		color: #9ca3af;
	}

	.loading-state {
		display: flex;
		justify-content: center;
		padding-block: 3rem;
	}

	.loading-spinner {
		width: 2rem;
		height: 2rem;
		border: 2px solid transparent;
		border-bottom-color: #e6b800;
		border-radius: 999px;
		animation: spin 800ms linear infinite;
	}

	.settings-layout {
		display: flex;
		gap: 2rem;
		align-items: flex-start;
	}

	.settings-sidebar {
		flex: 0 0 12rem;
	}

	.settings-nav {
		display: grid;
		gap: 0.25rem;
	}

	.settings-tab {
		width: 100%;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		border: 0;
		border-radius: 0.5rem;
		background: transparent;
		color: #374151;
		font: inherit;
		text-align: left;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			color 160ms ease;
	}

	.settings-tab:hover {
		background: #f3f4f6;
	}

	.settings-tab.active {
		background: rgb(230 184 0 / 5%);
		color: #b38f00;
	}

	:global(.dark) .settings-tab {
		color: #d1d5db;
	}

	:global(.dark) .settings-tab:hover {
		background: #374151;
	}

	:global(.dark) .settings-tab.active {
		background: rgb(179 143 0 / 30%);
		color: #ffd11a;
	}

	.settings-content {
		flex: 1 1 auto;
		min-width: 0;
	}

	.settings-card {
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #ffffff;
		padding: 1.5rem;
	}

	:global(.dark) .settings-card {
		border-color: #374151;
		background: #1f2937;
	}

	.form-stack {
		display: grid;
		gap: 1.5rem;
	}

	.field-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
	}

	.field-group {
		display: grid;
		gap: 0.5rem;
	}

	label,
	.setting-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	:global(.dark) label,
	:global(.dark) .setting-label {
		color: #d1d5db;
	}

	input:not([type='checkbox']):not([type='color']),
	select {
		width: 100%;
		min-height: 2.5rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		background: #ffffff;
		padding: 0.5rem 0.75rem;
		color: #111827;
		font: inherit;
	}

	:global(.dark) input:not([type='checkbox']):not([type='color']),
	:global(.dark) select {
		border-color: #4b5563;
		background: #374151;
		color: #ffffff;
	}

	input:focus-visible,
	select:focus-visible,
	button:focus-visible,
	a:focus-visible {
		outline: 3px solid rgb(230 184 0 / 35%);
		outline-offset: 2px;
	}

	.setting-row {
		justify-content: space-between;
		gap: 1rem;
	}

	.toggle-control {
		position: relative;
		display: inline-flex;
		align-items: center;
		flex: 0 0 auto;
		width: 2.75rem;
		height: 1.5rem;
		cursor: pointer;
	}

	.toggle-control input {
		position: absolute;
		width: 1px;
		height: 1px;
		margin: -1px;
		overflow: hidden;
		clip: rect(0 0 0 0);
		white-space: nowrap;
		border: 0;
		padding: 0;
	}

	.toggle-track {
		position: absolute;
		inset: 0;
		border-radius: 999px;
		background: #e5e7eb;
		transition:
			background-color 160ms ease,
			box-shadow 160ms ease;
	}

	.toggle-track::after {
		position: absolute;
		top: 0.125rem;
		left: 0.125rem;
		width: 1.25rem;
		height: 1.25rem;
		border: 1px solid #d1d5db;
		border-radius: 999px;
		background: #ffffff;
		content: '';
		transition: transform 160ms ease;
	}

	.toggle-control input:checked + .toggle-track {
		background: #e6b800;
	}

	.toggle-control input:checked + .toggle-track::after {
		transform: translateX(1.25rem);
		border-color: #ffffff;
	}

	.toggle-control input:focus-visible + .toggle-track {
		box-shadow: 0 0 0 4px rgb(230 184 0 / 30%);
	}

	:global(.dark) .toggle-track {
		background: #374151;
	}

	.item-list {
		display: grid;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.config-row {
		gap: 0.75rem;
		border-radius: 0.5rem;
		background: #f9fafb;
		padding: 0.75rem;
	}

	.config-row--new {
		border: 2px dashed #d1d5db;
		background: transparent;
	}

	:global(.dark) .config-row {
		background: #374151;
	}

	:global(.dark) .config-row--new {
		border-color: #4b5563;
		background: transparent;
	}

	.color-input {
		width: 2rem;
		height: 2rem;
		border: 0;
		border-radius: 0.375rem;
		background: transparent;
		padding: 0;
		cursor: pointer;
	}

	.compact-input {
		flex: 1 1 10rem;
		min-width: 8rem;
		min-height: 2.25rem;
		padding-block: 0.375rem;
		font-size: 0.875rem;
	}

	.checkbox-option {
		gap: 0.5rem;
		color: #4b5563;
		white-space: nowrap;
	}

	.checkbox-option input {
		width: 1rem;
		height: 1rem;
		accent-color: #e6b800;
	}

	:global(.dark) .checkbox-option {
		color: #9ca3af;
	}

	.label-preview {
		flex: 0 1 auto;
		max-width: 12rem;
		overflow: hidden;
		border-radius: 0.25rem;
		padding: 0.25rem 0.75rem;
		color: #ffffff;
		font-size: 0.75rem;
		line-height: 1.25;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.icon-button {
		width: 2rem;
		height: 2rem;
		border: 0;
		border-radius: 0.375rem;
		background: transparent;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			color 160ms ease,
			opacity 160ms ease;
	}

	.icon-button--danger {
		color: #ef4444;
	}

	.icon-button--danger:hover {
		background: #fef2f2;
	}

	:global(.dark) .icon-button--danger:hover {
		background: rgb(127 29 29 / 20%);
	}

	.icon-button--primary {
		background: #e6b800;
		color: #0d1117;
	}

	.icon-button--primary:hover {
		background: #b38f00;
	}

	.icon-button:disabled,
	.primary-button:disabled,
	.secondary-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.stored-note {
		margin-left: 0.5rem;
		color: #16a34a;
		font-size: 0.75rem;
	}

	:global(.dark) .stored-note {
		color: #4ade80;
	}

	.storage-actions {
		flex-wrap: wrap;
		gap: 1rem;
	}

	.primary-button,
	.secondary-button {
		width: fit-content;
		gap: 0.5rem;
		border: 0;
		border-radius: 0.5rem;
		padding: 0.5rem 1rem;
		font: inherit;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			color 160ms ease,
			opacity 160ms ease;
	}

	.primary-button {
		background: #e6b800;
		color: #0d1117;
	}

	.primary-button:hover:not(:disabled) {
		background: #b38f00;
	}

	.primary-button--with-icon {
		padding-inline: 1.5rem;
	}

	.secondary-button {
		background: transparent;
		color: #b38f00;
	}

	.secondary-button:hover:not(:disabled) {
		background: rgb(230 184 0 / 5%);
	}

	:global(.dark) .secondary-button {
		color: #ffd11a;
	}

	:global(.dark) .secondary-button:hover:not(:disabled) {
		background: rgb(179 143 0 / 20%);
	}

	.test-result {
		gap: 0.5rem;
		font-size: 0.875rem;
	}

	.test-result.success {
		color: #16a34a;
	}

	.test-result.error {
		color: #dc2626;
	}

	.save-panel {
		margin-top: 2rem;
		border-top: 1px solid #e5e7eb;
		padding-top: 1.5rem;
	}

	:global(.dark) .save-panel {
		border-color: #374151;
	}

	.board-settings-page :global(.icon-sm) {
		width: 1rem;
		height: 1rem;
	}

	.board-settings-page :global(.icon-md) {
		width: 1.25rem;
		height: 1.25rem;
	}

	.board-settings-page :global(.icon-lg) {
		width: 1.5rem;
		height: 1.5rem;
	}

	.board-settings-page :global(.spinning) {
		animation: spin 800ms linear infinite;
	}

	.board-settings-page :global(.hidden) {
		display: none;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 760px) {
		.settings-layout {
			display: grid;
			gap: 1rem;
		}

		.settings-sidebar {
			width: 100%;
			flex-basis: auto;
		}

		.settings-nav {
			display: flex;
			overflow-x: auto;
			padding-bottom: 0.25rem;
		}

		.settings-tab {
			width: auto;
			flex: 0 0 auto;
			white-space: nowrap;
		}

		.settings-card {
			padding: 1rem;
		}
	}

	@media (max-width: 640px) {
		.settings-container {
			padding-inline: 0.875rem;
		}

		.header-row {
			align-items: flex-start;
		}

		.title-row {
			align-items: flex-start;
		}

		.setting-row,
		.config-row {
			align-items: stretch;
		}

		.setting-row {
			display: grid;
			grid-template-columns: 1fr auto;
		}

		.config-row {
			flex-wrap: wrap;
		}

		.compact-input {
			flex-basis: calc(100% - 2.75rem);
		}

		.field-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
