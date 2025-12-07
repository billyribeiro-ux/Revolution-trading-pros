<script lang="ts">
	interface PdfFile {
		id: number;
		filename: string;
		file_size_formatted: string;
		generated_at: string;
		download_count: number;
		url: string;
	}

	interface Props {
		submissionId: number;
		formId: number;
		pdfs?: PdfFile[];
		allowGenerate?: boolean;
		showPreview?: boolean;
		variant?: 'button' | 'card' | 'inline';
	}

	let {
		submissionId,
		formId,
		pdfs = [],
		allowGenerate = true,
		showPreview = true,
		variant = 'button'
	}: Props = $props();

	let isGenerating = $state(false);
	let isLoading = $state(false);
	let previewUrl = $state<string | null>(null);
	let error = $state<string | null>(null);
	let pdfList = $state<PdfFile[]>([]);

	$effect(() => {
		pdfList = pdfs;
	});

	async function generatePdf() {
		isGenerating = true;
		error = null;

		try {
			const response = await fetch(`/api/forms/${formId}/submissions/${submissionId}/pdf`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' }
			});

			if (!response.ok) {
				throw new Error('Failed to generate PDF');
			}

			const result = await response.json();
			pdfList = [result.pdf, ...pdfList];
		} catch {
			error = 'Failed to generate PDF. Please try again.';
		} finally {
			isGenerating = false;
		}
	}

	async function downloadPdf(pdf: PdfFile) {
		try {
			const response = await fetch(`/api/forms/${formId}/submissions/${submissionId}/pdf/${pdf.id}/download`);

			if (!response.ok) {
				throw new Error('Failed to download PDF');
			}

			const blob = await response.blob();
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = pdf.filename;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);

			// Update download count
			const index = pdfList.findIndex((p) => p.id === pdf.id);
			if (index !== -1) {
				pdfList[index] = { ...pdfList[index], download_count: pdfList[index].download_count + 1 };
			}
		} catch {
			error = 'Failed to download PDF. Please try again.';
		}
	}

	async function previewPdf(pdf: PdfFile) {
		isLoading = true;
		error = null;

		try {
			const response = await fetch(`/api/forms/${formId}/submissions/${submissionId}/pdf/${pdf.id}/preview`);

			if (!response.ok) {
				throw new Error('Failed to load preview');
			}

			const blob = await response.blob();
			previewUrl = URL.createObjectURL(blob);
		} catch {
			error = 'Failed to load preview. Please try again.';
		} finally {
			isLoading = false;
		}
	}

	function closePreview() {
		if (previewUrl) {
			URL.revokeObjectURL(previewUrl);
			previewUrl = null;
		}
	}

	async function loadPdfs() {
		if (pdfList.length > 0) return;

		isLoading = true;
		try {
			const response = await fetch(`/api/forms/${formId}/submissions/${submissionId}/pdfs`);
			if (response.ok) {
				const result = await response.json();
				pdfList = result.pdfs || [];
			}
		} catch {
			// Silently fail - PDFs are optional
		} finally {
			isLoading = false;
		}
	}

	$effect(() => {
		loadPdfs();
	});
</script>

{#if variant === 'button'}
	<div class="pdf-download-button">
		{#if pdfList.length > 0}
			<div class="pdf-actions">
				{#each pdfList as pdf}
					<button type="button" class="download-btn" onclick={() => downloadPdf(pdf)}>
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
							<polyline points="7 10 12 15 17 10"></polyline>
							<line x1="12" y1="15" x2="12" y2="3"></line>
						</svg>
						Download PDF
					</button>
					{#if showPreview}
						<button type="button" class="preview-btn" onclick={() => previewPdf(pdf)} aria-label="Preview PDF">
							<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
								<circle cx="12" cy="12" r="3"></circle>
							</svg>
						</button>
					{/if}
				{/each}
			</div>
		{:else if allowGenerate}
			<button type="button" class="generate-btn" onclick={generatePdf} disabled={isGenerating}>
				{#if isGenerating}
					<span class="spinner"></span>
					Generating...
				{:else}
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
						<polyline points="14 2 14 8 20 8"></polyline>
						<line x1="12" y1="18" x2="12" y2="12"></line>
						<line x1="9" y1="15" x2="15" y2="15"></line>
					</svg>
					Generate PDF
				{/if}
			</button>
		{/if}
	</div>
{:else if variant === 'card'}
	<div class="pdf-download-card">
		<div class="card-header">
			<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
				<polyline points="14 2 14 8 20 8"></polyline>
				<line x1="16" y1="13" x2="8" y2="13"></line>
				<line x1="16" y1="17" x2="8" y2="17"></line>
				<polyline points="10 9 9 9 8 9"></polyline>
			</svg>
			<h3>PDF Documents</h3>
		</div>

		{#if isLoading}
			<div class="loading-state">
				<span class="spinner"></span>
				<span>Loading documents...</span>
			</div>
		{:else if pdfList.length > 0}
			<div class="pdf-list">
				{#each pdfList as pdf}
					<div class="pdf-item">
						<div class="pdf-icon">
							<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
								<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
								<polyline points="14 2 14 8 20 8"></polyline>
							</svg>
						</div>
						<div class="pdf-info">
							<span class="pdf-name">{pdf.filename}</span>
							<span class="pdf-meta">{pdf.file_size_formatted} &bull; {pdf.download_count} downloads</span>
						</div>
						<div class="pdf-actions">
							{#if showPreview}
								<button type="button" class="action-btn preview" onclick={() => previewPdf(pdf)} title="Preview" aria-label="Preview PDF">
									<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
										<circle cx="12" cy="12" r="3"></circle>
									</svg>
								</button>
							{/if}
							<button type="button" class="action-btn download" onclick={() => downloadPdf(pdf)} title="Download" aria-label="Download PDF">
								<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
									<polyline points="7 10 12 15 17 10"></polyline>
									<line x1="12" y1="15" x2="12" y2="3"></line>
								</svg>
							</button>
						</div>
					</div>
				{/each}
			</div>
		{:else}
			<div class="empty-state">
				<p>No PDF documents generated yet.</p>
				{#if allowGenerate}
					<button type="button" class="generate-btn" onclick={generatePdf} disabled={isGenerating}>
						{#if isGenerating}
							<span class="spinner"></span>
							Generating...
						{:else}
							Generate PDF
						{/if}
					</button>
				{/if}
			</div>
		{/if}

		{#if allowGenerate && pdfList.length > 0}
			<div class="card-footer">
				<button type="button" class="generate-btn secondary" onclick={generatePdf} disabled={isGenerating}>
					{#if isGenerating}
						<span class="spinner"></span>
						Generating...
					{:else}
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="12" y1="5" x2="12" y2="19"></line>
							<line x1="5" y1="12" x2="19" y2="12"></line>
						</svg>
						Generate New PDF
					{/if}
				</button>
			</div>
		{/if}
	</div>
{:else}
	<!-- Inline variant -->
	<div class="pdf-download-inline">
		{#if pdfList.length > 0}
			{#each pdfList as pdf}
				<a href={pdf.url} class="pdf-link" download={pdf.filename} onclick={() => downloadPdf(pdf)}>
					<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
						<polyline points="14 2 14 8 20 8"></polyline>
					</svg>
					{pdf.filename}
				</a>
			{/each}
		{:else if allowGenerate}
			<button type="button" class="pdf-link" onclick={generatePdf} disabled={isGenerating}>
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
					<polyline points="14 2 14 8 20 8"></polyline>
				</svg>
				{isGenerating ? 'Generating...' : 'Generate PDF'}
			</button>
		{/if}
	</div>
{/if}

{#if error}
	<div class="error-message">
		<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<circle cx="12" cy="12" r="10"></circle>
			<line x1="12" y1="8" x2="12" y2="12"></line>
			<line x1="12" y1="16" x2="12.01" y2="16"></line>
		</svg>
		{error}
	</div>
{/if}

{#if previewUrl}
	<div class="preview-overlay" onclick={closePreview} onkeydown={(e) => { if (e.key === 'Escape') closePreview(); if (e.key === 'Enter' || e.key === ' ') closePreview(); }} role="button" tabindex="0" aria-label="Close preview">
		<div class="preview-modal" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="presentation">
			<div class="preview-header">
				<h3>PDF Preview</h3>
				<button type="button" class="close-btn" onclick={closePreview} aria-label="Close preview">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<line x1="18" y1="6" x2="6" y2="18"></line>
						<line x1="6" y1="6" x2="18" y2="18"></line>
					</svg>
				</button>
			</div>
			<div class="preview-content">
				<iframe src={previewUrl} title="PDF Preview" width="100%" height="100%"></iframe>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Button variant */
	.pdf-download-button {
		display: inline-flex;
		gap: 0.5rem;
	}

	.pdf-actions {
		display: flex;
		gap: 0.25rem;
	}

	.download-btn,
	.generate-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background-color: #dc2626;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.download-btn:hover,
	.generate-btn:hover:not(:disabled) {
		background-color: #b91c1c;
	}

	.generate-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.generate-btn.secondary {
		background-color: #6b7280;
	}

	.generate-btn.secondary:hover:not(:disabled) {
		background-color: #4b5563;
	}

	.preview-btn {
		padding: 0.5rem;
		background-color: #f3f4f6;
		color: #374151;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.preview-btn:hover {
		background-color: #e5e7eb;
	}

	/* Card variant */
	.pdf-download-card {
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.card-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background-color: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
	}

	.card-header svg {
		color: #dc2626;
	}

	.card-header h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: #111827;
	}

	.pdf-list {
		padding: 0.5rem;
	}

	.pdf-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		border-radius: 0.5rem;
		transition: background-color 0.2s;
	}

	.pdf-item:hover {
		background-color: #f9fafb;
	}

	.pdf-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background-color: #fef2f2;
		border-radius: 0.5rem;
		color: #dc2626;
	}

	.pdf-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.pdf-name {
		font-weight: 500;
		color: #111827;
		font-size: 0.875rem;
	}

	.pdf-meta {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.action-btn {
		padding: 0.5rem;
		background-color: transparent;
		border: 1px solid #e5e7eb;
		border-radius: 0.375rem;
		cursor: pointer;
		color: #6b7280;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background-color: #f3f4f6;
		color: #374151;
	}

	.action-btn.download:hover {
		background-color: #dc2626;
		border-color: #dc2626;
		color: white;
	}

	.card-footer {
		padding: 0.75rem 1rem;
		background-color: #f9fafb;
		border-top: 1px solid #e5e7eb;
	}

	/* Inline variant */
	.pdf-download-inline {
		display: inline-flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.pdf-link {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.5rem;
		background-color: #fef2f2;
		color: #dc2626;
		border: none;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.pdf-link:hover {
		background-color: #fee2e2;
	}

	/* States */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		gap: 1rem;
		color: #6b7280;
		font-size: 0.875rem;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background-color: #fef2f2;
		color: #dc2626;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		margin-top: 0.5rem;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.loading-state .spinner {
		border-color: rgba(220, 38, 38, 0.3);
		border-top-color: #dc2626;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Preview modal */
	.preview-overlay {
		position: fixed;
		inset: 0;
		background-color: rgba(0, 0, 0, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 2rem;
	}

	.preview-modal {
		background-color: white;
		border-radius: 0.75rem;
		width: 100%;
		max-width: 900px;
		height: 90vh;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.preview-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.preview-header h3 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
	}

	.close-btn {
		padding: 0.5rem;
		background: none;
		border: none;
		color: #6b7280;
		cursor: pointer;
		border-radius: 0.375rem;
	}

	.close-btn:hover {
		background-color: #f3f4f6;
		color: #111827;
	}

	.preview-content {
		flex: 1;
		overflow: hidden;
	}

	.preview-content iframe {
		border: none;
	}
</style>
