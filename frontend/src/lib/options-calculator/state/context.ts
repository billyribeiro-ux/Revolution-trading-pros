import { createContext } from 'svelte';
import type { CalculatorState } from './calculator.svelte.js';
import type { MarketDataService } from '../data/market-data-service.svelte.js';
export const [getCalculator, setCalculator] = createContext<CalculatorState>();
export const [getMarketData, setMarketData] = createContext<MarketDataService>();
