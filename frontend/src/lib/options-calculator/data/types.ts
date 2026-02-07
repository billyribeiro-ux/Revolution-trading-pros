// ============================================================
// MARKET DATA LAYER — TYPE DEFINITIONS
// ============================================================

/** Stock quote from any provider */
export interface StockQuote {
	ticker: string;
	price: number;
	open: number;
	high: number;
	low: number;
	close: number;
	previousClose: number;
	volume: number;
	change: number;
	changePercent: number;
	marketCap?: number;
	timestamp: string;
	source: DataProviderName;
}

/** Single option contract quote */
export interface OptionQuote {
	symbol: string;
	underlying: string;
	type: 'call' | 'put';
	strike: number;
	expiration: string;
	lastPrice: number;
	bid: number;
	ask: number;
	mid: number;
	volume: number;
	openInterest: number;
	impliedVolatility: number;
	delta?: number;
	gamma?: number;
	theta?: number;
	vega?: number;
	rho?: number;
	inTheMoney: boolean;
	source: DataProviderName;
}

/** Full options chain for a ticker + expiration */
export interface OptionsChain {
	underlying: string;
	underlyingPrice: number;
	expiration: string;
	calls: OptionQuote[];
	puts: OptionQuote[];
	timestamp: string;
	source: DataProviderName;
}

/** Available expirations for a ticker */
export interface ExpirationList {
	underlying: string;
	expirations: string[];
	source: DataProviderName;
}

/** Historical OHLCV data point */
export interface OHLCV {
	date: string;
	open: number;
	high: number;
	low: number;
	close: number;
	volume: number;
}

/** Ticker search result */
export interface TickerSearchResult {
	ticker: string;
	name: string;
	type: 'stock' | 'etf' | 'index' | 'forex' | 'crypto';
	exchange: string;
	primaryExchange?: string;
}

/** Risk-free rate data */
export interface RiskFreeRateData {
	rate1M: number;
	rate3M: number;
	rate6M: number;
	rate1Y: number;
	rate2Y: number;
	rate10Y: number;
	date: string;
	source: DataProviderName;
}

/** Dividend info */
export interface DividendInfo {
	ticker: string;
	annualDividend: number;
	dividendYield: number;
	exDividendDate?: string;
	paymentDate?: string;
	frequency: 'annual' | 'semi-annual' | 'quarterly' | 'monthly' | 'none';
	source: DataProviderName;
}

/** Earnings info */
export interface EarningsInfo {
	ticker: string;
	nextEarningsDate?: string;
	daysToEarnings?: number;
	expectedMove?: number;
	expectedMovePercent?: number;
	source: DataProviderName;
}

/** Historical volatility data */
export interface HistoricalVolData {
	ticker: string;
	hv10: number;
	hv20: number;
	hv30: number;
	hv60: number;
	hv90: number;
	ivPercentile?: number;
	ivRank?: number;
	source: DataProviderName;
}

/** Mispricing detection result */
export interface MispricingResult {
	option: OptionQuote;
	theoreticalPrice: number;
	marketPrice: number;
	priceDifference: number;
	priceDifferencePct: number;
	direction: 'overpriced' | 'underpriced';
	significance: 'minor' | 'moderate' | 'significant';
}

/** Data provider names */
export type DataProviderName = 'polygon' | 'theta-data' | 'tradier' | 'fred' | 'yahoo' | 'mock';

/** Provider status */
export interface ProviderStatus {
	name: DataProviderName;
	displayName: string;
	isAvailable: boolean;
	isConfigured: boolean;
	lastUsed?: string;
	rateLimitRemaining?: number;
	rateLimitResetAt?: string;
	avgResponseTime?: number;
}

/** Provider capability flags */
export interface ProviderCapabilities {
	quotes: boolean;
	optionsChain: boolean;
	historicalPrices: boolean;
	historicalVol: boolean;
	dividends: boolean;
	earnings: boolean;
	riskFreeRate: boolean;
	tickerSearch: boolean;
	streaming: boolean;
}

/** Cache entry with metadata */
export interface CacheEntry<T> {
	data: T;
	timestamp: number;
	ttl: number;
	source: DataProviderName;
	key: string;
}

/** Rate limit tracking */
export interface RateLimitState {
	provider: DataProviderName;
	requestsRemaining: number;
	requestsLimit: number;
	resetTimestamp: number;
	requestsMade: number;
}

/** Data mode */
export type DataMode = 'manual' | 'live';

/** Combined market snapshot for a ticker */
export interface MarketSnapshot {
	quote: StockQuote;
	dividends: DividendInfo;
	earnings: EarningsInfo;
	historicalVol: HistoricalVolData;
	riskFreeRate: RiskFreeRateData;
	suggestedInputs: {
		spotPrice: number;
		volatility: number;
		dividendYield: number;
		riskFreeRate: number;
	};
}
