<script lang="ts">
	import { onMount } from 'svelte';
	import type { FormField } from '$lib/api/forms';
	import { sanitizeFormContent } from '$lib/utils/sanitize';

	interface Props {
		field: FormField;
		value?: any;
		error?: string[];
		onchange?: (value: any) => void;
	}

	let props: Props = $props();

	// ICT 7 Fix: Signature canvas state
	let signatureCanvas: HTMLCanvasElement | null = null;
	let signatureCtx: CanvasRenderingContext2D | null = null;
	let isDrawing = false;
	let lastX = 0;
	let lastY = 0;

	// Initialize signature canvas when field type is signature
	onMount(() => {
		if (props.props.field.field_type === 'signature') {
			initSignatureCanvas();
		}
	});

	function initSignatureCanvas() {
		signatureCanvas = document.getElementById(
			`field-${props.props.field.name}`
		) as HTMLCanvasElement;
		if (!signatureCanvas) return;

		signatureCtx = signatureCanvas.getContext('2d');
		if (!signatureCtx) return;

		// Set canvas styles
		signatureCtx.strokeStyle = '#1f2937';
		signatureCtx.lineWidth = 2;
		signatureCtx.lineCap = 'round';
		signatureCtx.lineJoin = 'round';

		// Mouse events
		signatureCanvas.addEventListener('mousedown', startDrawing);
		signatureCanvas.addEventListener('mousemove', draw);
		signatureCanvas.addEventListener('mouseup', stopDrawing);
		signatureCanvas.addEventListener('mouseout', stopDrawing);

		// Touch events for mobile
		signatureCanvas.addEventListener('touchstart', handleTouchStart, { passive: false });
		signatureCanvas.addEventListener('touchmove', handleTouchMove, { passive: false });
		signatureCanvas.addEventListener('touchend', stopDrawing);

		// Load existing value if any
		if (props.value && typeof props.value === 'string' && props.value.startsWith('data:image')) {
			const img = new Image();
			img.onload = () => {
				signatureCtx?.drawImage(img, 0, 0);
			};
			img.src = props.value;
		}
	}

	function startDrawing(e: MouseEvent) {
		isDrawing = true;
		const rect = signatureCanvas?.getBoundingClientRect();
		if (rect) {
			lastX = e.clientX - rect.left;
			lastY = e.clientY - rect.top;
		}
	}

	function draw(e: MouseEvent) {
		if (!isDrawing || !signatureCtx || !signatureCanvas) return;

		const rect = signatureCanvas.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const y = e.clientY - rect.top;

		signatureCtx.beginPath();
		signatureCtx.moveTo(lastX, lastY);
		signatureCtx.lineTo(x, y);
		signatureCtx.stroke();

		lastX = x;
		lastY = y;
	}

	function handleTouchStart(e: TouchEvent) {
		e.preventDefault();
		if (!signatureCanvas) return;

		const touch = e.touches[0];
		const rect = signatureCanvas.getBoundingClientRect();
		isDrawing = true;
		lastX = touch.clientX - rect.left;
		lastY = touch.clientY - rect.top;
	}

	function handleTouchMove(e: TouchEvent) {
		e.preventDefault();
		if (!isDrawing || !signatureCtx || !signatureCanvas) return;

		const touch = e.touches[0];
		const rect = signatureCanvas.getBoundingClientRect();
		const x = touch.clientX - rect.left;
		const y = touch.clientY - rect.top;

		signatureCtx.beginPath();
		signatureCtx.moveTo(lastX, lastY);
		signatureCtx.lineTo(x, y);
		signatureCtx.stroke();

		lastX = x;
		lastY = y;
	}

	function stopDrawing() {
		if (isDrawing && signatureCanvas) {
			isDrawing = false;
			// Save signature as base64 data URL
			const dataUrl = signatureCanvas.toDataURL('image/png');
			props.props.onchange?.(dataUrl);
		}
	}

	function clearSignature() {
		if (!signatureCtx || !signatureCanvas) return;
		signatureCtx.clearRect(0, 0, signatureCanvas.width, signatureCanvas.height);
		props.props.onchange?.('');
	}

	function handleChange(event: Event) {
		const target = event.target as HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement;

		let newValue: any;

		if (target instanceof HTMLInputElement) {
			if (target.type === 'checkbox') {
				if (props.props.field.field_type === 'checkbox') {
					// Multiple checkboxes - array
					const currentValues = Array.isArray(props.value) ? props.value : [];
					if (target.checked) {
						newValue = [...currentValues, target.value];
					} else {
						newValue = currentValues.filter((v: string) => v !== target.value);
					}
				} else {
					// Single checkbox
					newValue = target.checked;
				}
			} else if (target.type === 'file') {
				newValue = target.files?.[0] || null;
			} else if (target.type === 'number' || target.type === 'range') {
				newValue = target.value ? parseFloat(target.value) : null;
			} else {
				newValue = target.value;
			}
		} else {
			newValue = target.value;
		}

		props.props.onchange?.(newValue);
	}

	function handleCheckboxChange(optionValue: string, checked: boolean) {
		const currentValues = Array.isArray(props.value) ? props.value : [];
		const newValue = checked
			? [...currentValues, optionValue]
			: currentValues.filter((v: string) => v !== optionValue);

		props.props.onchange?.(newValue);
	}

	function isChecked(optionValue: string): boolean {
		return Array.isArray(props.value) && props.value.includes(optionValue);
	}

	function getInputClasses(): string {
		const baseClasses = 'form-input';
		const errorClass = props.error && props.error.length > 0 ? 'input-error' : '';
		return `${baseClasses} ${errorClass}`.trim();
	}
</script>

<div class="form-field">
	{#if props.field.field_type === 'heading'}
		<h3 class="field-heading">{props.field.label}</h3>
	{:else if props.field.field_type === 'divider'}
		<hr class="field-divider" />
	{:else if props.field.field_type === 'html'}
		<div class="field-html">
			{@html sanitizeFormContent(props.field.placeholder || '')}
		</div>
	{:else}
		<label class="field-label" for={`field-${props.field.name}`}>
			{props.field.label}
			{#if props.field.required}
				<span class="required">*</span>
			{/if}
		</label>

		{#if props.field.help_text}
			<div class="field-help">{props.field.help_text}</div>
		{/if}

		<!-- Text Input -->
		{#if props.field.field_type === 'text'}
			<input
				type="text"
				id={`field-${props.field.name}`}
				name={props.field.name}
				placeholder={props.field.placeholder || ''}
				value={props.value ?? ''}
				required={props.field.required}
				class={getInputClasses()}
				oninput={handleChange}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- Email -->
		{:else if props.field.field_type === 'email'}
			<input
				type="email"
				id={`field-${props.field.name}`}
				name={props.field.name}
				placeholder={props.field.placeholder || ''}
				value={props.value ?? ''}
				required={props.field.required}
				class={getInputClasses()}
				oninput={handleChange}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- Number -->
		{:else if props.field.field_type === 'number'}
			<input
				type="number"
				id={`field-${props.field.name}`}
				name={props.field.name}
				placeholder={props.field.placeholder || ''}
				value={props.value || ''}
				required={props.field.required}
				class={getInputClasses()}
				oninput={handleChange}
				min={props.field.validation?.min}
				max={props.field.validation?.max}
				step={typeof props.field.validation?.step === 'number' ||
				typeof props.field.validation?.step === 'string'
					? props.field.validation.step
					: 'any'}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- Phone -->
		{:else if props.field.field_type === 'tel'}
			<input
				type="tel"
				id={`field-${props.field.name}`}
				name={props.field.name}
				placeholder={props.field.placeholder || ''}
				value={props.value ?? ''}
				required={props.field.required}
				class={getInputClasses()}
				oninput={handleChange}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- URL -->
		{:else if props.field.field_type === 'url'}
			<input
				type="url"
				id={`field-${props.field.name}`}
				name={props.field.name}
				placeholder={props.field.placeholder || ''}
				value={props.value ?? ''}
				required={props.field.required}
				class={getInputClasses()}
				oninput={handleChange}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- Textarea -->
		{:else if props.field.field_type === 'textarea'}
			<textarea
				id={`field-${props.field.name}`}
				name={props.field.name}
				placeholder={props.field.placeholder || ''}
				value={props.value ?? ''}
				required={props.field.required}
				class={getInputClasses()}
				oninput={handleChange}
				rows={typeof props.field.attributes?.['rows'] === 'number'
					? props.field.attributes['rows']
					: typeof props.field.attributes?.['rows'] === 'string'
						? parseInt(props.field.attributes['rows'], 10)
						: 5}
				minlength={props.field.validation?.min_length}
				maxlength={props.field.validation?.max_length}
				{...(props.field.attributes as Record<string, any>) || {}}
			></textarea>

			<!-- Select Dropdown -->
		{:else if props.field.field_type === 'select'}
			<select
				id={`field-${props.field.name}`}
				name={props.field.name}
				value={props.value ?? ''}
				required={props.field.required}
				class={getInputClasses()}
				onchange={handleChange}
				{...(props.field.attributes as Record<string, any>) || {}}
			>
				<option value="">-- Select --</option>
				{#if props.field.options}
					{#each props.field.options as option}
						<option value={option}>{option}</option>
					{/each}
				{/if}
			</select>

			<!-- Radio Buttons -->
		{:else if props.field.field_type === 'radio'}
			<div class="radio-group">
				{#if props.field.options}
					{#each props.field.options as option}
						<label class="radio-label">
							<input
								type="radio"
								name={props.field.name}
								value={option}
								checked={props.value === option}
								required={props.field.required}
								onchange={handleChange}
								{...(props.field.attributes as Record<string, any>) || {}}
							/>
							<span>{option}</span>
						</label>
					{/each}
				{/if}
			</div>

			<!-- Checkboxes -->
		{:else if props.field.field_type === 'checkbox'}
			<div class="checkbox-group">
				{#if props.field.options}
					{#each props.field.options as option}
						{@const optionValue = typeof option === 'string' ? option : option.value}
						{@const optionLabel = typeof option === 'string' ? option : option.label}
						<label class="checkbox-label">
							<input
								type="checkbox"
								value={optionValue}
								checked={isChecked(optionValue)}
								onchange={(e: Event) =>
									handleCheckboxChange(optionValue, (e.currentTarget as HTMLInputElement).checked)}
								{...(props.field.attributes as Record<string, any>) || {}}
							/>
							<span>{optionLabel}</span>
						</label>
					{/each}
				{/if}
			</div>

			<!-- File Upload -->
		{:else if props.field.field_type === 'file'}
			<input
				type="file"
				id={`field-${props.field.name}`}
				name={props.field.name}
				required={props.field.required}
				class={getInputClasses()}
				onchange={handleChange}
				accept={typeof props.field.validation?.accept === 'string'
					? props.field.validation.accept
					: undefined}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>
			{#if props.field.validation?.max_size}
				<small class="field-help">Maximum file size: {props.field.validation.max_size}</small>
			{/if}

			<!-- Date -->
		{:else if props.field.field_type === 'date'}
			<input
				type="date"
				id={`field-${props.field.name}`}
				name={props.field.name}
				value={props.value ?? ''}
				required={props.field.required}
				class={getInputClasses()}
				oninput={handleChange}
				min={props.field.validation?.min}
				max={props.field.validation?.max}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- Time -->
		{:else if props.field.field_type === 'time'}
			<input
				type="time"
				id={`field-${props.field.name}`}
				name={props.field.name}
				value={props.value ?? ''}
				required={props.field.required}
				class={getInputClasses()}
				oninput={handleChange}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- Date & Time -->
		{:else if props.field.field_type === 'datetime'}
			<input
				type="datetime-local"
				id={`field-${props.field.name}`}
				name={props.field.name}
				value={props.value ?? ''}
				required={props.field.required}
				class={getInputClasses()}
				oninput={handleChange}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- Range Slider -->
		{:else if props.field.field_type === 'range'}
			<div class="range-wrapper">
				<input
					type="range"
					id={`field-${props.field.name}`}
					name={props.field.name}
					value={props.value || props.field.validation?.min || 0}
					required={props.field.required}
					class="form-range"
					oninput={handleChange}
					min={props.field.validation?.min || 0}
					max={props.field.validation?.max || 100}
					step={typeof props.field.validation?.step === 'number' ||
					typeof props.field.validation?.step === 'string'
						? props.field.validation.step
						: 1}
					{...(props.field.attributes as Record<string, any>) || {}}
				/>
				<output class="range-value">{value || props.field.validation?.min || 0}</output>
			</div>

			<!-- Color Picker -->
		{:else if props.field.field_type === 'color'}
			<input
				type="color"
				id={`field-${props.field.name}`}
				name={props.field.name}
				value={props.value ?? ''}
				required={props.field.required}
				class="form-color"
				oninput={handleChange}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- Hidden Field -->
		{:else if props.field.field_type === 'hidden'}
			<input
				type="hidden"
				name={props.field.name}
				value={props.value ?? ''}
				{...(props.field.attributes as Record<string, any>) || {}}
			/>

			<!-- Rating (Stars) -->
		{:else if props.field.field_type === 'rating'}
			<div class="rating-wrapper">
				{#each Array(props.field.validation?.max || 5) as _, i}
					<button
						type="button"
						class="star-button"
						class:active={value > i}
						onclick={() => props.onchange?.(i + 1)}
						aria-label={`Rate ${i + 1} stars`}
					>
						★
					</button>
				{/each}
			</div>

			<!-- Signature Pad - ICT 7 Fix: Full canvas drawing support -->
		{:else if props.field.field_type === 'signature'}
			<div class="signature-wrapper">
				<canvas
					id={`field-${props.field.name}`}
					class="signature-canvas"
					width="400"
					height="150"
					aria-label="Signature pad - draw your signature"
				></canvas>
				<div class="signature-actions">
					<button type="button" class="btn-clear-signature" onclick={clearSignature}>
						Clear Signature
					</button>
					{#if value}
						<span class="signature-status">Signature captured</span>
					{/if}
				</div>
			</div>

			<!-- ICT 7 Fix: Address Field - Complete multi-part address input -->
		{:else if props.field.field_type === 'address'}
			{@const addressValue = typeof value === 'object' && value !== null ? value : {}}
			<div class="address-wrapper">
				<div class="address-fields">
					<div class="address-row full-width">
						<label class="address-sub-label" for={`${props.field.name}-address1`}
							>Address Line 1</label
						>
						<input
							type="text"
							id={`${props.field.name}-address1`}
							name={`${props.field.name}[address_line_1]`}
							placeholder="Street address"
							value={addressValue.address_line_1 || ''}
							class={getInputClasses()}
							required={props.field.required}
							oninput={(e: Event) => {
								const newVal = {
									...addressValue,
									address_line_1: (e.target as HTMLInputElement).value
								};
								props.onchange?.(newVal);
							}}
						/>
					</div>
					{#if props.field.attributes?.show_address_2 !== false}
						<div class="address-row full-width">
							<label class="address-sub-label" for={`${props.field.name}-address2`}
								>Address Line 2</label
							>
							<input
								type="text"
								id={`${props.field.name}-address2`}
								name={`${props.field.name}[address_line_2]`}
								placeholder="Apartment, suite, unit, etc. (optional)"
								value={addressValue.address_line_2 || ''}
								class={getInputClasses()}
								oninput={(e: Event) => {
									const newVal = {
										...addressValue,
										address_line_2: (e.target as HTMLInputElement).value
									};
									props.onchange?.(newVal);
								}}
							/>
						</div>
					{/if}
					<div class="address-row-group">
						<div class="address-row city">
							<label class="address-sub-label" for={`${props.field.name}-city`}>City</label>
							<input
								type="text"
								id={`${props.field.name}-city`}
								name={`${props.field.name}[city]`}
								placeholder="City"
								value={addressValue.city || ''}
								class={getInputClasses()}
								required={props.field.required}
								oninput={(e: Event) => {
									const newVal = { ...addressValue, city: (e.target as HTMLInputElement).value };
									props.onchange?.(newVal);
								}}
							/>
						</div>
						<div class="address-row state">
							<label class="address-sub-label" for={`${props.field.name}-state`}
								>State/Province</label
							>
							<input
								type="text"
								id={`${props.field.name}-state`}
								name={`${props.field.name}[state]`}
								placeholder="State"
								value={addressValue.state || ''}
								class={getInputClasses()}
								required={props.field.required}
								oninput={(e: Event) => {
									const newVal = { ...addressValue, state: (e.target as HTMLInputElement).value };
									props.onchange?.(newVal);
								}}
							/>
						</div>
						<div class="address-row zip">
							<label class="address-sub-label" for={`${props.field.name}-zip`}
								>ZIP/Postal Code</label
							>
							<input
								type="text"
								id={`${props.field.name}-zip`}
								name={`${props.field.name}[zip]`}
								placeholder="ZIP Code"
								value={addressValue.zip || ''}
								class={getInputClasses()}
								required={props.field.required}
								oninput={(e: Event) => {
									const newVal = { ...addressValue, zip: (e.target as HTMLInputElement).value };
									props.onchange?.(newVal);
								}}
							/>
						</div>
					</div>
					{#if props.field.attributes?.show_country !== false}
						<div class="address-row full-width">
							<label class="address-sub-label" for={`${props.field.name}-country`}>Country</label>
							<select
								id={`${props.field.name}-country`}
								name={`${props.field.name}[country]`}
								value={addressValue.country || 'US'}
								class={getInputClasses()}
								onchange={(e: Event) => {
									const newVal = {
										...addressValue,
										country: (e.target as HTMLSelectElement).value
									};
									props.onchange?.(newVal);
								}}
							>
								<option value="US">United States</option>
								<option value="CA">Canada</option>
								<option value="GB">United Kingdom</option>
								<option value="AU">Australia</option>
								<option value="DE">Germany</option>
								<option value="FR">France</option>
								<option value="ES">Spain</option>
								<option value="IT">Italy</option>
								<option value="NL">Netherlands</option>
								<option value="MX">Mexico</option>
								<option value="BR">Brazil</option>
								<option value="JP">Japan</option>
								<option value="KR">South Korea</option>
								<option value="IN">India</option>
								<option value="SG">Singapore</option>
								<option value="NZ">New Zealand</option>
							</select>
						</div>
					{/if}
				</div>
			</div>

			<!-- ICT 7 Fix: Phone Field with International Support -->
		{:else if props.field.field_type === 'phone'}
			<div class="phone-wrapper">
				<input
					type="tel"
					id={`field-${props.field.name}`}
					name={props.field.name}
					placeholder={props.field.placeholder || '+1 (555) 123-4567'}
					value={props.value ?? ''}
					required={props.field.required}
					class={getInputClasses()}
					pattern={props.field.validation?.pattern || '[0-9+\\-\\s\\(\\)]+'}
					oninput={handleChange}
					{...(props.field.attributes as Record<string, any>) || {}}
				/>
				{#if props.field.help_text}
					<small class="phone-help">{props.field.help_text}</small>
				{/if}
			</div>

			<!-- Newsletter Subscribe Checkbox -->
		{:else if props.field.field_type === 'newsletter_subscribe'}
			<div class="newsletter-subscribe-wrapper">
				<label class="newsletter-checkbox-label">
					<input
						type="checkbox"
						id={`field-${props.field.name}`}
						name={props.field.name}
						checked={props.value === true || value === '1' || value === 'yes' || value === 'on'}
						onchange={(e: Event) => props.onchange?.((e.currentTarget as HTMLInputElement).checked)}
						{...(props.field.attributes as Record<string, any>) || {}}
					/>
					<span class="newsletter-checkbox-text">
						{props.field.options?.checkbox_label || 'Yes, I want to receive newsletters'}
					</span>
				</label>
				{#if props.field.options?.show_privacy_link !== false}
					<a
						href={props.field.options?.privacy_url || '/privacy-policy'}
						target="_blank"
						rel="noopener noreferrer"
						class="privacy-link"
					>
						Privacy Policy
					</a>
				{/if}
			</div>

			<!-- Newsletter Categories Multi-Select -->
		{:else if props.field.field_type === 'newsletter_categories'}
			<div class="newsletter-categories-wrapper">
				{#if props.field.options && Array.isArray(props.field.options)}
					{#each props.field.options as option}
						{@const optionValue = typeof option === 'string' ? option : option.value}
						{@const optionLabel = typeof option === 'string' ? option : option.label}
						{@const optionDescription = typeof option === 'object' ? option.description : ''}
						{@const optionIcon = typeof option === 'object' ? option.icon : null}
						{@const optionColor = typeof option === 'object' ? option.color : null}
						<label
							class="category-card"
							style={optionColor ? `border-left-color: ${optionColor}` : ''}
						>
							<input
								type="checkbox"
								value={optionValue}
								checked={isChecked(optionValue)}
								onchange={(e: Event) =>
									handleCheckboxChange(optionValue, (e.currentTarget as HTMLInputElement).checked)}
							/>
							<div class="category-content">
								{#if optionIcon}
									<span class="category-icon">{optionIcon}</span>
								{/if}
								<div class="category-text">
									<span class="category-label">{optionLabel}</span>
									{#if optionDescription}
										<span class="category-description">{optionDescription}</span>
									{/if}
								</div>
							</div>
						</label>
					{/each}
				{/if}
				{#if props.field.attributes?.['min_selections'] || props.field.attributes?.['max_selections']}
					<small class="category-hint">
						{#if props.field.attributes['min_selections'] && props.field.attributes['max_selections']}
							Select between {props.field.attributes['min_selections']} and {props.field.attributes[
								'max_selections'
							]} topics
						{:else if props.field.attributes['min_selections']}
							Select at least {props.field.attributes['min_selections']} topic(s)
						{:else if props.field.attributes['max_selections']}
							Select up to {props.field.attributes['max_selections']} topics
						{/if}
					</small>
				{/if}
			</div>

			<!-- Newsletter Frequency Select -->
		{:else if props.field.field_type === 'newsletter_frequency'}
			<select
				id={`field-${props.field.name}`}
				name={props.field.name}
				value={props.value || props.field.default_value || 'weekly'}
				required={props.field.required}
				class={getInputClasses()}
				onchange={handleChange}
				{...(props.field.attributes as Record<string, any>) || {}}
			>
				{#if props.field.options && Array.isArray(props.field.options)}
					{#each props.field.options as option}
						{@const optionValue = typeof option === 'string' ? option : option.value}
						{@const optionLabel = typeof option === 'string' ? option : option.label}
						<option value={optionValue}>{optionLabel}</option>
					{/each}
				{:else}
					<option value="daily">Daily Digest</option>
					<option value="weekly">Weekly Summary</option>
					<option value="biweekly">Bi-Weekly Updates</option>
					<option value="monthly">Monthly Newsletter</option>
				{/if}
			</select>
		{/if}

		{#if props.error && error.length > 0}
			<div class="field-error">
				{#each props.error as err}
					<p>{err}</p>
				{/each}
			</div>
		{/if}
	{/if}
</div>

<style>
	.form-field {
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
	}

	/* 2026 Mobile-First Responsive - 44px touch targets, 16px font to prevent iOS zoom */
	.form-input,
	.form-range,
	.form-color {
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 1rem; /* 16px prevents iOS zoom on focus */
		min-height: 44px; /* Touch target */
		transition: all 0.2s;
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
	}

	.form-input:focus {
		outline: none;
		border-color: #2563eb;
		box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
	}

	.input-error {
		border-color: #dc2626 !important;
	}

	.input-error:focus {
		box-shadow: 0 0 0 3px rgba(220, 38, 38, 0.1) !important;
	}

	textarea.form-input {
		resize: vertical;
		min-height: 120px; /* Enhanced for mobile touch */
		padding: 0.875rem 1rem;
	}

	select.form-input {
		cursor: pointer;
		background-color: white;
	}

	.radio-group,
	.checkbox-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.radio-label,
	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		font-size: 1rem; /* 16px for readability */
		min-height: 44px; /* Touch target */
		padding: 0.5rem 0.75rem;
		border-radius: 0.375rem;
		transition: background-color 0.2s;
		touch-action: manipulation;
	}

	.radio-label:hover,
	.checkbox-label:hover {
		background-color: #f9fafb;
	}

	.radio-label input,
	.checkbox-label input {
		cursor: pointer;
		width: 20px;
		height: 20px;
		min-width: 20px;
		accent-color: #2563eb;
	}

	.range-wrapper {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.form-range {
		flex: 1;
		padding: 0;
	}

	.range-value {
		min-width: 3rem;
		text-align: center;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.rating-wrapper {
		display: flex;
		gap: 0.25rem;
		flex-wrap: wrap;
	}

	.star-button {
		background: none;
		border: none;
		font-size: 2rem;
		color: #d1d5db;
		cursor: pointer;
		transition: color 0.2s;
		padding: 0.5rem;
		min-width: 44px; /* Touch target */
		min-height: 44px; /* Touch target */
		display: flex;
		align-items: center;
		justify-content: center;
		touch-action: manipulation;
	}

	.star-button:hover,
	.star-button.active {
		color: #fbbf24;
	}

	.signature-wrapper {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.signature-canvas {
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		background-color: white;
		touch-action: none;
		cursor: crosshair;
		max-width: 100%;
		height: auto;
	}

	.signature-actions {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.btn-clear-signature {
		padding: 0.75rem 1.25rem;
		background-color: #6b7280;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 1rem; /* 16px for readability */
		min-height: 44px; /* Touch target */
		cursor: pointer;
		touch-action: manipulation;
		transition: background-color 0.2s;
	}

	.btn-clear-signature:hover {
		background-color: #4b5563;
	}

	.signature-status {
		font-size: 0.875rem;
		color: #059669;
		font-weight: 500;
	}

	/* ICT 7 Fix: Address Field Styles - 2026 Mobile-First */
	.address-wrapper {
		width: 100%;
	}

	.address-fields {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.address-row {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.address-row.full-width {
		width: 100%;
	}

	.address-row-group {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	@media (min-width: 640px) {
		.address-row-group {
			flex-direction: row;
			gap: 1rem;
		}

		.address-row.city {
			flex: 2;
		}

		.address-row.state {
			flex: 1;
		}

		.address-row.zip {
			flex: 1;
		}
	}

	.address-sub-label {
		font-size: 0.75rem;
		color: #6b7280;
		font-weight: 500;
	}

	/* ICT 7 Fix: Phone Field Styles */
	.phone-wrapper {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.phone-help {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}

	.field-heading {
		font-size: 1.25rem;
		font-weight: 600;
		color: #111827;
		margin: 0.5rem 0;
	}

	.field-divider {
		border: none;
		border-top: 1px solid #e5e7eb;
		margin: 1rem 0;
	}

	.field-html {
		font-size: 0.875rem;
		color: #374151;
	}

	/* Newsletter Subscribe Styles */
	.newsletter-subscribe-wrapper {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.newsletter-checkbox-label {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		cursor: pointer;
		padding: 1rem;
		min-height: 48px; /* Enhanced touch target */
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		transition: all 0.2s;
		touch-action: manipulation;
	}

	.newsletter-checkbox-label:hover {
		background-color: #f3f4f6;
		border-color: #d1d5db;
	}

	.newsletter-checkbox-label input {
		margin-top: 0.125rem;
		width: 20px;
		height: 20px;
		min-width: 20px;
		cursor: pointer;
		accent-color: #2563eb;
	}

	.newsletter-checkbox-text {
		font-size: 1rem; /* 16px for readability */
		color: #374151;
		line-height: 1.5;
	}

	.privacy-link {
		font-size: 0.875rem;
		color: #6b7280;
		text-decoration: none;
		transition: color 0.2s;
		padding: 0.5rem 0;
		min-height: 44px; /* Touch target */
		display: inline-flex;
		align-items: center;
		touch-action: manipulation;
	}

	.privacy-link:hover {
		color: #2563eb;
		text-decoration: underline;
	}

	/* Newsletter Categories Styles - 2026 Mobile-First */
	.newsletter-categories-wrapper {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	@media (min-width: 640px) {
		.newsletter-categories-wrapper {
			display: grid;
			grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		}
	}

	.category-card {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		min-height: 48px; /* Touch target */
		background-color: #ffffff;
		border: 1px solid #e5e7eb;
		border-left-width: 3px;
		border-left-color: #d1d5db;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
		touch-action: manipulation;
	}

	.category-card:hover {
		border-color: #d1d5db;
		background-color: #f9fafb;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.category-card:has(input:checked) {
		border-color: #2563eb;
		border-left-color: currentColor;
		background-color: #eff6ff;
	}

	.category-card input {
		margin-top: 0.125rem;
		width: 20px;
		height: 20px;
		min-width: 20px;
		cursor: pointer;
		accent-color: #2563eb;
	}

	.category-content {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		flex: 1;
	}

	.category-icon {
		font-size: 1.25rem;
		line-height: 1;
	}

	.category-text {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.category-label {
		font-size: 1rem; /* 16px for readability */
		font-weight: 500;
		color: #111827;
	}

	.category-description {
		font-size: 0.75rem;
		color: #6b7280;
		line-height: 1.4;
	}

	.category-hint {
		grid-column: 1 / -1;
		font-size: 0.75rem;
		color: #6b7280;
		margin-top: 0.25rem;
	}
</style>
