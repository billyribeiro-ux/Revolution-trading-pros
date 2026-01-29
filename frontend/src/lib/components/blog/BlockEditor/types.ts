/**
 * Revolution Trading Pros - Enterprise Block Editor Types
 * ═══════════════════════════════════════════════════════════════════════════
 * World-class blog editor type definitions - surpassing Elementor Pro
 *
 * @version 4.0.0 Enterprise
 * @author Revolution Trading Pros
 */

// =============================================================================
// Block Types - 30+ block types surpassing Elementor Pro
// =============================================================================

export type BlockType =
	// Text Blocks
	| 'paragraph'
	| 'heading'
	| 'quote'
	| 'pullquote'
	| 'code'
	| 'preformatted'
	| 'list'
	| 'checklist'
	// Media Blocks
	| 'image'
	| 'gallery'
	| 'video'
	| 'audio'
	| 'file'
	| 'embed'
	| 'gif'
	// Layout Blocks
	| 'columns'
	| 'group'
	| 'separator'
	| 'spacer'
	| 'row'
	// Interactive Blocks
	| 'button'
	| 'buttons'
	| 'accordion'
	| 'tabs'
	| 'toggle'
	| 'toc' // Table of Contents
	// Trading-Specific Blocks
	| 'ticker'
	| 'chart'
	| 'priceAlert'
	| 'tradingIdea'
	| 'riskDisclaimer'
	// Advanced Blocks
	| 'callout'
	| 'card'
	| 'testimonial'
	| 'cta'
	| 'countdown'
	| 'socialShare'
	| 'author'
	| 'relatedPosts'
	| 'newsletter'
	// AI-Powered Blocks
	| 'aiGenerated'
	| 'aiSummary'
	| 'aiTranslation'
	// Custom/Dynamic
	| 'shortcode'
	| 'html'
	| 'reusable';

// =============================================================================
// Block Interface
// =============================================================================

export interface Block {
	id: string;
	type: BlockType;
	content: BlockContent;
	settings: BlockSettings;
	metadata: BlockMetadata;
}

export interface BlockContent {
	// Text content
	text?: string;
	html?: string;

	// Media content
	mediaId?: number;
	mediaUrl?: string;
	mediaAlt?: string;
	mediaCaption?: string;

	// Nested content
	children?: Block[];
	items?: BlockItem[];

	// Special content
	embedUrl?: string;
	embedType?:
		| 'youtube'
		| 'vimeo'
		| 'twitter'
		| 'instagram'
		| 'tiktok'
		| 'soundcloud'
		| 'spotify'
		| 'custom';

	// Code content
	code?: string;
	language?: string;

	// List content
	listType?: 'bullet' | 'number' | 'check';
	listItems?: string[];

	// Columns/Layout
	columnCount?: number;
	columnLayout?: string;

	// Trading-specific
	ticker?: string;
	chartType?: 'line' | 'candle' | 'bar';
	priceTarget?: number;

	// AI content
	aiPrompt?: string;
	aiModel?: string;
	aiOutput?: string;
}

export interface BlockItem {
	id: string;
	content: string;
	checked?: boolean;
	collapsed?: boolean;
	children?: BlockItem[];
}

export interface BlockSettings {
	// Typography
	textAlign?: 'left' | 'center' | 'right' | 'justify';
	fontSize?: string;
	fontWeight?: string;
	fontFamily?: string;
	lineHeight?: string;
	letterSpacing?: string;
	textColor?: string;
	textTransform?: 'none' | 'uppercase' | 'lowercase' | 'capitalize';

	// Heading specific
	level?: 1 | 2 | 3 | 4 | 5 | 6;
	anchor?: string;

	// Spacing - shorthand
	padding?: string;
	margin?: string;

	// Spacing - individual
	marginTop?: string;
	marginBottom?: string;
	marginLeft?: string;
	marginRight?: string;
	paddingTop?: string;
	paddingBottom?: string;
	paddingLeft?: string;
	paddingRight?: string;

	// Background
	backgroundColor?: string;
	backgroundImage?: string;
	backgroundGradient?: string;
	backgroundOverlay?: string;

	// Border
	borderWidth?: string;
	borderColor?: string;
	borderStyle?: 'solid' | 'dashed' | 'dotted' | 'none';
	borderRadius?: string;

	// Shadow
	boxShadow?: string;

	// Animation
	animation?: AnimationSettings;
	animationDuration?: string;
	animationDelay?: string;

	// Transform
	rotate?: string;
	scale?: string;
	translateX?: string;
	translateY?: string;
	skewX?: string;
	skewY?: string;

	// Visual Effects
	opacity?: number;
	blendMode?:
		| 'normal'
		| 'multiply'
		| 'screen'
		| 'overlay'
		| 'darken'
		| 'lighten'
		| 'color-dodge'
		| 'color-burn'
		| 'hard-light'
		| 'soft-light'
		| 'difference'
		| 'exclusion'
		| 'hue'
		| 'saturation'
		| 'color'
		| 'luminosity';

	// Filters
	filterBlur?: string;
	filterBrightness?: string;
	filterContrast?: string;
	filterGrayscale?: string;
	filterSaturate?: string;

	// Layout
	width?: string;
	maxWidth?: string;
	height?: string;
	display?: 'block' | 'flex' | 'grid' | 'inline' | 'none';
	flexDirection?: 'row' | 'column';
	justifyContent?: string;
	alignItems?: string;
	gap?: string;
	columnCount?: number;
	columnLayout?: string;

	// Image specific
	imageSize?: 'thumbnail' | 'medium' | 'large' | 'full' | 'custom';
	imageWidth?: number;
	imageHeight?: number;
	objectFit?: 'cover' | 'contain' | 'fill' | 'none';
	focalPoint?: { x: number; y: number };
	imageFilter?: string;

	// Link
	linkUrl?: string;
	linkTarget?: '_blank' | '_self';
	linkRel?: string;

	// Button specific
	buttonStyle?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'link';
	buttonSize?: 'small' | 'medium' | 'large';
	buttonIcon?: string;
	buttonIconPosition?: 'left' | 'right';

	// Responsive visibility
	hideOnDesktop?: boolean;
	hideOnTablet?: boolean;
	hideOnMobile?: boolean;

	// Responsive typography
	tabletFontSize?: string;
	mobileFontSize?: string;

	// Responsive spacing
	tabletPadding?: string;
	mobilePadding?: string;

	// Advanced
	customCSS?: string;
	customId?: string;
	customClass?: string;
	ariaLabel?: string;
	position?: 'static' | 'relative' | 'absolute' | 'fixed' | 'sticky';
	zIndex?: number;
	positionTop?: string;
	positionRight?: string;
	positionBottom?: string;
	positionLeft?: string;
}

export interface AnimationSettings {
	type: 'none' | 'fade' | 'slide' | 'zoom' | 'bounce' | 'flip' | 'rotate';
	direction?: 'up' | 'down' | 'left' | 'right';
	duration?: number;
	delay?: number;
	easing?: string;
	triggerOnScroll?: boolean;
	repeat?: boolean;
}

export interface BlockMetadata {
	createdAt: string;
	updatedAt: string;
	createdBy?: string;
	version: number;
	locked?: boolean;
	lockedBy?: string;
	comments?: BlockComment[];
	aiGenerated?: boolean;
}

export interface BlockComment {
	id: string;
	userId: string;
	userName: string;
	userAvatar?: string;
	content: string;
	createdAt: string;
	resolved?: boolean;
	replies?: BlockComment[];
}

// =============================================================================
// Editor State
// =============================================================================

export interface EditorState {
	blocks: Block[];
	selectedBlockId: string | null;
	selectedBlockIds: string[]; // Multi-selection support
	hoveredBlockId: string | null;
	focusedBlockId: string | null;
	clipboard: Block | null;
	clipboardMulti: Block[]; // Multi-block clipboard
	history: HistoryState;
	isDragging: boolean;
	draggedBlockId: string | null;
	draggedBlockIds: string[]; // Multi-block drag support
	dropTargetIndex: number | null;
	dropPosition: 'before' | 'after' | null; // Enhanced drop positioning
	viewMode: 'edit' | 'preview';
	devicePreview: 'desktop' | 'tablet' | 'mobile';
	zoom: number;
	showGrid: boolean;
	showOutlines: boolean;
	sidebarTab: 'blocks' | 'settings' | 'layers' | 'ai' | 'seo';
	isFullscreen: boolean;
	autosaveEnabled: boolean;
	lastSaved: string | null;
	hasUnsavedChanges: boolean;
	// Enhanced drag-drop state
	dragPreviewOffset: { x: number; y: number } | null;
	lastDropAction: DropAction | null; // For undo after drop
}

// Enhanced drag-drop types
export interface DropAction {
	type: 'reorder' | 'multi-reorder';
	fromIndices: number[];
	toIndex: number;
	blockIds: string[];
	timestamp: number;
}

export interface HistoryState {
	past: Block[][];
	present: Block[];
	future: Block[][];
	maxSize: number;
}

// =============================================================================
// Block Categories
// =============================================================================

export interface BlockCategory {
	id: string;
	name: string;
	icon: string;
	blocks: BlockType[];
	color: string;
}

export const BLOCK_CATEGORIES: BlockCategory[] = [
	{
		id: 'text',
		name: 'Text',
		icon: 'text',
		color: '#3b82f6',
		blocks: [
			'paragraph',
			'heading',
			'quote',
			'pullquote',
			'code',
			'preformatted',
			'list',
			'checklist'
		]
	},
	{
		id: 'media',
		name: 'Media',
		icon: 'photo',
		color: '#10b981',
		blocks: ['image', 'gallery', 'video', 'audio', 'file', 'embed', 'gif']
	},
	{
		id: 'layout',
		name: 'Layout',
		icon: 'layout',
		color: '#8b5cf6',
		blocks: ['columns', 'group', 'separator', 'spacer', 'row']
	},
	{
		id: 'interactive',
		name: 'Interactive',
		icon: 'click',
		color: '#f59e0b',
		blocks: ['button', 'buttons', 'accordion', 'tabs', 'toggle', 'toc']
	},
	{
		id: 'trading',
		name: 'Trading',
		icon: 'chart-line',
		color: '#ef4444',
		blocks: ['ticker', 'chart', 'priceAlert', 'tradingIdea', 'riskDisclaimer']
	},
	{
		id: 'advanced',
		name: 'Advanced',
		icon: 'sparkles',
		color: '#ec4899',
		blocks: [
			'callout',
			'card',
			'testimonial',
			'cta',
			'countdown',
			'socialShare',
			'author',
			'relatedPosts',
			'newsletter'
		]
	},
	{
		id: 'ai',
		name: 'AI Powered',
		icon: 'robot',
		color: '#06b6d4',
		blocks: ['aiGenerated', 'aiSummary', 'aiTranslation']
	},
	{
		id: 'custom',
		name: 'Custom',
		icon: 'code',
		color: '#64748b',
		blocks: ['shortcode', 'html', 'reusable']
	}
];

// =============================================================================
// Block Definitions
// =============================================================================

export interface BlockDefinition {
	type: BlockType;
	name: string;
	description: string;
	icon: string;
	category: string;
	keywords: string[];
	supports: BlockSupports;
	defaultContent: Partial<BlockContent>;
	defaultSettings: Partial<BlockSettings>;
}

export interface BlockSupports {
	align?: boolean;
	color?: boolean;
	typography?: boolean;
	spacing?: boolean;
	border?: boolean;
	shadow?: boolean;
	animation?: boolean;
	responsive?: boolean;
	customCSS?: boolean;
	anchor?: boolean;
	html?: boolean;
	reusable?: boolean;
	nested?: boolean;
	lock?: boolean;
}

export const BLOCK_DEFINITIONS: Record<BlockType, BlockDefinition> = {
	paragraph: {
		type: 'paragraph',
		name: 'Paragraph',
		description: 'A simple text paragraph',
		icon: 'align-left',
		category: 'text',
		keywords: ['text', 'content', 'body'],
		supports: { align: true, color: true, typography: true, spacing: true, customCSS: true },
		defaultContent: { text: '' },
		defaultSettings: { textAlign: 'left' }
	},
	heading: {
		type: 'heading',
		name: 'Heading',
		description: 'A heading for sections',
		icon: 'h-1',
		category: 'text',
		keywords: ['title', 'header', 'h1', 'h2', 'h3'],
		supports: {
			align: true,
			color: true,
			typography: true,
			spacing: true,
			anchor: true,
			customCSS: true
		},
		defaultContent: { text: '' },
		defaultSettings: { level: 2, textAlign: 'left' }
	},
	quote: {
		type: 'quote',
		name: 'Quote',
		description: 'A blockquote for citations',
		icon: 'quote',
		category: 'text',
		keywords: ['blockquote', 'citation', 'cite'],
		supports: {
			align: true,
			color: true,
			typography: true,
			spacing: true,
			border: true,
			customCSS: true
		},
		defaultContent: { text: '', html: '' },
		defaultSettings: { borderColor: '#3b82f6' }
	},
	pullquote: {
		type: 'pullquote',
		name: 'Pull Quote',
		description: 'A highlighted quote',
		icon: 'blockquote',
		category: 'text',
		keywords: ['featured', 'highlight', 'callout'],
		supports: { align: true, color: true, typography: true, spacing: true, customCSS: true },
		defaultContent: { text: '' },
		defaultSettings: { textAlign: 'center', fontSize: '1.5rem' }
	},
	code: {
		type: 'code',
		name: 'Code',
		description: 'A code snippet with syntax highlighting',
		icon: 'code',
		category: 'text',
		keywords: ['snippet', 'programming', 'syntax'],
		supports: { spacing: true, customCSS: true },
		defaultContent: { code: '', language: 'javascript' },
		defaultSettings: {}
	},
	preformatted: {
		type: 'preformatted',
		name: 'Preformatted',
		description: 'Preformatted text preserving spacing',
		icon: 'file-code',
		category: 'text',
		keywords: ['pre', 'monospace', 'fixed'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: { text: '' },
		defaultSettings: {}
	},
	list: {
		type: 'list',
		name: 'List',
		description: 'Bulleted or numbered list',
		icon: 'list',
		category: 'text',
		keywords: ['bullet', 'number', 'ordered', 'unordered'],
		supports: { color: true, typography: true, spacing: true, customCSS: true },
		defaultContent: { listType: 'bullet', listItems: [''] },
		defaultSettings: {}
	},
	checklist: {
		type: 'checklist',
		name: 'Checklist',
		description: 'Interactive checklist',
		icon: 'checklist',
		category: 'text',
		keywords: ['todo', 'tasks', 'checkbox'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: { items: [] },
		defaultSettings: {}
	},
	image: {
		type: 'image',
		name: 'Image',
		description: 'Insert an image',
		icon: 'photo',
		category: 'media',
		keywords: ['picture', 'photo', 'media'],
		supports: {
			align: true,
			spacing: true,
			border: true,
			shadow: true,
			animation: true,
			customCSS: true
		},
		defaultContent: {},
		defaultSettings: { imageSize: 'large' }
	},
	gallery: {
		type: 'gallery',
		name: 'Gallery',
		description: 'Multiple images in a grid',
		icon: 'photo-album',
		category: 'media',
		keywords: ['images', 'photos', 'grid', 'masonry'],
		supports: { spacing: true, customCSS: true },
		defaultContent: { items: [] },
		defaultSettings: { columnCount: 3 }
	},
	video: {
		type: 'video',
		name: 'Video',
		description: 'Embed a video',
		icon: 'video',
		category: 'media',
		keywords: ['movie', 'film', 'youtube', 'vimeo'],
		supports: { align: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	audio: {
		type: 'audio',
		name: 'Audio',
		description: 'Audio player',
		icon: 'volume',
		category: 'media',
		keywords: ['music', 'sound', 'podcast'],
		supports: { spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	file: {
		type: 'file',
		name: 'File',
		description: 'A downloadable file',
		icon: 'file-download',
		category: 'media',
		keywords: ['download', 'attachment', 'pdf'],
		supports: { spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	embed: {
		type: 'embed',
		name: 'Embed',
		description: 'Embed external content',
		icon: 'brand-youtube',
		category: 'media',
		keywords: ['iframe', 'widget', 'external'],
		supports: { align: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	gif: {
		type: 'gif',
		name: 'GIF',
		description: 'Animated GIF from Giphy',
		icon: 'gif',
		category: 'media',
		keywords: ['animation', 'giphy', 'meme'],
		supports: { align: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	columns: {
		type: 'columns',
		name: 'Columns',
		description: 'Multi-column layout',
		icon: 'columns',
		category: 'layout',
		keywords: ['grid', 'row', 'layout'],
		supports: { spacing: true, responsive: true, customCSS: true, nested: true },
		defaultContent: { columnCount: 2, children: [] },
		defaultSettings: { gap: '2rem' }
	},
	group: {
		type: 'group',
		name: 'Group',
		description: 'Group blocks together',
		icon: 'box',
		category: 'layout',
		keywords: ['container', 'wrapper', 'section'],
		supports: {
			color: true,
			spacing: true,
			border: true,
			shadow: true,
			customCSS: true,
			nested: true
		},
		defaultContent: { children: [] },
		defaultSettings: {}
	},
	separator: {
		type: 'separator',
		name: 'Separator',
		description: 'Horizontal line separator',
		icon: 'minus',
		category: 'layout',
		keywords: ['divider', 'hr', 'line'],
		supports: { spacing: true, color: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	spacer: {
		type: 'spacer',
		name: 'Spacer',
		description: 'Add vertical space',
		icon: 'spacing-vertical',
		category: 'layout',
		keywords: ['space', 'gap', 'margin'],
		supports: { responsive: true },
		defaultContent: {},
		defaultSettings: { height: '40px' }
	},
	row: {
		type: 'row',
		name: 'Row',
		description: 'Horizontal block arrangement',
		icon: 'layout-rows',
		category: 'layout',
		keywords: ['horizontal', 'flex', 'inline'],
		supports: { spacing: true, responsive: true, customCSS: true, nested: true },
		defaultContent: { children: [] },
		defaultSettings: { display: 'flex', gap: '1rem' }
	},
	button: {
		type: 'button',
		name: 'Button',
		description: 'Call-to-action button',
		icon: 'click',
		category: 'interactive',
		keywords: ['cta', 'link', 'action'],
		supports: {
			align: true,
			color: true,
			typography: true,
			spacing: true,
			border: true,
			shadow: true,
			animation: true,
			customCSS: true
		},
		defaultContent: { text: 'Click Here' },
		defaultSettings: { buttonStyle: 'primary', buttonSize: 'medium' }
	},
	buttons: {
		type: 'buttons',
		name: 'Button Group',
		description: 'Multiple buttons in a row',
		icon: 'apps',
		category: 'interactive',
		keywords: ['cta', 'actions', 'group'],
		supports: { align: true, spacing: true, customCSS: true, nested: true },
		defaultContent: { children: [] },
		defaultSettings: { gap: '0.5rem' }
	},
	accordion: {
		type: 'accordion',
		name: 'Accordion',
		description: 'Collapsible content sections',
		icon: 'layout-navbar-collapse',
		category: 'interactive',
		keywords: ['collapse', 'expand', 'faq'],
		supports: { color: true, spacing: true, border: true, customCSS: true },
		defaultContent: { items: [] },
		defaultSettings: {}
	},
	tabs: {
		type: 'tabs',
		name: 'Tabs',
		description: 'Tabbed content sections',
		icon: 'layout-distribute-horizontal',
		category: 'interactive',
		keywords: ['tab', 'switch', 'panel'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: { items: [] },
		defaultSettings: {}
	},
	toggle: {
		type: 'toggle',
		name: 'Toggle',
		description: 'Show/hide content toggle',
		icon: 'toggle-left',
		category: 'interactive',
		keywords: ['switch', 'spoiler', 'reveal'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: { children: [] },
		defaultSettings: {}
	},
	toc: {
		type: 'toc',
		name: 'Table of Contents',
		description: 'Auto-generated table of contents',
		icon: 'list-tree',
		category: 'interactive',
		keywords: ['navigation', 'headings', 'outline'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	ticker: {
		type: 'ticker',
		name: 'Stock Ticker',
		description: 'Real-time stock price display',
		icon: 'trending-up',
		category: 'trading',
		keywords: ['stock', 'price', 'market'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: { ticker: 'SPY' },
		defaultSettings: {}
	},
	chart: {
		type: 'chart',
		name: 'Trading Chart',
		description: 'Interactive trading chart',
		icon: 'chart-candle',
		category: 'trading',
		keywords: ['candlestick', 'graph', 'technical'],
		supports: { spacing: true, customCSS: true },
		defaultContent: { ticker: 'SPY', chartType: 'candle' },
		defaultSettings: { height: '400px' }
	},
	priceAlert: {
		type: 'priceAlert',
		name: 'Price Alert',
		description: 'Price target notification',
		icon: 'alert-circle',
		category: 'trading',
		keywords: ['target', 'alert', 'notification'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	tradingIdea: {
		type: 'tradingIdea',
		name: 'Trading Idea',
		description: 'Structured trade idea card',
		icon: 'bulb',
		category: 'trading',
		keywords: ['setup', 'trade', 'analysis'],
		supports: { color: true, spacing: true, border: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	riskDisclaimer: {
		type: 'riskDisclaimer',
		name: 'Risk Disclaimer',
		description: 'Trading risk warning',
		icon: 'alert-triangle',
		category: 'trading',
		keywords: ['warning', 'disclaimer', 'legal'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: {
			text: 'Trading involves substantial risk of loss and is not suitable for all investors.'
		},
		defaultSettings: { backgroundColor: '#fef3c7', borderColor: '#f59e0b' }
	},
	callout: {
		type: 'callout',
		name: 'Callout',
		description: 'Highlighted information box',
		icon: 'info-circle',
		category: 'advanced',
		keywords: ['info', 'notice', 'alert', 'warning'],
		supports: { color: true, spacing: true, border: true, customCSS: true },
		defaultContent: { text: '' },
		defaultSettings: {}
	},
	card: {
		type: 'card',
		name: 'Card',
		description: 'Content card with image',
		icon: 'id',
		category: 'advanced',
		keywords: ['box', 'container', 'feature'],
		supports: {
			color: true,
			spacing: true,
			border: true,
			shadow: true,
			customCSS: true,
			nested: true
		},
		defaultContent: { children: [] },
		defaultSettings: {}
	},
	testimonial: {
		type: 'testimonial',
		name: 'Testimonial',
		description: 'Customer testimonial',
		icon: 'message-circle',
		category: 'advanced',
		keywords: ['review', 'quote', 'customer'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	cta: {
		type: 'cta',
		name: 'Call to Action',
		description: 'CTA section with button',
		icon: 'speakerphone',
		category: 'advanced',
		keywords: ['action', 'conversion', 'signup'],
		supports: { color: true, spacing: true, customCSS: true, nested: true },
		defaultContent: { children: [] },
		defaultSettings: {}
	},
	countdown: {
		type: 'countdown',
		name: 'Countdown',
		description: 'Countdown timer',
		icon: 'clock',
		category: 'advanced',
		keywords: ['timer', 'deadline', 'urgency'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	socialShare: {
		type: 'socialShare',
		name: 'Social Share',
		description: 'Social media share buttons',
		icon: 'share',
		category: 'advanced',
		keywords: ['twitter', 'facebook', 'linkedin', 'share'],
		supports: { align: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	author: {
		type: 'author',
		name: 'Author Box',
		description: 'Author bio and info',
		icon: 'user',
		category: 'advanced',
		keywords: ['bio', 'writer', 'profile'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	relatedPosts: {
		type: 'relatedPosts',
		name: 'Related Posts',
		description: 'Related articles grid',
		icon: 'article',
		category: 'advanced',
		keywords: ['suggestions', 'similar', 'articles'],
		supports: { spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: { columnCount: 3 }
	},
	newsletter: {
		type: 'newsletter',
		name: 'Newsletter',
		description: 'Email signup form',
		icon: 'mail',
		category: 'advanced',
		keywords: ['email', 'subscribe', 'signup'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	aiGenerated: {
		type: 'aiGenerated',
		name: 'AI Content',
		description: 'AI-generated content block',
		icon: 'robot',
		category: 'ai',
		keywords: ['artificial', 'intelligence', 'generate'],
		supports: { align: true, color: true, typography: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	aiSummary: {
		type: 'aiSummary',
		name: 'AI Summary',
		description: 'Auto-generated content summary',
		icon: 'file-description',
		category: 'ai',
		keywords: ['tldr', 'abstract', 'brief'],
		supports: { color: true, spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	aiTranslation: {
		type: 'aiTranslation',
		name: 'AI Translation',
		description: 'Multi-language translation',
		icon: 'language',
		category: 'ai',
		keywords: ['translate', 'language', 'localize'],
		supports: { spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	shortcode: {
		type: 'shortcode',
		name: 'Shortcode',
		description: 'Custom shortcode',
		icon: 'brackets',
		category: 'custom',
		keywords: ['custom', 'dynamic', 'embed'],
		supports: { spacing: true, customCSS: true },
		defaultContent: {},
		defaultSettings: {}
	},
	html: {
		type: 'html',
		name: 'Custom HTML',
		description: 'Raw HTML code',
		icon: 'code',
		category: 'custom',
		keywords: ['raw', 'custom', 'embed'],
		supports: { spacing: true },
		defaultContent: { html: '' },
		defaultSettings: {}
	},
	reusable: {
		type: 'reusable',
		name: 'Reusable Block',
		description: 'Saved reusable block',
		icon: 'template',
		category: 'custom',
		keywords: ['saved', 'template', 'pattern'],
		supports: {},
		defaultContent: {},
		defaultSettings: {}
	}
};

// =============================================================================
// SEO Types
// =============================================================================

export interface SEOAnalysis {
	score: number;
	grade?: 'A' | 'B' | 'C' | 'D' | 'F';
	issues: SEOIssue[];
	suggestions: string[];
	keywordDensity: number;
	readabilityScore: string;
	readabilityGrade: number;
	wordCount: number;
	estimatedReadTime?: number;
	readingTime: number;
	titleScore?: number;
	metaScore?: number;
	contentScore?: number;
	headingStructure?: HeadingNode[];
	linksCount?: { internal: number; external: number };
	imagesWithoutAlt?: number;
}

export interface SEOIssue {
	type: 'error' | 'warning' | 'info' | 'success';
	category:
		| 'title'
		| 'description'
		| 'content'
		| 'headings'
		| 'images'
		| 'links'
		| 'keywords'
		| 'meta'
		| 'readability'
		| 'keyword'
		| 'slug'
		| 'structure';
	message: string;
	impact?: 'high' | 'medium' | 'low';
}

export interface HeadingNode {
	level: number;
	text: string;
	id: string;
	children: HeadingNode[];
}

// =============================================================================
// AI Types
// =============================================================================

export interface AIWritingRequest {
	prompt: string;
	context?: string;
	tone?: 'professional' | 'casual' | 'formal' | 'friendly' | 'persuasive';
	length?: 'short' | 'medium' | 'long';
	style?: 'blog' | 'news' | 'tutorial' | 'listicle' | 'review';
	keywords?: string[];
	language?: string;
}

export interface AIWritingResponse {
	content: string;
	suggestions?: string[];
	keywords?: string[];
	estimatedReadTime?: number;
}

export interface AIImageRequest {
	prompt: string;
	style?: 'realistic' | 'artistic' | 'cartoon' | 'abstract' | 'minimalist';
	aspectRatio?: '1:1' | '16:9' | '4:3' | '3:2' | '9:16';
	count?: number;
}

// =============================================================================
// Revision Types
// =============================================================================

export interface Revision {
	id: string;
	postId: string;
	blocks: Block[];
	createdAt: string;
	createdBy: string;
	createdByName: string;
	type: 'auto' | 'manual' | 'publish';
	title?: string;
	description?: string;
	author?: string;
	isAutosave?: boolean;
	wordCount: number;
	blockCount: number;
}

export interface RevisionDiff {
	added: Block[];
	removed: Block[];
	modified: { old: Block; new: Block }[];
}

// =============================================================================
// Export Types
// =============================================================================

export interface ExportOptions {
	format: 'html' | 'markdown' | 'json' | 'docx' | 'pdf';
	includeStyles?: boolean;
	includeImages?: boolean;
	optimizeImages?: boolean;
}
