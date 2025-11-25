<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { Card, Button, Input, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { popupsApi, type Popup } from '$lib/api/popups';

	const popupId = parseInt($page.params.id);

	// Form state
	let formData: Partial<Popup> = {
		name: '',
		type: 'exit_intent',
		status: 'draft',
		title: '',
		content: '',
		cta_text: '',
		cta_url: '',
		cta_new_tab: false,
		priority: 10,
		position: 'center',
		size: 'md',
		animation: 'zoom',
		show_close_button: true,
		close_on_overlay_click: true,
		auto_close_after: null,
		has_form: false,
		form_id: null,
		trigger_rules: {},
		frequency_rules: { frequency: 'once_per_session' },
		display_rules: { devices: ['desktop', 'tablet', 'mobile'] },
		design: {
			backgroundColor: '#ffffff',
			titleColor: '#1f2937',
			textColor: '#4b5563',
			buttonColor: '#3b82f6',
			buttonTextColor: '#ffffff'
		}
	};

	let loading = false;
	let initialLoading = true;
	let errors: Record<string, string> = {};

	// Trigger rules based on type
	let timedDelay = 5000;
	let scrollDepth = 50;
	let clickSelector = '[data-popup-trigger]';

	// Options (same as create page)
	const frequencyOptions = [
		{ value: 'once', label: 'Once (never show again)' },
		{ value: 'once_per_session', label: 'Once per session' },
		{ value: 'daily', label: 'Once per day' },
		{ value: 'weekly', label: 'Once per week' },
		{ value: 'always', label: 'Every page view' }
	];

	const typeOptions = [
		{ value: 'newsletter', label: '📧 Newsletter Signup' },
		{ value: 'exit_intent', label: '🚪 Exit Intent' },
		{ value: 'timed', label: '⏱️ Timed Popup' },
		{ value: 'scroll', label: '📜 Scroll Triggered' },
		{ value: 'click_trigger', label: '👆 Click Trigger' },
		{ value: 'content_locker', label: '🔒 Content Locker' }
	];

	const statusOptions = [
		{ value: 'draft', label: 'Draft' },
		{ value: 'published', label: 'Published' },
		{ value: 'paused', label: 'Paused' },
		{ value: 'archived', label: 'Archived' }
	];

	const positionOptions = [
		{ value: 'center', label: 'Center' },
		{ value: 'top', label: 'Top' },
		{ value: 'bottom', label: 'Bottom' },
		{ value: 'corner', label: 'Bottom Right Corner' }
	];

	const sizeOptions = [
		{ value: 'sm', label: 'Small' },
		{ value: 'md', label: 'Medium' },
		{ value: 'lg', label: 'Large' },
		{ value: 'xl', label: 'Extra Large' },
		{ value: 'full', label: 'Full Width' }
	];

	const animationOptions = [
		{ value: 'fade', label: '✨ Fade In' },
		{ value: 'slide', label: '⬆️ Slide Up' },
		{ value: 'zoom', label: '🔍 Zoom In' },
		{ value: 'bounce', label: '🏀 Bounce In' },
		{ value: 'rotate', label: '🔄 Rotate In' },
		{ value: 'flip', label: '🔃 Flip In' }
	];

	onMount(async () => {
		await loadPopup();
	});

	async function loadPopup() {
		try {
			initialLoading = true;
			const response = await popupsApi.get(popupId);
			formData = response.popup;

			// Extract trigger rules
			if (formData.trigger_rules) {
				if (formData.trigger_rules.delay !== undefined) {
					timedDelay = formData.trigger_rules.delay;
				}
				if (formData.trigger_rules.scroll_depth !== undefined) {
					scrollDepth = formData.trigger_rules.scroll_depth;
				}
				if (formData.trigger_rules.selector !== undefined) {
					clickSelector = formData.trigger_rules.selector;
				}
			}

			// Ensure display_rules has devices array
			if (!formData.display_rules) {
				formData.display_rules = { devices: ['desktop', 'tablet', 'mobile'] };
			}
			if (!formData.display_rules.devices) {
				formData.display_rules.devices = ['desktop', 'tablet', 'mobile'];
			}

			// Ensure frequency_rules exists
			if (!formData.frequency_rules) {
				formData.frequency_rules = { frequency: 'once_per_session' };
			}

			// Ensure design exists with all properties
			if (!formData.design) {
				formData.design = {
					backgroundColor: '#ffffff',
					titleColor: '#1f2937',
					textColor: '#4b5563',
					buttonColor: '#3b82f6',
					buttonTextColor: '#ffffff',
					buttonBorderRadius: 8,
					buttonShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
					buttonPadding: '0.875rem 1.5rem'
				};
			} else {
				// Ensure new properties exist on existing design objects
				if (formData.design.buttonBorderRadius === undefined)
					formData.design.buttonBorderRadius = 8;
				if (!formData.design.buttonShadow)
					formData.design.buttonShadow =
						'0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)';
				if (!formData.design.buttonPadding) formData.design.buttonPadding = '0.875rem 1.5rem';
			}
		} catch (error) {
			console.error('Failed to load popup:', error);
			addToast({ type: 'error', message: 'Failed to load popup' });
			goto('/admin/popups');
		} finally {
			initialLoading = false;
		}
	}

	function validateForm(): boolean {
		errors = {};

		if (!formData.name?.trim()) {
			errors.name = 'Name is required';
		}

		if (!formData.title?.trim()) {
			errors.title = 'Title is required';
		}

		if (!formData.content?.trim()) {
			errors.content = 'Content is required';
		}

		return Object.keys(errors).length === 0;
	}

	async function handleSubmit() {
		if (!validateForm()) {
			addToast({ type: 'error', message: 'Please fix the errors' });
			return;
		}

		// Build trigger rules based on type
		const triggerRules: Record<string, any> = {};

		switch (formData.type) {
			case 'timed':
				triggerRules.delay = timedDelay;
				break;
			case 'scroll':
				triggerRules.scroll_depth = scrollDepth;
				break;
			case 'click_trigger':
				triggerRules.selector = clickSelector;
				break;
		}

		formData.trigger_rules = triggerRules;

		try {
			loading = true;
			await popupsApi.update(popupId, formData);
			addToast({ type: 'success', message: 'Popup updated successfully!' });
			goto('/admin/popups');
		} catch (error: any) {
			console.error('Failed to update popup:', error);
			addToast({
				type: 'error',
				message: error.response?.data?.message || 'Failed to update popup'
			});
		} finally {
			loading = false;
		}
	}

	function handleCancel() {
		goto('/admin/popups');
	}
</script>

<svelte:head>
	<title>Edit Popup | Revolution Admin</title>
</svelte:head>

{#if initialLoading}
	<div class="flex items-center justify-center h-96">
		<div class="text-center">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
			<p class="mt-4 text-gray-600">Loading popup...</p>
		</div>
	</div>
{:else}
	<div class="max-w-5xl mx-auto">
		<!-- Header -->
		<div class="mb-6">
			<h1 class="text-3xl font-bold text-gray-900">Edit Popup</h1>
			<p class="text-gray-600 mt-1">Update your popup configuration</p>
		</div>

		<form on:submit|preventDefault={handleSubmit}>
			<div class="space-y-6">
				<!-- Basic Information -->
				<Card>
					<h2 class="text-xl font-semibold mb-4">Basic Information</h2>

					<div class="space-y-4">
						<div>
							<Input
								id="popup-internal-name-1"
								label="Internal Name *"
								bind:value={formData.name}
								placeholder="e.g., Exit Intent - Newsletter"
								error={errors.name}
							/>
							<p class="text-xs text-gray-500 mt-1">For your reference only, not shown to users</p>
						</div>

						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div>
								<Select
									id="popup-popup-type-2"
									label="Popup Type *"
									options={typeOptions}
									bind:value={formData.type}
								/>
							</div>

							<div>
								<Select
									id="popup-status-3"
									label="Status *"
									options={statusOptions}
									bind:value={formData.status}
								/>
							</div>
						</div>

						<div>
							<Input
								id="popup-priority-1100-4"
								label="Priority (1-100)"
								type="number"
								bind:value={formData.priority}
								min="1"
								max="100"
							/>
							<p class="text-xs text-gray-500 mt-1">Higher priority popups are shown first</p>
						</div>
					</div>
				</Card>

				<!-- Content -->
				<Card>
					<h2 class="text-xl font-semibold mb-4">Content</h2>

					<div class="space-y-4">
						<div>
							<Input
								id="popup-title-5"
								label="Title *"
								bind:value={formData.title}
								placeholder="e.g., Wait! Don't Leave Yet"
								error={errors.title}
							/>
						</div>

						<div>
							<label for="popup-content-text">Content *</label>
							<textarea
								id="popup-content-text"
								bind:value={formData.content}
								class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 {errors.content
									? 'border-red-500'
									: ''}"
								rows="4"
								placeholder="Enter your message (HTML allowed)"
							></textarea>
							{#if errors.content}
								<p class="text-xs text-red-600 mt-1">{errors.content}</p>
							{/if}
							<p class="text-xs text-gray-500 mt-1">HTML is supported for formatting</p>
						</div>

						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div>
								<Input
									id="popup-calltoaction-text-6"
									label="Call-to-Action Text"
									bind:value={formData.cta_text}
									placeholder="e.g., Get Started"
								/>
							</div>

							<div>
								<Input
									id="popup-cta-url-7"
									label="CTA URL"
									bind:value={formData.cta_url}
									placeholder="/subscribe"
								/>
							</div>
						</div>

						<div class="flex items-center gap-2">
							<input
								type="checkbox"
								id="cta_new_tab"
								bind:checked={formData.cta_new_tab}
								class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
							/>
							<label for="cta_new_tab" class="text-sm text-gray-700">
								Open CTA link in new tab
							</label>
						</div>
					</div>
				</Card>

				<!-- Trigger Settings -->
				<Card>
					<h2 class="text-xl font-semibold mb-4">Trigger Settings</h2>

					<div class="space-y-4">
						{#if formData.type === 'timed'}
							<div>
								<Input
									id="popup-delay-milliseconds-8"
									label="Delay (milliseconds)"
									type="number"
									bind:value={timedDelay}
									min="0"
									step="1000"
								/>
								<p class="text-xs text-gray-500 mt-1">
									Show popup after {(timedDelay / 1000).toFixed(0)} seconds
								</p>
							</div>
						{:else if formData.type === 'scroll'}
							<div>
								<Input
									id="popup-scroll-depth-9"
									label="Scroll Depth (%)"
									type="number"
									bind:value={scrollDepth}
									min="0"
									max="100"
								/>
								<p class="text-xs text-gray-500 mt-1">
									Show popup when user scrolls {scrollDepth}% down the page
								</p>
							</div>
						{:else if formData.type === 'click_trigger'}
							<div>
								<Input
									id="popup-css-selector-10"
									label="CSS Selector"
									bind:value={clickSelector}
									placeholder="[data-popup-trigger]"
								/>
								<p class="text-xs text-gray-500 mt-1">
									Show popup when elements matching this selector are clicked
								</p>
							</div>
						{:else if formData.type === 'exit_intent'}
							<div class="bg-blue-50 border border-blue-200 rounded-md p-4">
								<p class="text-sm text-blue-800">
									Exit intent popups trigger when the user's mouse moves toward the browser's top
									edge (leaving the page).
								</p>
							</div>
						{:else if formData.type === 'newsletter'}
							<div class="bg-blue-50 border border-blue-200 rounded-md p-4">
								<p class="text-sm text-blue-800">
									Newsletter popups display immediately or can be combined with timing/scroll
									settings.
								</p>
							</div>
						{/if}
					</div>
				</Card>

				<!-- Display Settings -->
				<Card>
					<h2 class="text-xl font-semibold mb-4">Display Settings</h2>

					<div class="space-y-4">
						<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
							<div>
								<Select
									id="popup-position-11"
									label="Position"
									options={positionOptions}
									bind:value={formData.position}
								/>
							</div>

							<div>
								<Select
									id="popup-size-12"
									label="Size"
									options={sizeOptions}
									bind:value={formData.size}
								/>
							</div>

							<div>
								<Select
									id="popup-animation-13"
									label="Animation"
									options={animationOptions}
									bind:value={formData.animation}
								/>
							</div>
						</div>

						<div>
							<Input
								id="popup-auto-close-after-seconds-14"
								label="Auto Close After (seconds)"
								type="number"
								bind:value={formData.auto_close_after}
								min="0"
								placeholder="Leave empty for manual close only"
							/>
						</div>

						<div class="space-y-2">
							<div class="flex items-center gap-2">
								<input
									type="checkbox"
									id="show_close_button"
									bind:checked={formData.show_close_button}
									class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
								/>
								<label for="show_close_button" class="text-sm text-gray-700">
									Show close button (X)
								</label>
							</div>

							<div class="flex items-center gap-2">
								<input
									type="checkbox"
									id="close_on_overlay_click"
									bind:checked={formData.close_on_overlay_click}
									class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
								/>
								<label for="close_on_overlay_click" class="text-sm text-gray-700">
									Close when clicking outside popup
								</label>
							</div>
						</div>
					</div>
				</Card>

				<!-- Frequency Rules -->
				<Card>
					<h2 class="text-xl font-semibold mb-4">Frequency & Targeting</h2>

					<div class="space-y-4">
						<div>
							<Select
								id="popup-show-frequency-15"
								label="Show Frequency"
								options={frequencyOptions}
								bind:value={formData.frequency_rules.frequency}
							/>
						</div>

						<div>
							<span class="block text-sm font-medium text-gray-700 mb-2">Target Devices</span>
							<div class="flex gap-4">
								<label class="flex items-center gap-2">
									<input
										type="checkbox"
										value="desktop"
										bind:group={formData.display_rules.devices}
										class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
									/>
									<span class="text-sm text-gray-700">Desktop</span>
								</label>
								<label class="flex items-center gap-2">
									<input
										type="checkbox"
										value="tablet"
										bind:group={formData.display_rules.devices}
										class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
									/>
									<span class="text-sm text-gray-700">Tablet</span>
								</label>
								<label class="flex items-center gap-2">
									<input
										type="checkbox"
										value="mobile"
										bind:group={formData.display_rules.devices}
										class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
									/>
									<span class="text-sm text-gray-700">Mobile</span>
								</label>
							</div>
						</div>
					</div>
				</Card>

				<!-- Design Customization -->
				<Card>
					<h2 class="text-xl font-semibold mb-4">🎨 Design Customization</h2>

					<div class="space-y-6">
						<!-- Popup Colors -->
						<div>
							<h3 class="text-sm font-semibold text-gray-800 mb-3">Popup Colors</h3>
							<div class="grid grid-cols-2 md:grid-cols-3 gap-4">
								<div>
									<Input
										id="popup-background-color-16"
										label="Background Color"
										type="color"
										bind:value={formData.design.backgroundColor}
									/>
								</div>

								<div>
									<Input
										id="popup-title-color-17"
										label="Title Color"
										type="color"
										bind:value={formData.design.titleColor}
									/>
								</div>

								<div>
									<Input
										id="popup-text-color-18"
										label="Text Color"
										type="color"
										bind:value={formData.design.textColor}
									/>
								</div>
							</div>
						</div>

						<!-- Button Styling -->
						<div>
							<h3 class="text-sm font-semibold text-gray-800 mb-3">Button Styling</h3>
							<div class="grid grid-cols-2 md:grid-cols-3 gap-4">
								<div>
									<Input
										id="popup-button-color-19"
										label="Button Color"
										type="color"
										bind:value={formData.design.buttonColor}
									/>
								</div>

								<div>
									<Input
										id="popup-button-text-color-20"
										label="Button Text Color"
										type="color"
										bind:value={formData.design.buttonTextColor}
									/>
								</div>

								<div>
									<Input
										id="popup-button-border-radius-px-21"
										label="Button Border Radius (px)"
										type="number"
										bind:value={formData.design.buttonBorderRadius}
										min="0"
										max="50"
										placeholder="8"
									/>
								</div>
							</div>
						</div>

						<!-- Advanced Button Effects -->
						<div>
							<h3 class="text-sm font-semibold text-gray-800 mb-3">Button Effects</h3>
							<div class="space-y-3">
								<div>
									<label
										for="button-shadow-edit"
										class="block text-sm font-medium text-gray-700 mb-1"
									>
										Button Shadow
									</label>
									<select
										id="button-shadow-edit"
										bind:value={formData.design.buttonShadow}
										class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
									>
										<option value="">None</option>
										<option value="0 1px 2px 0 rgba(0, 0, 0, 0.05)">Subtle</option>
										<option
											value="0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)"
											>Default</option
										>
										<option
											value="0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)"
											>Medium</option
										>
										<option
											value="0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)"
											>Large</option
										>
										<option value="0 25px 50px -12px rgba(0, 0, 0, 0.25)">Extra Large</option>
									</select>
									<p class="text-xs text-gray-500 mt-1">Shadow adds depth to the button</p>
								</div>

								<div>
									<label
										for="button-padding-edit"
										class="block text-sm font-medium text-gray-700 mb-1"
									>
										Button Padding
									</label>
									<select
										id="button-padding-edit"
										bind:value={formData.design.buttonPadding}
										class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
									>
										<option value="0.5rem 1rem">Small (0.5rem 1rem)</option>
										<option value="0.875rem 1.5rem">Default (0.875rem 1.5rem)</option>
										<option value="1rem 2rem">Medium (1rem 2rem)</option>
										<option value="1.25rem 2.5rem">Large (1.25rem 2.5rem)</option>
										<option value="1.5rem 3rem">Extra Large (1.5rem 3rem)</option>
									</select>
									<p class="text-xs text-gray-500 mt-1">Adjust button size and spacing</p>
								</div>
							</div>
						</div>
					</div>
				</Card>

				<!-- Stats Summary (Read-only) -->
				{#if formData.total_views !== undefined}
					<Card>
						<h2 class="text-xl font-semibold mb-4">Performance</h2>

						<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
							<div>
								<p class="text-sm text-gray-600">Views</p>
								<p class="text-2xl font-bold">{formData.total_views?.toLocaleString() || 0}</p>
							</div>
							<div>
								<p class="text-sm text-gray-600">Conversions</p>
								<p class="text-2xl font-bold text-green-600">
									{formData.total_conversions?.toLocaleString() || 0}
								</p>
							</div>
							<div>
								<p class="text-sm text-gray-600">Conversion Rate</p>
								<p class="text-2xl font-bold text-blue-600">{formData.conversion_rate || 0}%</p>
							</div>
							<div>
								<p class="text-sm text-gray-600">Status</p>
								<p class="text-sm font-semibold text-gray-700 uppercase mt-2">
									{formData.performance_status || 'N/A'}
								</p>
							</div>
						</div>

						<div class="mt-4">
							<Button variant="outline" on:click={() => goto(`/admin/popups/${popupId}/analytics`)}>
								View Full Analytics
							</Button>
						</div>
					</Card>
				{/if}

				<!-- Actions -->
				<div class="flex justify-end gap-3">
					<Button type="button" variant="ghost" on:click={handleCancel} disabled={loading}>
						Cancel
					</Button>
					<Button type="submit" disabled={loading}>
						{loading ? 'Saving...' : 'Save Changes'}
					</Button>
				</div>
			</div>
		</form>
	</div>
{/if}
