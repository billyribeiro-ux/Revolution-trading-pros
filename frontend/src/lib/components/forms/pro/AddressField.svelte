<script lang="ts">
	import type { FormField } from '$lib/api/forms';

	interface AddressValue {
		address_line_1: string;
		address_line_2: string;
		city: string;
		state: string;
		zip: string;
		country: string;
	}

	interface Props {
		field: FormField;
		value?: Partial<AddressValue>;
		error?: string[];
		onchange?: (value: AddressValue) => void;
	}

	let { field, value = {}, error, onchange }: Props = $props();

	const defaultValue: AddressValue = {
		address_line_1: '',
		address_line_2: '',
		city: '',
		state: '',
		zip: '',
		country: ''
	};

	let addressData = $state<AddressValue>({ ...defaultValue });

	const showAddress2 = $derived(field.attributes?.show_address_2 !== false);
	const showCountry = $derived(field.attributes?.show_country !== false);
	const enableAutocomplete = $derived(field.attributes?.enable_autocomplete !== false);

	// Sync with prop value changes
	$effect(() => {
		addressData = { ...defaultValue, ...value };
	});

	const countries = [
		{ code: 'US', name: 'United States' },
		{ code: 'CA', name: 'Canada' },
		{ code: 'GB', name: 'United Kingdom' },
		{ code: 'AU', name: 'Australia' },
		{ code: 'DE', name: 'Germany' },
		{ code: 'FR', name: 'France' },
		{ code: 'ES', name: 'Spain' },
		{ code: 'IT', name: 'Italy' },
		{ code: 'NL', name: 'Netherlands' },
		{ code: 'BE', name: 'Belgium' },
		{ code: 'CH', name: 'Switzerland' },
		{ code: 'AT', name: 'Austria' },
		{ code: 'SE', name: 'Sweden' },
		{ code: 'NO', name: 'Norway' },
		{ code: 'DK', name: 'Denmark' },
		{ code: 'FI', name: 'Finland' },
		{ code: 'IE', name: 'Ireland' },
		{ code: 'NZ', name: 'New Zealand' },
		{ code: 'SG', name: 'Singapore' },
		{ code: 'JP', name: 'Japan' },
		{ code: 'KR', name: 'South Korea' },
		{ code: 'IN', name: 'India' },
		{ code: 'BR', name: 'Brazil' },
		{ code: 'MX', name: 'Mexico' }
	];

	function handleFieldChange(fieldName: keyof AddressValue, newValue: string) {
		addressData = { ...addressData, [fieldName]: newValue };
		onchange?.(addressData);
	}

	function getInputClasses(hasError: boolean): string {
		return `address-input ${hasError ? 'input-error' : ''}`;
	}
</script>

<div class="address-field">
	<div class="field-label">
		{field.label}
		{#if field.required}
			<span class="required">*</span>
		{/if}
	</div>

	{#if field.help_text}
		<p class="field-help">{field.help_text}</p>
	{/if}

	<div class="address-fields">
		<!-- Address Line 1 -->
		<div class="field-row full">
			<label class="sub-label" for={`${field.name}-address1`}>Address Line 1</label>
			<input
				type="text"
				id={`${field.name}-address1`}
				name={`${field.name}[address_line_1]`}
				placeholder="Street address"
				value={addressData.address_line_1}
				required={field.required}
				class={getInputClasses(!!error)}
				autocomplete={enableAutocomplete ? 'address-line1' : 'off'}
				oninput={(e: Event) => handleFieldChange('address_line_1', (e.currentTarget as HTMLInputElement).value)}
			/>
		</div>

		<!-- Address Line 2 -->
		{#if showAddress2}
			<div class="field-row full">
				<label class="sub-label" for={`${field.name}-address2`}>Address Line 2</label>
				<input
					type="text"
					id={`${field.name}-address2`}
					name={`${field.name}[address_line_2]`}
					placeholder="Apartment, suite, unit, etc. (optional)"
					value={addressData.address_line_2}
					class="address-input"
					autocomplete={enableAutocomplete ? 'address-line2' : 'off'}
					oninput={(e: Event) => handleFieldChange('address_line_2', (e.currentTarget as HTMLInputElement).value)}
				/>
			</div>
		{/if}

		<!-- City & State Row -->
		<div class="field-row half-row">
			<div class="half-field">
				<label class="sub-label" for={`${field.name}-city`}>City</label>
				<input
					type="text"
					id={`${field.name}-city`}
					name={`${field.name}[city]`}
					placeholder="City"
					value={addressData.city}
					required={field.required}
					class={getInputClasses(!!error)}
					autocomplete={enableAutocomplete ? 'address-level2' : 'off'}
					oninput={(e: Event) => handleFieldChange('city', (e.currentTarget as HTMLInputElement).value)}
				/>
			</div>
			<div class="half-field">
				<label class="sub-label" for={`${field.name}-state`}>State / Province</label>
				<input
					type="text"
					id={`${field.name}-state`}
					name={`${field.name}[state]`}
					placeholder="State / Province"
					value={addressData.state}
					required={field.required}
					class={getInputClasses(!!error)}
					autocomplete={enableAutocomplete ? 'address-level1' : 'off'}
					oninput={(e: Event) => handleFieldChange('state', (e.currentTarget as HTMLInputElement).value)}
				/>
			</div>
		</div>

		<!-- Zip & Country Row -->
		<div class="field-row half-row">
			<div class="half-field">
				<label class="sub-label" for={`${field.name}-zip`}>ZIP / Postal Code</label>
				<input
					type="text"
					id={`${field.name}-zip`}
					name={`${field.name}[zip]`}
					placeholder="ZIP / Postal Code"
					value={addressData.zip}
					required={field.required}
					class={getInputClasses(!!error)}
					autocomplete={enableAutocomplete ? 'postal-code' : 'off'}
					oninput={(e: Event) => handleFieldChange('zip', (e.currentTarget as HTMLInputElement).value)}
				/>
			</div>
			{#if showCountry}
				<div class="half-field">
					<label class="sub-label" for={`${field.name}-country`}>Country</label>
					<select
						id={`${field.name}-country`}
						name={`${field.name}[country]`}
						value={addressData.country}
						required={field.required}
						class={getInputClasses(!!error)}
						autocomplete={enableAutocomplete ? 'country' : 'off'}
						onchange={(e: Event) => handleFieldChange('country', (e.currentTarget as HTMLInputElement).value)}
					>
						<option value="">Select country</option>
						{#each countries as country}
							<option value={country.code}>{country.name}</option>
						{/each}
					</select>
				</div>
			{/if}
		</div>
	</div>

	{#if error && error.length > 0}
		<div class="field-error">
			{#each error as err}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.address-field {
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

	.address-fields {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.field-row {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.field-row.full {
		width: 100%;
	}

	.field-row.half-row {
		flex-direction: row;
		gap: 0.75rem;
	}

	.half-field {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.sub-label {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.address-input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.address-input:focus {
		outline: none;
		border-color: #2563eb;
		box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
	}

	.input-error {
		border-color: #dc2626 !important;
	}

	select.address-input {
		cursor: pointer;
		background-color: white;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}

	@media (max-width: 640px) {
		.field-row.half-row {
			flex-direction: column;
		}
	}
</style>
