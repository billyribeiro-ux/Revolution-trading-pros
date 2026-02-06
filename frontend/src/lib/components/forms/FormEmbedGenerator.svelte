<script lang="ts">
	/**
	 * Form Embed Generator - Generate embed codes for forms
	 *
	 * Features:
	 * - Multiple embed types (iframe, JS, popup)
	 * - Customization options
	 * - Live preview
	 * - Copy to clipboard
	 * - QR code generation
	 * - Social sharing links
	 *
	 * @version 1.0.0
	 */

	import { getAuthToken } from '$lib/stores/auth.svelte';

	interface Props {
		formId: number;
		formSlug: string;
		formTitle: string;
	}

	let props: Props = $props();

	// Embed types
	const embedTypes = [
		{ value: 'iframe', label: 'Standard Embed (iframe)', icon: '📋' },
		{ value: 'javascript', label: 'JavaScript Widget', icon: '⚡' },
		{ value: 'popup', label: 'Popup/Modal', icon: '💬' },
		{ value: 'inline', label: 'Inline Injection', icon: '📥' }
	];

	// State
	let selectedType = $state('iframe');
	let embedCode = $state('');
	let shareUrl = $state('');
	let qrCodeUrl = $state('');
	let copied = $state(false);
	let loading = $state(false);

	// Options
	let options = $state({
		width: '100%',
		height: '600',
		responsive: true,
		border: false,
		shadow: true,
		rounded: true,
		theme: 'light',
		buttonText: 'Open Form',
		buttonClass: ''
	});

	// UTM parameters
	let utmParams = $state({
		source: '',
		medium: '',
		campaign: ''
	});

	// Generate embed code
	async function generateEmbed() {
		loading = true;
		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${props.formId}/embed`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify({
					type: selectedType,
					options: {
						...options,
						button_text: options.buttonText,
						button_class: options.buttonClass
					}
				})
			});

			if (response.ok) {
				const data = await response.json();
				embedCode = data.code;
			}
		} catch (error) {
			console.error('Failed to generate embed:', error);
		}
		loading = false;
	}

	// Generate shareable link
	async function generateShareLink() {
		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${props.formId}/share`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify({
					utm_source: utmParams.source || undefined,
					utm_medium: utmParams.medium || undefined,
					utm_campaign: utmParams.campaign || undefined
				})
			});

			if (response.ok) {
				const data = await response.json();
				shareUrl = data.url;
				qrCodeUrl = data.qr_code;
			}
		} catch (error) {
			console.error('Failed to generate share link:', error);
		}
	}

	// Copy to clipboard
	async function copyToClipboard(text: string) {
		try {
			await navigator.clipboard.writeText(text);
			copied = true;
			setTimeout(() => (copied = false), 2000);
		} catch (error) {
			console.error('Failed to copy:', error);
		}
	}

	// Initialize
	$effect(() => {
		generateEmbed();
		generateShareLink();
	});

	// Regenerate on type change
	$effect(() => {
		if (selectedType) {
			generateEmbed();
		}
	});
</script>

<div class="embed-generator">
	<div class="generator-header">
		<h3>Embed & Share</h3>
		<p class="subtitle">Share "{props.formTitle}" anywhere</p>
	</div>

	<div class="generator-content">
		<!-- Embed Type Selection -->
		<div class="section">
			<h3 class="section-label">Embed Type</h3>
			<div class="embed-types">
				{#each embedTypes as type}
					<button
						class="type-option"
						class:selected={selectedType === type.value}
						onclick={() => (selectedType = type.value)}
					>
						<span class="type-icon">{type.icon}</span>
						<span class="type-label">{type.label}</span>
					</button>
				{/each}
			</div>
		</div>

		<!-- Options -->
		<div class="section">
			<h3 class="section-label">Customization</h3>
			<div class="options-grid">
				{#if selectedType === 'iframe'}
					<div class="option">
						<label for="embed-width">Width</label>
						<input id="embed-width" type="text" bind:value={options.width} placeholder="100%" />
					</div>
					<div class="option">
						<label for="embed-height">Height</label>
						<input id="embed-height" type="text" bind:value={options.height} placeholder="600" />
					</div>
				{/if}

				{#if selectedType === 'popup'}
					<div class="option full-width">
						<label for="button-text">Button Text</label>
						<input
							id="button-text"
							type="text"
							bind:value={options.buttonText}
							placeholder="Open Form"
						/>
					</div>
				{/if}

				<div class="option checkbox-option">
					<label>
						<input type="checkbox" bind:checked={options.responsive} />
						Responsive
					</label>
				</div>

				<div class="option checkbox-option">
					<label>
						<input type="checkbox" bind:checked={options.shadow} />
						Shadow
					</label>
				</div>

				<div class="option checkbox-option">
					<label>
						<input type="checkbox" bind:checked={options.rounded} />
						Rounded Corners
					</label>
				</div>

				<div class="option">
					<label for="embed-theme">Theme</label>
					<select id="embed-theme" bind:value={options.theme}>
						<option value="light">Light</option>
						<option value="dark">Dark</option>
						<option value="auto">Auto</option>
					</select>
				</div>
			</div>

			<button class="btn-regenerate" onclick={generateEmbed} disabled={loading}>
				{loading ? 'Generating...' : 'Regenerate Code'}
			</button>
		</div>

		<!-- Embed Code -->
		<div class="section">
			<h3 class="section-label">Embed Code</h3>
			<div class="code-container">
				<pre class="code-block">{embedCode || 'Generating...'}</pre>
				<button class="btn-copy" onclick={() => copyToClipboard(embedCode)} disabled={!embedCode}>
					{copied ? 'Copied!' : 'Copy Code'}
				</button>
			</div>
		</div>

		<!-- Share Link -->
		<div class="section">
			<h3 class="section-label">Direct Link</h3>

			<!-- UTM Parameters -->
			<details class="utm-section">
				<summary>UTM Parameters (optional)</summary>
				<div class="utm-grid">
					<div class="option">
						<label for="utm-source">Source</label>
						<input
							id="utm-source"
							type="text"
							bind:value={utmParams.source}
							placeholder="e.g., newsletter"
						/>
					</div>
					<div class="option">
						<label for="utm-medium">Medium</label>
						<input
							id="utm-medium"
							type="text"
							bind:value={utmParams.medium}
							placeholder="e.g., email"
						/>
					</div>
					<div class="option">
						<label for="utm-campaign">Campaign</label>
						<input
							id="utm-campaign"
							type="text"
							bind:value={utmParams.campaign}
							placeholder="e.g., spring2025"
						/>
					</div>
				</div>
				<button class="btn-regenerate small" onclick={generateShareLink}> Update Link </button>
			</details>

			<div class="share-url-container">
				<input type="text" readonly value={shareUrl} class="share-url-input" />
				<button class="btn-copy" onclick={() => copyToClipboard(shareUrl)} disabled={!shareUrl}>
					Copy
				</button>
			</div>
		</div>

		<!-- QR Code -->
		{#if qrCodeUrl}
			<div class="section">
				<h3 class="section-label">QR Code</h3>
				<div class="qr-container">
					<img src={qrCodeUrl} alt="QR Code for {props.formTitle}" class="qr-code" />
					<a href={qrCodeUrl} download="form-qr-{props.formSlug}.png" class="btn-download">
						Download QR Code
					</a>
				</div>
			</div>
		{/if}

		<!-- Social Sharing -->
		<div class="section">
			<h3 class="section-label">Share on Social</h3>
			<div class="social-buttons">
				<a
					href="https://twitter.com/intent/tweet?url={encodeURIComponent(
						shareUrl
					)}&text={encodeURIComponent(props.formTitle)}"
					target="_blank"
					rel="noopener noreferrer"
					class="social-btn twitter"
				>
					Twitter
				</a>
				<a
					href="https://www.facebook.com/sharer/sharer.php?u={encodeURIComponent(shareUrl)}"
					target="_blank"
					rel="noopener noreferrer"
					class="social-btn facebook"
				>
					Facebook
				</a>
				<a
					href="https://www.linkedin.com/sharing/share-offsite/?url={encodeURIComponent(shareUrl)}"
					target="_blank"
					rel="noopener noreferrer"
					class="social-btn linkedin"
				>
					LinkedIn
				</a>
				<a
					href="mailto:?subject={encodeURIComponent(props.formTitle)}&body={encodeURIComponent(
						shareUrl
					)}"
					class="social-btn email"
				>
					Email
				</a>
			</div>
		</div>
	</div>
</div>

<style>
	.embed-generator {
		background: white;
		border-radius: 0.75rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	.generator-header {
		padding: 1.5rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.generator-header h3 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: #111827;
	}

	.subtitle {
		margin: 0.25rem 0 0 0;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.generator-content {
		padding: 1.5rem;
	}

	.section {
		margin-bottom: 1.5rem;
	}

	.section:last-child {
		margin-bottom: 0;
	}

	.section-label {
		display: block;
		font-weight: 500;
		font-size: 0.875rem;
		color: #374151;
		margin-bottom: 0.75rem;
	}

	.embed-types {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.75rem;
	}

	.type-option {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: #f9fafb;
		border: 2px solid #e5e7eb;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-option:hover {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.type-option.selected {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.type-icon {
		font-size: 1.25rem;
	}

	.type-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.options-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.option {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.option.full-width {
		grid-column: 1 / -1;
	}

	.option.checkbox-option {
		flex-direction: row;
		align-items: center;
	}

	.option.checkbox-option label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.option label {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.option input[type='text'],
	.option select {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
	}

	.option input[type='text']:focus,
	.option select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.btn-regenerate {
		margin-top: 1rem;
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-regenerate:hover {
		background: #2563eb;
	}

	.btn-regenerate:disabled {
		background: #9ca3af;
		cursor: not-allowed;
	}

	.btn-regenerate.small {
		padding: 0.375rem 0.75rem;
		font-size: 0.75rem;
	}

	.code-container {
		position: relative;
	}

	.code-block {
		background: #1f2937;
		color: #e5e7eb;
		padding: 1rem;
		border-radius: 0.5rem;
		font-size: 0.75rem;
		font-family: 'SF Mono', Monaco, 'Courier New', monospace;
		overflow-x: auto;
		white-space: pre-wrap;
		word-break: break-all;
		max-height: 200px;
		margin: 0;
	}

	.btn-copy {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: #374151;
		color: white;
		border: none;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-copy:hover {
		background: #4b5563;
	}

	.btn-copy:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.utm-section {
		margin-bottom: 1rem;
		padding: 0.75rem;
		background: #f9fafb;
		border-radius: 0.5rem;
	}

	.utm-section summary {
		cursor: pointer;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.utm-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.75rem;
		margin-top: 0.75rem;
	}

	.share-url-container {
		display: flex;
		gap: 0.5rem;
	}

	.share-url-input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		background: #f9fafb;
	}

	.qr-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		padding: 1.5rem;
		background: #f9fafb;
		border-radius: 0.5rem;
	}

	.qr-code {
		width: 150px;
		height: 150px;
		border-radius: 0.5rem;
	}

	.btn-download {
		padding: 0.5rem 1rem;
		background: #10b981;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		text-decoration: none;
		transition: background 0.2s;
	}

	.btn-download:hover {
		background: #059669;
	}

	.social-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.social-btn {
		padding: 0.5rem 1rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		text-decoration: none;
		color: white;
		transition: opacity 0.2s;
	}

	.social-btn:hover {
		opacity: 0.9;
	}

	.social-btn.twitter {
		background: #1da1f2;
	}

	.social-btn.facebook {
		background: #4267b2;
	}

	.social-btn.linkedin {
		background: #0077b5;
	}

	.social-btn.email {
		background: #6b7280;
	}

	@media (max-width: 640px) {
		.embed-types {
			grid-template-columns: 1fr;
		}

		.options-grid {
			grid-template-columns: 1fr;
		}

		.utm-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
