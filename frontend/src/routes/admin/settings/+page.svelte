<script lang="ts">
	import { onMount } from 'svelte';
	import { settingsApi, AdminApiError } from '$lib/api/admin';
	import { IconSettings, IconCheck, IconX } from '@tabler/icons-svelte';

	let loading = true;
	let saving = false;
	let error = '';
	let success = '';
	let settings: Record<string, any> = {};
	let groupedSettings: Record<string, any[]> = {};

	onMount(async () => {
		await loadSettings();
	});

	async function loadSettings() {
		loading = true;
		error = '';
		try {
			const response = await settingsApi.list();
			groupedSettings = response.data;
			
			// Flatten for editing
			Object.values(response.data).forEach((group: any) => {
				group.forEach((setting: any) => {
					settings[setting.key] = setting.value;
				});
			});
		} catch (err) {
			if (err instanceof AdminApiError) {
				error = err.message;
			} else {
				error = 'Error loading settings';
			}
			console.error('Failed to load settings:', err);
		} finally {
			loading = false;
		}
	}

	async function saveSettings() {
		saving = true;
		error = '';
		success = '';
		try {
			await settingsApi.update(settings);
			success = 'Settings saved successfully!';
			setTimeout(() => {
				success = '';
			}, 3000);
		} catch (err) {
			if (err instanceof AdminApiError) {
				error = err.message;
			} else {
				error = 'Error saving settings';
			}
			console.error('Failed to save settings:', err);
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head>
	<title>Settings | Admin</title>
</svelte:head>

<div class="admin-page">
	<div class="page-header">
		<div>
			<h1>Settings</h1>
			<p>Configure your site settings</p>
		</div>
		<button class="btn-primary" on:click={saveSettings} disabled={saving}>
			{#if saving}
				Saving...
			{:else}
				<IconCheck size={18} />
				Save Settings
			{/if}
		</button>
	</div>

	{#if success}
		<div class="alert success">
			<IconCheck size={20} />
			{success}
		</div>
	{/if}

	{#if error}
		<div class="alert error">
			<IconX size={20} />
			{error}
		</div>
	{/if}

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading settings...</p>
		</div>
	{:else if Object.keys(groupedSettings).length === 0}
		<div class="empty-state">
			<IconSettings size={64} stroke={1} />
			<h3>No settings found</h3>
			<p>Settings will appear here once configured</p>
		</div>
	{:else}
		<div class="settings-groups">
			{#each Object.entries(groupedSettings) as [group, groupSettings]}
				<div class="settings-group">
					<h2 class="group-title">{group || 'General'}</h2>
					<div class="settings-list">
						{#each groupSettings as setting}
							<div class="setting-item">
								<div class="setting-info">
									<label for={setting.key}>{setting.key.replace(/_/g, ' ')}</label>
									{#if setting.description}
										<p class="setting-description">{setting.description}</p>
									{/if}
								</div>
								<div class="setting-control">
									{#if setting.type === 'boolean'}
										<label class="toggle">
											<input
												type="checkbox"
												id={setting.key}
												bind:checked={settings[setting.key]}
											/>
											<span class="toggle-slider"></span>
										</label>
									{:else if setting.type === 'number'}
										<input
											type="number"
											id={setting.key}
											bind:value={settings[setting.key]}
											class="input"
										/>
									{:else}
										<input
											type="text"
											id={setting.key}
											bind:value={settings[setting.key]}
											class="input"
										/>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.admin-page {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem;
		background: #0f172a;
		min-height: 100vh;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
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
		font-size: 0.95rem;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-radius: 8px;
		margin-bottom: 1.5rem;
		font-weight: 500;
	}

	.alert.success {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.alert.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.loading,
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.loading .spinner {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(148, 163, 184, 0.1);
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 1.5rem;
	}

	.settings-groups {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.settings-group {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		overflow: hidden;
	}

	.group-title {
		padding: 1.5rem;
		background: rgba(15, 23, 42, 0.8);
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
		text-transform: capitalize;
	}

	.settings-list {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.setting-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.setting-item:last-child {
		border-bottom: none;
		padding-bottom: 0;
	}

	.setting-info {
		flex: 1;
	}

	.setting-info label {
		display: block;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.25rem;
		text-transform: capitalize;
	}

	.setting-description {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	.setting-control {
		min-width: 300px;
	}

	.input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.95rem;
		transition: all 0.2s;
	}

	.input:focus {
		outline: none;
		border-color: rgba(59, 130, 246, 0.5);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.toggle {
		position: relative;
		display: inline-block;
		width: 52px;
		height: 28px;
	}

	.toggle input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.toggle-slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(148, 163, 184, 0.2);
		border: 1px solid rgba(148, 163, 184, 0.3);
		transition: 0.3s;
		border-radius: 28px;
	}

	.toggle-slider:before {
		position: absolute;
		content: '';
		height: 20px;
		width: 20px;
		left: 3px;
		bottom: 3px;
		background: #94a3b8;
		transition: 0.3s;
		border-radius: 50%;
	}

	.toggle input:checked + .toggle-slider {
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		border-color: transparent;
	}

	.toggle input:checked + .toggle-slider:before {
		transform: translateX(24px);
		background: white;
	}

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.setting-item {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.setting-control {
			width: 100%;
			min-width: unset;
		}
	}
</style>
