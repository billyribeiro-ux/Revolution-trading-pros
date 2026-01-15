<script lang="ts">
	import { onMount } from 'svelte';
	import { boardsAPI } from '$lib/api/boards';
	import type { BoardsSettings, StorageConfig } from '$lib/boards/types';
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
	let loading = $state(true);
	let saving = $state(false);
	let testingStorage = $state(false);
	let storageTestResult = $state<{ success: boolean; message: string } | null>(null);
	let activeTab = $state<'general' | 'stages' | 'labels' | 'storage' | 'time' | 'integrations'>('general');

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
			storageConfig = storageRes;
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
			alert('Settings saved successfully!');
		} catch (error) {
			console.error('Failed to save settings:', error);
			alert('Failed to save settings');
		} finally {
			saving = false;
		}
	}

	async function saveStorageConfig() {
		if (!storageConfig) return;
		saving = true;
		try {
			await boardsAPI.updateStorageConfig(storageConfig);
			alert('Storage configuration saved!');
		} catch (error) {
			console.error('Failed to save storage config:', error);
			alert('Failed to save storage configuration');
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
		} catch (error) {
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

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center gap-4">
				<a
					href="/admin/boards"
					class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
				>
					<IconArrowLeft class="w-5 h-5" />
				</a>
				<div class="flex items-center gap-3">
					<div class="p-2 bg-[rgba(230,184,0,0.15)] dark:bg-[rgba(230,184,0,0.2)] rounded-lg">
						<IconSettings class="w-6 h-6 text-[#E6B800] dark:text-[#FFD11A]" />
					</div>
					<div>
						<h1 class="text-2xl font-bold text-gray-900 dark:text-white">Board Settings</h1>
						<p class="text-sm text-gray-500 dark:text-gray-400">Configure default settings for project boards</p>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		{#if loading}
			<div class="flex items-center justify-center py-12">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#E6B800]"></div>
			</div>
		{:else}
			<div class="flex gap-8">
				<!-- Sidebar -->
				<div class="w-48 flex-shrink-0">
					<nav class="space-y-1">
						<button
							onclick={() => activeTab = 'general'}
							class="w-full px-3 py-2 text-left rounded-lg flex items-center gap-2 {activeTab === 'general' ? 'bg-[rgba(230,184,0,0.1)] dark:bg-[rgba(230,184,0,0.2)] text-[#E6B800] dark:text-[#FFD11A]' : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
						>
							<IconLayoutKanban class="w-4 h-4" />
							General
						</button>
						<button
							onclick={() => activeTab = 'stages'}
							class="w-full px-3 py-2 text-left rounded-lg flex items-center gap-2 {activeTab === 'stages' ? 'bg-[rgba(230,184,0,0.1)] dark:bg-[rgba(230,184,0,0.2)] text-[#E6B800] dark:text-[#FFD11A]' : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
						>
							<IconPalette class="w-4 h-4" />
							Default Stages
						</button>
						<button
							onclick={() => activeTab = 'labels'}
							class="w-full px-3 py-2 text-left rounded-lg flex items-center gap-2 {activeTab === 'labels' ? 'bg-[rgba(230,184,0,0.1)] dark:bg-[rgba(230,184,0,0.2)] text-[#E6B800] dark:text-[#FFD11A]' : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
						>
							<IconPalette class="w-4 h-4" />
							Default Labels
						</button>
						<button
							onclick={() => activeTab = 'time'}
							class="w-full px-3 py-2 text-left rounded-lg flex items-center gap-2 {activeTab === 'time' ? 'bg-[rgba(230,184,0,0.1)] dark:bg-[rgba(230,184,0,0.2)] text-[#E6B800] dark:text-[#FFD11A]' : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
						>
							<IconClock class="w-4 h-4" />
							Time Tracking
						</button>
						<button
							onclick={() => activeTab = 'storage'}
							class="w-full px-3 py-2 text-left rounded-lg flex items-center gap-2 {activeTab === 'storage' ? 'bg-[rgba(230,184,0,0.1)] dark:bg-[rgba(230,184,0,0.2)] text-[#E6B800] dark:text-[#FFD11A]' : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
						>
							<IconCloud class="w-4 h-4" />
							Cloud Storage
						</button>
					</nav>
				</div>

				<!-- Content -->
				<div class="flex-1">
					<div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6">
						{#if activeTab === 'general'}
							<h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-6">General Settings</h2>
							<div class="space-y-6">
								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="default-board-type">
										Default Board Type
									</label>
									<select
										id="default-board-type"
										bind:value={settings.default_board_type}
										class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
									>
										<option value="kanban">Kanban Board</option>
										<option value="list">List View</option>
										<option value="calendar">Calendar View</option>
										<option value="table">Table View</option>
									</select>
								</div>

								<div class="flex items-center justify-between">
									<div>
										<div class="text-sm font-medium text-gray-700 dark:text-gray-300">Enable Webhooks</div>
										<div class="text-xs text-gray-500 dark:text-gray-400">Allow boards to send webhook notifications</div>
									</div>
									<label class="relative inline-flex items-center cursor-pointer">
										<input type="checkbox" bind:checked={settings.webhooks_enabled} class="sr-only peer" />
										<div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-[rgba(230,184,0,0.3)] dark:peer-focus:ring-[rgba(230,184,0,0.3)] rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-[#E6B800]"></div>
									</label>
								</div>
							</div>

						{:else if activeTab === 'stages'}
							<h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-6">Default Stages</h2>
							<p class="text-sm text-gray-500 dark:text-gray-400 mb-4">These stages will be created automatically for new boards.</p>

							<div class="space-y-3 mb-6">
								{#each settings.default_stages as stage, index}
									<div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
										<input
											type="color"
											bind:value={stage.color}
											class="w-8 h-8 rounded cursor-pointer"
										/>
										<input
											type="text"
											bind:value={stage.title}
											class="flex-1 px-3 py-1.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm"
										/>
										<label class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
											<input type="checkbox" bind:checked={stage.auto_complete} class="rounded" />
											Auto-complete
										</label>
										<button
											onclick={() => removeStage(index)}
											class="p-1.5 text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded"
										>
											<IconTrash class="w-4 h-4" />
										</button>
									</div>
								{/each}
							</div>

							<div class="flex items-center gap-3 p-3 border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg">
								<input
									type="color"
									bind:value={newStage.color}
									class="w-8 h-8 rounded cursor-pointer"
								/>
								<input
									type="text"
									bind:value={newStage.title}
									placeholder="Stage name..."
									class="flex-1 px-3 py-1.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm"
								/>
								<label class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
									<input type="checkbox" bind:checked={newStage.auto_complete} class="rounded" />
									Auto-complete
								</label>
								<button
									onclick={addStage}
									disabled={!newStage.title.trim()}
									class="px-3 py-1.5 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] text-sm rounded-lg disabled:opacity-50"
								>
									<IconPlus class="w-4 h-4" />
								</button>
							</div>

						{:else if activeTab === 'labels'}
							<h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-6">Default Labels</h2>
							<p class="text-sm text-gray-500 dark:text-gray-400 mb-4">These labels will be available for new boards.</p>

							<div class="space-y-3 mb-6">
								{#each settings.default_labels as label, index}
									<div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
										<input
											type="color"
											bind:value={label.color}
											class="w-8 h-8 rounded cursor-pointer"
										/>
										<input
											type="text"
											bind:value={label.title}
											class="flex-1 px-3 py-1.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm"
										/>
										<span
											class="px-3 py-1 text-xs text-white rounded"
											style="background-color: {label.color}"
										>
											{label.title}
										</span>
										<button
											onclick={() => removeLabel(index)}
											class="p-1.5 text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded"
										>
											<IconTrash class="w-4 h-4" />
										</button>
									</div>
								{/each}
							</div>

							<div class="flex items-center gap-3 p-3 border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg">
								<input
									type="color"
									bind:value={newLabel.color}
									class="w-8 h-8 rounded cursor-pointer"
								/>
								<input
									type="text"
									bind:value={newLabel.title}
									placeholder="Label name..."
									class="flex-1 px-3 py-1.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm"
								/>
								<button
									onclick={addLabel}
									disabled={!newLabel.title.trim()}
									class="px-3 py-1.5 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] text-sm rounded-lg disabled:opacity-50"
								>
									<IconPlus class="w-4 h-4" />
								</button>
							</div>

						{:else if activeTab === 'time'}
							<h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-6">Time Tracking Settings</h2>

							<div class="space-y-6">
								<div class="flex items-center justify-between">
									<div>
										<div class="text-sm font-medium text-gray-700 dark:text-gray-300">Enable Time Tracking</div>
										<div class="text-xs text-gray-500 dark:text-gray-400">Allow users to log time on tasks</div>
									</div>
									<label class="relative inline-flex items-center cursor-pointer">
										<input type="checkbox" bind:checked={settings.time_tracking_enabled} class="sr-only peer" />
										<div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-[rgba(230,184,0,0.3)] dark:peer-focus:ring-[rgba(230,184,0,0.3)] rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-[#E6B800]"></div>
									</label>
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="default-hourly-rate">
										Default Hourly Rate ($)
									</label>
									<input
										type="number"
										id="default-hourly-rate"
										bind:value={settings.default_hourly_rate}
										min="0"
										step="0.01"
										class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
									/>
								</div>

								<div class="grid grid-cols-2 gap-4">
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="working-hours">
											Working Hours/Day
										</label>
										<input
											type="number"
											id="working-hours"
											bind:value={settings.working_hours_per_day}
											min="1"
											max="24"
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="working-days">
											Working Days/Week
										</label>
										<input
											type="number"
											id="working-days"
											bind:value={settings.working_days_per_week}
											min="1"
											max="7"
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>
								</div>
							</div>

						{:else if activeTab === 'storage'}
							<h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-6">Cloud Storage Configuration</h2>
							<p class="text-sm text-gray-500 dark:text-gray-400 mb-4">Configure where attachments are stored. Leave empty to use local storage.</p>

							<div class="space-y-6">
								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="storage-driver">
										Storage Driver
									</label>
									<select
										id="storage-driver"
										bind:value={storageConfig!.driver}
										class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
									>
										<option value="local">Local Storage</option>
										<option value="s3">Amazon S3</option>
										<option value="r2">Cloudflare R2</option>
										<option value="digitalocean">DigitalOcean Spaces</option>
										<option value="backblaze">Backblaze B2</option>
									</select>
								</div>

								{#if storageConfig && storageConfig.driver !== 'local'}
									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="bucket-name">
											Bucket Name
										</label>
										<input
											type="text"
											id="bucket-name"
											bind:value={storageConfig.bucket}
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>

									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="region">
											Region
										</label>
										<input
											type="text"
											id="region"
											bind:value={storageConfig.region}
											placeholder="us-east-1"
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>

									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="endpoint">
											Endpoint (optional)
										</label>
										<input
											type="text"
											id="endpoint"
											bind:value={storageConfig.endpoint}
											placeholder="https://..."
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>

									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="access-key">
											Access Key
										</label>
										<input
											type="password"
											id="access-key"
											bind:value={storageConfig.access_key}
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>

									<div>
										<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2" for="secret-key">
											Secret Key
										</label>
										<input
											type="password"
											id="secret-key"
											bind:value={storageConfig.secret_key}
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										/>
									</div>

									<div class="flex items-center gap-4">
										<button
											onclick={testStorageConnection}
											disabled={testingStorage}
											class="px-4 py-2 text-[#E6B800] hover:bg-[rgba(230,184,0,0.1)] dark:hover:bg-[rgba(230,184,0,0.15)] rounded-lg flex items-center gap-2 disabled:opacity-50"
										>
											{#if testingStorage}
												<IconRefresh class="w-4 h-4 animate-spin" />
											{:else}
												<IconRefresh class="w-4 h-4" />
											{/if}
											Test Connection
										</button>
										{#if storageTestResult}
											<span class="flex items-center gap-2 text-sm {storageTestResult.success ? 'text-green-600' : 'text-red-600'}">
												{#if storageTestResult.success}
													<IconCheck class="w-4 h-4" />
												{:else}
													<IconAlertCircle class="w-4 h-4" />
												{/if}
												{storageTestResult.message}
											</span>
										{/if}
									</div>

									<button
										onclick={saveStorageConfig}
										disabled={saving}
										class="px-4 py-2 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg disabled:opacity-50"
									>
										Save Storage Config
									</button>
								{/if}
							</div>
						{/if}

						<!-- Save Button -->
						{#if activeTab !== 'storage'}
							<div class="mt-8 pt-6 border-t border-gray-200 dark:border-gray-700">
								<button
									onclick={saveSettings}
									disabled={saving}
									class="px-6 py-2 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg disabled:opacity-50 flex items-center gap-2"
								>
									{#if saving}
										<IconRefresh class="w-4 h-4 animate-spin" />
									{:else}
										<IconCheck class="w-4 h-4" />
									{/if}
									Save Settings
								</button>
							</div>
						{/if}
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>
