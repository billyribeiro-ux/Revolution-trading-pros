<script lang="ts">
	import {
		IconRefresh,
		IconUpload,
		IconDownload,
		IconFileSpreadsheet,
		IconPdf,
		IconAlertTriangle,
		IconUserPlus
	} from '$lib/icons';

	export type HeaderAction =
		| { type: 'refresh' }
		| { type: 'import' }
		| { type: 'export'; format: 'csv' | 'xlsx' | 'pdf' }
		| { type: 'win-back' }
		| { type: 'create' };

	interface Props {
		exportFormat: 'csv' | 'xlsx' | 'pdf';
		exporting: boolean;
		onAction: (action: HeaderAction) => void;
	}

	let { exportFormat, exporting, onAction }: Props = $props();
</script>

<header class="page-header">
	<h1>Members Command Center</h1>
	<p class="subtitle">Comprehensive member management and analytics</p>
	<div class="header-actions">
		<button
			class="btn-secondary"
			onclick={() => onAction({ type: 'refresh' })}
			title="Refresh data"
		>
			<IconRefresh size={18} />
			Refresh
		</button>
		<button class="btn-secondary" onclick={() => onAction({ type: 'import' })}>
			<IconUpload size={18} />
			Import
		</button>
		<div class="export-dropdown">
			<button
				class="btn-secondary"
				onclick={() => onAction({ type: 'export', format: exportFormat })}
				disabled={exporting}
			>
				<IconDownload size={18} />
				{exporting ? 'Exporting...' : `Export ${exportFormat.toUpperCase()}`}
			</button>
			<div class="export-options">
				<button onclick={() => onAction({ type: 'export', format: 'csv' })} disabled={exporting}>
					<IconDownload size={14} />
					CSV
				</button>
				<button onclick={() => onAction({ type: 'export', format: 'xlsx' })} disabled={exporting}>
					<IconFileSpreadsheet size={14} />
					Excel
				</button>
				<button onclick={() => onAction({ type: 'export', format: 'pdf' })} disabled={exporting}>
					<IconPdf size={14} />
					PDF
				</button>
			</div>
		</div>
		<button class="btn-secondary" onclick={() => onAction({ type: 'win-back' })}>
			<IconAlertTriangle size={18} />
			Win-Back
		</button>
		<button class="btn-primary" onclick={() => onAction({ type: 'create' })}>
			<IconUserPlus size={18} />
			Create Member
		</button>
	</div>
</header>

<style>
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1.5rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	.export-dropdown {
		position: relative;
		display: inline-block;
	}

	.export-dropdown:hover .export-options {
		display: flex;
	}

	.export-options {
		display: none;
		position: absolute;
		top: 100%;
		left: 0;
		z-index: 20;
		flex-direction: column;
		min-width: 140px;
		background: var(--admin-surface-primary, rgba(30, 41, 59, 0.98));
		border: 1px solid var(--admin-border-subtle, rgba(148, 163, 184, 0.15));
		border-radius: var(--radius-md, 0.5rem);
		padding: 0.375rem;
		margin-top: 0.25rem;
		box-shadow: 0 10px 30px -5px rgba(0, 0, 0, 0.4);
		animation: exportDropIn 0.15s ease;
	}

	@keyframes exportDropIn {
		from {
			opacity: 0;
			transform: translateY(-6px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.export-options button {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		color: var(--admin-text-secondary, #cbd5e1);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.8125rem;
		font-weight: 500;
		text-align: left;
		border-radius: var(--radius-sm, 0.25rem);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.export-options button:hover:not(:disabled) {
		background: var(--admin-surface-hover, rgba(230, 184, 0, 0.1));
		color: var(--primary-500);
	}

	.export-options button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Duplicated from parent — btn-primary / btn-secondary live in parent's scoped CSS */
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

	/* Responsive */
	@media (max-width: 1023px) {
		.header-actions {
			flex-wrap: wrap;
			gap: 0.5rem;
		}
	}

	@media (max-width: 767px) {
		.header-actions {
			flex-wrap: wrap;
		}
	}

	@media (max-width: 380px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.75rem;
		}

		.header-actions {
			width: 100%;
		}
	}

	@media print {
		.header-actions {
			display: none;
		}
	}
</style>
