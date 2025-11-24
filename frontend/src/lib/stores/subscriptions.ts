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

		// Load subscriptions
		loadSubscriptions: async (filters?: SubscriptionFilters) => {
			update((state) => ({ ...state, loading: true, error: null }));

			try {
				// TODO: Replace with actual API call
				// const subscriptions = await getSubscriptions(filters);
				const subscriptions: Subscription[] = []; // Mock data

				update((state) => ({
					...state,
					subscriptions,
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

		// Update subscription status
		updateStatus: async (
			subscriptionId: string,
			newStatus: SubscriptionStatus,
			reason?: string
		) => {
			update((state) => ({ ...state, loading: true }));

			try {
				// TODO: API call to update status
				// await updateSubscriptionStatus(subscriptionId, newStatus, reason);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId
							? {
									...sub,
									status: newStatus,
									updatedAt: new Date().toISOString(),
									...(newStatus === 'cancelled' && {
										cancelledAt: new Date().toISOString(),
										cancellationReason: reason
									}),
									...(newStatus === 'on-hold' && {
										pausedAt: new Date().toISOString(),
										pauseReason: reason
									})
								}
							: sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to update subscription', loading: false }));
			}
		},

		// Pause subscription
		pauseSubscription: async (subscriptionId: string, reason: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				// TODO: API call
				// await pauseSubscription(subscriptionId, reason);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId
							? {
									...sub,
									status: 'on-hold',
									pausedAt: new Date().toISOString(),
									pauseReason: reason,
									updatedAt: new Date().toISOString()
								}
							: sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to pause subscription', loading: false }));
			}
		},

		// Resume subscription
		resumeSubscription: async (subscriptionId: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				// TODO: API call
				// await resumeSubscription(subscriptionId);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId
							? {
									...sub,
									status: 'active',
									pausedAt: undefined,
									pauseReason: undefined,
									updatedAt: new Date().toISOString()
								}
							: sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to resume subscription', loading: false }));
			}
		},

		// Cancel subscription
		cancelSubscription: async (
			subscriptionId: string,
			reason: string,
			immediate: boolean = false
		) => {
			update((state) => ({ ...state, loading: true }));

			try {
				// TODO: API call
				// await cancelSubscription(subscriptionId, reason, immediate);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId
							? {
									...sub,
									status: immediate ? 'cancelled' : 'pending-cancel',
									cancelledAt: immediate ? new Date().toISOString() : undefined,
									cancellationReason: reason,
									autoRenew: false,
									updatedAt: new Date().toISOString()
								}
							: sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to cancel subscription', loading: false }));
			}
		},

		// Reactivate cancelled subscription
		reactivateSubscription: async (subscriptionId: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				// TODO: API call
				// await reactivateSubscription(subscriptionId);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId
							? {
									...sub,
									status: 'active',
									cancelledAt: undefined,
									cancellationReason: undefined,
									autoRenew: true,
									updatedAt: new Date().toISOString()
								}
							: sub
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

		// Retry failed payment
		retryPayment: async (subscriptionId: string, paymentId: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				// TODO: API call to retry payment
				// await retryPayment(subscriptionId, paymentId);

				update((state) => ({ ...state, loading: false }));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to retry payment', loading: false }));
			}
		},

		// Update payment method
		updatePaymentMethod: async (
			subscriptionId: string,
			paymentMethod: Subscription['paymentMethod']
		) => {
			update((state) => ({ ...state, loading: true }));

			try {
				// TODO: API call
				// await updatePaymentMethod(subscriptionId, paymentMethod);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId
							? { ...sub, paymentMethod, updatedAt: new Date().toISOString() }
							: sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to update payment method', loading: false }));
			}
		},

		// Process renewal
		processRenewal: async (subscriptionId: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				// TODO: API call to process renewal
				// const result = await processRenewal(subscriptionId);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId
							? {
									...sub,
									renewalCount: sub.renewalCount + 1,
									lastPaymentDate: new Date().toISOString(),
									nextPaymentDate: calculateNextPaymentDate(sub.interval),
									successfulPayments: sub.successfulPayments + 1,
									totalPaid: sub.totalPaid + sub.price,
									updatedAt: new Date().toISOString()
								}
							: sub
					),
					loading: false
				}));
			} catch (error) {
				update((state) => ({ ...state, error: 'Failed to process renewal', loading: false }));
			}
		},

		// Record payment failure
		recordPaymentFailure: async (subscriptionId: string, reason: string) => {
			update((state) => ({ ...state, loading: true }));

			try {
				// TODO: API call
				// await recordPaymentFailure(subscriptionId, reason);

				update((state) => ({
					...state,
					subscriptions: state.subscriptions.map((sub) =>
						sub.id === subscriptionId
							? {
									...sub,
									status: sub.failedPayments + 1 >= 3 ? 'on-hold' : 'active',
									failedPayments: sub.failedPayments + 1,
									pauseReason:
										sub.failedPayments + 1 >= 3 ? 'Multiple payment failures' : undefined,
									updatedAt: new Date().toISOString()
								}
							: sub
					),
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

// Helper function to calculate next payment date
function calculateNextPaymentDate(interval: SubscriptionInterval): string {
	const now = new Date();

	switch (interval) {
		case 'monthly':
			now.setMonth(now.getMonth() + 1);
			break;
		case 'quarterly':
			now.setMonth(now.getMonth() + 3);
			break;
		case 'yearly':
			now.setFullYear(now.getFullYear() + 1);
			break;
	}

	return now.toISOString();
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
