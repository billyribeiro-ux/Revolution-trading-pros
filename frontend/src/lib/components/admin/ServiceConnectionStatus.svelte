<script lang="ts">
	/**
	 * Service Connection Status - Apple Principal Engineer ICT11+ Grade
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Premium connection status component that displays real-time service connectivity
	 * with Apple-level polish, beautiful animations, and seamless UX.
	 *
	 * Features:
	 * - Real-time connection status from store
	 * - Multiple display variants (card, inline, badge, banner)
	 * - One-click navigation to connections page
	 * - Feature-based or service-specific status
	 * - Apple-grade animations and transitions
	 *
	 * @level ICT11+ Principal Engineer - Premium Enterprise UX
	 */

	import { goto } from '$app/navigation';
	import { fade, fly, scale, slide } from 'svelte/transition';
	import { backOut, cubicOut } from 'svelte/easing';
	import {
		connections,
		isAnalyticsConnected,
		isSeoConnected,
		isEmailConnected,
		isPaymentConnected,
		isCrmConnected,
		isFormsConnected,
		isBehaviorConnected,
		FEATURE_SERVICES,
		type ServiceKey
	} from '$lib/stores/connections.svelte';

	// Feature configuration - maps features to display info
	const FEATURE_CONFIG: Record<
		string,
		{
			name: string;
			description: string;
			icon: string;
			color: string;
			features: string[];
			primaryService?: ServiceKey;
		}
	> = {
		analytics: {
			name: 'Analytics',
			description: 'Track visitors, sessions, and user behavior across your site',
			icon: '📊',
			color: '#F9AB00',
			features: [
				'Real-time visitor tracking',
				'Session analytics',
				'User behavior insights',
				'Conversion tracking'
			],
			primaryService: 'google_analytics'
		},
		seo: {
			name: 'SEO Tools',
			description: 'Monitor search rankings, indexing status, and SEO performance',
			icon: '🔍',
			color: '#4285F4',
			features: [
				'Search ranking monitoring',
				'Indexing status',
				'Keyword performance',
				'Backlink analysis'
			],
			primaryService: 'google_search_console'
		},
		email: {
			name: 'Email Services',
			description: 'Send transactional and marketing emails to your users',
			icon: '✉️',
			color: '#1A82E2',
			features: [
				'Transactional emails',
				'Marketing campaigns',
				'Email analytics',
				'Delivery tracking'
			],
			primaryService: 'sendgrid'
		},
		payment: {
			name: 'Payment Processing',
			description: 'Accept payments and manage subscriptions seamlessly',
			icon: '💳',
			color: '#635BFF',
			features: [
				'Credit card payments',
				'Subscription billing',
				'Invoice management',
				'Revenue analytics'
			],
			primaryService: 'stripe'
		},
		crm: {
			name: 'CRM & Contacts',
			description: 'Manage customer relationships, contacts, and sales pipelines',
			icon: '👥',
			color: '#FF7A59',
			features: [
				'Contact management',
				'Sales pipeline',
				'Customer insights',
				'Automation workflows'
			],
			primaryService: 'fluent_crm_pro'
		},
		forms: {
			name: 'Form Builder',
			description: 'Create and manage forms with advanced features',
			icon: '📝',
			color: '#10B981',
			features: [
				'Drag-drop builder',
				'Conditional logic',
				'Payment forms',
				'Submissions management'
			],
			primaryService: 'fluent_forms_pro'
		},
		behavior: {
			name: 'Behavior Tracking',
			description: 'Track user interactions, clicks, and engagement patterns',
			icon: '🎯',
			color: '#8B5CF6',
			features: ['Click tracking', 'Scroll depth', 'Event tracking', 'Heatmaps'],
			primaryService: 'google_analytics'
		},
		social: {
			name: 'Social Media',
			description: 'Connect your social media accounts for tracking and sharing',
			icon: '🔗',
			color: '#1877F2',
			features: ['Social tracking', 'Share buttons', 'Pixel integration', 'Audience sync'],
			primaryService: 'facebook'
		},
		ai: {
			name: 'AI Services',
			description: 'Enable AI-powered features for content and automation',
			icon: '🤖',
			color: '#10A37F',
			features: ['Content generation', 'Smart suggestions', 'Automation', 'Analysis'],
			primaryService: 'openai'
		},
		monitoring: {
			name: 'Error Monitoring',
			description: 'Track errors, performance issues, and application health',
			icon: '🔔',
			color: '#362D59',
			features: ['Error tracking', 'Performance monitoring', 'Alerts', 'Debug tools'],
			primaryService: 'sentry'
		}
	};

	type FeatureKey = keyof typeof FEATURE_CONFIG;

	interface Props {
		/** Feature category to check (analytics, seo, payment, etc.) */
		feature?: FeatureKey;
		/** Specific service key to check (google_analytics, stripe, etc.) */
		serviceKey?: ServiceKey;
		/** Custom service name to display */
		serviceName?: string;
		/** Custom description */
		description?: string;
		/** Display variant */
		variant?: 'card' | 'inline' | 'badge' | 'banner' | 'minimal';
		/** Show feature list */
		showFeatures?: boolean;
		/** Custom color */
		color?: string;
		/** Custom icon */
		icon?: string;
		/** Callback when connected */
		onConnected?: () => void;
	}

	let props: Props = $props();

	// Destructure with defaults for internal use
	const feature = $derived(props.feature);
	const serviceKey = $derived(props.serviceKey);
	const serviceName = $derived(props.serviceName);
	const description = $derived(props.description);
	const variant = $derived(props.variant ?? 'card');
	const showFeatures = $derived(props.showFeatures ?? true);
	const color = $derived(props.color);
	const icon = $derived(props.icon);
	const onConnected = $derived(props.onConnected);

	// Derive configuration from feature or custom props
	let config = $derived.by(() => {
		if (feature && FEATURE_CONFIG[feature]) {
			return FEATURE_CONFIG[feature];
		}
		return {
			name: serviceName || 'Service',
			description: description || 'Connect this service to enable features',
			icon: icon || '🔌',
			color: color || '#E6B800',
			features: [],
			primaryService: serviceKey
		};
	});

	// Check connection status
	let isConnected = $derived.by(() => {
		if (serviceKey) {
			return connections.isConnected(serviceKey);
		}
		if (feature) {
			switch (feature) {
				case 'analytics':
					return $isAnalyticsConnected;
				case 'seo':
					return $isSeoConnected;
				case 'email':
					return $isEmailConnected;
				case 'payment':
					return $isPaymentConnected;
				case 'crm':
					return $isCrmConnected;
				case 'forms':
					return $isFormsConnected;
				case 'behavior':
					return $isBehaviorConnected;
				default:
					return connections.isFeatureConnected(feature as keyof typeof FEATURE_SERVICES);
			}
		}
		return false;
	});

	// Navigate to connections page with pre-selected service
	function handleConnect() {
		const targetService = serviceKey || config.primaryService;
		if (targetService) {
			goto(`/admin/connections?connect=${targetService}`);
		} else if (feature) {
			goto(`/admin/connections?category=${feature}`);
		} else {
			goto('/admin/connections');
		}
	}

	// Notify parent when connected
	$effect(() => {
		if (isConnected && onConnected) {
			onConnected();
		}
	});
</script>

<!-- Only render if NOT connected -->
{#if !isConnected}
	{#if variant === 'card'}
		<!-- Full Card Variant - Premium Design -->
		<div
			class="service-status-card"
			in:scale={{ duration: 400, start: 0.95, easing: backOut }}
			role="alert"
			aria-live="polite"
		>
			<!-- Gradient Background Glow -->
			<div class="card-glow" style="--glow-color: {config.color}"></div>

			<!-- Decorative Elements -->
			<div class="decorative-orb orb-1" style="--orb-color: {config.color}"></div>
			<div class="decorative-orb orb-2" style="--orb-color: {config.color}"></div>

			<!-- Content -->
			<div class="card-content">
				<!-- Icon & Status -->
				<div class="icon-section">
					<div class="icon-container" style="--icon-color: {config.color}">
						<span class="service-icon">{config.icon}</span>
						<div class="status-indicator">
							<svg
								class="disconnect-icon"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path d="M18.36 6.64a9 9 0 1 1-12.73 0" />
								<line x1="12" y1="2" x2="12" y2="12" />
							</svg>
						</div>
					</div>
					<div class="status-badge">
						<span class="status-dot"></span>
						<span>Disconnected</span>
					</div>
				</div>

				<!-- Text Content -->
				<div class="text-section">
					<h3 class="title">Connect {config.name}</h3>
					<p class="description">{config.description}</p>
				</div>

				<!-- Features List -->
				{#if showFeatures && config.features.length > 0}
					<div class="features-section" in:slide={{ duration: 300, delay: 100 }}>
						<p class="features-label">This connection enables:</p>
						<ul class="features-list">
							{#each config.features as featureItem, i}
								<li in:fly={{ x: -10, duration: 200, delay: 150 + i * 50 }}>
									<span class="check-icon" style="color: {config.color}">✓</span>
									{featureItem}
								</li>
							{/each}
						</ul>
					</div>
				{/if}

				<!-- Connect Button -->
				<button class="connect-button" onclick={handleConnect} style="--btn-color: {config.color}">
					<svg
						class="btn-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z" />
						<path d="M12 6v6l4 2" />
					</svg>
					<span>Connect {config.name}</span>
					<svg
						class="arrow-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M5 12h14M12 5l7 7-7 7" />
					</svg>
				</button>

				<!-- Security Note -->
				<div class="security-note">
					<svg
						class="lock-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
						<path d="M7 11V7a5 5 0 0 1 10 0v4" />
					</svg>
					<span>Your credentials are encrypted and stored securely</span>
				</div>
			</div>
		</div>
	{:else if variant === 'banner'}
		<!-- Banner Variant - For page headers -->
		<div
			class="service-status-banner"
			in:fly={{ y: -20, duration: 400, easing: cubicOut }}
			role="alert"
		>
			<div class="banner-glow" style="--glow-color: {config.color}"></div>
			<div class="banner-content">
				<div class="banner-icon" style="--icon-color: {config.color}">
					<span>{config.icon}</span>
				</div>
				<div class="banner-text">
					<h4>{config.name} Not Connected</h4>
					<p>{config.description}</p>
				</div>
				<button class="banner-button" onclick={handleConnect} style="--btn-color: {config.color}">
					<span>Connect Now</span>
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M5 12h14M12 5l7 7-7 7" />
					</svg>
				</button>
			</div>
		</div>
	{:else if variant === 'inline'}
		<!-- Inline Variant - For within content -->
		<div class="service-status-inline" in:fade={{ duration: 300 }} role="alert">
			<div class="inline-icon" style="color: {config.color}">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<circle cx="12" cy="12" r="10" />
					<line x1="12" y1="8" x2="12" y2="12" />
					<line x1="12" y1="16" x2="12.01" y2="16" />
				</svg>
			</div>
			<span class="inline-message">{config.name} not connected</span>
			<button class="inline-button" onclick={handleConnect}>
				Connect
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M5 12h14M12 5l7 7-7 7" />
				</svg>
			</button>
		</div>
	{:else if variant === 'badge'}
		<!-- Badge Variant - Minimal indicator -->
		<button
			class="service-status-badge"
			onclick={handleConnect}
			in:scale={{ duration: 200, start: 0.9 }}
			title="Click to connect {config.name}"
		>
			<span class="badge-dot"></span>
			<span class="badge-text">Disconnected</span>
		</button>
	{:else if variant === 'minimal'}
		<!-- Minimal Variant - Just a button -->
		<button
			class="service-status-minimal"
			onclick={handleConnect}
			in:fade={{ duration: 200 }}
			style="--btn-color: {config.color}"
		>
			<svg class="plug-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M18.36 6.64a9 9 0 1 1-12.73 0" />
				<line x1="12" y1="2" x2="12" y2="12" />
			</svg>
			<span>Connect {config.name}</span>
		</button>
	{/if}
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	   Card Variant Styles
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.service-status-card {
		position: relative;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.95), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 24px;
		padding: 2.5rem;
		overflow: hidden;
		backdrop-filter: blur(20px);
	}

	.card-glow {
		position: absolute;
		top: -100px;
		left: 50%;
		transform: translateX(-50%);
		width: 400px;
		height: 400px;
		background: radial-gradient(circle, var(--glow-color, #e6b800) 0%, transparent 70%);
		opacity: 0.08;
		pointer-events: none;
	}

	.decorative-orb {
		position: absolute;
		border-radius: 50%;
		background: linear-gradient(135deg, var(--orb-color, #e6b800), transparent);
		opacity: 0.03;
		pointer-events: none;
	}

	.orb-1 {
		width: 200px;
		height: 200px;
		bottom: -60px;
		right: -60px;
	}

	.orb-2 {
		width: 120px;
		height: 120px;
		top: 20%;
		left: -40px;
	}

	.card-content {
		position: relative;
		z-index: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}

	.icon-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.icon-container {
		position: relative;
		width: 80px;
		height: 80px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(255, 255, 255, 0.05), rgba(255, 255, 255, 0.02));
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 24px;
	}

	.service-icon {
		font-size: 2.5rem;
		filter: grayscale(50%);
		opacity: 0.7;
	}

	.status-indicator {
		position: absolute;
		bottom: -8px;
		right: -8px;
		width: 32px;
		height: 32px;
		background: linear-gradient(135deg, #475569, #334155);
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}

	.disconnect-icon {
		width: 18px;
		height: 18px;
		color: #94a3b8;
	}

	.status-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(248, 113, 113, 0.1);
		border: 1px solid rgba(248, 113, 113, 0.2);
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #fca5a5;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.status-dot {
		width: 6px;
		height: 6px;
		background: #f87171;
		border-radius: 50%;
	}

	.text-section {
		margin-bottom: 1.5rem;
	}

	.title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.75rem 0;
	}

	.description {
		font-size: 1rem;
		color: #94a3b8;
		margin: 0;
		max-width: 400px;
		line-height: 1.6;
	}

	.features-section {
		width: 100%;
		max-width: 350px;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 16px;
		padding: 1.25rem;
		margin-bottom: 1.5rem;
		text-align: left;
	}

	.features-label {
		font-size: 0.7rem;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: #64748b;
		margin: 0 0 0.75rem 0;
	}

	.features-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.features-list li {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		color: #cbd5e1;
		font-size: 0.875rem;
		padding: 0.375rem 0;
	}

	.check-icon {
		font-weight: 700;
		font-size: 0.75rem;
	}

	.connect-button {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 2rem;
		background: linear-gradient(
			135deg,
			var(--btn-color, #e6b800),
			color-mix(in srgb, var(--btn-color, #e6b800), #b38f00 40%)
		);
		color: #0d1117;
		border: none;
		border-radius: 16px;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		box-shadow: 0 10px 30px color-mix(in srgb, var(--btn-color, #e6b800) 30%, transparent);
	}

	.connect-button:hover {
		transform: translateY(-3px);
		box-shadow: 0 15px 40px color-mix(in srgb, var(--btn-color, #e6b800) 40%, transparent);
	}

	.connect-button:active {
		transform: translateY(-1px);
	}

	.btn-icon,
	.arrow-icon {
		width: 20px;
		height: 20px;
	}

	.security-note {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 1.5rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.lock-icon {
		width: 14px;
		height: 14px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Banner Variant Styles
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.service-status-banner {
		position: relative;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.95), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(245, 158, 11, 0.2);
		border-radius: 16px;
		padding: 1rem 1.5rem;
		overflow: hidden;
	}

	.banner-glow {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 100%;
		background: linear-gradient(
			90deg,
			color-mix(in srgb, var(--glow-color, #f59e0b) 10%, transparent),
			transparent
		);
		pointer-events: none;
	}

	.banner-content {
		position: relative;
		z-index: 1;
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.banner-icon {
		flex-shrink: 0;
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: color-mix(in srgb, var(--icon-color, #f59e0b) 15%, transparent);
		border-radius: 12px;
		font-size: 1.25rem;
	}

	.banner-text {
		flex: 1;
		min-width: 0;
	}

	.banner-text h4 {
		margin: 0;
		font-size: 0.9375rem;
		font-weight: 600;
		color: #fbbf24;
	}

	.banner-text p {
		margin: 0.25rem 0 0 0;
		font-size: 0.8125rem;
		color: #94a3b8;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.banner-button {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: color-mix(in srgb, var(--btn-color, #f59e0b) 20%, transparent);
		border: 1px solid color-mix(in srgb, var(--btn-color, #f59e0b) 30%, transparent);
		border-radius: 10px;
		color: #fbbf24;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.banner-button:hover {
		background: color-mix(in srgb, var(--btn-color, #f59e0b) 30%, transparent);
		border-color: color-mix(in srgb, var(--btn-color, #f59e0b) 40%, transparent);
	}

	.banner-button svg {
		width: 16px;
		height: 16px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Inline Variant Styles
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.service-status-inline {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		background: linear-gradient(135deg, rgba(245, 158, 11, 0.08), rgba(245, 158, 11, 0.04));
		border: 1px solid rgba(245, 158, 11, 0.15);
		border-radius: 12px;
	}

	.inline-icon {
		flex-shrink: 0;
	}

	.inline-icon svg {
		width: 20px;
		height: 20px;
	}

	.inline-message {
		flex: 1;
		font-size: 0.875rem;
		color: #fbbf24;
		font-weight: 500;
	}

	.inline-button {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: rgba(245, 158, 11, 0.15);
		border: 1px solid rgba(245, 158, 11, 0.25);
		border-radius: 8px;
		color: #fbbf24;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.inline-button:hover {
		background: rgba(245, 158, 11, 0.25);
		border-color: rgba(245, 158, 11, 0.35);
	}

	.inline-button svg {
		width: 14px;
		height: 14px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Badge Variant Styles
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.service-status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: rgba(100, 116, 139, 0.15);
		border: 1px solid rgba(100, 116, 139, 0.2);
		border-radius: 20px;
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.service-status-badge:hover {
		background: rgba(245, 158, 11, 0.15);
		border-color: rgba(245, 158, 11, 0.25);
		color: #fbbf24;
	}

	.badge-dot {
		width: 6px;
		height: 6px;
		background: #64748b;
		border-radius: 50%;
		transition: background 0.2s;
	}

	.service-status-badge:hover .badge-dot {
		background: #f59e0b;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Minimal Variant Styles
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.service-status-minimal {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: transparent;
		border: 1px dashed rgba(148, 163, 184, 0.3);
		border-radius: 10px;
		color: #94a3b8;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.service-status-minimal:hover {
		background: color-mix(in srgb, var(--btn-color, #e6b800) 10%, transparent);
		border-color: color-mix(in srgb, var(--btn-color, #e6b800) 40%, transparent);
		border-style: solid;
		color: var(--btn-color, #e6b800);
	}

	.plug-icon {
		width: 16px;
		height: 16px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Responsive Adjustments
	   ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 640px) {
		.service-status-card {
			padding: 1.5rem;
		}

		.title {
			font-size: 1.25rem;
		}

		.connect-button {
			width: 100%;
			justify-content: center;
		}

		.banner-content {
			flex-wrap: wrap;
		}

		.banner-text {
			flex: 1 1 100%;
			order: 1;
		}

		.banner-icon {
			order: 0;
		}

		.banner-button {
			order: 2;
			width: 100%;
			justify-content: center;
			margin-top: 0.75rem;
		}
	}
</style>
