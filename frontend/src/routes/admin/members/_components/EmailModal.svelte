<script lang="ts">
	import { IconX, IconSend } from '$lib/icons';
	import type { EmailTemplate } from '$lib/api/members';

	interface Props {
		recipientCount: number;
		templates: EmailTemplate[];
		subject: string;
		body: string;
		onClose: () => void;
		onApplyTemplate: (template: { subject: string; body?: string }) => void;
		onSend: () => void;
	}

	let {
		recipientCount,
		templates,
		subject = $bindable(),
		body = $bindable(),
		onClose,
		onApplyTemplate,
		onSend
	}: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={(e: MouseEvent) => {
		if (e.target === e.currentTarget) onClose();
	}}
	onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
	role="dialog"
	tabindex="-1"
	aria-modal="true"
>
	<div class="modal-content" role="document">
		<div class="modal-header">
			<h2>Send Email to {recipientCount} Member{recipientCount > 1 ? 's' : ''}</h2>
			<button class="close-btn" onclick={onClose}>
				<IconX size={20} />
			</button>
		</div>

		<div class="modal-body">
			<div class="template-selector">
				<span class="template-label">Quick Templates</span>
				<div class="template-buttons">
					{#each templates as template (template.name)}
						<button class="template-btn" onclick={() => onApplyTemplate(template)}>
							{template.name}
						</button>
					{/each}
				</div>
			</div>

			<div class="form-group">
				<label for="email-subject">Subject</label>
				<input
					id="email-subject"
					name="email-subject"
					type="text"
					bind:value={subject}
					placeholder="Email subject..."
				/>
			</div>

			<div class="form-group">
				<label for="email-body">Body</label>
				<textarea
					id="email-body"
					bind:value={body}
					rows="10"
					placeholder="Email body... Use {'{{ name }}'} for personalization"
				></textarea>
			</div>
		</div>

		<div class="modal-footer">
			<button class="btn-secondary" onclick={onClose}>Cancel</button>
			<button class="btn-primary" onclick={onSend} disabled={!subject || !body}>
				<IconSend size={18} />
				Send Email
			</button>
		</div>
	</div>
</div>

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: linear-gradient(145deg, var(--bg-elevated) 0%, var(--bg-base) 100%);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow:
			0 25px 60px -15px rgba(0, 0, 0, 0.7),
			0 0 40px -10px rgba(230, 184, 0, 0.1);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid var(--border-muted);
		background: rgba(13, 17, 23, 0.5);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 10px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(100, 116, 139, 0.3);
		color: var(--text-primary);
	}

	.modal-body {
		padding: 1.5rem;
	}

	.template-selector {
		margin-bottom: 1.5rem;
	}

	.template-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.template-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.template-btn {
		padding: 0.625rem 1rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.25);
		border-radius: 10px;
		color: var(--primary-500);
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.template-btn:hover {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.4);
		transform: translateY(-1px);
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary);
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(13, 17, 23, 0.6);
		border: 1px solid var(--border-default);
		border-radius: 10px;
		color: var(--text-primary);
		font-size: 0.9375rem;
		font-family: inherit;
		resize: vertical;
		transition: all 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--primary-500);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.form-group input::placeholder,
	.form-group textarea::placeholder {
		color: var(--text-tertiary);
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid var(--border-muted);
		background: rgba(13, 17, 23, 0.3);
	}

	.btn-secondary,
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: var(--text-primary);
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		color: var(--text-primary);
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
		box-shadow: 0 4px 14px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:hover:not(:disabled) {
		background: linear-gradient(135deg, var(--primary-400), var(--primary-500));
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>
