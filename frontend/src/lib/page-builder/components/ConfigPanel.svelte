<script lang="ts">
	/**
	 * Configuration Panel - Block Settings Editor
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * 
	 * Right-side panel for configuring selected block properties.
	 */

	import type { BuilderStore } from '../store.svelte';
	import type { ComponentType, CourseHeaderConfig, VideoPlayerConfig, VideoStackConfig, ClassDownloadsConfig, SpacerConfig, DividerConfig } from '../types';
	import { getComponentByType } from '../registry';

	interface Props {
		store: BuilderStore;
	}

	let { store }: Props = $props();

	const selectedBlock = $derived(store.selectedBlock);
	const componentInfo = $derived(selectedBlock ? getComponentByType(selectedBlock.type) : null);

	function updateConfig(updates: Record<string, unknown>) {
		if (selectedBlock) {
			store.updateBlockConfig(selectedBlock.id, updates);
		}
	}
</script>

<aside class="config-panel" class:has-selection={!!selectedBlock}>
	{#if selectedBlock && componentInfo}
		<div class="panel-header">
			<span class="panel-icon">{componentInfo.icon}</span>
			<h2>{componentInfo.name}</h2>
			<button class="close-btn" onclick={() => store.clearSelection()} aria-label="Close panel">
				<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
			</button>
		</div>

		<div class="panel-content">
			<!-- Course Header Config -->
			{#if selectedBlock.type === 'course-header'}
				{@const config = selectedBlock.config as CourseHeaderConfig}
				<div class="config-section">
					<label class="config-label" for="config-title">Title</label>
					<input 
						id="config-title"
						type="text" 
						class="config-input"
						value={config.title}
						oninput={(e) => updateConfig({ title: (e.target as HTMLInputElement).value })}
					/>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-subtitle">Subtitle</label>
					<input 
						id="config-subtitle"
						type="text" 
						class="config-input"
						value={config.subtitle ?? ''}
						oninput={(e) => updateConfig({ subtitle: (e.target as HTMLInputElement).value })}
						placeholder="Optional subtitle"
					/>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-description">Description</label>
					<textarea
						id="config-description" 
						class="config-textarea"
						value={config.description ?? ''}
						oninput={(e) => updateConfig({ description: (e.target as HTMLTextAreaElement).value })}
						placeholder="Optional description"
						rows="3"
					></textarea>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-bgcolor">Background Color</label>
					<div class="color-input-group">
						<input 
							id="config-bgcolor"
							type="color" 
							class="config-color"
							value={config.backgroundColor ?? '#143E59'}
							oninput={(e) => updateConfig({ backgroundColor: (e.target as HTMLInputElement).value })}
						/>
						<input 
							type="text" 
							class="config-input"
							value={config.backgroundColor ?? '#143E59'}
							oninput={(e) => updateConfig({ backgroundColor: (e.target as HTMLInputElement).value })}
						/>
					</div>
				</div>
				<div class="config-section">
					<label class="config-checkbox">
						<input 
							type="checkbox" 
							checked={config.showLoginButton ?? true}
							onchange={(e) => updateConfig({ showLoginButton: (e.target as HTMLInputElement).checked })}
						/>
						Show Login Button
					</label>
				</div>
				{#if config.showLoginButton}
					<div class="config-section">
						<label class="config-label" for="config-btn-text">Button Text</label>
						<input 
							id="config-btn-text"
							type="text" 
							class="config-input"
							value={config.loginButtonText ?? 'LOGIN TO THE CLASSROOM'}
							oninput={(e) => updateConfig({ loginButtonText: (e.target as HTMLInputElement).value })}
						/>
					</div>
				{/if}

			<!-- Video Player Config -->
			{:else if selectedBlock.type === 'video-player'}
				{@const config = selectedBlock.config as VideoPlayerConfig}
				<div class="config-section">
					<label class="config-label" for="config-video-title">Video Title</label>
					<input 
						id="config-video-title"
						type="text" 
						class="config-input"
						value={config.title}
						oninput={(e) => updateConfig({ title: (e.target as HTMLInputElement).value })}
					/>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-video-subtitle">Subtitle / Date</label>
					<input 
						id="config-video-subtitle"
						type="text" 
						class="config-input"
						value={config.subtitle ?? ''}
						oninput={(e) => updateConfig({ subtitle: (e.target as HTMLInputElement).value })}
						placeholder="e.g., Wed, Jan 08, 2025, PM - Henry"
					/>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-video-guid">Bunny Video GUID</label>
					<input 
						id="config-video-guid"
						type="text" 
						class="config-input"
						value={config.bunnyVideoGuid ?? ''}
						oninput={(e) => updateConfig({ bunnyVideoGuid: (e.target as HTMLInputElement).value })}
						placeholder="Enter video GUID or upload"
					/>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-library-id">Bunny Library ID</label>
					<input 
						id="config-library-id"
						type="number" 
						class="config-input"
						value={config.bunnyLibraryId ?? ''}
						oninput={(e) => updateConfig({ bunnyLibraryId: parseInt((e.target as HTMLInputElement).value) || undefined })}
						placeholder="Library ID"
					/>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-thumbnail">Thumbnail URL</label>
					<input 
						id="config-thumbnail"
						type="text" 
						class="config-input"
						value={config.thumbnailUrl ?? ''}
						oninput={(e) => updateConfig({ thumbnailUrl: (e.target as HTMLInputElement).value })}
						placeholder="Optional thumbnail"
					/>
				</div>
				<div class="config-section upload-section">
					<button class="upload-btn">
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" x2="12" y1="3" y2="15"/></svg>
						Upload Video
					</button>
					<button class="upload-btn secondary">
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/></svg>
						Upload Thumbnail
					</button>
				</div>

			<!-- Spacer Config -->
			{:else if selectedBlock.type === 'spacer'}
				{@const config = selectedBlock.config as SpacerConfig}
				<div class="config-section">
					<label class="config-label" for="config-height">Height (px)</label>
					<div class="slider-group">
						<input 
							id="config-height"
							type="range" 
							class="config-slider"
							min="10"
							max="200"
							step="5"
							value={config.height ?? 40}
							oninput={(e) => updateConfig({ height: parseInt((e.target as HTMLInputElement).value) })}
						/>
						<span class="slider-value">{config.height ?? 40}px</span>
					</div>
				</div>

			<!-- Divider Config -->
			{:else if selectedBlock.type === 'divider'}
				{@const config = selectedBlock.config as DividerConfig}
				<div class="config-section">
					<label class="config-label" for="config-style">Style</label>
					<select 
						id="config-style"
						class="config-select"
						value={config.style ?? 'solid'}
						onchange={(e) => updateConfig({ style: (e.target as HTMLSelectElement).value })}
					>
						<option value="solid">Solid</option>
						<option value="dashed">Dashed</option>
						<option value="dotted">Dotted</option>
					</select>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-divider-color">Color</label>
					<div class="color-input-group">
						<input 
							id="config-divider-color"
							type="color" 
							class="config-color"
							value={config.color ?? '#E0E0E0'}
							oninput={(e) => updateConfig({ color: (e.target as HTMLInputElement).value })}
						/>
						<input 
							type="text" 
							class="config-input"
							value={config.color ?? '#E0E0E0'}
							oninput={(e) => updateConfig({ color: (e.target as HTMLInputElement).value })}
						/>
					</div>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-thickness">Thickness (px)</label>
					<input 
						id="config-thickness"
						type="number" 
						class="config-input"
						min="1"
						max="10"
						value={config.thickness ?? 1}
						oninput={(e) => updateConfig({ thickness: parseInt((e.target as HTMLInputElement).value) })}
					/>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-margin-top">Margin Top (px)</label>
					<input 
						id="config-margin-top"
						type="number" 
						class="config-input"
						min="0"
						max="100"
						value={config.marginTop ?? 20}
						oninput={(e) => updateConfig({ marginTop: parseInt((e.target as HTMLInputElement).value) })}
					/>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-margin-bottom">Margin Bottom (px)</label>
					<input 
						id="config-margin-bottom"
						type="number" 
						class="config-input"
						min="0"
						max="100"
						value={config.marginBottom ?? 20}
						oninput={(e) => updateConfig({ marginBottom: parseInt((e.target as HTMLInputElement).value) })}
					/>
				</div>

			<!-- Class Downloads Config -->
			{:else if selectedBlock.type === 'class-downloads'}
				{@const config = selectedBlock.config as ClassDownloadsConfig}
				<div class="config-section">
					<label class="config-label" for="config-section-title">Section Title</label>
					<input 
						id="config-section-title"
						type="text" 
						class="config-input"
						value={config.title ?? 'Class Downloads'}
						oninput={(e) => updateConfig({ title: (e.target as HTMLInputElement).value })}
					/>
				</div>
				<div class="config-section">
					<label class="config-label" for="config-max-height">Max Height</label>
					<select 
						id="config-max-height"
						class="config-select"
						value={config.maxHeight ?? '400px'}
						onchange={(e) => updateConfig({ maxHeight: (e.target as HTMLSelectElement).value })}
					>
						<option value="300px">300px</option>
						<option value="400px">400px</option>
						<option value="500px">500px</option>
						<option value="600px">600px</option>
						<option value="auto">Auto (no limit)</option>
					</select>
				</div>
				<div class="config-info">
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
					<span>Files are managed in the Downloads tab</span>
				</div>

			<!-- Video Stack Config -->
			{:else if selectedBlock.type === 'video-stack'}
				{@const config = selectedBlock.config as VideoStackConfig}
				<div class="config-section">
					<label class="config-label" for="config-sort-order">Sort Order</label>
					<select 
						id="config-sort-order"
						class="config-select"
						value={config.sortOrder ?? 'newest'}
						onchange={(e) => updateConfig({ sortOrder: (e.target as HTMLSelectElement).value })}
					>
						<option value="newest">Newest First</option>
						<option value="oldest">Oldest First</option>
						<option value="manual">Manual Order</option>
					</select>
				</div>
				<div class="config-section">
					<label class="config-checkbox">
						<input 
							type="checkbox" 
							checked={config.showDates ?? true}
							onchange={(e) => updateConfig({ showDates: (e.target as HTMLInputElement).checked })}
						/>
						Show Dates
					</label>
				</div>
				<div class="config-section">
					<span class="config-label">Videos ({config.videos?.length ?? 0})</span>
					<button class="add-video-btn">
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" x2="12" y1="5" y2="19"/><line x1="5" x2="19" y1="12" y2="12"/></svg>
						Add Video
					</button>
				</div>
			{/if}
		</div>

		<div class="panel-footer">
			<button class="delete-block-btn" onclick={() => { if (selectedBlock) store.removeBlock(selectedBlock.id); }}>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>
				Delete Block
			</button>
		</div>
	{:else}
		<div class="no-selection">
			<div class="no-selection-icon">👆</div>
			<h3>Select a Block</h3>
			<p>Click on a block in the canvas to edit its properties</p>
		</div>
	{/if}
</aside>

<style>
	.config-panel {
		width: 300px;
		height: 100%;
		background: #FFFFFF;
		border-left: 1px solid #E5E7EB;
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
		transition: width 0.2s ease;
	}

	.panel-header {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 16px 20px;
		border-bottom: 1px solid #E5E7EB;
	}

	.panel-icon {
		font-size: 20px;
	}

	.panel-header h2 {
		flex: 1;
		font-size: 16px;
		font-weight: 600;
		color: #1F2937;
		margin: 0;
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #6B7280;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.close-btn:hover {
		background: #F3F4F6;
		color: #1F2937;
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 20px;
	}

	.config-section {
		margin-bottom: 20px;
	}

	.config-label {
		display: block;
		font-size: 12px;
		font-weight: 600;
		color: #374151;
		margin-bottom: 6px;
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	.config-input,
	.config-select,
	.config-textarea {
		width: 100%;
		padding: 10px 12px;
		background: #F9FAFB;
		border: 1px solid #E5E7EB;
		border-radius: 6px;
		font-size: 14px;
		color: #1F2937;
		transition: border-color 0.15s ease;
		box-sizing: border-box;
	}

	.config-input:focus,
	.config-select:focus,
	.config-textarea:focus {
		outline: none;
		border-color: #143E59;
		background: #FFFFFF;
	}

	.config-textarea {
		resize: vertical;
		font-family: inherit;
	}

	.config-color {
		width: 40px;
		height: 40px;
		padding: 2px;
		border: 1px solid #E5E7EB;
		border-radius: 6px;
		cursor: pointer;
	}

	.color-input-group {
		display: flex;
		gap: 8px;
	}

	.color-input-group .config-input {
		flex: 1;
	}

	.config-checkbox {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 14px;
		color: #374151;
		cursor: pointer;
	}

	.config-checkbox input {
		width: 18px;
		height: 18px;
		accent-color: #143E59;
	}

	.slider-group {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.config-slider {
		flex: 1;
		height: 6px;
		-webkit-appearance: none;
		appearance: none;
		background: #E5E7EB;
		border-radius: 3px;
		cursor: pointer;
	}

	.config-slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		width: 18px;
		height: 18px;
		background: #143E59;
		border-radius: 50%;
		cursor: pointer;
	}

	.slider-value {
		min-width: 50px;
		font-size: 14px;
		font-weight: 500;
		color: #1F2937;
		text-align: right;
	}

	.upload-section {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.upload-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px;
		background: #143E59;
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.upload-btn:hover {
		background: #0F2D42;
	}

	.upload-btn.secondary {
		background: #F3F4F6;
		color: #374151;
		border: 1px solid #E5E7EB;
	}

	.upload-btn.secondary:hover {
		background: #E5E7EB;
	}

	.add-video-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		padding: 12px;
		background: #F3F4F6;
		border: 2px dashed #D1D5DB;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 500;
		color: #6B7280;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.add-video-btn:hover {
		background: #E5E7EB;
		border-color: #9CA3AF;
		color: #374151;
	}

	.config-info {
		display: flex;
		align-items: flex-start;
		gap: 8px;
		padding: 12px;
		background: #F0F9FF;
		border: 1px solid #BAE6FD;
		border-radius: 6px;
		font-size: 13px;
		color: #0369A1;
	}

	.config-info svg {
		flex-shrink: 0;
		margin-top: 1px;
	}

	.panel-footer {
		padding: 16px 20px;
		border-top: 1px solid #E5E7EB;
	}

	.delete-block-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		padding: 12px;
		background: #FEF2F2;
		color: #DC2626;
		border: 1px solid #FECACA;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.delete-block-btn:hover {
		background: #FEE2E2;
		border-color: #F87171;
	}

	.no-selection {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 40px;
		text-align: center;
	}

	.no-selection-icon {
		font-size: 48px;
		margin-bottom: 16px;
	}

	.no-selection h3 {
		font-size: 16px;
		font-weight: 600;
		color: #1F2937;
		margin: 0 0 8px 0;
	}

	.no-selection p {
		font-size: 14px;
		color: #6B7280;
		margin: 0;
	}

	/* Scrollbar */
	.panel-content::-webkit-scrollbar {
		width: 6px;
	}

	.panel-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.panel-content::-webkit-scrollbar-thumb {
		background: #D1D5DB;
		border-radius: 3px;
	}

	.panel-content::-webkit-scrollbar-thumb:hover {
		background: #9CA3AF;
	}
</style>
