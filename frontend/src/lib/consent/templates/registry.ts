/**
 * Banner Template Registry
 *
 * Contains 20 production-ready banner templates covering various styles.
 *
 * @module consent/templates/registry
 * @version 1.0.0
 */

import type { BannerTemplate } from './types';

/**
 * Default typography settings
 */
const defaultTypography = {
	fontFamily: 'system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
	titleSize: '1.125rem',
	titleWeight: '600',
	descriptionSize: '0.875rem',
	buttonSize: '0.875rem',
	buttonWeight: '600',
	lineHeight: '1.5',
};

/**
 * Default spacing settings
 */
const defaultSpacing = {
	padding: '1.5rem',
	gap: '1rem',
	buttonPadding: '0.75rem 1.5rem',
	borderRadius: '12px',
	buttonBorderRadius: '8px',
};

/**
 * Default mobile configuration
 */
const defaultMobile = {
	position: 'bottom' as const,
	useDrawer: false,
	stackButtons: true,
	compact: false,
	fullWidthButtons: true,
	padding: '1rem',
};

/**
 * Default tablet configuration
 */
const defaultTablet = {
	position: 'bottom' as const,
	maxWidth: '600px',
	stackButtons: false,
};

/**
 * Default copy
 */
const defaultCopy = {
	title: 'We value your privacy',
	description: 'We use cookies to enhance your browsing experience, serve personalized content, and analyze our traffic. By clicking "Accept All", you consent to our use of cookies.',
	acceptAll: 'Accept All',
	rejectAll: 'Reject All',
	customize: 'Customize',
	privacyPolicy: 'Privacy Policy',
	cookiePolicy: 'Cookie Policy',
};

// =============================================================================
// TEMPLATES
// =============================================================================

/**
 * Template 1: Minimal Dark
 */
const minimalDark: BannerTemplate = {
	id: 'minimal-dark',
	name: 'Minimal Dark',
	description: 'Clean, minimal design with dark background. Perfect for modern websites.',
	category: 'minimal',
	isEditable: true,
	position: 'bottom',
	style: 'minimal',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: false,
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 300,
	colors: {
		background: '#0a0a0a',
		text: '#ffffff',
		textMuted: '#a1a1aa',
		accent: '#3b82f6',
		acceptButton: '#3b82f6',
		acceptButtonText: '#ffffff',
		rejectButton: 'transparent',
		rejectButtonText: '#a1a1aa',
		customizeButton: 'transparent',
		customizeButtonText: '#ffffff',
		border: '#27272a',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing, borderRadius: '0' },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 -4px 20px rgba(0, 0, 0, 0.3)',
};

/**
 * Template 2: Minimal Light
 */
const minimalLight: BannerTemplate = {
	id: 'minimal-light',
	name: 'Minimal Light',
	description: 'Clean, minimal design with light background. Great for corporate sites.',
	category: 'minimal',
	isEditable: true,
	position: 'bottom',
	style: 'minimal',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: false,
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 300,
	colors: {
		background: '#ffffff',
		text: '#18181b',
		textMuted: '#71717a',
		accent: '#2563eb',
		acceptButton: '#2563eb',
		acceptButtonText: '#ffffff',
		rejectButton: 'transparent',
		rejectButtonText: '#71717a',
		customizeButton: 'transparent',
		customizeButtonText: '#18181b',
		border: '#e4e4e7',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing, borderRadius: '0' },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 -4px 20px rgba(0, 0, 0, 0.1)',
};

/**
 * Template 3: Glassmorphism
 */
const glassmorphism: BannerTemplate = {
	id: 'glassmorphism',
	name: 'Glassmorphism',
	description: 'Trendy frosted glass effect with blur and transparency.',
	category: 'modern',
	isEditable: true,
	position: 'bottom',
	style: 'glassmorphism',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'cookie',
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 400,
	colors: {
		background: 'rgba(255, 255, 255, 0.1)',
		backdrop: 'rgba(0, 0, 0, 0.3)',
		text: '#ffffff',
		textMuted: 'rgba(255, 255, 255, 0.7)',
		accent: '#8b5cf6',
		acceptButton: 'rgba(139, 92, 246, 0.9)',
		acceptButtonText: '#ffffff',
		rejectButton: 'rgba(255, 255, 255, 0.1)',
		rejectButtonText: '#ffffff',
		customizeButton: 'transparent',
		customizeButtonText: '#ffffff',
		border: 'rgba(255, 255, 255, 0.2)',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing, borderRadius: '16px' },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	backdropBlur: '20px',
	boxShadow: '0 8px 32px rgba(0, 0, 0, 0.3)',
	border: '1px solid rgba(255, 255, 255, 0.2)',
};

/**
 * Template 4: Gradient Purple
 */
const gradientPurple: BannerTemplate = {
	id: 'gradient-purple',
	name: 'Gradient Purple',
	description: 'Vibrant purple gradient with modern appeal.',
	category: 'modern',
	isEditable: true,
	position: 'bottom',
	style: 'gradient',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'shield',
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 350,
	colors: {
		background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
		text: '#ffffff',
		textMuted: 'rgba(255, 255, 255, 0.8)',
		accent: '#ffffff',
		acceptButton: '#ffffff',
		acceptButtonText: '#764ba2',
		rejectButton: 'rgba(255, 255, 255, 0.2)',
		rejectButtonText: '#ffffff',
		customizeButton: 'transparent',
		customizeButtonText: '#ffffff',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 -4px 30px rgba(102, 126, 234, 0.4)',
};

/**
 * Template 5: Gradient Ocean
 */
const gradientOcean: BannerTemplate = {
	id: 'gradient-ocean',
	name: 'Gradient Ocean',
	description: 'Calming blue-teal gradient inspired by the ocean.',
	category: 'modern',
	isEditable: true,
	position: 'bottom',
	style: 'gradient',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'lock',
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 350,
	colors: {
		background: 'linear-gradient(135deg, #0ea5e9 0%, #06b6d4 50%, #14b8a6 100%)',
		text: '#ffffff',
		textMuted: 'rgba(255, 255, 255, 0.85)',
		accent: '#ffffff',
		acceptButton: '#ffffff',
		acceptButtonText: '#0891b2',
		rejectButton: 'rgba(255, 255, 255, 0.2)',
		rejectButtonText: '#ffffff',
		customizeButton: 'transparent',
		customizeButtonText: '#ffffff',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 -4px 30px rgba(14, 165, 233, 0.4)',
};

/**
 * Template 6: Floating Card
 */
const floatingCard: BannerTemplate = {
	id: 'floating-card',
	name: 'Floating Card',
	description: 'Elegant floating card in the corner. Less intrusive.',
	category: 'modern',
	isEditable: true,
	position: 'bottom-right',
	style: 'card',
	maxWidth: '420px',
	showCloseButton: true,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'cookie',
	buttonVariant: 'rounded',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['reject', 'customize', 'accept'],
	animation: 'scale',
	animationDuration: 300,
	colors: {
		background: '#ffffff',
		text: '#1f2937',
		textMuted: '#6b7280',
		accent: '#4f46e5',
		acceptButton: '#4f46e5',
		acceptButtonText: '#ffffff',
		rejectButton: '#f3f4f6',
		rejectButtonText: '#6b7280',
		customizeButton: '#f3f4f6',
		customizeButtonText: '#374151',
		border: '#e5e7eb',
	},
	typography: { ...defaultTypography, titleSize: '1rem', descriptionSize: '0.8125rem' },
	spacing: { ...defaultSpacing, padding: '1.25rem', borderRadius: '16px' },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile, position: 'bottom', useDrawer: true },
	tablet: { ...defaultTablet, position: 'bottom-right', maxWidth: '400px' },
	boxShadow: '0 10px 40px rgba(0, 0, 0, 0.15)',
};

/**
 * Template 7: Floating Dark Card
 */
const floatingDarkCard: BannerTemplate = {
	id: 'floating-dark-card',
	name: 'Floating Dark Card',
	description: 'Dark themed floating card. Modern and sophisticated.',
	category: 'dark',
	isEditable: true,
	position: 'bottom-left',
	style: 'card',
	maxWidth: '420px',
	showCloseButton: true,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'shield',
	buttonVariant: 'rounded',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['reject', 'customize', 'accept'],
	animation: 'scale',
	animationDuration: 300,
	colors: {
		background: '#18181b',
		text: '#fafafa',
		textMuted: '#a1a1aa',
		accent: '#22c55e',
		acceptButton: '#22c55e',
		acceptButtonText: '#ffffff',
		rejectButton: '#27272a',
		rejectButtonText: '#a1a1aa',
		customizeButton: '#27272a',
		customizeButtonText: '#fafafa',
		border: '#3f3f46',
	},
	typography: { ...defaultTypography, titleSize: '1rem', descriptionSize: '0.8125rem' },
	spacing: { ...defaultSpacing, padding: '1.25rem', borderRadius: '16px' },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile, position: 'bottom', useDrawer: true },
	tablet: { ...defaultTablet, position: 'bottom-left', maxWidth: '400px' },
	boxShadow: '0 10px 40px rgba(0, 0, 0, 0.5)',
};

/**
 * Template 8: Corporate Blue
 */
const corporateBlue: BannerTemplate = {
	id: 'corporate-blue',
	name: 'Corporate Blue',
	description: 'Professional corporate design with trust-building blue tones.',
	category: 'corporate',
	isEditable: true,
	position: 'bottom',
	style: 'flat',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'lock',
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 300,
	colors: {
		background: '#1e3a5f',
		text: '#ffffff',
		textMuted: '#94a3b8',
		accent: '#38bdf8',
		acceptButton: '#38bdf8',
		acceptButtonText: '#0c4a6e',
		rejectButton: 'transparent',
		rejectButtonText: '#94a3b8',
		customizeButton: 'transparent',
		customizeButtonText: '#ffffff',
		border: '#334155',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing },
	copy: {
		...defaultCopy,
		title: 'Cookie Notice',
		description: 'We use cookies and similar technologies to provide our services, enhance your experience, and understand how our services are used. By continuing to use this website, you consent to our use of these technologies.',
	},
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 -4px 20px rgba(30, 58, 95, 0.4)',
};

/**
 * Template 9: Enterprise Gray
 */
const enterpriseGray: BannerTemplate = {
	id: 'enterprise-gray',
	name: 'Enterprise Gray',
	description: 'Neutral enterprise design suitable for B2B applications.',
	category: 'corporate',
	isEditable: true,
	position: 'bottom',
	style: 'flat',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: false,
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['reject', 'customize', 'accept'],
	animation: 'slide-up',
	animationDuration: 250,
	colors: {
		background: '#f8fafc',
		text: '#0f172a',
		textMuted: '#64748b',
		accent: '#0f172a',
		acceptButton: '#0f172a',
		acceptButtonText: '#ffffff',
		rejectButton: '#e2e8f0',
		rejectButtonText: '#475569',
		customizeButton: 'transparent',
		customizeButtonText: '#0f172a',
		border: '#e2e8f0',
	},
	typography: { ...defaultTypography, fontFamily: '"Inter", system-ui, sans-serif' },
	spacing: { ...defaultSpacing, borderRadius: '0', buttonBorderRadius: '4px' },
	copy: {
		...defaultCopy,
		title: 'Cookie Preferences',
		description: 'This website uses cookies to ensure you get the best experience. You can manage your preferences at any time.',
		acceptAll: 'Accept',
		rejectAll: 'Decline',
		customize: 'Manage Preferences',
	},
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 -1px 3px rgba(0, 0, 0, 0.1)',
	border: '1px solid #e2e8f0',
};

/**
 * Template 10: Playful Rounded
 */
const playfulRounded: BannerTemplate = {
	id: 'playful-rounded',
	name: 'Playful Rounded',
	description: 'Fun, friendly design with rounded elements and warm colors.',
	category: 'playful',
	isEditable: true,
	position: 'bottom',
	style: 'floating',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'cookie',
	buttonVariant: 'pill',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'bounce',
	animationDuration: 500,
	colors: {
		background: '#fef3c7',
		text: '#78350f',
		textMuted: '#92400e',
		accent: '#f59e0b',
		acceptButton: '#f59e0b',
		acceptButtonText: '#ffffff',
		rejectButton: '#fef9c3',
		rejectButtonText: '#92400e',
		customizeButton: 'transparent',
		customizeButtonText: '#78350f',
		border: '#fcd34d',
	},
	typography: { ...defaultTypography, fontFamily: '"Nunito", "Comic Sans MS", cursive, sans-serif' },
	spacing: { ...defaultSpacing, borderRadius: '24px', buttonBorderRadius: '9999px' },
	copy: {
		...defaultCopy,
		title: '🍪 Cookie Time!',
		description: 'We use cookies to make your experience sweeter! Accept all to enjoy the full flavor, or customize to pick your favorites.',
		acceptAll: 'Yum, Accept All!',
		rejectAll: 'No Thanks',
		customize: 'Pick & Choose',
	},
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 4px 20px rgba(245, 158, 11, 0.3)',
};

/**
 * Template 11: Neon Cyberpunk
 */
const neonCyberpunk: BannerTemplate = {
	id: 'neon-cyberpunk',
	name: 'Neon Cyberpunk',
	description: 'Futuristic cyberpunk style with neon accents.',
	category: 'dark',
	isEditable: true,
	position: 'bottom',
	style: 'modern',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'shield',
	buttonVariant: 'outline',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 300,
	colors: {
		background: '#0d0d0d',
		text: '#00ff88',
		textMuted: '#00cc6a',
		accent: '#ff00ff',
		acceptButton: 'transparent',
		acceptButtonText: '#00ff88',
		rejectButton: 'transparent',
		rejectButtonText: '#ff00ff',
		customizeButton: 'transparent',
		customizeButtonText: '#00ffff',
		border: '#00ff88',
	},
	typography: { ...defaultTypography, fontFamily: '"JetBrains Mono", "Fira Code", monospace' },
	spacing: { ...defaultSpacing, borderRadius: '0', buttonBorderRadius: '0' },
	copy: {
		...defaultCopy,
		title: '> PRIVACY_PROTOCOL',
		description: 'SYSTEM.REQUIRES.CONSENT // Enable tracking modules to enhance your experience. Choose your data permissions.',
		acceptAll: '[ACCEPT_ALL]',
		rejectAll: '[REJECT]',
		customize: '[CONFIG]',
	},
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 0 20px rgba(0, 255, 136, 0.3), 0 0 40px rgba(255, 0, 255, 0.1)',
	border: '1px solid #00ff88',
};

/**
 * Template 12: Elegant Serif
 */
const elegantSerif: BannerTemplate = {
	id: 'elegant-serif',
	name: 'Elegant Serif',
	description: 'Sophisticated design with serif typography for luxury brands.',
	category: 'corporate',
	isEditable: true,
	position: 'bottom',
	style: 'classic',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: false,
	buttonVariant: 'outline',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'fade',
	animationDuration: 400,
	colors: {
		background: '#faf9f7',
		text: '#1c1917',
		textMuted: '#78716c',
		accent: '#b45309',
		acceptButton: '#1c1917',
		acceptButtonText: '#faf9f7',
		rejectButton: 'transparent',
		rejectButtonText: '#78716c',
		customizeButton: 'transparent',
		customizeButtonText: '#1c1917',
		border: '#d6d3d1',
	},
	typography: {
		...defaultTypography,
		fontFamily: '"Playfair Display", "Georgia", serif',
		titleWeight: '500',
	},
	spacing: { ...defaultSpacing, borderRadius: '0', buttonBorderRadius: '0' },
	copy: {
		...defaultCopy,
		title: 'Privacy Notice',
		description: 'We employ cookies to deliver an exceptional experience tailored to your preferences. Your privacy matters to us.',
	},
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: 'none',
	border: '1px solid #d6d3d1',
};

/**
 * Template 13: Neumorphism Light
 */
const neumorphismLight: BannerTemplate = {
	id: 'neumorphism-light',
	name: 'Neumorphism Light',
	description: 'Soft UI design with subtle shadows creating depth.',
	category: 'modern',
	isEditable: true,
	position: 'bottom',
	style: 'neumorphism',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'cookie',
	buttonVariant: 'rounded',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 350,
	colors: {
		background: '#e0e5ec',
		text: '#2d3748',
		textMuted: '#718096',
		accent: '#4299e1',
		acceptButton: '#e0e5ec',
		acceptButtonText: '#4299e1',
		rejectButton: '#e0e5ec',
		rejectButtonText: '#718096',
		customizeButton: '#e0e5ec',
		customizeButtonText: '#2d3748',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing, borderRadius: '20px', buttonBorderRadius: '12px' },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '20px 20px 60px #bec3c9, -20px -20px 60px #ffffff',
};

/**
 * Template 14: Top Bar Minimal
 */
const topBarMinimal: BannerTemplate = {
	id: 'top-bar-minimal',
	name: 'Top Bar Minimal',
	description: 'Compact top bar that\'s unobtrusive yet visible.',
	category: 'minimal',
	isEditable: true,
	position: 'top',
	style: 'minimal',
	maxWidth: '100%',
	showCloseButton: true,
	showPrivacyLink: false,
	showIcon: false,
	buttonVariant: 'solid',
	showRejectButton: false,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'accept'],
	animation: 'slide-down',
	animationDuration: 250,
	colors: {
		background: '#1f2937',
		text: '#f9fafb',
		textMuted: '#9ca3af',
		accent: '#10b981',
		acceptButton: '#10b981',
		acceptButtonText: '#ffffff',
		rejectButton: 'transparent',
		rejectButtonText: '#9ca3af',
		customizeButton: 'transparent',
		customizeButtonText: '#f9fafb',
	},
	typography: { ...defaultTypography, titleSize: '0.875rem', descriptionSize: '0.8125rem' },
	spacing: { ...defaultSpacing, padding: '0.75rem 1.5rem', borderRadius: '0', buttonBorderRadius: '6px', buttonPadding: '0.5rem 1rem' },
	copy: {
		title: '',
		description: 'We use cookies to improve your experience.',
		acceptAll: 'Got it',
		rejectAll: '',
		customize: 'Learn more',
	},
	mobile: { ...defaultMobile, position: 'top', compact: true },
	tablet: { ...defaultTablet, position: 'top' },
	boxShadow: '0 4px 6px rgba(0, 0, 0, 0.1)',
};

/**
 * Template 15: Center Modal
 */
const centerModal: BannerTemplate = {
	id: 'center-modal',
	name: 'Center Modal',
	description: 'Attention-grabbing centered modal overlay.',
	category: 'modern',
	isEditable: true,
	position: 'center',
	style: 'card',
	maxWidth: '500px',
	showCloseButton: true,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'shield',
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['reject', 'customize', 'accept'],
	animation: 'scale',
	animationDuration: 300,
	colors: {
		background: '#ffffff',
		backdrop: 'rgba(0, 0, 0, 0.5)',
		text: '#111827',
		textMuted: '#6b7280',
		accent: '#6366f1',
		acceptButton: '#6366f1',
		acceptButtonText: '#ffffff',
		rejectButton: '#f3f4f6',
		rejectButtonText: '#6b7280',
		customizeButton: '#f3f4f6',
		customizeButtonText: '#374151',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing, padding: '2rem', borderRadius: '16px' },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile, position: 'center' },
	tablet: { ...defaultTablet, position: 'center', maxWidth: '480px' },
	boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
};

/**
 * Template 16: Gradient Sunset
 */
const gradientSunset: BannerTemplate = {
	id: 'gradient-sunset',
	name: 'Gradient Sunset',
	description: 'Warm sunset gradient with orange and pink tones.',
	category: 'modern',
	isEditable: true,
	position: 'bottom',
	style: 'gradient',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'cookie',
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 350,
	colors: {
		background: 'linear-gradient(135deg, #f97316 0%, #ec4899 50%, #8b5cf6 100%)',
		text: '#ffffff',
		textMuted: 'rgba(255, 255, 255, 0.85)',
		accent: '#ffffff',
		acceptButton: '#ffffff',
		acceptButtonText: '#ec4899',
		rejectButton: 'rgba(255, 255, 255, 0.2)',
		rejectButtonText: '#ffffff',
		customizeButton: 'transparent',
		customizeButtonText: '#ffffff',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 -4px 30px rgba(236, 72, 153, 0.4)',
};

/**
 * Template 17: Dark Minimal Card
 */
const darkMinimalCard: BannerTemplate = {
	id: 'dark-minimal-card',
	name: 'Dark Minimal Card',
	description: 'Ultra-minimal dark card with subtle styling.',
	category: 'dark',
	isEditable: true,
	position: 'bottom-right',
	style: 'card',
	maxWidth: '380px',
	showCloseButton: true,
	showPrivacyLink: false,
	showIcon: false,
	buttonVariant: 'solid',
	showRejectButton: false,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'accept'],
	animation: 'slide-left',
	animationDuration: 300,
	colors: {
		background: '#171717',
		text: '#fafafa',
		textMuted: '#737373',
		accent: '#fafafa',
		acceptButton: '#fafafa',
		acceptButtonText: '#171717',
		rejectButton: '#262626',
		rejectButtonText: '#a3a3a3',
		customizeButton: 'transparent',
		customizeButtonText: '#a3a3a3',
		border: '#262626',
	},
	typography: { ...defaultTypography, titleSize: '0.9375rem', descriptionSize: '0.8125rem' },
	spacing: { ...defaultSpacing, padding: '1.25rem', borderRadius: '12px', buttonBorderRadius: '6px' },
	copy: {
		...defaultCopy,
		title: 'Cookies',
		description: 'This site uses cookies to enhance your experience.',
		acceptAll: 'Accept',
		customize: 'Settings',
	},
	mobile: { ...defaultMobile, position: 'bottom', useDrawer: true },
	tablet: { ...defaultTablet, position: 'bottom-right', maxWidth: '360px' },
	boxShadow: '0 8px 30px rgba(0, 0, 0, 0.4)',
	border: '1px solid #262626',
};

/**
 * Template 18: Fullscreen Blur
 */
const fullscreenBlur: BannerTemplate = {
	id: 'fullscreen-blur',
	name: 'Fullscreen Blur',
	description: 'Full-screen overlay with blurred background for maximum attention.',
	category: 'modern',
	isEditable: true,
	position: 'fullscreen',
	style: 'glassmorphism',
	maxWidth: '600px',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'shield',
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['reject', 'customize', 'accept'],
	animation: 'fade',
	animationDuration: 400,
	colors: {
		background: 'rgba(0, 0, 0, 0.8)',
		backdrop: 'rgba(0, 0, 0, 0.6)',
		text: '#ffffff',
		textMuted: 'rgba(255, 255, 255, 0.7)',
		accent: '#3b82f6',
		acceptButton: '#3b82f6',
		acceptButtonText: '#ffffff',
		rejectButton: 'rgba(255, 255, 255, 0.1)',
		rejectButtonText: '#ffffff',
		customizeButton: 'rgba(255, 255, 255, 0.1)',
		customizeButtonText: '#ffffff',
		border: 'rgba(255, 255, 255, 0.1)',
	},
	typography: { ...defaultTypography, titleSize: '1.5rem' },
	spacing: { ...defaultSpacing, padding: '2.5rem', borderRadius: '20px', gap: '1.5rem' },
	copy: { ...defaultCopy },
	mobile: { ...defaultMobile, position: 'fullscreen' },
	tablet: { ...defaultTablet, position: 'fullscreen', maxWidth: '550px' },
	backdropBlur: '10px',
	boxShadow: '0 0 100px rgba(59, 130, 246, 0.2)',
	border: '1px solid rgba(255, 255, 255, 0.1)',
};

/**
 * Template 19: Nature Green
 */
const natureGreen: BannerTemplate = {
	id: 'nature-green',
	name: 'Nature Green',
	description: 'Eco-friendly green theme for environmentally conscious brands.',
	category: 'light',
	isEditable: true,
	position: 'bottom',
	style: 'flat',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: true,
	iconType: 'checkmark',
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 300,
	colors: {
		background: '#f0fdf4',
		text: '#14532d',
		textMuted: '#166534',
		accent: '#22c55e',
		acceptButton: '#22c55e',
		acceptButtonText: '#ffffff',
		rejectButton: '#dcfce7',
		rejectButtonText: '#166534',
		customizeButton: 'transparent',
		customizeButtonText: '#14532d',
		border: '#bbf7d0',
	},
	typography: { ...defaultTypography },
	spacing: { ...defaultSpacing },
	copy: {
		...defaultCopy,
		title: '🌿 Privacy & Cookies',
		description: 'We care about your privacy as much as we care about the environment. Choose how you\'d like us to use cookies.',
	},
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 -4px 20px rgba(34, 197, 94, 0.15)',
	border: '1px solid #bbf7d0',
};

/**
 * Template 20: Premium Gold
 */
const premiumGold: BannerTemplate = {
	id: 'premium-gold',
	name: 'Premium Gold',
	description: 'Luxurious gold accents for premium and exclusive brands.',
	category: 'corporate',
	isEditable: true,
	position: 'bottom',
	style: 'classic',
	maxWidth: '100%',
	showCloseButton: false,
	showPrivacyLink: true,
	showIcon: false,
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'fade',
	animationDuration: 400,
	colors: {
		background: 'linear-gradient(135deg, #1a1a1a 0%, #2d2d2d 100%)',
		text: '#fafafa',
		textMuted: '#a3a3a3',
		accent: '#d4af37',
		acceptButton: 'linear-gradient(135deg, #d4af37 0%, #f4d03f 100%)',
		acceptButtonText: '#1a1a1a',
		rejectButton: 'transparent',
		rejectButtonText: '#a3a3a3',
		customizeButton: 'transparent',
		customizeButtonText: '#d4af37',
		border: '#d4af37',
	},
	typography: {
		...defaultTypography,
		fontFamily: '"Cinzel", "Times New Roman", serif',
		titleWeight: '500',
	},
	spacing: { ...defaultSpacing },
	copy: {
		...defaultCopy,
		title: 'Privacy Preferences',
		description: 'We value your privacy and are committed to being transparent about how we collect and use your information.',
	},
	mobile: { ...defaultMobile },
	tablet: { ...defaultTablet },
	boxShadow: '0 -4px 30px rgba(212, 175, 55, 0.2)',
	border: '1px solid rgba(212, 175, 55, 0.3)',
};

/**
 * Template 21: Revolution Trading Template
 *
 * Custom Revolution Trading consent banner design.
 * White background, orange accept button, gray reject/options buttons.
 */
const revolutionTrading: BannerTemplate = {
	id: 'revolution-trading',
	name: 'Revolution Trading Template',
	description: 'Revolution Trading consent banner. White background with orange accent.',
	category: 'corporate',
	isEditable: true,
	position: 'bottom',
	style: 'flat',
	maxWidth: '100%',
	showCloseButton: true,
	showPrivacyLink: true,
	showIcon: false,
	buttonVariant: 'solid',
	showRejectButton: true,
	showCustomizeButton: true,
	buttonOrder: ['customize', 'reject', 'accept'],
	animation: 'slide-up',
	animationDuration: 300,
	colors: {
		background: '#ffffff',
		text: '#5D5D5D',
		textMuted: '#5D5D5D',
		accent: '#005BD3',
		acceptButton: '#E16B43',
		acceptButtonText: '#ffffff',
		rejectButton: '#F0F0F0',
		rejectButtonText: '#212121',
		customizeButton: '#F0F0F0',
		customizeButtonText: '#212121',
		border: '#333333',
	},
	typography: {
		fontFamily: 'Arial, sans-serif',
		titleSize: '0',
		titleWeight: '400',
		descriptionSize: '20px',
		buttonSize: '16px',
		buttonWeight: '500',
		lineHeight: '1.5',
	},
	spacing: {
		padding: '45px 80px 40px 80px',
		gap: '0',
		buttonPadding: '20px 25px 20px 20px',
		borderRadius: '0',
		buttonBorderRadius: '0',
	},
	copy: {
		title: '',
		description: 'By clicking "Accept," you agree to our Terms of Use, Privacy Policy and consent to the use of cookies and similar tracking technologies to, among other things, serve you relevant ads ourselves or through our third-party ad partners with whom data from cookies is shared.',
		acceptAll: 'Accept',
		rejectAll: 'Reject',
		customize: 'More options',
		privacyPolicy: 'Privacy Policy',
		cookiePolicy: 'Cookie Policy',
	},
	mobile: {
		position: 'bottom',
		useDrawer: false,
		stackButtons: true,
		compact: false,
		fullWidthButtons: true,
		padding: '30px 20px',
	},
	tablet: {
		position: 'bottom',
		maxWidth: '100%',
		stackButtons: false,
	},
	boxShadow: '0 -4px 20px rgba(0, 0, 0, 0.15)',
	border: '1px solid #333333',
};

// =============================================================================
// REGISTRY
// =============================================================================

/**
 * All available templates
 */
export const BANNER_TEMPLATES: BannerTemplate[] = [
	minimalDark,
	minimalLight,
	glassmorphism,
	gradientPurple,
	gradientOcean,
	floatingCard,
	floatingDarkCard,
	corporateBlue,
	enterpriseGray,
	playfulRounded,
	neonCyberpunk,
	elegantSerif,
	neumorphismLight,
	topBarMinimal,
	centerModal,
	gradientSunset,
	darkMinimalCard,
	fullscreenBlur,
	natureGreen,
	premiumGold,
	revolutionTrading,
];

/**
 * Get template by ID
 */
export function getTemplate(id: string): BannerTemplate | undefined {
	return BANNER_TEMPLATES.find((t) => t.id === id);
}

/**
 * Get templates by category
 */
export function getTemplatesByCategory(category: string): BannerTemplate[] {
	return BANNER_TEMPLATES.filter((t) => t.category === category);
}

/**
 * Get all template categories
 */
export function getTemplateCategories(): string[] {
	return [...new Set(BANNER_TEMPLATES.map((t) => t.category))];
}

/**
 * Get template previews (lightweight list)
 */
export function getTemplatePreviews(): { id: string; name: string; category: string; description: string }[] {
	return BANNER_TEMPLATES.map((t) => ({
		id: t.id,
		name: t.name,
		category: t.category,
		description: t.description,
	}));
}

/**
 * Default template ID
 */
export const DEFAULT_TEMPLATE_ID = 'minimal-dark';
