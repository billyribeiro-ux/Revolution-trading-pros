/**
 * Browser Format Detection Service
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Intelligent format selection based on:
 * - Browser capabilities (Accept header)
 * - Device characteristics
 * - Network conditions
 * - Image content type
 *
 * Format priority:
 * 1. AVIF (best compression, growing support)
 * 2. WebP (good compression, wide support)
 * 3. JPEG XL (future format, limited support)
 * 4. JPEG/PNG (universal fallback)
 *
 * @version 1.0.0
 */

// Browser format support detection
const formatSupport = {
  avif: {
    // Browsers that support AVIF
    browsers: [
      { name: 'Chrome', minVersion: 85 },
      { name: 'Firefox', minVersion: 93 },
      { name: 'Opera', minVersion: 71 },
      { name: 'Edge', minVersion: 121 },
      { name: 'Safari', minVersion: 16 },
    ],
    acceptHeader: 'image/avif',
    quality: { min: 30, max: 80, default: 50 },
    bestFor: ['photos', 'illustrations', 'graphics'],
  },
  webp: {
    browsers: [
      { name: 'Chrome', minVersion: 32 },
      { name: 'Firefox', minVersion: 65 },
      { name: 'Opera', minVersion: 19 },
      { name: 'Edge', minVersion: 18 },
      { name: 'Safari', minVersion: 14 },
    ],
    acceptHeader: 'image/webp',
    quality: { min: 50, max: 90, default: 75 },
    bestFor: ['photos', 'illustrations', 'icons'],
  },
  jxl: {
    browsers: [
      // Limited support, mostly behind flags
      { name: 'Chrome', minVersion: 91, flag: true },
      { name: 'Firefox', minVersion: 90, flag: true },
    ],
    acceptHeader: 'image/jxl',
    quality: { min: 40, max: 90, default: 70 },
    bestFor: ['photos', 'high-fidelity'],
  },
};

// Content-type optimizations
const contentOptimizations = {
  photo: {
    preferredFormat: 'avif',
    fallbackFormat: 'webp',
    quality: 75,
  },
  illustration: {
    preferredFormat: 'webp',
    fallbackFormat: 'png',
    quality: 85,
  },
  icon: {
    preferredFormat: 'webp',
    fallbackFormat: 'png',
    quality: 90,
    preserveTransparency: true,
  },
  screenshot: {
    preferredFormat: 'webp',
    fallbackFormat: 'png',
    quality: 90,
  },
  graphic: {
    preferredFormat: 'webp',
    fallbackFormat: 'png',
    quality: 85,
    preserveTransparency: true,
  },
};

class FormatDetectionService {
  constructor() {
    this.cacheControl = {
      immutable: true,
      maxAge: 31536000, // 1 year for immutable resources
    };
  }

  /**
   * Detect best format from Accept header
   */
  detectFromAcceptHeader(acceptHeader) {
    if (!acceptHeader) {
      return { format: 'jpeg', reason: 'no-accept-header' };
    }

    const accept = acceptHeader.toLowerCase();

    // Check for modern format support
    if (accept.includes('image/avif')) {
      return { format: 'avif', reason: 'accept-header-avif' };
    }

    if (accept.includes('image/webp')) {
      return { format: 'webp', reason: 'accept-header-webp' };
    }

    if (accept.includes('image/jxl')) {
      return { format: 'jxl', reason: 'accept-header-jxl' };
    }

    // Fallback to JPEG for photos, PNG for graphics
    return { format: 'jpeg', reason: 'accept-header-fallback' };
  }

  /**
   * Detect from User-Agent
   */
  detectFromUserAgent(userAgent) {
    if (!userAgent) {
      return { format: 'jpeg', reason: 'no-user-agent' };
    }

    const ua = userAgent.toLowerCase();

    // Parse browser and version
    let browser = null;
    let version = 0;

    if (ua.includes('chrome/')) {
      const match = ua.match(/chrome\/(\d+)/);
      browser = 'Chrome';
      version = match ? parseInt(match[1]) : 0;
    } else if (ua.includes('firefox/')) {
      const match = ua.match(/firefox\/(\d+)/);
      browser = 'Firefox';
      version = match ? parseInt(match[1]) : 0;
    } else if (ua.includes('safari/') && !ua.includes('chrome')) {
      const match = ua.match(/version\/(\d+)/);
      browser = 'Safari';
      version = match ? parseInt(match[1]) : 0;
    } else if (ua.includes('edg/')) {
      const match = ua.match(/edg\/(\d+)/);
      browser = 'Edge';
      version = match ? parseInt(match[1]) : 0;
    } else if (ua.includes('opera/') || ua.includes('opr/')) {
      const match = ua.match(/(?:opera|opr)\/(\d+)/);
      browser = 'Opera';
      version = match ? parseInt(match[1]) : 0;
    }

    if (!browser) {
      return { format: 'jpeg', reason: 'unknown-browser' };
    }

    // Check format support
    for (const [format, support] of Object.entries(formatSupport)) {
      const browserSupport = support.browsers.find(
        b => b.name === browser && version >= b.minVersion && !b.flag
      );

      if (browserSupport) {
        return {
          format,
          reason: `user-agent-${browser.toLowerCase()}-${version}`,
          browser,
          version,
        };
      }
    }

    return {
      format: 'jpeg',
      reason: `user-agent-fallback-${browser.toLowerCase()}-${version}`,
      browser,
      version,
    };
  }

  /**
   * Get optimal format based on request
   */
  getOptimalFormat(request, options = {}) {
    const {
      acceptHeader = request?.headers?.accept,
      userAgent = request?.headers?.['user-agent'],
      contentType = 'photo',
      hasAlpha = false,
      isAnimated = false,
      preferQuality = false,
      preferSpeed = false,
    } = options;

    // Start with Accept header detection (most reliable)
    let result = this.detectFromAcceptHeader(acceptHeader);

    // If no modern format detected, try User-Agent
    if (result.format === 'jpeg') {
      const uaResult = this.detectFromUserAgent(userAgent);
      if (uaResult.format !== 'jpeg') {
        result = uaResult;
      }
    }

    // Adjust based on content characteristics
    if (hasAlpha && result.format === 'jpeg') {
      result.format = 'png';
      result.reason += '-alpha-required';
    }

    if (isAnimated) {
      // Keep GIF for animated images (or WebP animated if supported)
      if (result.format === 'avif' || result.format === 'webp') {
        result.animated = true;
      } else {
        result.format = 'gif';
        result.reason += '-animated';
      }
    }

    // Apply content-type optimizations
    const contentOpt = contentOptimizations[contentType];
    if (contentOpt) {
      result.quality = contentOpt.quality;
      result.preserveTransparency = contentOpt.preserveTransparency;
    }

    // Adjust quality based on preferences
    if (preferQuality) {
      result.quality = Math.min((result.quality || 80) + 10, 95);
    }

    if (preferSpeed) {
      result.quality = Math.max((result.quality || 80) - 10, 50);
    }

    return result;
  }

  /**
   * Generate Vary header for caching
   */
  getVaryHeader() {
    return 'Accept';
  }

  /**
   * Get cache control header
   */
  getCacheControlHeader(isImmutable = true) {
    if (isImmutable) {
      return `public, max-age=${this.cacheControl.maxAge}, immutable`;
    }
    return 'public, max-age=86400, stale-while-revalidate=604800';
  }

  /**
   * Generate srcset with format hints
   */
  generateSrcset(baseUrl, sizes, detectedFormat) {
    const srcset = sizes.map(size => {
      const url = this.appendFormatParam(baseUrl, detectedFormat, size);
      return `${url} ${size}w`;
    });

    return srcset.join(', ');
  }

  /**
   * Append format and size parameters to URL
   */
  appendFormatParam(url, format, width) {
    const separator = url.includes('?') ? '&' : '?';
    let params = `${separator}f=${format}`;

    if (width) {
      params += `&w=${width}`;
    }

    return url + params;
  }

  /**
   * Get picture element sources
   */
  getPictureSources(baseUrl, sizes = [320, 640, 1024, 1920]) {
    const sources = [];

    // AVIF source (highest priority)
    sources.push({
      type: 'image/avif',
      srcset: this.generateSrcset(baseUrl, sizes, 'avif'),
    });

    // WebP source
    sources.push({
      type: 'image/webp',
      srcset: this.generateSrcset(baseUrl, sizes, 'webp'),
    });

    // Fallback (JPEG/PNG based on content)
    sources.push({
      type: 'image/jpeg',
      srcset: this.generateSrcset(baseUrl, sizes, 'jpeg'),
    });

    return sources;
  }

  /**
   * Get format capabilities
   */
  getFormatCapabilities(format) {
    return formatSupport[format] || {
      browsers: [],
      quality: { min: 50, max: 95, default: 80 },
      bestFor: ['general'],
    };
  }

  /**
   * Check if format is supported by specific browser
   */
  isFormatSupported(format, browser, version) {
    const support = formatSupport[format];
    if (!support) return false;

    return support.browsers.some(
      b => b.name === browser && version >= b.minVersion && !b.flag
    );
  }

  /**
   * Get recommended format for content type
   */
  getRecommendedFormat(contentType, supportedFormats = ['avif', 'webp', 'jpeg', 'png']) {
    const opt = contentOptimizations[contentType];
    if (!opt) {
      return supportedFormats[0] || 'jpeg';
    }

    if (supportedFormats.includes(opt.preferredFormat)) {
      return opt.preferredFormat;
    }

    if (supportedFormats.includes(opt.fallbackFormat)) {
      return opt.fallbackFormat;
    }

    return supportedFormats[0] || 'jpeg';
  }
}

// Export singleton
export const formatDetection = new FormatDetectionService();
export default FormatDetectionService;
