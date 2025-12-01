<script lang="ts">
	import { onMount } from 'svelte';
	import { IconDeviceFloppy, IconRefresh } from '@tabler/icons-svelte';

	let settings: any = {};
	let loading = false;
	let saving = false;

	const settingGroups = [
		{
			name: 'General',
			key: 'general',
			fields: [
				{ key: 'site_name', label: 'Site Name', type: 'text' },
				{ key: 'site_description', label: 'Site Description', type: 'textarea' },
				{ key: 'default_og_image', label: 'Default OG Image URL', type: 'url' }
			]
		},
		{
			name: 'Robots',
			key: 'robots',
			fields: [
				{ key: 'noindex_default', label: 'NoIndex by Default', type: 'checkbox' },
				{ key: 'nofollow_default', label: 'NoFollow by Default', type: 'checkbox' }
			]
		},
		{
			name: 'Sitemap',
			key: 'sitemap',
			fields: [
				{ key: 'sitemap_submit_google', label: 'Submit to Google', type: 'checkbox' },
				{ key: 'sitemap_submit_bing', label: 'Submit to Bing', type: 'checkbox' }
			]
		}
	];

	onMount(() => {
		loadSettings();
	});

	async function loadSettings() {
		loading = true;
		try {
			const response = await fetch('/api/seo/settings/grouped');
			const data = await response.json();
			settings = data.data || {};
		} catch (error) {
			console.error('Failed to load settings:', error);
		} finally {
			loading = false;
		}
	}

	async function saveSettings() {
		saving = true;
		try {
			const settingsArray = Object.entries(settings).flatMap(
				([group, groupSettings]: [string, any]) =>
					Object.entries(groupSettings).map(([key, value]) => ({
						key,
						value,
						group
					}))
			);

			await fetch('/api/seo/settings/bulk-update', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ settings: settingsArray })
			});

			alert('Settings saved successfully!');
		} catch (error) {
			console.error('Failed to save settings:', error);
			alert('Failed to save settings');
		} finally {
			saving = false;
		}
	}

	function getSetting(group: string, key: string, defaultValue: any = '') {
		return settings[group]?.[key] ?? defaultValue;
	}

	function setSetting(group: string, key: string, value: any) {
		if (!settings[group]) {
			settings[group] = {};
		}
		settings[group][key] = value;
	}
</script>

<svelte:head>
	<title>SEO Settings | Admin</title>
</svelte:head>

<div class="settings-page">
	<header class="page-header">
		<div>
			<h1>SEO Settings</h1>
			<p>Configure SEO plugin settings and preferences</p>
		</div>
		<div class="header-actions">
			<button class="btn-secondary" onclick={loadSettings} disabled={loading}>
				<IconRefresh size={18} />
				Reset
			</button>
			<button class="btn-primary" onclick={saveSettings} disabled={saving}>
				<IconDeviceFloppy size={18} />
				{saving ? 'Saving...' : 'Save Changes'}
			</button>
		</div>
	</header>

	<div class="settings-content">
		{#each settingGroups as group}
			<div class="setting-group">
				<h2>{group.name}</h2>
				<div class="fields">
					{#each group.fields as field}
						<div class="field">
							<label for={field.key}>
								{field.label}
							</label>

							{#if field.type === 'text' || field.type === 'url'}
								<input
									id={field.key}
									type={field.type}
									value={getSetting(group.key, field.key)}
									oninput={(e) => setSetting(group.key, field.key, e.currentTarget.value)}
								/>
							{:else if field.type === 'textarea'}
								<textarea
									id={field.key}
									value={getSetting(group.key, field.key)}
									oninput={(e) => setSetting(group.key, field.key, e.currentTarget.value)}
									rows="3"
								></textarea>
							{:else if field.type === 'checkbox'}
								<input
									id={field.key}
									type="checkbox"
									checked={getSetting(group.key, field.key, false)}
									onchange={(e) => setSetting(group.key, field.key, e.currentTarget.checked)}
								/>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.settings-page {
		padding: 2rem;
		max-width: 1000px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-primary,
	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: white;
		color: #666;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover:not(:disabled) {
		background: #f8f9fa;
	}

	.settings-content {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.setting-group {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 2rem;
	}

	.setting-group h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1.5rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid #f0f0f0;
	}

	.fields {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.field label {
		display: block;
		font-weight: 500;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.field input[type='text'],
	.field input[type='url'],
	.field textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: inherit;
		transition: border-color 0.2s;
	}

	.field input[type='text']:focus,
	.field input[type='url']:focus,
	.field textarea:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.field textarea {
		resize: vertical;
	}

	.field input[type='checkbox'] {
		width: 20px;
		height: 20px;
		cursor: pointer;
	}
</style>
