<script lang="ts">
	interface Props {
		formId: number;
		formData: Record<string, unknown>;
		email?: string;
		currentStep?: number;
		onSave?: (hash: string) => void;
		label?: string;
		variant?: 'button' | 'link';
	}

	let {
		formId,
		formData,
		email = '',
		currentStep = 1,
		onSave,
		label = 'Save Progress',
		variant = 'button'
	}: Props = $props();

	let isSaving = $state(false);
	let showEmailModal = $state(false);
	let saveEmail = $state(email);
	let savedHash = $state<string | null>(null);
	let saveError = $state<string | null>(null);
	let copySuccess = $state(false);

	async function saveProgress() {
		if (!saveEmail && !email) {
			showEmailModal = true;
			return;
		}

		isSaving = true;
		saveError = null;

		try {
			const response = await fetch('/api/forms/drafts', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					form_id: formId,
					email: saveEmail || email,
					form_data: formData,
					current_step: currentStep
				})
			});

			if (!response.ok) {
				throw new Error('Failed to save progress');
			}

			const result = await response.json();
			savedHash = result.hash;
			onSave?.(result.hash);
		} catch {
			saveError = 'Failed to save your progress. Please try again.';
		} finally {
			isSaving = false;
		}
	}

	function handleEmailSubmit() {
		if (saveEmail) {
			showEmailModal = false;
			saveProgress();
		}
	}

	function getResumeUrl(): string {
		return `${window.location.origin}/forms/${formId}/resume/${savedHash}`;
	}

	async function copyResumeLink() {
		try {
			await navigator.clipboard.writeText(getResumeUrl());
			copySuccess = true;
			setTimeout(() => {
				copySuccess = false;
			}, 2000);
		} catch {
			// Fallback for older browsers
			const input = document.createElement('input');
			input.value = getResumeUrl();
			document.body.appendChild(input);
			input.select();
			document.execCommand('copy');
			document.body.removeChild(input);
			copySuccess = true;
			setTimeout(() => {
				copySuccess = false;
			}, 2000);
		}
	}

	function closeModal() {
		showEmailModal = false;
		savedHash = null;
		saveError = null;
	}
</script>

<!-- Save Button/Link -->
{#if variant === 'link'}
	<button type="button" class="save-link" onclick={saveProgress} disabled={isSaving}>
		{#if isSaving}
			<span class="spinner-small"></span>
			Saving...
		{:else}
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
				<polyline points="17 21 17 13 7 13 7 21"></polyline>
				<polyline points="7 3 7 8 15 8"></polyline>
			</svg>
			{label}
		{/if}
	</button>
{:else}
	<button type="button" class="save-button" onclick={saveProgress} disabled={isSaving}>
		{#if isSaving}
			<span class="spinner"></span>
			Saving...
		{:else}
			<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
				<polyline points="17 21 17 13 7 13 7 21"></polyline>
				<polyline points="7 3 7 8 15 8"></polyline>
			</svg>
			{label}
		{/if}
	</button>
{/if}

<!-- Email Modal -->
{#if showEmailModal}
	<div class="modal-overlay" onclick={closeModal} onkeydown={(e) => e.key === 'Escape' && closeModal()} role="dialog" tabindex="-1">
		<div class="modal-content" onclick={(e) => e.stopPropagation()} role="document">
			<button type="button" class="modal-close" onclick={closeModal} aria-label="Close">
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<line x1="18" y1="6" x2="6" y2="18"></line>
					<line x1="6" y1="6" x2="18" y2="18"></line>
				</svg>
			</button>

			<div class="modal-icon">
				<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
					<polyline points="22,6 12,13 2,6"></polyline>
				</svg>
			</div>

			<h3 class="modal-title">Save Your Progress</h3>
			<p class="modal-description">
				Enter your email to receive a link to resume your form later.
			</p>

			<form onsubmit={(e) => { e.preventDefault(); handleEmailSubmit(); }}>
				<input
					type="email"
					placeholder="your@email.com"
					bind:value={saveEmail}
					class="email-input"
					required
				/>

				<div class="modal-actions">
					<button type="button" class="btn-cancel" onclick={closeModal}>Cancel</button>
					<button type="submit" class="btn-save" disabled={!saveEmail}>
						Save & Send Link
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Success Modal -->
{#if savedHash}
	<div class="modal-overlay" onclick={closeModal} onkeydown={(e) => e.key === 'Escape' && closeModal()} role="dialog" tabindex="-1">
		<div class="modal-content success" onclick={(e) => e.stopPropagation()} role="document">
			<div class="success-icon">
				<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
					<polyline points="22 4 12 14.01 9 11.01"></polyline>
				</svg>
			</div>

			<h3 class="modal-title">Progress Saved!</h3>
			<p class="modal-description">
				Your form progress has been saved. A resume link has been sent to your email.
			</p>

			<div class="resume-link-box">
				<input type="text" value={getResumeUrl()} readonly class="resume-link-input" />
				<button type="button" class="copy-btn" onclick={copyResumeLink}>
					{#if copySuccess}
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<polyline points="20 6 9 17 4 12"></polyline>
						</svg>
					{:else}
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
							<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
						</svg>
					{/if}
				</button>
			</div>

			<button type="button" class="btn-done" onclick={closeModal}>Done</button>
		</div>
	</div>
{/if}

<!-- Error Display -->
{#if saveError}
	<div class="save-error">
		<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<circle cx="12" cy="12" r="10"></circle>
			<line x1="12" y1="8" x2="12" y2="12"></line>
			<line x1="12" y1="16" x2="12.01" y2="16"></line>
		</svg>
		{saveError}
	</div>
{/if}

<style>
	.save-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background-color: #6b7280;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.save-button:hover:not(:disabled) {
		background-color: #4b5563;
	}

	.save-button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.save-link {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem;
		background: none;
		border: none;
		color: #2563eb;
		font-size: 0.875rem;
		cursor: pointer;
		transition: color 0.2s;
	}

	.save-link:hover:not(:disabled) {
		color: #1d4ed8;
		text-decoration: underline;
	}

	.save-link:disabled {
		color: #9ca3af;
		cursor: not-allowed;
	}

	.spinner,
	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.spinner-small {
		width: 14px;
		height: 14px;
		border-color: rgba(37, 99, 235, 0.3);
		border-top-color: #2563eb;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.modal-overlay {
		position: fixed;
		inset: 0;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal-content {
		position: relative;
		background-color: white;
		border-radius: 0.75rem;
		padding: 2rem;
		max-width: 400px;
		width: 100%;
		text-align: center;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
	}

	.modal-close {
		position: absolute;
		top: 1rem;
		right: 1rem;
		background: none;
		border: none;
		color: #9ca3af;
		cursor: pointer;
		padding: 0.25rem;
	}

	.modal-close:hover {
		color: #374151;
	}

	.modal-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 64px;
		height: 64px;
		background-color: #eff6ff;
		border-radius: 50%;
		color: #2563eb;
		margin-bottom: 1rem;
	}

	.success-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		background-color: #ecfdf5;
		border-radius: 50%;
		color: #059669;
		margin-bottom: 1rem;
	}

	.modal-title {
		font-size: 1.25rem;
		font-weight: 600;
		color: #111827;
		margin: 0 0 0.5rem;
	}

	.modal-description {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0 0 1.5rem;
	}

	.email-input {
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		margin-bottom: 1rem;
	}

	.email-input:focus {
		outline: none;
		border-color: #2563eb;
		box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-cancel,
	.btn-save,
	.btn-done {
		flex: 1;
		padding: 0.75rem 1rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-cancel {
		background-color: white;
		border: 1px solid #d1d5db;
		color: #374151;
	}

	.btn-cancel:hover {
		background-color: #f3f4f6;
	}

	.btn-save {
		background-color: #2563eb;
		border: none;
		color: white;
	}

	.btn-save:hover:not(:disabled) {
		background-color: #1d4ed8;
	}

	.btn-save:disabled {
		background-color: #9ca3af;
		cursor: not-allowed;
	}

	.btn-done {
		background-color: #059669;
		border: none;
		color: white;
	}

	.btn-done:hover {
		background-color: #047857;
	}

	.resume-link-box {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
	}

	.resume-link-input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		background-color: #f9fafb;
		color: #374151;
	}

	.copy-btn {
		padding: 0.5rem;
		background-color: #f3f4f6;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		cursor: pointer;
		color: #374151;
	}

	.copy-btn:hover {
		background-color: #e5e7eb;
	}

	.save-error {
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
</style>
