/**
 * Core Block Type Definitions
 * Complete type system for all blocks
 */

// Base block type
export type BlockType =
  // Content
  | 'paragraph'
  | 'heading'
  | 'quote'
  | 'pullquote'
  | 'code'
  | 'preformatted'
  | 'list'
  | 'checklist'
  // Media
  | 'image'
  | 'video'
  | 'audio'
  | 'gallery'
  | 'file'
  | 'embed'
  | 'gif'
  // Interactive
  | 'accordion'
  | 'tabs'
  | 'toggle'
  | 'toc'
  | 'buttons'
  // Layout
  | 'columns'
  | 'group'
  | 'row'
  | 'divider'
  // Trading
  | 'ticker'
  | 'chart'
  | 'priceAlert'
  | 'tradingIdea'
  // AI
  | 'aiGenerated'
  | 'aiSummary'
  | 'aiTranslation'
  // Advanced
  | 'card'
  | 'testimonial'
  | 'cta'
  | 'countdown'
  | 'socialShare'
  | 'author'
  | 'relatedPosts'
  | 'newsletter'
  | 'separator'
  | 'spacer'
  | 'html'
  | 'button'
  | 'callout'
  | 'riskDisclaimer'
  | 'shortcode'
  | 'reusable';

// Block content (varies by type)
export interface BlockContent {
  // Text content
  text?: string;
  html?: string;

  // Rich text
  title?: string;
  description?: string;

  // Media
  mediaUrl?: string;
  mediaAlt?: string;
  mediaCaption?: string;

  // Code
  code?: string;
  language?: string;

  // List
  listType?: 'bullet' | 'number';
  listItems?: string[];

  // Checklist
  items?: Array<{
    id: string;
    content: string;
    checked: boolean;
  }>;

  // Accordion
  accordionItems?: Array<{
    id: string;
    title: string;
    content: string;
  }>;

  // Tabs
  tabs?: Array<{
    id: string;
    label: string;
    content: string;
  }>;

  // Gallery
  galleryImages?: Array<{
    id?: string;
    url: string;
    alt: string;
    caption?: string;
  }>;

  // Columns
  columns?: Array<{
    blocks: Block[];
    width?: string;
  }>;

  // Group
  blocks?: Block[];

  // Embed
  embedUrl?: string;
  embedType?: 'youtube' | 'vimeo' | 'twitter' | 'instagram' | 'tiktok' | 'soundcloud' | 'spotify' | 'custom';

  // Trading
  symbol?: string;
  direction?: 'long' | 'short';
  entry?: number;
  stopLoss?: number;
  takeProfit?: number;
  confidence?: number;

  // Button
  buttonText?: string;
  buttonUrl?: string;
  buttonStyle?: 'primary' | 'secondary' | 'outline' | 'ghost';
  buttonSize?: 'small' | 'medium' | 'large';

  // CTA
  ctaHeading?: string;
  ctaDescription?: string;
  ctaPrimaryButton?: { text: string; url: string };
  ctaSecondaryButton?: { text: string; url: string };

  // Newsletter
  newsletterPlaceholder?: string;
  newsletterButtonText?: string;

  // Author
  authorName?: string;
  authorTitle?: string;
  authorBio?: string;
  authorPhoto?: string;
  authorSocials?: Array<{ platform: string; url: string }>;

  // Any other data
  [key: string]: any;
}

// Block settings
export interface BlockSettings {
  // Typography
  textAlign?: 'left' | 'center' | 'right' | 'justify';
  fontSize?: string;
  fontWeight?: string;
  fontFamily?: string;
  lineHeight?: string;
  textColor?: string;

  // Spacing
  marginTop?: string;
  marginBottom?: string;
  paddingTop?: string;
  paddingBottom?: string;
  paddingLeft?: string;
  paddingRight?: string;

  // Background
  backgroundColor?: string;
  backgroundImage?: string;

  // Border
  borderWidth?: string;
  borderColor?: string;
  borderStyle?: string;
  borderRadius?: string;

  // Effects
  boxShadow?: string;
  opacity?: number;

  // Layout
  maxWidth?: string;
  alignment?: 'left' | 'center' | 'right';

  // Block-specific
  level?: 1 | 2 | 3 | 4 | 5 | 6; // Heading level
  anchor?: string; // Heading anchor
  objectFit?: 'cover' | 'contain'; // Image fit
  galleryLayout?: 'grid' | 'masonry' | 'carousel';
  galleryColumns?: number;
  allowMultiple?: boolean; // Accordion
  defaultOpen?: boolean; // Toggle, Accordion
  iconStyle?: 'chevron' | 'plusminus'; // Accordion, Toggle
  style?: 'solid' | 'dashed' | 'dotted' | 'double'; // Divider
  width?: 'small' | 'medium' | 'large' | 'full'; // Divider
  color?: string; // Divider
  spacing?: 'small' | 'medium' | 'large'; // Divider, Spacer
  icon?: 'none' | 'minus' | 'star' | 'circle' | 'square'; // Divider
  padding?: 'none' | 'small' | 'medium' | 'large'; // Group
  targetDate?: string; // Countdown

  // Any other settings
  [key: string]: any;
}

// Block metadata
export interface BlockMetadata {
  createdAt?: number;
  updatedAt?: number;
  createdBy?: string;
  version?: number;
  locked?: boolean;
  [key: string]: any;
}

// Complete block interface
export interface Block {
  id: string;
  type: BlockType;
  content: BlockContent;
  settings: BlockSettings;
  metadata: BlockMetadata;
}

// Block creation payload
export type CreateBlockPayload = Omit<Block, 'id' | 'metadata'> & {
  metadata?: Partial<BlockMetadata>;
};

// Block update payload
export type UpdateBlockPayload = Partial<Omit<Block, 'id'>>;
