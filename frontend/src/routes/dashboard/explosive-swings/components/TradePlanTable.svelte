<script lang="ts">
	/**
	 * TradePlanTable Component - Apple Principal Engineer ICT Level 7
	 * Extracted from WeeklyHero.svelte for better maintainability
	 * 
	 * @description Trade plan table with expandable notes
	 * @version 1.0.0 - ICT 7 compliance
	 */

	interface TradePlanEntry {
		ticker: string;
		bias: 'Bullish' | 'Bearish' | 'Neutral' | string;
		entry: string;
		target1: string;
		target2: string;
		target3: string;
		runner: string;
		stop: string;
		optionsStrike: string;
		optionsExp: string;
		notes: string;
	}

	interface Props {
		tradePlan: TradePlanEntry[];
		sheetUrl?: string;
		isAdmin?: boolean;
		onAddEntry?: () => void;
		onEditEntry?: (entry: TradePlanEntry) => void;
	}

	const {
		tradePlan,
		sheetUrl = 'https://docs.google.com/spreadsheets/d/your-sheet-id',
		isAdmin = false,
		onAddEntry,
		onEditEntry
	}: Props = $props();

	let expandedTradeNotes = $state(new Set<string>());

	function toggleTradeNotes(ticker: string) {
		const newSet = new Set(expandedTradeNotes);
		if (newSet.has(ticker)) {
			newSet.delete(ticker);
		} else {
			newSet.add(ticker);
		}
		expandedTradeNotes = newSet;
	}
</script>

<div class="trade-sheet-container">
	<div class="trade-sheet-header">
		<h3>Trade Plan & Entries</h3>
		{#if isAdmin && onAddEntry}
			<button class="add-entry-btn" onclick={onAddEntry} type="button">
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					width="18"
					height="18"
					aria-hidden="true"
				>
					<path d="M12 5v14M5 12h14" />
				</svg>
				Add Entry
			</button>
		{/if}
	</div>
	<div class="table-wrapper">
		<table class="trade-sheet">
			<thead>
				<tr>
					<th>Ticker</th>
					<th>Bias</th>
					<th>Entry</th>
					<th>Target 1</th>
					<th>Target 2</th>
					<th>Target 3</th>
					<th>Runner</th>
					<th>Stop</th>
					<th>Options</th>
					<th>Exp</th>
					<th class="notes-th">Notes</th>
				</tr>
			</thead>
			<tbody>
				{#each tradePlan as trade (trade.ticker)}
					<tr class:has-notes-open={expandedTradeNotes.has(trade.ticker)}>
						<td class="ticker-cell">
							<strong>{trade.ticker}</strong>
							{#if isAdmin && onEditEntry}
								<button
									class="edit-entry-btn"
									onclick={() => onEditEntry(trade)}
									aria-label="Edit {trade.ticker}"
								>
									<svg
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										width="14"
										height="14"
									>
										<path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7" />
										<path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z" />
									</svg>
								</button>
							{/if}
						</td>
						<td>
							<span class="bias bias--{trade.bias.toLowerCase()}">{trade.bias}</span>
						</td>
						<td class="entry-cell">{trade.entry}</td>
						<td class="target-cell">{trade.target1}</td>
						<td class="target-cell">{trade.target2}</td>
						<td class="target-cell">{trade.target3}</td>
						<td class="runner-cell">{trade.runner}</td>
						<td class="stop-cell">{trade.stop}</td>
						<td class="options-cell">{trade.optionsStrike}</td>
						<td class="exp-cell">{trade.optionsExp}</td>
						<td class="notes-toggle-cell">
							<button
								class="table-notes-btn"
								class:expanded={expandedTradeNotes.has(trade.ticker)}
								onclick={() => toggleTradeNotes(trade.ticker)}
								aria-label="Toggle notes for {trade.ticker}"
							>
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
						</td>
					</tr>
					{#if expandedTradeNotes.has(trade.ticker)}
						<tr class="notes-row expanded">
							<td colspan="11">
								<div class="trade-notes-panel">
									<div class="trade-notes-badge">{trade.ticker}</div>
									<p>{trade.notes}</p>
								</div>
							</td>
						</tr>
					{/if}
				{/each}
			</tbody>
		</table>
	</div>
	<div class="sheet-footer">
		<a href={sheetUrl} target="_blank" class="google-sheet-link">
			<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
				<path
					d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 17h2v-7H7v7zm4 0h2V7h-2v10zm4 0h2v-4h-2v4z"
				/>
			</svg>
			Open in Google Sheets
		</a>
	</div>
</div>

<style>
	.trade-sheet-container {
		background: var(--color-bg-card);
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
	}

	.trade-sheet-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 24px;
		background: linear-gradient(135deg, #f0f9ff 0%, var(--color-info-bg) 100%);
		border-bottom: 2px solid var(--color-brand-primary);
	}

	.trade-sheet-header h3 {
		font-family:
			'Montserrat',
			-apple-system,
			BlinkMacSystemFont,
			sans-serif;
		font-size: 20px;
		font-weight: 700;
		color: var(--color-brand-primary);
		margin: 0;
	}

	.add-entry-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: var(--color-brand-primary);
		color: var(--color-bg-card);
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.add-entry-btn:hover {
		background: var(--color-brand-primary-hover);
		transform: translateY(-1px);
	}

	.table-wrapper {
		overflow-x: auto;
		-webkit-overflow-scrolling: touch;
	}

	.trade-sheet {
		width: 100%;
		border-collapse: collapse;
		font-size: 13px;
		min-width: 900px;
	}

	.trade-sheet th {
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
		font-weight: 700;
		text-align: left;
		padding: 14px 12px;
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-bottom: 2px solid var(--color-border-default);
		white-space: nowrap;
	}

	.trade-sheet td {
		padding: 14px 12px;
		border-bottom: 1px solid var(--color-border-subtle);
		color: var(--color-text-primary);
	}

	.trade-sheet tbody tr {
		transition: background 0.15s ease;
	}

	.trade-sheet tbody tr:hover {
		background: var(--color-bg-subtle);
	}

	.ticker-cell {
		font-weight: 700;
		color: var(--color-brand-primary);
		position: relative;
	}

	.edit-entry-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		color: var(--color-text-muted);
		cursor: pointer;
		margin-left: 8px;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.edit-entry-btn:hover {
		background: var(--color-bg-subtle);
		color: var(--color-brand-primary);
	}

	.bias {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.bias--bullish {
		background: var(--color-success-bg);
		color: var(--color-success);
	}

	.bias--bearish {
		background: var(--color-loss-bg);
		color: var(--color-loss);
	}

	.bias--neutral {
		background: var(--color-bg-subtle);
		color: var(--color-text-muted);
	}

	.entry-cell,
	.target-cell,
	.runner-cell {
		font-weight: 600;
		color: var(--color-success);
	}

	.stop-cell {
		color: var(--color-loss);
		font-weight: 600;
	}

	.options-cell {
		font-weight: 600;
		color: var(--color-purple-dark);
	}

	.exp-cell {
		font-size: 12px;
		color: var(--color-text-tertiary);
	}

	.notes-th {
		width: 60px;
		text-align: center;
	}

	.notes-toggle-cell {
		text-align: center;
		vertical-align: middle;
	}

	.table-notes-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: var(--color-bg-subtle);
		border: 1.5px solid var(--color-border-default);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.25s ease;
		color: var(--color-text-muted);
	}

	.table-notes-btn:hover {
		background: var(--color-border-default);
		border-color: var(--color-brand-primary);
		color: var(--color-brand-primary);
	}

	.table-notes-btn.expanded {
		background: var(--color-brand-primary);
		border-color: var(--color-brand-primary);
		color: var(--color-bg-card);
	}

	.table-notes-btn .chevron-icon {
		transition: transform 0.3s ease;
	}

	.table-notes-btn.expanded .chevron-icon {
		transform: rotate(180deg);
	}

	.table-notes-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	tr.has-notes-open td {
		background: #f0f9ff !important;
	}

	.notes-row.expanded td {
		text-align: left;
		padding: 0;
		background: transparent !important;
		border-bottom: 2px solid var(--color-brand-primary);
	}

	.trade-notes-panel {
		display: flex;
		align-items: flex-start;
		gap: 16px;
		padding: 18px 24px;
		background: linear-gradient(135deg, #f0f9ff 0%, var(--color-info-bg) 100%);
		animation: notesSlide 0.3s ease;
	}

	@keyframes notesSlide {
		from {
			opacity: 0;
			transform: translateY(-8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.trade-notes-badge {
		flex-shrink: 0;
		background: var(--color-brand-primary);
		color: var(--color-bg-card);
		font-size: 12px;
		font-weight: 800;
		padding: 6px 14px;
		border-radius: 6px;
		letter-spacing: 0.05em;
	}

	.trade-notes-panel p {
		flex: 1;
		font-size: 14px;
		color: #0c4a6e;
		line-height: 1.7;
		margin: 0;
		font-weight: 500;
	}

	.sheet-footer {
		padding: 20px;
		text-align: center;
	}

	.google-sheet-link {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		background: var(--color-brand-primary);
		color: var(--color-bg-card);
		padding: 14px 28px;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s;
	}

	.google-sheet-link:hover {
		background: var(--color-brand-primary-hover);
	}

	/* Responsive */
	@media (min-width: 1024px) {
		.trade-sheet {
			font-size: var(--text-base);
			min-width: 100%;
		}
	}

	@media (min-width: 1920px) {
		.trade-sheet th {
			padding: 18px 16px;
			font-size: 13px;
		}

		.trade-sheet td {
			padding: 16px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.table-notes-btn,
		.add-entry-btn {
			transition: none;
		}

		.trade-notes-panel {
			animation: none;
		}
	}
</style>

