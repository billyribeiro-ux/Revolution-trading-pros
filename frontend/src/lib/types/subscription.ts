export interface Subscription {
	id: string;
	user_id: string;
	status: 'active' | 'pending' | 'on-hold' | 'cancelled' | 'expired' | 'pending-cancel' | 'trial';
	plan: SubscriptionPlan;
	trial_ends_at: string | null;
	current_period_start: string | null;
	current_period_end: string | null;
	cancelled_at: string | null;
	paused_at: string | null;
	expires_at: string | null;
	payment_method: string | null;
	amount_paid: number;
	billing_cycles_completed: number;
	notes: string | null;
	created_at: string;
	updated_at: string;

	// Helper fields for UI
	productName?: string; // Mapped from plan.name
	price?: number; // Mapped from plan.price
	interval?: string; // Mapped from plan.billing_period
	nextPaymentDate?: string; // Mapped from current_period_end
}

export interface SubscriptionPlan {
	id: string;
	name: string;
	slug: string;
	description: string | null;
	price: number;
	billing_period: string;
	billing_interval: number;
	trial_days: number;
	currency: string;
	is_active: boolean;
}

export interface SubscriptionStats {
	total_subscriptions: number;
	active_subscriptions: number;
	monthly_revenue: number;
	churn_rate: number;
}
