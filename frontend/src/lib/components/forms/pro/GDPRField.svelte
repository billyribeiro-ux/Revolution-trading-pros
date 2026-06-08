<script lang="ts">
	/* eslint svelte/no-at-html-tags: "off" -- every {@html} in this file renders sanitizer-cleaned HTML (sanitizeHtml/sanitizeBlogContent/etc.) or serialized JSON-LD; audited 2026-05-30 */
	import type { FormField } from '$lib/api/forms';
	import { sanitizeFormContent } from '$lib/utils/sanitize';
	import Icon from '$lib/components/Icon.svelte';

	interface Props {
		field: FormField;
		value?: boolean;
		error?: string[];
		onchange?: (value: boolean) => void;
	}

	let props: Props = $props();

	// R8-A: `field.attributes` is now `JsonValue | undefined`. Narrow per-key.
	function asString(v: unknown, fallback: string): string {
		return typeof v === 'string' ? v : fallback;
	}

	const consentText = $derived(
		asString(
			props.field.attributes?.consent_text,
			'I agree to the processing of my personal data in accordance with the Privacy Policy.'
		)
	);
	const privacyUrl = $derived(asString(props.field.attributes?.privacy_url, '/privacy-policy'));
	const termsUrl = $derived(asString(props.field.attributes?.terms_url, '/terms-of-service'));
	const showPrivacyLink = $derived(props.field.attributes?.show_privacy_link !== false);
	const showTermsLink = $derived(props.field.attributes?.show_terms_link !== false);
	const fieldType = $derived<'gdpr' | 'terms'>(
		props.field.attributes?.field_type === 'terms' ? 'terms' : 'gdpr'
	);

	function handleChange(event: Event) {
		const target = event.target as HTMLInputElement;
		props.onchange?.(target.checked);
	}
</script>

<div class="gdpr-field">
	{#if props.field.label}
		<span class="field-label">
			{props.field.label}
			{#if props.field.required}
				<span class="required">*</span>
			{/if}
		</span>
	{/if}

	<div class={['consent-wrapper', { 'has-error': props.error && props.error.length > 0 }]}>
		<label class="consent-label">
			<input
				type="checkbox"
				name={props.field.name}
				checked={props.value ?? false}
				required={props.field.required}
				class="consent-checkbox"
				onchange={handleChange}
			/>

			<span class="consent-content">
				{#if fieldType === 'gdpr'}
					<span class="gdpr-icon">
						<Icon name="IconShieldCheck" size={20} />
					</span>
				{/if}

				<span class="consent-text">
					{@html sanitizeFormContent(consentText)}
				</span>
			</span>
		</label>

		{#if showPrivacyLink || showTermsLink}
			<div class="legal-links">
				{#if showPrivacyLink}
					<a href={privacyUrl} target="_blank" rel="noopener noreferrer" class="legal-link">
						<Icon name="IconShield" size={14} />
						Privacy Policy
					</a>
				{/if}
				{#if showTermsLink}
					<a href={termsUrl} target="_blank" rel="noopener noreferrer" class="legal-link">
						<Icon name="IconFileText" size={14} />
						Terms of Service
					</a>
				{/if}
			</div>
		{/if}
	</div>

	{#if props.field.help_text}
		<p class="field-help">{props.field.help_text}</p>
	{/if}

	{#if props.error && props.error.length > 0}
		<div class="field-error">
			{#each props.error as err (err)}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.gdpr-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.field-label {
		font-weight: 500;
		font-size: 0.875rem;
		color: #374151;
		display: block;
	}

	.required {
		color: #dc2626;
		margin-left: 0.25rem;
	}

	.consent-wrapper {
		padding: 1rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		transition: all 0.2s;
	}

	.consent-wrapper:hover {
		background-color: #f3f4f6;
		border-color: #d1d5db;
	}

	.consent-wrapper.has-error {
		border-color: #dc2626;
		background-color: #fef2f2;
	}

	.consent-label {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		cursor: pointer;
	}

	.consent-checkbox {
		width: 1.25rem;
		height: 1.25rem;
		margin-top: 0.125rem;
		accent-color: #2563eb;
		cursor: pointer;
		flex-shrink: 0;
	}

	.consent-content {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		flex: 1;
	}

	.gdpr-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background-color: #dbeafe;
		border-radius: 50%;
		color: #2563eb;
		flex-shrink: 0;
	}

	.consent-text {
		font-size: 0.875rem;
		color: #374151;
		line-height: 1.5;
	}

	.consent-text :global(a) {
		color: #2563eb;
		text-decoration: none;
	}

	.consent-text :global(a:hover) {
		text-decoration: underline;
	}

	.legal-links {
		display: flex;
		gap: 1rem;
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px dashed #e5e7eb;
		flex-wrap: wrap;
	}

	.legal-link {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: #6b7280;
		text-decoration: none;
		transition: color 0.2s;
	}

	.legal-link:hover {
		color: #2563eb;
	}

	.field-help {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}
</style>
