/**
 * Enterprise Popup Branding API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Full customization API for enterprise branding:
 * - Logo insertion with multiple positions
 * - Custom font families and typography
 * - Brand color schemes
 * - Theme presets (light, dark, glass, custom)
 * - White-label support
 * - Custom CSS injection
 * - Responsive design tokens
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @license MIT
 */

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface LogoConfig {
	/** Logo image URL */
	url: string;
	/** Alt text for accessibility */
	alt: string;
	/** Position within popup */
	position: 'top-left' | 'top-center' | 'top-right' | 'bottom-left' | 'bottom-center' | 'bottom-right' | 'inline-title';
	/** Logo size */
	size: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | 'custom';
	/** Custom width (when size is 'custom') */
	width?: string;
	/** Custom height (when size is 'custom') */
	height?: string;
	/** Max width constraint */
	maxWidth?: string;
	/** Padding around logo */
	padding?: string;
	/** Click action URL */
	linkUrl?: string;
	/** Open link in new tab */
	linkNewTab?: boolean;
}

export interface FontConfig {
	/** Primary font family */
	primary: string;
	/** Secondary/heading font family */
	secondary?: string;
	/** Monospace font for code */
	monospace?: string;
	/** Font weights to load */
	weights: number[];
	/** Google Fonts URL or custom font source */
	source?: 'google' | 'custom' | 'system';
	/** Custom font URL (for source='custom') */
	customUrl?: string;
}

export interface TypographyConfig {
	/** Base font size */
	baseFontSize: string;
	/** Line height multiplier */
	lineHeight: number;
	/** Letter spacing */
	letterSpacing: string;
	/** Title font size */
	titleSize: string;
	/** Title font weight */
	titleWeight: number;
	/** Title letter spacing */
	titleLetterSpacing?: string;
	/** Subtitle font size */
	subtitleSize: string;
	/** Body text size */
	bodySize: string;
	/** Small text size */
	smallSize: string;
}

export interface ColorPalette {
	/** Primary brand color */
	primary: string;
	/** Primary hover state */
	primaryHover: string;
	/** Primary active state */
	primaryActive: string;
	/** Secondary brand color */
	secondary: string;
	/** Secondary hover state */
	secondaryHover: string;
	/** Accent color */
	accent: string;
	/** Success color */
	success: string;
	/** Warning color */
	warning: string;
	/** Error/danger color */
	error: string;
	/** Background color */
	background: string;
	/** Surface/card background */
	surface: string;
	/** Text primary color */
	text: string;
	/** Text secondary/muted color */
	textMuted: string;
	/** Border color */
	border: string;
	/** Overlay color */
	overlay: string;
}

export interface ThemePreset {
	name: string;
	colors: ColorPalette;
	borderRadius: string;
	shadow: string;
	backdropBlur: string;
}

export interface BrandingConfig {
	/** Logo configuration */
	logo?: LogoConfig;
	/** Secondary/badge logo */
	secondaryLogo?: LogoConfig;
	/** Favicon override */
	favicon?: string;
	/** Font configuration */
	fonts: FontConfig;
	/** Typography settings */
	typography: TypographyConfig;
	/** Color palette */
	colors: ColorPalette;
	/** Theme preset name */
	theme: 'light' | 'dark' | 'glass' | 'minimal' | 'custom';
	/** Custom CSS to inject */
	customCSS?: string;
	/** CSS variables to override */
	cssVariables?: Record<string, string>;
	/** Border radius scale */
	borderRadius: {
		sm: string;
		md: string;
		lg: string;
		xl: string;
		full: string;
	};
	/** Shadow scale */
	shadows: {
		sm: string;
		md: string;
		lg: string;
		xl: string;
	};
	/** Animation preferences */
	animations: {
		enabled: boolean;
		duration: string;
		easing: string;
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Default Configurations
// ═══════════════════════════════════════════════════════════════════════════

export const DEFAULT_FONTS: FontConfig = {
	primary: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
	secondary: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
	monospace: 'JetBrains Mono, Menlo, Monaco, "Courier New", monospace',
	weights: [400, 500, 600, 700],
	source: 'google',
};

export const DEFAULT_TYPOGRAPHY: TypographyConfig = {
	baseFontSize: '16px',
	lineHeight: 1.6,
	letterSpacing: '-0.01em',
	titleSize: '1.875rem',
	titleWeight: 700,
	titleLetterSpacing: '-0.02em',
	subtitleSize: '1.25rem',
	bodySize: '1rem',
	smallSize: '0.875rem',
};

export const THEME_PRESETS: Record<string, ThemePreset> = {
	light: {
		name: 'Light',
		colors: {
			primary: '#6366f1',
			primaryHover: '#4f46e5',
			primaryActive: '#4338ca',
			secondary: '#8b5cf6',
			secondaryHover: '#7c3aed',
			accent: '#ec4899',
			success: '#10b981',
			warning: '#f59e0b',
			error: '#ef4444',
			background: '#ffffff',
			surface: '#f8fafc',
			text: '#1e293b',
			textMuted: '#64748b',
			border: '#e2e8f0',
			overlay: 'rgba(0, 0, 0, 0.5)',
		},
		borderRadius: '16px',
		shadow: '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
		backdropBlur: '0',
	},
	dark: {
		name: 'Dark',
		colors: {
			primary: '#818cf8',
			primaryHover: '#6366f1',
			primaryActive: '#4f46e5',
			secondary: '#a78bfa',
			secondaryHover: '#8b5cf6',
			accent: '#f472b6',
			success: '#34d399',
			warning: '#fbbf24',
			error: '#f87171',
			background: '#0f172a',
			surface: '#1e293b',
			text: '#f1f5f9',
			textMuted: '#94a3b8',
			border: '#334155',
			overlay: 'rgba(0, 0, 0, 0.75)',
		},
		borderRadius: '16px',
		shadow: '0 25px 50px -12px rgba(0, 0, 0, 0.5)',
		backdropBlur: '16px',
	},
	glass: {
		name: 'Glass',
		colors: {
			primary: '#6366f1',
			primaryHover: '#4f46e5',
			primaryActive: '#4338ca',
			secondary: '#8b5cf6',
			secondaryHover: '#7c3aed',
			accent: '#ec4899',
			success: '#10b981',
			warning: '#f59e0b',
			error: '#ef4444',
			background: 'rgba(15, 23, 42, 0.8)',
			surface: 'rgba(30, 41, 59, 0.6)',
			text: '#f1f5f9',
			textMuted: '#94a3b8',
			border: 'rgba(99, 102, 241, 0.3)',
			overlay: 'rgba(0, 0, 0, 0.6)',
		},
		borderRadius: '24px',
		shadow: '0 25px 50px -12px rgba(99, 102, 241, 0.25)',
		backdropBlur: '24px',
	},
	minimal: {
		name: 'Minimal',
		colors: {
			primary: '#000000',
			primaryHover: '#1f2937',
			primaryActive: '#374151',
			secondary: '#6b7280',
			secondaryHover: '#4b5563',
			accent: '#3b82f6',
			success: '#22c55e',
			warning: '#eab308',
			error: '#dc2626',
			background: '#ffffff',
			surface: '#ffffff',
			text: '#000000',
			textMuted: '#6b7280',
			border: '#e5e7eb',
			overlay: 'rgba(0, 0, 0, 0.4)',
		},
		borderRadius: '8px',
		shadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
		backdropBlur: '0',
	},
};

export const DEFAULT_BRANDING: BrandingConfig = {
	fonts: DEFAULT_FONTS,
	typography: DEFAULT_TYPOGRAPHY,
	colors: THEME_PRESETS.dark.colors,
	theme: 'dark',
	borderRadius: {
		sm: '4px',
		md: '8px',
		lg: '16px',
		xl: '24px',
		full: '9999px',
	},
	shadows: {
		sm: '0 1px 2px 0 rgba(0, 0, 0, 0.05)',
		md: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
		lg: '0 10px 15px -3px rgba(0, 0, 0, 0.1)',
		xl: '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
	},
	animations: {
		enabled: true,
		duration: '300ms',
		easing: 'cubic-bezier(0.4, 0, 0.2, 1)',
	},
};

// ═══════════════════════════════════════════════════════════════════════════
// Branding Service
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Enterprise Branding Service for popup customization.
 */
export class PopupBrandingService {
	private config: BrandingConfig;
	private styleElement: HTMLStyleElement | null = null;

	constructor(config: Partial<BrandingConfig> = {}) {
		this.config = this.mergeConfig(config);
	}

	/**
	 * Merge provided config with defaults.
	 */
	private mergeConfig(config: Partial<BrandingConfig>): BrandingConfig {
		return {
			...DEFAULT_BRANDING,
			...config,
			fonts: { ...DEFAULT_FONTS, ...config.fonts },
			typography: { ...DEFAULT_TYPOGRAPHY, ...config.typography },
			colors: { ...DEFAULT_BRANDING.colors, ...config.colors },
			borderRadius: { ...DEFAULT_BRANDING.borderRadius, ...config.borderRadius },
			shadows: { ...DEFAULT_BRANDING.shadows, ...config.shadows },
			animations: { ...DEFAULT_BRANDING.animations, ...config.animations },
		};
	}

	/**
	 * Apply branding to document.
	 */
	applyBranding(): void {
		if (typeof document === 'undefined') return;

		// Inject fonts
		this.injectFonts();

		// Generate and inject CSS
		const css = this.generateCSS();
		this.injectCSS(css);
	}

	/**
	 * Update branding configuration.
	 */
	updateConfig(config: Partial<BrandingConfig>): void {
		this.config = this.mergeConfig(config);
		this.applyBranding();
	}

	/**
	 * Apply a theme preset.
	 */
	applyTheme(themeName: keyof typeof THEME_PRESETS): void {
		const preset = THEME_PRESETS[themeName];
		if (!preset) return;

		this.config.colors = preset.colors;
		this.config.theme = themeName as BrandingConfig['theme'];
		this.applyBranding();
	}

	/**
	 * Get logo HTML for insertion.
	 */
	getLogoHTML(config?: LogoConfig): string {
		const logo = config || this.config.logo;
		if (!logo) return '';

		const sizeMap = {
			xs: '24px',
			sm: '32px',
			md: '48px',
			lg: '64px',
			xl: '96px',
		};

		const width = logo.size === 'custom' ? logo.width : sizeMap[logo.size] || '48px';
		const height = logo.size === 'custom' ? logo.height : 'auto';

		const imgHTML = `
			<img
				src="${logo.url}"
				alt="${logo.alt}"
				style="width: ${width}; height: ${height}; max-width: ${logo.maxWidth || '100%'}; object-fit: contain;"
				loading="lazy"
			/>
		`;

		if (logo.linkUrl) {
			return `<a href="${logo.linkUrl}" ${logo.linkNewTab ? 'target="_blank" rel="noopener noreferrer"' : ''} style="display: inline-block; padding: ${logo.padding || '0'};">${imgHTML}</a>`;
		}

		return `<div style="padding: ${logo.padding || '0'};">${imgHTML}</div>`;
	}

	/**
	 * Get CSS variables object for inline styles.
	 */
	getCSSVariables(): Record<string, string> {
		const { colors, typography, borderRadius, shadows, animations } = this.config;

		return {
			'--popup-color-primary': colors.primary,
			'--popup-color-primary-hover': colors.primaryHover,
			'--popup-color-secondary': colors.secondary,
			'--popup-color-accent': colors.accent,
			'--popup-color-success': colors.success,
			'--popup-color-warning': colors.warning,
			'--popup-color-error': colors.error,
			'--popup-color-background': colors.background,
			'--popup-color-surface': colors.surface,
			'--popup-color-text': colors.text,
			'--popup-color-text-muted': colors.textMuted,
			'--popup-color-border': colors.border,
			'--popup-color-overlay': colors.overlay,
			'--popup-font-primary': this.config.fonts.primary,
			'--popup-font-secondary': this.config.fonts.secondary || this.config.fonts.primary,
			'--popup-font-size-base': typography.baseFontSize,
			'--popup-line-height': typography.lineHeight.toString(),
			'--popup-title-size': typography.titleSize,
			'--popup-title-weight': typography.titleWeight.toString(),
			'--popup-border-radius-sm': borderRadius.sm,
			'--popup-border-radius-md': borderRadius.md,
			'--popup-border-radius-lg': borderRadius.lg,
			'--popup-border-radius-xl': borderRadius.xl,
			'--popup-shadow-sm': shadows.sm,
			'--popup-shadow-md': shadows.md,
			'--popup-shadow-lg': shadows.lg,
			'--popup-shadow-xl': shadows.xl,
			'--popup-animation-duration': animations.duration,
			'--popup-animation-easing': animations.easing,
			...this.config.cssVariables,
		};
	}

	/**
	 * Generate popup-specific CSS.
	 */
	private generateCSS(): string {
		const vars = this.getCSSVariables();
		const varDeclarations = Object.entries(vars)
			.map(([key, value]) => `${key}: ${value};`)
			.join('\n  ');

		let css = `
/* Enterprise Popup Branding - Auto-generated */
:root {
  ${varDeclarations}
}

.popup-branded {
  font-family: var(--popup-font-primary);
  font-size: var(--popup-font-size-base);
  line-height: var(--popup-line-height);
  color: var(--popup-color-text);
  background-color: var(--popup-color-background);
}

.popup-branded .popup-title {
  font-family: var(--popup-font-secondary);
  font-size: var(--popup-title-size);
  font-weight: var(--popup-title-weight);
  letter-spacing: ${this.config.typography.titleLetterSpacing || '-0.02em'};
  color: var(--popup-color-text);
}

.popup-branded .popup-button-primary {
  background-color: var(--popup-color-primary);
  color: white;
  border: none;
  border-radius: var(--popup-border-radius-md);
  transition: all var(--popup-animation-duration) var(--popup-animation-easing);
}

.popup-branded .popup-button-primary:hover {
  background-color: var(--popup-color-primary-hover);
  transform: translateY(-2px);
  box-shadow: var(--popup-shadow-lg);
}

.popup-branded .popup-button-secondary {
  background-color: transparent;
  color: var(--popup-color-text);
  border: 1px solid var(--popup-color-border);
  border-radius: var(--popup-border-radius-md);
  transition: all var(--popup-animation-duration) var(--popup-animation-easing);
}

.popup-branded .popup-button-secondary:hover {
  background-color: var(--popup-color-surface);
  border-color: var(--popup-color-primary);
}

.popup-branded .popup-logo {
  margin-bottom: 1rem;
}

.popup-branded .popup-logo-top-left { position: absolute; top: 1rem; left: 1rem; }
.popup-branded .popup-logo-top-center { display: flex; justify-content: center; margin-bottom: 1rem; }
.popup-branded .popup-logo-top-right { position: absolute; top: 1rem; right: 1rem; }
.popup-branded .popup-logo-bottom-left { position: absolute; bottom: 1rem; left: 1rem; }
.popup-branded .popup-logo-bottom-center { display: flex; justify-content: center; margin-top: 1rem; }
.popup-branded .popup-logo-bottom-right { position: absolute; bottom: 1rem; right: 1rem; }
.popup-branded .popup-logo-inline-title { display: inline-flex; align-items: center; gap: 0.75rem; }
`;

		// Add custom CSS if provided
		if (this.config.customCSS) {
			css += `\n/* Custom CSS */\n${this.config.customCSS}`;
		}

		return css;
	}

	/**
	 * Inject CSS into document.
	 */
	private injectCSS(css: string): void {
		if (typeof document === 'undefined') return;

		// Remove existing style element
		if (this.styleElement) {
			this.styleElement.remove();
		}

		// Create and inject new style element
		this.styleElement = document.createElement('style');
		this.styleElement.id = 'popup-branding-styles';
		this.styleElement.textContent = css;
		document.head.appendChild(this.styleElement);
	}

	/**
	 * Inject fonts into document.
	 */
	private injectFonts(): void {
		if (typeof document === 'undefined') return;

		const { fonts } = this.config;

		if (fonts.source === 'google') {
			// Construct Google Fonts URL
			const families = [fonts.primary];
			if (fonts.secondary && fonts.secondary !== fonts.primary) {
				families.push(fonts.secondary);
			}

			const weights = fonts.weights.join(';');
			const familiesParam = families
				.map((f) => f.split(',')[0].trim())
				.filter((f) => !f.startsWith('-apple'))
				.map((f) => `family=${encodeURIComponent(f)}:wght@${weights}`)
				.join('&');

			if (familiesParam) {
				const linkId = 'popup-google-fonts';
				let link = document.getElementById(linkId) as HTMLLinkElement;

				if (!link) {
					link = document.createElement('link');
					link.id = linkId;
					link.rel = 'stylesheet';
					document.head.appendChild(link);
				}

				link.href = `https://fonts.googleapis.com/css2?${familiesParam}&display=swap`;
			}
		} else if (fonts.source === 'custom' && fonts.customUrl) {
			const linkId = 'popup-custom-fonts';
			let link = document.getElementById(linkId) as HTMLLinkElement;

			if (!link) {
				link = document.createElement('link');
				link.id = linkId;
				link.rel = 'stylesheet';
				document.head.appendChild(link);
			}

			link.href = fonts.customUrl;
		}
	}

	/**
	 * Remove branding (cleanup).
	 */
	removeBranding(): void {
		if (this.styleElement) {
			this.styleElement.remove();
			this.styleElement = null;
		}
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Factory Functions
// ═══════════════════════════════════════════════════════════════════════════

let brandingService: PopupBrandingService | null = null;

/**
 * Get the popup branding service singleton.
 */
export function getPopupBrandingService(): PopupBrandingService {
	if (!brandingService) {
		brandingService = new PopupBrandingService();
	}
	return brandingService;
}

/**
 * Create a new popup branding service with config.
 */
export function createPopupBrandingService(config: Partial<BrandingConfig>): PopupBrandingService {
	return new PopupBrandingService(config);
}

export default getPopupBrandingService;
