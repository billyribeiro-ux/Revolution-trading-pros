<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconPhoto,
		IconRefresh,
		IconSparkles,
		IconCheck,
		IconAlertTriangle,
		IconSettings
	} from '$lib/icons';
	import {
		imageSeoSettings,
		processImages,
		defaultImageSeoSettings,
		type CaseTransform,
		type ImageMetadata,
		type ProcessedImage
	} from '$lib/seo';

	// State using Svelte 5 runes
	let images = $state<ProcessedImage[]>([]);
	let settings = $state(defaultImageSeoSettings);
	let loading = $state(false);
	let processing = $state(false);
	let activeTab = $state<'images' | 'settings'>('images');

	// Stats
	let stats = $derived({
		total: images.length,
		withAlt: images.filter((i) => i.generatedAlt).length,
		withTitle: images.filter((i) => i.generatedTitle).length,
		avgScore: images.length
			? Math.round(images.reduce((acc, i) => acc + i.seoScore, 0) / images.length)
			: 0,
		issues: images.reduce((acc, i) => acc + i.issues.length, 0)
	});

	const caseOptions: { value: CaseTransform; label: string }[] = [
		{ value: 'titlecase', label: 'Title Case' },
		{ value: 'sentencecase', label: 'Sentence case' },
		{ value: 'lowercase', label: 'lowercase' },
		{ value: 'uppercase', label: 'UPPERCASE' },
		{ value: 'none', label: 'No change' }
	];

	onMount(() => {
		// Load settings from store
		const unsubscribe = imageSeoSettings.subscribe((value) => {
			settings = value;
		});

		loadImages();

		return () => unsubscribe();
	});

	async function loadImages() {
		loading = true;
		try {
			// In production, scan content for images
			const sampleImages: ImageMetadata[] = [
				{
					src: '/images/courses/day-trading-masterclass.jpg',
					alt: '',
					width: 1200,
					height: 630
				},
				{
					src: '/images/blog/SP500_Analysis_2025.png',
					alt: 'Stock chart',
					width: 800,
					height: 600
				},
				{
					src: '/images/team/john_smith_headshot.jpg',
					alt: '',
					width: 400,
					height: 400
				},
				{
					src: '/images/indicators/RSI-indicator-example.webp',
					alt: '',
					width: 1000,
					height: 500
				},
				{
					src: '/images/123456.jpg',
					alt: '',
					width: 600,
					height: 400
				}
			];

			images = processImages(sampleImages, settings);
		} finally {
			loading = false;
		}
	}

	async function processAllImages() {
		processing = true;
		try {
			// Reprocess with current settings
			const rawImages = images.map((img) => ({
				src: img.src,
				alt: img.alt,
				title: img.title,
				width: img.width,
				height: img.height
			}));
			images = processImages(rawImages, settings);
			alert('All images processed successfully!');
		} finally {
			processing = false;
		}
	}

	function saveSettings() {
		imageSeoSettings.set(settings);
		alert('Settings saved!');
		// Reprocess images with new settings
		loadImages();
	}

	function resetSettings() {
		if (confirm('Reset all settings to defaults?')) {
			settings = defaultImageSeoSettings;
			imageSeoSettings.reset();
		}
	}

	function getScoreColor(score: number): string {
		if (score >= 80) return '#10b981';
		if (score >= 60) return '#f59e0b';
		return '#ef4444';
	}
</script>

<svelte:head>
	<title>Image SEO | SEO</title>
</svelte:head>

<div class="image-seo-page">
	<header class="page-header">
		<div>
			<h1>
				<IconPhoto size={28} />
				Image SEO
			</h1>
			<p>Optimize images with auto-generated alt text and titles</p>
		</div>
		<div class="header-actions">
			<button class="btn-secondary" onclick={loadImages} disabled={loading}>
				<IconRefresh size={18} class={loading ? 'spinning' : ''} />
				Scan Images
			</button>
			<button class="btn-primary" onclick={processAllImages} disabled={processing}>
				<IconSparkles size={18} />
				{processing ? 'Processing...' : 'Optimize All'}
			</button>
		</div>
	</header>

	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-value">{stats.total}</div>
			<div class="stat-label">Total Images</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{stats.withAlt}</div>
			<div class="stat-label">With Alt Text</div>
		</div>
		<div class="stat-card">
			<div class="stat-value" style="color: {getScoreColor(stats.avgScore)}">{stats.avgScore}%</div>
			<div class="stat-label">Avg SEO Score</div>
		</div>
		<div class="stat-card">
			<div class="stat-value" style="color: {stats.issues > 0 ? '#ef4444' : '#10b981'}">
				{stats.issues}
			</div>
			<div class="stat-label">Issues Found</div>
		</div>
	</div>

	<div class="tabs">
		<button
			class="tab"
			class:active={activeTab === 'images'}
			onclick={() => (activeTab = 'images')}
		>
			<IconPhoto size={18} />
			Images
		</button>
		<button
			class="tab"
			class:active={activeTab === 'settings'}
			onclick={() => (activeTab = 'settings')}
		>
			<IconSettings size={18} />
			Settings
		</button>
	</div>

	{#if activeTab === 'images'}
		<div class="images-section">
			{#if loading}
				<div class="loading">Scanning for images...</div>
			{:else if images.length === 0}
				<div class="empty-state">
					<IconPhoto size={48} />
					<p>No images found. Click "Scan Images" to analyze your content.</p>
				</div>
			{:else}
				<div class="images-list">
					{#each images as image}
						<div class="image-item">
							<div class="image-preview">
								<img src={image.src} alt={image.generatedAlt || 'Image preview'} />
								<div class="score-badge" style="background: {getScoreColor(image.seoScore)}">
									{image.seoScore}
								</div>
							</div>
							<div class="image-details">
								<div class="image-filename">{image.filename}.{image.extension}</div>

								<div class="attribute-row">
									<span class="attribute-label">Alt Text:</span>
									<span class="attribute-value" class:empty={!image.generatedAlt}>
										{image.generatedAlt || 'Not set'}
									</span>
								</div>

								<div class="attribute-row">
									<span class="attribute-label">Title:</span>
									<span class="attribute-value" class:empty={!image.generatedTitle}>
										{image.generatedTitle || 'Not set'}
									</span>
								</div>

								{#if image.suggestedFilename}
									<div class="attribute-row">
										<span class="attribute-label">Suggested filename:</span>
										<span class="attribute-value suggested">{image.suggestedFilename}</span>
									</div>
								{/if}

								{#if image.issues.length > 0}
									<div class="issues">
										{#each image.issues as issue}
											<div class="issue error">
												<IconAlertTriangle size={14} />
												{issue}
											</div>
										{/each}
									</div>
								{/if}

								{#if image.recommendations.length > 0}
									<div class="recommendations">
										{#each image.recommendations as rec}
											<div class="recommendation">
												<IconCheck size={14} />
												{rec}
											</div>
										{/each}
									</div>
								{/if}
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{:else}
		<div class="settings-section">
			<div class="settings-card">
				<h3>Alt Text Settings</h3>
				<div class="setting-row">
					<label class="checkbox-label">
						<input id="alt-text-enabled" name="alt-text-enabled" type="checkbox" bind:checked={settings.altTextEnabled} />
						Enable auto alt text generation
					</label>
				</div>
				<div class="setting-row">
					<label for="altFormat">Alt Text Format</label>
					<input
						type="text"
						id="altFormat"
						bind:value={settings.altTextFormat}
						placeholder="%filename%"
					/>
					<span class="hint">Variables: %filename%, %title%, %site%, %sep%</span>
				</div>
				<div class="setting-row">
					<label for="altCase">Case Transformation</label>
					<select id="altCase" bind:value={settings.altTextCase}>
						{#each caseOptions as opt}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>
				</div>
				<div class="setting-row inline">
					<div>
						<label for="altPrefix">Prefix</label>
						<input type="text" id="altPrefix" bind:value={settings.altTextPrefix} />
					</div>
					<div>
						<label for="altSuffix">Suffix</label>
						<input type="text" id="altSuffix" bind:value={settings.altTextSuffix} />
					</div>
				</div>
				<div class="setting-row">
					<label class="checkbox-label">
						<input id="alt-strip-numbers" name="alt-strip-numbers" type="checkbox" bind:checked={settings.altTextStripNumbers} />
						Remove numbers from filename
					</label>
				</div>
				<div class="setting-row">
					<label class="checkbox-label">
						<input id="alt-strip-special" name="alt-strip-special" type="checkbox" bind:checked={settings.altTextStripSpecialChars} />
						Remove special characters
					</label>
				</div>
			</div>

			<div class="settings-card">
				<h3>Title Attribute Settings</h3>
				<div class="setting-row">
					<label class="checkbox-label">
						<input id="title-enabled" name="title-enabled" type="checkbox" bind:checked={settings.titleEnabled} />
						Enable auto title generation
					</label>
				</div>
				<div class="setting-row">
					<label for="titleFormat">Title Format</label>
					<input
						type="text"
						id="titleFormat"
						bind:value={settings.titleFormat}
						placeholder="%filename% %sep% %site%"
					/>
				</div>
				<div class="setting-row">
					<label for="titleCase">Case Transformation</label>
					<select id="titleCase" bind:value={settings.titleCase}>
						{#each caseOptions as opt}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>
				</div>
			</div>

			<div class="settings-card">
				<h3>General Settings</h3>
				<div class="setting-row">
					<label for="siteName">Site Name</label>
					<input type="text" id="siteName" bind:value={settings.siteName} />
				</div>
				<div class="setting-row">
					<label for="separator">Separator</label>
					<input type="text" id="separator" bind:value={settings.separator} maxlength="5" />
				</div>
				<div class="setting-row">
					<label class="checkbox-label">
						<input id="overwrite-existing" name="overwrite-existing" type="checkbox" bind:checked={settings.overwriteExisting} />
						Overwrite existing alt/title attributes
					</label>
				</div>
			</div>

			<div class="settings-actions">
				<button class="btn-secondary" onclick={resetSettings}>Reset to Defaults</button>
				<button class="btn-primary" onclick={saveSettings}>Save Settings</button>
			</div>
		</div>
	{/if}
</div>

<style>
	.image-seo-page {
		padding: 2rem;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
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
		gap: 1rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		border: none;
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
		color: #374151;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover {
		background: #f9fafb;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 1.5rem;
		text-align: center;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.25rem;
	}

	.stat-label {
		color: #666;
		font-size: 0.85rem;
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		border-bottom: 1px solid #e5e5e5;
		padding-bottom: 0.5rem;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: transparent;
		border: none;
		border-radius: 6px 6px 0 0;
		cursor: pointer;
		color: #666;
		font-weight: 500;
		transition: all 0.2s;
	}

	.tab:hover {
		background: #f9fafb;
	}

	.tab.active {
		background: #3b82f6;
		color: white;
	}

	.images-section,
	.settings-section {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 12px;
		padding: 1.5rem;
	}

	.loading,
	.empty-state {
		text-align: center;
		padding: 3rem;
		color: #666;
	}

	.images-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.image-item {
		display: flex;
		gap: 1.5rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 8px;
	}

	.image-preview {
		position: relative;
		width: 120px;
		height: 120px;
		flex-shrink: 0;
		border-radius: 6px;
		overflow: hidden;
		background: #e5e5e5;
	}

	.image-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.score-badge {
		position: absolute;
		top: 4px;
		right: 4px;
		padding: 2px 8px;
		border-radius: 4px;
		color: white;
		font-size: 0.75rem;
		font-weight: 700;
	}

	.image-details {
		flex: 1;
		min-width: 0;
	}

	.image-filename {
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 0.75rem;
		font-size: 0.9rem;
	}

	.attribute-row {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
		font-size: 0.85rem;
	}

	.attribute-label {
		color: #666;
		min-width: 100px;
	}

	.attribute-value {
		color: #1a1a1a;
		flex: 1;
	}

	.attribute-value.empty {
		color: #ef4444;
		font-style: italic;
	}

	.attribute-value.suggested {
		color: #3b82f6;
		font-family: monospace;
		font-size: 0.8rem;
	}

	.issues,
	.recommendations {
		margin-top: 0.75rem;
	}

	.issue,
	.recommendation {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		margin-bottom: 0.25rem;
	}

	.issue.error {
		color: #ef4444;
	}

	.recommendation {
		color: #10b981;
	}

	.settings-card {
		background: #f9fafb;
		border-radius: 8px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.settings-card h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1.25rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid #e5e5e5;
	}

	.setting-row {
		margin-bottom: 1rem;
	}

	.setting-row.inline {
		display: flex;
		gap: 1rem;
	}

	.setting-row.inline > div {
		flex: 1;
	}

	.setting-row label {
		display: block;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.setting-row input[type='text'],
	.setting-row select {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
	}

	.setting-row input:focus,
	.setting-row select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.hint {
		display: block;
		font-size: 0.8rem;
		color: #666;
		margin-top: 0.5rem;
	}

	.settings-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 2rem;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 768px) {
		.image-seo-page {
			padding: 1rem;
		}

		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			flex-direction: column;
		}

		.image-item {
			flex-direction: column;
		}

		.image-preview {
			width: 100%;
			height: 200px;
		}

		.setting-row.inline {
			flex-direction: column;
		}
	}
</style>
