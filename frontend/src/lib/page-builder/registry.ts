/**
 * Component Registry
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * Central registry of all available page builder components.
 * Includes all 18 CMS v2 block types + legacy course components.
 * Easy to extend - just add new entries to the registry.
 */

import type { ComponentRegistryEntry, ComponentType } from './types';

export const componentRegistry: ComponentRegistryEntry[] = [
	// ═══════════════════════════════════════════════════════════════════════════
	// LEGACY COURSE COMPONENTS
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'course-header',
		name: 'Course Header',
		description: 'Title section with course info and optional login button',
		icon: '📋',
		category: 'content',
		defaultConfig: {
			title: 'Course Title',
			subtitle: '',
			description: '',
			instructorName: '',
			instructorTitle: '',
			showLoginButton: true,
			loginButtonText: 'LOGIN TO THE CLASSROOM',
			backgroundColor: '#143E59',
			textColor: '#FFFFFF'
		},
		tags: ['course', 'header']
	},
	{
		type: 'video-player',
		name: 'Video Player',
		description: 'Single video with title header',
		icon: '🎬',
		category: 'media',
		defaultConfig: {
			title: 'Video Title',
			subtitle: '',
			videoId: '',
			bunnyVideoGuid: '',
			thumbnailUrl: '',
			autoplay: false
		},
		tags: ['video', 'media']
	},
	{
		type: 'video-stack',
		name: 'Video Stack',
		description: 'Multiple videos stacked vertically',
		icon: '📚',
		category: 'media',
		defaultConfig: {
			videos: [],
			showDates: true,
			sortOrder: 'newest'
		},
		tags: ['video', 'playlist']
	},
	{
		type: 'class-downloads',
		name: 'Class Downloads',
		description: 'File download section with Box-like interface',
		icon: '📁',
		category: 'content',
		defaultConfig: {
			title: 'Class Downloads',
			maxHeight: '400px'
		},
		tags: ['files', 'downloads']
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// CMS V2 - HERO & MARKETING
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'hero-slider',
		name: 'Hero Slider',
		description: 'Full-width carousel with CTA buttons',
		icon: '🎠',
		category: 'content',
		defaultConfig: {
			slides: [
				{
					id: 'slide-1',
					title: 'Welcome to Revolution Trading Pros',
					subtitle: 'Transform Your Trading Journey',
					ctaText: 'Get Started',
					ctaUrl: '/checkout',
					ctaStyle: 'primary',
					backgroundColor: '#143E59',
					textColor: '#FFFFFF',
					alignment: 'center'
				}
			],
			autoplay: true,
			autoplayInterval: 5000,
			showDots: true,
			showArrows: true,
			height: '600px'
		},
		tags: ['hero', 'slider', 'carousel', 'marketing'],
		isPremium: true
	},
	{
		type: 'cta-banner',
		name: 'CTA Banner',
		description: 'Call-to-action section with buttons',
		icon: '📣',
		category: 'content',
		defaultConfig: {
			title: 'Ready to Start Trading?',
			subtitle: 'Join thousands of successful traders',
			ctaText: 'Get Started Now',
			ctaUrl: '/checkout',
			ctaStyle: 'primary',
			backgroundType: 'gradient',
			backgroundColor: '#143E59',
			backgroundGradient: 'linear-gradient(135deg, #143E59 0%, #1a5276 100%)',
			textColor: '#FFFFFF',
			alignment: 'center',
			padding: 'large'
		},
		tags: ['cta', 'marketing', 'banner']
	},
	{
		type: 'email-capture',
		name: 'Email Capture',
		description: 'Newsletter signup form',
		icon: '📧',
		category: 'content',
		defaultConfig: {
			title: 'Stay Updated',
			subtitle: 'Get the latest trading insights delivered to your inbox',
			placeholder: 'Enter your email',
			buttonText: 'Subscribe',
			successMessage: 'Thanks for subscribing!',
			style: 'card',
			backgroundColor: '#f8f9fa',
			textColor: '#333333',
			showNameField: false
		},
		tags: ['email', 'newsletter', 'form']
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// CMS V2 - TRADING SPECIFIC
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'trading-rooms-grid',
		name: 'Trading Rooms Grid',
		description: 'Display trading rooms with live status',
		icon: '📊',
		category: 'trading',
		defaultConfig: {
			columns: 3,
			showStatus: true,
			showSchedule: true,
			showDescription: true,
			maxItems: 6
		},
		tags: ['trading', 'rooms', 'grid'],
		isPremium: true
	},
	{
		type: 'alert-services-grid',
		name: 'Alert Services Grid',
		description: 'Display alert services with pricing',
		icon: '🔔',
		category: 'trading',
		defaultConfig: {
			columns: 3,
			showPricing: true,
			showDescription: true,
			maxItems: 6
		},
		tags: ['alerts', 'services', 'grid'],
		isPremium: true
	},
	{
		type: 'indicators-showcase',
		name: 'Indicators Showcase',
		description: 'Display trading indicators',
		icon: '📈',
		category: 'trading',
		defaultConfig: {
			layout: 'grid',
			columns: 3,
			maxItems: 6,
			showPricing: true,
			showDescription: true,
			showPlatform: true
		},
		tags: ['indicators', 'trading', 'tools'],
		isPremium: true
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// CMS V2 - CONTENT GRIDS
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'testimonials-masonry',
		name: 'Testimonials Masonry',
		description: 'Pinterest-style testimonial layout',
		icon: '💬',
		category: 'content',
		defaultConfig: {
			columns: 3,
			maxItems: 9,
			showAvatar: true,
			showRating: true,
			backgroundColor: '#ffffff'
		},
		tags: ['testimonials', 'reviews', 'social-proof']
	},
	{
		type: 'features-grid',
		name: 'Features Grid',
		description: 'Icon/image based feature highlights',
		icon: '✨',
		category: 'content',
		defaultConfig: {
			columns: 3,
			items: [
				{
					id: 'feature-1',
					icon: '📊',
					title: 'Real-Time Analysis',
					description: 'Get instant market insights'
				},
				{
					id: 'feature-2',
					icon: '🎓',
					title: 'Expert Education',
					description: 'Learn from professional traders'
				},
				{
					id: 'feature-3',
					icon: '💬',
					title: 'Community Support',
					description: 'Join a thriving trading community'
				}
			],
			style: 'cards',
			iconSize: 'large'
		},
		tags: ['features', 'benefits', 'icons']
	},
	{
		type: 'blog-feed',
		name: 'Blog Feed',
		description: 'Display recent blog posts',
		icon: '📰',
		category: 'content',
		defaultConfig: {
			layout: 'grid',
			columns: 3,
			maxItems: 6,
			showExcerpt: true,
			showAuthor: true,
			showDate: true,
			showImage: true,
			showCategory: true
		},
		tags: ['blog', 'posts', 'articles']
	},
	{
		type: 'courses-grid',
		name: 'Courses Grid',
		description: 'Display available courses',
		icon: '🎓',
		category: 'content',
		defaultConfig: {
			layout: 'grid',
			columns: 3,
			maxItems: 6,
			showProgress: false,
			showInstructor: true,
			showDuration: true,
			showLevel: true
		},
		tags: ['courses', 'education', 'learning']
	},
	{
		type: 'stats-counter',
		name: 'Stats Counter',
		description: 'Animated number counters',
		icon: '🔢',
		category: 'content',
		defaultConfig: {
			items: [
				{ id: 'stat-1', value: 18000, suffix: '+', label: 'Active Traders' },
				{ id: 'stat-2', value: 50, suffix: '+', label: 'Trading Courses' },
				{ id: 'stat-3', value: 99, suffix: '%', label: 'Satisfaction Rate' },
				{ id: 'stat-4', value: 24, suffix: '/7', label: 'Support' }
			],
			columns: 4,
			style: 'minimal',
			animate: true,
			animationDuration: 2000,
			backgroundColor: 'transparent',
			textColor: '#333333'
		},
		tags: ['stats', 'numbers', 'counters', 'social-proof']
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// CMS V2 - TEXT & MEDIA
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'rich-text',
		name: 'Rich Text',
		description: 'WYSIWYG content block',
		icon: '📝',
		category: 'content',
		defaultConfig: {
			content: '<p>Enter your content here...</p>',
			contentFormat: 'html',
			maxWidth: '800px',
			textAlign: 'left'
		},
		tags: ['text', 'content', 'wysiwyg']
	},
	{
		type: 'image',
		name: 'Image',
		description: 'Single image with optional caption',
		icon: '🖼️',
		category: 'media',
		defaultConfig: {
			imageUrl: '',
			alt: 'Image description',
			caption: '',
			width: '100%',
			objectFit: 'cover',
			alignment: 'center',
			borderRadius: '8px',
			shadow: 'medium'
		},
		tags: ['image', 'photo', 'media']
	},
	{
		type: 'video-embed',
		name: 'Video Embed',
		description: 'YouTube, Vimeo, or Bunny.net video',
		icon: '🎥',
		category: 'media',
		defaultConfig: {
			videoSource: 'youtube',
			videoId: '',
			aspectRatio: '16:9',
			autoplay: false,
			muted: false,
			loop: false,
			controls: true
		},
		tags: ['video', 'embed', 'youtube', 'vimeo']
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// CMS V2 - LAYOUT & STRUCTURE
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'spacer',
		name: 'Spacer',
		description: 'Adjustable vertical space',
		icon: '↕️',
		category: 'layout',
		defaultConfig: {
			height: 40,
			mobileHeight: 20,
			backgroundColor: 'transparent'
		},
		tags: ['space', 'gap', 'layout']
	},
	{
		type: 'divider',
		name: 'Divider',
		description: 'Horizontal line separator',
		icon: '➖',
		category: 'layout',
		defaultConfig: {
			style: 'solid',
			color: '#E0E0E0',
			thickness: 1,
			width: '100%',
			marginTop: 20,
			marginBottom: 20,
			alignment: 'center'
		},
		tags: ['line', 'separator', 'layout']
	},
	{
		type: 'two-column-layout',
		name: 'Two Column Layout',
		description: 'Side-by-side content sections',
		icon: '⬜⬜',
		category: 'layout',
		defaultConfig: {
			leftColumn: {
				type: 'text',
				content: '<p>Left column content</p>',
				padding: '20px',
				backgroundColor: 'transparent'
			},
			rightColumn: {
				type: 'image',
				imageUrl: '',
				padding: '20px',
				backgroundColor: 'transparent'
			},
			ratio: '50-50',
			gap: 'medium',
			verticalAlignment: 'center',
			reverseOnMobile: false,
			stackOnMobile: true
		},
		tags: ['columns', 'layout', 'grid']
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// CMS V2 - COMMERCE
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'pricing-table',
		name: 'Pricing Table',
		description: 'Compare pricing plans',
		icon: '💰',
		category: 'commerce',
		defaultConfig: {
			columns: 3,
			plans: [
				{
					id: 'plan-basic',
					name: 'Basic',
					description: 'Perfect for beginners',
					monthlyPrice: 97,
					annualPrice: 970,
					currency: 'USD',
					features: [
						{ text: 'Access to basic courses', included: true },
						{ text: 'Community access', included: true },
						{ text: 'Live trading rooms', included: false },
						{ text: 'Personal mentorship', included: false }
					],
					ctaText: 'Get Started',
					ctaUrl: '/checkout/basic'
				},
				{
					id: 'plan-pro',
					name: 'Pro',
					description: 'For serious traders',
					monthlyPrice: 197,
					annualPrice: 1970,
					currency: 'USD',
					features: [
						{ text: 'All basic features', included: true },
						{ text: 'Advanced courses', included: true },
						{ text: 'Live trading rooms', included: true },
						{ text: 'Personal mentorship', included: false }
					],
					ctaText: 'Go Pro',
					ctaUrl: '/checkout/pro',
					isPopular: true,
					badge: 'Most Popular'
				},
				{
					id: 'plan-elite',
					name: 'Elite',
					description: 'Maximum results',
					monthlyPrice: 497,
					annualPrice: 4970,
					currency: 'USD',
					features: [
						{ text: 'All Pro features', included: true },
						{ text: 'Personal mentorship', included: true },
						{ text: 'Priority support', included: true },
						{ text: 'Exclusive strategies', included: true }
					],
					ctaText: 'Join Elite',
					ctaUrl: '/checkout/elite'
				}
			],
			showMonthly: true,
			showAnnual: true,
			defaultBilling: 'monthly',
			highlightedPlanId: 'plan-pro'
		},
		tags: ['pricing', 'plans', 'commerce'],
		isPremium: true
	},
	{
		type: 'faq-accordion',
		name: 'FAQ Accordion',
		description: 'Expandable Q&A section',
		icon: '❓',
		category: 'content',
		defaultConfig: {
			items: [
				{
					id: 'faq-1',
					question: 'What is included in the membership?',
					answer:
						'<p>Your membership includes access to all trading rooms, educational content, and our community of traders.</p>'
				},
				{
					id: 'faq-2',
					question: 'Can I cancel my subscription?',
					answer:
						'<p>Yes, you can cancel your subscription at any time. No long-term contracts required.</p>'
				},
				{
					id: 'faq-3',
					question: 'Do you offer refunds?',
					answer: '<p>We offer a 30-day money-back guarantee on all subscriptions.</p>'
				}
			],
			style: 'bordered',
			allowMultipleOpen: false,
			schema: true
		},
		tags: ['faq', 'questions', 'help']
	}
];

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

export function getComponentByType(type: ComponentType): ComponentRegistryEntry | undefined {
	return componentRegistry.find((c) => c.type === type);
}

export function getComponentsByCategory(
	category: 'content' | 'media' | 'layout' | 'commerce' | 'trading'
): ComponentRegistryEntry[] {
	return componentRegistry.filter((c) => c.category === category);
}

export function createDefaultConfig(type: ComponentType): Record<string, unknown> {
	const entry = getComponentByType(type);
	return entry ? JSON.parse(JSON.stringify(entry.defaultConfig)) : {};
}

export function getComponentsByTags(tags: string[]): ComponentRegistryEntry[] {
	return componentRegistry.filter((c) => tags.some((tag) => c.tags?.includes(tag)));
}

export function searchComponents(query: string): ComponentRegistryEntry[] {
	const lowerQuery = query.toLowerCase();
	return componentRegistry.filter(
		(c) =>
			c.name.toLowerCase().includes(lowerQuery) ||
			c.description.toLowerCase().includes(lowerQuery) ||
			c.tags?.some((tag) => tag.includes(lowerQuery))
	);
}

export function getPremiumComponents(): ComponentRegistryEntry[] {
	return componentRegistry.filter((c) => c.isPremium);
}

export function getCategories(): Array<{
	id: string;
	name: string;
	count: number;
}> {
	const categories = ['content', 'media', 'layout', 'commerce', 'trading'] as const;
	return categories.map((cat) => ({
		id: cat,
		name: cat.charAt(0).toUpperCase() + cat.slice(1),
		count: componentRegistry.filter((c) => c.category === cat).length
	}));
}
