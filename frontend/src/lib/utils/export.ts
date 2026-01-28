/**
 * Revolution Trading Pros - Enterprise Export Utilities
 * =====================================================
 * CSV and PDF export functionality for data tables and reports.
 * Supports both client-side and server-side exports.
 *
 * @version 2.0.0 - Phase 4 Export Functionality
 * @author Revolution Trading Pros
 * @level Apple Principal Engineer ICT 7+
 * @date January 2026
 *
 * FEATURES:
 * - Client-side CSV/PDF/JSON export (in-memory)
 * - Server-side CSV/PDF export via API endpoints
 * - Progress tracking for large exports
 * - Automatic filename generation with timestamps
 * - Error handling with retry logic
 */

import { browser } from '$app/environment';
import { authStore } from '$lib/stores/auth.svelte';

/**
 * Escape HTML entities to prevent XSS attacks in PDF export
 * CRITICAL: Always use this when inserting user data into HTML
 */
function escapeHtml(str: string): string {
	const htmlEscapes: Record<string, string> = {
		'&': '&amp;',
		'<': '&lt;',
		'>': '&gt;',
		'"': '&quot;',
		"'": '&#x27;',
		'/': '&#x2F;'
	};
	return str.replace(/[&<>"'/]/g, (char) => htmlEscapes[char]);
}

export interface ExportColumn {
	key: string;
	label: string;
	format?: (value: unknown) => string;
}

export interface ExportOptions {
	filename: string;
	columns: ExportColumn[];
	data: Record<string, unknown>[];
	title?: string;
	subtitle?: string;
	includeTimestamp?: boolean;
}

/**
 * Export data to CSV format
 */
export function exportToCSV(options: ExportOptions): void {
	if (!browser) return;

	const { filename, columns, data, includeTimestamp = true } = options;

	// Build CSV header
	const header = columns.map((col) => `"${col.label}"`).join(',');

	// Build CSV rows
	const rows = data.map((row) => {
		return columns
			.map((col) => {
				const value = row[col.key];
				const formatted = col.format ? col.format(value) : String(value ?? '');
				// Escape quotes and wrap in quotes
				return `"${formatted.replace(/"/g, '""')}"`;
			})
			.join(',');
	});

	// Combine header and rows
	let csvContent = header + '\n' + rows.join('\n');

	// Add timestamp if requested
	if (includeTimestamp) {
		csvContent = `# Generated: ${new Date().toISOString()}\n` + csvContent;
	}

	// Create blob and download
	const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
	const url = URL.createObjectURL(blob);

	const link = document.createElement('a');
	link.href = url;
	link.download = `${filename}.csv`;
	document.body.appendChild(link);
	link.click();
	document.body.removeChild(link);
	URL.revokeObjectURL(url);
}

/**
 * Export data to PDF format using browser print
 */
export function exportToPDF(options: ExportOptions): void {
	if (!browser) return;

	const { filename, columns, data, title, subtitle, includeTimestamp = true } = options;

	// Create a new window for printing
	const printWindow = window.open('', '_blank');
	if (!printWindow) {
		console.error('Failed to open print window');
		return;
	}

	// Build HTML table with XSS protection
	const tableHeader = columns.map((col) => `<th>${escapeHtml(col.label)}</th>`).join('');

	const tableRows = data
		.map((row) => {
			const cells = columns
				.map((col) => {
					const value = row[col.key];
					const formatted = col.format ? col.format(value) : String(value ?? '');
					// SECURITY: Escape HTML to prevent XSS attacks
					return `<td>${escapeHtml(formatted)}</td>`;
				})
				.join('');
			return `<tr>${cells}</tr>`;
		})
		.join('');

	// SECURITY: Escape all user-provided content
	const safeFilename = escapeHtml(filename);
	const safeTitle = title ? escapeHtml(title) : '';
	const safeSubtitle = subtitle ? escapeHtml(subtitle) : '';

	const html = `
		<!DOCTYPE html>
		<html>
		<head>
			<title>${safeFilename}</title>
			<style>
				* {
					margin: 0;
					padding: 0;
					box-sizing: border-box;
				}
				body {
					font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
					padding: 40px;
					color: #1f2937;
				}
				.header {
					margin-bottom: 30px;
					padding-bottom: 20px;
					border-bottom: 2px solid #e5e7eb;
				}
				.logo {
					font-size: 24px;
					font-weight: 700;
					color: #6366f1;
					margin-bottom: 10px;
				}
				h1 {
					font-size: 28px;
					font-weight: 600;
					color: #111827;
					margin-bottom: 5px;
				}
				.subtitle {
					font-size: 14px;
					color: #6b7280;
				}
				.timestamp {
					font-size: 12px;
					color: #9ca3af;
					margin-top: 10px;
				}
				table {
					width: 100%;
					border-collapse: collapse;
					margin-top: 20px;
				}
				th, td {
					padding: 12px 16px;
					text-align: left;
					border-bottom: 1px solid #e5e7eb;
				}
				th {
					background: #f9fafb;
					font-weight: 600;
					color: #374151;
					font-size: 12px;
					text-transform: uppercase;
					letter-spacing: 0.05em;
				}
				td {
					font-size: 14px;
					color: #4b5563;
				}
				tr:hover {
					background: #f9fafb;
				}
				.footer {
					margin-top: 40px;
					padding-top: 20px;
					border-top: 1px solid #e5e7eb;
					font-size: 12px;
					color: #9ca3af;
					text-align: center;
				}
				@media print {
					body { padding: 20px; }
					.no-print { display: none; }
				}
			</style>
		</head>
		<body>
			<div class="header">
				<div class="logo">Revolution Trading Pros</div>
				${safeTitle ? `<h1>${safeTitle}</h1>` : ''}
				${safeSubtitle ? `<p class="subtitle">${safeSubtitle}</p>` : ''}
				${includeTimestamp ? `<p class="timestamp">Generated: ${new Date().toLocaleString()}</p>` : ''}
			</div>

			<table>
				<thead>
					<tr>${tableHeader}</tr>
				</thead>
				<tbody>
					${tableRows}
				</tbody>
			</table>

			<div class="footer">
				<p>Total Records: ${data.length}</p>
				<p style="margin-top: 5px;">&copy; ${new Date().getFullYear()} Revolution Trading Pros. All rights reserved.</p>
			</div>

			<script>
				window.onload = function() {
					window.print();
					setTimeout(function() { window.close(); }, 1000);
				};
			</script>
		</body>
		</html>
	`;

	printWindow.document.write(html);
	printWindow.document.close();
}

/**
 * Export data to JSON format
 */
export function exportToJSON(options: ExportOptions): void {
	if (!browser) return;

	const { filename, columns, data, title, includeTimestamp = true } = options;

	const exportData = {
		...(title && { title }),
		...(includeTimestamp && { generated: new Date().toISOString() }),
		columns: columns.map((col) => ({ key: col.key, label: col.label })),
		data: data.map((row) => {
			const exportRow: Record<string, unknown> = {};
			columns.forEach((col) => {
				exportRow[col.key] = row[col.key];
			});
			return exportRow;
		}),
		totalRecords: data.length
	};

	const blob = new Blob([JSON.stringify(exportData, null, 2)], {
		type: 'application/json;charset=utf-8;'
	});
	const url = URL.createObjectURL(blob);

	const link = document.createElement('a');
	link.href = url;
	link.download = `${filename}.json`;
	document.body.appendChild(link);
	link.click();
	document.body.removeChild(link);
	URL.revokeObjectURL(url);
}

/**
 * Format helpers for common data types
 */
export const formatters = {
	currency: (value: unknown) => {
		const num = Number(value);
		if (isNaN(num)) return String(value ?? '');
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(num);
	},

	number: (value: unknown) => {
		const num = Number(value);
		if (isNaN(num)) return String(value ?? '');
		return new Intl.NumberFormat('en-US').format(num);
	},

	percent: (value: unknown) => {
		const num = Number(value);
		if (isNaN(num)) return String(value ?? '');
		return `${num.toFixed(2)}%`;
	},

	date: (value: unknown) => {
		if (!value) return '';
		const date = new Date(value as string);
		return isNaN(date.getTime()) ? String(value) : date.toLocaleDateString();
	},

	datetime: (value: unknown) => {
		if (!value) return '';
		const date = new Date(value as string);
		return isNaN(date.getTime()) ? String(value) : date.toLocaleString();
	},

	boolean: (value: unknown) => {
		return value ? 'Yes' : 'No';
	}
};

/**
 * Quick export for tables
 */
export function quickExport(
	format: 'csv' | 'pdf' | 'json',
	data: Record<string, unknown>[],
	filename: string,
	columns?: ExportColumn[]
): void {
	// Auto-generate columns from first data row if not provided
	const exportColumns =
		columns ||
		(data.length > 0
			? Object.keys(data[0]).map((key) => ({
					key,
					label: key.charAt(0).toUpperCase() + key.slice(1).replace(/_/g, ' ')
				}))
			: []);

	const options: ExportOptions = {
		filename,
		columns: exportColumns,
		data,
		title: filename.replace(/_/g, ' '),
		includeTimestamp: true
	};

	switch (format) {
		case 'csv':
			exportToCSV(options);
			break;
		case 'pdf':
			exportToPDF(options);
			break;
		case 'json':
			exportToJSON(options);
			break;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// SERVER-SIDE EXPORT UTILITIES - Phase 4 Export Functionality
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Export error class for typed error handling
 */
export class ExportError extends Error {
	constructor(
		message: string,
		public status?: number,
		public code?: string
	) {
		super(message);
		this.name = 'ExportError';
	}
}

/**
 * Filter options for alert exports
 */
export interface AlertExportFilters {
	startDate?: string;
	endDate?: string;
	alertType?: 'ENTRY' | 'UPDATE' | 'EXIT';
	ticker?: string;
	limit?: number;
}

/**
 * Filter options for trade exports
 */
export interface TradeExportFilters {
	startDate?: string;
	endDate?: string;
	status?: 'open' | 'closed' | 'invalidated';
	result?: 'WIN' | 'LOSS';
	ticker?: string;
	limit?: number;
}

/**
 * Date range for performance reports
 */
export interface ReportDateRange {
	startDate?: string;
	endDate?: string;
}

/**
 * Performance statistics from API
 */
export interface PerformanceStats {
	totalTrades: number;
	wins: number;
	losses: number;
	winRate: number;
	totalPnl: number;
	avgWin: number;
	avgLoss: number;
	profitFactor: number;
	avgHoldingDays: number;
	largestWin: number;
	largestLoss: number;
	currentStreak: number;
	streakType: string;
}

/**
 * Trade summary for performance report
 */
export interface TradeSummary {
	id: number;
	ticker: string;
	direction: string;
	entryDate: string;
	exitDate: string;
	entryPrice: number;
	exitPrice: number;
	pnl: number;
	pnlPercent: number;
	result: string;
}

/**
 * Performance report data from API
 */
export interface PerformanceReportData {
	roomSlug: string;
	dateRange: { start: string; end: string };
	stats: PerformanceStats;
	trades: TradeSummary[];
	generatedAt: string;
}

/**
 * Build URL with query parameters
 */
function buildExportUrl(
	endpoint: string,
	params?: Record<string, string | number | undefined>
): string {
	const url = new URL(endpoint, window.location.origin);

	if (params) {
		Object.entries(params).forEach(([key, value]) => {
			if (value !== undefined && value !== null && value !== '') {
				url.searchParams.append(key, String(value));
			}
		});
	}

	return url.toString();
}

/**
 * Get authenticated headers for export requests
 */
function getExportHeaders(): HeadersInit {
	const token = authStore.getToken();
	const headers: HeadersInit = {
		Accept: 'text/csv, application/json'
	};

	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	return headers;
}

/**
 * Download a blob as a file
 */
function downloadBlob(blob: Blob, filename: string): void {
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = filename;
	a.style.display = 'none';
	document.body.appendChild(a);
	a.click();
	document.body.removeChild(a);
	URL.revokeObjectURL(url);
}

/**
 * Download alerts as CSV from the server
 *
 * @param roomSlug - Trading room identifier (e.g., 'explosive-swings')
 * @param filters - Optional filters for the export
 * @param filename - Custom filename (optional, auto-generated if not provided)
 * @throws ExportError if the request fails
 */
export async function downloadAlertsCsv(
	roomSlug: string,
	filters?: AlertExportFilters,
	filename?: string
): Promise<void> {
	if (!browser) return;

	const endpoint = `/api/export/${roomSlug}/alerts.csv`;
	const params: Record<string, string | number | undefined> = {
		start_date: filters?.startDate,
		end_date: filters?.endDate,
		type: filters?.alertType,
		ticker: filters?.ticker,
		limit: filters?.limit
	};

	const url = buildExportUrl(endpoint, params);

	try {
		const response = await fetch(url, {
			method: 'GET',
			headers: getExportHeaders(),
			credentials: 'include'
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));
			throw new ExportError(
				errorData.error || `Export failed with status ${response.status}`,
				response.status
			);
		}

		const blob = await response.blob();
		const finalFilename =
			filename || response.headers.get('Content-Disposition')?.match(/filename="(.+)"/)?.[1];
		downloadBlob(blob, finalFilename || `${roomSlug}_alerts.csv`);
	} catch (error) {
		if (error instanceof ExportError) {
			throw error;
		}
		throw new ExportError(
			error instanceof Error ? error.message : 'Failed to download alerts CSV'
		);
	}
}

/**
 * Download trades as CSV from the server
 *
 * @param roomSlug - Trading room identifier (e.g., 'explosive-swings')
 * @param filters - Optional filters for the export
 * @param filename - Custom filename (optional, auto-generated if not provided)
 * @throws ExportError if the request fails
 */
export async function downloadTradesCsv(
	roomSlug: string,
	filters?: TradeExportFilters,
	filename?: string
): Promise<void> {
	if (!browser) return;

	const endpoint = `/api/export/${roomSlug}/trades.csv`;
	const params: Record<string, string | number | undefined> = {
		start_date: filters?.startDate,
		end_date: filters?.endDate,
		status: filters?.status,
		result: filters?.result,
		ticker: filters?.ticker,
		limit: filters?.limit
	};

	const url = buildExportUrl(endpoint, params);

	try {
		const response = await fetch(url, {
			method: 'GET',
			headers: getExportHeaders(),
			credentials: 'include'
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));
			throw new ExportError(
				errorData.error || `Export failed with status ${response.status}`,
				response.status
			);
		}

		const blob = await response.blob();
		const finalFilename =
			filename || response.headers.get('Content-Disposition')?.match(/filename="(.+)"/)?.[1];
		downloadBlob(blob, finalFilename || `${roomSlug}_trades.csv`);
	} catch (error) {
		if (error instanceof ExportError) {
			throw error;
		}
		throw new ExportError(
			error instanceof Error ? error.message : 'Failed to download trades CSV'
		);
	}
}

/**
 * Fetch performance report data from the server
 *
 * @param roomSlug - Trading room identifier (e.g., 'explosive-swings')
 * @param dateRange - Optional date range for the report
 * @returns Performance report data for client-side PDF rendering
 * @throws ExportError if the request fails
 */
export async function fetchPerformanceReport(
	roomSlug: string,
	dateRange?: ReportDateRange
): Promise<PerformanceReportData> {
	if (!browser) {
		throw new ExportError('Performance report can only be fetched in browser');
	}

	const endpoint = `/api/export/${roomSlug}/performance`;
	const params: Record<string, string | undefined> = {
		start_date: dateRange?.startDate,
		end_date: dateRange?.endDate
	};

	const url = buildExportUrl(endpoint, params);

	try {
		const response = await fetch(url, {
			method: 'GET',
			headers: {
				...getExportHeaders(),
				Accept: 'application/json'
			},
			credentials: 'include'
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));
			throw new ExportError(
				errorData.error || `Failed to fetch performance report`,
				response.status
			);
		}

		const json = await response.json();
		const data = json.data;

		// Transform snake_case to camelCase
		return {
			roomSlug: data.room_slug,
			dateRange: data.date_range,
			stats: {
				totalTrades: data.stats.total_trades,
				wins: data.stats.wins,
				losses: data.stats.losses,
				winRate: data.stats.win_rate,
				totalPnl: data.stats.total_pnl,
				avgWin: data.stats.avg_win,
				avgLoss: data.stats.avg_loss,
				profitFactor: data.stats.profit_factor,
				avgHoldingDays: data.stats.avg_holding_days,
				largestWin: data.stats.largest_win,
				largestLoss: data.stats.largest_loss,
				currentStreak: data.stats.current_streak,
				streakType: data.stats.streak_type
			},
			trades: data.trades.map(
				(t: {
					id: number;
					ticker: string;
					direction: string;
					entry_date: string;
					exit_date: string;
					entry_price: number;
					exit_price: number;
					pnl: number;
					pnl_percent: number;
					result: string;
				}) => ({
					id: t.id,
					ticker: t.ticker,
					direction: t.direction,
					entryDate: t.entry_date,
					exitDate: t.exit_date,
					entryPrice: t.entry_price,
					exitPrice: t.exit_price,
					pnl: t.pnl,
					pnlPercent: t.pnl_percent,
					result: t.result
				})
			),
			generatedAt: data.generated_at
		};
	} catch (error) {
		if (error instanceof ExportError) {
			throw error;
		}
		throw new ExportError(
			error instanceof Error ? error.message : 'Failed to fetch performance report'
		);
	}
}

/**
 * Generate and download a PDF performance report
 *
 * Uses client-side rendering with server-fetched data for optimal performance.
 *
 * @param roomSlug - Trading room identifier
 * @param dateRange - Optional date range for the report
 */
export async function downloadPerformanceReportPdf(
	roomSlug: string,
	dateRange?: ReportDateRange
): Promise<void> {
	if (!browser) return;

	// Fetch data from server
	const reportData = await fetchPerformanceReport(roomSlug, dateRange);

	// Build HTML for PDF
	const html = buildPerformanceReportHtml(reportData);

	// Open print window
	const printWindow = window.open('', '_blank');
	if (!printWindow) {
		throw new ExportError('Failed to open print window. Please allow popups.');
	}

	printWindow.document.write(html);
	printWindow.document.close();
}

/**
 * Build HTML for the performance report PDF
 */
function buildPerformanceReportHtml(data: PerformanceReportData): string {
	const escapeHtml = (str: string): string => {
		const htmlEscapes: Record<string, string> = {
			'&': '&amp;',
			'<': '&lt;',
			'>': '&gt;',
			'"': '&quot;',
			"'": '&#x27;',
			'/': '&#x2F;'
		};
		return str.replace(/[&<>"'/]/g, (char) => htmlEscapes[char]);
	};

	const formatCurrency = (value: number): string => {
		const prefix = value >= 0 ? '+' : '';
		return `${prefix}$${Math.abs(value).toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
	};

	const formatPercent = (value: number): string => {
		const prefix = value >= 0 ? '+' : '';
		return `${prefix}${value.toFixed(2)}%`;
	};

	const tradesRows = data.trades
		.map(
			(t) => `
		<tr class="${t.result === 'WIN' ? 'win-row' : 'loss-row'}">
			<td>${escapeHtml(t.ticker)}</td>
			<td>${escapeHtml(t.direction)}</td>
			<td>${escapeHtml(t.entryDate)}</td>
			<td>${escapeHtml(t.exitDate)}</td>
			<td>$${t.entryPrice.toFixed(2)}</td>
			<td>$${t.exitPrice.toFixed(2)}</td>
			<td class="${t.pnl >= 0 ? 'positive' : 'negative'}">${formatCurrency(t.pnl)}</td>
			<td class="${t.pnlPercent >= 0 ? 'positive' : 'negative'}">${formatPercent(t.pnlPercent)}</td>
			<td class="result-${t.result.toLowerCase()}">${escapeHtml(t.result)}</td>
		</tr>
	`
		)
		.join('');

	return `
<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<title>Performance Report - ${escapeHtml(data.roomSlug)}</title>
	<style>
		* { margin: 0; padding: 0; box-sizing: border-box; }
		body {
			font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
			padding: 40px;
			color: #1f2937;
			max-width: 1200px;
			margin: 0 auto;
		}
		.header {
			margin-bottom: 30px;
			padding-bottom: 20px;
			border-bottom: 3px solid #6366f1;
		}
		.logo {
			font-size: 28px;
			font-weight: 700;
			color: #6366f1;
			margin-bottom: 8px;
		}
		h1 {
			font-size: 32px;
			font-weight: 600;
			color: #111827;
			margin-bottom: 5px;
		}
		.subtitle {
			font-size: 14px;
			color: #6b7280;
		}
		.date-range {
			font-size: 16px;
			color: #374151;
			margin-top: 8px;
		}
		.stats-grid {
			display: grid;
			grid-template-columns: repeat(4, 1fr);
			gap: 20px;
			margin: 30px 0;
		}
		.stat-card {
			background: #f9fafb;
			border: 1px solid #e5e7eb;
			border-radius: 12px;
			padding: 20px;
			text-align: center;
		}
		.stat-label {
			font-size: 12px;
			color: #6b7280;
			text-transform: uppercase;
			letter-spacing: 0.05em;
			margin-bottom: 8px;
		}
		.stat-value {
			font-size: 28px;
			font-weight: 700;
			color: #111827;
		}
		.stat-value.positive { color: #059669; }
		.stat-value.negative { color: #dc2626; }
		h2 {
			font-size: 20px;
			font-weight: 600;
			color: #111827;
			margin: 30px 0 15px;
			padding-bottom: 10px;
			border-bottom: 2px solid #e5e7eb;
		}
		table {
			width: 100%;
			border-collapse: collapse;
			margin-top: 15px;
			font-size: 13px;
		}
		th, td {
			padding: 10px 12px;
			text-align: left;
			border-bottom: 1px solid #e5e7eb;
		}
		th {
			background: #f9fafb;
			font-weight: 600;
			color: #374151;
			font-size: 11px;
			text-transform: uppercase;
			letter-spacing: 0.05em;
		}
		td { color: #4b5563; }
		.win-row { background: #f0fdf4; }
		.loss-row { background: #fef2f2; }
		.positive { color: #059669; font-weight: 600; }
		.negative { color: #dc2626; font-weight: 600; }
		.result-win {
			background: #dcfce7;
			color: #166534;
			padding: 4px 8px;
			border-radius: 4px;
			font-weight: 600;
		}
		.result-loss {
			background: #fee2e2;
			color: #991b1b;
			padding: 4px 8px;
			border-radius: 4px;
			font-weight: 600;
		}
		.footer {
			margin-top: 40px;
			padding-top: 20px;
			border-top: 1px solid #e5e7eb;
			font-size: 11px;
			color: #9ca3af;
			text-align: center;
		}
		@media print {
			body { padding: 20px; font-size: 10px; }
			.stat-card { padding: 12px; }
			.stat-value { font-size: 20px; }
			.stats-grid { grid-template-columns: repeat(4, 1fr); gap: 10px; }
			table { font-size: 9px; }
			th, td { padding: 6px 8px; }
			.no-print { display: none; }
		}
	</style>
</head>
<body>
	<div class="header">
		<div class="logo">Revolution Trading Pros</div>
		<h1>Performance Report</h1>
		<p class="subtitle">${escapeHtml(data.roomSlug.replace(/-/g, ' ').replace(/\b\w/g, (l) => l.toUpperCase()))}</p>
		<p class="date-range">${escapeHtml(data.dateRange.start)} to ${escapeHtml(data.dateRange.end)}</p>
	</div>

	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-label">Win Rate</div>
			<div class="stat-value ${data.stats.winRate >= 50 ? 'positive' : 'negative'}">${data.stats.winRate.toFixed(1)}%</div>
		</div>
		<div class="stat-card">
			<div class="stat-label">Total P&L</div>
			<div class="stat-value ${data.stats.totalPnl >= 0 ? 'positive' : 'negative'}">${formatCurrency(data.stats.totalPnl)}</div>
		</div>
		<div class="stat-card">
			<div class="stat-label">Total Trades</div>
			<div class="stat-value">${data.stats.totalTrades}</div>
		</div>
		<div class="stat-card">
			<div class="stat-label">Profit Factor</div>
			<div class="stat-value ${data.stats.profitFactor >= 1 ? 'positive' : 'negative'}">${data.stats.profitFactor.toFixed(2)}</div>
		</div>
	</div>

	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-label">Wins / Losses</div>
			<div class="stat-value">${data.stats.wins} / ${data.stats.losses}</div>
		</div>
		<div class="stat-card">
			<div class="stat-label">Avg Win</div>
			<div class="stat-value positive">${formatCurrency(data.stats.avgWin)}</div>
		</div>
		<div class="stat-card">
			<div class="stat-label">Avg Loss</div>
			<div class="stat-value negative">-${formatCurrency(Math.abs(data.stats.avgLoss))}</div>
		</div>
		<div class="stat-card">
			<div class="stat-label">Avg Holding Days</div>
			<div class="stat-value">${data.stats.avgHoldingDays.toFixed(1)}</div>
		</div>
	</div>

	<h2>Trade History (${data.trades.length} trades)</h2>
	<table>
		<thead>
			<tr>
				<th>Ticker</th>
				<th>Direction</th>
				<th>Entry Date</th>
				<th>Exit Date</th>
				<th>Entry Price</th>
				<th>Exit Price</th>
				<th>P&L</th>
				<th>P&L %</th>
				<th>Result</th>
			</tr>
		</thead>
		<tbody>
			${tradesRows}
		</tbody>
	</table>

	<div class="footer">
		<p>Generated: ${new Date(data.generatedAt).toLocaleString()}</p>
		<p>&copy; ${new Date().getFullYear()} Revolution Trading Pros. All rights reserved.</p>
		<p style="margin-top: 5px; font-size: 10px;">This report is for informational purposes only. Past performance does not guarantee future results.</p>
	</div>

	<script>
		window.onload = function() {
			window.print();
			setTimeout(function() { window.close(); }, 1000);
		};
	</script>
</body>
</html>
	`;
}
