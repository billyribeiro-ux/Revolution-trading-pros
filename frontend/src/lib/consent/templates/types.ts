/**
 * Banner Template System Types
 *
 * Comprehensive type definitions for the consent banner template system.
 *
 * @module consent/templates/types
 * @version 1.0.0
 */

/**
 * Template position on screen
 */
export type TemplatePosition =
	| 'bottom'
	| 'top'
	| 'bottom-left'
	| 'bottom-right'
	| 'top-left'
	| 'top-right'
	| 'center'
	| 'fullscreen';

/**
 * Template style variant
 */
export type TemplateStyle =
	| 'minimal'
	| 'modern'
	| 'classic'
	| 'glassmorphism'
	| 'neumorphism'
	| 'gradient'
	| 'flat'
	| 'outlined'
	| 'floating'
	| 'card';

/**
 * Button style variant
 */
export type ButtonVariant =
	| 'solid'
	| 'outline'
	| 'ghost'
	| 'gradient'
	| 'pill'
	| 'rounded'
	| 'square';

/**
 * Animation type
 */
export type AnimationType =
	| 'none'
	| 'fade'
	| 'slide-up'
	| 'slide-down'
	| 'slide-left'
	| 'slide-right'
	| 'scale'
	| 'bounce';

/**
 * Color scheme configuration
 */
export interface ColorScheme {
	/** Background color or gradient */
	background: string;
	/** Backdrop/overlay color */
	backdrop?: string;
	/** Primary text color */
	text: string;
	/** Secondary/muted text color */
	textMuted: string;
	/** Primary accent color */
	accent: string;
	/** Secondary accent color */
	accentSecondary?: string;
	/** Accept button background */
	acceptButton: string;
	/** Accept button text */
	acceptButtonText: string;
	/** Reject button background */
	rejectButton: string;
	/** Reject button text */
	rejectButtonText: string;
	/** Customize button background */
	customizeButton?: string;
	/** Customize button text */
	customizeButtonText?: string;
	/** Border color */
	border?: string;
	/** Shadow color */
	shadow?: string;
}

/**
 * Typography configuration
 */
export interface Typography {
	/** Font family */
	fontFamily: string;
	/** Title font size */
	titleSize: string;
	/** Title font weight */
	titleWeight: string;
	/** Description font size */
	descriptionSize: string;
	/** Button font size */
	buttonSize: string;
	/** Button font weight */
	buttonWeight: string;
	/** Line height */
	lineHeight: string;
}

/**
 * Spacing configuration
 */
export interface Spacing {
	/** Padding */
	padding: string;
	/** Gap between elements */
	gap: string;
	/** Button padding */
	buttonPadding: string;
	/** Border radius */
	borderRadius: string;
	/** Button border radius */
	buttonBorderRadius: string;
}

/**
 * Mobile-specific configuration
 */
export interface MobileConfig {
	/** Position on mobile */
	position: TemplatePosition;
	/** Use bottom sheet drawer */
	useDrawer: boolean;
	/** Stack buttons vertically */
	stackButtons: boolean;
	/** Compact mode */
	compact: boolean;
	/** Full width buttons */
	fullWidthButtons: boolean;
	/** Custom padding */
	padding?: string;
	/** Custom font sizes */
	fontSize?: string;
}

/**
 * Tablet-specific configuration
 */
export interface TabletConfig {
	/** Position on tablet */
	position: TemplatePosition;
	/** Max width */
	maxWidth: string;
	/** Stack buttons */
	stackButtons: boolean;
}

/**
 * Copy/text configuration
 */
export interface TemplateCopy {
	/** Banner title */
	title: string;
	/** Banner description */
	description: string;
	/** Accept all button text */
	acceptAll: string;
	/** Reject all button text */
	rejectAll: string;
	/** Customize/preferences button text */
	customize: string;
	/** Privacy policy link text */
	privacyPolicy?: string;
	/** Cookie policy link text */
	cookiePolicy?: string;
}

/**
 * Complete banner template configuration
 */
export interface BannerTemplate {
	/** Unique template ID */
	id: string;
	/** Template display name */
	name: string;
	/** Template description */
	description: string;
	/** Template category */
	category: 'minimal' | 'modern' | 'corporate' | 'playful' | 'dark' | 'light' | 'custom';
	/** Preview image URL */
	previewImage?: string;
	/** Is this a premium template */
	isPremium?: boolean;
	/** Is this template editable */
	isEditable: boolean;

	// Layout
	/** Position on desktop */
	position: TemplatePosition;
	/** Template style */
	style: TemplateStyle;
	/** Max width (CSS value) */
	maxWidth: string;
	/** Show close button */
	showCloseButton: boolean;
	/** Show privacy policy link */
	showPrivacyLink: boolean;
	/** Show cookie icon/illustration */
	showIcon: boolean;
	/** Icon type */
	iconType?: 'cookie' | 'shield' | 'lock' | 'checkmark' | 'custom';
	/** Custom icon SVG */
	customIcon?: string;

	// Buttons
	/** Button variant */
	buttonVariant: ButtonVariant;
	/** Show reject button */
	showRejectButton: boolean;
	/** Show customize button */
	showCustomizeButton: boolean;
	/** Button order */
	buttonOrder: ('accept' | 'reject' | 'customize')[];

	// Animation
	/** Entry animation */
	animation: AnimationType;
	/** Animation duration (ms) */
	animationDuration: number;

	// Colors
	/** Color scheme */
	colors: ColorScheme;

	// Typography
	/** Typography settings */
	typography: Typography;

	// Spacing
	/** Spacing settings */
	spacing: Spacing;

	// Copy
	/** Text content */
	copy: TemplateCopy;

	// Responsive
	/** Mobile configuration */
	mobile: MobileConfig;
	/** Tablet configuration */
	tablet: TabletConfig;

	// Advanced
	/** Custom CSS */
	customCSS?: string;
	/** Backdrop blur */
	backdropBlur?: string;
	/** Box shadow */
	boxShadow?: string;
	/** Border */
	border?: string;
}

/**
 * Template customization (user overrides)
 */
export interface TemplateCustomization {
	/** Base template ID */
	templateId: string;
	/** Custom name */
	name?: string;
	/** Color overrides */
	colors?: Partial<ColorScheme>;
	/** Typography overrides */
	typography?: Partial<Typography>;
	/** Spacing overrides */
	spacing?: Partial<Spacing>;
	/** Copy overrides */
	copy?: Partial<TemplateCopy>;
	/** Position override */
	position?: TemplatePosition;
	/** Custom CSS */
	customCSS?: string;
	/** Created at */
	createdAt: string;
	/** Updated at */
	updatedAt: string;
}

/**
 * Active template configuration (stored in backend)
 */
export interface ActiveTemplateConfig {
	/** Selected template ID */
	templateId: string;
	/** Customizations applied */
	customization?: TemplateCustomization;
	/** A/B test variants */
	abTestVariants?: {
		templateId: string;
		weight: number;
	}[];
	/** Last updated */
	updatedAt: string;
	/** Updated by user ID */
	updatedBy?: string;
}

/**
 * Template preview data
 */
export interface TemplatePreview {
	id: string;
	name: string;
	category: string;
	previewImage: string;
	description: string;
	isPremium: boolean;
}
