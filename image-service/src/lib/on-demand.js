/**
 * On-Demand Variant Generator
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Generates image variants lazily when first requested.
 * Improves upload speed by deferring processing until needed.
 */

import sharp from 'sharp';
import { cache } from './cache.js';
import { storage } from './storage.js';

const RESPONSIVE_SIZES = {
  xs: 320,
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  '2xl': 1920,
};

const QUALITY_PRESETS = {
  maximum: { webp: 95, avif: 90, jpeg: 95 },
  balanced: { webp: 85, avif: 80, jpeg: 85 },
  performance: { webp: 75, avif: 70, jpeg: 75 },
  thumbnail: { webp: 70, avif: 65, jpeg: 70 },
};

class OnDemandGenerator {
  constructor() {
    this.generatingVariants = new Map(); // Prevent duplicate generation
  }

  /**
   * Get or generate a variant
   */
  async getVariant(mediaId, variantSpec) {
    const { format, width, height, quality = 'balanced' } = variantSpec;
    const variantKey = this.buildVariantKey(mediaId, variantSpec);

    // Check cache first
    const cached = await cache.getOnDemandVariant(mediaId, variantKey);
    if (cached) {
      return cached;
    }

    // Check if variant already exists in storage
    const storageKey = `media/${mediaId}/variants/${variantKey}`;
    const exists = await storage.exists(storageKey);
    if (exists) {
      const url = storage.getPublicUrl(storageKey);
      await cache.cacheOnDemandVariant(mediaId, variantKey, { url, key: storageKey });
      return { url, key: storageKey };
    }

    // Check if we're already generating this variant
    if (this.generatingVariants.has(storageKey)) {
      return this.generatingVariants.get(storageKey);
    }

    // Generate the variant
    const generationPromise = this.generateVariant(mediaId, variantSpec, storageKey);
    this.generatingVariants.set(storageKey, generationPromise);

    try {
      const result = await generationPromise;
      await cache.cacheOnDemandVariant(mediaId, variantKey, result);
      return result;
    } finally {
      this.generatingVariants.delete(storageKey);
    }
  }

  /**
   * Build variant key string
   */
  buildVariantKey(mediaId, spec) {
    const parts = [];
    if (spec.format) parts.push(spec.format);
    if (spec.width) parts.push(`w${spec.width}`);
    if (spec.height) parts.push(`h${spec.height}`);
    if (spec.sizeName) parts.push(spec.sizeName);
    if (spec.quality && spec.quality !== 'balanced') parts.push(spec.quality);
    return parts.join('-') || 'default';
  }

  /**
   * Generate a specific variant
   */
  async generateVariant(mediaId, spec, storageKey) {
    // Get original image
    const originalKey = `media/${mediaId}/original`;

    // Try to find the original file
    let originalBuffer;
    let originalFormat;

    for (const ext of ['jpeg', 'jpg', 'png', 'webp', 'gif']) {
      try {
        const testKey = `${originalKey}/*.${ext}`;
        const file = await storage.get(`media/${mediaId}/original`);
        if (file) {
          originalBuffer = Buffer.isBuffer(file.body) ? file.body : await this.streamToBuffer(file.body);
          originalFormat = ext;
          break;
        }
      } catch {
        continue;
      }
    }

    // If we can't find original by extension, try getting metadata from cache
    const metadata = await cache.getMediaMeta(mediaId);
    if (metadata && metadata.originalKey) {
      const file = await storage.get(metadata.originalKey);
      originalBuffer = Buffer.isBuffer(file.body) ? file.body : await this.streamToBuffer(file.body);
    }

    if (!originalBuffer) {
      throw new Error(`Original image not found for media ${mediaId}`);
    }

    const qualityPreset = QUALITY_PRESETS[spec.quality] || QUALITY_PRESETS.balanced;
    let pipeline = sharp(originalBuffer);

    // Apply resize
    if (spec.width || spec.height) {
      pipeline = pipeline.resize(spec.width || null, spec.height || null, {
        withoutEnlargement: true,
        fit: spec.fit || 'inside',
      });
    }

    // Apply format conversion
    const outputFormat = spec.format || 'webp';
    switch (outputFormat) {
      case 'webp':
        pipeline = pipeline.webp({ quality: qualityPreset.webp });
        break;
      case 'avif':
        pipeline = pipeline.avif({ quality: qualityPreset.avif });
        break;
      case 'jpeg':
      case 'jpg':
        pipeline = pipeline.jpeg({ quality: qualityPreset.jpeg, progressive: true });
        break;
      case 'png':
        pipeline = pipeline.png({ compressionLevel: 9 });
        break;
    }

    const outputBuffer = await pipeline.toBuffer();
    const outputMeta = await sharp(outputBuffer).metadata();

    // Upload to storage
    await storage.upload(storageKey, outputBuffer, `image/${outputFormat}`);

    return {
      key: storageKey,
      url: storage.getPublicUrl(storageKey),
      width: outputMeta.width,
      height: outputMeta.height,
      size: outputBuffer.length,
      format: outputFormat,
    };
  }

  /**
   * Get responsive variant by size name
   */
  async getResponsiveVariant(mediaId, sizeName, format = 'webp') {
    const width = RESPONSIVE_SIZES[sizeName];
    if (!width) {
      throw new Error(`Unknown responsive size: ${sizeName}`);
    }

    return this.getVariant(mediaId, {
      format,
      width,
      sizeName,
    });
  }

  /**
   * Get thumbnail
   */
  async getThumbnail(mediaId, size = 300) {
    return this.getVariant(mediaId, {
      format: 'webp',
      width: size,
      height: size,
      fit: 'cover',
      sizeName: 'thumb',
      quality: 'thumbnail',
    });
  }

  /**
   * Pregenerate common variants (can be called in background)
   */
  async pregenerateCommonVariants(mediaId) {
    const tasks = [
      // WebP full size
      this.getVariant(mediaId, { format: 'webp' }),
      // Thumbnail
      this.getThumbnail(mediaId),
      // Common responsive sizes
      this.getResponsiveVariant(mediaId, 'sm'),
      this.getResponsiveVariant(mediaId, 'md'),
      this.getResponsiveVariant(mediaId, 'lg'),
    ];

    // Run in background, don't wait
    Promise.all(tasks).catch((err) => {
      console.error(`Background variant generation failed for ${mediaId}:`, err);
    });
  }

  /**
   * Convert stream to buffer
   */
  async streamToBuffer(stream) {
    const chunks = [];
    for await (const chunk of stream) {
      chunks.push(chunk);
    }
    return Buffer.concat(chunks);
  }

  /**
   * Get all available responsive sizes
   */
  getResponsiveSizes() {
    return { ...RESPONSIVE_SIZES };
  }
}

// Export singleton
export const onDemand = new OnDemandGenerator();
export default OnDemandGenerator;
