<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * AlertCard Component - Individual Alert Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays a single alert with type-specific styling and interactions
	 * @version 4.1.0 - Visual Polish Pass
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { Alert } from '../types';
	import { formatPercent, formatPrice, formatRelativeTime } from '../utils/formatters';

	interface Props {
		alert: Alert;
		isAdmin?: boolean;
		onViewTradePlan?: (alert: Alert) => void;
		onEdit?: (alert: Alert) => void;
		onDelete?: (alertId: string) => void;
	}

	const { alert, isAdmin = false, onViewTradePlan, onEdit, onDelete }: Props = $props();

	let isNotesExpanded = $state(false);
	let isCopied = $state(false);

	// Get config based on alert type
	const alertConfig = $derived(() => {
		switch (alert.type) {
			case 'ENTRY':
				return {
					borderColor: 'border-l-teal-500',
					bgClass: alert.isNew ? 'bg-teal-50/60' : 'bg-white',
					badgeClass: 'badge-entry',
					badgeText: 'ENTRY',
					icon: `<path fill-rule="evenodd" d="M12 3.75a.75.75 0 01.75.75v6.75h6.75a.75.75 0 010 1.5h-6.75v6.75a.75.75 0 01-1.5 0v-6.75H4.5a.75.75 0 010-1.5h6.75V4.5a.75.75 0 01.75-.75z" clip-rule="evenodd" />`
				};
			case 'UPDATE':
				return {
					borderColor: 'border-l-amber-500',
					bgClass: alert.isNew ? 'bg-amber-50/60' : 'bg-white',
					badgeClass: 'badge-update',
					badgeText: 'UPDATE',
					icon: `<path fill-rule="evenodd" d="M4.755 10.059a7.5 7.5 0 0112.548-3.364l1.903 1.903h-3.183a.75.75 0 100 1.5h4.992a.75.75 0 00.75-.75V4.356a.75.75 0 00-1.5 0v3.18l-1.9-1.9A9 9 0 003.306 9.67a.75.75 0 101.45.388zm15.408 3.352a.75.75 0 00-.919.53 7.5 7.5 0 01-12.548 3.364l-1.902-1.903h3.183a.75.75 0 000-1.5H2.984a.75.75 0 00-.75.75v4.992a.75.75 0 001.5 0v-3.18l1.9 1.9a9 9 0 0015.059-4.035.75.75 0 00-.53-.918z" clip-rule="evenodd" />`
				};
			case 'EXIT':
				const isWin = alert.resultPercent !== undefined && alert.resultPercent >= 0;
				return {
					borderColor: isWin ? 'border-l-emerald-500' : 'border-l-red-500',
					bgClass: isWin ? 'bg-emerald-50/40' : 'bg-red-50/40',
					badgeClass: isWin ? 'badge-exit-win' : 'badge-exit-loss',
					badgeText: 'EXIT',
					icon: `<path fill-rule="evenodd" d="M16.5 4.478v.227a48.816 48.816 0 013.878.512.75.75 0 11-.256 1.478l-.209-.035-1.005 13.07a3 3 0 01-2.991 2.77H8.084a3 3 0 01-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 01-.256-1.478A48.567 48.567 0 017.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 013.369 0c1.603.051 2.815 1.387 2.815 2.951zm-6.136-1.452a51.196 51.196 0 013.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 00-6 0v-.113c0-.794.609-1.428 1.364-1.452zm-.355 5.945a.75.75 0 10-1.5.058l.347 9a.75.75 0 101.499-.058l-.346-9zm5.48.058a.75.75 0 10-1.498-.058l-.347 9a.75.75 0 001.5.058l.345-9z" clip-rule="evenodd" />`
				};
			default:
				return {
					borderColor: 'border-l-slate-400',
					bgClass: 'bg-white',
					badgeClass: 'badge-default',
					badgeText: alert.type,
					icon: ''
				};
		}
	});

	function toggleNotes() {
		isNotesExpanded = !isNotesExpanded;
	}

	async function copyToClipboard() {
		const text = `${alert.ticker} ${alert.type}\n${alert.title}\n${alert.description}${alert.tosString ? '\nTOS: ' + alert.tosString : ''}`;
		try {
			await navigator.clipboard.writeText(text);
			isCopied = true;
			setTimeout(() => {
				isCopied = false;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy:', err);
		}
	}
</script>

<article 
	class="alert-card {alertConfig().borderColor} {alertConfig().bgClass}"
	class:is-new={alert.isNew}
	class:has-notes-open={isNotesExpanded}
	aria-label="{alert.ticker} {alert.type} alert"
>
	<!-- NEW Badge -->
	{#if alert.isNew}
		<div class="new-badge">
			<span class="pulse-dot"></span>
			NEW
		</div>
	{/if}

	<!-- Header Row -->
	<div class="alert-header">
		<div class="header-left">
			<span class="type-badge {alertConfig().badgeClass}">
				{alertConfig().badgeText}
			</span>
			<span class="ticker">{alert.ticker}</span>
			<span class="timestamp">{formatRelativeTime(alert.timestamp)}</span>
		</div>
		
		<button 
			class="notes-toggle"
			class:expanded={isNotesExpanded}
			onclick={toggleNotes}
			aria-expanded={isNotesExpanded}
			aria-controls="notes-{alert.id}"
		>
			<span class="notes-label">Notes</span>
			<svg class="chevron" viewBox="0 0 20 20" fill="currentColor">
				<path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" clip-rule="evenodd" />
			</svg>
		</button>
	</div>

	<!-- Title -->
	<h3 class="alert-title">{alert.title}</h3>

	<!-- Description -->
	<p class="alert-description">{alert.description}</p>

	<!-- Price Info (for ENTRY alerts) -->
	{#if alert.type === 'ENTRY' && (alert.entryPrice || alert.targetPrice || alert.stopPrice)}
		<div class="price-info">
			{#if alert.entryPrice}
				<div class="price-item">
					<span class="price-label">Entry</span>
					<span class="price-value">{formatPrice(alert.entryPrice)}</span>
				</div>
			{/if}
			{#if alert.targetPrice}
				<div class="price-item">
					<span class="price-label">Target</span>
					<span class="price-value target">{formatPrice(alert.targetPrice)}</span>
				</div>
			{/if}
			{#if alert.stopPrice}
				<div class="price-item">
					<span class="price-label">Stop</span>
					<span class="price-value stop">{formatPrice(alert.stopPrice)}</span>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Result (for EXIT alerts) -->
	{#if alert.type === 'EXIT' && alert.resultPercent !== undefined}
		<div class="exit-result" class:positive={alert.resultPercent >= 0} class:negative={alert.resultPercent < 0}>
			<span class="result-value">{formatPercent(alert.resultPercent)}</span>
			<span class="result-label">{alert.resultPercent >= 0 ? 'Profit' : 'Loss'}</span>
		</div>
	{/if}

	<!-- TOS String -->
	{#if alert.tosString}
		<div class="tos-container">
			<code class="tos-string">{alert.tosString}</code>
			<button class="tos-copy" onclick={copyToClipboard} aria-label="Copy ThinkOrSwim string">
				{#if isCopied}
					<svg viewBox="0 0 20 20" fill="currentColor">
						<path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
					</svg>
				{:else}
					<svg viewBox="0 0 20 20" fill="currentColor">
						<path d="M7 3.5A1.5 1.5 0 018.5 2h3.879a1.5 1.5 0 011.06.44l3.122 3.12A1.5 1.5 0 0117 6.622V12.5a1.5 1.5 0 01-1.5 1.5h-1v-3.379a3 3 0 00-.879-2.121L10.5 5.379A3 3 0 008.379 4.5H7v-1z" />
						<path d="M4.5 6A1.5 1.5 0 003 7.5v9A1.5 1.5 0 004.5 18h7a1.5 1.5 0 001.5-1.5v-5.879a1.5 1.5 0 00-.44-1.06L9.44 6.439A1.5 1.5 0 008.378 6H4.5z" />
					</svg>
				{/if}
			</button>
		</div>
	{/if}

	<!-- Actions Row -->
	<div class="actions-row">
		<button class="action-btn copy-btn" class:copied={isCopied} onclick={copyToClipboard}>
			{#if isCopied}
				<svg viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
				</svg>
				Copied!
			{:else}
				<svg viewBox="0 0 20 20" fill="currentColor">
					<path d="M7 3.5A1.5 1.5 0 018.5 2h3.879a1.5 1.5 0 011.06.44l3.122 3.12A1.5 1.5 0 0117 6.622V12.5a1.5 1.5 0 01-1.5 1.5h-1v-3.379a3 3 0 00-.879-2.121L10.5 5.379A3 3 0 008.379 4.5H7v-1z" />
					<path d="M4.5 6A1.5 1.5 0 003 7.5v9A1.5 1.5 0 004.5 18h7a1.5 1.5 0 001.5-1.5v-5.879a1.5 1.5 0 00-.44-1.06L9.44 6.439A1.5 1.5 0 008.378 6H4.5z" />
				</svg>
				Copy
			{/if}
		</button>

		{#if alert.type === 'ENTRY' && onViewTradePlan}
			<button class="action-btn primary" onclick={() => onViewTradePlan(alert)}>
				View Trade Plan
				<svg viewBox="0 0 20 20" fill="currentColor">
					<path fill-rule="evenodd" d="M3 10a.75.75 0 01.75-.75h10.638L10.23 5.29a.75.75 0 111.04-1.08l5.5 5.25a.75.75 0 010 1.08l-5.5 5.25a.75.75 0 11-1.04-1.08l4.158-3.96H3.75A.75.75 0 013 10z" clip-rule="evenodd" />
				</svg>
			</button>
		{/if}

		{#if isAdmin}
			<div class="admin-actions">
				{#if onEdit}
					<button class="admin-btn edit" onclick={() => onEdit(alert)}>Edit</button>
				{/if}
				{#if onDelete}
					<button class="admin-btn delete" onclick={() => onDelete(alert.id)}>Delete</button>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Expandable Notes Panel -->
	{#if isNotesExpanded && alert.notes}
		<div class="notes-panel" id="notes-{alert.id}">
			<div class="notes-header">
				<span class="notes-ticker">{alert.ticker}</span>
				<span class="notes-title">Trade Notes</span>
			</div>
			<div class="notes-content">
				<p>{alert.notes}</p>
			</div>
		</div>
	{/if}
</article>

<style>
	/* ═══════════════════════════════════════════════════════════════════════
	   CARD BASE
	   ═══════════════════════════════════════════════════════════════════════ */
	.alert-card {
		position: relative;
		border: 1px solid #e2e8f0;
		border-left-width: 3px;
		border-radius: 10px;
		padding: 14px 16px;
		margin-bottom: 12px;
		transition: all 0.2s ease-out;
	}

	.alert-card:hover {
		border-color: #cbd5e1;
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);
	}

	.alert-card.is-new {
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
	}

	.alert-card.has-notes-open {
		border-color: #143e59;
		box-shadow: 0 8px 30px rgba(20, 62, 89, 0.12);
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   NEW BADGE
	   ═══════════════════════════════════════════════════════════════════════ */
	.new-badge {
		position: absolute;
		top: -8px;
		right: 12px;
		display: flex;
		align-items: center;
		gap: 4px;
		background: linear-gradient(135deg, #f69532 0%, #e8860d 100%);
		color: #fff;
		font-size: 9px;
		font-weight: 800;
		padding: 3px 8px;
		border-radius: 8px;
		letter-spacing: 0.05em;
		box-shadow: 0 2px 6px rgba(246, 149, 50, 0.35);
	}

	.pulse-dot {
		width: 5px;
		height: 5px;
		background: #fff;
		border-radius: 50%;
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; transform: scale(1); }
		50% { opacity: 0.6; transform: scale(1.2); }
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════ */
	.alert-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 8px;
		flex-wrap: wrap;
		gap: 8px;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 8px;
		flex-wrap: wrap;
	}

	/* Type Badges */
	.type-badge {
		font-size: 9px;
		font-weight: 800;
		padding: 3px 7px;
		border-radius: 4px;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}

	.badge-entry {
		background: linear-gradient(135deg, #ccfbf1 0%, #99f6e4 100%);
		color: #0f766e;
	}

	.badge-update {
		background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
		color: #92400e;
	}

	.badge-exit-win {
		background: linear-gradient(135deg, #dcfce7 0%, #bbf7d0 100%);
		color: #166534;
	}

	.badge-exit-loss {
		background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
		color: #991b1b;
	}

	.badge-default {
		background: #f1f5f9;
		color: #475569;
	}

	.ticker {
		font-size: 14px;
		font-weight: 700;
		color: #0f172a;
		letter-spacing: 0.02em;
	}

	.timestamp {
		font-size: 11px;
		color: #94a3b8;
	}

	/* Notes Toggle */
	.notes-toggle {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 5px 10px;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 6px;
		font-size: 11px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.notes-toggle:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
		color: #334155;
	}

	.notes-toggle.expanded {
		background: #143e59;
		border-color: #143e59;
		color: #fff;
	}

	.notes-toggle .chevron {
		width: 14px;
		height: 14px;
		transition: transform 0.3s ease;
	}

	.notes-toggle.expanded .chevron {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   CONTENT
	   ═══════════════════════════════════════════════════════════════════════ */
	.alert-title {
		font-size: 14px;
		font-weight: 600;
		color: #0f172a;
		margin: 0 0 6px 0;
		line-height: 1.35;
	}

	.alert-description {
		font-size: 13px;
		color: #475569;
		margin: 0 0 12px 0;
		line-height: 1.5;
	}

	/* Price Info */
	.price-info {
		display: flex;
		gap: 16px;
		padding: 10px 12px;
		background: #f8fafc;
		border-radius: 8px;
		margin-bottom: 12px;
		flex-wrap: wrap;
	}

	.price-item {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.price-label {
		font-size: 11px;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.price-value {
		font-size: 14px;
		font-weight: 700;
		color: #0f172a;
		font-variant-numeric: tabular-nums;
	}

	.price-value.target {
		color: #059669;
	}

	.price-value.stop {
		color: #dc2626;
	}

	/* Exit Result */
	.exit-result {
		display: flex;
		align-items: baseline;
		gap: 6px;
		padding: 8px 12px;
		border-radius: 8px;
		margin-bottom: 12px;
	}

	.exit-result.positive {
		background: linear-gradient(135deg, #ecfdf5 0%, #d1fae5 100%);
	}

	.exit-result.negative {
		background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
	}

	.result-value {
		font-size: 18px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.exit-result.positive .result-value {
		color: #059669;
	}

	.exit-result.negative .result-value {
		color: #dc2626;
	}

	.result-label {
		font-size: 12px;
		font-weight: 600;
		color: #64748b;
	}

	/* TOS String */
	.tos-container {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 8px;
		background: #1e293b;
		border-radius: 6px;
		padding: 8px 10px;
		margin-bottom: 12px;
	}

	.tos-string {
		font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
		font-size: 11px;
		color: #94a3b8;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.tos-copy {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 5px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tos-copy:hover {
		background: rgba(255, 255, 255, 0.2);
		color: #fff;
	}

	.tos-copy svg {
		width: 14px;
		height: 14px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   ACTIONS
	   ═══════════════════════════════════════════════════════════════════════ */
	.actions-row {
		display: flex;
		align-items: center;
		gap: 8px;
		flex-wrap: wrap;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 5px;
		padding: 6px 10px;
		font-size: 12px;
		font-weight: 600;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
		border: 1px solid;
	}

	.action-btn svg {
		width: 14px;
		height: 14px;
	}

	.copy-btn {
		background: #f8fafc;
		border-color: #e2e8f0;
		color: #64748b;
	}

	.copy-btn:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
		color: #334155;
	}

	.copy-btn.copied {
		background: #dcfce7;
		border-color: #86efac;
		color: #166534;
	}

	.action-btn.primary {
		background: #143e59;
		border-color: #143e59;
		color: #fff;
	}

	.action-btn.primary:hover {
		background: #1e5175;
		border-color: #1e5175;
	}

	/* Admin Actions */
	.admin-actions {
		display: flex;
		gap: 8px;
		margin-left: auto;
	}

	.admin-btn {
		padding: 6px 12px;
		font-size: 12px;
		font-weight: 600;
		border-radius: 6px;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.admin-btn.edit {
		background: #e0f2fe;
		color: #0369a1;
	}

	.admin-btn.edit:hover {
		background: #bae6fd;
	}

	.admin-btn.delete {
		background: #fee2e2;
		color: #991b1b;
	}

	.admin-btn.delete:hover {
		background: #fecaca;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   NOTES PANEL
	   ═══════════════════════════════════════════════════════════════════════ */
	.notes-panel {
		margin-top: 16px;
		border-radius: 12px;
		overflow: hidden;
		border: 1px solid #e2e8f0;
		animation: slideDown 0.3s ease-out;
	}

	@keyframes slideDown {
		from {
			opacity: 0;
			transform: translateY(-8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.notes-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 18px;
		background: linear-gradient(135deg, #143e59 0%, #1e5175 100%);
	}

	.notes-ticker {
		background: #fff;
		color: #143e59;
		font-size: 11px;
		font-weight: 800;
		padding: 5px 12px;
		border-radius: 6px;
		letter-spacing: 0.05em;
	}

	.notes-title {
		color: rgba(255, 255, 255, 0.9);
		font-size: 13px;
		font-weight: 600;
	}

	.notes-content {
		padding: 18px 20px;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
	}

	.notes-content p {
		font-size: 14px;
		color: #334155;
		line-height: 1.7;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════ */
	@media (max-width: 640px) {
		.alert-card {
			padding: 16px;
		}

		.alert-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.notes-toggle {
			align-self: flex-end;
			margin-top: -32px;
		}

		.price-info {
			flex-direction: column;
			gap: 12px;
		}

		.actions-row {
			flex-direction: column;
			align-items: stretch;
		}

		.admin-actions {
			margin-left: 0;
			justify-content: flex-end;
		}
	}
</style>