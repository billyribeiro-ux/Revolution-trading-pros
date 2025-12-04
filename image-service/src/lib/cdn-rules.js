/**
 * CDN Rules & Compression Profiles
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Configurable compression profiles for different use cases:
 * - Web (balanced quality/size)
 * - Social (optimized for social media platforms)
 * - Print (high quality)
 * - Thumbnail (aggressive compression)
 * - Archive (lossless preservation)
 *
 * CDN-aware transformations with:
 * - Cloudflare Image Resizing compatible
 * - Smart cache key generation
 * - Edge-side format negotiation
 *
 * @version 1.0.0
 */

// Predefined compression profiles
export const compressionProfiles = {
  // Default web optimization - good balance
  web: {
    name: 'Web Optimized',
    description: 'Balanced quality and file size for web use',
    formats: {
      avif: { quality: 65, effort: 4 },
      webp: { quality: 80, effort: 4 },
      jpeg: { quality: 82, progressive: true },
      png: { compressionLevel: 9, palette: true },
    },
    breakpoints: [320, 640, 768, 1024, 1280, 1920],
    maxWidth: 2560,
    maxHeight: 2560,
    preserveMetadata: false,
    sharpen: { sigma: 0.5 },
  },

  // Social media optimization
  social: {
    name: 'Social Media',
    description: 'Optimized for social platforms (Facebook, Twitter, etc.)',
    formats: {
      avif: { quality: 70, effort: 4 },
      webp: { quality: 85, effort: 4 },
      jpeg: { quality: 85, progressive: true },
      png: { compressionLevel: 9 },
    },
    breakpoints: [400, 800, 1200],
    maxWidth: 1200,
    maxHeight: 1200,
    preserveMetadata: false,
    sharpen: { sigma: 0.3 },
    // Platform-specific sizes
    presets: {
      'og-image': { width: 1200, height: 630 },
      'twitter-card': { width: 1200, height: 675 },
      'instagram-square': { width: 1080, height: 1080 },
      'instagram-portrait': { width: 1080, height: 1350 },
      'instagram-landscape': { width: 1080, height: 566 },
      'linkedin': { width: 1200, height: 627 },
      'pinterest': { width: 1000, height: 1500 },
    },
  },

  // E-commerce product images
  ecommerce: {
    name: 'E-commerce',
    description: 'High quality product images with zoom support',
    formats: {
      avif: { quality: 75, effort: 5 },
      webp: { quality: 88, effort: 5 },
      jpeg: { quality: 90, progressive: true },
      png: { compressionLevel: 8 },
    },
    breakpoints: [200, 400, 600, 800, 1200, 1600],
    maxWidth: 2000,
    maxHeight: 2000,
    preserveMetadata: false,
    sharpen: { sigma: 0.4 },
    background: '#ffffff',
  },

  // Thumbnail generation - aggressive compression
  thumbnail: {
    name: 'Thumbnail',
    description: 'Small preview images with aggressive compression',
    formats: {
      avif: { quality: 50, effort: 6 },
      webp: { quality: 65, effort: 6 },
      jpeg: { quality: 70, progressive: false },
      png: { compressionLevel: 9, palette: true, colors: 128 },
    },
    breakpoints: [50, 100, 150, 200, 300],
    maxWidth: 400,
    maxHeight: 400,
    preserveMetadata: false,
    sharpen: { sigma: 0.6 },
  },

  // High quality - minimal compression
  highQuality: {
    name: 'High Quality',
    description: 'Maximum quality with minimal compression',
    formats: {
      avif: { quality: 85, effort: 9, lossless: false },
      webp: { quality: 95, effort: 6 },
      jpeg: { quality: 95, progressive: true },
      png: { compressionLevel: 6 },
    },
    breakpoints: [640, 1280, 1920, 2560, 3840],
    maxWidth: 4096,
    maxHeight: 4096,
    preserveMetadata: true,
    sharpen: false,
  },

  // Archive - lossless preservation
  archive: {
    name: 'Archive',
    description: 'Lossless compression for archival purposes',
    formats: {
      avif: { lossless: true, effort: 9 },
      webp: { lossless: true, effort: 6 },
      png: { compressionLevel: 9 },
    },
    breakpoints: [],
    maxWidth: 8192,
    maxHeight: 8192,
    preserveMetadata: true,
    sharpen: false,
  },

  // Avatar/Profile pictures
  avatar: {
    name: 'Avatar',
    description: 'Circular profile pictures',
    formats: {
      avif: { quality: 70, effort: 4 },
      webp: { quality: 85, effort: 4 },
      jpeg: { quality: 85, progressive: false },
      png: { compressionLevel: 9 },
    },
    breakpoints: [32, 48, 64, 96, 128, 256],
    maxWidth: 512,
    maxHeight: 512,
    preserveMetadata: false,
    sharpen: { sigma: 0.4 },
    circle: true,
  },

  // Banner/Hero images
  banner: {
    name: 'Banner',
    description: 'Wide format hero and banner images',
    formats: {
      avif: { quality: 70, effort: 5 },
      webp: { quality: 85, effort: 5 },
      jpeg: { quality: 85, progressive: true },
      png: { compressionLevel: 8 },
    },
    breakpoints: [640, 1024, 1440, 1920, 2560],
    maxWidth: 3840,
    maxHeight: 1080,
    preserveMetadata: false,
    sharpen: { sigma: 0.3 },
  },
};

// CDN transformation rules
export const cdnRules = {
  // Cloudflare-compatible transformation URL patterns
  cloudflare: {
    baseUrl: '/cdn-cgi/image',
    paramMapping: {
      width: 'w',
      height: 'h',
      quality: 'q',
      format: 'f',
      fit: 'fit',
      gravity: 'g',
      sharpen: 'sharpen',
      blur: 'blur',
      brightness: 'brightness',
      contrast: 'contrast',
      gamma: 'gamma',
      rotate: 'rotate',
      background: 'background',
    },
    fitOptions: {
      cover: 'cover',
      contain: 'contain',
      fill: 'scale-down',
      inside: 'pad',
      outside: 'crop',
    },
  },

  // Imgix-compatible transformation
  imgix: {
    baseUrl: '',
    paramMapping: {
      width: 'w',
      height: 'h',
      quality: 'q',
      format: 'fm',
      fit: 'fit',
      crop: 'crop',
      auto: 'auto',
      dpr: 'dpr',
    },
  },

  // Custom R2 transformation (our service)
  r2: {
    baseUrl: '/api/media/transform',
    paramMapping: {
      width: 'w',
      height: 'h',
      quality: 'q',
      format: 'f',
      fit: 'fit',
      profile: 'profile',
    },
  },
};

class CDNRulesService {
  constructor() {
    this.defaultProfile = 'web';
    this.cdnProvider = 'r2';
  }

  /**
   * Get a compression profile by name
   */
  getProfile(name) {
    return compressionProfiles[name] || compressionProfiles[this.defaultProfile];
  }

  /**
   * List all available profiles
   */
  listProfiles() {
    return Object.entries(compressionProfiles).map(([key, profile]) => ({
      id: key,
      name: profile.name,
      description: profile.description,
    }));
  }

  /**
   * Get format options for a profile
   */
  getFormatOptions(profileName, format) {
    const profile = this.getProfile(profileName);
    return profile.formats[format] || profile.formats.jpeg;
  }

  /**
   * Get breakpoints for a profile
   */
  getBreakpoints(profileName) {
    const profile = this.getProfile(profileName);
    return profile.breakpoints;
  }

  /**
   * Generate transformation URL
   */
  generateTransformUrl(originalUrl, options = {}) {
    const {
      width,
      height,
      quality,
      format = 'auto',
      fit = 'cover',
      profile = this.defaultProfile,
      provider = this.cdnProvider,
    } = options;

    const rules = cdnRules[provider];
    if (!rules) {
      throw new Error(`Unknown CDN provider: ${provider}`);
    }

    const params = new URLSearchParams();

    // Apply profile defaults
    const profileConfig = this.getProfile(profile);

    if (width) params.set(rules.paramMapping.width, String(width));
    if (height) params.set(rules.paramMapping.height, String(height));

    if (format === 'auto') {
      params.set(rules.paramMapping.format, 'auto');
    } else if (format) {
      params.set(rules.paramMapping.format, format);
    }

    if (quality) {
      params.set(rules.paramMapping.quality, String(quality));
    } else if (format && profileConfig.formats[format]) {
      params.set(rules.paramMapping.quality, String(profileConfig.formats[format].quality));
    }

    if (fit && rules.fitOptions) {
      params.set(rules.paramMapping.fit, rules.fitOptions[fit] || fit);
    }

    // Build final URL
    const separator = originalUrl.includes('?') ? '&' : '?';
    return `${rules.baseUrl}/${originalUrl}${separator}${params.toString()}`;
  }

  /**
   * Generate srcset for responsive images
   */
  generateSrcset(originalUrl, options = {}) {
    const {
      profile = this.defaultProfile,
      format = 'auto',
      maxWidth,
    } = options;

    const profileConfig = this.getProfile(profile);
    let breakpoints = profileConfig.breakpoints;

    // Filter breakpoints if maxWidth specified
    if (maxWidth) {
      breakpoints = breakpoints.filter(bp => bp <= maxWidth);
    }

    const srcset = breakpoints.map(width => {
      const url = this.generateTransformUrl(originalUrl, {
        width,
        format,
        profile,
      });
      return `${url} ${width}w`;
    });

    return srcset.join(', ');
  }

  /**
   * Generate sizes attribute
   */
  generateSizes(breakpoints = []) {
    if (breakpoints.length === 0) {
      return '100vw';
    }

    const sizes = breakpoints.map((bp, i) => {
      if (i === breakpoints.length - 1) {
        return `${bp}px`;
      }
      return `(max-width: ${breakpoints[i + 1]}px) ${bp}px`;
    });

    return sizes.join(', ');
  }

  /**
   * Get cache key for transformation
   */
  getCacheKey(originalUrl, options) {
    const parts = [
      originalUrl,
      options.width || 'auto',
      options.height || 'auto',
      options.format || 'auto',
      options.quality || 'default',
      options.fit || 'cover',
    ];
    return parts.join('-');
  }

  /**
   * Get cache headers for transformed image
   */
  getCacheHeaders(options = {}) {
    const {
      maxAge = 31536000, // 1 year
      immutable = true,
      isPrivate = false,
    } = options;

    const directives = [
      isPrivate ? 'private' : 'public',
      `max-age=${maxAge}`,
    ];

    if (immutable) {
      directives.push('immutable');
    }

    return {
      'Cache-Control': directives.join(', '),
      'Vary': 'Accept',
    };
  }

  /**
   * Validate transformation options
   */
  validateOptions(options) {
    const errors = [];

    if (options.width && (options.width < 1 || options.width > 8192)) {
      errors.push('Width must be between 1 and 8192');
    }

    if (options.height && (options.height < 1 || options.height > 8192)) {
      errors.push('Height must be between 1 and 8192');
    }

    if (options.quality && (options.quality < 1 || options.quality > 100)) {
      errors.push('Quality must be between 1 and 100');
    }

    if (options.format && !['auto', 'avif', 'webp', 'jpeg', 'png', 'gif'].includes(options.format)) {
      errors.push('Invalid format');
    }

    return {
      valid: errors.length === 0,
      errors,
    };
  }

  /**
   * Get social media presets
   */
  getSocialPresets() {
    return compressionProfiles.social.presets;
  }

  /**
   * Generate social media image URL
   */
  generateSocialUrl(originalUrl, platform, options = {}) {
    const presets = this.getSocialPresets();
    const preset = presets[platform];

    if (!preset) {
      throw new Error(`Unknown social platform: ${platform}`);
    }

    return this.generateTransformUrl(originalUrl, {
      ...options,
      width: preset.width,
      height: preset.height,
      fit: 'cover',
      profile: 'social',
    });
  }
}

// Export singleton
export const cdnRulesService = new CDNRulesService();
export default CDNRulesService;
