<script lang="ts">
	/**
	 * GeolocationAddress Component (FluentForms 6.1.0 Pro - August 2025)
	 *
	 * Address field with HTML5 geolocation support for autocomplete.
	 * Location latitude and longitude are now available for submission.
	 */

	interface AddressValue {
		address_line_1: string;
		address_line_2: string;
		city: string;
		state: string;
		zip: string;
		country: string;
		latitude?: number;
		longitude?: number;
		formatted_address?: string;
	}

	interface Props {
		name: string;
		value?: AddressValue;
		label?: string;
		required?: boolean;
		disabled?: boolean;
		showGeolocation?: boolean;
		autoDetect?: boolean;
		showMap?: boolean;
		error?: string;
		onchange?: (value: AddressValue) => void;
	}

	let {
		name,
		value = {
			address_line_1: '',
			address_line_2: '',
			city: '',
			state: '',
			zip: '',
			country: ''
		},
		label = 'Address',
		required = false,
		disabled = false,
		showGeolocation = true,
		autoDetect = false,
		showMap = false,
		error = '',
		onchange
	}: Props = $props();

	let addressValue = $state<AddressValue>({
		address_line_1: '',
		address_line_2: '',
		city: '',
		state: '',
		zip: '',
		country: ''
	});
	let isDetecting = $state(false);
	let geolocationError = $state('');
	let geolocationSupported = $state(typeof navigator !== 'undefined' && 'geolocation' in navigator);

	// Sync addressValue with value prop
	$effect(() => {
		if (value !== undefined) addressValue = { ...value };
	});

	// Auto-detect location on mount if enabled
	$effect(() => {
		if (autoDetect && geolocationSupported && !addressValue.latitude) {
			detectLocation();
		}
	});

	function handleInputChange(field: keyof AddressValue, inputValue: string) {
		addressValue = { ...addressValue, [field]: inputValue };
		notifyChange();
	}

	function notifyChange() {
		if (onchange) {
			onchange(addressValue);
		}
	}

	async function detectLocation() {
		if (!geolocationSupported || disabled) return;

		isDetecting = true;
		geolocationError = '';

		try {
			const position = await new Promise<GeolocationPosition>((resolve, reject) => {
				navigator.geolocation.getCurrentPosition(resolve, reject, {
					enableHighAccuracy: true,
					timeout: 10000,
					maximumAge: 0
				});
			});

			const { latitude, longitude } = position.coords;

			addressValue = {
				...addressValue,
				latitude,
				longitude
			};

			// Try to reverse geocode
			await reverseGeocode(latitude, longitude);
		} catch (err) {
			const error = err as GeolocationPositionError;
			switch (error.code) {
				case error.PERMISSION_DENIED:
					geolocationError = 'Location access denied. Please enable location permissions.';
					break;
				case error.POSITION_UNAVAILABLE:
					geolocationError = 'Location information unavailable.';
					break;
				case error.TIMEOUT:
					geolocationError = 'Location request timed out.';
					break;
				default:
					geolocationError = 'Unable to detect location.';
			}
		} finally {
			isDetecting = false;
		}
	}

	async function reverseGeocode(lat: number, lng: number) {
		try {
			// Using OpenStreetMap Nominatim (free, no API key needed)
			const response = await fetch(
				`https://nominatim.openstreetmap.org/reverse?format=json&lat=${lat}&lon=${lng}&addressdetails=1`,
				{
					headers: {
						'Accept-Language': 'en'
					}
				}
			);

			if (!response.ok) return;

			const data = await response.json();

			if (data.address) {
				addressValue = {
					...addressValue,
					address_line_1: data.address.road || data.address.street || '',
					city: data.address.city || data.address.town || data.address.village || '',
					state: data.address.state || data.address.county || '',
					zip: data.address.postcode || '',
					country: data.address.country || '',
					formatted_address: data.display_name || ''
				};
				notifyChange();
			}
		} catch {
			// Silently fail reverse geocoding
		}
	}

	function clearLocation() {
		addressValue = {
			...addressValue,
			latitude: undefined,
			longitude: undefined,
			formatted_address: undefined
		};
		notifyChange();
	}
</script>

<fieldset class="geolocation-address" class:disabled class:has-error={error}>
	<legend class="address-label">
		{label}
		{#if required}
			<span class="required-marker">*</span>
		{/if}
	</legend>

	{#if showGeolocation && geolocationSupported}
		<div class="geolocation-controls">
			<button
				type="button"
				class="detect-location-btn"
				onclick={detectLocation}
				disabled={disabled || isDetecting}
			>
				{#if isDetecting}
					<span class="spinner"></span>
					Detecting...
				{:else}
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10"></circle>
						<circle cx="12" cy="12" r="3"></circle>
						<line x1="12" y1="2" x2="12" y2="6"></line>
						<line x1="12" y1="18" x2="12" y2="22"></line>
						<line x1="2" y1="12" x2="6" y2="12"></line>
						<line x1="18" y1="12" x2="22" y2="12"></line>
					</svg>
					Use My Location
				{/if}
			</button>

			{#if addressValue.latitude && addressValue.longitude}
				<div class="location-info">
					<span class="coordinates">
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"></path>
							<circle cx="12" cy="10" r="3"></circle>
						</svg>
						{addressValue.latitude.toFixed(6)}, {addressValue.longitude.toFixed(6)}
					</span>
					<button type="button" class="clear-location" onclick={clearLocation} title="Clear location">
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="18" y1="6" x2="6" y2="18"></line>
							<line x1="6" y1="6" x2="18" y2="18"></line>
						</svg>
					</button>
				</div>
			{/if}
		</div>

		{#if geolocationError}
			<div class="geolocation-error">{geolocationError}</div>
		{/if}
	{/if}

	<div class="address-fields">
		<div class="field-row full-width">
			<label for="{name}_line1">Street Address</label>
			<input
				type="text"
				id="{name}_line1"
				name="{name}[address_line_1]"
				value={addressValue.address_line_1}
				oninput={(e: Event) => handleInputChange('address_line_1', (e.target as HTMLInputElement).value)}
				placeholder="123 Main Street"
				{disabled}
				{required}
			/>
		</div>

		<div class="field-row full-width">
			<label for="{name}_line2">Address Line 2 <span class="optional">(Optional)</span></label>
			<input
				type="text"
				id="{name}_line2"
				name="{name}[address_line_2]"
				value={addressValue.address_line_2}
				oninput={(e: Event) => handleInputChange('address_line_2', (e.target as HTMLInputElement).value)}
				placeholder="Apt, Suite, Unit, etc."
				{disabled}
			/>
		</div>

		<div class="field-row">
			<label for="{name}_city">City</label>
			<input
				type="text"
				id="{name}_city"
				name="{name}[city]"
				value={addressValue.city}
				oninput={(e: Event) => handleInputChange('city', (e.target as HTMLInputElement).value)}
				placeholder="City"
				{disabled}
				{required}
			/>
		</div>

		<div class="field-row">
			<label for="{name}_state">State / Province</label>
			<input
				type="text"
				id="{name}_state"
				name="{name}[state]"
				value={addressValue.state}
				oninput={(e: Event) => handleInputChange('state', (e.target as HTMLInputElement).value)}
				placeholder="State"
				{disabled}
				{required}
			/>
		</div>

		<div class="field-row">
			<label for="{name}_zip">ZIP / Postal Code</label>
			<input
				type="text"
				id="{name}_zip"
				name="{name}[zip]"
				value={addressValue.zip}
				oninput={(e: Event) => handleInputChange('zip', (e.target as HTMLInputElement).value)}
				placeholder="12345"
				{disabled}
				{required}
			/>
		</div>

		<div class="field-row">
			<label for="{name}_country">Country</label>
			<input
				type="text"
				id="{name}_country"
				name="{name}[country]"
				value={addressValue.country}
				oninput={(e: Event) => handleInputChange('country', (e.target as HTMLInputElement).value)}
				placeholder="United States"
				{disabled}
				{required}
			/>
		</div>
	</div>

	<!-- Hidden fields for lat/long -->
	{#if addressValue.latitude && addressValue.longitude}
		<input type="hidden" name="{name}[latitude]" value={addressValue.latitude} />
		<input type="hidden" name="{name}[longitude]" value={addressValue.longitude} />
	{/if}

	{#if showMap && addressValue.latitude && addressValue.longitude}
		<div class="map-preview">
			<img
				src="https://staticmap.openstreetmap.de/staticmap.php?center={addressValue.latitude},{addressValue.longitude}&zoom=15&size=400x200&markers={addressValue.latitude},{addressValue.longitude},red-pushpin"
				alt="Location map preview"
				loading="lazy"
			/>
		</div>
	{/if}

	{#if error}
		<div class="error-message">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"></circle>
				<line x1="12" y1="8" x2="12" y2="12"></line>
				<line x1="12" y1="16" x2="12.01" y2="16"></line>
			</svg>
			{error}
		</div>
	{/if}
</fieldset>

<style>
	.geolocation-address {
		border: none;
		padding: 0;
		margin: 0;
	}

	.address-label {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #374151;
		margin-bottom: 1rem;
		display: block;
	}

	.required-marker {
		color: #ef4444;
		margin-left: 0.25rem;
	}

	.geolocation-controls {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1rem;
		padding: 0.75rem;
		background-color: #f0fdf4;
		border: 1px solid #86efac;
		border-radius: 0.5rem;
	}

	.detect-location-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background-color: #059669;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.detect-location-btn:hover:not(:disabled) {
		background-color: #047857;
	}

	.detect-location-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.location-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8125rem;
		color: #059669;
	}

	.coordinates {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.clear-location {
		padding: 0.25rem;
		background: none;
		border: none;
		color: #9ca3af;
		cursor: pointer;
		border-radius: 0.25rem;
	}

	.clear-location:hover {
		color: #ef4444;
		background-color: #fef2f2;
	}

	.geolocation-error {
		padding: 0.5rem 0.75rem;
		background-color: #fef2f2;
		color: #dc2626;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		margin-bottom: 1rem;
	}

	.address-fields {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.field-row {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.field-row.full-width {
		grid-column: 1 / -1;
	}

	.field-row label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #6b7280;
	}

	.optional {
		font-weight: 400;
		color: #9ca3af;
	}

	.field-row input {
		padding: 0.625rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.9375rem;
		color: #374151;
		transition: all 0.15s;
	}

	.field-row input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.field-row input::placeholder {
		color: #9ca3af;
	}

	.map-preview {
		margin-top: 1rem;
		border-radius: 0.5rem;
		overflow: hidden;
		border: 1px solid #e5e7eb;
	}

	.map-preview img {
		width: 100%;
		height: auto;
		display: block;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Disabled State */
	.disabled {
		opacity: 0.6;
	}

	.disabled input {
		cursor: not-allowed;
		background-color: #f9fafb;
	}

	/* Error State */
	.has-error input {
		border-color: #fca5a5;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.75rem;
		color: #dc2626;
		font-size: 0.8125rem;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.address-fields {
			grid-template-columns: 1fr;
		}

		.geolocation-controls {
			flex-direction: column;
			align-items: flex-start;
		}
	}
</style>
