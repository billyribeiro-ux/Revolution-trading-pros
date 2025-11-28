<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let settings = {
		enabled: true,
		title: 'In This Article',
		position: 'inline' as 'inline' | 'sidebar' | 'floating',
		minHeadings: 2,
		maxDepth: 4,
		showNumbers: true,
		collapsible: true,
		defaultExpanded: true,
		showProgress: true,
		smoothScroll: true,
		highlightActive: true,
		showFloatingWidget: true,
		floatingWidgetScrollThreshold: 400
	};

	const dispatch = createEventDispatcher();

	function handleChange() {
		dispatch('change', settings);
	}

	function resetToDefaults() {
		settings = {
			enabled: true,
			title: 'In This Article',
			position: 'inline',
			minHeadings: 2,
			maxDepth: 4,
			showNumbers: true,
			collapsible: true,
			defaultExpanded: true,
			showProgress: true,
			smoothScroll: true,
			highlightActive: true,
			showFloatingWidget: true,
			floatingWidgetScrollThreshold: 400
		};
		handleChange();
	}
</script>

<div class="toc-settings-panel">
	<div class="panel-header">
		<div class="header-content">
			<div class="header-icon">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<line x1="3" y1="6" x2="21" y2="6"></line>
					<line x1="3" y1="12" x2="15" y2="12"></line>
					<line x1="3" y1="18" x2="18" y2="18"></line>
				</svg>
			</div>
			<div>
				<h3>Table of Contents Settings</h3>
				<p>Configure how the TOC appears on blog posts</p>
			</div>
		</div>
		<button class="reset-btn" on:click={resetToDefaults}>
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<polyline points="1 4 1 10 7 10"></polyline>
				<path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"></path>
			</svg>
			Reset to Defaults
		</button>
	</div>

	<div class="settings-grid">
		<!-- Enable/Disable TOC -->
		<div class="setting-group full-width">
			<label class="toggle-setting">
				<input type="checkbox" bind:checked={settings.enabled} on:change={handleChange} />
				<span class="toggle-slider"></span>
				<div class="toggle-label">
					<span class="label-title">Enable Table of Contents</span>
					<span class="label-description">Show TOC on blog posts with multiple headings</span>
				</div>
			</label>
		</div>

		{#if settings.enabled}
			<!-- Title -->
			<div class="setting-group">
				<label class="setting-label" for="toc-title">
					<span class="label-title">TOC Title</span>
					<span class="label-description">The heading displayed above the table of contents</span>
				</label>
				<input
					type="text"
					id="toc-title"
					bind:value={settings.title}
					on:input={handleChange}
					placeholder="In This Article"
					class="text-input"
				/>
			</div>

			<!-- Position -->
			<div class="setting-group">
				<label class="setting-label">
					<span class="label-title">Display Position</span>
					<span class="label-description">Where to show the inline TOC</span>
				</label>
				<div class="radio-group">
					<label class="radio-option">
						<input
							type="radio"
							name="position"
							value="inline"
							bind:group={settings.position}
							on:change={handleChange}
						/>
						<span class="radio-display">
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
								<line x1="7" y1="8" x2="17" y2="8"></line>
								<line x1="7" y1="12" x2="14" y2="12"></line>
								<line x1="7" y1="16" x2="15" y2="16"></line>
							</svg>
							<span>Inline (Top)</span>
						</span>
					</label>
					<label class="radio-option">
						<input
							type="radio"
							name="position"
							value="sidebar"
							bind:group={settings.position}
							on:change={handleChange}
						/>
						<span class="radio-display">
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
								<line x1="15" y1="3" x2="15" y2="21"></line>
								<line x1="17" y1="8" x2="20" y2="8"></line>
								<line x1="17" y1="12" x2="20" y2="12"></line>
							</svg>
							<span>Sidebar</span>
						</span>
					</label>
				</div>
			</div>

			<!-- Min Headings -->
			<div class="setting-group">
				<label class="setting-label" for="min-headings">
					<span class="label-title">Minimum Headings</span>
					<span class="label-description">Only show TOC if this many headings exist</span>
				</label>
				<div class="number-input-wrapper">
					<button
						class="number-btn"
						on:click={() => {
							if (settings.minHeadings > 1) {
								settings.minHeadings--;
								handleChange();
							}
						}}
					>
						-
					</button>
					<input
						type="number"
						id="min-headings"
						bind:value={settings.minHeadings}
						on:change={handleChange}
						min="1"
						max="10"
						class="number-input"
					/>
					<button
						class="number-btn"
						on:click={() => {
							if (settings.minHeadings < 10) {
								settings.minHeadings++;
								handleChange();
							}
						}}
					>
						+
					</button>
				</div>
			</div>

			<!-- Max Depth -->
			<div class="setting-group">
				<label class="setting-label" for="max-depth">
					<span class="label-title">Maximum Depth</span>
					<span class="label-description">Deepest heading level to include (H2-H6)</span>
				</label>
				<select id="max-depth" bind:value={settings.maxDepth} on:change={handleChange} class="select-input">
					<option value={2}>H2 only</option>
					<option value={3}>H2 - H3</option>
					<option value={4}>H2 - H4</option>
					<option value={5}>H2 - H5</option>
					<option value={6}>H2 - H6 (All)</option>
				</select>
			</div>

			<!-- Display Options -->
			<div class="setting-group full-width">
				<span class="section-title">Display Options</span>
				<div class="checkbox-grid">
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={settings.showNumbers} on:change={handleChange} />
						<span class="checkbox-display"></span>
						<span>Show section numbers</span>
					</label>
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={settings.collapsible} on:change={handleChange} />
						<span class="checkbox-display"></span>
						<span>Make collapsible</span>
					</label>
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={settings.defaultExpanded} on:change={handleChange} />
						<span class="checkbox-display"></span>
						<span>Expanded by default</span>
					</label>
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={settings.showProgress} on:change={handleChange} />
						<span class="checkbox-display"></span>
						<span>Show reading progress</span>
					</label>
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={settings.smoothScroll} on:change={handleChange} />
						<span class="checkbox-display"></span>
						<span>Smooth scroll navigation</span>
					</label>
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={settings.highlightActive} on:change={handleChange} />
						<span class="checkbox-display"></span>
						<span>Highlight active section</span>
					</label>
				</div>
			</div>

			<!-- Floating Widget Settings -->
			<div class="setting-group full-width">
				<label class="toggle-setting">
					<input type="checkbox" bind:checked={settings.showFloatingWidget} on:change={handleChange} />
					<span class="toggle-slider"></span>
					<div class="toggle-label">
						<span class="label-title">Show Floating Widget</span>
						<span class="label-description">Display a floating TOC button for quick navigation</span>
					</div>
				</label>
			</div>

			{#if settings.showFloatingWidget}
				<div class="setting-group">
					<label class="setting-label" for="scroll-threshold">
						<span class="label-title">Show After Scrolling</span>
						<span class="label-description">Pixels to scroll before showing floating widget</span>
					</label>
					<div class="range-input-wrapper">
						<input
							type="range"
							id="scroll-threshold"
							bind:value={settings.floatingWidgetScrollThreshold}
							on:input={handleChange}
							min="100"
							max="1000"
							step="50"
							class="range-input"
						/>
						<span class="range-value">{settings.floatingWidgetScrollThreshold}px</span>
					</div>
				</div>
			{/if}
		{/if}
	</div>

	<!-- Preview -->
	{#if settings.enabled}
		<div class="preview-section">
			<h4>Preview</h4>
			<div class="preview-container">
				<div class="preview-toc">
					<div class="preview-header">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="3" y1="6" x2="21" y2="6"></line>
							<line x1="3" y1="12" x2="15" y2="12"></line>
							<line x1="3" y1="18" x2="18" y2="18"></line>
						</svg>
						<span>{settings.title}</span>
						{#if settings.collapsible}
							<svg class="chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<polyline points="6 9 12 15 18 9"></polyline>
							</svg>
						{/if}
					</div>
					{#if settings.defaultExpanded}
						<div class="preview-list">
							<div class="preview-item active">
								{#if settings.showNumbers}<span class="num">1</span>{/if}
								<span>Introduction</span>
							</div>
							<div class="preview-item">
								{#if settings.showNumbers}<span class="num">2</span>{/if}
								<span>Getting Started</span>
							</div>
							{#if settings.maxDepth >= 3}
								<div class="preview-item nested">
									{#if settings.showNumbers}<span class="num">2.1</span>{/if}
									<span>Prerequisites</span>
								</div>
							{/if}
							<div class="preview-item">
								{#if settings.showNumbers}<span class="num">3</span>{/if}
								<span>Conclusion</span>
							</div>
						</div>
					{/if}
					{#if settings.showProgress}
						<div class="preview-progress">
							<span>25% complete</span>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.toc-settings-panel {
		background: rgba(30, 41, 59, 0.5);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.5rem;
		background: rgba(15, 23, 42, 0.5);
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		border-radius: 12px;
	}

	.header-icon svg {
		width: 24px;
		height: 24px;
		color: white;
	}

	.panel-header h3 {
		font-size: 1.125rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.panel-header p {
		font-size: 0.875rem;
		color: #94a3b8;
		margin: 0.25rem 0 0;
	}

	.reset-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.reset-btn:hover {
		background: rgba(248, 113, 113, 0.1);
		border-color: rgba(248, 113, 113, 0.3);
		color: #f87171;
	}

	.reset-btn svg {
		width: 16px;
		height: 16px;
	}

	.settings-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
		padding: 1.5rem;
	}

	.setting-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.setting-group.full-width {
		grid-column: 1 / -1;
	}

	.setting-label {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.label-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.label-description {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.section-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	/* Toggle Switch */
	.toggle-setting {
		display: flex;
		align-items: center;
		gap: 1rem;
		cursor: pointer;
	}

	.toggle-setting input {
		display: none;
	}

	.toggle-slider {
		width: 48px;
		height: 26px;
		background: rgba(148, 163, 184, 0.2);
		border-radius: 13px;
		position: relative;
		transition: all 0.3s ease;
		flex-shrink: 0;
	}

	.toggle-slider::before {
		content: '';
		position: absolute;
		top: 3px;
		left: 3px;
		width: 20px;
		height: 20px;
		background: white;
		border-radius: 50%;
		transition: all 0.3s ease;
	}

	.toggle-setting input:checked + .toggle-slider {
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
	}

	.toggle-setting input:checked + .toggle-slider::before {
		transform: translateX(22px);
	}

	.toggle-label {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	/* Text Input */
	.text-input,
	.select-input {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
		transition: all 0.2s ease;
	}

	.text-input:focus,
	.select-input:focus {
		outline: none;
		border-color: #60a5fa;
		box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.1);
	}

	.select-input {
		cursor: pointer;
	}

	/* Number Input */
	.number-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0;
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		overflow: hidden;
	}

	.number-btn {
		width: 40px;
		height: 40px;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		color: #94a3b8;
		font-size: 1.25rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.number-btn:hover {
		background: rgba(96, 165, 250, 0.2);
		color: #60a5fa;
	}

	.number-input {
		width: 60px;
		padding: 0.5rem;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.875rem;
		text-align: center;
	}

	.number-input:focus {
		outline: none;
	}

	/* Radio Group */
	.radio-group {
		display: flex;
		gap: 0.75rem;
	}

	.radio-option {
		flex: 1;
		cursor: pointer;
	}

	.radio-option input {
		display: none;
	}

	.radio-display {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.5);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		transition: all 0.2s ease;
	}

	.radio-display svg {
		width: 32px;
		height: 32px;
		color: #94a3b8;
	}

	.radio-display span {
		font-size: 0.75rem;
		color: #94a3b8;
		font-weight: 500;
	}

	.radio-option input:checked + .radio-display {
		border-color: #60a5fa;
		background: rgba(96, 165, 250, 0.1);
	}

	.radio-option input:checked + .radio-display svg,
	.radio-option input:checked + .radio-display span {
		color: #60a5fa;
	}

	/* Checkbox Grid */
	.checkbox-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.75rem;
	}

	.checkbox-option {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.3);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.checkbox-option:hover {
		background: rgba(15, 23, 42, 0.5);
	}

	.checkbox-option input {
		display: none;
	}

	.checkbox-display {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(148, 163, 184, 0.3);
		border-radius: 4px;
		position: relative;
		transition: all 0.2s ease;
		flex-shrink: 0;
	}

	.checkbox-option input:checked + .checkbox-display {
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		border-color: #3b82f6;
	}

	.checkbox-option input:checked + .checkbox-display::after {
		content: '';
		position: absolute;
		top: 2px;
		left: 6px;
		width: 5px;
		height: 10px;
		border: solid white;
		border-width: 0 2px 2px 0;
		transform: rotate(45deg);
	}

	.checkbox-option span {
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	/* Range Input */
	.range-input-wrapper {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.range-input {
		flex: 1;
		-webkit-appearance: none;
		appearance: none;
		height: 6px;
		background: rgba(148, 163, 184, 0.2);
		border-radius: 3px;
		outline: none;
	}

	.range-input::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 20px;
		height: 20px;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		border-radius: 50%;
		cursor: pointer;
		border: 2px solid white;
		box-shadow: 0 2px 8px rgba(59, 130, 246, 0.3);
	}

	.range-value {
		font-size: 0.875rem;
		color: #60a5fa;
		font-weight: 600;
		min-width: 60px;
		text-align: right;
	}

	/* Preview Section */
	.preview-section {
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		background: rgba(15, 23, 42, 0.3);
	}

	.preview-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 0 0 1rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.preview-container {
		display: flex;
		justify-content: center;
	}

	.preview-toc {
		width: 280px;
		background: rgba(30, 41, 59, 0.95);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		overflow: hidden;
	}

	.preview-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1rem;
		background: rgba(15, 23, 42, 0.5);
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.preview-header svg {
		width: 16px;
		height: 16px;
	}

	.preview-header .chevron {
		margin-left: auto;
	}

	.preview-list {
		padding: 0.5rem 0;
	}

	.preview-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.preview-item.active {
		color: #60a5fa;
		background: rgba(96, 165, 250, 0.1);
	}

	.preview-item.nested {
		padding-left: 2rem;
		font-size: 0.75rem;
	}

	.preview-item .num {
		font-size: 0.6875rem;
		background: rgba(96, 165, 250, 0.1);
		color: #60a5fa;
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		font-weight: 600;
	}

	.preview-progress {
		padding: 0.625rem 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		font-size: 0.75rem;
		color: #94a3b8;
		background: rgba(15, 23, 42, 0.3);
	}

	/* Responsive */
	@media (max-width: 768px) {
		.panel-header {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.settings-grid {
			grid-template-columns: 1fr;
		}

		.checkbox-grid {
			grid-template-columns: 1fr;
		}

		.radio-group {
			flex-direction: column;
		}
	}
</style>
