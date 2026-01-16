<script lang="ts">
	/**
	 * Banner Renderer Component - Svelte 5
	 *
	 * Renders consent banner based on the active template configuration.
	 * Supports all 20 template styles with responsive mobile/tablet layouts.
	 *
	 * Updated: December 2025 - Migrated to Svelte 5 runes ($props, $state, $derived)
	 * @component
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { consentStore, openPreferencesModal } from '../store.svelte';
	import { activeTemplate, isPreviewMode } from './store';
	import { t } from '../i18n';
	import { recordImpression, recordDecision } from '../ab-testing';
	import type { BannerTemplate } from './types';

	// Svelte 5: Props using $props() rune
	interface Props {
		forceShow?: boolean;
	}

	let { forceShow = false }: Props = $props();

	// Svelte 5: Reactive state using $state() rune
	let mounted = $state(false);
	let visible = $state(false);
	let closing = $state(false);
	let isMobile = $state(false);
	let isTablet = $state(false);
	let drawerOpen = $state(false);

	// Svelte 5: Derived values using $derived() rune
	// ICT9+ Hydration-Safe: Only show after mount to prevent SSR/client mismatch
	let template = $derived($activeTemplate);
	let shouldShow = $derived(mounted && (forceShow || $isPreviewMode || (!$consentStore.hasInteracted && visible)));

	// Detect device type
	function updateDeviceType() {
		if (!browser) return;
		isMobile = window.innerWidth < 640;
		isTablet = window.innerWidth >= 640 && window.innerWidth < 1024;
	}

	onMount(() => {
		// ICT9+ Hydration-Safe: Mark mounted first
		mounted = true;
		visible = true;
		updateDeviceType();
		window.addEventListener('resize', updateDeviceType);

		if (!$consentStore.hasInteracted) {
			recordImpression();
		}

		return () => {
			window.removeEventListener('resize', updateDeviceType);
		};
	});

	// Get effective position based on device
	function getEffectivePosition(template: BannerTemplate): string {
		if (isMobile) return template.mobile.position;
		if (isTablet) return template.tablet.position;
		return template.position;
	}

	// Get effective styles
	function getContainerStyles(template: BannerTemplate): string {
		const position = getEffectivePosition(template);
		const styles: string[] = [];

		// Background
		styles.push(`background: ${template.colors.background}`);

		// Text color
		styles.push(`color: ${template.colors.text}`);

		// Max width
		if (isMobile) {
			styles.push('max-width: 100%');
		} else if (isTablet) {
			styles.push(`max-width: ${template.tablet.maxWidth}`);
		} else if (!['bottom', 'top', 'fullscreen'].includes(position)) {
			styles.push(`max-width: ${template.maxWidth}`);
		}

		// Border radius
		const borderRadius = position === 'bottom' || position === 'top' ? '0' : template.spacing.borderRadius;
		styles.push(`border-radius: ${borderRadius}`);

		// Padding
		const padding = isMobile ? template.mobile.padding || template.spacing.padding : template.spacing.padding;
		styles.push(`padding: ${padding}`);

		// Box shadow
		if (template.boxShadow) {
			styles.push(`box-shadow: ${template.boxShadow}`);
		}

		// Border
		if (template.border) {
			styles.push(`border: ${template.border}`);
		}

		// Backdrop blur
		if (template.backdropBlur) {
			styles.push(`backdrop-filter: blur(${template.backdropBlur})`);
			styles.push(`-webkit-backdrop-filter: blur(${template.backdropBlur})`);
		}

		// Font family
		styles.push(`font-family: ${template.typography.fontFamily}`);

		// Custom CSS
		if (template.customCSS) {
			styles.push(template.customCSS);
		}

		return styles.join('; ');
	}

	// Get position classes
	function getPositionClasses(template: BannerTemplate): string {
		const position = getEffectivePosition(template);
		const classes = ['consent-banner', `position-${position}`];

		if (template.style) {
			classes.push(`style-${template.style}`);
		}

		if (closing) {
			classes.push('closing');
		}

		if (isMobile && template.mobile.useDrawer) {
			classes.push('drawer');
			if (drawerOpen) classes.push('drawer-open');
		}

		return classes.join(' ');
	}

	// Get animation styles
	function getAnimationStyles(template: BannerTemplate): string {
		return `
			--animation-type: ${template.animation};
			--animation-duration: ${template.animationDuration}ms;
		`;
	}

	// Button order and visibility
	function getButtons(template: BannerTemplate) {
		const buttons = [];

		for (const btn of template.buttonOrder) {
			if (btn === 'accept') {
				buttons.push({
					type: 'accept',
					text: template.copy.acceptAll || $t.acceptAll,
					onClick: handleAcceptAll,
					style: getButtonStyle(template, 'accept'),
				});
			} else if (btn === 'reject' && template.showRejectButton) {
				buttons.push({
					type: 'reject',
					text: template.copy.rejectAll || $t.rejectAll,
					onClick: handleRejectAll,
					style: getButtonStyle(template, 'reject'),
				});
			} else if (btn === 'customize' && template.showCustomizeButton) {
				buttons.push({
					type: 'customize',
					text: template.copy.customize || $t.customize,
					onClick: handleCustomize,
					style: getButtonStyle(template, 'customize'),
				});
			}
		}

		return buttons;
	}

	// Get button styles
	function getButtonStyle(template: BannerTemplate, type: 'accept' | 'reject' | 'customize'): string {
		const styles: string[] = [];

		if (type === 'accept') {
			styles.push(`background: ${template.colors.acceptButton}`);
			styles.push(`color: ${template.colors.acceptButtonText}`);
		} else if (type === 'reject') {
			styles.push(`background: ${template.colors.rejectButton}`);
			styles.push(`color: ${template.colors.rejectButtonText}`);
		} else {
			styles.push(`background: ${template.colors.customizeButton || 'transparent'}`);
			styles.push(`color: ${template.colors.customizeButtonText || template.colors.text}`);
		}

		styles.push(`padding: ${template.spacing.buttonPadding}`);
		styles.push(`border-radius: ${template.spacing.buttonBorderRadius}`);
		styles.push(`font-size: ${template.typography.buttonSize}`);
		styles.push(`font-weight: ${template.typography.buttonWeight}`);

		if (template.buttonVariant === 'outline') {
			styles.push(`border: 2px solid currentColor`);
		} else if (template.buttonVariant === 'pill') {
			styles.push(`border-radius: 9999px`);
		}

		return styles.join('; ');
	}

	// Get icon SVG
	function getIcon(template: BannerTemplate): string {
		if (template.customIcon) return template.customIcon;

		switch (template.iconType) {
			case 'cookie':
				return `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2a10 10 0 1 0 10 10 4 4 0 0 1-5-5 4 4 0 0 1-5-5"/><path d="M8.5 8.5v.01"/><path d="M16 15.5v.01"/><path d="M12 12v.01"/><path d="M11 17v.01"/><path d="M7 14v.01"/></svg>`;
			case 'shield':
				return `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><path d="m9 12 2 2 4-4"/></svg>`;
			case 'lock':
				return `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>`;
			case 'checkmark':
				return `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><path d="m9 11 3 3L22 4"/></svg>`;
			default:
				return '';
		}
	}

	// Event handlers
	function handleAcceptAll() {
		closing = true;
		recordDecision('accept_all');
		setTimeout(() => {
			consentStore.acceptAll('banner');
		}, template.animationDuration);
	}

	function handleRejectAll() {
		closing = true;
		recordDecision('reject_all');
		setTimeout(() => {
			consentStore.rejectAll('banner');
		}, template.animationDuration);
	}

	function handleCustomize() {
		recordDecision('customize');
		openPreferencesModal();
	}

	function handleClose() {
		closing = true;
		setTimeout(() => {
			visible = false;
		}, template.animationDuration);
	}
</script>

{#if shouldShow}
	<!-- Backdrop for center/fullscreen -->
	{#if getEffectivePosition(template) === 'center' || getEffectivePosition(template) === 'fullscreen'}
		<div
			class="consent-backdrop"
			style="background: {template.colors.backdrop || 'rgba(0,0,0,0.5)'}"
			onclick={template.showCloseButton ? handleClose : undefined}
			role="presentation"
		></div>
	{/if}

	<div
		class={getPositionClasses(template)}
		style="{getContainerStyles(template)}; {getAnimationStyles(template)}"
		role="dialog"
		aria-label="Cookie consent"
		aria-modal={getEffectivePosition(template) === 'center' || getEffectivePosition(template) === 'fullscreen'}
	>
		<!-- Close button -->
		{#if template.showCloseButton}
			<button class="close-btn" onclick={handleClose} aria-label="Close">
				<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M18 6 6 18"/><path d="m6 6 12 12"/>
				</svg>
			</button>
		{/if}

		<!-- Content wrapper -->
		<div class="content-wrapper" class:has-icon={template.showIcon} class:stack-buttons={isMobile && template.mobile.stackButtons}>
			<!-- Icon -->
			{#if template.showIcon}
				<div class="icon" style="color: {template.colors.accent}">
					{@html getIcon(template)}
				</div>
			{/if}

			<!-- Text content -->
			<div class="text-content">
				{#if template.copy.title}
					<h2
						class="title"
						style="font-size: {template.typography.titleSize}; font-weight: {template.typography.titleWeight}"
					>
						{template.copy.title}
					</h2>
				{/if}
				<p
					class="description"
					style="font-size: {template.typography.descriptionSize}; color: {template.colors.textMuted}; line-height: {template.typography.lineHeight}"
				>
					{template.copy.description}
				</p>
				{#if template.showPrivacyLink}
					<div class="links" style="color: {template.colors.accent}">
						<a href="/cookie-policy">{template.copy.cookiePolicy || 'Cookie Policy'}</a>
					</div>
				{/if}
			</div>

			<!-- Buttons -->
			<div
				class="buttons"
				class:stack={isMobile && template.mobile.stackButtons}
				class:full-width={isMobile && template.mobile.fullWidthButtons}
				style="gap: {template.spacing.gap}"
			>
				{#each getButtons(template) as button}
					<button
						class="btn btn-{button.type}"
						style={button.style}
						onclick={button.onClick}
					>
						{button.text}
					</button>
				{/each}
			</div>
		</div>
	</div>
{/if}

<style>
	/* Base styles */
	.consent-banner {
		position: fixed;
		z-index: 99999;
		transition: all var(--animation-duration, 300ms) ease;
	}

	/* Position variants */
	.position-bottom {
		bottom: 0;
		left: 0;
		right: 0;
	}

	.position-top {
		top: 0;
		left: 0;
		right: 0;
	}

	.position-bottom-left {
		bottom: 1rem;
		left: 1rem;
	}

	.position-bottom-right {
		bottom: 1rem;
		right: 1rem;
	}

	.position-top-left {
		top: 1rem;
		left: 1rem;
	}

	.position-top-right {
		top: 1rem;
		right: 1rem;
	}

	.position-center {
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}

	.position-fullscreen {
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* Animation entry */
	.consent-banner {
		animation: bannerEntry var(--animation-duration, 300ms) ease forwards;
	}

	.consent-banner.closing {
		animation: bannerExit var(--animation-duration, 300ms) ease forwards;
	}

	@keyframes bannerEntry {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@keyframes bannerExit {
		from {
			opacity: 1;
			transform: translateY(0);
		}
		to {
			opacity: 0;
			transform: translateY(20px);
		}
	}

	.position-top .consent-banner,
	.position-top-left .consent-banner,
	.position-top-right .consent-banner {
		animation-name: bannerEntryTop;
	}

	@keyframes bannerEntryTop {
		from {
			opacity: 0;
			transform: translateY(-20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.position-center .consent-banner,
	.position-fullscreen .consent-banner {
		animation-name: bannerEntryScale;
	}

	@keyframes bannerEntryScale {
		from {
			opacity: 0;
			transform: translate(-50%, -50%) scale(0.95);
		}
		to {
			opacity: 1;
			transform: translate(-50%, -50%) scale(1);
		}
	}

	/* Backdrop */
	.consent-backdrop {
		position: fixed;
		inset: 0;
		z-index: 99998;
		animation: fadeIn 300ms ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* Close button */
	.close-btn {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		padding: 0.5rem;
		background: transparent;
		border: none;
		cursor: pointer;
		opacity: 0.6;
		transition: opacity 0.2s;
		color: inherit;
	}

	.close-btn:hover {
		opacity: 1;
	}

	/* Content wrapper */
	.content-wrapper {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.content-wrapper.has-icon {
		flex-direction: row;
	}

	/* Icon */
	.icon {
		flex-shrink: 0;
	}

	/* Text content */
	.text-content {
		flex: 1;
		min-width: 0;
	}

	.title {
		margin: 0 0 0.5rem 0;
	}

	.description {
		margin: 0 0 0.75rem 0;
	}

	.links {
		font-size: 0.75rem;
		margin-bottom: 0.5rem;
	}

	.links a {
		color: inherit;
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	/* Buttons */
	.buttons {
		display: flex;
		align-items: center;
		flex-shrink: 0;
	}

	.buttons.stack {
		flex-direction: column;
		width: 100%;
	}

	.buttons.full-width .btn {
		width: 100%;
		justify-content: center;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border: none;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.btn:hover {
		filter: brightness(1.1);
		transform: translateY(-1px);
	}

	.btn:active {
		transform: translateY(0);
	}

	/* Drawer mode for mobile */
	.drawer {
		transform: translateY(100%);
		transition: transform 0.3s ease;
	}

	.drawer.drawer-open {
		transform: translateY(0);
	}

	/* Responsive adjustments */
	@media (max-width: 640px) {
		.content-wrapper {
			flex-direction: column;
		}

		.buttons {
			margin-top: 0.5rem;
		}

		.position-bottom-left,
		.position-bottom-right {
			left: 0.5rem;
			right: 0.5rem;
			bottom: 0.5rem;
			max-width: calc(100% - 1rem) !important;
		}

		.position-top-left,
		.position-top-right {
			left: 0.5rem;
			right: 0.5rem;
			top: 0.5rem;
			max-width: calc(100% - 1rem) !important;
		}
	}

	/* Tablet adjustments */
	@media (min-width: 640px) and (max-width: 1023px) {
		.content-wrapper {
			flex-direction: row;
			align-items: center;
		}
	}

	/* Respect reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.consent-banner {
			animation: none;
			transition: none;
		}

		.consent-backdrop {
			animation: none;
		}
	}
</style>
