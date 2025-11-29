/**
 * Consent Receipt/Proof System
 *
 * Generates downloadable consent receipts for:
 * - Legal compliance (GDPR Article 7)
 * - User transparency
 * - Audit documentation
 *
 * @module consent/consent-receipt
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState, ConsentAuditEntry } from './types';
import { getAuditLog } from './audit-log';

/**
 * Consent receipt data structure
 */
export interface ConsentReceipt {
	receiptId: string;
	generatedAt: string;
	websiteUrl: string;
	websiteName: string;

	// Consent details
	consentId: string;
	consentDate: string;
	consentMethod: string;
	expiresAt?: string;

	// Categories
	categories: {
		necessary: boolean;
		analytics: boolean;
		marketing: boolean;
		preferences: boolean;
	};

	// Privacy signals
	privacySignals?: {
		gpc: boolean;
		dnt: boolean;
		region?: string;
	};

	// Audit trail
	auditTrail: ConsentAuditEntry[];

	// Technical details
	userAgent: string;
	ipHash?: string;
	schemaVersion: string;

	// Verification
	checksum: string;
}

/**
 * Generate a unique receipt ID
 */
function generateReceiptId(): string {
	const timestamp = Date.now().toString(36);
	const random = Math.random().toString(36).substring(2, 10);
	return `RCP-${timestamp}-${random}`.toUpperCase();
}

/**
 * Calculate a simple checksum for verification
 */
function calculateChecksum(data: string): string {
	let hash = 0;
	for (let i = 0; i < data.length; i++) {
		const char = data.charCodeAt(i);
		hash = (hash << 5) - hash + char;
		hash = hash & hash; // Convert to 32-bit integer
	}
	return Math.abs(hash).toString(16).padStart(8, '0');
}

/**
 * Generate a consent receipt from current state
 */
export function generateConsentReceipt(
	consent: ConsentState,
	websiteName: string = 'Revolution Trading Pros'
): ConsentReceipt {
	const receiptData = {
		receiptId: generateReceiptId(),
		generatedAt: new Date().toISOString(),
		websiteUrl: browser ? window.location.origin : '',
		websiteName,

		consentId: consent.consentId || 'N/A',
		consentDate: consent.updatedAt,
		consentMethod: consent.consentMethod || 'unknown',
		expiresAt: consent.expiresAt,

		categories: {
			necessary: consent.necessary,
			analytics: consent.analytics,
			marketing: consent.marketing,
			preferences: consent.preferences,
		},

		privacySignals: consent.privacySignals
			? {
					gpc: consent.privacySignals.gpc,
					dnt: consent.privacySignals.dnt,
					region: consent.privacySignals.region,
				}
			: undefined,

		auditTrail: getAuditLog().slice(-10), // Last 10 entries

		userAgent: browser ? navigator.userAgent : '',
		schemaVersion: '1.0.0',
		checksum: '', // Will be filled below
	};

	// Calculate checksum
	const dataForChecksum = JSON.stringify({
		consentId: receiptData.consentId,
		consentDate: receiptData.consentDate,
		categories: receiptData.categories,
	});
	receiptData.checksum = calculateChecksum(dataForChecksum);

	return receiptData;
}

/**
 * Export consent receipt as JSON
 */
export function exportReceiptAsJSON(receipt: ConsentReceipt): string {
	return JSON.stringify(receipt, null, 2);
}

/**
 * Download consent receipt as JSON file
 */
export function downloadReceiptAsJSON(receipt: ConsentReceipt): void {
	if (!browser) return;

	const json = exportReceiptAsJSON(receipt);
	const blob = new Blob([json], { type: 'application/json' });
	const url = URL.createObjectURL(blob);

	const a = document.createElement('a');
	a.href = url;
	a.download = `consent-receipt-${receipt.receiptId}.json`;
	document.body.appendChild(a);
	a.click();
	document.body.removeChild(a);

	URL.revokeObjectURL(url);
}

/**
 * Generate HTML receipt for printing/PDF
 */
export function generateReceiptHTML(receipt: ConsentReceipt): string {
	const categoryList = Object.entries(receipt.categories)
		.map(
			([cat, enabled]) => `
      <tr>
        <td style="padding: 8px; border-bottom: 1px solid #eee; text-transform: capitalize;">${cat}</td>
        <td style="padding: 8px; border-bottom: 1px solid #eee;">
          <span style="color: ${enabled ? '#22c55e' : '#ef4444'}; font-weight: bold;">
            ${enabled ? '✓ Enabled' : '✗ Disabled'}
          </span>
        </td>
      </tr>
    `
		)
		.join('');

	const auditList = receipt.auditTrail
		.map(
			(entry) => `
      <tr>
        <td style="padding: 6px; font-size: 12px;">${new Date(entry.timestamp).toLocaleString()}</td>
        <td style="padding: 6px; font-size: 12px;">${entry.action}</td>
      </tr>
    `
		)
		.join('');

	return `
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Consent Receipt - ${receipt.receiptId}</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      max-width: 800px;
      margin: 0 auto;
      padding: 40px 20px;
      color: #333;
    }
    .header {
      text-align: center;
      border-bottom: 2px solid #0ea5e9;
      padding-bottom: 20px;
      margin-bottom: 30px;
    }
    .header h1 { margin: 0; color: #0ea5e9; }
    .header p { color: #666; margin: 5px 0; }
    .section { margin-bottom: 25px; }
    .section h2 {
      font-size: 16px;
      color: #0ea5e9;
      border-bottom: 1px solid #eee;
      padding-bottom: 8px;
    }
    table { width: 100%; border-collapse: collapse; }
    .info-row td { padding: 8px 0; }
    .info-row td:first-child { font-weight: 500; width: 180px; }
    .footer {
      margin-top: 40px;
      padding-top: 20px;
      border-top: 1px solid #eee;
      font-size: 12px;
      color: #666;
      text-align: center;
    }
    .checksum {
      font-family: monospace;
      background: #f5f5f5;
      padding: 10px;
      border-radius: 4px;
      text-align: center;
    }
    @media print {
      body { padding: 20px; }
    }
  </style>
</head>
<body>
  <div class="header">
    <h1>🔒 Consent Receipt</h1>
    <p>${receipt.websiteName}</p>
    <p style="font-size: 12px;">Receipt ID: ${receipt.receiptId}</p>
  </div>

  <div class="section">
    <h2>Consent Details</h2>
    <table class="info-row">
      <tr>
        <td>Consent ID:</td>
        <td><code>${receipt.consentId}</code></td>
      </tr>
      <tr>
        <td>Date Given:</td>
        <td>${new Date(receipt.consentDate).toLocaleString()}</td>
      </tr>
      <tr>
        <td>Method:</td>
        <td style="text-transform: capitalize;">${receipt.consentMethod}</td>
      </tr>
      <tr>
        <td>Expires:</td>
        <td>${receipt.expiresAt ? new Date(receipt.expiresAt).toLocaleString() : 'N/A'}</td>
      </tr>
    </table>
  </div>

  <div class="section">
    <h2>Cookie Categories</h2>
    <table>
      ${categoryList}
    </table>
  </div>

  ${
		receipt.privacySignals
			? `
  <div class="section">
    <h2>Privacy Signals Detected</h2>
    <table class="info-row">
      <tr>
        <td>Global Privacy Control:</td>
        <td>${receipt.privacySignals.gpc ? 'Yes (Respected)' : 'No'}</td>
      </tr>
      <tr>
        <td>Do Not Track:</td>
        <td>${receipt.privacySignals.dnt ? 'Yes (Respected)' : 'No'}</td>
      </tr>
      <tr>
        <td>Detected Region:</td>
        <td>${receipt.privacySignals.region || 'Unknown'}</td>
      </tr>
    </table>
  </div>
  `
			: ''
	}

  ${
		receipt.auditTrail.length > 0
			? `
  <div class="section">
    <h2>Consent History</h2>
    <table>
      <thead>
        <tr>
          <th style="text-align: left; padding: 6px;">Date</th>
          <th style="text-align: left; padding: 6px;">Action</th>
        </tr>
      </thead>
      <tbody>
        ${auditList}
      </tbody>
    </table>
  </div>
  `
			: ''
	}

  <div class="section">
    <h2>Verification</h2>
    <div class="checksum">
      Checksum: <strong>${receipt.checksum}</strong>
    </div>
    <p style="font-size: 12px; color: #666; text-align: center; margin-top: 10px;">
      This checksum can be used to verify the integrity of this receipt.
    </p>
  </div>

  <div class="footer">
    <p>Generated on ${new Date(receipt.generatedAt).toLocaleString()}</p>
    <p>${receipt.websiteUrl}</p>
    <p>Schema Version: ${receipt.schemaVersion}</p>
  </div>
</body>
</html>
  `.trim();
}

/**
 * Download receipt as HTML (for printing to PDF)
 */
export function downloadReceiptAsHTML(receipt: ConsentReceipt): void {
	if (!browser) return;

	const html = generateReceiptHTML(receipt);
	const blob = new Blob([html], { type: 'text/html' });
	const url = URL.createObjectURL(blob);

	const a = document.createElement('a');
	a.href = url;
	a.download = `consent-receipt-${receipt.receiptId}.html`;
	document.body.appendChild(a);
	a.click();
	document.body.removeChild(a);

	URL.revokeObjectURL(url);
}

/**
 * Open receipt in new window for printing
 */
export function printReceipt(receipt: ConsentReceipt): void {
	if (!browser) return;

	const html = generateReceiptHTML(receipt);
	const printWindow = window.open('', '_blank');

	if (printWindow) {
		printWindow.document.write(html);
		printWindow.document.close();
		printWindow.focus();

		// Wait for content to load, then print
		setTimeout(() => {
			printWindow.print();
		}, 250);
	}
}

/**
 * Verify receipt checksum
 */
export function verifyReceiptChecksum(receipt: ConsentReceipt): boolean {
	const dataForChecksum = JSON.stringify({
		consentId: receipt.consentId,
		consentDate: receipt.consentDate,
		categories: receipt.categories,
	});

	const calculatedChecksum = calculateChecksum(dataForChecksum);
	return calculatedChecksum === receipt.checksum;
}
