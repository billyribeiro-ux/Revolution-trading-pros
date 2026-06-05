<script lang="ts">
	/**
	 * PaymentMethodSelector Component (FluentForms Pro 6.1.8 - Updated Jan 2026)
	 *
	 * Responsive payment method selector with touch-friendly design.
	 *
	 * @version 2.1.0 - Svelte 5 + Responsive Design
	 */

	import type { FormField } from '$lib/api/forms';
	import Icon from '$lib/components/Icon.svelte';

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
			icon: 'card',
			description: 'Powered by Stripe',
			enabled: true
		},
		{
			id: 'paypal',
			name: 'PayPal',
			icon: 'paypal',
			description: 'Pay with PayPal account',
			enabled: true
		},
		{
			id: 'square',
			name: 'Square',
			icon: 'square',
			description: 'Powered by Square',
			enabled: false
		},
		{
			id: 'razorpay',
			name: 'Razorpay',
			icon: 'razorpay',
			description: 'Popular in India',
			enabled: false
		},
		{
			id: 'mollie',
			name: 'Mollie',
			icon: 'mollie',
			description: 'Popular in Europe',
			enabled: false
		}
	];

	let props: Props = $props();

	const enabledMethods = $derived(
		(props.availableMethods ?? defaultMethods).filter((m) => m.enabled)
	);

	function handleSelect(methodId: string) {
		props.onchange?.(methodId);
	}

	function isSelected(methodId: string): boolean {
		return (props.value ?? '') === methodId;
	}

	function getMethodIcon(iconType: string): string {
		const icons: Record<string, string> = {
			card: 'M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z',
			paypal:
				'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15h-2v-6h2v6zm0-8h-2V7h2v2z',
			square: 'M4 4h16v16H4V4z',
			razorpay:
				'M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z',
			mollie: 'M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5'
		};
		return icons[iconType] || icons.card;
	}
</script>

<!-- Responsive Payment Method Selector - Mobile-first design -->
<div class="payment-selector">
	<label class="field-label" for="payment-method-{props.field.name}">
		{props.field.label || 'Payment Method'}
		{#if props.field.required}
			<span class="required-indicator">*</span>
		{/if}
	</label>

	{#if props.field.help_text}
		<p class="field-help">{props.field.help_text}</p>
	{/if}

	<!-- Payment Methods List - Touch-friendly buttons -->
	<div class="methods-list">
		{#each enabledMethods as method (method.id)}
			<button
				type="button"
				class={['method-button', isSelected(method.id) && 'method-button--selected']}
				onclick={() => handleSelect(method.id)}
			>
				<!-- Hidden radio for accessibility -->
				<input
					id="payment-method-{method.id}"
					type="radio"
					name={props.field.name}
					value={method.id}
					checked={isSelected(method.id)}
					class="visually-hidden"
					onchange={() => handleSelect(method.id)}
				/>

				<!-- Custom radio indicator -->
				<span class={['radio-indicator', isSelected(method.id) && 'radio-indicator--selected']}>
					{#if isSelected(method.id)}
						<span class="radio-indicator__dot"></span>
					{/if}
				</span>

				<!-- Method Icon -->
				<span class="method-icon">
					<svg
						class="method-icon__svg"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						stroke-width="1.5"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d={getMethodIcon(method.icon)} />
					</svg>
				</span>

				<!-- Method Info -->
				<div class="method-info">
					<span class="method-name">{method.name}</span>
					{#if method.description}
						<span class="method-description">{method.description}</span>
					{/if}
				</div>

				<!-- Checkmark for selected -->
				{#if isSelected(method.id)}
					<span class="selected-check">
						<Icon name="IconCheck" size={14} />
					</span>
				{/if}
			</button>
		{/each}
	</div>

	<!-- Payment method specific info - Responsive cards -->
	{#if (props.value ?? '') === 'stripe'}
		<div class="method-details method-details--stripe">
			<!-- Card Icons - Responsive grid -->
			<div class="brand-list">
				{#each ['Visa', 'Mastercard', 'Amex', 'Discover'] as cardType (cardType)}
					<span class="brand-badge">
						<Icon name="IconCreditCard" size={14} />
						{cardType}
					</span>
				{/each}
			</div>
			<!-- Security Note -->
			<p class="security-note">
				<span class="security-note__icon">
					<Icon name="IconLock" size={16} />
				</span>
				<span>Secured by Stripe. Your card details are encrypted.</span>
			</p>
		</div>
	{:else if (props.value ?? '') === 'paypal'}
		<div class="method-details method-details--paypal">
			<p class="security-note">
				<span class="security-note__icon">
					<Icon name="IconShield" size={16} />
				</span>
				<span>You will be redirected to PayPal to complete your payment.</span>
			</p>
		</div>
	{/if}

	<!-- Error Messages -->
	{#if props.error && props.error.length > 0}
		<div class="error-panel">
			{#each props.error as err (err)}
				<p class="error-message">{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.payment-selector {
		display: flex;
		width: 100%;
		max-width: 100%;
		flex-direction: column;
		gap: 0.5rem;
		margin-inline: auto;
		padding: 0.75rem;
		padding-bottom: calc(0.75rem + env(safe-area-inset-bottom));
	}

	.field-label {
		color: #374151;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.required-indicator {
		margin-left: 0.25rem;
		color: #dc2626;
	}

	.field-help {
		margin: 0;
		color: #6b7280;
		font-size: 0.75rem;
	}

	.methods-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.method-button {
		display: flex;
		width: 100%;
		min-height: 56px;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		border: 2px solid #e5e7eb;
		border-radius: 0.5rem;
		background: #fff;
		cursor: pointer;
		text-align: left;
		touch-action: manipulation;
		transition:
			background-color 0.2s ease,
			border-color 0.2s ease,
			box-shadow 0.2s ease,
			transform 0.2s ease;
	}

	.method-button:hover {
		border-color: #9ca3af;
		background: #f9fafb;
	}

	.method-button:focus-visible {
		outline: none;
		box-shadow:
			0 0 0 2px #3b82f6,
			0 0 0 4px #fff;
	}

	.method-button:active {
		transform: scale(0.98);
	}

	.method-button--selected {
		border-color: #2563eb;
		background: #eff6ff;
	}

	.visually-hidden {
		position: absolute;
		width: 1px;
		height: 1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		clip-path: inset(50%);
	}

	.radio-indicator {
		display: flex;
		width: 1.25rem;
		height: 1.25rem;
		flex: 0 0 auto;
		align-items: center;
		justify-content: center;
		border: 2px solid #d1d5db;
		border-radius: 999px;
		transition:
			background-color 0.2s ease,
			border-color 0.2s ease;
	}

	.radio-indicator--selected {
		border-color: #2563eb;
		background: #2563eb;
	}

	.radio-indicator__dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 999px;
		background: #fff;
	}

	.method-icon {
		display: flex;
		width: 2.5rem;
		height: 2.5rem;
		flex: 0 0 auto;
		align-items: center;
		justify-content: center;
		border-radius: 0.5rem;
		background: #f3f4f6;
		color: #4b5563;
	}

	.method-icon__svg {
		width: 1.25rem;
		height: 1.25rem;
	}

	.method-info {
		display: flex;
		min-width: 0;
		flex: 1 1 auto;
		flex-direction: column;
		gap: 0.125rem;
	}

	.method-name,
	.method-description {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.method-name {
		color: #111827;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.method-description {
		color: #6b7280;
		font-size: 0.75rem;
	}

	.selected-check {
		display: flex;
		width: 1.5rem;
		height: 1.5rem;
		flex: 0 0 auto;
		align-items: center;
		justify-content: center;
		border-radius: 999px;
		background: #2563eb;
		color: #fff;
	}

	.method-details {
		margin-top: 0.5rem;
		padding: 0.75rem;
		border-left: 4px solid;
		border-radius: 0.5rem;
		background: #f9fafb;
	}

	.method-details--stripe {
		border-left-color: #6366f1;
	}

	.method-details--paypal {
		border-left-color: #f59e0b;
	}

	.brand-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.brand-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.5rem;
		border: 1px solid #e5e7eb;
		border-radius: 0.25rem;
		background: #fff;
		color: #4b5563;
		font-size: 0.75rem;
	}

	.security-note {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin: 0;
		color: #4b5563;
		font-size: 0.75rem;
	}

	.security-note__icon {
		display: flex;
		flex: 0 0 auto;
		color: #059669;
	}

	.error-panel {
		margin-top: 0.5rem;
		padding: 0.75rem;
		border: 1px solid #fecaca;
		border-radius: 0.5rem;
		background: #fef2f2;
	}

	.error-message {
		margin: 0;
		color: #dc2626;
		font-size: 0.75rem;
	}

	@media (min-width: 640px) {
		.payment-selector {
			max-width: 32rem;
			gap: 0.75rem;
			padding: 1rem;
			padding-bottom: calc(1rem + env(safe-area-inset-bottom));
		}

		.field-label {
			font-size: 1rem;
		}

		.field-help,
		.method-description,
		.security-note,
		.error-message {
			font-size: 0.875rem;
		}

		.methods-list {
			gap: 0.75rem;
		}

		.method-button {
			min-height: 64px;
			gap: 1rem;
			padding: 1rem;
		}

		.radio-indicator {
			width: 1.5rem;
			height: 1.5rem;
		}

		.radio-indicator__dot {
			width: 0.625rem;
			height: 0.625rem;
		}

		.method-icon {
			width: 3rem;
			height: 3rem;
		}

		.method-icon__svg {
			width: 1.5rem;
			height: 1.5rem;
		}

		.method-name {
			font-size: 1rem;
		}

		.selected-check {
			width: 1.75rem;
			height: 1.75rem;
		}

		.method-details {
			margin-top: 0.75rem;
			padding: 1rem;
		}

		.brand-list {
			gap: 0.75rem;
			margin-bottom: 1rem;
		}

		.brand-badge {
			padding: 0.375rem 0.75rem;
			font-size: 0.875rem;
		}

		.error-panel {
			padding: 1rem;
		}
	}

	@media (min-width: 768px) {
		.payment-selector {
			max-width: 36rem;
			padding: 1.5rem;
			padding-bottom: calc(1.5rem + env(safe-area-inset-bottom));
		}

		.method-button {
			min-height: 72px;
		}
	}

	@media (min-width: 1024px) {
		.payment-selector {
			max-width: 42rem;
		}
	}
</style>
