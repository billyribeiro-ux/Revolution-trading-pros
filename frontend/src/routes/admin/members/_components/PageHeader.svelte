<script lang="ts">
	import {
		IconRefresh,
		IconUpload,
		IconDownload,
		IconFileSpreadsheet,
		IconPdf,
		IconAlertTriangle,
		IconUserPlus,
		IconChevronDown
	} from '#lib/icons/index.js';

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
	const formatId = $props.id();
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
			<button
				type="button"
				class="btn-secondary export-format-toggle"
				popovertarget={formatId}
				aria-label="Choose export format"
				disabled={exporting}
			>
				<IconChevronDown size={18} />
			</button>
			<div
				class="export-options"
				id={formatId}
				popover="auto"
				role="group"
				aria-label="Export formats"
			>
				<button
					type="button"
					popovertarget={formatId}
					popovertargetaction="hide"
					onclick={() => onAction({ type: 'export', format: 'csv' })}
					disabled={exporting}
				>
					<IconDownload size={14} />
					CSV
				</button>
				<button
					type="button"
					popovertarget={formatId}
					popovertargetaction="hide"
					onclick={() => onAction({ type: 'export', format: 'xlsx' })}
					disabled={exporting}
				>
					<IconFileSpreadsheet size={14} />
					Excel
				</button>
				<button
					type="button"
					popovertarget={formatId}
					popovertargetaction="hide"
					onclick={() => onAction({ type: 'export', format: 'pdf' })}
					disabled={exporting}
				>
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
		display: inline-flex;
		gap: 0.25rem;
	}

	.export-dropdown button {
		scroll-margin-block: calc(var(--admin-header-height, 70px) + 1rem);
	}

	.export-format-toggle {
		padding-inline: 0.75rem;
	}

	.export-options:popover-open {
		display: flex;
	}

	.export-options {
		display: none;
		position: fixed;
		inset: 0;
		width: max-content;
		max-width: calc(100vw - 2rem);
		max-height: calc(100dvh - 2rem);
		overflow: auto;
		flex-direction: column;
		min-width: 140px;
		background: var(--admin-surface-primary, var(--bg-elevated, #161b22));
		border: 1px solid var(--admin-border-subtle, var(--border-muted, #21262d));
		border-radius: var(--radius-md, 0.5rem);
		padding: 0.375rem;
		margin: auto;
		box-shadow: 0 10px 30px -5px rgba(0, 0, 0, 0.4);
		animation: exportDropIn 0.15s ease;
	}

	@supports (top: anchor(bottom)) {
		.export-options {
			position-anchor: auto;
			inset: auto;
			top: calc(anchor(bottom) + 0.25rem);
			left: anchor(left);
			margin: 0;
			position-try-fallbacks:
				flip-block,
				flip-inline,
				flip-block flip-inline;
		}
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
		color: var(--admin-text-secondary, var(--text-secondary, #8b949e));
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
		background: var(--bg-hover, #252b33);
		color: var(--text-primary);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary);
	}

	.btn-primary {
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.btn-primary:hover:not(:disabled) {
		background: var(--primary-400);
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
