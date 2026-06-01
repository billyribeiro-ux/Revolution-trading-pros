<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconMailFast from '@tabler/icons-svelte-runes/icons/mail-fast';
	import IconList from '@tabler/icons-svelte-runes/icons/list';
	import IconTag from '@tabler/icons-svelte-runes/icons/tag';
	import IconNotes from '@tabler/icons-svelte-runes/icons/notes';
	import IconSend from '@tabler/icons-svelte-runes/icons/send';
	import type { Contact, EmailTemplateOption } from './types';

	interface Props {
		open: boolean;
		contact: Contact | null;
		templates: EmailTemplateOption[];
		loadingTemplates: boolean;
		sending: boolean;
		subject: string;
		body: string;
		selectedTemplateId: string | number | null;
		onClose: () => void;
		onSend: () => void;
		onTemplateChange: (id: string | number | null) => void;
	}

	let {
		open,
		contact,
		templates,
		loadingTemplates,
		sending,
		subject = $bindable(),
		body = $bindable(),
		selectedTemplateId,
		onClose,
		onSend,
		onTemplateChange
	}: Props = $props();
</script>

{#if open}
	<div
		class="modal-overlay"
		onclick={(e: MouseEvent) => e.target === e.currentTarget && onClose()}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		aria-labelledby="send-email-title"
		tabindex="-1"
	>
		<div class="modal modal-large" role="document">
			<div class="modal-header">
				<div class="modal-title-section">
					<h3 id="send-email-title">
						<IconMailFast size={22} />
						Send Email
					</h3>
					{#if contact}
						<span class="email-recipient">
							To: <strong>{contact.full_name}</strong> ({contact.email})
						</span>
					{/if}
				</div>
				<button class="modal-close" onclick={onClose} aria-label="Close modal">
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body email-form">
				<!-- Template Selector -->
				<div class="form-group">
					<label for="email-template" class="form-label">
						<IconList size={16} />
						Email Template
						<span class="label-optional">(optional)</span>
					</label>
					<select
						id="email-template"
						class="form-select"
						value={selectedTemplateId ?? ''}
						onchange={(e) => onTemplateChange((e.target as HTMLSelectElement).value || null)}
						disabled={loadingTemplates}
					>
						<option value="">
							{loadingTemplates ? 'Loading templates...' : '-- Select a template --'}
						</option>
						{#each templates as template (template.id)}
							<option value={template.id}>{template.name}</option>
						{/each}
					</select>
					{#if templates.length === 0 && !loadingTemplates}
						<p class="form-hint">No templates available. You can compose a custom email below.</p>
					{/if}
				</div>

				<!-- Subject Line -->
				<div class="form-group">
					<label for="email-subject" class="form-label required">
						<IconTag size={16} />
						Subject Line
					</label>
					<input
						id="email-subject"
						name="email-subject"
						type="text"
						class="form-input"
						placeholder="Enter email subject..."
						bind:value={subject}
						disabled={sending}
					/>
				</div>

				<!-- Email Body -->
				<div class="form-group">
					<label for="email-body" class="form-label required">
						<IconNotes size={16} />
						Email Body
					</label>
					<textarea
						id="email-body"
						class="form-textarea"
						placeholder="Compose your email message here..."
						bind:value={body}
						rows="10"
						disabled={sending}
					></textarea>
					<p class="form-hint">You can use basic text formatting. HTML tags will be preserved.</p>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose} disabled={sending}>Cancel</button>
				<button
					class="btn-primary send-btn"
					onclick={onSend}
					disabled={sending || !subject.trim() || !body.trim()}
				>
					{#if sending}
						<div class="btn-spinner"></div>
						Sending...
					{:else}
						<IconSend size={18} />
						Send Email
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		width: 100%;
		max-width: 480px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		overflow: hidden;
	}

	.modal-large {
		max-width: 680px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid #334155;
	}

	.modal-title-section {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.modal-title-section h3 {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 0;
		font-size: 1.15rem;
		font-weight: 600;
		color: white;
	}

	.modal-title-section h3 :global(svg) {
		color: #f97316;
	}

	.email-recipient {
		font-size: 0.8rem;
		color: #64748b;
	}

	.email-recipient strong {
		color: #94a3b8;
	}

	.modal-close {
		display: flex;
		padding: 8px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
	}

	.modal-close:hover {
		color: white;
	}

	.modal-body {
		padding: 20px;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 20px;
		border-top: 1px solid #334155;
	}

	.email-form {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-label {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 0.85rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.form-label :global(svg) {
		color: #64748b;
	}

	.form-label.required::after {
		content: '*';
		color: #f87171;
		margin-left: 2px;
	}

	.label-optional {
		font-weight: 400;
		color: #64748b;
		font-size: 0.75rem;
	}

	.form-select {
		width: 100%;
		padding: 12px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		font-family: inherit;
		cursor: pointer;
		transition: all 0.2s;
		appearance: none;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%2364748b' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: right 12px center;
		padding-right: 40px;
	}

	.form-select:hover:not(:disabled) {
		border-color: #475569;
	}

	.form-select:focus {
		outline: none;
		border-color: #f97316;
		box-shadow: 0 0 0 3px rgba(249, 115, 22, 0.1);
	}

	.form-select:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.form-input {
		width: 100%;
		padding: 12px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		font-family: inherit;
		transition: all 0.2s;
	}

	.form-input:hover:not(:disabled) {
		border-color: #475569;
	}

	.form-input:focus {
		outline: none;
		border-color: #f97316;
		box-shadow: 0 0 0 3px rgba(249, 115, 22, 0.1);
	}

	.form-input:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.form-input::placeholder {
		color: #475569;
	}

	.form-textarea {
		width: 100%;
		padding: 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		font-family: inherit;
		resize: vertical;
		min-height: 180px;
		line-height: 1.6;
		transition: all 0.2s;
	}

	.form-textarea:hover:not(:disabled) {
		border-color: #475569;
	}

	.form-textarea:focus {
		outline: none;
		border-color: #f97316;
		box-shadow: 0 0 0 3px rgba(249, 115, 22, 0.1);
	}

	.form-textarea:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.form-textarea::placeholder {
		color: #475569;
	}

	.form-hint {
		font-size: 0.75rem;
		color: #64748b;
		margin: 0;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #0f172a;
		border: 1px solid #334155;
		color: #e2e8f0;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: #1e293b;
		border-color: #475569;
	}

	.send-btn {
		min-width: 140px;
		justify-content: center;
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 720px) {
		.modal-large {
			max-width: 100%;
			margin: 16px;
		}
	}

	@media (max-width: 479.98px) {
		.modal-title-section h3 {
			font-size: 1rem;
		}

		.form-textarea {
			min-height: 140px;
		}
	}
</style>
