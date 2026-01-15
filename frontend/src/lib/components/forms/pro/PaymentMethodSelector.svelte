<script lang="ts">
	import type { FormField } from '$lib/api/forms';

	interface PaymentMethod {
		id: string;
		name: string;
		icon: string;
		description?: string;
		enabled: boolean;
	}

	interface Props {
		field: FormField;
		value?: string;
		error?: string[];
		availableMethods?: PaymentMethod[];
		onchange?: (value: string) => void;
	}

	const defaultMethods: PaymentMethod[] = [
		{
			id: 'stripe',
			name: 'Credit/Debit Card',
			icon: '💳',
			description: 'Powered by Stripe',
			enabled: true
		},
		{
			id: 'paypal',
			name: 'PayPal',
			icon: '🅿️',
			description: 'Pay with PayPal account',
			enabled: true
		},
		{
			id: 'square',
			name: 'Square',
			icon: '◻️',
			description: 'Powered by Square',
			enabled: false
		},
		{
			id: 'razorpay',
			name: 'Razorpay',
			icon: '💰',
			description: 'Popular in India',
			enabled: false
		},
		{
			id: 'mollie',
			name: 'Mollie',
			icon: '🔷',
			description: 'Popular in Europe',
			enabled: false
		}
	];

	let { field, value = '', error, availableMethods = defaultMethods, onchange }: Props = $props();

	const enabledMethods = $derived(availableMethods.filter((m) => m.enabled));

	function handleSelect(methodId: string) {
		onchange?.(methodId);
	}
</script>

<div class="payment-method-selector">
	<label class="field-label" for="payment-method-{field.name}">
		{field.label || 'Payment Method'}
		{#if field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if field.help_text}
		<p class="field-help">{field.help_text}</p>
	{/if}

	<div class="methods-list">
		{#each enabledMethods as method (method.id)}
			<button
				type="button"
				class="method-option"
				class:selected={value === method.id}
				onclick={() => handleSelect(method.id)}
			>
				<input
					id="payment-method-{method.id}"
					type="radio"
					name={field.name}
					value={method.id}
					checked={value === method.id}
					class="method-radio"
					onchange={() => handleSelect(method.id)}
				/>

				<span class="method-icon">{method.icon}</span>

				<div class="method-info">
					<span class="method-name">{method.name}</span>
					{#if method.description}
						<span class="method-description">{method.description}</span>
					{/if}
				</div>

				{#if value === method.id}
					<span class="check-icon">
						<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
							<polyline points="20 6 9 17 4 12"></polyline>
						</svg>
					</span>
				{/if}
			</button>
		{/each}
	</div>

	<!-- Payment method specific info -->
	{#if value === 'stripe'}
		<div class="method-details stripe">
			<div class="card-icons">
				<span class="card-icon" title="Visa">💳 Visa</span>
				<span class="card-icon" title="Mastercard">💳 Mastercard</span>
				<span class="card-icon" title="American Express">💳 Amex</span>
				<span class="card-icon" title="Discover">💳 Discover</span>
			</div>
			<p class="security-note">
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
					<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
				</svg>
				Secured by Stripe. Your card details are encrypted.
			</p>
		</div>
	{:else if value === 'paypal'}
		<div class="method-details paypal">
			<p class="security-note">
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
				</svg>
				You will be redirected to PayPal to complete your payment.
			</p>
		</div>
	{/if}

	{#if error && error.length > 0}
		<div class="field-error">
			{#each error as err}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.payment-method-selector {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.field-label {
		font-weight: 500;
		font-size: 0.875rem;
		color: #374151;
	}

	.required {
		color: #dc2626;
		margin-left: 0.25rem;
	}

	.field-help {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.methods-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.method-option {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 1rem;
		border: 2px solid #e5e7eb;
		border-radius: 0.5rem;
		background-color: white;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
	}

	.method-option:hover {
		border-color: #d1d5db;
		background-color: #f9fafb;
	}

	.method-option.selected {
		border-color: #E6B800;
		background-color: #eff6ff;
	}

	.method-radio {
		width: 18px;
		height: 18px;
		accent-color: #E6B800;
		cursor: pointer;
	}

	.method-icon {
		font-size: 1.5rem;
		line-height: 1;
	}

	.method-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.method-name {
		font-weight: 500;
		color: #111827;
	}

	.method-description {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.check-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background-color: #E6B800;
		border-radius: 50%;
		color: white;
	}

	.method-details {
		padding: 1rem;
		background-color: #f9fafb;
		border-radius: 0.5rem;
		border-left: 3px solid #E6B800;
	}

	.method-details.stripe {
		border-left-color: #E6B800;
	}

	.method-details.paypal {
		border-left-color: #f59e0b;
	}

	.card-icons {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
		margin-bottom: 0.75rem;
	}

	.card-icon {
		font-size: 0.75rem;
		color: #6b7280;
		padding: 0.25rem 0.5rem;
		background-color: white;
		border-radius: 0.25rem;
		border: 1px solid #e5e7eb;
	}

	.security-note {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.security-note svg {
		color: #059669;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}
</style>
