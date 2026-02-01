<script lang="ts">
	/**
	 * PaymentSummary Component (FluentForms Pro 6.1.8 - Updated Jan 2026)
	 *
	 * Responsive order summary with touch-friendly design.
	 *
	 * @version 2.1.0 - Svelte 5 + Responsive Design
	 */

	interface LineItem {
		id: string;
		name: string;
		price: number;
		quantity: number;
		subtotal: number;
	}

	interface Props {
		items: LineItem[];
		currency?: string;
		discount?: {
			code: string;
			type: 'percentage' | 'fixed';
			value: number;
			amount: number;
		} | null;
		tax?: {
			rate: number;
			amount: number;
		} | null;
		showBreakdown?: boolean;
	}

	let {
		items = [],
		currency = 'USD',
		discount = null,
		tax = null,
		showBreakdown = true
	}: Props = $props();

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}

	const subtotal = $derived(items.reduce((sum, item) => sum + item.subtotal, 0));

	const discountAmount = $derived(discount?.amount || 0);

	const taxableAmount = $derived(subtotal - discountAmount);

	const taxAmount = $derived(tax?.amount || 0);

	const total = $derived(taxableAmount + taxAmount);
</script>

<!-- Responsive Payment Summary Component - Mobile-first design -->
<div class="w-full max-w-full sm:max-w-lg md:max-w-xl lg:max-w-2xl mx-auto bg-white border border-gray-200 rounded-xl sm:rounded-2xl overflow-hidden shadow-sm pb-[env(safe-area-inset-bottom)]">
	<!-- Summary Header -->
	<div class="flex items-center gap-2 sm:gap-3 p-3 sm:p-4 md:p-5 bg-gray-50 border-b border-gray-200">
		<svg
			class="w-5 h-5 sm:w-6 sm:h-6 text-gray-700 flex-shrink-0"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
		>
			<circle cx="9" cy="21" r="1"></circle>
			<circle cx="20" cy="21" r="1"></circle>
			<path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"></path>
		</svg>
		<span class="text-sm sm:text-base md:text-lg font-semibold text-gray-900">Order Summary</span>
	</div>

	<!-- Line Items - Responsive list -->
	{#if showBreakdown && items.length > 0}
		<div class="p-3 sm:p-4 md:p-5 border-b border-gray-200">
			{#each items as item, index (item.id)}
				<div
					class="flex justify-between items-center py-2.5 sm:py-3 min-h-[44px]"
					class:border-b={index < items.length - 1}
					class:border-dashed={index < items.length - 1}
					class:border-gray-200={index < items.length - 1}
				>
					<div class="flex items-center gap-2 sm:gap-3 flex-1 min-w-0">
						<span class="text-sm sm:text-base text-gray-700 truncate">{item.name}</span>
						{#if item.quantity > 1}
							<span class="text-xs sm:text-sm text-gray-500 px-1.5 sm:px-2 py-0.5 sm:py-1 bg-gray-100 rounded flex-shrink-0">
								x{item.quantity}
							</span>
						{/if}
					</div>
					<span class="text-sm sm:text-base font-medium text-gray-900 ml-3 flex-shrink-0">
						{formatCurrency(item.subtotal)}
					</span>
				</div>
			{/each}
		</div>
	{/if}

	<!-- Calculations Section -->
	<div class="p-3 sm:p-4 md:p-5 border-b border-gray-200 space-y-2 sm:space-y-3">
		<!-- Subtotal Row -->
		<div class="flex justify-between items-center py-1 sm:py-1.5 min-h-[36px] sm:min-h-[40px]">
			<span class="text-sm sm:text-base text-gray-500">Subtotal</span>
			<span class="text-sm sm:text-base text-gray-700">{formatCurrency(subtotal)}</span>
		</div>

		<!-- Discount Row -->
		{#if discount}
			<div class="flex justify-between items-center py-1 sm:py-1.5 min-h-[36px] sm:min-h-[40px]">
				<span class="flex items-center gap-1.5 sm:gap-2 text-sm sm:text-base text-emerald-600">
					<svg
						class="w-3.5 h-3.5 sm:w-4 sm:h-4 flex-shrink-0"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<polyline points="20 12 20 22 4 22 4 12"></polyline>
						<rect x="2" y="7" width="20" height="5"></rect>
						<line x1="12" y1="22" x2="12" y2="7"></line>
						<path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z"></path>
						<path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z"></path>
					</svg>
					<span class="truncate">Discount ({discount.code})</span>
					{#if discount.type === 'percentage'}
						<span class="text-xs px-1.5 py-0.5 bg-emerald-100 text-emerald-700 rounded flex-shrink-0">
							-{discount.value}%
						</span>
					{/if}
				</span>
				<span class="text-sm sm:text-base font-medium text-emerald-600 flex-shrink-0">
					-{formatCurrency(discountAmount)}
				</span>
			</div>
		{/if}

		<!-- Tax Row -->
		{#if tax}
			<div class="flex justify-between items-center py-1 sm:py-1.5 min-h-[36px] sm:min-h-[40px]">
				<span class="text-sm sm:text-base text-gray-500">
					Tax ({tax.rate}%)
				</span>
				<span class="text-sm sm:text-base text-gray-700">{formatCurrency(taxAmount)}</span>
			</div>
		{/if}
	</div>

	<!-- Total Section - Prominent display -->
	<div class="flex justify-between items-center p-4 sm:p-5 md:p-6 bg-blue-900 text-white">
		<span class="text-base sm:text-lg font-medium">Total</span>
		<span class="text-xl sm:text-2xl md:text-3xl font-bold">{formatCurrency(total)}</span>
	</div>

	<!-- Savings Badge - Touch-friendly -->
	{#if discount}
		<div class="flex items-center justify-center gap-2 p-3 sm:p-4 bg-emerald-50 text-emerald-700 min-h-[44px]">
			<svg
				class="w-4 h-4 sm:w-5 sm:h-5 flex-shrink-0"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
			>
				<polyline points="20 6 9 17 4 12"></polyline>
			</svg>
			<span class="text-sm sm:text-base font-medium">
				You saved {formatCurrency(discountAmount)}!
			</span>
		</div>
	{/if}
</div>
