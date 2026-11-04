import { expect, it, vi } from 'vite-plus/test';
import { render, waitFor } from '@testing-library/svelte';
import StripePayment from './StripePayment.svelte';

const fixture = vi.hoisted(() => {
	const card = { mount: vi.fn(), on: vi.fn(), destroy: vi.fn() };
	const wallet = { mount: vi.fn(), on: vi.fn(), destroy: vi.fn() };
	const walletElements = { create: vi.fn(() => wallet), update: vi.fn() };
	const stripe = {
		elements: vi.fn((options?: unknown) => (options ? walletElements : { create: () => card })),
		confirmCardPayment: vi
			.fn()
			.mockResolvedValue({ error: { message: 'Controlled fixture decline' } })
	};
	return { card, wallet, walletElements, stripe, load: vi.fn().mockResolvedValue(stripe) };
});
vi.mock('@stripe/stripe-js', () => ({ loadStripe: fixture.load }));

it('waits for card mounting and updates wallet totals without remounting or reconfirming', async () => {
	fixture.load.mockResolvedValue(fixture.stripe);
	fixture.stripe.elements.mockImplementation((options?: unknown) =>
		options ? fixture.walletElements : { create: () => fixture.card }
	);
	fixture.walletElements.create.mockReturnValue(fixture.wallet);
	fixture.stripe.confirmCardPayment.mockResolvedValue({
		error: { message: 'Controlled fixture decline' }
	});
	const onerror = vi.fn();
	const view = render(StripePayment, {
		publicKey: 'pk_test_fixture',
		amount: 10,
		currency: 'USD',
		clientSecret: 'fixture-secret',
		onerror
	});
	await waitFor(() => expect(fixture.card.mount).toHaveBeenCalledTimes(1));
	await waitFor(() => expect(fixture.stripe.confirmCardPayment).toHaveBeenCalledTimes(1));
	expect(fixture.card.mount.mock.invocationCallOrder[0]).toBeLessThan(
		fixture.stripe.confirmCardPayment.mock.invocationCallOrder[0] ?? 0
	);
	await waitFor(() => expect(onerror).toHaveBeenCalledWith('Controlled fixture decline'));
	await view.rerender({ amount: 20.01, currency: 'EUR' });
	await waitFor(() =>
		expect(fixture.walletElements.update).toHaveBeenCalledWith({ amount: 2001, currency: 'eur' })
	);
	expect(fixture.card.mount).toHaveBeenCalledTimes(1);
	expect(fixture.wallet.mount).toHaveBeenCalledTimes(1);
	expect(fixture.stripe.confirmCardPayment).toHaveBeenCalledTimes(1);
});
