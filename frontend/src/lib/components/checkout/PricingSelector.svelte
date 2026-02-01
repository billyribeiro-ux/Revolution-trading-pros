<script lang="ts">
	/**
	 * Pricing Selector Component
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Apple ICT 7+ Principal Engineer Grade - January 2026
	 *
	 * Displays subscription plan variants (monthly/quarterly/annual) with
	 * savings badges and recommended highlighting. Used in checkout flows.
	 */

	import type { SubscriptionPlan } from '$lib/api/plans';
	import {
		formatPlanPrice,
		getBillingIntervalText,
		getMonthlyEquivalent,
		sortPlansByBillingCycle
	} from '$lib/api/plans';

	// Props
	interface Props {
		plans: SubscriptionPlan[];
		selectedPlanId?: number;
		onSelect?: (plan: SubscriptionPlan) => void;
		loading?: boolean;
		compact?: boolean;
	}

	let {
		plans,
		selectedPlanId = $bindable(),
		onSelect,
		loading = false,
		compact = false
	}: Props = $props();

	// Derived
	let sortedPlans = $derived(sortPlansByBillingCycle(plans));

	function handleSelect(plan: SubscriptionPlan) {
		selectedPlanId = plan.id;
		onSelect?.(plan);
	}

	function getBillingLabel(cycle: string): string {
		switch (cycle) {
			case 'monthly':
				return 'Monthly';
			case 'quarterly':
				return '3 Months';
			case 'annual':
			case 'yearly':
				return '12 Months';
			default:
				return cycle;
		}
	}
</script>

{#if loading}
	<div class="pricing-loading">
		<div class="spinner"></div>
		<p>Loading pricing options...</p>
	</div>
{:else if sortedPlans.length === 0}
	<div class="pricing-empty">
		<p>No pricing plans available</p>
	</div>
{:else}
	<div class="pricing-selector" class:compact>
		{#each sortedPlans as plan (plan.id)}
			{@const isSelected = selectedPlanId === plan.id}
			{@const monthlyEquiv = getMonthlyEquivalent(plan)}

			<button
				type="button"
				class="pricing-option"
				class:selected={isSelected}
				class:popular={plan.is_popular}
				onclick={() => handleSelect(plan)}
			>
				{#if plan.is_popular}
					<div class="popular-badge">Most Popular</div>
				{/if}

				{#if plan.savings_percent && plan.savings_percent > 0}
					<div class="savings-badge">Save {plan.savings_percent}%</div>
				{/if}

				<div class="option-content">
					<div class="billing-cycle">{getBillingLabel(plan.billing_cycle)}</div>

					<div class="price-display">
						<span class="price-amount">{formatPlanPrice(plan.price)}</span>
						<span class="price-interval">{getBillingIntervalText(plan.billing_cycle)}</span>
					</div>

					{#if plan.billing_cycle !== 'monthly' && !compact}
						<div class="monthly-equivalent">
							{formatPlanPrice(monthlyEquiv)}/mo equivalent
						</div>
					{/if}

					{#if plan.description && !compact}
						<div class="plan-description">{plan.description}</div>
					{/if}
				</div>

				<div class="selection-indicator">
					{#if isSelected}
						<svg class="check-icon" fill="currentColor" viewBox="0 0 20 20">
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
					{:else}
						<div class="radio-circle"></div>
					{/if}
				</div>
			</button>
		{/each}
	</div>
{/if}

<style>
	.pricing-selector {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	/* Pricing Option - 2026 Mobile-First with touch targets */
	.pricing-option {
		position: relative;
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		min-height: 60px; /* Enhanced touch target */
		background: #ffffff;
		border: 2px solid #e5e7eb;
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s ease;
		text-align: left;
		width: 100%;
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
		font-size: 16px; /* Prevents iOS zoom */
	}

	.pricing-option:hover {
		border-color: #0984ae;
		background: #f8fafc;
	}

	.pricing-option.selected {
		border-color: #0984ae;
		background: linear-gradient(135deg, rgba(9, 132, 174, 0.05), rgba(9, 132, 174, 0.1));
		box-shadow: 0 0 0 3px rgba(9, 132, 174, 0.15);
	}

	.pricing-option.popular {
		border-color: #f59e0b;
	}

	.pricing-option.popular.selected {
		border-color: #0984ae;
	}

	.popular-badge {
		position: absolute;
		top: -10px;
		left: 50%;
		transform: translateX(-50%);
		padding: 0.25rem 0.75rem;
		background: linear-gradient(135deg, #f59e0b, #d97706);
		color: white;
		font-size: 0.7rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-radius: 9999px;
		white-space: nowrap;
	}

	.savings-badge {
		position: absolute;
		top: -10px;
		right: 1rem;
		padding: 0.25rem 0.625rem;
		background: linear-gradient(135deg, #10b981, #059669);
		color: white;
		font-size: 0.7rem;
		font-weight: 700;
		border-radius: 9999px;
		white-space: nowrap;
	}

	.option-content {
		flex: 1;
	}

	.billing-cycle {
		font-size: 0.875rem;
		font-weight: 600;
		color: #64748b;
		margin-bottom: 0.25rem;
	}

	.price-display {
		display: flex;
		align-items: baseline;
		gap: 0.375rem;
	}

	.price-amount {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1e293b;
	}

	.price-interval {
		font-size: 0.875rem;
		color: #64748b;
	}

	.monthly-equivalent {
		font-size: 0.75rem;
		color: #94a3b8;
		margin-top: 0.25rem;
	}

	.plan-description {
		font-size: 0.8rem;
		color: #64748b;
		margin-top: 0.5rem;
		line-height: 1.4;
	}

	.selection-indicator {
		flex-shrink: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.radio-circle {
		width: 20px;
		height: 20px;
		border: 2px solid #cbd5e1;
		border-radius: 50%;
		transition: all 0.2s;
	}

	.pricing-option:hover .radio-circle {
		border-color: #0984ae;
	}

	.check-icon {
		width: 24px;
		height: 24px;
		color: #0984ae;
	}

	/* Compact Mode */
	.pricing-selector.compact .pricing-option {
		padding: 1rem 1.25rem;
	}

	.pricing-selector.compact .price-amount {
		font-size: 1.25rem;
	}

	.pricing-selector.compact .billing-cycle {
		font-size: 0.8rem;
	}

	/* Loading & Empty States */
	.pricing-loading,
	.pricing-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		text-align: center;
		color: #64748b;
		gap: 0.75rem;
	}

	.spinner {
		width: 28px;
		height: 28px;
		border: 3px solid #e5e7eb;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Responsive */
	@media (min-width: 640px) {
		.pricing-selector:not(.compact) {
			flex-direction: row;
		}

		.pricing-selector:not(.compact) .pricing-option {
			flex: 1;
			flex-direction: column;
			text-align: center;
			padding: 1.5rem 1rem;
		}

		.pricing-selector:not(.compact) .option-content {
			order: 1;
		}

		.pricing-selector:not(.compact) .selection-indicator {
			order: 2;
			margin-top: 1rem;
		}

		.pricing-selector:not(.compact) .price-display {
			flex-direction: column;
			align-items: center;
			gap: 0;
		}

		.pricing-selector:not(.compact) .price-amount {
			font-size: 1.75rem;
		}

		.pricing-selector:not(.compact) .savings-badge {
			top: -10px;
			right: 50%;
			transform: translateX(50%);
		}

		.pricing-selector:not(.compact) .popular-badge {
			top: -12px;
		}
	}
</style>
