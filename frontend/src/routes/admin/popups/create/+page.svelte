<script lang="ts">
	import { goto } from '$app/navigation';
	import { IconDeviceFloppy, IconEye, IconArrowLeft } from '$lib/icons';
	import { createPopup } from '$lib/api/popups';
	import {
		defaultPopupConfig,
		type Popup
	} from '$lib/stores/popups';
	import { sanitizePopupContent } from '$lib/utils/sanitize';

	let activeTab: 'content' | 'design' | 'display' | 'buttons' | 'form' = 'content';
	let showPreview = false;
	let saving = false;

	// Initialize with default config - use Required for properties we know are defined
	let popup = {
		...JSON.parse(JSON.stringify(defaultPopupConfig)),
		name: '',
		title: '',
		content: {
			type: 'html',
			data: '<p>Your popup content goes here...</p>'
		},
		buttons: [],
		formFields: []
	} as Partial<Popup> & { displayRules: NonNullable<Popup['displayRules']> };

	async function handleSave() {
		if (!popup.name) {
			alert('Please enter a popup name');
			return;
		}

		saving = true;
		try {
			await createPopup(popup as any);
			alert('Popup created successfully!');
			goto('/admin/popups');
		} catch (error) {
			console.error('Error saving popup:', error);
			alert('Failed to save popup');
		} finally {
			saving = false;
		}
	}

	function addButton() {
		if (!popup.buttons) popup.buttons = [];
		popup.buttons.push({
			text: 'Click Me',
			action: 'close',
			style: 'primary'
		});
		popup.buttons = [...popup.buttons];
	}

	function removeButton(index: number) {
		if (!popup.buttons) return;
		popup.buttons.splice(index, 1);
		popup.buttons = [...popup.buttons];
	}

	function addFormField() {
		if (!popup.formFields) popup.formFields = [];
		popup.formFields.push({
			name: 'field_' + Date.now(),
			type: 'text',
			placeholder: 'Enter value',
			required: false
		});
		popup.formFields = [...popup.formFields];
	}

	function removeFormField(index: number) {
		if (!popup.formFields) return;
		popup.formFields.splice(index, 1);
		popup.formFields = [...popup.formFields];
	}
</script>

<div class="creator-page">
	<!-- Header -->
	<div class="page-header">
		<div class="header-left">
			<a href="/admin/popups" class="back-btn">
				<IconArrowLeft size={20} />
				<span>Back</span>
			</a>
			<div>
				<h1 class="page-title">Create Popup</h1>
				<p class="page-subtitle">Design a custom popup for your site</p>
			</div>
		</div>
		<div class="header-actions">
			<button class="btn-secondary" onclick={() => (showPreview = !showPreview)}>
				<IconEye size={20} />
				<span>{showPreview ? 'Hide' : 'Show'} Preview</span>
			</button>
			<button class="btn-primary" onclick={handleSave} disabled={saving}>
				<IconDeviceFloppy size={20} />
				<span>{saving ? 'Saving...' : 'Save Popup'}</span>
			</button>
		</div>
	</div>

	<div class="creator-layout">
		<!-- Editor Panel -->
		<div class="editor-panel">
			<!-- Tabs -->
			<div class="tabs">
				<button
					class="tab"
					class:active={activeTab === 'content'}
					onclick={() => (activeTab = 'content')}
				>
					Content
				</button>
				<button
					class="tab"
					class:active={activeTab === 'design'}
					onclick={() => (activeTab = 'design')}
				>
					Design
				</button>
				<button
					class="tab"
					class:active={activeTab === 'display'}
					onclick={() => (activeTab = 'display')}
				>
					Display Rules
				</button>
				<button
					class="tab"
					class:active={activeTab === 'buttons'}
					onclick={() => (activeTab = 'buttons')}
				>
					Buttons
				</button>
				<button
					class="tab"
					class:active={activeTab === 'form'}
					onclick={() => (activeTab = 'form')}
				>
					Form
				</button>
			</div>

			<!-- Tab Content -->
			<div class="tab-content">
				<!-- Content Tab -->
				{#if activeTab === 'content'}
					<div class="form-section">
						<h3 class="section-title">Basic Information</h3>

						<div class="form-group">
							<label for="name">Popup Name (Internal)</label>
							<input
								type="text"
								id="name"
								bind:value={popup.name}
								placeholder="e.g., Summer Sale 2024"
								class="form-input"
							/>
						</div>

						<div class="form-group">
							<label for="title">Popup Title (Visible)</label>
							<input
								type="text"
								id="title"
								bind:value={popup.title}
								placeholder="e.g., Limited Time Offer!"
								class="form-input"
							/>
						</div>

						<div class="form-group">
							<label for="content-type">Content Type</label>
							<select id="content-type" bind:value={popup.content.type} class="form-input">
								<option value="html">HTML</option>
								<option value="markdown">Markdown</option>
							</select>
						</div>

						<div class="form-group">
							<label for="content-data">Content</label>
							<textarea
								id="content-data"
								bind:value={popup.content.data}
								placeholder="Enter your popup content..."
								class="form-textarea"
								rows="10"
							></textarea>
							<p class="form-help">Use HTML tags for formatting</p>
						</div>

						<div class="form-group">
							<label class="checkbox-label">
								<input type="checkbox" bind:checked={popup.isActive} />
								<span>Active (show on site)</span>
							</label>
						</div>

						<div class="form-group">
							<label class="checkbox-label">
								<input type="checkbox" bind:checked={popup.closeButton} />
								<span>Show close button</span>
							</label>
						</div>

						<div class="form-group">
							<label class="checkbox-label">
								<input type="checkbox" bind:checked={popup.closeOnOverlayClick} />
								<span>Close when clicking overlay</span>
							</label>
						</div>
					</div>
				{/if}

				<!-- Design Tab -->
				{#if activeTab === 'design'}
					<div class="form-section">
						<h3 class="section-title">Layout</h3>

						<div class="form-row">
							<div class="form-group">
								<label for="width">Width</label>
								<input type="text" id="width" bind:value={popup.design.width} class="form-input" />
							</div>
							<div class="form-group">
								<label for="max-width">Max Width</label>
								<input
									type="text"
									id="max-width"
									bind:value={popup.design.maxWidth}
									class="form-input"
								/>
							</div>
						</div>

						<div class="form-group">
							<label for="padding">Padding</label>
							<input
								type="text"
								id="padding"
								bind:value={popup.design.padding}
								class="form-input"
							/>
						</div>

						<h3 class="section-title">Colors</h3>

						<div class="form-group">
							<label for="bg-color">Background Color</label>
							<input
								type="color"
								id="bg-color"
								bind:value={popup.design.backgroundColor}
								class="form-color"
							/>
						</div>

						<div class="form-group">
							<label for="text-color">Text Color</label>
							<input
								type="color"
								id="text-color"
								bind:value={popup.design.textColor}
								class="form-color"
							/>
						</div>

						<div class="form-group">
							<label for="border-color">Border Color</label>
							<input
								type="color"
								id="border-color"
								bind:value={popup.design.borderColor}
								class="form-color"
							/>
						</div>

						<h3 class="section-title">Styling</h3>

						<div class="form-group">
							<label for="border-radius">Border Radius</label>
							<input
								type="text"
								id="border-radius"
								bind:value={popup.design.borderRadius}
								class="form-input"
							/>
						</div>

						<div class="form-group">
							<label for="box-shadow">Box Shadow</label>
							<input
								type="text"
								id="box-shadow"
								bind:value={popup.design.boxShadow}
								class="form-input"
							/>
						</div>

						<div class="form-group">
							<label for="backdrop-blur">Backdrop Blur</label>
							<input
								type="text"
								id="backdrop-blur"
								bind:value={popup.design.backdropBlur}
								class="form-input"
							/>
						</div>

						<h3 class="section-title">Animation</h3>

						<div class="form-row">
							<div class="form-group">
								<label for="entrance-anim">Entrance</label>
								<select id="entrance-anim" bind:value={popup.animation.entrance} class="form-input">
									<option value="fade">Fade</option>
									<option value="slide-up">Slide Up</option>
									<option value="slide-down">Slide Down</option>
									<option value="slide-left">Slide Left</option>
									<option value="slide-right">Slide Right</option>
									<option value="zoom">Zoom</option>
									<option value="bounce">Bounce</option>
									<option value="flip">Flip</option>
								</select>
							</div>
							<div class="form-group">
								<label for="exit-anim">Exit</label>
								<select id="exit-anim" bind:value={popup.animation.exit} class="form-input">
									<option value="fade">Fade</option>
									<option value="slide-up">Slide Up</option>
									<option value="slide-down">Slide Down</option>
									<option value="slide-left">Slide Left</option>
									<option value="slide-right">Slide Right</option>
									<option value="zoom">Zoom</option>
									<option value="bounce">Bounce</option>
									<option value="flip">Flip</option>
								</select>
							</div>
						</div>

						<div class="form-group">
							<label for="anim-duration">Duration (ms)</label>
							<input
								type="number"
								id="anim-duration"
								bind:value={popup.animation.duration}
								class="form-input"
							/>
						</div>
					</div>
				{/if}

				<!-- Display Rules Tab -->
				{#if activeTab === 'display'}
					<div class="form-section">
						<h3 class="section-title">Timing</h3>

						<div class="form-group">
							<label for="delay">Delay (seconds)</label>
							<input
								type="number"
								id="delay"
								bind:value={popup.displayRules.delaySeconds}
								class="form-input"
								min="0"
							/>
						</div>

						<div class="form-group">
							<label class="checkbox-label">
								<input type="checkbox" bind:checked={popup.displayRules.showOnScroll} />
								<span>Trigger on scroll</span>
							</label>
						</div>

						{#if popup.displayRules.showOnScroll}
							<div class="form-group indent">
								<label for="scroll-pct">Scroll Percentage</label>
								<input
									type="number"
									id="scroll-pct"
									bind:value={popup.displayRules.scrollPercentage}
									class="form-input"
									min="0"
									max="100"
								/>
							</div>
						{/if}

						<div class="form-group">
							<label class="checkbox-label">
								<input type="checkbox" bind:value={popup.displayRules.showOnExit} />
								<span>Show on exit intent</span>
							</label>
						</div>

						<h3 class="section-title">Frequency</h3>

						<div class="form-group">
							<label for="frequency">Show Frequency</label>
							<select id="frequency" bind:value={popup.displayRules.frequency} class="form-input">
								<option value="always">Every time</option>
								<option value="once-per-session">Once per session</option>
								<option value="once-per-day">Once per day</option>
								<option value="once-per-week">Once per week</option>
								<option value="once-ever">Once ever</option>
							</select>
						</div>

						<h3 class="section-title">Targeting</h3>

						<div class="form-group">
							<label for="pages">Include Pages (one per line)</label>
							<textarea
								id="pages"
								bind:value={popup.displayRules.pages}
								class="form-textarea"
								rows="4"
								placeholder="/*&#10;/products/*&#10;/blog/*"
							></textarea>
							<p class="form-help">Use * for wildcards. Example: /products/*</p>
						</div>

						<div class="form-group">
							<label for="exclude-pages">Exclude Pages (one per line)</label>
							<textarea
								id="exclude-pages"
								bind:value={popup.displayRules.excludePages}
								class="form-textarea"
								rows="4"
								placeholder="/checkout&#10;/cart"
							></textarea>
						</div>

						<div class="form-group">
							<label for="device-target">Device Targeting</label>
							<select
								id="device-target"
								bind:value={popup.displayRules.deviceTargeting}
								class="form-input"
							>
								<option value="all">All devices</option>
								<option value="desktop">Desktop only</option>
								<option value="mobile">Mobile only</option>
								<option value="tablet">Tablet only</option>
							</select>
						</div>
					</div>
				{/if}

				<!-- Buttons Tab -->
				{#if activeTab === 'buttons'}
					<div class="form-section">
						<div class="section-header">
							<h3 class="section-title">Popup Buttons</h3>
							<button class="btn-add" onclick={addButton}>+ Add Button</button>
						</div>

						{#if popup.buttons && popup.buttons.length > 0}
							{#each popup.buttons as button, index}
								<div class="button-config">
									<div class="button-header">
										<h4>Button {index + 1}</h4>
										<button class="btn-remove" onclick={() => removeButton(index)}>Remove</button>
									</div>

									<div class="form-group">
										<label for={`btn-text-${index}`}>Button Text</label>
										<input
											id={`btn-text-${index}`}
											type="text"
											bind:value={button.text}
											class="form-input"
										/>
									</div>

									<div class="form-group">
										<label for={`btn-style-${index}`}>Style</label>
										<select id={`btn-style-${index}`} bind:value={button.style} class="form-input">
											<option value="primary">Primary</option>
											<option value="secondary">Secondary</option>
											<option value="outline">Outline</option>
											<option value="ghost">Ghost</option>
										</select>
									</div>

									<div class="form-group">
										<label for={`btn-action-${index}`}>Action</label>
										<select
											id={`btn-action-${index}`}
											bind:value={button.action}
											class="form-input"
										>
											<option value="close">Close popup</option>
											<option value="submit">Submit form</option>
											<option value="custom">Custom (link)</option>
										</select>
									</div>

									{#if button.action === 'custom'}
										<div class="form-group">
											<label for={`btn-link-${index}`}>Link URL</label>
											<input
												id={`btn-link-${index}`}
												type="text"
												bind:value={button.link}
												class="form-input"
												placeholder="https://..."
											/>
										</div>
									{/if}
								</div>
							{/each}
						{:else}
							<p class="empty-message">No buttons added. Click "Add Button" to create one.</p>
						{/if}
					</div>
				{/if}

				<!-- Form Tab -->
				{#if activeTab === 'form'}
					<div class="form-section">
						<div class="section-header">
							<h3 class="section-title">Form Fields</h3>
							<button class="btn-add" onclick={addFormField}>+ Add Field</button>
						</div>

						<div class="form-group">
							<label for="form-action">Form Action URL</label>
							<input
								type="text"
								id="form-action"
								bind:value={popup.formAction}
								class="form-input"
								placeholder="/api/form-submit"
							/>
							<p class="form-help">API endpoint to receive form submissions</p>
						</div>

						{#if popup.formFields && popup.formFields.length > 0}
							{#each popup.formFields as field, index}
								<div class="button-config">
									<div class="button-header">
										<h4>Field {index + 1}</h4>
										<button class="btn-remove" onclick={() => removeFormField(index)}
											>Remove</button
										>
									</div>

									<div class="form-group">
										<label for={`field-name-${index}`}>Field Name</label>
										<input
											id={`field-name-${index}`}
											type="text"
											bind:value={field.name}
											class="form-input"
										/>
									</div>

									<div class="form-group">
										<label for={`field-type-${index}`}>Field Type</label>
										<select id={`field-type-${index}`} bind:value={field.type} class="form-input">
											<option value="text">Text</option>
											<option value="email">Email</option>
											<option value="tel">Phone</option>
											<option value="textarea">Textarea</option>
											<option value="select">Select Dropdown</option>
										</select>
									</div>

									<div class="form-group">
										<label for={`field-placeholder-${index}`}>Placeholder</label>
										<input
											id={`field-placeholder-${index}`}
											type="text"
											bind:value={field.placeholder}
											class="form-input"
										/>
									</div>

									<div class="form-group">
										<label class="checkbox-label">
											<input type="checkbox" bind:checked={field.required} />
											<span>Required field</span>
										</label>
									</div>

									{#if field.type === 'select'}
										<div class="form-group">
											<label for={`field-options-${index}`}>Options (one per line)</label>
											<textarea
												id={`field-options-${index}`}
												bind:value={field.options}
												class="form-textarea"
												rows="4"
												placeholder="Option 1&#10;Option 2&#10;Option 3"
											></textarea>
										</div>
									{/if}
								</div>
							{/each}
						{:else}
							<p class="empty-message">No form fields added.</p>
						{/if}
					</div>
				{/if}
			</div>
		</div>

		<!-- Preview Panel -->
		{#if showPreview}
			<div class="preview-panel">
				<h3 class="preview-title">Live Preview</h3>
				<div class="preview-container">
					<div
						class="preview-popup"
						style="
            width: {popup.design?.width};
            max-width: {popup.design?.maxWidth};
            padding: {popup.design?.padding};
            background-color: {popup.design?.backgroundColor};
            color: {popup.design?.textColor};
            border-radius: {popup.design?.borderRadius};
            border: {popup.design?.borderWidth} {popup.design?.borderStyle} {popup.design
							?.borderColor};
            box-shadow: {popup.design?.boxShadow};
          "
					>
						{#if popup.title}
							<h2 class="preview-popup-title">{popup.title}</h2>
						{/if}
						<div class="preview-popup-content">
							{@html sanitizePopupContent(popup.content?.data || '')}
						</div>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.creator-page {
		max-width: 100%;
		height: 100%;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
		gap: 2rem;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 1.5rem;
	}

	.back-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(230, 184, 0, 0.1);
		color: #E6B800;
		border-radius: 8px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.back-btn:hover {
		background: rgba(230, 184, 0, 0.2);
	}

	.page-title {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.25rem;
	}

	.page-subtitle {
		font-size: 1rem;
		color: #64748b;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-primary,
	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		font-weight: 600;
		border-radius: 12px;
		border: none;
		cursor: pointer;
		transition: all 0.3s;
	}

	.btn-primary {
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		color: #0D1117;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: rgba(230, 184, 0, 0.1);
		color: #E6B800;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
	}

	.creator-layout {
		display: grid;
		grid-template-columns: 1fr 400px;
		gap: 2rem;
		height: calc(100vh - 200px);
	}

	.editor-panel {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 16px;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.tabs {
		display: flex;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
		background: rgba(0, 0, 0, 0.2);
	}

	.tab {
		flex: 1;
		padding: 1rem;
		background: none;
		border: none;
		color: #94a3b8;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border-bottom: 2px solid transparent;
	}

	.tab:hover {
		color: #e2e8f0;
	}

	.tab.active {
		color: #E6B800;
		border-bottom-color: #E6B800;
		background: rgba(230, 184, 0, 0.05);
	}

	.tab-content {
		flex: 1;
		overflow-y: auto;
		padding: 2rem;
	}

	.form-section {
		max-width: 700px;
	}

	.section-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 1.5rem;
		margin-top: 2rem;
	}

	.section-title:first-child {
		margin-top: 0;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.btn-add {
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		color: #0D1117;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: block;
		font-weight: 600;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
	}

	.form-input,
	.form-textarea,
	.form-color {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(0, 0, 0, 0.3);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 1rem;
	}

	.form-color {
		height: 50px;
		cursor: pointer;
	}

	.form-input:focus,
	.form-textarea:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-textarea {
		resize: vertical;
		font-family: monospace;
	}

	.form-help {
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #64748b;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.checkbox-label input[type='checkbox'] {
		width: 20px;
		height: 20px;
		cursor: pointer;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.button-config {
		background: rgba(0, 0, 0, 0.3);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 1rem;
	}

	.button-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.button-header h4 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.btn-remove {
		padding: 0.5rem 1rem;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: none;
		border-radius: 6px;
		font-weight: 600;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.btn-remove:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.empty-message {
		color: #64748b;
		font-style: italic;
		text-align: center;
		padding: 2rem;
	}

	.indent {
		margin-left: 2rem;
	}

	/* Preview Panel */
	.preview-panel {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 16px;
		padding: 2rem;
		position: sticky;
		top: 2rem;
		height: fit-content;
		max-height: calc(100vh - 200px);
		overflow-y: auto;
	}

	.preview-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 1.5rem;
	}

	.preview-container {
		background: rgba(0, 0, 0, 0.3);
		border-radius: 12px;
		padding: 2rem;
		min-height: 300px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.preview-popup {
		position: relative;
	}

	.preview-popup-title {
		font-size: 1.5rem;
		font-weight: 700;
		margin-bottom: 1rem;
	}

	.preview-popup-content {
		line-height: 1.6;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.creator-layout {
			grid-template-columns: 1fr;
		}

		.preview-panel {
			position: static;
		}
	}
</style>
