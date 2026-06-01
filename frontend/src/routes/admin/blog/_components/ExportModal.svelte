<script lang="ts">
	import { IconX, IconDownload } from '$lib/icons';

	type ExportFormat = 'csv' | 'json' | 'wordpress';

	type Props = {
		open: boolean;
		format: ExportFormat;
		selectedCount: number;
		onClose: () => void;
		onExport: () => void;
	};

	let { open, format = $bindable(), selectedCount, onClose, onExport }: Props = $props();
</script>

{#if open}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={onClose}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h2>Export Posts</h2>
				<button class="btn-icon" onclick={onClose}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-content">
				<div class="export-options">
					<label>
						<input
							id="export-csv"
							name="export-format"
							type="radio"
							bind:group={format}
							value="csv"
						/>
						CSV Format
					</label>
					<label>
						<input
							id="export-json"
							name="export-format"
							type="radio"
							bind:group={format}
							value="json"
						/>
						JSON Format
					</label>
					<label>
						<input
							id="export-wordpress"
							name="export-format"
							type="radio"
							bind:group={format}
							value="wordpress"
						/>
						WordPress XML
					</label>
				</div>
				<p class="export-info">
					{selectedCount > 0
						? `Exporting ${selectedCount} selected posts`
						: 'Exporting all posts matching current filters'}
				</p>
			</div>
			<div class="modal-actions">
				<button class="btn-secondary" onclick={onClose}> Cancel </button>
				<button class="btn-primary" onclick={onExport}>
					<IconDownload size={18} />
					Export
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: rgba(30, 41, 59, 0.98);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 12px;
		width: 90%;
		max-width: 600px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.2);
	}

	.modal-header h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.modal-content {
		flex: 1;
		padding: 1.5rem;
		overflow-y: auto;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.2);
	}

	.export-options {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.export-options label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		cursor: pointer;
		color: #cbd5e1;
		transition: all 0.2s;
	}

	.export-options label:hover {
		border-color: rgba(59, 130, 246, 0.3);
	}

	.export-options label:has(:global(input[type='radio']:checked)) {
		background: rgba(59, 130, 246, 0.1);
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.export-info {
		color: #94a3b8;
		font-size: 0.9rem;
		padding: 1rem;
		background: rgba(59, 130, 246, 0.1);
		border-radius: 8px;
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-icon {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
		text-decoration: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		box-shadow: 0 2px 8px rgba(99, 102, 241, 0.25);
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(99, 102, 241, 0.4);
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.5);
	}

	.btn-icon {
		padding: 0.5rem;
		background: var(--admin-btn-bg);
		color: var(--admin-text-secondary);
	}

	.btn-icon:hover {
		background: var(--admin-btn-bg-hover);
		color: var(--admin-accent-primary);
	}

	@media (max-width: 639.98px) {
		.btn-primary,
		.btn-secondary {
			padding: 0.5rem 0.75rem;
			font-size: 0.8125rem;
			width: 100%;
			justify-content: center;
		}
	}

	@media (max-width: 380px) {
		.modal-content {
			padding: 1rem;
			margin: 0.5rem;
			max-width: calc(100vw - 1rem);
		}
	}

	@media (hover: none) and (pointer: coarse) {
		.btn-primary,
		.btn-secondary,
		.btn-icon {
			min-height: 44px;
			min-width: 44px;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.btn-primary,
		.btn-secondary {
			transition: none;
		}
	}

	@media (prefers-contrast: high) {
		.modal-content {
			border-width: 2px;
		}
	}
</style>
