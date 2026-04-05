<script lang="ts">
	/**
	 * PaymentMethodSelector Component (FluentForms Pro 6.1.8 - Updated Jan 2026)
	 *
	 * Responsive payment method selector with touch-friendly design.
	 *
	 * @version 2.1.0 - Svelte 5 + Responsive Design
	 */

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

<div class="pms-wrap">
	<label class="pms-label" for="payment-method-{props.field.name}">
		{props.field.label || 'Payment Method'}
		{#if props.field.required}
			<span class="pms-required">*</span>
		{/if}
	</label>

	{#if props.field.help_text}
		<p class="pms-help">{props.field.help_text}</p>
	{/if}

	<div class="pms-list">
		{#each enabledMethods as method (method.id)}
			<button
				type="button"
				class="pms-method"
				data-selected={(props.value ?? '') === method.id ? '' : undefined}
				onclick={() => handleSelect(method.id)}
			>
				<input
					id="payment-method-{method.id}"
					type="radio"
					name={props.field.name}
					value={method.id}
					checked={(props.value ?? '') === method.id}
					class="sr-only"
					onchange={() => handleSelect(method.id)}
				/>

				<span class="pms-radio" data-selected={(props.value ?? '') === method.id ? '' : undefined}>
					{#if (props.value ?? '') === method.id}
						<span class="pms-radio-dot"></span>
					{/if}
				</span>

				<span class="pms-icon-wrap">
					<svg
						class="pms-icon"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						stroke-width="1.5"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d={getMethodIcon(method.icon)} />
					</svg>
				</span>

				<div class="pms-method-info">
					<span class="pms-method-name">{method.name}</span>
					{#if method.description}
						<span class="pms-method-desc">{method.description}</span>
					{/if}
				</div>

				{#if (props.value ?? '') === method.id}
					<span class="pms-check">
						<svg
							class="pms-check-icon"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="3"
						>
							<polyline points="20 6 9 17 4 12"></polyline>
						</svg>
					</span>
				{/if}
			</button>
		{/each}
	</div>

	{#if (props.value ?? '') === 'stripe'}
		<div class="pms-info pms-info-indigo">
			<div class="pms-cards">
				{#each ['Visa', 'Mastercard', 'Amex', 'Discover'] as cardType}
					<span class="pms-card-badge">
						<svg
							class="pms-card-icon"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							stroke-width="1.5"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"
							/>
						</svg>
						{cardType}
					</span>
				{/each}
			</div>
			<p class="pms-security">
				<svg
					class="pms-sec-icon"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
					<path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
				</svg>
				<span>Secured by Stripe. Your card details are encrypted.</span>
			</p>
		</div>
	{:else if (props.value ?? '') === 'paypal'}
		<div class="pms-info pms-info-amber">
			<p class="pms-security">
				<svg
					class="pms-sec-icon"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
				</svg>
				<span>You will be redirected to PayPal to complete your payment.</span>
			</p>
		</div>
	{/if}

	{#if props.error && props.error.length > 0}
		<div class="pms-error">
			{#each props.error as err}
				<p class="pms-error-text">{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.pms-wrap {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		inline-size: 100%;
		max-inline-size: 100%;
		margin-inline: auto;
		padding: 0.75rem;
		padding-block-end: env(safe-area-inset-bottom);

		@media (min-width: 640px) {
			gap: 0.75rem;
			max-inline-size: 32rem;
			padding: 1rem;
		}
		@media (min-width: 768px) {
			max-inline-size: 36rem;
			padding: 1.5rem;
		}
		@media (min-width: 1024px) {
			max-inline-size: 42rem;
		}
	}

	.pms-label {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.35 0.01 265);

		@media (min-width: 640px) {
			font-size: var(--text-base);
		}
	}

	.pms-required {
		color: oklch(0.5 0.2 25);
		margin-inline-start: 0.25rem;
	}

	.pms-help {
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 265);
		margin: 0;

		@media (min-width: 640px) {
			font-size: var(--text-sm);
		}
	}

	.pms-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;

		@media (min-width: 640px) {
			gap: 0.75rem;
		}
	}

	.pms-method {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		inline-size: 100%;
		min-block-size: 3.5rem;
		padding: 0.75rem;
		border: 2px solid oklch(0.9 0.005 265);
		border-radius: var(--radius-lg);
		background-color: oklch(1 0 0);
		cursor: pointer;
		transition:
			border-color 200ms,
			background-color 200ms,
			transform 100ms;
		text-align: start;

		@media (min-width: 640px) {
			gap: 1rem;
			min-block-size: 4rem;
			padding: 1rem;
		}
		@media (min-width: 768px) {
			min-block-size: 4.5rem;
		}

		&:hover {
			border-color: oklch(0.7 0.005 265);
			background-color: oklch(0.97 0.005 265);
		}
		&:active {
			transform: scale(0.98);
		}
		&:focus-visible {
			outline: none;
			border-color: oklch(0.5 0.18 260);
			box-shadow: 0 0 0 2px oklch(0.5 0.18 260 / 0.2);
		}
	}

	.pms-method[data-selected] {
		border-color: oklch(0.5 0.18 260);
		background-color: oklch(0.95 0.02 260);
	}

	.pms-radio {
		flex-shrink: 0;
		inline-size: 1.25rem;
		block-size: 1.25rem;
		border-radius: 50%;
		border: 2px solid oklch(0.7 0.005 265);
		display: flex;
		align-items: center;
		justify-content: center;
		transition:
			border-color 200ms,
			background-color 200ms;

		@media (min-width: 640px) {
			inline-size: 1.5rem;
			block-size: 1.5rem;
		}
	}

	.pms-radio[data-selected] {
		border-color: oklch(0.5 0.18 260);
		background-color: oklch(0.5 0.18 260);
	}

	.pms-radio-dot {
		inline-size: 0.5rem;
		block-size: 0.5rem;
		background-color: oklch(1 0 0);
		border-radius: 50%;

		@media (min-width: 640px) {
			inline-size: 0.625rem;
			block-size: 0.625rem;
		}
	}

	.pms-icon-wrap {
		flex-shrink: 0;
		inline-size: 2.5rem;
		block-size: 2.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: oklch(0.95 0.005 265);
		border-radius: var(--radius-lg);

		@media (min-width: 640px) {
			inline-size: 3rem;
			block-size: 3rem;
		}
	}

	.pms-icon {
		inline-size: 1.25rem;
		block-size: 1.25rem;
		color: oklch(0.45 0.005 265);

		@media (min-width: 640px) {
			inline-size: 1.5rem;
			block-size: 1.5rem;
		}
	}

	.pms-method-info {
		flex: 1;
		min-inline-size: 0;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;

		@media (min-width: 640px) {
			gap: 0.25rem;
		}
	}

	.pms-method-name {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;

		@media (min-width: 640px) {
			font-size: var(--text-base);
		}
	}

	.pms-method-desc {
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 265);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;

		@media (min-width: 640px) {
			font-size: var(--text-sm);
		}
	}

	.pms-check {
		flex-shrink: 0;
		inline-size: 1.5rem;
		block-size: 1.5rem;
		background-color: oklch(0.5 0.18 260);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;

		@media (min-width: 640px) {
			inline-size: 1.75rem;
			block-size: 1.75rem;
		}
	}

	.pms-check-icon {
		inline-size: 0.875rem;
		block-size: 0.875rem;
		color: oklch(1 0 0);

		@media (min-width: 640px) {
			inline-size: 1rem;
			block-size: 1rem;
		}
	}

	.pms-info {
		margin-block-start: 0.5rem;
		padding: 0.75rem;
		background-color: oklch(0.97 0.005 265);
		border-radius: var(--radius-lg);
		border-inline-start-width: 4px;
		border-inline-start-style: solid;

		@media (min-width: 640px) {
			margin-block-start: 0.75rem;
			padding: 1rem;
		}
	}

	.pms-info-indigo {
		border-inline-start-color: oklch(0.5 0.15 260);
	}
	.pms-info-amber {
		border-inline-start-color: oklch(0.7 0.12 85);
	}

	.pms-cards {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-block-end: 0.75rem;

		@media (min-width: 640px) {
			gap: 0.75rem;
			margin-block-end: 1rem;
		}
	}

	.pms-card-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 265);
		padding-inline: 0.5rem;
		padding-block: 0.25rem;
		background-color: oklch(1 0 0);
		border-radius: var(--radius-sm);
		border: 1px solid oklch(0.9 0.005 265);

		@media (min-width: 640px) {
			font-size: var(--text-sm);
			padding-inline: 0.75rem;
			padding-block: 0.375rem;
		}
	}

	.pms-card-icon {
		inline-size: 0.875rem;
		block-size: 0.875rem;

		@media (min-width: 640px) {
			inline-size: 1rem;
			block-size: 1rem;
		}
	}

	.pms-security {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 265);
		margin: 0;

		@media (min-width: 640px) {
			font-size: var(--text-sm);
		}
	}

	.pms-sec-icon {
		inline-size: 1rem;
		block-size: 1rem;
		color: oklch(0.45 0.15 160);
		flex-shrink: 0;

		@media (min-width: 640px) {
			inline-size: 1.25rem;
			block-size: 1.25rem;
		}
	}

	.pms-error {
		margin-block-start: 0.5rem;
		padding: 0.75rem;
		background-color: oklch(0.95 0.02 25);
		border: 1px solid oklch(0.85 0.05 25);
		border-radius: var(--radius-lg);

		@media (min-width: 640px) {
			padding: 1rem;
		}
	}

	.pms-error-text {
		font-size: var(--text-xs);
		color: oklch(0.5 0.2 25);
		margin: 0;

		@media (min-width: 640px) {
			font-size: var(--text-sm);
		}
	}

	.sr-only {
		position: absolute;
		inline-size: 1px;
		block-size: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}
</style>
