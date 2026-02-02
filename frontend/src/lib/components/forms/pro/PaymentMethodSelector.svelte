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

	const enabledMethods = $derived((props.availableMethods ?? defaultMethods).filter((m) => m.enabled));

	function handleSelect(methodId: string) {
		props.onchange?.(methodId);
	}

	function getMethodIcon(iconType: string): string {
		const icons: Record<string, string> = {
			card: 'M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z',
			paypal: 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15h-2v-6h2v6zm0-8h-2V7h2v2z',
			square: 'M4 4h16v16H4V4z',
			razorpay: 'M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z',
			mollie: 'M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5'
		};
		return icons[iconType] || icons.card;
	}
</script>

<!-- Responsive Payment Method Selector - Mobile-first design -->
<div class="flex flex-col gap-2 sm:gap-3 w-full max-w-full sm:max-w-lg md:max-w-xl lg:max-w-2xl mx-auto p-3 sm:p-4 md:p-6 pb-[env(safe-area-inset-bottom)]">
	<label
		class="text-sm sm:text-base font-medium text-gray-700"
		for="payment-method-{props.field.name}"
	>
		{props.field.label || 'Payment Method'}
		{#if props.field.required}
			<span class="text-red-600 ml-1">*</span>
		{/if}
	</label>

	{#if props.field.help_text}
		<p class="text-xs sm:text-sm text-gray-500 m-0">{props.field.help_text}</p>
	{/if}

	<!-- Payment Methods List - Touch-friendly buttons -->
	<div class="flex flex-col gap-2 sm:gap-3">
		{#each enabledMethods as method (method.id)}
			<button
				type="button"
				class="flex items-center gap-3 sm:gap-4 w-full min-h-[56px] sm:min-h-[64px] md:min-h-[72px] p-3 sm:p-4 border-2 rounded-lg bg-white cursor-pointer transition-all duration-200 text-left touch-manipulation active:scale-[0.98] hover:border-gray-400 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
				class:border-blue-600={(props.value ?? '') === method.id}
				class:bg-blue-50={(props.value ?? '') === method.id}
				class:border-gray-200={value !== method.id}
				onclick={() => handleSelect(method.id)}
			>
				<!-- Hidden radio for accessibility -->
				<input
					id="payment-method-{method.id}"
					type="radio"
					name={props.field.name}
					value={method.id}
					checked={(props.value ?? '') === method.id}
					class="sr-only"
					onchange={() => handleSelect(method.id)}
				/>

				<!-- Custom radio indicator -->
				<span
					class="flex-shrink-0 w-5 h-5 sm:w-6 sm:h-6 rounded-full border-2 flex items-center justify-center transition-colors"
					class:border-blue-600={(props.value ?? '') === method.id}
					class:bg-blue-600={(props.value ?? '') === method.id}
					class:border-gray-300={value !== method.id}
				>
					{#if (props.value ?? '') === method.id}
						<span class="w-2 h-2 sm:w-2.5 sm:h-2.5 bg-white rounded-full"></span>
					{/if}
				</span>

				<!-- Method Icon -->
				<span class="flex-shrink-0 w-10 h-10 sm:w-12 sm:h-12 flex items-center justify-center bg-gray-100 rounded-lg">
					<svg
						class="w-5 h-5 sm:w-6 sm:h-6 text-gray-600"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						stroke-width="1.5"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d={getMethodIcon(method.icon)} />
					</svg>
				</span>

				<!-- Method Info -->
				<div class="flex-1 min-w-0 flex flex-col gap-0.5 sm:gap-1">
					<span class="text-sm sm:text-base font-medium text-gray-900 truncate">
						{method.name}
					</span>
					{#if method.description}
						<span class="text-xs sm:text-sm text-gray-500 truncate">
							{method.description}
						</span>
					{/if}
				</div>

				<!-- Checkmark for selected -->
				{#if (props.value ?? '') === method.id}
					<span class="flex-shrink-0 w-6 h-6 sm:w-7 sm:h-7 bg-blue-600 rounded-full flex items-center justify-center">
						<svg
							class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-white"
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

	<!-- Payment method specific info - Responsive cards -->
	{#if (props.value ?? '') === 'stripe'}
		<div class="mt-2 sm:mt-3 p-3 sm:p-4 bg-gray-50 rounded-lg border-l-4 border-indigo-500">
			<!-- Card Icons - Responsive grid -->
			<div class="flex flex-wrap gap-2 sm:gap-3 mb-3 sm:mb-4">
				{#each ['Visa', 'Mastercard', 'Amex', 'Discover'] as cardType}
					<span class="inline-flex items-center gap-1.5 text-xs sm:text-sm text-gray-600 px-2 sm:px-3 py-1 sm:py-1.5 bg-white rounded border border-gray-200">
						<svg class="w-3.5 h-3.5 sm:w-4 sm:h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
							<path stroke-linecap="round" stroke-linejoin="round" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
						</svg>
						{cardType}
					</span>
				{/each}
			</div>
			<!-- Security Note -->
			<p class="flex items-center gap-2 text-xs sm:text-sm text-gray-600 m-0">
				<svg
					class="w-4 h-4 sm:w-5 sm:h-5 text-emerald-600 flex-shrink-0"
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
		<div class="mt-2 sm:mt-3 p-3 sm:p-4 bg-gray-50 rounded-lg border-l-4 border-amber-500">
			<p class="flex items-center gap-2 text-xs sm:text-sm text-gray-600 m-0">
				<svg
					class="w-4 h-4 sm:w-5 sm:h-5 text-emerald-600 flex-shrink-0"
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

	<!-- Error Messages -->
	{#if props.error && error.length > 0}
		<div class="mt-2 p-3 sm:p-4 bg-red-50 border border-red-200 rounded-lg">
			{#each props.error as err}
				<p class="text-xs sm:text-sm text-red-600 m-0">{err}</p>
			{/each}
		</div>
	{/if}
</div>
