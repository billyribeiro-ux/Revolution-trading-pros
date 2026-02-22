<script lang="ts">
import { logger } from '$lib/utils/logger';
	import type { Form } from '$lib/api/forms';

	interface Props {
		form: Form;
	}

	let props: Props = $props();

	let embedType: 'iframe' | 'script' | 'link' = $state('iframe');
	let copiedMessage = $state('');

	// Get base URL
	const baseUrl = typeof window !== 'undefined' ? window.location.origin : '';
	let embedUrl = $state('');
	let directUrl = $state('');

	$effect(() => {
		embedUrl = `${baseUrl}/embed/form/${props.form.slug}`;
		directUrl = `${baseUrl}/forms/${props.form.slug}`;
	});

	// Generate embed codes
	let iframeCode = $derived(
		`<iframe src="${embedUrl}" width="100%" height="600" frameborder="0" style="border: none; max-width: 800px;"></iframe>`
	);

	let scriptCode = $derived(`<div id="form-${props.form.slug}"></div>
<script>
(function() {
  const iframe = document.createElement('iframe');
  iframe.src = '${embedUrl}';
  iframe.width = '100%';
  iframe.height = '600';
  iframe.frameBorder = '0';
  iframe.style.border = 'none';
  iframe.style.maxWidth = '800px';
  document.getElementById('form-${props.form.slug}').appendChild(iframe);

  // Listen for form submission events
  window.addEventListener('message', function(event) {
    if (event.data.type === 'form-submitted' && event.data.formSlug === '${props.form.slug}') {
      logger.info('Form submitted:', event.data.submissionId);
      // You can add custom handling here
    }
  });
})();
${'</'}script>`);

	let directLinkCode = $derived(`<a href="${directUrl}" target="_blank">Fill out our form</a>`);

	let shortcode = $derived(`[revolution_form slug="${props.form.slug}"]`);

	function copyToClipboard(text: string, type: string) {
		navigator.clipboard
			.writeText(text)
			.then(() => {
				copiedMessage = `${type} copied to clipboard!`;
				setTimeout(() => {
					copiedMessage = '';
				}, 3000);
			})
			.catch(() => {
				copiedMessage = 'Failed to copy';
			});
	}

	function getCode(): string {
		switch (embedType) {
			case 'iframe':
				return iframeCode;
			case 'script':
				return scriptCode;
			case 'link':
				return directLinkCode;
			default:
				return '';
		}
	}
</script>

<div class="embed-generator">
	<h3>Embed This Form</h3>
	<p class="description">Choose how you want to embed this form on your website:</p>

	<div class="embed-type-selector">
		<button
			class="type-btn"
			class:active={embedType === 'iframe'}
			onclick={() => (embedType = 'iframe')}
		>
			<span class="icon">🖼️</span>
			<span class="label">iFrame Embed</span>
		</button>

		<button
			class="type-btn"
			class:active={embedType === 'script'}
			onclick={() => (embedType = 'script')}
		>
			<span class="icon">📜</span>
			<span class="label">JavaScript Embed</span>
		</button>

		<button
			class="type-btn"
			class:active={embedType === 'link'}
			onclick={() => (embedType = 'link')}
		>
			<span class="icon">🔗</span>
			<span class="label">Direct Link</span>
		</button>
	</div>

	<div class="code-section">
		<div class="code-header">
			<h4>
				{#if embedType === 'iframe'}
					iFrame Embed Code
				{:else if embedType === 'script'}
					JavaScript Embed Code
				{:else}
					Direct Link HTML
				{/if}
			</h4>
			<button class="copy-btn" onclick={() => copyToClipboard(getCode(), 'Code')}>
				📋 Copy Code
			</button>
		</div>

		<pre class="code-block"><code>{getCode()}</code></pre>

		<div class="usage-instructions">
			{#if embedType === 'iframe'}
				<p>
					<strong>How to use:</strong> Paste this code into your HTML where you want the form to appear.
				</p>
				<ul>
					<li>Responsive and works on all devices</li>
					<li>Self-contained (no dependencies)</li>
					<li>Automatically adjusts to your site's width</li>
				</ul>
			{:else if embedType === 'script'}
				<p>
					<strong>How to use:</strong> Paste this code into your HTML. The script will automatically create
					an iframe.
				</p>
				<ul>
					<li>More control over placement</li>
					<li>Listens for form submission events</li>
					<li>Can trigger custom JavaScript on submission</li>
				</ul>
			{:else}
				<p>
					<strong>How to use:</strong> Use this link to direct users to the standalone form page.
				</p>
				<ul>
					<li>Opens in new tab</li>
					<li>Direct URL: {directUrl}</li>
					<li>Great for email campaigns or social media</li>
				</ul>
			{/if}
		</div>
	</div>

	<div class="additional-section">
		<h4>WordPress Shortcode</h4>
		<div class="code-header">
			<pre class="code-block inline"><code>{shortcode}</code></pre>
			<button class="copy-btn" onclick={() => copyToClipboard(shortcode, 'Shortcode')}>
				📋 Copy
			</button>
		</div>
		<p class="note">Use this if you have a Revolution Forms WordPress plugin installed.</p>
	</div>

	<div class="urls-section">
		<h4>Direct URLs</h4>
		<div class="url-row">
			<span class="url-label">Embed URL:</span>
			<code class="url-value">{embedUrl}</code>
			<button class="copy-btn-small" onclick={() => copyToClipboard(embedUrl, 'URL')}>📋</button>
		</div>
		<div class="url-row">
			<span class="url-label">Direct URL:</span>
			<code class="url-value">{directUrl}</code>
			<button class="copy-btn-small" onclick={() => copyToClipboard(directUrl, 'URL')}>📋</button>
		</div>
	</div>

	{#if copiedMessage}
		<div class="copy-notification">
			✓ {copiedMessage}
		</div>
	{/if}
</div>

<style>
	.embed-generator {
		padding: 2rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
	}

	h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.75rem 0;
	}

	.description {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0 0 1.5rem 0;
	}

	.embed-type-selector {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.type-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background: #1e293b;
		border: 2px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-btn:hover {
		border-color: rgba(99, 102, 241, 0.4);
		background: rgba(99, 102, 241, 0.05);
	}

	.type-btn.active {
		border-color: #6366f1;
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
	}

	.type-btn .icon {
		font-size: 2rem;
	}

	.type-btn .label {
		font-size: 0.875rem;
		font-weight: 500;
	}

	.code-section,
	.additional-section {
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.code-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.copy-btn,
	.copy-btn-small {
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.copy-btn:hover,
	.copy-btn-small:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
	}

	.copy-btn-small {
		padding: 0.375rem 0.75rem;
		font-size: 0.7rem;
	}

	.code-block {
		background: #0f172a;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		padding: 1rem;
		overflow-x: auto;
		font-family: 'Monaco', 'Menlo', monospace;
		font-size: 0.75rem;
		line-height: 1.5;
		color: #e2e8f0;
		margin: 0;
	}

	.code-block.inline {
		display: inline-block;
		padding: 0.5rem 0.75rem;
		margin: 0;
	}

	.code-block code {
		color: #a5b4fc;
	}

	.usage-instructions {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.usage-instructions p {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0 0 0.5rem 0;
	}

	.usage-instructions ul {
		margin: 0.5rem 0 0 1.5rem;
		color: #94a3b8;
		font-size: 0.8125rem;
	}

	.usage-instructions li {
		margin-bottom: 0.25rem;
	}

	.note {
		color: #94a3b8;
		font-size: 0.8125rem;
		margin: 0.5rem 0 0 0;
	}

	.urls-section {
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		padding: 1.5rem;
	}

	.url-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.url-row:last-child {
		margin-bottom: 0;
	}

	.url-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #94a3b8;
		min-width: 100px;
	}

	.url-value {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: #0f172a;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		font-family: monospace;
		font-size: 0.75rem;
		color: #a5b4fc;
		overflow-x: auto;
		white-space: nowrap;
	}

	.copy-notification {
		position: fixed;
		bottom: 2rem;
		right: 2rem;
		padding: 1rem 1.5rem;
		background: linear-gradient(135deg, #10b981, #059669);
		color: white;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		box-shadow: 0 10px 25px rgba(16, 185, 129, 0.3);
		animation: slideIn 0.3s ease-out;
		z-index: 1000;
	}

	@keyframes slideIn {
		from {
			transform: translateY(100%);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}

	@media (max-width: 768px) {
		.embed-type-selector {
			grid-template-columns: 1fr;
		}

		.url-row {
			flex-direction: column;
			align-items: stretch;
		}

		.url-label {
			min-width: auto;
		}
	}
</style>
