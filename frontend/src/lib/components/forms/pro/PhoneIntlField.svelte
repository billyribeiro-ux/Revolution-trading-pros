<script lang="ts">
	import type { FormField } from '$lib/api/forms';
	import Icon from '$lib/components/Icon.svelte';

	interface Country {
		code: string;
		name: string;
		dialCode: string;
		flag: string;
	}

	interface PhoneValue {
		country_code: string;
		dial_code: string;
		number: string;
		full: string;
	}

	interface Props {
		field: FormField;
		value?: Partial<PhoneValue>;
		error?: string[];
		onchange?: (value: PhoneValue) => void;
	}

	let props: Props = $props();

	const countries: Country[] = [
		{ code: 'US', name: 'United States', dialCode: '+1', flag: '🇺🇸' },
		{ code: 'CA', name: 'Canada', dialCode: '+1', flag: '🇨🇦' },
		{ code: 'GB', name: 'United Kingdom', dialCode: '+44', flag: '🇬🇧' },
		{ code: 'AU', name: 'Australia', dialCode: '+61', flag: '🇦🇺' },
		{ code: 'DE', name: 'Germany', dialCode: '+49', flag: '🇩🇪' },
		{ code: 'FR', name: 'France', dialCode: '+33', flag: '🇫🇷' },
		{ code: 'ES', name: 'Spain', dialCode: '+34', flag: '🇪🇸' },
		{ code: 'IT', name: 'Italy', dialCode: '+39', flag: '🇮🇹' },
		{ code: 'NL', name: 'Netherlands', dialCode: '+31', flag: '🇳🇱' },
		{ code: 'BE', name: 'Belgium', dialCode: '+32', flag: '🇧🇪' },
		{ code: 'CH', name: 'Switzerland', dialCode: '+41', flag: '🇨🇭' },
		{ code: 'AT', name: 'Austria', dialCode: '+43', flag: '🇦🇹' },
		{ code: 'SE', name: 'Sweden', dialCode: '+46', flag: '🇸🇪' },
		{ code: 'NO', name: 'Norway', dialCode: '+47', flag: '🇳🇴' },
		{ code: 'DK', name: 'Denmark', dialCode: '+45', flag: '🇩🇰' },
		{ code: 'FI', name: 'Finland', dialCode: '+358', flag: '🇫🇮' },
		{ code: 'IE', name: 'Ireland', dialCode: '+353', flag: '🇮🇪' },
		{ code: 'NZ', name: 'New Zealand', dialCode: '+64', flag: '🇳🇿' },
		{ code: 'SG', name: 'Singapore', dialCode: '+65', flag: '🇸🇬' },
		{ code: 'JP', name: 'Japan', dialCode: '+81', flag: '🇯🇵' },
		{ code: 'KR', name: 'South Korea', dialCode: '+82', flag: '🇰🇷' },
		{ code: 'CN', name: 'China', dialCode: '+86', flag: '🇨🇳' },
		{ code: 'IN', name: 'India', dialCode: '+91', flag: '🇮🇳' },
		{ code: 'BR', name: 'Brazil', dialCode: '+55', flag: '🇧🇷' },
		{ code: 'MX', name: 'Mexico', dialCode: '+52', flag: '🇲🇽' },
		{ code: 'AR', name: 'Argentina', dialCode: '+54', flag: '🇦🇷' },
		{ code: 'ZA', name: 'South Africa', dialCode: '+27', flag: '🇿🇦' },
		{ code: 'AE', name: 'United Arab Emirates', dialCode: '+971', flag: '🇦🇪' },
		{ code: 'SA', name: 'Saudi Arabia', dialCode: '+966', flag: '🇸🇦' },
		{ code: 'IL', name: 'Israel', dialCode: '+972', flag: '🇮🇱' },
		{ code: 'PH', name: 'Philippines', dialCode: '+63', flag: '🇵🇭' },
		{ code: 'TH', name: 'Thailand', dialCode: '+66', flag: '🇹🇭' },
		{ code: 'MY', name: 'Malaysia', dialCode: '+60', flag: '🇲🇾' },
		{ code: 'ID', name: 'Indonesia', dialCode: '+62', flag: '🇮🇩' },
		{ code: 'VN', name: 'Vietnam', dialCode: '+84', flag: '🇻🇳' },
		{ code: 'PK', name: 'Pakistan', dialCode: '+92', flag: '🇵🇰' },
		{ code: 'BD', name: 'Bangladesh', dialCode: '+880', flag: '🇧🇩' },
		{ code: 'NG', name: 'Nigeria', dialCode: '+234', flag: '🇳🇬' },
		{ code: 'EG', name: 'Egypt', dialCode: '+20', flag: '🇪🇬' },
		{ code: 'RU', name: 'Russia', dialCode: '+7', flag: '🇷🇺' },
		{ code: 'PL', name: 'Poland', dialCode: '+48', flag: '🇵🇱' },
		{ code: 'UA', name: 'Ukraine', dialCode: '+380', flag: '🇺🇦' },
		{ code: 'TR', name: 'Turkey', dialCode: '+90', flag: '🇹🇷' },
		{ code: 'GR', name: 'Greece', dialCode: '+30', flag: '🇬🇷' },
		{ code: 'PT', name: 'Portugal', dialCode: '+351', flag: '🇵🇹' },
		{ code: 'CZ', name: 'Czech Republic', dialCode: '+420', flag: '🇨🇿' },
		{ code: 'RO', name: 'Romania', dialCode: '+40', flag: '🇷🇴' },
		{ code: 'HU', name: 'Hungary', dialCode: '+36', flag: '🇭🇺' },
		{ code: 'CL', name: 'Chile', dialCode: '+56', flag: '🇨🇱' },
		{ code: 'CO', name: 'Colombia', dialCode: '+57', flag: '🇨🇴' }
	];

	let showDropdown = $state(false);
	let searchQuery = $state('');

	const defaultCountry = $derived(
		typeof props.field.attributes?.default_country === 'string'
			? (props.field.attributes.default_country as string)
			: 'US'
	);
	// Writable $derived — `selectCountry` can override locally, and a prop
	// change re-syncs the selection.
	let selectedCountry = $derived<Country>(
		countries.find((c) => c.code === (props.value?.country_code || defaultCountry)) || countries[0]
	);
	// Writable $derived — `handlePhoneInput` can override locally while the
	// user types; a prop change re-syncs the displayed value.
	let phoneNumber = $derived<string>(props.value?.number || '');

	const filteredCountries = $derived(
		searchQuery
			? countries.filter(
					(c) =>
						c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
						c.dialCode.includes(searchQuery) ||
						c.code.toLowerCase().includes(searchQuery.toLowerCase())
				)
			: countries
	);

	function selectCountry(country: Country) {
		selectedCountry = country;
		showDropdown = false;
		searchQuery = '';
		emitChange();
	}

	function handlePhoneInput(event: Event) {
		const target = event.target as HTMLInputElement;
		// Only allow numbers, spaces, and dashes
		phoneNumber = target.value.replace(/[^\d\s\-()]/g, '');
		emitChange();
	}

	function emitChange() {
		const fullNumber = `${selectedCountry.dialCode}${phoneNumber.replace(/\D/g, '')}`;
		props.onchange?.({
			country_code: selectedCountry.code,
			dial_code: selectedCountry.dialCode,
			number: phoneNumber,
			full: fullNumber
		});
	}

	function toggleDropdown() {
		showDropdown = !showDropdown;
		if (showDropdown) {
			searchQuery = '';
		}
	}

	function handleDocumentClick(event: MouseEvent) {
		if (!showDropdown) return;

		const target = event.target as HTMLElement;
		if (!target.closest('.country-selector')) {
			showDropdown = false;
		}
	}
</script>

<svelte:document onclick={handleDocumentClick} />

<div class="phone-intl-field">
	<label class="field-label" for={`field-${props.field.name}`}>
		{props.field.label}
		{#if props.field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if props.field.help_text}
		<p class="field-help">{props.field.help_text}</p>
	{/if}

	<div class="phone-input-wrapper">
		<div class="country-selector">
			<button type="button" class="country-button" onclick={toggleDropdown}>
				<span class="country-flag">{selectedCountry.flag}</span>
				<span class="dial-code">{selectedCountry.dialCode}</span>
				<Icon name="IconChevronDown" size={12} />
			</button>

			{#if showDropdown}
				<div class="country-dropdown">
					<div class="search-wrapper">
						<input
							type="text"
							placeholder="Search country..."
							bind:value={searchQuery}
							class="country-search"
							onclick={(e: MouseEvent) => e.stopPropagation()}
						/>
					</div>
					<div class="country-list">
						{#each filteredCountries as country (country.code)}
							<button
								type="button"
								class={['country-option', { selected: selectedCountry.code === country.code }]}
								onclick={(e: MouseEvent) => {
									e.stopPropagation();
									selectCountry(country);
								}}
							>
								<span class="country-flag">{country.flag}</span>
								<span class="country-name">{country.name}</span>
								<span class="country-dial">{country.dialCode}</span>
							</button>
						{/each}
						{#if filteredCountries.length === 0}
							<div class="no-results">No countries found</div>
						{/if}
					</div>
				</div>
			{/if}
		</div>

		<input
			type="tel"
			id={`field-${props.field.name}`}
			name={props.field.name}
			placeholder={props.field.placeholder || 'Phone number'}
			value={phoneNumber}
			required={props.field.required}
			class={['phone-input', { 'input-error': props.error && props.error.length > 0 }]}
			oninput={handlePhoneInput}
		/>
	</div>

	<input
		type="hidden"
		name={`${props.field.name}_full`}
		value={`${selectedCountry.dialCode}${phoneNumber.replace(/\D/g, '')}`}
	/>
	<input type="hidden" name={`${props.field.name}_country`} value={selectedCountry.code} />

	{#if props.error && props.error.length > 0}
		<div class="field-error">
			{#each props.error as err (err)}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.phone-intl-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
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

	.phone-input-wrapper {
		display: flex;
		gap: 0;
	}

	.country-selector {
		position: relative;
	}

	/* 2026 Mobile-First: 44px touch targets */
	.country-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		min-height: 44px; /* Touch target */
		border: 1px solid #d1d5db;
		border-right: none;
		border-radius: 0.375rem 0 0 0.375rem;
		background-color: #f9fafb;
		cursor: pointer;
		font-size: 1rem; /* 16px prevents iOS zoom */
		transition: all 0.2s;
		touch-action: manipulation;
	}

	.country-button:hover {
		background-color: #f3f4f6;
	}

	.country-flag {
		font-size: 1.125rem;
		line-height: 1;
	}

	.dial-code {
		color: #374151;
		font-weight: 500;
	}

	.country-dropdown {
		position: absolute;
		top: 100%;
		left: 0;
		width: 280px;
		max-height: 300px;
		background-color: white;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
		z-index: 100;
		margin-top: 0.25rem;
		overflow: hidden;
	}

	.search-wrapper {
		padding: 0.5rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.country-search {
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 0.25rem;
		font-size: 1rem; /* 16px prevents iOS zoom */
		min-height: 44px; /* Touch target */
		touch-action: manipulation;
	}

	.country-search:focus {
		outline: none;
		border-color: #2563eb;
	}

	.country-list {
		max-height: 220px;
		overflow-y: auto;
	}

	.country-option {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem 1rem;
		min-height: 44px; /* Touch target */
		border: none;
		background-color: white;
		cursor: pointer;
		text-align: left;
		font-size: 1rem; /* 16px for readability */
		transition: background-color 0.15s;
		touch-action: manipulation;
	}

	.country-option:hover {
		background-color: #f3f4f6;
	}

	.country-option.selected {
		background-color: #eff6ff;
	}

	.country-name {
		flex: 1;
		color: #374151;
	}

	.country-dial {
		color: #6b7280;
		font-size: 0.75rem;
	}

	.no-results {
		padding: 1rem;
		text-align: center;
		color: #6b7280;
		font-size: 0.875rem;
	}

	.phone-input {
		flex: 1;
		padding: 0.75rem 1rem;
		min-height: 44px; /* Touch target */
		border: 1px solid #d1d5db;
		border-radius: 0 0.375rem 0.375rem 0;
		font-size: 1rem; /* 16px prevents iOS zoom */
		transition: all 0.2s;
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
	}

	.phone-input:focus {
		outline: none;
		border-color: #2563eb;
		box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
	}

	.phone-input.input-error {
		border-color: #dc2626;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}
</style>
