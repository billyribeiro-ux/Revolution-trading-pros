<script lang="ts">
	import { goto } from '$app/navigation';
	import { Card, Input, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { popupsApi, type Popup } from '$lib/api/popups';

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
		display_rules: {
			devices: ['desktop', 'tablet', 'mobile'],
			include_pages: [],
			exclude_pages: []
		},
		design: {
			backgroundColor: '#ffffff',
			titleColor: '#1f2937',
			textColor: '#4b5563',
			buttonColor: '#3b82f6',
			buttonTextColor: '#ffffff',
			buttonBorderRadius: 8,
			buttonShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
			buttonPadding: '0.875rem 1.5rem',
			overlayColor: '#000000',
			overlayOpacity: 50,
			overlayBlur: 4
		}
	});

	let loading = $state(false);
	let errors = $state<Record<string, string>>({});
	let showPreview = $state(false);

	// Trigger rules based on type
	let timedDelay = $state(5000);
	let scrollDepth = $state(50);
	let clickSelector = $state('[data-popup-trigger]');
	let inactivityTimeout = $state(30);

	// Page targeting
	let includePages = $state('');
	let excludePages = $state('');

	// Secondary CTA
	let hasSecondaryCta = $state(false);
	let secondaryCtaText = $state('No thanks');
	let secondaryCtaAction = $state<'close' | 'url'>('close');
	let secondaryCtaUrl = $state('');

	// Video embed
	let hasVideo = $state(false);
	let videoUrl = $state('');
	let videoAutoplay = $state(false);

	// Form integration
	let hasForm = $state(false);
	let selectedFormId = $state<string | null>(null);
	let availableForms = $state<Array<{ id: string; name: string }>>([]);

	// Fetch available forms on mount
	$effect(() => {
		loadForms();
	});

	async function loadForms() {
		try {
			const response = await fetch('/api/admin/forms');
			if (response.ok) {
				const data = await response.json();
				availableForms = data.forms || data || [];
			}
		} catch (e) {
			console.error('Failed to load forms:', e);
		}
	}

	// Template selection
	let selectedTemplate = $state<string | null>(null);

	// Popup templates definitions
	const templates: Record<string, Partial<Popup>> = {
		newsletter: {
			name: 'Newsletter Signup',
			type: 'timed',
			title: 'Join Our Newsletter',
			content: '<p>Get exclusive trading insights delivered to your inbox weekly.</p>',
			cta_text: 'Subscribe Now',
			cta_url: '/subscribe',
			design: {
				backgroundColor: '#1e293b',
				titleColor: '#f1f5f9',
				textColor: '#94a3b8',
				buttonColor: '#3b82f6',
				buttonTextColor: '#ffffff',
				buttonBorderRadius: 8,
				overlayColor: '#000000',
				overlayOpacity: 70
			},
			frequency_rules: { frequency: 'once' }
		},
		exit_intent: {
			name: 'Exit Intent Offer',
			type: 'exit_intent',
			title: 'Wait! Before You Go...',
			content: '<p>Get 10% off your first purchase with code <strong>SAVE10</strong></p>',
			cta_text: 'Claim Offer',
			cta_url: '/pricing?coupon=SAVE10',
			design: {
				backgroundColor: '#7c3aed',
				titleColor: '#ffffff',
				textColor: '#e9d5ff',
				buttonColor: '#fbbf24',
				buttonTextColor: '#1f2937',
				buttonBorderRadius: 8,
				overlayColor: '#000000',
				overlayOpacity: 80
			},
			frequency_rules: { frequency: 'once_per_session' }
		},
		promotional: {
			name: 'Promotional Banner',
			type: 'timed',
			title: 'Limited Time Offer!',
			content: '<p>Save 25% on all courses this week only.</p>',
			cta_text: 'Shop Now',
			cta_url: '/courses',
			design: {
				backgroundColor: '#dc2626',
				titleColor: '#ffffff',
				textColor: '#fecaca',
				buttonColor: '#ffffff',
				buttonTextColor: '#dc2626',
				buttonBorderRadius: 8,
				overlayColor: '#000000',
				overlayOpacity: 60
			},
			frequency_rules: { frequency: 'daily' }
		},
		announcement: {
			name: 'Announcement',
			type: 'timed',
			title: 'Important Update',
			content: '<p>We have exciting new features to share with you!</p>',
			cta_text: 'Learn More',
			cta_url: '/blog/announcement',
			design: {
				backgroundColor: '#0ea5e9',
				titleColor: '#ffffff',
				textColor: '#e0f2fe',
				buttonColor: '#ffffff',
				buttonTextColor: '#0ea5e9',
				buttonBorderRadius: 8,
				overlayColor: '#000000',
				overlayOpacity: 50
			},
			frequency_rules: { frequency: 'once' }
		},
		countdown: {
			name: 'Countdown Timer',
			type: 'timed',
			title: 'Flash Sale Ends Soon!',
			content: '<p>Hurry! Only <strong>3 hours</strong> left to save big.</p>',
			cta_text: 'Get Deal',
			cta_url: '/deals',
			design: {
				backgroundColor: '#f97316',
				titleColor: '#ffffff',
				textColor: '#fed7aa',
				buttonColor: '#1f2937',
				buttonTextColor: '#ffffff',
				buttonBorderRadius: 8,
				overlayColor: '#000000',
				overlayOpacity: 70
			},
			frequency_rules: { frequency: 'always' }
		},
		video: {
			name: 'Video Popup',
			type: 'click_trigger',
			title: 'Watch Our Demo',
			content: '<p>See how our platform can help you succeed.</p>',
			cta_text: 'Get Started',
			cta_url: '/signup',
			design: {
				backgroundColor: '#1f2937',
				titleColor: '#f1f5f9',
				textColor: '#9ca3af',
				buttonColor: '#10b981',
				buttonTextColor: '#ffffff',
				buttonBorderRadius: 8,
				overlayColor: '#000000',
				overlayOpacity: 85
			},
			frequency_rules: { frequency: 'once_per_session' }
		},
		social_proof: {
			name: 'Social Proof',
			type: 'scroll',
			title: 'Join 10,000+ Traders',
			content: '<p>See why professionals trust Revolution Trading Pros.</p>',
			cta_text: 'Join Now',
			cta_url: '/signup',
			design: {
				backgroundColor: '#065f46',
				titleColor: '#ffffff',
				textColor: '#a7f3d0',
				buttonColor: '#fbbf24',
				buttonTextColor: '#1f2937',
				buttonBorderRadius: 8,
				overlayColor: '#000000',
				overlayOpacity: 60
			},
			frequency_rules: { frequency: 'once_per_session' }
		}
	};

	function applyTemplate(templateId: string): void {
		if (templateId === 'scratch') {
			selectedTemplate = null;
			// Reset to defaults
			formData = {
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
				display_rules: {
					devices: ['desktop', 'tablet', 'mobile'],
					include_pages: [],
					exclude_pages: []
				},
				design: {
					backgroundColor: '#ffffff',
					titleColor: '#1f2937',
					textColor: '#4b5563',
					buttonColor: '#3b82f6',
					buttonTextColor: '#ffffff',
					buttonBorderRadius: 8,
					buttonShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
					buttonPadding: '0.875rem 1.5rem',
					overlayColor: '#000000',
					overlayOpacity: 50,
					overlayBlur: 4
				}
			};
			return;
		}

		const template = templates[templateId];
		if (template) {
			selectedTemplate = templateId;
			formData = {
				...formData,
				...template,
				design: {
					...formData.design,
					...template.design
				}
			};
		}
	}

	// Frequency options
	const frequencyOptions = [
		{ value: 'once', label: 'Once (never show again)' },
		{ value: 'once_per_session', label: 'Once per session' },
		{ value: 'daily', label: 'Once per day' },
		{ value: 'weekly', label: 'Once per week' },
		{ value: 'always', label: 'Every page view' }
	];

	// Type options
	const typeOptions = [
		{ value: 'newsletter', label: 'Newsletter Signup' },
		{ value: 'exit_intent', label: 'Exit Intent' },
		{ value: 'timed', label: 'Timed Popup' },
		{ value: 'scroll', label: 'Scroll Triggered' },
		{ value: 'click_trigger', label: 'Click Trigger' },
		{ value: 'inactivity', label: 'Inactivity Trigger' },
		{ value: 'content_locker', label: 'Content Locker' }
	];

	const statusOptions = [
		{ value: 'draft', label: 'Draft' },
		{ value: 'published', label: 'Published' },
		{ value: 'paused', label: 'Paused' }
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
		const triggerRules: Record<string, any> = {};

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
			case 'exit_intent':
				triggerRules['exit_intent'] = true;
				break;
			case 'inactivity':
				triggerRules['inactivity_timeout'] = inactivityTimeout;
				break;
		}

		formData.trigger_rules = triggerRules;

		// Build display rules with page targeting
		formData.display_rules = {
			...formData.display_rules,
			include_pages: includePages.split('\n').filter(p => p.trim()),
			exclude_pages: excludePages.split('\n').filter(p => p.trim())
		};

		// Add video embed if enabled
		if (hasVideo && videoUrl) {
			formData.design = {
				...formData.design,
				video_url: videoUrl,
				video_autoplay: videoAutoplay
			};
		}

		// Add secondary CTA if enabled
		if (hasSecondaryCta) {
			formData.design = {
				...formData.design,
				secondary_cta_text: secondaryCtaText,
				secondary_cta_action: secondaryCtaAction,
				secondary_cta_url: secondaryCtaAction === 'url' ? secondaryCtaUrl : undefined
			};
		}

		// Add form integration if enabled
		if (hasForm && selectedFormId) {
			formData.has_form = true;
			formData.form_id = selectedFormId;
		}

		try {
			loading = true;
			await popupsApi.create(formData);
			addToast({ type: 'success', message: 'Popup created successfully!' });
			goto('/admin/popups');
		} catch (error: any) {
			console.error('Failed to create popup:', error);
			addToast({
				type: 'error',
				message: error.response?.data?.message || 'Failed to create popup'
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
	<title>Create Popup | Revolution Admin</title>
</svelte:head>

<div class="admin-popups-new">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Header -->
		<header class="page-header">
			<div class="header-row">
				<a href="/admin/popups" class="back-btn">Back to Popups</a>
				<button
					type="button"
					onclick={() => showPreview = !showPreview}
					class="preview-btn"
				>
					{showPreview ? 'Hide Preview' : 'Show Preview'}
				</button>
			</div>
			<h1>Create New Popup</h1>
			<p class="subtitle">Design and configure your popup</p>
		</header>
	</div>

	<!-- Template Selector -->
	<Card class="template-selector">
		<h2 class="text-xl font-semibold mb-4">Start from Template</h2>
		<p class="text-sm text-gray-600 mb-4">Choose a template to get started quickly, or start from scratch.</p>

		<div class="template-grid">
			<button
				type="button"
				onclick={() => applyTemplate('scratch')}
				class="template-card {!selectedTemplate ? 'selected' : ''}"
			>
				<div class="template-icon">+</div>
				<div class="template-name">From Scratch</div>
			</button>

			<button
				type="button"
				onclick={() => applyTemplate('newsletter')}
				class="template-card {selectedTemplate === 'newsletter' ? 'selected' : ''}"
			>
				<div class="template-icon">&#9993;</div>
				<div class="template-name">Newsletter</div>
			</button>

			<button
				type="button"
				onclick={() => applyTemplate('exit_intent')}
				class="template-card {selectedTemplate === 'exit_intent' ? 'selected' : ''}"
			>
				<div class="template-icon">&#128682;</div>
				<div class="template-name">Exit Intent</div>
			</button>

			<button
				type="button"
				onclick={() => applyTemplate('promotional')}
				class="template-card {selectedTemplate === 'promotional' ? 'selected' : ''}"
			>
				<div class="template-icon">&#127873;</div>
				<div class="template-name">Promotional</div>
			</button>

			<button
				type="button"
				onclick={() => applyTemplate('announcement')}
				class="template-card {selectedTemplate === 'announcement' ? 'selected' : ''}"
			>
				<div class="template-icon">&#128227;</div>
				<div class="template-name">Announcement</div>
			</button>

			<button
				type="button"
				onclick={() => applyTemplate('countdown')}
				class="template-card {selectedTemplate === 'countdown' ? 'selected' : ''}"
			>
				<div class="template-icon">&#9200;</div>
				<div class="template-name">Countdown</div>
			</button>

			<button
				type="button"
				onclick={() => applyTemplate('video')}
				class="template-card {selectedTemplate === 'video' ? 'selected' : ''}"
			>
				<div class="template-icon">&#9654;</div>
				<div class="template-name">Video</div>
			</button>

			<button
				type="button"
				onclick={() => applyTemplate('social_proof')}
				class="template-card {selectedTemplate === 'social_proof' ? 'selected' : ''}"
			>
				<div class="template-icon">&#11088;</div>
				<div class="template-name">Social Proof</div>
			</button>
		</div>
	</Card>

	<!-- Live Preview Panel -->
	{#if showPreview}
		<div class="preview-panel">
			<h3 class="preview-title">Live Preview</h3>
			<div
				class="preview-container"
				style="
					background-color: {formData.design?.overlayColor || '#000000'};
					opacity: {(formData.design?.overlayOpacity || 50) / 100};
				"
			>
				<div
					class="preview-popup"
					style="
						background-color: {formData.design?.backgroundColor || '#ffffff'};
						border-radius: 12px;
					"
				>
					{#if formData.title}
						<h4
							class="preview-popup-title"
							style="color: {formData.design?.titleColor || '#1f2937'}"
						>
							{formData.title}
						</h4>
					{/if}
					{#if formData.content}
						<div
							class="preview-popup-content"
							style="color: {formData.design?.textColor || '#4b5563'}"
						>
							{@html formData.content}
						</div>
					{/if}
					{#if formData.cta_text}
						<button
							type="button"
							class="preview-popup-cta"
							style="
								background-color: {formData.design?.buttonColor || '#3b82f6'};
								color: {formData.design?.buttonTextColor || '#ffffff'};
								border-radius: {formData.design?.buttonBorderRadius || 8}px;
							"
						>
							{formData.cta_text}
						</button>
					{/if}
				</div>
			</div>
		</div>
	{/if}

	<form onsubmit={handleSubmit}>
		<div class="space-y-6">
			<!-- Basic Information -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Basic Information</h2>

				<div class="space-y-4">
					<div>
						<Input
							id="popup-name"
							label="Internal Name *"
							bind:value={formData['name']}
							placeholder="e.g., Exit Intent - Newsletter"
							{...errors['name'] && { error: errors['name'] }}
						/>
						<p class="text-xs text-gray-500 mt-1">For your reference only, not shown to users</p>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div>
							<Select
								id="popup-type"
								label="Popup Type *"
								options={typeOptions}
								bind:value={formData.type}
							/>
						</div>

						<div>
							<Select
								id="popup-status"
								label="Status *"
								options={statusOptions}
								bind:value={formData.status}
							/>
						</div>
					</div>

					<div>
						<Input
							id="popup-priority"
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
							id="popup-title"
							label="Title *"
							bind:value={formData['title']}
							placeholder="e.g., Wait! Don't Leave Yet"
							{...errors['title'] && { error: errors['title'] }}
						/>
					</div>

					<div>
						<label for="popup-content" class="block text-sm font-medium text-gray-700 mb-1">
							Content *
						</label>
						<textarea
							id="popup-content"
							bind:value={formData['content']}
							class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 {errors[
								'content'
							]
								? ' border-red-500'
								: ''}"
							rows="4"
							placeholder="Enter your message (HTML allowed)"
						></textarea>
						{#if errors['content']}
							<p class="text-xs text-red-600 mt-1">{errors['content']}</p>
						{/if}
						<p class="text-xs text-gray-500 mt-1">HTML is supported for formatting</p>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div>
							<Input
								id="popup-cta-text"
								label="Call-to-Action Text"
								bind:value={formData.cta_text}
								placeholder="e.g., Get Started"
							/>
						</div>

						<div>
							<Input
								id="popup-cta-url"
								label="CTA URL"
								bind:value={formData.cta_url}
								placeholder="/subscribe"
							/>
						</div>
					</div>

					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							id="cta_new_tab" name="cta_new_tab"
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
					{:else if formData.type === 'inactivity'}
						<div>
							<Input
								id="popup-inactivity-timeout"
								label="Inactivity Timeout (seconds)"
								type="number"
								bind:value={inactivityTimeout}
								min="5"
								max="300"
							/>
							<p class="text-xs text-gray-500 mt-1">
								Show popup after {inactivityTimeout} seconds of user inactivity
							</p>
						</div>
					{:else if formData.type === 'exit_intent'}
						<div class="bg-blue-50 border border-blue-200 rounded-md p-4">
							<p class="text-sm text-blue-800">
								Exit intent popups trigger when the user's mouse moves toward the browser's top edge
								(leaving the page).
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

			<!-- Secondary CTA -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Secondary Action</h2>
				<p class="text-sm text-gray-600 mb-4">Add a secondary button for users who want to decline or take an alternative action.</p>

				<div class="space-y-4">
					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							id="has_secondary_cta" name="has_secondary_cta"
							bind:checked={hasSecondaryCta}
							class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
						/>
						<label for="has_secondary_cta" class="text-sm text-gray-700">
							Enable secondary CTA button
						</label>
					</div>

					{#if hasSecondaryCta}
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div>
								<Input
									id="secondary-cta-text"
									label="Button Text"
									bind:value={secondaryCtaText}
									placeholder="No thanks"
								/>
							</div>
							<div>
								<label for="secondary-cta-action" class="block text-sm font-medium text-gray-700 mb-1">
									Action
								</label>
								<select
									id="secondary-cta-action"
									bind:value={secondaryCtaAction}
									class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
								>
									<option value="close">Close Popup</option>
									<option value="url">Navigate to URL</option>
								</select>
							</div>
						</div>

						{#if secondaryCtaAction === 'url'}
							<div>
								<Input
									id="secondary-cta-url"
									label="Redirect URL"
									bind:value={secondaryCtaUrl}
									placeholder="/no-thanks"
								/>
							</div>
						{/if}
					{/if}
				</div>
			</Card>

			<!-- Video Embed -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Video Embed</h2>
				<p class="text-sm text-gray-600 mb-4">Add a video to your popup for higher engagement.</p>

				<div class="space-y-4">
					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							id="has_video" name="has_video"
							bind:checked={hasVideo}
							class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
						/>
						<label for="has_video" class="text-sm text-gray-700">
							Include video in popup
						</label>
					</div>

					{#if hasVideo}
						<div>
							<Input
								id="video-url"
								label="Video URL"
								bind:value={videoUrl}
								placeholder="https://www.youtube.com/watch?v=..."
							/>
							<p class="text-xs text-gray-500 mt-1">Supports YouTube, Vimeo, or direct video URLs</p>
						</div>

						<div class="flex items-center gap-2">
							<input
								type="checkbox"
								id="video_autoplay" name="video_autoplay"
								bind:checked={videoAutoplay}
								class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
							/>
							<label for="video_autoplay" class="text-sm text-gray-700">
								Autoplay video (muted)
							</label>
						</div>
					{/if}
				</div>
			</Card>

			<!-- Form Integration -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Form Integration</h2>
				<p class="text-sm text-gray-600 mb-4">Embed an existing form in your popup to collect leads or feedback.</p>

				<div class="space-y-4">
					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							id="has_form" name="has_form"
							bind:checked={hasForm}
							class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
						/>
						<label for="has_form" class="text-sm text-gray-700">
							Include form in popup
						</label>
					</div>

					{#if hasForm}
						<div>
							<label for="form-selector" class="block text-sm font-medium text-gray-700 mb-1">
								Select Form
							</label>
							{#if availableForms.length > 0}
								<select
									id="form-selector"
									bind:value={selectedFormId}
									class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
								>
									<option value="">-- Select a form --</option>
									{#each availableForms as form}
										<option value={form.id}>{form.name}</option>
									{/each}
								</select>
							{:else}
								<div class="bg-yellow-50 border border-yellow-200 rounded-md p-3">
									<p class="text-sm text-yellow-800">
										No forms available. <a href="/admin/forms" class="text-yellow-600 underline">Create a form</a> first.
									</p>
								</div>
							{/if}
						</div>

						{#if selectedFormId}
							<div class="bg-green-50 border border-green-200 rounded-md p-3">
								<p class="text-sm text-green-800">
									Form will be displayed inside the popup. Form submissions will be tracked as conversions.
								</p>
							</div>
						{/if}
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
								id="popup-position"
								label="Position"
								options={positionOptions}
								bind:value={formData.position}
							/>
						</div>

						<div>
							<Select
								id="popup-size"
								label="Size"
								options={sizeOptions}
								bind:value={formData.size}
							/>
						</div>

						<div>
							<Select
								id="popup-animation"
								label="Animation"
								options={animationOptions}
								bind:value={formData.animation}
							/>
						</div>
					</div>

					<div>
						<Input
							id="auto-close"
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
								id="show_close_button" name="show_close_button"
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
								id="close_on_overlay_click" name="close_on_overlay_click"
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
							id="show-frequency"
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
									id="device-desktop" name="device-desktop" type="checkbox"
									value="desktop"
									bind:group={formData.display_rules.devices}
									class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
								/>
								<span class="text-sm text-gray-700">Desktop</span>
							</label>
							<label class="flex items-center gap-2">
								<input
									id="device-tablet" name="device-tablet" type="checkbox"
									value="tablet"
									bind:group={formData.display_rules.devices}
									class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
								/>
								<span class="text-sm text-gray-700">Tablet</span>
							</label>
							<label class="flex items-center gap-2">
								<input
									id="device-mobile" name="device-mobile" type="checkbox"
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

			<!-- Page Targeting -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Page Targeting</h2>
				<p class="text-sm text-gray-600 mb-4">Control which pages the popup appears on. Use wildcards (*) for pattern matching.</p>

				<div class="space-y-4">
					<div>
						<label for="include-pages" class="block text-sm font-medium text-gray-700 mb-1">
							Include Pages
						</label>
						<textarea
							id="include-pages"
							bind:value={includePages}
							class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
							rows="3"
							placeholder="/pricing&#10;/products/*&#10;/checkout"
						></textarea>
						<p class="text-xs text-gray-500 mt-1">One URL pattern per line. Use * as wildcard. Leave empty to show on all pages.</p>
					</div>

					<div>
						<label for="exclude-pages" class="block text-sm font-medium text-gray-700 mb-1">
							Exclude Pages
						</label>
						<textarea
							id="exclude-pages"
							bind:value={excludePages}
							class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
							rows="3"
							placeholder="/admin/*&#10;/login&#10;/register"
						></textarea>
						<p class="text-xs text-gray-500 mt-1">Pages to exclude even if they match include patterns.</p>
					</div>
				</div>
			</Card>

			<!-- Scheduling -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Scheduling</h2>
				<p class="text-sm text-gray-600 mb-4">Set start and end dates to control when this popup is active. Leave empty for always-on.</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<div>
						<Input
							id="popup-start-date"
							label="Start Date"
							type="datetime-local"
							bind:value={(formData as any).start_date}
						/>
						<p class="text-xs text-gray-500 mt-1">When the popup should start showing</p>
					</div>

					<div>
						<Input
							id="popup-end-date"
							label="End Date"
							type="datetime-local"
							bind:value={(formData as any).end_date}
						/>
						<p class="text-xs text-gray-500 mt-1">When the popup should stop showing</p>
					</div>
				</div>
			</Card>

			<!-- Overlay Settings -->
			<Card>
				<h2 class="text-xl font-semibold mb-4">Overlay Settings</h2>
				<p class="text-sm text-gray-600 mb-4">Customize the backdrop overlay that appears behind the popup.</p>

				<div class="space-y-4">
					<div class="grid grid-cols-2 md:grid-cols-3 gap-4">
						<div>
							<Input
								id="overlay-color"
								label="Overlay Color"
								type="color"
								bind:value={formData.design.overlayColor}
							/>
						</div>

						<div>
							<Input
								id="overlay-opacity"
								label="Overlay Opacity (%)"
								type="number"
								bind:value={formData.design.overlayOpacity}
								min="0"
								max="100"
							/>
						</div>

						<div>
							<Input
								id="overlay-blur"
								label="Backdrop Blur (px)"
								type="number"
								bind:value={formData.design.overlayBlur}
								min="0"
								max="20"
							/>
						</div>
					</div>

					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							id="closeOnEscape" name="closeOnEscape"
							bind:checked={formData.closeOnEscape}
							class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
						/>
						<label for="closeOnEscape" class="text-sm text-gray-700">
							Close popup when pressing Escape key
						</label>
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
									id="bg-color"
									label="Background Color"
									type="color"
									bind:value={formData.design.backgroundColor}
								/>
							</div>

							<div>
								<Input
									id="title-color"
									label="Title Color"
									type="color"
									bind:value={formData.design.titleColor}
								/>
							</div>

							<div>
								<Input
									id="text-color"
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
									id="button-color"
									label="Button Color"
									type="color"
									bind:value={formData.design.buttonColor}
								/>
							</div>

							<div>
								<Input
									id="button-text-color"
									label="Button Text Color"
									type="color"
									bind:value={formData.design.buttonTextColor}
								/>
							</div>

							<div>
								<Input
									id="button-radius"
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
								<label for="button-shadow" class="block text-sm font-medium text-gray-700 mb-1">
									Button Shadow
								</label>
								<select
									id="button-shadow"
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
								<label for="button-padding" class="block text-sm font-medium text-gray-700 mb-1">
									Button Padding
								</label>
								<select
									id="button-padding"
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

			<!-- Actions -->
			<div class="form-actions">
				<button type="button" class="btn-secondary" onclick={handleCancel} disabled={loading}>
					Cancel
				</button>
				<button type="submit" class="btn-primary" disabled={loading}>
					{loading ? 'Creating...' : 'Create Popup'}
				</button>
			</div>
		</div>
	</form>
</div>

<style>
	.admin-popups-new {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.header-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0.75rem 0 0.25rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.4rem 0.75rem;
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border-radius: 6px;
		text-decoration: none;
		font-size: 0.875rem;
		transition: all 0.2s;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.back-btn:hover {
		background: rgba(100, 116, 139, 0.3);
	}

	.preview-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(99, 102, 241, 0.2);
		color: #a5b4fc;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		border: 1px solid rgba(99, 102, 241, 0.3);
		cursor: pointer;
		transition: all 0.2s;
	}

	.preview-btn:hover {
		background: rgba(99, 102, 241, 0.3);
	}

	/* Template Selector */
	:global(.template-selector) {
		margin-bottom: 1.5rem !important;
	}

	.template-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
		gap: 1rem;
	}

	.template-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.template-card:hover {
		border-color: rgba(99, 102, 241, 0.5);
		background: rgba(15, 23, 42, 0.8);
	}

	.template-card.selected {
		border-color: #6366f1;
		background: rgba(99, 102, 241, 0.1);
	}

	.template-icon {
		font-size: 2rem;
		margin-bottom: 0.5rem;
	}

	.template-name {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #e2e8f0;
	}

	/* Live Preview Panel */
	.preview-panel {
		margin-bottom: 1.5rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
	}

	.preview-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.preview-container {
		position: relative;
		min-height: 300px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
	}

	.preview-popup {
		max-width: 400px;
		width: 100%;
		padding: 1.5rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	.preview-popup-title {
		font-size: 1.25rem;
		font-weight: 700;
		margin: 0 0 0.75rem 0;
	}

	.preview-popup-content {
		font-size: 0.9375rem;
		line-height: 1.5;
		margin-bottom: 1rem;
	}

	.preview-popup-content :global(p) {
		margin: 0 0 0.5rem 0;
	}

	.preview-popup-cta {
		display: block;
		width: 100%;
		padding: 0.75rem 1.5rem;
		font-weight: 600;
		font-size: 1rem;
		border: none;
		cursor: pointer;
		text-align: center;
	}

	.form-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		margin-top: 2rem;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		font-weight: 600;
		border-radius: 6px;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		opacity: 0.9;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		font-weight: 500;
		border-radius: 6px;
		border: 1px solid rgba(100, 116, 139, 0.3);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		opacity: 0.9;
	}

	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Override Card component styles for dark theme */
	:global(.page .space-y-6 > div) {
		background: rgba(30, 41, 59, 0.4) !important;
		border: 1px solid rgba(148, 163, 184, 0.1) !important;
		border-radius: 8px !important;
	}

	/* Form labels and text */
	:global(.page label),
	:global(.page .text-sm.font-medium) {
		color: #e2e8f0 !important;
	}

	:global(.page .text-gray-700),
	:global(.page .text-gray-600),
	:global(.page .text-gray-500) {
		color: #94a3b8 !important;
	}

	:global(.page .text-gray-900),
	:global(.page .text-gray-800) {
		color: #f1f5f9 !important;
	}

	:global(.page h2) {
		color: #f1f5f9 !important;
	}

	:global(.page h3) {
		color: #e2e8f0 !important;
	}

	/* Form inputs */
	:global(.page input[type='text']),
	:global(.page input[type='number']),
	:global(.page input[type='color']),
	:global(.page textarea),
	:global(.page select) {
		background: rgba(15, 23, 42, 0.6) !important;
		border: 1px solid rgba(148, 163, 184, 0.2) !important;
		border-radius: 6px !important;
		color: #f1f5f9 !important;
	}

	:global(.page input:focus),
	:global(.page textarea:focus),
	:global(.page select:focus) {
		border-color: rgba(99, 102, 241, 0.5) !important;
		outline: none !important;
	}

	/* Checkboxes */
	:global(.page input[type='checkbox']) {
		accent-color: var(--primary-500);
	}

	/* Info boxes */
	:global(.page .bg-blue-50) {
		background: rgba(99, 102, 241, 0.1) !important;
		border-color: rgba(99, 102, 241, 0.3) !important;
	}

	:global(.page .text-blue-800) {
		color: #a5b4fc !important;
	}

	/* Error states */
	:global(.page .text-red-600) {
		color: #f87171 !important;
	}

	:global(.admin-popups-new .border-red-500) {
		border-color: rgba(239, 68, 68, 0.5) !important;
	}

	@media (max-width: 768px) {
		.admin-popups-new {
			padding: 1rem;
		}

		.form-actions {
			flex-direction: column;
		}
	}
</style>
