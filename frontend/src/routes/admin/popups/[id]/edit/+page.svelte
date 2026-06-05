<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { Card, Button, Input, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { popupsApi, type Popup } from '$lib/api/popups';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';

	const popupId = parseInt(page.params['id'] ?? '0');

	// Form state
	let formData = $state<Partial<Popup>>({
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
		has_form: false,
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
	});

	let loading = $state(false);
	let initialLoading = $state(true);
	let errors = $state<Record<string, string>>({});
	let _showAbTestModal = $state(false);

	// Trigger rules based on type
	let timedDelay = $state(5000);
	let scrollDepth = $state(50);
	let clickSelector = $state('[data-popup-trigger]');

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
			if (response.popup) {
				formData = response.popup;
			}

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

		if (!formData['name']?.trim()) {
			errors['name'] = 'Name is required';
		}

		if (!formData['title']?.trim()) {
			errors['title'] = 'Title is required';
		}

		if (!formData['content']?.trim()) {
			errors['content'] = 'Content is required';
		}

		return Object.keys(errors).length === 0;
	}

	async function handleSubmit() {
		if (!validateForm()) {
			addToast({ type: 'error', message: 'Please fix the errors' });
			return;
		}

		// Build trigger rules based on type
		const triggerRules: Record<string, unknown> = {};

		switch (formData.type) {
			case 'timed':
				triggerRules['delay'] = timedDelay;
				break;
			case 'scroll':
				triggerRules['scroll_depth'] = scrollDepth;
				break;
			case 'click_trigger':
				triggerRules['selector'] = clickSelector;
				break;
		}

		formData.trigger_rules = triggerRules;

		try {
			loading = true;
			await popupsApi.update(popupId, formData);
			addToast({ type: 'success', message: 'Popup updated successfully!' });
			goto('/admin/popups');
		} catch (error) {
			console.error('Failed to update popup:', error);
			const err = error as { response?: { data?: { message?: string } } };
			addToast({
				type: 'error',
				message: err.response?.data?.message || 'Failed to update popup'
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
	<div class="loading-state">
		<div class="loading-panel">
			<div class="loading-spinner"></div>
			<p class="loading-copy">Loading popup...</p>
		</div>
	</div>
{:else}
	<div class="popup-edit-page">
		<!-- Header -->
		<div class="page-header">
			<h1 class="page-title">Edit Popup</h1>
			<p class="page-subtitle">Update your popup configuration</p>
		</div>

		<form onsubmit={handleSubmit}>
			<div class="form-stack">
				<!-- Basic Information -->
				<Card>
					<h2 class="section-heading">Basic Information</h2>

					<div class="field-stack">
						<div>
							<Input
								id="popup-internal-name-1"
								label="Internal Name *"
								bind:value={formData['name']}
								placeholder="e.g., Exit Intent - Newsletter"
								{...errors['name'] && { error: errors['name'] }}
							/>
							<p class="field-help">For your reference only, not shown to users</p>
						</div>

						<div class="form-grid form-grid--two">
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
							<p class="field-help">Higher priority popups are shown first</p>
						</div>
					</div>
				</Card>

				<!-- Content -->
				<Card>
					<h2 class="section-heading">Content</h2>

					<div class="field-stack">
						<div>
							<Input
								id="popup-title-5"
								label="Title *"
								bind:value={formData['title']}
								placeholder="e.g., Wait! Don't Leave Yet"
								{...errors['title'] && { error: errors['title'] }}
							/>
						</div>

						<div>
							<label for="popup-content-text" class="field-label">Content *</label>
							<textarea
								id="popup-content-text"
								bind:value={formData['content']}
								class={{ 'native-control': true, 'has-error': Boolean(errors['content']) }}
								rows="4"
								placeholder="Enter your message (HTML allowed)"
							></textarea>
							{#if errors['content']}
								<p class="field-error">{errors['content']}</p>
							{/if}
							<p class="field-help">HTML is supported for formatting</p>
						</div>

						<div class="form-grid form-grid--two">
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

						<div class="checkbox-row">
							<input
								type="checkbox"
								id="cta_new_tab"
								name="cta_new_tab"
								bind:checked={formData.cta_new_tab}
								class="checkbox-control"
							/>
							<label for="cta_new_tab" class="checkbox-label"> Open CTA link in new tab </label>
						</div>
					</div>
				</Card>

				<!-- Trigger Settings -->
				<Card>
					<h2 class="section-heading">Trigger Settings</h2>

					<div class="field-stack">
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
								<p class="field-help">
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
								<p class="field-help">
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
								<p class="field-help">
									Show popup when elements matching this selector are clicked
								</p>
							</div>
						{:else if formData.type === 'exit_intent'}
							<div class="info-callout">
								<p>
									Exit intent popups trigger when the user's mouse moves toward the browser's top
									edge (leaving the page).
								</p>
							</div>
						{:else if formData.type === 'newsletter'}
							<div class="info-callout">
								<p>
									Newsletter popups display immediately or can be combined with timing/scroll
									settings.
								</p>
							</div>
						{/if}
					</div>
				</Card>

				<!-- Display Settings -->
				<Card>
					<h2 class="section-heading">Display Settings</h2>

					<div class="field-stack">
						<div class="form-grid form-grid--three">
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

						<div class="check-stack">
							<div class="checkbox-row">
								<input
									type="checkbox"
									id="show_close_button"
									name="show_close_button"
									bind:checked={formData.show_close_button}
									class="checkbox-control"
								/>
								<label for="show_close_button" class="checkbox-label">
									Show close button (X)
								</label>
							</div>

							<div class="checkbox-row">
								<input
									type="checkbox"
									id="close_on_overlay_click"
									name="close_on_overlay_click"
									bind:checked={formData.close_on_overlay_click}
									class="checkbox-control"
								/>
								<label for="close_on_overlay_click" class="checkbox-label">
									Close when clicking outside popup
								</label>
							</div>
						</div>
					</div>
				</Card>

				<!-- Frequency Rules -->
				<Card>
					<h2 class="section-heading">Frequency & Targeting</h2>

					<div class="field-stack">
						<div>
							<Select
								id="popup-show-frequency-15"
								label="Show Frequency"
								options={frequencyOptions}
								bind:value={formData.frequency_rules.frequency}
							/>
						</div>

						<div>
							<span class="field-label">Target Devices</span>
							<div class="device-options">
								<label class="checkbox-row">
									<input
										id="device-desktop"
										name="device-desktop"
										type="checkbox"
										value="desktop"
										bind:group={formData.display_rules.devices}
										class="checkbox-control"
									/>
									<span class="checkbox-label">Desktop</span>
								</label>
								<label class="checkbox-row">
									<input
										id="device-tablet"
										name="device-tablet"
										type="checkbox"
										value="tablet"
										bind:group={formData.display_rules.devices}
										class="checkbox-control"
									/>
									<span class="checkbox-label">Tablet</span>
								</label>
								<label class="checkbox-row">
									<input
										id="device-mobile"
										name="device-mobile"
										type="checkbox"
										value="mobile"
										bind:group={formData.display_rules.devices}
										class="checkbox-control"
									/>
									<span class="checkbox-label">Mobile</span>
								</label>
							</div>
						</div>
					</div>
				</Card>

				<!-- Design Customization -->
				<Card>
					<h2 class="section-heading">🎨 Design Customization</h2>

					<div class="section-stack">
						<!-- Popup Colors -->
						<div>
							<h3 class="subsection-heading">Popup Colors</h3>
							<div class="form-grid form-grid--three">
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
							<h3 class="subsection-heading">Button Styling</h3>
							<div class="form-grid form-grid--three">
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
							<h3 class="subsection-heading">Button Effects</h3>
							<div class="field-stack field-stack--compact">
								<div>
									<label for="button-shadow-edit" class="field-label"> Button Shadow </label>
									<select
										id="button-shadow-edit"
										bind:value={formData.design.buttonShadow}
										class="native-control"
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
									<p class="field-help">Shadow adds depth to the button</p>
								</div>

								<div>
									<label for="button-padding-edit" class="field-label"> Button Padding </label>
									<select
										id="button-padding-edit"
										bind:value={formData.design.buttonPadding}
										class="native-control"
									>
										<option value="0.5rem 1rem">Small (0.5rem 1rem)</option>
										<option value="0.875rem 1.5rem">Default (0.875rem 1.5rem)</option>
										<option value="1rem 2rem">Medium (1rem 2rem)</option>
										<option value="1.25rem 2.5rem">Large (1.25rem 2.5rem)</option>
										<option value="1.5rem 3rem">Extra Large (1.5rem 3rem)</option>
									</select>
									<p class="field-help">Adjust button size and spacing</p>
								</div>
							</div>
						</div>
					</div>
				</Card>

				<!-- Scheduling -->
				<Card>
					<h2 class="section-heading">Scheduling</h2>
					<p class="section-copy">
						Set start and end dates to control when this popup is active. Leave empty for always-on.
					</p>

					<div class="form-grid form-grid--two">
						<div>
							<Input
								id="popup-start-date"
								label="Start Date"
								type="datetime-local"
								bind:value={formData.start_date}
							/>
							<p class="field-help">When the popup should start showing</p>
						</div>

						<div>
							<Input
								id="popup-end-date"
								label="End Date"
								type="datetime-local"
								bind:value={formData.end_date}
							/>
							<p class="field-help">When the popup should stop showing</p>
						</div>
					</div>

					{#if formData.start_date || formData.end_date}
						<div class="schedule-preview">
							<h4>Schedule Preview</h4>
							<p>
								{#if formData.start_date && formData.end_date}
									Active from <strong>{new Date(formData.start_date).toLocaleString()}</strong>
									to <strong>{new Date(formData.end_date).toLocaleString()}</strong>
								{:else if formData.start_date}
									Starts <strong>{new Date(formData.start_date).toLocaleString()}</strong>
									(no end date)
								{:else if formData.end_date}
									Ends <strong>{new Date(formData.end_date).toLocaleString()}</strong>
									(already started)
								{/if}
							</p>
						</div>
					{/if}
				</Card>

				<!-- A/B Testing -->
				<Card>
					<h2 class="section-heading">A/B Testing</h2>
					<p class="section-copy">
						Create variants of this popup to test different content, designs, or CTAs.
					</p>

					{#if formData.abTestId}
						<div class="active-test-card">
							<div class="active-test-header">
								<span class="status-pill status-pill--success">A/B Test Active</span>
								<span class="active-test-id">Test ID: {formData.abTestId}</span>
							</div>
							{#if formData.variantTitle}
								<p class="active-test-copy">
									This is variant: <strong>{formData.variantTitle}</strong>
								</p>
							{/if}
						</div>
					{:else}
						<div class="empty-test-card">
							<div class="empty-test-icon">
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: chart-bar w-12 h-12 -->
								<IconChartBar size={48} aria-hidden="true" />
							</div>
							<h3 class="empty-test-title">No A/B Test Running</h3>
							<p class="empty-test-copy">
								Create an A/B test to compare different versions of this popup and find what works
								best.
							</p>
							<Button variant="outline" type="button" onclick={() => (_showAbTestModal = true)}>
								Create A/B Test
							</Button>
						</div>
					{/if}

					<div class="tips-card">
						<h4 class="tips-title">Tips for A/B Testing</h4>
						<ul class="tips-list">
							<li>Test one element at a time (headline, CTA, image)</li>
							<li>Run tests for at least 7 days for statistical significance</li>
							<li>Aim for at least 1000 impressions per variant</li>
							<li>Focus on conversion rate, not just clicks</li>
						</ul>
					</div>
				</Card>

				<!-- Stats Summary (Read-only) -->
				{#if formData.total_views !== undefined}
					<Card>
						<h2 class="section-heading">Performance</h2>

						<div class="performance-grid">
							<div class="metric-card">
								<p class="metric-label">Views</p>
								<p class="metric-value">{formData.total_views?.toLocaleString() || 0}</p>
							</div>
							<div class="metric-card">
								<p class="metric-label">Conversions</p>
								<p class="metric-value metric-value--success">
									{formData.total_conversions?.toLocaleString() || 0}
								</p>
							</div>
							<div class="metric-card">
								<p class="metric-label">Conversion Rate</p>
								<p class="metric-value metric-value--accent">{formData.conversion_rate || 0}%</p>
							</div>
							<div class="metric-card">
								<p class="metric-label">Status</p>
								<p class="metric-status">
									{formData.performance_status || 'N/A'}
								</p>
							</div>
						</div>

						<div class="inline-actions">
							<Button variant="outline" onclick={() => goto(`/admin/popups/${popupId}/analytics`)}>
								View Full Analytics
							</Button>
						</div>
					</Card>
				{/if}

				<!-- Actions -->
				<div class="form-actions">
					<Button type="button" variant="ghost" onclick={handleCancel} disabled={loading}>
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

<style>
	.loading-state {
		display: grid;
		min-height: 24rem;
		place-items: center;
	}

	.loading-panel {
		text-align: center;
	}

	.loading-spinner {
		width: 3rem;
		height: 3rem;
		margin: 0 auto;
		border: 2px solid rgba(37, 99, 235, 0.2);
		border-bottom-color: #2563eb;
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.loading-copy {
		margin: 1rem 0 0;
		color: #4b5563;
	}

	.popup-edit-page {
		max-width: 64rem;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 1.5rem;
	}

	.page-title {
		margin: 0;
		color: #111827;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.page-subtitle {
		margin: 0.25rem 0 0;
		color: #4b5563;
	}

	.form-stack,
	.section-stack {
		display: grid;
		gap: 1.5rem;
	}

	.field-stack {
		display: grid;
		gap: 1rem;
	}

	.field-stack--compact {
		gap: 0.75rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: minmax(0, 1fr);
		gap: 1rem;
	}

	.section-heading {
		margin: 0 0 1rem;
		color: #111827;
		font-size: 1.25rem;
		font-weight: 700;
		line-height: 1.3;
	}

	.section-copy {
		margin: 0 0 1rem;
		color: #4b5563;
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.subsection-heading {
		margin: 0 0 0.75rem;
		color: #1f2937;
		font-size: 0.875rem;
		font-weight: 700;
		line-height: 1.4;
	}

	.field-label {
		display: block;
		margin: 0 0 0.25rem;
		color: #374151;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.field-help,
	.field-error {
		margin: 0.25rem 0 0;
		font-size: 0.75rem;
		line-height: 1.5;
	}

	.field-help {
		color: #6b7280;
	}

	.field-error {
		color: #dc2626;
	}

	.native-control {
		width: 100%;
		min-height: 2.5rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: #ffffff;
		color: #111827;
		padding: 0.5rem 0.75rem;
		font: inherit;
		line-height: 1.5;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.native-control:focus {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.18);
		outline: none;
	}

	.native-control.has-error {
		border-color: #ef4444;
	}

	textarea.native-control {
		min-height: 7rem;
		resize: vertical;
	}

	.checkbox-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
	}

	.checkbox-control {
		width: 1rem;
		height: 1rem;
		flex: 0 0 auto;
		border-color: #d1d5db;
		accent-color: #2563eb;
	}

	.checkbox-label {
		color: #374151;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.check-stack {
		display: grid;
		gap: 0.5rem;
	}

	.device-options {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem 1rem;
	}

	.info-callout,
	.schedule-preview {
		border: 1px solid #bfdbfe;
		border-radius: 6px;
		background: #eff6ff;
		padding: 1rem;
	}

	.info-callout p,
	.schedule-preview p {
		margin: 0;
		color: #1e40af;
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.schedule-preview {
		margin-top: 1rem;
		padding: 0.75rem;
	}

	.schedule-preview h4 {
		margin: 0 0 0.25rem;
		color: #1e40af;
		font-size: 0.875rem;
		font-weight: 700;
		line-height: 1.4;
	}

	.active-test-card {
		margin-bottom: 1rem;
		border: 1px solid #bbf7d0;
		border-radius: 6px;
		background: #f0fdf4;
		padding: 1rem;
	}

	.active-test-header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.status-pill {
		display: inline-flex;
		align-items: center;
		border-radius: 999px;
		padding: 0.125rem 0.625rem;
		font-size: 0.75rem;
		font-weight: 600;
		line-height: 1.5;
	}

	.status-pill--success {
		background: #dcfce7;
		color: #166534;
	}

	.active-test-id {
		color: #15803d;
		font-size: 0.875rem;
	}

	.active-test-copy {
		margin: 0;
		color: #166534;
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.empty-test-card {
		border: 2px dashed #e5e7eb;
		border-radius: 8px;
		padding: 1.5rem;
		text-align: center;
	}

	.empty-test-icon {
		display: grid;
		margin-bottom: 0.75rem;
		color: #9ca3af;
		place-items: center;
	}

	.empty-test-title {
		margin: 0 0 0.5rem;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.empty-test-copy {
		max-width: 36rem;
		margin: 0 auto 1rem;
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.tips-card {
		margin-top: 1rem;
		border-radius: 6px;
		background: #f9fafb;
		padding: 1rem;
	}

	.tips-title {
		margin: 0 0 0.5rem;
		color: #1f2937;
		font-size: 0.875rem;
		font-weight: 700;
		line-height: 1.4;
	}

	.tips-list {
		display: grid;
		gap: 0.25rem;
		margin: 0;
		padding-left: 1.25rem;
		color: #4b5563;
		font-size: 0.875rem;
		line-height: 1.6;
	}

	.performance-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
	}

	.metric-card {
		min-width: 0;
	}

	.metric-label {
		margin: 0;
		color: #4b5563;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.metric-value {
		margin: 0.125rem 0 0;
		color: #111827;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.metric-value--success {
		color: #16a34a;
	}

	.metric-value--accent {
		color: #2563eb;
	}

	.metric-status {
		margin: 0.5rem 0 0;
		color: #374151;
		font-size: 0.875rem;
		font-weight: 700;
		letter-spacing: 0.04em;
		line-height: 1.5;
		text-transform: uppercase;
	}

	.inline-actions {
		margin-top: 1rem;
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
	}

	@media (min-width: 768px) {
		.form-grid--two {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}

		.form-grid--three {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}

		.performance-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (max-width: 767.98px) {
		.popup-edit-page {
			padding-inline: 0.25rem;
		}

		.form-actions {
			flex-direction: column-reverse;
		}
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
