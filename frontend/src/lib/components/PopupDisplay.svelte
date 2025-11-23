<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import { popupsApi } from '$lib/api/popups';
	import type { Popup } from '$lib/stores/popups';
	import { IconX } from '@tabler/icons-svelte';

	export let pageUrl: string = '';

	let popups: Popup[] = [];
	let currentPopup: Popup | null = null;
	let show = false;
	let viewId: number | null = null;
	let isClosing = false;

	// Tracking
	let scrollDepth = 0;
	let timeOnPage = 0;
	let exitIntentTriggered = false;

	// Timers
	let timeInterval: any;
	let showTimeout: any;

	// Accessibility
	let popupElement: HTMLElement;
	let previousFocusedElement: HTMLElement | null = null;

	// Button ripple effect state
	let rippleActive = false;
	let rippleX = 0;
	let rippleY = 0;

	onMount(async () => {
		if (!browser) return;

		await loadActivePopups();
		setupTracking();
	});

	onDestroy(() => {
		if (timeInterval) clearInterval(timeInterval);
		if (showTimeout) clearTimeout(showTimeout);
		removeEventListeners();
		restoreFocus();
	});

	async function loadActivePopups() {
		try {
			const device = getDeviceType();
			const response = await popupsApi.getActive(pageUrl || window.location.href);
			popups = response || [];

			if (popups.length > 0) {
				// Show highest priority popup
				currentPopup = popups.sort((a, b) => b.priority - a.priority)[0];
				setupTriggers(currentPopup);
			}
		} catch (error) {
			console.error('Failed to load popups:', error);
		}
	}

	function setupTriggers(popup: Popup) {
		const rules = popup.trigger_rules || {};

		switch (popup.type) {
			case 'timed':
				const delay = rules.delay || 5000;
				showTimeout = setTimeout(() => showPopup(popup), delay);
				break;

			case 'exit_intent':
				document.addEventListener('mouseout', handleExitIntent);
				break;

			case 'scroll':
				window.addEventListener('scroll', handleScroll);
				break;

			case 'click_trigger':
				const selector = rules.selector || '[data-popup-trigger]';
				document.querySelectorAll(selector).forEach((el) => {
					el.addEventListener('click', () => showPopup(popup));
				});
				break;

			default:
				// Immediate display
				setTimeout(() => showPopup(popup), 100);
		}
	}

	function setupTracking() {
		// Track time on page
		timeInterval = setInterval(() => {
			timeOnPage += 1000;
		}, 1000);

		// Track scroll depth
		window.addEventListener('scroll', updateScrollDepth);
	}

	function removeEventListeners() {
		document.removeEventListener('mouseout', handleExitIntent);
		window.removeEventListener('scroll', handleScroll);
		window.removeEventListener('scroll', updateScrollDepth);
	}

	function handleExitIntent(e: MouseEvent) {
		if (exitIntentTriggered || !currentPopup) return;

		// Trigger when mouse leaves top of page
		if (e.clientY <= 0) {
			exitIntentTriggered = true;
			showPopup(currentPopup);
			document.removeEventListener('mouseout', handleExitIntent);
		}
	}

	function handleScroll() {
		if (!currentPopup) return;

		const rules = currentPopup.trigger_rules || {};
		const targetDepth = rules.scroll_depth || 50; // Default 50%

		updateScrollDepth();

		if (scrollDepth >= targetDepth && !show) {
			showPopup(currentPopup);
			window.removeEventListener('scroll', handleScroll);
		}
	}

	function updateScrollDepth() {
		const windowHeight = window.innerHeight;
		const documentHeight = document.documentElement.scrollHeight;
		const scrollTop = window.pageYOffset || document.documentElement.scrollTop;

		scrollDepth = ((scrollTop + windowHeight) / documentHeight) * 100;
	}

	async function showPopup(popup: Popup) {
		currentPopup = popup;
		show = true;
		isClosing = false;

		// Save current focus for restoration
		if (browser && document.activeElement instanceof HTMLElement) {
			previousFocusedElement = document.activeElement;
		}

		// Track view
		try {
			const popupIdNum = parseInt(popup.id, 10);
			await popupsApi.trackView(popupIdNum);
			// Note: viewId tracking could be enhanced by updating the API to return view_id
			viewId = popupIdNum; // Use popup.id as fallback for tracking
		} catch (error) {
			console.error('Failed to track view:', error);
		}

		// Focus management - wait for element to render
		setTimeout(() => {
			if (popupElement) {
				const firstFocusable = popupElement.querySelector<HTMLElement>(
					'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
				);
				firstFocusable?.focus();
			}
		}, 100);

		// Auto close
		if (popup.auto_close_after) {
			setTimeout(closePopup, popup.auto_close_after * 1000);
		}
	}

	function closePopup() {
		isClosing = true;

		// Wait for exit animation to complete
		setTimeout(() => {
			show = false;
			isClosing = false;
			removeEventListeners();
			restoreFocus();
		}, 300);
	}

	function restoreFocus() {
		if (previousFocusedElement) {
			previousFocusedElement.focus();
			previousFocusedElement = null;
		}
	}

	function handleOverlayClick() {
		if (currentPopup?.close_on_overlay_click) {
			closePopup();
		}
	}

	async function handleConversion(type: string, data: any = {}) {
		if (!currentPopup || !viewId) return;

		try {
			const popupIdNum = parseInt(currentPopup.id, 10);
			await popupsApi.trackConversion(popupIdNum, { action: type, ...data });
			closePopup();
		} catch (error) {
			console.error('Failed to track conversion:', error);
		}
	}

	function handleCTAClick(event: MouseEvent) {
		// Ripple effect
		if (event.currentTarget instanceof HTMLElement) {
			const rect = event.currentTarget.getBoundingClientRect();
			rippleX = event.clientX - rect.left;
			rippleY = event.clientY - rect.top;
			rippleActive = true;

			setTimeout(() => {
				rippleActive = false;
			}, 600);
		}

		handleConversion('cta_click');

		if (currentPopup?.cta_url) {
			if (currentPopup.cta_new_tab) {
				window.open(currentPopup.cta_url, '_blank');
			} else {
				window.location.href = currentPopup.cta_url;
			}
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (!show) return;

		if (event.key === 'Escape' && currentPopup?.show_close_button) {
			closePopup();
		}

		// Trap focus within popup
		if (event.key === 'Tab' && popupElement) {
			const focusableElements = popupElement.querySelectorAll<HTMLElement>(
				'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
			);

			if (focusableElements.length === 0) return;

			const firstElement = focusableElements[0];
			const lastElement = focusableElements[focusableElements.length - 1];

			if (event.shiftKey && document.activeElement === firstElement) {
				lastElement.focus();
				event.preventDefault();
			} else if (!event.shiftKey && document.activeElement === lastElement) {
				firstElement.focus();
				event.preventDefault();
			}
		}
	}

	function getDeviceType(): string {
		if (!browser) return 'desktop';

		const width = window.innerWidth;
		if (width < 768) return 'mobile';
		if (width < 1024) return 'tablet';
		return 'desktop';
	}

	function getPositionClasses(position: string): string {
		switch (position) {
			case 'top':
				return 'items-start justify-center pt-4 sm:pt-8';
			case 'bottom':
				return 'items-end justify-center pb-4 sm:pb-8';
			case 'corner':
				return 'items-end justify-end p-4 sm:p-6';
			default:
				return 'items-center justify-center';
		}
	}

	function getSizeClass(size: string): string {
		switch (size) {
			case 'sm':
				return 'max-w-xs sm:max-w-sm';
			case 'md':
				return 'max-w-sm sm:max-w-md';
			case 'lg':
				return 'max-w-md sm:max-w-lg';
			case 'xl':
				return 'max-w-lg sm:max-w-xl';
			case 'full':
				return 'max-w-full m-2 sm:m-4';
			default:
				return 'max-w-sm sm:max-w-md';
		}
	}

	function getAnimationClass(animation: string): string {
		const baseClass = isClosing ? 'popup-exit' : 'popup-enter';

		switch (animation) {
			case 'fade':
				return `${baseClass}-fade`;
			case 'slide':
				return `${baseClass}-slide`;
			case 'zoom':
				return `${baseClass}-zoom`;
			case 'bounce':
				return `${baseClass}-bounce`;
			case 'rotate':
				return `${baseClass}-rotate`;
			case 'flip':
				return `${baseClass}-flip`;
			default:
				return `${baseClass}-fade`;
		}
	}

	function getButtonStyles(design: any): string {
		const styles: string[] = [];

		if (design?.buttonColor) {
			styles.push(`background-color: ${design.buttonColor}`);
		}

		if (design?.buttonTextColor) {
			styles.push(`color: ${design.buttonTextColor}`);
		}

		if (design?.buttonBorderRadius) {
			styles.push(`border-radius: ${design.buttonBorderRadius}px`);
		}

		if (design?.buttonShadow) {
			styles.push(`box-shadow: ${design.buttonShadow}`);
		}

		if (design?.buttonPadding) {
			styles.push(`padding: ${design.buttonPadding}`);
		}

		return styles.join('; ');
	}


	// Reactive keyboard event listener
	$: if (browser && show) {
		window.addEventListener('keydown', handleKeydown);
	} else if (browser) {
		window.removeEventListener('keydown', handleKeydown);
	}
</script>

{#if show && currentPopup}
	<div
		class="popup-overlay {getPositionClasses(currentPopup.position)}"
		class:closing={isClosing}
		on:click={handleOverlayClick}
		on:keydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		aria-labelledby="popup-title"
		aria-describedby="popup-content"
		tabindex="-1"
	>
		<!-- Backdrop with smooth transition -->
		<div class="popup-backdrop" class:closing={isClosing}></div>

		<!-- Popup Container -->
		<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
		<div
			bind:this={popupElement}
			class="popup-container {getSizeClass(currentPopup.size)} {getAnimationClass(
				currentPopup.animation
			)}"
			on:click|stopPropagation
			on:keydown|stopPropagation
			role="document"
			style={currentPopup.design?.backgroundColor
				? `background-color: ${currentPopup.design.backgroundColor}`
				: ''}
		>
			<!-- Close Button -->
			{#if currentPopup.show_close_button}
				<button
					on:click={closePopup}
					class="popup-close-button"
					aria-label="Close popup"
					title="Close (Esc)"
				>
					<IconX size={20} />
				</button>
			{/if}

			<!-- Content -->
			<div class="popup-content">
				{#if currentPopup.title}
					<h2
						id="popup-title"
						class="popup-title"
						style={currentPopup.design?.titleColor
							? `color: ${currentPopup.design.titleColor}`
							: ''}
					>
						{currentPopup.title}
					</h2>
				{/if}

				{#if currentPopup.content}
					<div
						id="popup-content"
						class="popup-body"
						style={currentPopup.design?.textColor ? `color: ${currentPopup.design.textColor}` : ''}
					>
						{@html currentPopup.content}
					</div>
				{/if}

				<!-- Form Integration -->
				{#if currentPopup.has_form && currentPopup.form_id}
					<div class="popup-form">
						<!-- Form will be loaded here -->
						<p class="text-sm text-gray-500">Form integration coming soon...</p>
					</div>
				{/if}

				<!-- CTA Button with advanced effects -->
				{#if currentPopup.cta_text}
					<button
						on:click={handleCTAClick}
						class="popup-cta-button"
						style={getButtonStyles(currentPopup.design)}
						aria-label={currentPopup.cta_text}
					>
						<span class="popup-cta-text">{currentPopup.cta_text}</span>

						<!-- Ripple effect -->
						{#if rippleActive}
							<span class="button-ripple" style="left: {rippleX}px; top: {rippleY}px;"></span>
						{/if}
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	/* Overlay */
	.popup-overlay {
		position: fixed;
		inset: 0;
		z-index: 9999;
		display: flex;
		padding: 1rem;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
	}

	/* Backdrop */
	.popup-backdrop {
		position: absolute;
		inset: 0;
		background-color: rgba(0, 0, 0, 0.75);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		animation: backdropFadeIn 0.3s ease-out;
	}

	.popup-backdrop.closing {
		animation: backdropFadeOut 0.3s ease-in;
		opacity: 0;
	}

	/* Container */
	.popup-container {
		position: relative;
		background: white;
		border-radius: 16px;
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.25),
			0 0 0 1px rgba(0, 0, 0, 0.05);
		width: 100%;
		max-height: calc(100vh - 2rem);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		will-change: transform, opacity;
	}

	/* Mobile optimizations */
	@media (max-width: 640px) {
		.popup-container {
			border-radius: 12px;
			max-height: calc(100vh - 1rem);
		}

		.popup-overlay {
			padding: 0.5rem;
		}
	}

	/* Tablet optimizations */
	@media (min-width: 641px) and (max-width: 1024px) {
		.popup-container {
			max-width: 90vw;
		}
	}

	/* Close Button */
	.popup-close-button {
		position: absolute;
		top: 1rem;
		right: 1rem;
		z-index: 10;
		padding: 0.5rem;
		color: #9ca3af;
		background-color: transparent;
		border: none;
		border-radius: 9999px;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.popup-close-button:hover {
		color: #374151;
		background-color: #f3f4f6;
		transform: scale(1.1);
	}

	.popup-close-button:active {
		transform: scale(0.95);
	}

	.popup-close-button:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* Content */
	.popup-content {
		padding: 2rem;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
	}

	@media (max-width: 640px) {
		.popup-content {
			padding: 1.5rem;
		}
	}

	/* Title */
	.popup-title {
		font-size: 1.875rem;
		font-weight: 700;
		color: #111827;
		margin-bottom: 1rem;
		line-height: 1.25;
	}

	@media (max-width: 640px) {
		.popup-title {
			font-size: 1.5rem;
		}
	}

	/* Body */
	.popup-body {
		color: #374151;
		margin-bottom: 1.5rem;
		line-height: 1.625;
	}

	/* Form */
	.popup-form {
		margin-bottom: 1.5rem;
	}

	/* CTA Button */
	.popup-cta-button {
		position: relative;
		width: 100%;
		padding: 0.875rem 1.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: white;
		background-color: #3b82f6;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		overflow: hidden;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		box-shadow:
			0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -1px rgba(0, 0, 0, 0.06);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.popup-cta-button:hover {
		transform: translateY(-2px);
		box-shadow:
			0 10px 15px -3px rgba(0, 0, 0, 0.1),
			0 4px 6px -2px rgba(0, 0, 0, 0.05);
	}

	.popup-cta-button:active {
		transform: translateY(0);
		box-shadow:
			0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -1px rgba(0, 0, 0, 0.06);
	}

	.popup-cta-button:focus-visible {
		outline: 2px solid currentColor;
		outline-offset: 2px;
	}

	.popup-cta-text {
		position: relative;
		z-index: 1;
	}

	/* Button Ripple Effect */
	.button-ripple {
		position: absolute;
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background-color: rgba(255, 255, 255, 0.6);
		transform: translate(-50%, -50%);
		animation: rippleEffect 0.6s ease-out;
		pointer-events: none;
	}

	@keyframes rippleEffect {
		from {
			width: 0;
			height: 0;
			opacity: 1;
		}
		to {
			width: 300px;
			height: 300px;
			opacity: 0;
		}
	}

	/* === ENTRANCE ANIMATIONS === */

	/* Fade */
	.popup-enter-fade {
		animation: fadeIn 0.4s cubic-bezier(0.4, 0, 0.2, 1);
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* Slide */
	.popup-enter-slide {
		animation: slideUp 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	@keyframes slideUp {
		from {
			transform: translateY(100%);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}

	/* Zoom */
	.popup-enter-zoom {
		animation: zoomIn 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	@keyframes zoomIn {
		from {
			transform: scale(0.8);
			opacity: 0;
		}
		to {
			transform: scale(1);
			opacity: 1;
		}
	}

	/* Bounce */
	.popup-enter-bounce {
		animation: bounceIn 0.8s cubic-bezier(0.68, -0.55, 0.265, 1.55);
	}

	@keyframes bounceIn {
		0% {
			transform: scale(0.3);
			opacity: 0;
		}
		50% {
			transform: scale(1.05);
		}
		70% {
			transform: scale(0.9);
		}
		100% {
			transform: scale(1);
			opacity: 1;
		}
	}

	/* Rotate */
	.popup-enter-rotate {
		animation: rotateIn 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	@keyframes rotateIn {
		from {
			transform: rotate(-180deg) scale(0.5);
			opacity: 0;
		}
		to {
			transform: rotate(0) scale(1);
			opacity: 1;
		}
	}

	/* Flip */
	.popup-enter-flip {
		animation: flipIn 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	@keyframes flipIn {
		from {
			transform: perspective(1000px) rotateX(-90deg);
			opacity: 0;
		}
		to {
			transform: perspective(1000px) rotateX(0);
			opacity: 1;
		}
	}

	/* === EXIT ANIMATIONS === */

	/* Fade Out */
	.popup-exit-fade {
		animation: fadeOut 0.3s cubic-bezier(0.4, 0, 1, 1);
	}

	@keyframes fadeOut {
		from {
			opacity: 1;
		}
		to {
			opacity: 0;
		}
	}

	/* Slide Out */
	.popup-exit-slide {
		animation: slideDown 0.3s cubic-bezier(0.4, 0, 1, 1);
	}

	@keyframes slideDown {
		from {
			transform: translateY(0);
			opacity: 1;
		}
		to {
			transform: translateY(100%);
			opacity: 0;
		}
	}

	/* Zoom Out */
	.popup-exit-zoom {
		animation: zoomOut 0.3s cubic-bezier(0.4, 0, 1, 1);
	}

	@keyframes zoomOut {
		from {
			transform: scale(1);
			opacity: 1;
		}
		to {
			transform: scale(0.8);
			opacity: 0;
		}
	}

	/* Bounce Out */
	.popup-exit-bounce {
		animation: bounceOut 0.5s cubic-bezier(0.6, -0.28, 0.735, 0.045);
	}

	@keyframes bounceOut {
		0% {
			transform: scale(1);
		}
		25% {
			transform: scale(0.95);
		}
		50% {
			transform: scale(1.1);
		}
		100% {
			transform: scale(0);
			opacity: 0;
		}
	}

	/* Rotate Out */
	.popup-exit-rotate {
		animation: rotateOut 0.4s cubic-bezier(0.6, -0.28, 0.735, 0.045);
	}

	@keyframes rotateOut {
		from {
			transform: rotate(0) scale(1);
			opacity: 1;
		}
		to {
			transform: rotate(180deg) scale(0.5);
			opacity: 0;
		}
	}

	/* Flip Out */
	.popup-exit-flip {
		animation: flipOut 0.4s cubic-bezier(0.6, -0.28, 0.735, 0.045);
	}

	@keyframes flipOut {
		from {
			transform: perspective(1000px) rotateX(0);
			opacity: 1;
		}
		to {
			transform: perspective(1000px) rotateX(90deg);
			opacity: 0;
		}
	}

	/* Backdrop animations */
	@keyframes backdropFadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes backdropFadeOut {
		from {
			opacity: 1;
		}
		to {
			opacity: 0;
		}
	}

	/* Reduce motion for accessibility */
	@media (prefers-reduced-motion: reduce) {
		.popup-container,
		.popup-backdrop,
		.popup-cta-button,
		.popup-close-button {
			animation: none !important;
			transition: none !important;
		}
	}

	/* High contrast mode support */
	@media (prefers-contrast: high) {
		.popup-container {
			border: 2px solid currentColor;
		}

		.popup-cta-button {
			border: 2px solid currentColor;
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.popup-container {
			background-color: #1f2937;
			color: #f3f4f6;
		}

		.popup-title {
			color: #f3f4f6;
		}

		.popup-body {
			color: #d1d5db;
		}
	}
</style>
