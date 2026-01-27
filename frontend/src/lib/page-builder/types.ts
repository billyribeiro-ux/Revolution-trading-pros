/**
 * Page Builder Type Definitions
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * Core types for the drag-and-drop page builder system.
 * Supports all 18 block types for the custom CMS implementation.
 * Designed for extensibility and type safety.
 */

// ═══════════════════════════════════════════════════════════════════════════════
// COMPONENT TYPES - All 18 Block Types + Legacy Course Components
// ═══════════════════════════════════════════════════════════════════════════════

export type ComponentType =
	// Legacy course components
	| 'course-header'
	| 'video-player'
	| 'video-stack'
	| 'class-downloads'
	// CMS v2 Block Types (18 total)
	| 'hero-slider'
	| 'trading-rooms-grid'
	| 'alert-services-grid'
	| 'testimonials-masonry'
	| 'features-grid'
	| 'rich-text'
	| 'image'
	| 'video-embed'
	| 'spacer'
	| 'divider'
	| 'email-capture'
	| 'blog-feed'
	| 'indicators-showcase'
	| 'courses-grid'
	| 'faq-accordion'
	| 'pricing-table'
	| 'cta-banner'
	| 'stats-counter'
	| 'two-column-layout';

// ═══════════════════════════════════════════════════════════════════════════════
// LEGACY COMPONENT CONFIGURATIONS
// ═══════════════════════════════════════════════════════════════════════════════

export interface CourseHeaderConfig {
	title: string;
	subtitle?: string;
	description?: string;
	instructorName?: string;
	instructorTitle?: string;
	instructorAvatar?: string;
	showLoginButton?: boolean;
	loginButtonText?: string;
	loginButtonUrl?: string;
	backgroundColor?: string;
	textColor?: string;
}

export interface VideoPlayerConfig {
	videoId?: string;
	bunnyVideoGuid?: string;
	bunnyLibraryId?: number;
	title: string;
	subtitle?: string;
	thumbnailUrl?: string;
	duration?: number;
	autoplay?: boolean;
}

export interface VideoStackConfig {
	videos: VideoPlayerConfig[];
	showDates?: boolean;
	sortOrder?: 'newest' | 'oldest' | 'manual';
}

export interface ClassDownloadsConfig {
	courseId?: string;
	courseSlug?: string;
	title?: string;
	maxHeight?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CMS V2 BLOCK CONFIGURATIONS
// ═══════════════════════════════════════════════════════════════════════════════

/** Hero Slider - Full-width carousel with CTA buttons */
export interface HeroSliderConfig {
	slides: HeroSlide[];
	autoplay?: boolean;
	autoplayInterval?: number; // ms
	showDots?: boolean;
	showArrows?: boolean;
	height?: string;
}

export interface HeroSlide {
	id: string;
	title: string;
	subtitle?: string;
	description?: string;
	backgroundImageId?: string;
	backgroundImageUrl?: string;
	backgroundColor?: string;
	textColor?: string;
	ctaText?: string;
	ctaUrl?: string;
	ctaStyle?: 'primary' | 'secondary' | 'outline';
	secondaryCtaText?: string;
	secondaryCtaUrl?: string;
	alignment?: 'left' | 'center' | 'right';
}

/** Trading Rooms Grid - Display trading rooms with status */
export interface TradingRoomsGridConfig {
	columns?: 2 | 3 | 4;
	showStatus?: boolean;
	showSchedule?: boolean;
	showDescription?: boolean;
	maxItems?: number;
	roomIds?: string[]; // Specific rooms to show, or empty for all
}

/** Alert Services Grid - Display alert services */
export interface AlertServicesGridConfig {
	columns?: 2 | 3 | 4;
	showPricing?: boolean;
	showDescription?: boolean;
	maxItems?: number;
	serviceIds?: string[];
}

/** Testimonials Masonry - Pinterest-style testimonial layout */
export interface TestimonialsMasonryConfig {
	columns?: 2 | 3 | 4;
	maxItems?: number;
	showAvatar?: boolean;
	showRating?: boolean;
	testimonialIds?: string[];
	backgroundColor?: string;
}

/** Features Grid - Icon/image based feature highlights */
export interface FeaturesGridConfig {
	columns?: 2 | 3 | 4;
	items: FeatureItem[];
	style?: 'cards' | 'minimal' | 'icons';
	iconSize?: 'small' | 'medium' | 'large';
}

export interface FeatureItem {
	id: string;
	icon?: string;
	imageId?: string;
	imageUrl?: string;
	title: string;
	description?: string;
	linkText?: string;
	linkUrl?: string;
}

/** Rich Text - WYSIWYG content block */
export interface RichTextConfig {
	content: string; // HTML content from Tiptap
	contentFormat?: 'html' | 'markdown' | 'raw';
	maxWidth?: string;
	textAlign?: 'left' | 'center' | 'right' | 'justify';
}

/** Image - Single image with optional caption */
export interface ImageConfig {
	imageId?: string;
	imageUrl?: string;
	alt: string;
	caption?: string;
	width?: string;
	height?: string;
	objectFit?: 'cover' | 'contain' | 'fill' | 'none';
	alignment?: 'left' | 'center' | 'right';
	linkUrl?: string;
	linkTarget?: '_self' | '_blank';
	borderRadius?: string;
	shadow?: 'none' | 'small' | 'medium' | 'large';
}

/** Video Embed - YouTube, Vimeo, or Bunny.net video */
export interface VideoEmbedConfig {
	videoSource: 'youtube' | 'vimeo' | 'bunny' | 'custom';
	videoId?: string;
	videoUrl?: string;
	bunnyVideoGuid?: string;
	bunnyLibraryId?: number;
	title?: string;
	thumbnail?: string;
	aspectRatio?: '16:9' | '4:3' | '1:1' | '9:16';
	autoplay?: boolean;
	muted?: boolean;
	loop?: boolean;
	controls?: boolean;
}

/** Spacer - Vertical spacing */
export interface SpacerConfig {
	height: number; // in pixels
	mobileHeight?: number;
	backgroundColor?: string;
}

/** Divider - Horizontal line separator */
export interface DividerConfig {
	style?: 'solid' | 'dashed' | 'dotted' | 'double';
	color?: string;
	thickness?: number;
	width?: string;
	marginTop?: number;
	marginBottom?: number;
	alignment?: 'left' | 'center' | 'right';
}

/** Email Capture - Newsletter signup form */
export interface EmailCaptureConfig {
	title?: string;
	subtitle?: string;
	placeholder?: string;
	buttonText?: string;
	successMessage?: string;
	formId?: string;
	listId?: string;
	style?: 'inline' | 'stacked' | 'card';
	backgroundColor?: string;
	textColor?: string;
	showNameField?: boolean;
}

/** Blog Feed - Display recent blog posts */
export interface BlogFeedConfig {
	layout?: 'grid' | 'list' | 'carousel';
	columns?: 2 | 3 | 4;
	maxItems?: number;
	showExcerpt?: boolean;
	showAuthor?: boolean;
	showDate?: boolean;
	showImage?: boolean;
	showCategory?: boolean;
	categoryIds?: string[];
	tagIds?: string[];
}

/** Indicators Showcase - Display trading indicators */
export interface IndicatorsShowcaseConfig {
	layout?: 'grid' | 'list' | 'featured';
	columns?: 2 | 3 | 4;
	maxItems?: number;
	showPricing?: boolean;
	showDescription?: boolean;
	showPlatform?: boolean;
	indicatorIds?: string[];
}

/** Courses Grid - Display courses */
export interface CoursesGridConfig {
	layout?: 'grid' | 'list' | 'carousel';
	columns?: 2 | 3 | 4;
	maxItems?: number;
	showProgress?: boolean;
	showInstructor?: boolean;
	showDuration?: boolean;
	showLevel?: boolean;
	categoryIds?: string[];
	courseIds?: string[];
}

/** FAQ Accordion - Expandable Q&A section */
export interface FaqAccordionConfig {
	items: FaqItem[];
	style?: 'simple' | 'bordered' | 'cards';
	allowMultipleOpen?: boolean;
	defaultOpenIndex?: number;
	schema?: boolean; // Generate FAQ schema.org markup
}

export interface FaqItem {
	id: string;
	question: string;
	answer: string; // HTML content
}

/** Pricing Table - Compare pricing plans */
export interface PricingTableConfig {
	columns?: 2 | 3 | 4;
	plans: PricingPlan[];
	showMonthly?: boolean;
	showAnnual?: boolean;
	defaultBilling?: 'monthly' | 'annual';
	highlightedPlanId?: string;
}

export interface PricingPlan {
	id: string;
	name: string;
	description?: string;
	monthlyPrice?: number;
	annualPrice?: number;
	currency?: string;
	features: PricingFeature[];
	ctaText?: string;
	ctaUrl?: string;
	isPopular?: boolean;
	badge?: string;
}

export interface PricingFeature {
	text: string;
	included: boolean;
	tooltip?: string;
}

/** CTA Banner - Call-to-action section */
export interface CtaBannerConfig {
	title: string;
	subtitle?: string;
	description?: string;
	ctaText: string;
	ctaUrl: string;
	ctaStyle?: 'primary' | 'secondary' | 'outline';
	secondaryCtaText?: string;
	secondaryCtaUrl?: string;
	backgroundType?: 'color' | 'gradient' | 'image';
	backgroundColor?: string;
	backgroundGradient?: string;
	backgroundImageId?: string;
	backgroundImageUrl?: string;
	textColor?: string;
	alignment?: 'left' | 'center' | 'right';
	padding?: 'small' | 'medium' | 'large';
}

/** Stats Counter - Animated number counters */
export interface StatsCounterConfig {
	items: StatItem[];
	columns?: 2 | 3 | 4;
	style?: 'minimal' | 'cards' | 'icons';
	animate?: boolean;
	animationDuration?: number;
	backgroundColor?: string;
	textColor?: string;
}

export interface StatItem {
	id: string;
	value: number;
	prefix?: string;
	suffix?: string;
	label: string;
	icon?: string;
	description?: string;
}

/** Two Column Layout - Side-by-side content sections */
export interface TwoColumnLayoutConfig {
	leftColumn: ColumnContent;
	rightColumn: ColumnContent;
	ratio?: '50-50' | '33-67' | '67-33' | '40-60' | '60-40';
	gap?: 'small' | 'medium' | 'large';
	verticalAlignment?: 'top' | 'center' | 'bottom';
	reverseOnMobile?: boolean;
	stackOnMobile?: boolean;
}

export interface ColumnContent {
	type: 'text' | 'image' | 'video' | 'blocks';
	content?: string; // HTML for text
	imageId?: string;
	imageUrl?: string;
	videoId?: string;
	blocks?: PageBlock[]; // Nested blocks
	padding?: string;
	backgroundColor?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// UNION TYPE FOR ALL CONFIGS
// ═══════════════════════════════════════════════════════════════════════════════

export type ComponentConfig =
	// Legacy
	| CourseHeaderConfig
	| VideoPlayerConfig
	| VideoStackConfig
	| ClassDownloadsConfig
	// CMS v2
	| HeroSliderConfig
	| TradingRoomsGridConfig
	| AlertServicesGridConfig
	| TestimonialsMasonryConfig
	| FeaturesGridConfig
	| RichTextConfig
	| ImageConfig
	| VideoEmbedConfig
	| SpacerConfig
	| DividerConfig
	| EmailCaptureConfig
	| BlogFeedConfig
	| IndicatorsShowcaseConfig
	| CoursesGridConfig
	| FaqAccordionConfig
	| PricingTableConfig
	| CtaBannerConfig
	| StatsCounterConfig
	| TwoColumnLayoutConfig;

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE BUILDER BLOCK
// ═══════════════════════════════════════════════════════════════════════════════

export interface PageBlock {
	id: string;
	type: ComponentType;
	config: ComponentConfig;
	order: number;
	/** Block-level settings (visibility, spacing, etc.) */
	settings?: BlockSettings;
}

/** Settings that apply to any block */
export interface BlockSettings {
	isHidden?: boolean;
	cssClass?: string;
	cssId?: string;
	marginTop?: string;
	marginBottom?: string;
	paddingTop?: string;
	paddingBottom?: string;
	backgroundColor?: string;
	backgroundImageId?: string;
	hideOnMobile?: boolean;
	hideOnDesktop?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE LAYOUT
// ═══════════════════════════════════════════════════════════════════════════════

export interface PageLayout {
	id?: string;
	courseId?: string;
	courseSlug?: string;
	contentId?: string; // Link to CMS content
	title: string;
	blocks: PageBlock[];
	createdAt?: string;
	updatedAt?: string;
	publishedAt?: string;
	status: 'draft' | 'published' | 'archived';
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPONENT REGISTRY ENTRY
// ═══════════════════════════════════════════════════════════════════════════════

export interface ComponentRegistryEntry {
	type: ComponentType;
	name: string;
	description: string;
	icon: string;
	category: 'content' | 'media' | 'layout' | 'commerce' | 'trading';
	defaultConfig: ComponentConfig;
	/** Tags for filtering */
	tags?: string[];
	/** Whether this is a premium/advanced component */
	isPremium?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════════
// DRAG STATE
// ═══════════════════════════════════════════════════════════════════════════════

export interface DragState {
	isDragging: boolean;
	draggedType: ComponentType | null;
	draggedBlockId: string | null;
	dropTargetIndex: number | null;
}

// ═══════════════════════════════════════════════════════════════════════════════
// BUILDER STATE
// ═══════════════════════════════════════════════════════════════════════════════

export interface BuilderState {
	layout: PageLayout;
	selectedBlockId: string | null;
	dragState: DragState;
	isPreviewMode: boolean;
	isSaving: boolean;
	hasUnsavedChanges: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CMS CONTENT TYPES (for type-safe content editing)
// ═══════════════════════════════════════════════════════════════════════════════

export type CmsContentType =
	| 'page'
	| 'blog_post'
	| 'alert_service'
	| 'trading_room'
	| 'indicator'
	| 'course'
	| 'lesson'
	| 'testimonial'
	| 'faq'
	| 'author'
	| 'topic_cluster'
	| 'weekly_watchlist'
	| 'resource';

export type CmsContentStatus =
	| 'draft'
	| 'in_review'
	| 'approved'
	| 'scheduled'
	| 'published'
	| 'archived';

/** CMS Content item */
export interface CmsContent {
	id: string;
	contentType: CmsContentType;
	slug: string;
	locale: string;
	title: string;
	subtitle?: string;
	excerpt?: string;
	content?: string;
	contentBlocks?: PageBlock[];
	featuredImageId?: string;
	featuredImageUrl?: string;
	metaTitle?: string;
	metaDescription?: string;
	status: CmsContentStatus;
	publishedAt?: string;
	scheduledPublishAt?: string;
	authorId?: string;
	authorName?: string;
	createdAt: string;
	updatedAt: string;
}
