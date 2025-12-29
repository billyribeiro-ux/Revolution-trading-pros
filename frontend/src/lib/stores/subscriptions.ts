import { writable, derived } from 'svelte/store';

// Subscription status types (like WooCommerce Subscriptions)
export type SubscriptionStatus =
	| 'active' // Subscription is active and payments are processing
	| 'pending' // Waiting for first payment
	| 'on-hold' // Paused due to payment failure or manual hold
	| 'cancelled' // Cancelled by user or admin
	| 'expired' // Subscription period ended
	| 'pending-cancel'; // Scheduled to cancel at end of period

export type PaymentStatus = 'paid' | 'failed' | 'pending' | 'refunded' | 'partially-refunded';

export type SubscriptionInterval = 'monthly' | 'quarterly' | 'yearly';

export interface SubscriptionPayment {
	id: string;
	amount: number;
	status: PaymentStatus;
	paymentDate: string;
	dueDate: string;
	paymentMethod: string;
	failureReason?: string;
	retryCount?: number;
	nextRetryDate?: string;
}

export interface Subscription {
	id: string;
	userId: string;
	productId: string;
	productName: string;
	status: SubscriptionStatus;
	interval: SubscriptionInterval;

	// Pricing
	price: number;
	currency: string;

	// Dates
	startDate: string;
	nextPaymentDate: string;
	lastPaymentDate?: string;
	endDate?: string;
	cancelledAt?: string;
	pausedAt?: string;

	// Payment tracking
	totalPaid: number;
	failedPayments: number;
	successfulPayments: number;
	paymentHistory: SubscriptionPayment[];

	// Actions taken
	pauseReason?: string;
	cancellationReason?: string;

	// Renewal tracking
	renewalCount: number;
	autoRenew: boolean;

	// Trial period
	trialEndDate?: string;
	isTrialing: boolean;

	// Payment method
	paymentMethod: {
		type: 'card' | 'paypal' | 'bank';
		last4?: string;
		brand?: string;
		expiryMonth?: number;
		expiryYear?: number;
	};

	// Notifications
	emailsSent: {
		type: 'renewal' | 'failed-payment' | 'cancelled' | 'paused' | 'reactivated';
		sentAt: string;
		subject: string;
	}[];

	// Metadata
	createdAt: string;
	updatedAt: string;
	notes?: string;
}

export interface SubscriptionFilters {
	status?: SubscriptionStatus[];
	interval?: SubscriptionInterval[];
	searchQuery?: string;
	dateFrom?: string;
	dateTo?: string;
}

export interface SubscriptionStats {
	total: number;
	active: number;
	totalActive: number;
	newThisMonth: number;
	onHold: number;
	cancelled: number;
	expired: number;
	pendingCancel: number;
	monthlyRecurringRevenue: number;
	churnRate: number;
	averageLifetimeValue: number;
}

// Store state
interface SubscriptionState {
	subscriptions: Subscription[];
	filters: SubscriptionFilters;
	loading: boolean;
	error: string | null;
}

const initialState: SubscriptionState = {
	subscriptions: [],
	filters: {},
	loading: false,
	error: null
};

// Create the subscription store
function createSubscriptionStore() {
	const { subscribe, set, update } = writable<SubscriptionState>(initialState);

	return {
		subscribe,

		// Load subscriptions - Uses enterprise API service
		loadSubscriptions: async (filters?: SubscriptionFilters) => {
			update((state) => ({ ...state, loading: true, error: null }));

			try {
				// Import dynamically to avoid circular dependencies
				const { getSubscriptions } = await import('$lib/api/subscriptions');
				const subscriptions = await getSubscriptions(filters);

				update((state) => ({
					...state,
					subscriptions: subscriptions as Subscription[],
					filters: filters || {},
					loading: false
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: 'Failed to load subscriptions',
					loading: false
				}));
			}
		},

		// Update subscription status - Connected to enterprise API
		updateStatus: async (
			subscriptionId: string,
			newStatus: SubscriptionStatus,
			reason?: string
		) => {
			update((state) => ({ ...state, loading: true }));

			try {
				const { updateSubscription } = await import('$lib/api/subscriptions');
				const updated = await updateSubscription(subscriptionId, {
					status: newStatus,
					...(reason && { cancellationReason: reason, pauseReason: reason })
				});

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId ? { ...sub, ...updated } : sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to update subscription', loading: false }));
			}
		},

		// Pause subscription - Connected to enterprise API
		pauseSubscription: async (subscriptionId: string, reason: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				const { pauseSubscription } = await import('$lib/api/subscriptions');
				const updated = await pauseSubscription(subscriptionId, reason);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId ? { ...sub, ...updated } : sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to pause subscription', loading: false }));
			}
		},

		// Resume subscription - Connected to enterprise API
		resumeSubscription: async (subscriptionId: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				const { resumeSubscription } = await import('$lib/api/subscriptions');
				const updated = await resumeSubscription(subscriptionId);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId ? { ...sub, ...updated } : sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to resume subscription', loading: false }));
			}
		},

		// Cancel subscription - Connected to enterprise API
		cancelSubscription: async (
			subscriptionId: string,
			reason: string,
			immediate: boolean = false
		) => {
			update((state) => ({ ...state, loading: true }));

			try {
				const { cancelSubscription } = await import('$lib/api/subscriptions');
				const updated = await cancelSubscription(subscriptionId, reason, immediate);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId ? { ...sub, ...updated } : sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to cancel subscription', loading: false }));
			}
		},

		// Reactivate cancelled subscription - Connected to enterprise API
		reactivateSubscription: async (subscriptionId: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				const { reactivateSubscription } = await import('$lib/api/subscriptions');
				const updated = await reactivateSubscription(subscriptionId);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId ? { ...sub, ...updated } : sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: 'Failed to reactivate subscription',
					loading: false
				}));
			}
		},

		// Retry failed payment - Connected to enterprise API
		retryPayment: async (subscriptionId: string, paymentId: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				const { retryPayment } = await import('$lib/api/subscriptions');
				await retryPayment(subscriptionId, paymentId);

				// Reload subscriptions to get updated payment status
				const { getSubscriptions } = await import('$lib/api/subscriptions');
				const subscriptions = await getSubscriptions();

				update((state) => ({
					...state,
					subscriptions: subscriptions as Subscription[],
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to retry payment', loading: false }));
			}
		},

		// Update payment method - Connected to enterprise API
		updatePaymentMethod: async (
			subscriptionId: string,
			paymentMethod: Subscription['paymentMethod']
		) => {
			update((state) => ({ ...state, loading: true }));

			try {
				const { updatePaymentMethod } = await import('$lib/api/subscriptions');
				const updated = await updatePaymentMethod(subscriptionId, paymentMethod as any);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId ? { ...sub, ...updated } : sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to update payment method', loading: false }));
			}
		},

		// Process renewal - Connected to enterprise API
		processRenewal: async (subscriptionId: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				const { processPayment } = await import('$lib/api/subscriptions');
				await processPayment(subscriptionId);

				// Reload subscriptions to get updated renewal status
				const { getSubscriptions } = await import('$lib/api/subscriptions');
				const subscriptions = await getSubscriptions();

				update((state) => ({
					...state,
					subscriptions: subscriptions as Subscription[],
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to process renewal', loading: false }));
			}
		},

		// Record payment failure - handled by API webhook
		recordPaymentFailure: async (_subscriptionId: string, _reason: string) => {
			// Payment failures are tracked automatically by the enterprise API service via webhooks
			// This method is here for backwards compatibility but now just reloads data
			update((state) => ({ ...state, loading: true }));

			try {
				const { getSubscriptions } = await import('$lib/api/subscriptions');
				const subscriptions = await getSubscriptions();

				update((state) => ({
					...state,
					subscriptions: subscriptions as Subscription[],
					loading: false
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: 'Failed to record payment failure',
					loading: false
				}));
			}
		},

		// Set filters
		setFilters: (filters: SubscriptionFilters) => {
			update((state) => ({ ...state, filters }));
		},

		// Clear filters
		clearFilters: () => {
			update((state) => ({ ...state, filters: {} }));
		},

		// Reset store
		reset: () => {
			set(initialState);
		}
	};
}

// Create the store instance
export const subscriptionStore = createSubscriptionStore();

// Derived stores for filtered data
export const filteredSubscriptions = derived(subscriptionStore, ($store) => {
	let filtered = $store.subscriptions;

	// Filter by status
	if ($store.filters.status && $store.filters.status.length > 0) {
		filtered = filtered.filter((sub) => $store.filters.status!.includes(sub.status));
	}

	// Filter by interval
	if ($store.filters.interval && $store.filters.interval.length > 0) {
		filtered = filtered.filter((sub) => $store.filters.interval!.includes(sub.interval));
	}

	// Search filter
	if ($store.filters.searchQuery) {
		const query = $store.filters.searchQuery.toLowerCase();
		filtered = filtered.filter(
			(sub) =>
				sub.productName.toLowerCase().includes(query) ||
				sub.userId.toLowerCase().includes(query) ||
				sub.id.toLowerCase().includes(query)
		);
	}

	// Date filters
	if ($store.filters.dateFrom) {
		filtered = filtered.filter((sub) => sub.createdAt >= $store.filters.dateFrom!);
	}

	if ($store.filters.dateTo) {
		filtered = filtered.filter((sub) => sub.createdAt <= $store.filters.dateTo!);
	}

	return filtered;
});

// Derived store for statistics
export const subscriptionStats = derived(subscriptionStore, ($store) => {
	const subs = $store.subscriptions;

	const activeCount = subs.filter((s) => s.status === 'active').length;
	const thisMonth = new Date();
	thisMonth.setDate(1);
	thisMonth.setHours(0, 0, 0, 0);
	const newThisMonthCount = subs.filter((s) => {
		const createdDate = new Date(s.createdAt);
		return createdDate >= thisMonth && s.status === 'active';
	}).length;

	const stats: SubscriptionStats = {
		total: subs.length,
		active: activeCount,
		totalActive: activeCount,
		newThisMonth: newThisMonthCount,
		onHold: subs.filter((s) => s.status === 'on-hold').length,
		cancelled: subs.filter((s) => s.status === 'cancelled').length,
		expired: subs.filter((s) => s.status === 'expired').length,
		pendingCancel: subs.filter((s) => s.status === 'pending-cancel').length,
		monthlyRecurringRevenue: calculateMRR(subs),
		churnRate: calculateChurnRate(subs),
		averageLifetimeValue: calculateAvgLTV(subs)
	};

	return stats;
});

// Helper functions for statistics
function calculateMRR(subscriptions: Subscription[]): number {
	return subscriptions
		.filter((s) => s.status === 'active' || s.status === 'pending-cancel')
		.reduce((sum, sub) => {
			let monthlyValue = sub.price;

			if (sub.interval === 'quarterly') {
				monthlyValue = sub.price / 3;
			} else if (sub.interval === 'yearly') {
				monthlyValue = sub.price / 12;
			}

			return sum + monthlyValue;
		}, 0);
}

function calculateChurnRate(subscriptions: Subscription[]): number {
	const activeAtStart = subscriptions.filter((s) => s.status === 'active').length;
	const cancelled = subscriptions.filter((s) => s.status === 'cancelled').length;

	if (activeAtStart === 0) return 0;

	return (cancelled / (activeAtStart + cancelled)) * 100;
}

function calculateAvgLTV(subscriptions: Subscription[]): number {
	if (subscriptions.length === 0) return 0;

	const totalRevenue = subscriptions.reduce((sum, sub) => sum + sub.totalPaid, 0);
	return totalRevenue / subscriptions.length;
}
