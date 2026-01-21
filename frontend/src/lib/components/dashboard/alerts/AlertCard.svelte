<!--
	AlertCard.svelte - Single Alert Display Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple Principal Engineer ICT 11+ Standards
	Svelte 5 January 2026 Syntax
	
	Single Responsibility: Display a single alert with its badge, content, and actions
	
	@version 1.0.0
	@since January 2026
-->
<script lang="ts">
	import type { Alert, RoomAlert } from './types';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 $props() pattern
	// ═══════════════════════════════════════════════════════════════════════════
	interface Props {
		alert: Alert;
		index?: number;
		isAdmin?: boolean;
		isNotesExpanded?: boolean;
		onToggleNotes?: (id: number) => void;
		onCopy?: (alert: Alert) => void;
		onEdit?: (alert: Alert) => void;
		onDelete?: (id: number) => void;
		isCopied?: boolean;
	}

	let {
		alert,
		index = 0,
		isAdmin = false,
		isNotesExpanded = false,
		onToggleNotes,
		onCopy,
		onEdit,
		onDelete,
		isCopied = false
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function extractPrice(message: string): string | null {
		const priceMatch = message.match(/\$[\d,]+\.?\d*/g);
		return priceMatch ? priceMatch[0] : null;
	}
</script>

<div
	class="alert-card"
	class:is-new={alert.isNew}
	class:has-notes-open={isNotesExpanded}
	style="animation-delay: {index * 50}ms"
>
	{#if alert.isNew}
		<span class="new-badge pulse">NEW</span>
	{/if}

	<!-- Alert Header Row with Directional Icon -->
	<div class="alert-row">
		<div class="alert-info">
			<!-- Directional Arrow Icon -->
			{#if alert.type === 'ENTRY'}
				<svg class="direction-icon direction-up" viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
					<path d="M7 14l5-5 5 5H7z"/>
				</svg>
			{:else if alert.type === 'EXIT'}
				<svg class="direction-icon direction-down" viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
					<path d="M7 10l5 5 5-5H7z"/>
				</svg>
			{:else}
				<svg class="direction-icon direction-neutral" viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
					<circle cx="12" cy="12" r="4"/>
				</svg>
			{/if}
			<span class="alert-type alert-type--{alert.type.toLowerCase()}">{alert.type}</span>
			<span class="alert-ticker">{alert.ticker}</span>
			<span class="alert-time">{alert.time}</span>
		</div>
		<!-- Price Display (Bold, Prominent) -->
		{#if extractPrice(alert.message)}
			<div class="alert-price">
				<span class="price-value">{extractPrice(alert.message)}</span>
			</div>
		{/if}
	</div>

	<h3>{alert.title}</h3>
	<p class="alert-message">{alert.message}</p>

	<!-- Action Buttons Row -->
	<div class="alert-actions-row">
		<button
			class="copy-btn"
			class:copied={isCopied}
			onclick={() => onCopy?.(alert)}
			aria-label="Copy trade details"
		>
			{#if isCopied}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="16" height="16">
					<path d="M20 6L9 17l-5-5"/>
				</svg>
				<span>Copied!</span>
			{:else}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
					<rect x="9" y="9" width="13" height="13" rx="2"/>
					<path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
				</svg>
				<span>Copy</span>
			{/if}
		</button>
		<button
			class="notes-chevron"
			class:expanded={isNotesExpanded}
			onclick={() => onToggleNotes?.(alert.id)}
			aria-label="Toggle trade notes"
			aria-expanded={isNotesExpanded}
		>
			<span class="notes-label">Notes</span>
			<svg
				class="chevron-icon"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				width="18"
				height="18"
			>
				<path d="M19 9l-7 7-7-7" />
			</svg>
		</button>
	</div>

	<!-- TOS String (if available) -->
	{#if alert.tosString}
		<div class="tos-display">
			<code>{alert.tosString}</code>
			<button 
				class="tos-copy-btn"
				onclick={() => navigator.clipboard.writeText(alert.tosString || '')}
				aria-label="Copy TOS string"
			>
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14">
					<rect x="9" y="9" width="13" height="13" rx="2"/>
					<path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
				</svg>
			</button>
		</div>
	{/if}

	<!-- Expandable Notes Panel -->
	{#if isNotesExpanded}
		<div class="notes-panel">
			<div class="notes-panel-header">
				<div class="notes-ticker-badge">{alert.ticker}</div>
				<span class="notes-title">Trade Analysis & Notes</span>
			</div>
			<div class="notes-panel-body">
				<p>{alert.notes}</p>
			</div>
		</div>
	{/if}

	<!-- Admin Actions -->
	{#if isAdmin}
		<div class="admin-actions">
			<button
				class="admin-action-btn edit"
				onclick={() => onEdit?.(alert)}
			>
				Edit
			</button>
			<button 
				class="admin-action-btn delete" 
				onclick={() => onDelete?.(alert.id)}
			>
				Delete
			</button>
		</div>
	{/if}
</div>

<style>
	/* Alert Card with Slide-in Animation */
	.alert-card {
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 20px;
		position: relative;
		transition: all 0.2s;
		text-align: left;
		animation: alertSlideIn 0.4s ease-out backwards;
	}

	@keyframes alertSlideIn {
		from {
			opacity: 0;
			transform: translateY(-12px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.alert-card:hover {
		border-color: #143e59;
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.08);
	}

	.alert-card.is-new {
		background: #fffbf5;
		border-color: #f69532;
		box-shadow: 0 4px 15px rgba(246, 149, 50, 0.15);
	}

	.alert-card.has-notes-open {
		border-color: #143e59;
		box-shadow: 0 8px 30px rgba(20, 62, 89, 0.12);
	}

	.new-badge {
		position: absolute;
		top: -10px;
		right: 15px;
		background: #f69532;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		padding: 4px 12px;
		border-radius: 10px;
	}

	.new-badge.pulse {
		animation: badgePulse 2s ease-in-out infinite;
	}

	@keyframes badgePulse {
		0%, 100% { box-shadow: 0 0 0 0 rgba(246, 149, 50, 0.4); }
		50% { box-shadow: 0 0 0 8px rgba(246, 149, 50, 0); }
	}

	/* Directional Icons */
	.direction-icon {
		flex-shrink: 0;
	}

	.direction-up {
		color: #22c55e;
	}

	.direction-down {
		color: #3b82f6;
	}

	.direction-neutral {
		color: #f59e0b;
	}

	/* Alert Row */
	.alert-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 12px;
	}

	.alert-info {
		display: flex;
		align-items: center;
		gap: 12px;
		flex-wrap: wrap;
	}

	.alert-type {
		font-size: 11px;
		font-weight: 700;
		padding: 4px 10px;
		border-radius: 4px;
	}

	.alert-type--entry {
		background: #dcfce7;
		color: #166534;
	}

	.alert-type--exit {
		background: #dbeafe;
		color: #1e40af;
	}

	.alert-type--update {
		background: #fef3c7;
		color: #92400e;
	}

	.alert-ticker {
		font-size: 16px;
		font-weight: 700;
		color: #143e59;
	}

	.alert-time {
		font-size: 12px;
		color: #888;
	}

	/* Price Display */
	.alert-price {
		background: linear-gradient(135deg, #143e59 0%, #1e5175 100%);
		padding: 8px 16px;
		border-radius: 8px;
		margin-left: auto;
	}

	.price-value {
		color: #fff;
		font-size: 16px;
		font-weight: 700;
		font-family: 'Montserrat', sans-serif;
	}

	.alert-card h3 {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 8px 0;
		color: #333;
	}

	.alert-card p.alert-message {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0;
	}

	/* Action Buttons Row */
	.alert-actions-row {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 10px;
		margin-top: 12px;
		padding-top: 12px;
		border-top: 1px solid #e5e7eb;
	}

	/* Copy Button */
	.copy-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		background: #f1f5f9;
		border: 1px solid #e2e8f0;
		padding: 6px 12px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.copy-btn:hover {
		background: #e2e8f0;
		color: #143e59;
		border-color: #143e59;
	}

	.copy-btn.copied {
		background: #dcfce7;
		border-color: #22c55e;
		color: #166534;
	}

	/* Notes Chevron */
	.notes-chevron {
		display: flex;
		align-items: center;
		gap: 6px;
		background: transparent;
		border: none;
		padding: 8px 12px;
		border-radius: 8px;
		font-size: 12px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		transition: all 0.25s ease;
	}

	.notes-chevron:hover {
		background: #f1f5f9;
		color: #143e59;
	}

	.notes-chevron.expanded {
		background: #143e59;
		color: #fff;
	}

	.notes-label {
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.chevron-icon {
		transition: transform 0.3s ease;
	}

	.notes-chevron.expanded .chevron-icon {
		transform: rotate(180deg);
	}

	/* TOS Display */
	.tos-display {
		display: flex;
		align-items: center;
		justify-content: space-between;
		background: #1a1a2e;
		border-radius: 8px;
		padding: 10px 14px;
		margin-top: 12px;
	}

	.tos-display code {
		color: #22c55e;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 13px;
		font-weight: 600;
	}

	.tos-copy-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		padding: 6px;
		border-radius: 4px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tos-copy-btn:hover {
		background: rgba(255, 255, 255, 0.2);
		color: #fff;
	}

	/* Notes Panel */
	.notes-panel {
		margin-top: 16px;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border-radius: 12px;
		overflow: hidden;
		border: 1px solid #e2e8f0;
		animation: panelSlide 0.35s cubic-bezier(0.4, 0, 0.2, 1);
	}

	@keyframes panelSlide {
		from {
			opacity: 0;
			transform: translateY(-8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.notes-panel-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 18px;
		background: linear-gradient(135deg, #143e59 0%, #1e5175 100%);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.notes-ticker-badge {
		background: #fff;
		color: #143e59;
		font-size: 12px;
		font-weight: 800;
		padding: 5px 12px;
		border-radius: 6px;
		letter-spacing: 0.05em;
	}

	.notes-title {
		color: rgba(255, 255, 255, 0.9);
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 0.02em;
	}

	.notes-panel-body {
		padding: 18px 20px;
	}

	.notes-panel-body p {
		font-size: 14px;
		color: #334155;
		line-height: 1.75;
		margin: 0;
		font-weight: 500;
	}

	/* Admin Actions */
	.admin-actions {
		display: flex;
		gap: 8px;
		margin-top: 12px;
		padding-top: 12px;
		border-top: 1px solid #e5e7eb;
	}

	.admin-action-btn {
		padding: 6px 14px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.admin-action-btn.edit {
		background: #e0f2fe;
		color: #0369a1;
	}

	.admin-action-btn.edit:hover {
		background: #bae6fd;
	}

	.admin-action-btn.delete {
		background: #fee2e2;
		color: #991b1b;
	}

	.admin-action-btn.delete:hover {
		background: #fecaca;
	}
</style>
