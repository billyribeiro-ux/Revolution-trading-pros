/**
 * Revolution Trading Pros - Enterprise Export Utilities
 * =====================================================
 * CSV and PDF export functionality for data tables and reports.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */

import { browser } from '$app/environment';

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

	// Build HTML table
	const tableHeader = columns.map((col) => `<th>${col.label}</th>`).join('');

	const tableRows = data
		.map((row) => {
			const cells = columns
				.map((col) => {
					const value = row[col.key];
					const formatted = col.format ? col.format(value) : String(value ?? '');
					return `<td>${formatted}</td>`;
				})
				.join('');
			return `<tr>${cells}</tr>`;
		})
		.join('');

	const html = `
		<!DOCTYPE html>
		<html>
		<head>
			<title>${filename}</title>
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
				${title ? `<h1>${title}</h1>` : ''}
				${subtitle ? `<p class="subtitle">${subtitle}</p>` : ''}
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
