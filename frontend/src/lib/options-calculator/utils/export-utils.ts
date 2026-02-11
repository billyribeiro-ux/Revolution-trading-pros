// ============================================================
// EXPORT UTILITIES — PNG, CSV Generation
// Branded screenshot engine + data export helpers.
// ============================================================

import type {
	BSInputs,
	OptionType,
	AllGreeks,
	ProbabilityResult,
	ScreenshotConfig
} from '../engine/types.js';
import { formatCurrency, formatGreek } from './formatters.js';

/** Default screenshot configuration */
export const DEFAULT_SCREENSHOT_CONFIG: ScreenshotConfig = {
	zone: 'results-chart',
	aspectRatio: 'auto',
	filename: 'options-analysis',
	scale: 2,
	showLogo: true,
	showInfoBar: true,
	showFrame: true
};

// ────────────────────────────────────────────────────────────
// PNG SCREENSHOT ENGINE
// ────────────────────────────────────────────────────────────

/**
 * Capture a DOM element as a branded PNG screenshot.
 *
 * Composites a professional frame around the captured content:
 * - Top info bar: ticker, price summary, date/time
 * - Bottom footer: "Revolution Trading Pros" + URL
 * - Optional gradient border frame (indigo → teal)
 * - Aspect ratio cropping for social media
 * - 2x/3x resolution for crisp sharing
 *
 * Returns the Blob (also triggers download automatically).
 * Returns null on failure.
 */
export async function captureScreenshot(
	element: HTMLElement,
	config: Partial<ScreenshotConfig> = {}
): Promise<Blob | null> {
	const opts: ScreenshotConfig = { ...DEFAULT_SCREENSHOT_CONFIG, ...config };
	const scale = opts.scale ?? 2;
	const isDark = opts.theme !== 'light';
	const bgColor = opts.backgroundColor ?? (isDark ? '#0a0a0f' : '#f4f5f9');

	try {
		const html2canvas = (await import('html2canvas')).default;

		// Step 1: Capture the raw element
		const rawCanvas = await html2canvas(element, {
			scale,
			backgroundColor: bgColor,
			useCORS: true,
			logging: false,
			windowWidth: element.scrollWidth,
			windowHeight: element.scrollHeight
		});

		// Step 2: Calculate branded frame dimensions
		const pad = opts.showFrame ? 16 * scale : 0;
		const infoH = opts.showInfoBar ? 48 * scale : 0;
		const footH = opts.showLogo ? 56 * scale : 0;
		const bw = opts.showFrame ? 2 * scale : 0;

		const totalW = rawCanvas.width + pad * 2 + bw * 2;
		const totalH = rawCanvas.height + pad * 2 + bw * 2 + infoH + footH;

		// Step 3: Apply aspect ratio
		let finalW = totalW;
		let finalH = totalH;

		if (opts.aspectRatio === '16:9') {
			const ratio = 16 / 9;
			if (totalW / totalH > ratio) {
				finalH = totalH;
				finalW = Math.round(totalH * ratio);
			} else {
				finalW = totalW;
				finalH = Math.round(totalW / ratio);
			}
		} else if (opts.aspectRatio === '4:5') {
			finalW = totalW;
			finalH = Math.round(totalW / (4 / 5));
		} else if (opts.aspectRatio === '1:1') {
			const maxDim = Math.max(totalW, totalH);
			finalW = maxDim;
			finalH = maxDim;
		}

		// Step 4: Create final branded canvas
		const canvas = document.createElement('canvas');
		canvas.width = finalW;
		canvas.height = finalH;
		const ctx = canvas.getContext('2d');
		if (!ctx) return null;

		// Background
		ctx.fillStyle = bgColor;
		ctx.fillRect(0, 0, finalW, finalH);

		// Gradient border frame
		if (opts.showFrame) {
			const grad = ctx.createLinearGradient(0, 0, finalW, finalH);
			grad.addColorStop(0, '#6366f1');
			grad.addColorStop(0.5, '#00d4aa');
			grad.addColorStop(1, '#6366f1');
			ctx.strokeStyle = grad;
			ctx.lineWidth = bw;
			ctx.beginPath();
			ctx.roundRect(bw / 2, bw / 2, finalW - bw, finalH - bw, 12 * scale);
			ctx.stroke();
		}

		// Info bar (top)
		if (opts.showInfoBar) {
			const barY = bw + pad;
			const barX = bw + pad;
			const barW = finalW - (bw + pad) * 2;

			ctx.fillStyle = isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.03)';
			ctx.beginPath();
			ctx.roundRect(barX, barY, barW, infoH - 8 * scale, 8 * scale);
			ctx.fill();

			const textCol = isDark ? '#e8e8f0' : '#111827';
			const mutedCol = isDark ? '#8888aa' : '#6b7280';

			if (opts.ticker) {
				ctx.font = `bold ${16 * scale}px "Plus Jakarta Sans","Inter",system-ui,sans-serif`;
				ctx.fillStyle = textCol;
				ctx.textAlign = 'left';
				ctx.textBaseline = 'middle';
				ctx.fillText(opts.ticker, barX + 16 * scale, barY + infoH / 2 - 4 * scale);

				if (opts.summaryText) {
					const tw = ctx.measureText(opts.ticker).width;
					ctx.font = `${13 * scale}px "JetBrains Mono","Fira Code",monospace`;
					ctx.fillStyle = mutedCol;
					ctx.fillText(
						`  |  ${opts.summaryText}`,
						barX + 16 * scale + tw,
						barY + infoH / 2 - 4 * scale
					);
				}
			}

			// Date/time (right side)
			const now = new Date();
			const dateStr = now.toLocaleDateString('en-US', {
				month: 'short',
				day: 'numeric',
				year: 'numeric'
			});
			const timeStr = now.toLocaleTimeString('en-US', {
				hour: 'numeric',
				minute: '2-digit',
				timeZoneName: 'short'
			});
			ctx.font = `${12 * scale}px "Inter",system-ui,sans-serif`;
			ctx.fillStyle = mutedCol;
			ctx.textAlign = 'right';
			ctx.fillText(
				`${dateStr} \u2022 ${timeStr}`,
				barX + barW - 16 * scale,
				barY + infoH / 2 - 4 * scale
			);
		}

		// Calculator content
		const contentX = bw + pad;
		const contentY = bw + pad + infoH;
		ctx.drawImage(rawCanvas, contentX, contentY);

		// Footer
		if (opts.showLogo) {
			const footY = contentY + rawCanvas.height + 8 * scale;
			const footX = bw + pad;
			const footW = finalW - (bw + pad) * 2;

			ctx.fillStyle = isDark ? 'rgba(255,255,255,0.03)' : 'rgba(0,0,0,0.02)';
			ctx.beginPath();
			ctx.roundRect(footX, footY, footW, footH - 16 * scale, 8 * scale);
			ctx.fill();

			// Try loading logo image, fall back to text
			let logoLoaded = false;
			try {
				const logoImg = new Image();
				logoImg.crossOrigin = 'anonymous';
				await new Promise<void>((resolve, reject) => {
					logoImg.onload = () => resolve();
					logoImg.onerror = () => reject(new Error('Logo not found'));
					logoImg.src = '/images/rtp-logo.png';
					setTimeout(() => reject(new Error('Logo timeout')), 2000);
				});

				const logoH = (footH - 16 * scale) * 0.6;
				const logoW = logoH * (logoImg.naturalWidth / logoImg.naturalHeight);
				const logoX = footX + 20 * scale;
				const logoY = footY + (footH - 16 * scale - logoH) / 2;
				ctx.drawImage(logoImg, logoX, logoY, logoW, logoH);
				logoLoaded = true;

				ctx.font = `${12 * scale}px "Inter",system-ui,sans-serif`;
				ctx.fillStyle = isDark ? '#8888aa' : '#6b7280';
				ctx.textAlign = 'left';
				ctx.textBaseline = 'middle';
				ctx.fillText(
					'Revolution Trading Pros',
					logoX + logoW + 12 * scale,
					footY + (footH - 16 * scale) / 2
				);
			} catch {
				// Logo not available — text fallback
			}

			if (!logoLoaded) {
				ctx.font = `bold ${14 * scale}px "Plus Jakarta Sans","Inter",system-ui,sans-serif`;
				ctx.fillStyle = isDark ? '#e8e8f0' : '#111827';
				ctx.textAlign = 'left';
				ctx.textBaseline = 'middle';
				ctx.fillText(
					'Revolution Trading Pros',
					footX + 20 * scale,
					footY + (footH - 16 * scale) / 2
				);
			}

			// URL on the right
			ctx.font = `${11 * scale}px "Inter",system-ui,sans-serif`;
			ctx.fillStyle = isDark ? '#6366f1' : '#4f46e5';
			ctx.textAlign = 'right';
			ctx.textBaseline = 'middle';
			ctx.fillText(
				'revolutiontradingpros.com',
				footX + footW - 20 * scale,
				footY + (footH - 16 * scale) / 2
			);
		}

		// Step 5: Export as blob
		const filename = opts.filename ?? 'options-analysis';
		const tickerSuffix = opts.ticker ? `-${opts.ticker}` : '';
		const fullFilename = `${filename}${tickerSuffix}-${Date.now()}`;

		return new Promise((resolve) => {
			canvas.toBlob(
				(blob) => {
					if (blob) {
						const url = URL.createObjectURL(blob);
						const a = document.createElement('a');
						a.href = url;
						a.download = `${fullFilename}.png`;
						a.click();
						URL.revokeObjectURL(url);
					}
					resolve(blob);
				},
				'image/png',
				1.0
			);
		});
	} catch (error) {
		console.error('[ExportUtils] PNG capture failed:', error);
		return null;
	}
}

// ────────────────────────────────────────────────────────────
// CSV EXPORT
// ────────────────────────────────────────────────────────────

/**
 * Generate CSV content for Greeks across a range of strikes.
 * Includes metadata header with input parameters.
 */
export function generateGreeksCSV(
	inputs: BSInputs,
	type: OptionType,
	greeksData: { strike: number; greeks: AllGreeks }[]
): string {
	const headers = [
		'Strike',
		'Type',
		'Delta',
		'Gamma',
		'Theta (daily)',
		'Vega (per 1%)',
		'Rho (per 1%)',
		'Charm',
		'Vanna',
		'Volga',
		'Speed',
		'Color',
		'Zomma'
	];

	const rows = greeksData.map((row) => [
		row.strike.toFixed(2),
		type,
		row.greeks.first.delta.toFixed(6),
		row.greeks.first.gamma.toFixed(8),
		row.greeks.first.theta.toFixed(6),
		row.greeks.first.vega.toFixed(6),
		row.greeks.first.rho.toFixed(6),
		row.greeks.second.charm.toFixed(8),
		row.greeks.second.vanna.toFixed(8),
		row.greeks.second.volga.toFixed(6),
		row.greeks.second.speed.toFixed(10),
		row.greeks.second.color.toFixed(10),
		row.greeks.second.zomma.toFixed(8)
	]);

	const metadata = [
		`# Black-Scholes Options Calculator \u2014 Revolution Trading Pros`,
		`# Generated: ${new Date().toISOString()}`,
		`# Spot: ${inputs.spotPrice}, Strike: ${inputs.strikePrice}, Vol: ${(inputs.volatility * 100).toFixed(1)}%, Time: ${inputs.timeToExpiry.toFixed(4)}yr, Rate: ${(inputs.riskFreeRate * 100).toFixed(2)}%, Div: ${(inputs.dividendYield * 100).toFixed(2)}%`,
		``
	];

	return [...metadata, headers.join(','), ...rows.map((r) => r.join(','))].join('\n');
}

/**
 * Trigger a CSV file download in the browser.
 */
export function downloadCSV(content: string, filename: string = 'greeks-export'): void {
	const blob = new Blob([content], { type: 'text/csv;charset=utf-8;' });
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = `${filename}.csv`;
	a.click();
	URL.revokeObjectURL(url);
}

// ────────────────────────────────────────────────────────────
// TEXT ANALYSIS SUMMARY (for PDF / clipboard)
// ────────────────────────────────────────────────────────────

/**
 * Generate a comprehensive plain-text analysis summary.
 * Suitable for PDF export or clipboard copy.
 */
export function generateAnalysisSummary(
	inputs: BSInputs,
	type: OptionType,
	optionPrice: number,
	greeks: AllGreeks,
	probabilities: ProbabilityResult
): string {
	return `BLACK-SCHOLES OPTIONS ANALYSIS
Generated by Revolution Trading Pros
Date: ${new Date().toLocaleDateString('en-US', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' })}

\u2550\u2550\u2550 INPUT PARAMETERS \u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550
Spot Price (S):          ${formatCurrency(inputs.spotPrice)}
Strike Price (K):        ${formatCurrency(inputs.strikePrice)}
Implied Volatility:      ${(inputs.volatility * 100).toFixed(1)}%
Time to Expiry (T):      ${inputs.timeToExpiry.toFixed(4)} years (${Math.round(inputs.timeToExpiry * 365)} days)
Risk-Free Rate (r):      ${(inputs.riskFreeRate * 100).toFixed(2)}%
Dividend Yield (q):      ${(inputs.dividendYield * 100).toFixed(2)}%

\u2550\u2550\u2550 PRICING RESULT \u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550
Option Type:             ${type.toUpperCase()}
Option Price:            ${formatCurrency(optionPrice)}

\u2550\u2550\u2550 FIRST-ORDER GREEKS \u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550
Delta (\u0394):    ${formatGreek(greeks.first.delta, 'delta')}
Gamma (\u0393):    ${formatGreek(greeks.first.gamma, 'gamma')}
Theta (\u0398):    ${formatGreek(greeks.first.theta, 'theta')} per day
Vega (\u03bd):     ${formatGreek(greeks.first.vega, 'vega')} per 1% IV
Rho (\u03c1):      ${formatGreek(greeks.first.rho, 'rho')} per 1% rate

\u2550\u2550\u2550 SECOND-ORDER GREEKS \u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550
Charm:    ${formatGreek(greeks.second.charm, 'charm')}
Vanna:    ${formatGreek(greeks.second.vanna, 'vanna')}
Volga:    ${formatGreek(greeks.second.volga, 'volga')}
Speed:    ${formatGreek(greeks.second.speed, 'speed')}
Color:    ${formatGreek(greeks.second.color, 'color')}
Zomma:    ${formatGreek(greeks.second.zomma, 'zomma')}

\u2550\u2550\u2550 PROBABILITY ANALYSIS \u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550\u2550
Probability ITM:         ${(probabilities.probabilityITM * 100).toFixed(1)}%
Probability OTM:         ${(probabilities.probabilityOTM * 100).toFixed(1)}%
Prob. of Touching:       ${(probabilities.probabilityOfTouching * 100).toFixed(1)}%
Expected Move:           \u00b1${formatCurrency(probabilities.expectedMove)} (${probabilities.expectedMovePercent.toFixed(1)}%)
1 SD Range:              ${formatCurrency(probabilities.oneSDRange[0])} \u2014 ${formatCurrency(probabilities.oneSDRange[1])}
2 SD Range:              ${formatCurrency(probabilities.twoSDRange[0])} \u2014 ${formatCurrency(probabilities.twoSDRange[1])}

Powered by Revolution Trading Pros
https://revolutiontradingpros.com`;
}
