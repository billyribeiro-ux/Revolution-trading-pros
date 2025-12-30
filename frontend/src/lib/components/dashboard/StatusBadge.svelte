<script lang="ts">
	/**
	 * StatusBadge - Shared Component for Order/Subscription Status
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for displaying status badges with consistent styling.
	 * Used in orders, subscriptions, and other status displays.
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */

	type StatusType = 'completed' | 'pending' | 'processing' | 'failed' | 'refunded' | 'cancelled' | 'active' | 'on-hold' | 'expired' | 'success' | 'info' | 'warning' | 'error' | 'danger';

	interface Props {
		/** The status to display */
		status: string;
		/** Optional size variant */
		size?: 'sm' | 'md' | 'lg';
	}

	let { status, size = 'md' }: Props = $props();

	// Normalize status to standard variant
	function getVariant(status: string): StatusType {
		const normalized = status.toLowerCase().replace(/[^a-z-]/g, '');

		const statusMap: Record<string, StatusType> = {
			// Order statuses
			'completed': 'completed',
			'complete': 'completed',
			'pending': 'pending',
			'pendingpayment': 'pending',
			'processing': 'processing',
			'failed': 'failed',
			'refunded': 'refunded',
			'cancelled': 'cancelled',
			'canceled': 'cancelled',
			// Subscription statuses
			'active': 'active',
			'onhold': 'on-hold',
			'on-hold': 'on-hold',
			'expired': 'expired',
			// Generic statuses
			'success': 'success',
			'info': 'info',
			'warning': 'warning',
			'error': 'error',
			'danger': 'danger'
		};

		return statusMap[normalized] || 'pending';
	}

	// Capitalize first letter for display
	function capitalize(str: string): string {
		return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase().replace(/-/g, ' ');
	}

	let variant = $derived(getVariant(status));
	let displayText = $derived(capitalize(status));
</script>

<span class="status-badge status--{variant} size--{size}">
	{displayText}
</span>

<style>
	/* Base Badge Styles */
	.status-badge {
		display: inline-block;
		font-weight: 600;
		border-radius: 4px;
		text-transform: capitalize;
		white-space: nowrap;
	}

	/* Size Variants */
	.size--sm {
		padding: 2px 8px;
		font-size: 11px;
	}

	.size--md {
		padding: 4px 10px;
		font-size: 12px;
	}

	.size--lg {
		padding: 6px 14px;
		font-size: 14px;
	}

	/* Status Variants - Order Statuses */
	.status--completed,
	.status--success,
	.status--active {
		background: #e8f5e9;
		color: #2e7d32;
	}

	.status--pending,
	.status--warning,
	.status--on-hold {
		background: #fff3e0;
		color: #ef6c00;
	}

	.status--processing,
	.status--info {
		background: #e3f2fd;
		color: #1565c0;
	}

	.status--failed,
	.status--error,
	.status--danger,
	.status--expired {
		background: #ffebee;
		color: #c62828;
	}

	.status--refunded {
		background: #f3e5f5;
		color: #7b1fa2;
	}

	.status--cancelled {
		background: #fafafa;
		color: #616161;
	}
</style>
