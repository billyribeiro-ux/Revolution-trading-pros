// ============================================================
// MOCK DATA ADAPTER — Development & Fallback
// Generates realistic market data without any API calls.
// ============================================================

import type { MarketDataProvider } from '../provider-interface.js';
import type {
	DataProviderName,
	ProviderCapabilities,
	ProviderStatus,
	StockQuote,
	OptionsChain,
	OptionQuote,
	ExpirationList,
	TickerSearchResult,
	RiskFreeRateData,
	DividendInfo,
	EarningsInfo,
	HistoricalVolData,
	OHLCV,
} from '../types.js';
import { price as bsPrice } from '../../engine/black-scholes.js';
import { firstOrderGreeks } from '../../engine/greeks.js';
import type { BSInputs } from '../../engine/types.js';

const MOCK_STOCKS: Record<string, { name: string; price: number; div: number; type: 'stock' | 'etf' }> = {
	AAPL: { name: 'Apple Inc.', price: 237.42, div: 0.0044, type: 'stock' },
	MSFT: { name: 'Microsoft Corporation', price: 441.28, div: 0.0072, type: 'stock' },
	GOOGL: { name: 'Alphabet Inc.', price: 192.15, div: 0.0, type: 'stock' },
	AMZN: { name: 'Amazon.com Inc.', price: 224.89, div: 0.0, type: 'stock' },
	TSLA: { name: 'Tesla Inc.', price: 352.76, div: 0.0, type: 'stock' },
	NVDA: { name: 'NVIDIA Corporation', price: 138.52, div: 0.0004, type: 'stock' },
	META: { name: 'Meta Platforms Inc.', price: 612.34, div: 0.0, type: 'stock' },
	SPY: { name: 'SPDR S&P 500 ETF Trust', price: 602.18, div: 0.0126, type: 'etf' },
	QQQ: { name: 'Invesco QQQ Trust', price: 527.45, div: 0.0055, type: 'etf' },
	IWM: { name: 'iShares Russell 2000 ETF', price: 224.67, div: 0.0112, type: 'etf' },
	JPM: { name: 'JPMorgan Chase & Co.', price: 258.93, div: 0.0198, type: 'stock' },
	AMD: { name: 'Advanced Micro Devices', price: 164.21, div: 0.0, type: 'stock' },
	NFLX: { name: 'Netflix Inc.', price: 982.15, div: 0.0, type: 'stock' },
	DIS: { name: 'The Walt Disney Company', price: 112.45, div: 0.0, type: 'stock' },
	BA: { name: 'Boeing Company', price: 178.32, div: 0.0, type: 'stock' },
};

function addNoise(base: number, pct: number = 0.002): number {
	return base * (1 + (Math.random() - 0.5) * 2 * pct);
}

function generateExpirations(count: number = 12): string[] {
	const expirations: string[] = [];
	const today = new Date();
	const current = new Date(today);

	current.setDate(current.getDate() + ((5 - current.getDay() + 7) % 7 || 7));

	for (let i = 0; i < count; i++) {
		expirations.push(current.toISOString().split('T')[0]);
		current.setDate(current.getDate() + 7);
	}

	for (let m = 1; m <= 6; m++) {
		const date = new Date(today.getFullYear(), today.getMonth() + m, 1);
		const firstDay = date.getDay();
		const firstFriday = firstDay <= 5 ? 5 - firstDay : 12 - firstDay;
		date.setDate(firstFriday + 14 + 1);
		const dateStr = date.toISOString().split('T')[0];
		if (!expirations.includes(dateStr)) {
			expirations.push(dateStr);
		}
	}

	return expirations.sort();
}

function generateMockChain(
	ticker: string,
	spotPrice: number,
	expiration: string,
	divYield: number,
): OptionsChain {
	const today = new Date();
	const expiryDate = new Date(expiration);
	const daysToExpiry = Math.max(1, Math.round((expiryDate.getTime() - today.getTime()) / 86_400_000));
	const timeToExpiry = daysToExpiry / 365;

	const strikeStep = spotPrice > 200 ? 5 : spotPrice > 50 ? 2.5 : 1;
	const minStrike = Math.floor((spotPrice * 0.8) / strikeStep) * strikeStep;
	const maxStrike = Math.ceil((spotPrice * 1.2) / strikeStep) * strikeStep;

	const strikes: number[] = [];
	for (let s = minStrike; s <= maxStrike; s += strikeStep) {
		strikes.push(s);
	}

	const riskFreeRate = 0.0428;
	const baseVol = 0.25 + Math.random() * 0.10;

	const calls: OptionQuote[] = [];
	const puts: OptionQuote[] = [];

	for (const strike of strikes) {
		const moneyness = strike / spotPrice;
		const skew = 0.08 * (1 - moneyness);
		const smile = 0.03 * (moneyness - 1) ** 2;
		const iv = Math.max(0.05, baseVol + skew + smile);

		const bsInputs: BSInputs = {
			spotPrice,
			strikePrice: strike,
			volatility: iv,
			timeToExpiry,
			riskFreeRate,
			dividendYield: divYield,
		};

		const pricing = bsPrice(bsInputs);
		const callGreeks = firstOrderGreeks(bsInputs, 'call');
		const putGreeks = firstOrderGreeks(bsInputs, 'put');

		const spreadPct = 0.02 + 0.05 * Math.abs(moneyness - 1);
		const callMid = pricing.callPrice;
		const putMid = pricing.putPrice;

		calls.push({
			symbol: `${ticker}${expiration.replace(/-/g, '')}C${strike.toFixed(0).padStart(8, '0')}`,
			underlying: ticker,
			type: 'call',
			strike,
			expiration,
			lastPrice: addNoise(callMid),
			bid: Math.max(0, callMid * (1 - spreadPct / 2)),
			ask: callMid * (1 + spreadPct / 2),
			mid: callMid,
			volume: Math.round(Math.random() * 5000 * Math.exp(-2 * Math.abs(moneyness - 1))),
			openInterest: Math.round(Math.random() * 20000 * Math.exp(-1.5 * Math.abs(moneyness - 1))),
			impliedVolatility: iv,
			delta: callGreeks.delta,
			gamma: callGreeks.gamma,
			theta: callGreeks.theta,
			vega: callGreeks.vega,
			rho: callGreeks.rho,
			inTheMoney: spotPrice > strike,
			source: 'mock' as DataProviderName,
		});

		puts.push({
			symbol: `${ticker}${expiration.replace(/-/g, '')}P${strike.toFixed(0).padStart(8, '0')}`,
			underlying: ticker,
			type: 'put',
			strike,
			expiration,
			lastPrice: addNoise(putMid),
			bid: Math.max(0, putMid * (1 - spreadPct / 2)),
			ask: putMid * (1 + spreadPct / 2),
			mid: putMid,
			volume: Math.round(Math.random() * 4000 * Math.exp(-2 * Math.abs(moneyness - 1))),
			openInterest: Math.round(Math.random() * 18000 * Math.exp(-1.5 * Math.abs(moneyness - 1))),
			impliedVolatility: iv,
			delta: putGreeks.delta,
			gamma: putGreeks.gamma,
			theta: putGreeks.theta,
			vega: putGreeks.vega,
			rho: putGreeks.rho,
			inTheMoney: spotPrice < strike,
			source: 'mock' as DataProviderName,
		});
	}

	return {
		underlying: ticker,
		underlyingPrice: spotPrice,
		expiration,
		calls,
		puts,
		timestamp: new Date().toISOString(),
		source: 'mock',
	};
}

export function createMockAdapter(): MarketDataProvider {
	const capabilities: ProviderCapabilities = {
		quotes: true,
		optionsChain: true,
		historicalPrices: true,
		historicalVol: true,
		dividends: true,
		earnings: true,
		riskFreeRate: true,
		tickerSearch: true,
		streaming: false,
	};

	return {
		name: 'mock',
		displayName: 'Mock Data (Development)',
		capabilities,

		async isAvailable() {
			return true;
		},

		async getStatus(): Promise<ProviderStatus> {
			return {
				name: 'mock',
				displayName: 'Mock Data (Development)',
				isAvailable: true,
				isConfigured: true,
				avgResponseTime: 5,
			};
		},

		async getQuote(ticker: string): Promise<StockQuote> {
			const stock = MOCK_STOCKS[ticker.toUpperCase()];
			if (!stock) throw new Error(`Unknown ticker: ${ticker}`);
			const p = addNoise(stock.price);
			const prevClose = addNoise(stock.price, 0.01);
			return {
				ticker: ticker.toUpperCase(),
				price: p,
				open: addNoise(stock.price, 0.005),
				high: p * 1.008,
				low: p * 0.992,
				close: p,
				previousClose: prevClose,
				volume: Math.round(Math.random() * 50_000_000),
				change: p - prevClose,
				changePercent: ((p - prevClose) / prevClose) * 100,
				timestamp: new Date().toISOString(),
				source: 'mock',
			};
		},

		async getHistoricalPrices(ticker: string, from: string, to: string): Promise<OHLCV[]> {
			const stock = MOCK_STOCKS[ticker.toUpperCase()];
			if (!stock) throw new Error(`Unknown ticker: ${ticker}`);

			const startDate = new Date(from);
			const endDate = new Date(to);
			const days = Math.round((endDate.getTime() - startDate.getTime()) / 86_400_000);
			const data: OHLCV[] = [];

			let currentPrice = stock.price * 0.9;
			for (let i = 0; i < days; i++) {
				const date = new Date(startDate);
				date.setDate(date.getDate() + i);
				if (date.getDay() === 0 || date.getDay() === 6) continue;

				const dailyReturn = (Math.random() - 0.48) * 0.03;
				currentPrice *= 1 + dailyReturn;
				const high = currentPrice * (1 + Math.random() * 0.015);
				const low = currentPrice * (1 - Math.random() * 0.015);

				data.push({
					date: date.toISOString().split('T')[0],
					open: addNoise(currentPrice, 0.005),
					high,
					low,
					close: currentPrice,
					volume: Math.round(30_000_000 + Math.random() * 40_000_000),
				});
			}
			return data;
		},

		async getOptionsChain(ticker: string, expiration: string): Promise<OptionsChain> {
			const stock = MOCK_STOCKS[ticker.toUpperCase()];
			if (!stock) throw new Error(`Unknown ticker: ${ticker}`);
			return generateMockChain(ticker.toUpperCase(), stock.price, expiration, stock.div);
		},

		async getExpirations(ticker: string): Promise<ExpirationList> {
			return {
				underlying: ticker.toUpperCase(),
				expirations: generateExpirations(12),
				source: 'mock',
			};
		},

		async getHistoricalVol(ticker: string): Promise<HistoricalVolData> {
			const baseVol = 0.20 + Math.random() * 0.15;
			return {
				ticker: ticker.toUpperCase(),
				hv10: baseVol * (0.9 + Math.random() * 0.2),
				hv20: baseVol * (0.95 + Math.random() * 0.1),
				hv30: baseVol,
				hv60: baseVol * (1.0 + Math.random() * 0.1),
				hv90: baseVol * (1.0 + Math.random() * 0.15),
				ivPercentile: Math.round(Math.random() * 100),
				ivRank: Math.round(Math.random() * 100),
				source: 'mock',
			};
		},

		async getDividendInfo(ticker: string): Promise<DividendInfo> {
			const stock = MOCK_STOCKS[ticker.toUpperCase()];
			if (!stock) throw new Error(`Unknown ticker: ${ticker}`);
			return {
				ticker: ticker.toUpperCase(),
				annualDividend: stock.price * stock.div,
				dividendYield: stock.div,
				frequency: stock.div > 0 ? 'quarterly' : 'none',
				source: 'mock',
			};
		},

		async getEarningsInfo(ticker: string): Promise<EarningsInfo> {
			const daysToEarnings = Math.round(15 + Math.random() * 60);
			const nextDate = new Date();
			nextDate.setDate(nextDate.getDate() + daysToEarnings);
			return {
				ticker: ticker.toUpperCase(),
				nextEarningsDate: nextDate.toISOString().split('T')[0],
				daysToEarnings,
				expectedMovePercent: 4 + Math.random() * 6,
				source: 'mock',
			};
		},

		async getRiskFreeRate(): Promise<RiskFreeRateData> {
			return {
				rate1M: 0.0432,
				rate3M: 0.0428,
				rate6M: 0.0415,
				rate1Y: 0.0398,
				rate2Y: 0.0385,
				rate10Y: 0.0395,
				date: new Date().toISOString().split('T')[0],
				source: 'mock',
			};
		},

		async searchTickers(query: string, limit: number = 10): Promise<TickerSearchResult[]> {
			const q = query.toUpperCase();
			const results: TickerSearchResult[] = [];

			for (const [ticker, stock] of Object.entries(MOCK_STOCKS)) {
				if (ticker.includes(q) || stock.name.toUpperCase().includes(q)) {
					results.push({
						ticker,
						name: stock.name,
						type: stock.type,
						exchange: 'NASDAQ',
					});
				}
				if (results.length >= limit) break;
			}
			return results;
		},
	};
}
