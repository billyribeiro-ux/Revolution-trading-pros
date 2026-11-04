import { afterEach, expect, it, vi } from 'vite-plus/test';
import { fireEvent, render, waitFor } from '@testing-library/svelte';
import { tick } from 'svelte';
import Harness from './TradeHookHarness.svelte';
import type { PriceData } from '#lib/services/price-feed.js';

const fixture = vi.hoisted(() => ({
	get: vi.fn(),
	subscribe: vi.fn(),
	unsubscribe: vi.fn(),
	off: vi.fn(),
	listeners: [] as ((prices: Map<string, PriceData>) => void)[]
}));
vi.mock('#lib/api/enterprise/client.js', () => ({
	getEnterpriseClient: () => ({ get: fixture.get })
}));
vi.mock('#lib/services/price-feed.js', () => ({
	getPriceFeed: () => ({
		subscribe: fixture.subscribe,
		unsubscribe: fixture.unsubscribe,
		onUpdate: (callback: (prices: Map<string, PriceData>) => void) => {
			fixture.listeners.push(callback);
			return fixture.off;
		}
	})
}));
afterEach(() => {
	vi.useRealTimers();
	vi.clearAllMocks();
	fixture.listeners.length = 0;
});

it('loads once, keeps quote updates reactive, reconciles positions and releases the feed on unmount', async () => {
	fixture.get.mockResolvedValue({
		success: true,
		data: [
			{
				id: 1,
				ticker: 'AAPL',
				status: 'open',
				entry_price: 100,
				entry_date: '2026-11-01',
				quantity: 1
			}
		]
	});
	const view = render(Harness);
	await waitFor(() => expect(view.getByTestId('positions')).toHaveTextContent('1'));
	expect(fixture.get).toHaveBeenCalledTimes(1);
	expect(fixture.subscribe).toHaveBeenCalledWith(['AAPL']);
	const listener = fixture.listeners[0];
	if (!listener) throw new Error('Price listener was not installed');
	listener(
		new Map([
			[
				'AAPL',
				{
					ticker: 'AAPL',
					price: 120,
					change: 20,
					changePercent: 20,
					timestamp: Date.now(),
					marketOpen: true
				}
			]
		])
	);
	await tick();
	expect(view.getByTestId('quote')).toHaveTextContent('AAPL: 120');
	expect(fixture.listeners).toHaveLength(1);
	expect(fixture.get).toHaveBeenCalledTimes(1);
	fixture.get.mockResolvedValue({
		success: true,
		data: [
			{
				id: 2,
				ticker: 'MSFT',
				status: 'open',
				entry_price: 200,
				entry_date: '2026-11-01',
				quantity: 1
			}
		]
	});
	await fireEvent.click(view.getByRole('button', { name: 'Refresh' }));
	await waitFor(() => expect(view.getByTestId('quote')).toHaveTextContent('MSFT: 200'));
	expect(fixture.unsubscribe).toHaveBeenCalledWith(['AAPL']);
	expect(fixture.subscribe).toHaveBeenCalledWith(['MSFT']);
	view.unmount();
	expect(fixture.off).toHaveBeenCalledTimes(1);
	expect(fixture.unsubscribe).toHaveBeenCalledWith(['MSFT']);
});

it('stops automatic refresh when removed', async () => {
	fixture.get.mockResolvedValue({ success: true, data: [] });
	vi.useFakeTimers({
		toFake: ['setInterval', 'clearInterval']
	});
	const view = render(Harness, { autoRefresh: true });
	await tick();
	await vi.advanceTimersByTimeAsync(250);
	expect(fixture.get).toHaveBeenCalledTimes(3);
	view.unmount();
	await vi.advanceTimersByTimeAsync(500);
	expect(fixture.get).toHaveBeenCalledTimes(3);
});
