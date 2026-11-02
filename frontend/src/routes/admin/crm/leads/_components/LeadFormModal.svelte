<script lang="ts">
	import { IconUserPlus, IconEdit, IconCheck, IconX } from '#lib/icons/index.js';

	export interface LeadFormData {
		first_name: string;
		last_name: string;
		email: string;
		phone: string;
		company_name: string;
		job_title: string;
		source: string;
		estimated_value: number;
		notes: string;
	}

	interface SourceOption {
		value: string;
		label: string;
	}

	interface Props {
		mode: 'add' | 'edit';
		formData: LeadFormData;
		formError: string;
		formLoading: boolean;
		sourceOptions: SourceOption[];
		onClose: () => void;
		onSubmit: () => void;
	}

	let {
		mode,
		formData = $bindable(),
		formError,
		formLoading,
		sourceOptions,
		onClose,
		onSubmit
	}: Props = $props();

	let idPrefix = $derived(mode === 'edit' ? 'edit_' : '');
	let title = $derived(mode === 'edit' ? 'Edit Lead' : 'Add New Lead');
	let submitLabel = $derived(mode === 'edit' ? 'Save Changes' : 'Create Lead');
	let loadingLabel = $derived(mode === 'edit' ? 'Saving...' : 'Creating...');
</script>

<div
	class="modal-overlay"
	onclick={(e) => {
		if (e.target === e.currentTarget) onClose();
	}}
	onkeydown={(e) => e.key === 'Escape' && onClose()}
	role="dialog"
	aria-modal="true"
	tabindex="-1"
>
	<div class="modal" role="document">
		<div class="modal-header">
			<h3>
				{#if mode === 'edit'}
					<IconEdit size={20} />
				{:else}
					<IconUserPlus size={20} />
				{/if}
				{title}
			</h3>
			<button class="modal-close" onclick={onClose}>
				<IconX size={20} />
			</button>
		</div>
		<div class="modal-body">
			{#if formError}
				<div class="form-error">{formError}</div>
			{/if}
			<div class="form-grid">
				<div class="form-group">
					<label for="{idPrefix}first_name">First Name *</label>
					<input autocomplete="given-name"
						id="{idPrefix}first_name"
						name="{idPrefix}first_name"
						type="text"
						bind:value={formData.first_name}
						required
					/>
				</div>
				<div class="form-group">
					<label for="{idPrefix}last_name">Last Name *</label>
					<input autocomplete="family-name"
						id="{idPrefix}last_name"
						name="{idPrefix}last_name"
						type="text"
						bind:value={formData.last_name}
						required
					/>
				</div>
				<div class="form-group full-width">
					<label for="{idPrefix}email">Email *</label>
					<input
						id="{idPrefix}email"
						name="{idPrefix}email"
						autocomplete="email"
						type="email"
						bind:value={formData.email}
						required
					/>
				</div>
				<div class="form-group">
					<label for="{idPrefix}phone">Phone</label>
					<input
						id="{idPrefix}phone"
						name="{idPrefix}phone"
						autocomplete="tel"
						type="tel"
						bind:value={formData.phone}
					/>
				</div>
				<div class="form-group">
					<label for="{idPrefix}company">Company</label>
					<input autocomplete="off"
						id="{idPrefix}company"
						name="{idPrefix}company"
						type="text"
						bind:value={formData.company_name}
					/>
				</div>
				<div class="form-group">
					<label for="{idPrefix}job_title">Job Title</label>
					<input autocomplete="off"
						id="{idPrefix}job_title"
						name="{idPrefix}job_title"
						type="text"
						bind:value={formData.job_title}
					/>
				</div>
				<div class="form-group">
					<label for="{idPrefix}source">Lead Source</label>
					<select id="{idPrefix}source" bind:value={formData.source}>
						{#each sourceOptions.filter((o) => o.value !== 'all') as option (option.value)}
							<option value={option.value}>{option.label}</option>
						{/each}
					</select>
				</div>
				<div class="form-group full-width">
					<label for="{idPrefix}estimated_value">Estimated Value ($)</label>
					<input autocomplete="off"
						id="{idPrefix}estimated_value"
						name="{idPrefix}estimated_value"
						type="number"
						min="0"
						bind:value={formData.estimated_value}
					/>
				</div>
				{#if mode === 'add'}
					<div class="form-group full-width">
						<label for="notes">Notes</label>
						<textarea id="notes" rows="3" bind:value={formData.notes}></textarea>
					</div>
				{/if}
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onClose}> Cancel </button>
			<button
				class="btn-primary"
				onclick={onSubmit}
				disabled={formLoading || !formData.email || !formData.first_name}
			>
				{#if formLoading}
					{loadingLabel}
				{:else}
					<IconCheck size={18} />
					{submitLabel}
				{/if}
			</button>
		</div>
	</div>
</div>

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
		max-width: 560px;
		max-height: 90vh;
		overflow-y: auto;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 16px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.modal-header h3 {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 0;
		font-size: 1.1rem;
		font-weight: 600;
		color: white;
	}

	.modal-header h3 :global(svg) {
		color: var(--primary-500);
	}

	.modal-close {
		display: flex;
		padding: 8px;
		background: transparent;
		border: none;
		color: var(--text-tertiary, #6e7681);
		cursor: pointer;
		transition: color 0.2s;
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
		border-top: 1px solid var(--border-default, #30363d);
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 0.8rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 10px 14px;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: white;
		font-size: 0.9rem;
		font-family: inherit;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-error {
		margin-bottom: 16px;
		padding: 12px;
		background: rgba(248, 81, 73, 0.1);
		border: 1px solid rgba(248, 81, 73, 0.3);
		border-radius: 8px;
		color: var(--error-emphasis, #f85149);
		font-size: 0.875rem;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 0.75rem 1rem;
		background: var(--bg-hover, #252b33);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
		border-color: var(--border-default, #30363d);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 0.75rem 1.25rem;
		background: var(--primary-500);
		color: var(--bg-base, #0d1117);
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	@media (max-width: 767.98px) {
		.form-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
