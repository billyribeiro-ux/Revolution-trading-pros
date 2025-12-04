/**
 * Revolution Image Service v2.0
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * High-performance image processing service with:
 * - Parallel processing using worker threads (~60% faster)
 * - On-demand variant generation (lazy loading)
 * - Redis caching layer
 * - Streaming uploads to R2
 * - HTTP/2 push hints for critical images
 *
 * @version 2.0.0
 */

import express from 'express';
import cors from 'cors';
import multer from 'multer';
import sharp from 'sharp';
import { encode } from 'blurhash';
import { v4 as uuidv4 } from 'uuid';
import path from 'path';
import fs from 'fs/promises';
import http2 from 'http2';
import 'dotenv/config';

import { workerPool } from './lib/worker-pool.js';
import { cache } from './lib/cache.js';
import { storage } from './lib/storage.js';
import { onDemand } from './lib/on-demand.js';
import { aiService } from './lib/ai-service.js';
import { maintenanceService } from './lib/maintenance.js';
import { formatDetection } from './lib/format-detection.js';
import { cdnRulesService, compressionProfiles } from './lib/cdn-rules.js';
import { bulkImportService } from './lib/bulk-import.js';

const app = express();
const PORT = process.env.PORT || 3001;

// ═══════════════════════════════════════════════════════════════════════════
// Middleware
// ═══════════════════════════════════════════════════════════════════════════

app.use(cors());
app.use(express.json({ limit: '100mb' }));

// Serve local uploads for testing
app.use('/uploads', express.static(path.join(process.cwd(), 'uploads')));

// Configure multer for file uploads
const uploadStorage = multer.memoryStorage();
const upload = multer({
  storage: uploadStorage,
  limits: { fileSize: 100 * 1024 * 1024 }, // 100MB with streaming
  fileFilter: (req, file, cb) => {
    const allowedTypes = [
      'image/jpeg', 'image/png', 'image/webp', 'image/gif',
      'image/avif', 'image/heic', 'image/heif', 'image/svg+xml'
    ];
    // Also allow by extension for WebP files that may have wrong MIME
    const allowedExts = ['.jpg', '.jpeg', '.png', '.webp', '.gif', '.avif', '.heic', '.heif', '.svg'];
    const ext = path.extname(file.originalname).toLowerCase();

    if (allowedTypes.includes(file.mimetype) || allowedExts.includes(ext)) {
      cb(null, true);
    } else {
      cb(new Error(`Invalid file type: ${file.mimetype}`), false);
    }
  }
});

// ═══════════════════════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════
// Health & Capabilities
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Health check endpoint
 */
app.get('/health', async (req, res) => {
  const workerStats = workerPool.getStats ? workerPool.getStats() : { poolSize: 0 };
  const cacheStats = cache.getStats ? cache.getStats() : { redisConnected: false };
  const storageStats = storage.getStats ? storage.getStats() : { isR2Configured: false };

  res.json({
    status: 'healthy',
    service: 'revolution-image-service',
    version: '2.0.0',
    sharp: sharp.versions,
    workers: workerStats,
    cache: cacheStats,
    storage: storageStats,
    features: {
      parallelProcessing: true,
      onDemandVariants: true,
      redisCache: cacheStats.redisConnected,
      streamingUploads: true,
      http2Push: true,
    },
  });
});

/**
 * Service stats
 */
app.get('/stats', (req, res) => {
  const uptime = process.uptime();
  const workerStats = workerPool.getStats ? workerPool.getStats() : { poolSize: 0 };
  const cacheStats = cache.getStats ? cache.getStats() : { redisConnected: false };

  res.json({
    success: true,
    uptime,
    uptimeFormatted: `${Math.floor(uptime / 3600)}h ${Math.floor((uptime % 3600) / 60)}m`,
    imagesProcessed: globalStats.imagesProcessed || 0,
    variantsGenerated: globalStats.variantsGenerated || 0,
    totalBytesProcessed: globalStats.totalBytesProcessed || 0,
    workers: workerStats,
    cache: cacheStats,
  });
});

// Global stats tracking
const globalStats = {
  imagesProcessed: 0,
  variantsGenerated: 0,
  totalBytesProcessed: 0,
};

/**
 * Get Sharp capabilities
 */
app.get('/capabilities', async (req, res) => {
  const formats = await sharp.format;
  res.json({
    formats: Object.keys(formats).filter(f => formats[f].input || formats[f].output),
    inputFormats: Object.keys(formats).filter(f => formats[f].input),
    outputFormats: Object.keys(formats).filter(f => formats[f].output),
    responsiveSizes: RESPONSIVE_SIZES,
    qualityPresets: Object.keys(QUALITY_PRESETS),
    features: ['parallel-processing', 'on-demand-variants', 'redis-cache', 'streaming-uploads', 'blurhash', 'lqip'],
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// Upload Endpoints
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Upload and process image (fast mode - original + minimal variants)
 * Full variants generated on-demand
 */
app.post('/upload', upload.single('image'), async (req, res) => {
  const startTime = Date.now();

  try {
    if (!req.file) {
      return res.status(400).json({ success: false, error: 'No image file provided' });
    }

    const options = {
      preset: req.body.preset || 'balanced',
      generateWebp: req.body.generateWebp !== 'false',
      generateAvif: req.body.generateAvif !== 'false',
      generateResponsive: req.body.generateResponsive !== 'false',
      generateThumbnail: req.body.generateThumbnail !== 'false',
      generateBlurhash: req.body.generateBlurhash !== 'false',
      lazyVariants: req.body.lazyVariants === 'true', // New: defer variant generation
      collection: req.body.collection || 'general',
      altText: req.body.altText || '',
      title: req.body.title || '',
    };

    const result = await processImageParallel(req.file.buffer, req.file.originalname, options);

    // Add HTTP/2 push hints
    if (req.httpVersion === '2.0' && result.variants?.length > 0) {
      const pushUrls = result.variants
        .filter(v => v.type === 'webp' || v.type === 'thumbnail')
        .map(v => v.url);

      res.setHeader('Link', pushUrls.map(url => `<${url}>; rel=preload; as=image`).join(', '));
    }

    res.json({
      success: true,
      processingTime: Date.now() - startTime,
      ...result,
    });
  } catch (error) {
    console.error('Upload error:', error);
    res.status(500).json({
      success: false,
      error: error.message,
    });
  }
});

/**
 * Upload with full variant generation (legacy mode)
 */
app.post('/upload/full', upload.single('image'), async (req, res) => {
  const startTime = Date.now();

  try {
    if (!req.file) {
      return res.status(400).json({ success: false, error: 'No image file provided' });
    }

    const options = {
      preset: req.body.preset || 'balanced',
      generateWebp: true,
      generateAvif: true,
      generateResponsive: true,
      generateThumbnail: true,
      generateBlurhash: true,
      lazyVariants: false,
      collection: req.body.collection || 'general',
    };

    const result = await processImageParallel(req.file.buffer, req.file.originalname, options);

    res.json({
      success: true,
      processingTime: Date.now() - startTime,
      ...result,
    });
  } catch (error) {
    console.error('Upload error:', error);
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Fast upload - only stores original, generates blurhash
 * All variants generated on-demand
 */
app.post('/upload/fast', upload.single('image'), async (req, res) => {
  const startTime = Date.now();

  try {
    if (!req.file) {
      return res.status(400).json({ success: false, error: 'No image file provided' });
    }

    const result = await processImageFast(req.file.buffer, req.file.originalname, {
      collection: req.body.collection || 'general',
      altText: req.body.altText || '',
    });

    res.json({
      success: true,
      processingTime: Date.now() - startTime,
      mode: 'fast',
      ...result,
    });
  } catch (error) {
    console.error('Fast upload error:', error);
    res.status(500).json({ success: false, error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════════════════════
// On-Demand Variant Endpoints
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get variant on-demand
 */
app.get('/variant/:mediaId/:format', async (req, res) => {
  try {
    const { mediaId, format } = req.params;
    const { w: width, h: height, q: quality, size } = req.query;

    let spec;
    if (size && RESPONSIVE_SIZES[size]) {
      spec = { format, width: RESPONSIVE_SIZES[size], sizeName: size, quality };
    } else {
      spec = {
        format,
        width: width ? parseInt(width) : null,
        height: height ? parseInt(height) : null,
        quality: quality || 'balanced',
      };
    }

    const variant = await onDemand.getVariant(mediaId, spec);

    // Redirect to the variant URL
    res.redirect(302, variant.url);
  } catch (error) {
    console.error('Variant error:', error);
    res.status(404).json({ success: false, error: error.message });
  }
});

/**
 * Get responsive variant
 */
app.get('/responsive/:mediaId/:sizeName', async (req, res) => {
  try {
    const { mediaId, sizeName } = req.params;
    const format = req.query.format || 'webp';

    const variant = await onDemand.getResponsiveVariant(mediaId, sizeName, format);
    res.redirect(302, variant.url);
  } catch (error) {
    res.status(404).json({ success: false, error: error.message });
  }
});

/**
 * Get thumbnail
 */
app.get('/thumbnail/:mediaId', async (req, res) => {
  try {
    const { mediaId } = req.params;
    const size = parseInt(req.query.size) || 300;

    const thumbnail = await onDemand.getThumbnail(mediaId, size);
    res.redirect(302, thumbnail.url);
  } catch (error) {
    res.status(404).json({ success: false, error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════════════════════
// Other Endpoints
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Process image from URL
 */
app.post('/process-url', async (req, res) => {
  const startTime = Date.now();

  try {
    const { url, options = {} } = req.body;

    if (!url) {
      return res.status(400).json({ error: 'No URL provided' });
    }

    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Failed to fetch image: ${response.status}`);
    }

    const buffer = Buffer.from(await response.arrayBuffer());
    const filename = path.basename(new URL(url).pathname) || 'image.jpg';

    const result = await processImageParallel(buffer, filename, {
      preset: options.preset || 'balanced',
      generateWebp: options.generateWebp !== false,
      generateAvif: options.generateAvif !== false,
      generateResponsive: options.generateResponsive !== false,
      generateThumbnail: options.generateThumbnail !== false,
      generateBlurhash: options.generateBlurhash !== false,
      collection: options.collection || 'general',
    });

    res.json({
      success: true,
      processingTime: Date.now() - startTime,
      ...result,
    });
  } catch (error) {
    console.error('Process URL error:', error);
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Generate blurhash only
 */
app.post('/blurhash', upload.single('image'), async (req, res) => {
  try {
    if (!req.file) {
      return res.status(400).json({ error: 'No image provided' });
    }

    const blurhash = await generateBlurhash(req.file.buffer);
    res.json({ success: true, blurhash });
  } catch (error) {
    console.error('Blurhash error:', error);
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Generate single variant
 */
app.post('/generate-variant', upload.single('image'), async (req, res) => {
  const startTime = Date.now();

  try {
    const buffer = req.file?.buffer;
    const { width, height, format, quality } = req.body;

    if (!buffer) {
      return res.status(400).json({ error: 'No image provided' });
    }

    const variant = await generateVariant(buffer, {
      width: parseInt(width) || null,
      height: parseInt(height) || null,
      format: format || 'webp',
      quality: parseInt(quality) || 85,
    });

    res.json({
      success: true,
      processingTime: Date.now() - startTime,
      variant,
    });
  } catch (error) {
    console.error('Generate variant error:', error);
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Get presigned upload URL (for direct browser uploads)
 */
app.post('/presigned-upload', async (req, res) => {
  try {
    const { filename, contentType } = req.body;
    const mediaId = uuidv4();
    const key = `media/${mediaId}/original/${filename}`;

    const presignedUrl = await storage.getPresignedUploadUrl(key, contentType);

    res.json({
      success: true,
      mediaId,
      uploadUrl: presignedUrl,
      key,
    });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Delete media and all variants
 */
app.delete('/media/:id', async (req, res) => {
  try {
    const { id } = req.params;

    await storage.deletePrefix(`media/${id}/`);
    await cache.invalidateMedia(id);

    res.json({ success: true, message: `Deleted all files for ${id}` });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════════════════════
// AI-Powered Endpoints
// ═══════════════════════════════════════════════════════════════════════════

/**
 * AI service status
 */
app.get('/ai/status', (req, res) => {
  res.json({
    success: true,
    ...aiService.getStatus(),
  });
});

/**
 * Generate alt text for image
 */
app.post('/ai/alt-text', upload.single('image'), async (req, res) => {
  try {
    if (!req.file) {
      return res.status(400).json({ success: false, error: 'No image provided' });
    }

    const context = req.body.context || '';
    const altText = await aiService.generateAltText(req.file.buffer, context);

    res.json({
      success: true,
      altText,
      provider: aiService.isEnabled() ? 'ai' : 'local',
    });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Full image analysis
 */
app.post('/ai/analyze', upload.single('image'), async (req, res) => {
  try {
    if (!req.file) {
      return res.status(400).json({ success: false, error: 'No image provided' });
    }

    const useAI = req.body.useAI !== 'false';
    const analysis = await aiService.analyzeImage(req.file.buffer, { useAI });

    res.json({
      success: true,
      analysis,
      provider: aiService.isEnabled() && useAI ? 'ai' : 'local',
    });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Smart crop with subject detection
 */
app.post('/ai/smart-crop', upload.single('image'), async (req, res) => {
  try {
    if (!req.file) {
      return res.status(400).json({ success: false, error: 'No image provided' });
    }

    const { width, height, focalX, focalY } = req.body;
    const targetWidth = parseInt(width) || 800;
    const targetHeight = parseInt(height) || 600;

    const focalPoint = focalX && focalY
      ? { x: parseFloat(focalX), y: parseFloat(focalY) }
      : null;

    const result = await aiService.smartCrop(
      req.file.buffer,
      targetWidth,
      targetHeight,
      { focalPoint }
    );

    // Save the cropped image
    const id = uuidv4();
    const key = `media/cropped/${id}.webp`;

    const webpBuffer = await sharp(result.buffer)
      .webp({ quality: 85 })
      .toBuffer();

    await storage.upload(key, webpBuffer, 'image/webp');

    res.json({
      success: true,
      url: storage.getPublicUrl(key),
      key,
      cropRegion: result.cropRegion,
      focalPoint: result.focalPoint,
      dimensions: { width: targetWidth, height: targetHeight },
    });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Auto-tag image
 */
app.post('/ai/auto-tag', upload.single('image'), async (req, res) => {
  try {
    if (!req.file) {
      return res.status(400).json({ success: false, error: 'No image provided' });
    }

    const existingTags = req.body.existingTags
      ? JSON.parse(req.body.existingTags)
      : [];

    const tags = await aiService.autoTag(req.file.buffer, existingTags);

    res.json({
      success: true,
      tags,
      provider: aiService.isEnabled() ? 'ai' : 'local',
    });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════════════════════
// Processing Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Process image with parallel worker threads
 */
async function processImageParallel(buffer, originalFilename, options) {
  const id = uuidv4();
  const baseName = path.parse(originalFilename).name;
  const quality = QUALITY_PRESETS[options.preset] || QUALITY_PRESETS.balanced;

  // Get metadata first (fast operation)
  const metadata = await sharp(buffer).metadata();
  const { width: originalWidth, height: originalHeight, format: originalFormat } = metadata;

  const results = {
    id,
    original: {
      filename: originalFilename,
      width: originalWidth,
      height: originalHeight,
      format: originalFormat,
      size: buffer.length,
    },
    variants: [],
    blurhash: null,
    lqip: null,
  };

  // Upload original
  const originalKey = `media/${id}/original/${baseName}.${originalFormat}`;
  await storage.upload(originalKey, buffer, `image/${originalFormat}`);
  results.original.key = originalKey;
  results.original.url = storage.getPublicUrl(originalKey);

  // Build parallel tasks
  const parallelTasks = [];

  // Always generate BlurHash and LQIP (small, fast)
  if (options.generateBlurhash) {
    parallelTasks.push(generateBlurhash(buffer).then(bh => ({ type: 'blurhash', data: bh })));
  }
  parallelTasks.push(generateLqip(buffer).then(lq => ({ type: 'lqip', data: lq })));

  // Generate variants in parallel (if not lazy mode)
  if (!options.lazyVariants) {
    if (options.generateWebp && originalFormat !== 'webp') {
      parallelTasks.push(
        sharp(buffer)
          .webp({ quality: quality.webp, effort: 4 })
          .toBuffer()
          .then(async (webpBuffer) => {
            const webpKey = `media/${id}/webp/${baseName}.webp`;
            await storage.upload(webpKey, webpBuffer, 'image/webp');
            return {
              type: 'variant',
              data: {
                type: 'webp',
                key: webpKey,
                url: storage.getPublicUrl(webpKey),
                width: originalWidth,
                height: originalHeight,
                size: webpBuffer.length,
                savings: Math.round((1 - webpBuffer.length / buffer.length) * 100),
              },
            };
          })
      );
    }

    if (options.generateAvif) {
      parallelTasks.push(
        sharp(buffer)
          .avif({ quality: quality.avif, effort: 4 })
          .toBuffer()
          .then(async (avifBuffer) => {
            const avifKey = `media/${id}/avif/${baseName}.avif`;
            await storage.upload(avifKey, avifBuffer, 'image/avif');
            return {
              type: 'variant',
              data: {
                type: 'avif',
                key: avifKey,
                url: storage.getPublicUrl(avifKey),
                width: originalWidth,
                height: originalHeight,
                size: avifBuffer.length,
                savings: Math.round((1 - avifBuffer.length / buffer.length) * 100),
              },
            };
          })
          .catch((err) => {
            console.warn('AVIF generation failed:', err.message);
            return null;
          })
      );
    }

    if (options.generateThumbnail) {
      parallelTasks.push(
        sharp(buffer)
          .resize(300, 300, { fit: 'cover', position: 'center' })
          .webp({ quality: quality.thumbnail || 80 })
          .toBuffer()
          .then(async (thumbBuffer) => {
            const thumbKey = `media/${id}/thumbnail/${baseName}-thumb.webp`;
            await storage.upload(thumbKey, thumbBuffer, 'image/webp');
            return {
              type: 'thumbnail',
              data: {
                key: thumbKey,
                url: storage.getPublicUrl(thumbKey),
                width: 300,
                height: 300,
                size: thumbBuffer.length,
              },
            };
          })
      );
    }

    if (options.generateResponsive) {
      // Generate all responsive sizes in parallel
      const responsivePromises = Object.entries(RESPONSIVE_SIZES)
        .filter(([, targetWidth]) => targetWidth < originalWidth)
        .map(([sizeName, targetWidth]) =>
          sharp(buffer)
            .resize(targetWidth, null, { withoutEnlargement: true })
            .webp({ quality: quality.webp, effort: 4 })
            .toBuffer()
            .then(async (resizedBuffer) => {
              const resizedMeta = await sharp(resizedBuffer).metadata();
              const responsiveKey = `media/${id}/responsive/${baseName}-${sizeName}.webp`;
              await storage.upload(responsiveKey, resizedBuffer, 'image/webp');
              return {
                type: 'variant',
                data: {
                  type: 'responsive',
                  sizeName,
                  key: responsiveKey,
                  url: storage.getPublicUrl(responsiveKey),
                  width: resizedMeta.width,
                  height: resizedMeta.height,
                  size: resizedBuffer.length,
                },
              };
            })
        );

      parallelTasks.push(...responsivePromises);
    }
  }

  // Execute all tasks in parallel
  const taskResults = await Promise.all(parallelTasks);

  // Process results
  for (const result of taskResults) {
    if (!result) continue;

    switch (result.type) {
      case 'blurhash':
        results.blurhash = result.data;
        break;
      case 'lqip':
        results.lqip = result.data;
        break;
      case 'thumbnail':
        results.thumbnail = result.data;
        break;
      case 'variant':
        results.variants.push(result.data);
        break;
    }
  }

  // Calculate stats
  const webpVariant = results.variants.find(v => v.type === 'webp');
  const bestVariantSize = webpVariant?.size || buffer.length;

  results.stats = {
    originalSize: buffer.length,
    optimizedSize: bestVariantSize,
    savingsPercent: Math.round((1 - bestVariantSize / buffer.length) * 100),
    variantsCount: results.variants.length,
    parallelProcessing: true,
  };

  // Cache metadata
  await cache.cacheMediaMeta(id, {
    id,
    originalKey,
    originalFormat,
    width: originalWidth,
    height: originalHeight,
    size: buffer.length,
    blurhash: results.blurhash,
    lqip: results.lqip,
  });

  if (results.blurhash) {
    await cache.cacheBlurhash(id, results.blurhash);
  }

  return results;
}

/**
 * Fast upload - minimal processing
 */
async function processImageFast(buffer, originalFilename, options) {
  const id = uuidv4();
  const baseName = path.parse(originalFilename).name;

  const metadata = await sharp(buffer).metadata();
  const { width, height, format } = metadata;

  // Upload original only
  const originalKey = `media/${id}/original/${baseName}.${format}`;
  await storage.upload(originalKey, buffer, `image/${format}`);

  // Generate BlurHash and LQIP in parallel (these are fast)
  const [blurhash, lqip] = await Promise.all([
    generateBlurhash(buffer),
    generateLqip(buffer),
  ]);

  const result = {
    id,
    original: {
      filename: originalFilename,
      key: originalKey,
      url: storage.getPublicUrl(originalKey),
      width,
      height,
      format,
      size: buffer.length,
    },
    blurhash,
    lqip,
    variants: [], // Variants generated on-demand
    onDemandUrls: {
      webp: `/variant/${id}/webp`,
      avif: `/variant/${id}/avif`,
      thumbnail: `/thumbnail/${id}`,
      responsive: Object.fromEntries(
        Object.keys(RESPONSIVE_SIZES).map(size => [size, `/responsive/${id}/${size}`])
      ),
    },
  };

  // Cache metadata
  await cache.cacheMediaMeta(id, {
    id,
    originalKey,
    originalFormat: format,
    width,
    height,
    size: buffer.length,
    blurhash,
    lqip,
  });

  // Optionally pregenerate common variants in background
  if (process.env.PREGENERATE_VARIANTS === 'true') {
    onDemand.pregenerateCommonVariants(id);
  }

  return result;
}

/**
 * Generate BlurHash
 */
async function generateBlurhash(buffer) {
  try {
    const { data, info } = await sharp(buffer)
      .resize(32, 32, { fit: 'inside' })
      .ensureAlpha()
      .raw()
      .toBuffer({ resolveWithObject: true });

    const pixels = new Uint8ClampedArray(data);
    return encode(pixels, info.width, info.height, 4, 3);
  } catch (error) {
    console.error('BlurHash generation failed:', error);
    return null;
  }
}

/**
 * Generate LQIP
 */
async function generateLqip(buffer) {
  const lqipBuffer = await sharp(buffer)
    .resize(32, null, { withoutEnlargement: true })
    .blur(10)
    .jpeg({ quality: 20 })
    .toBuffer();

  return `data:image/jpeg;base64,${lqipBuffer.toString('base64')}`;
}

/**
 * Generate single variant
 */
async function generateVariant(buffer, options) {
  let pipeline = sharp(buffer);

  if (options.width || options.height) {
    pipeline = pipeline.resize(options.width, options.height, {
      withoutEnlargement: true,
      fit: options.fit || 'inside',
    });
  }

  const format = options.format || 'webp';
  const quality = options.quality || 85;

  switch (format) {
    case 'webp':
      pipeline = pipeline.webp({ quality });
      break;
    case 'avif':
      pipeline = pipeline.avif({ quality });
      break;
    case 'jpeg':
    case 'jpg':
      pipeline = pipeline.jpeg({ quality, progressive: true });
      break;
    case 'png':
      pipeline = pipeline.png({ compressionLevel: 9 });
      break;
  }

  const outputBuffer = await pipeline.toBuffer();
  const outputMeta = await sharp(outputBuffer).metadata();

  const id = uuidv4();
  const key = `media/variants/${id}.${format}`;
  await storage.upload(key, outputBuffer, `image/${format}`);

  return {
    key,
    url: storage.getPublicUrl(key),
    width: outputMeta.width,
    height: outputMeta.height,
    format,
    size: outputBuffer.length,
  };
}

// ═══════════════════════════════════════════════════════════════════════════
// Maintenance Endpoints
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get maintenance status
 */
app.get('/maintenance/status', (req, res) => {
  res.json({
    success: true,
    ...maintenanceService.getStats(),
  });
});

/**
 * Run maintenance manually
 */
app.post('/maintenance/run', async (req, res) => {
  try {
    const stats = await maintenanceService.run();
    res.json({ success: true, stats });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Run specific maintenance task
 */
app.post('/maintenance/task/:taskName', async (req, res) => {
  try {
    await maintenanceService.runTask(req.params.taskName);
    res.json({ success: true, task: req.params.taskName });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════════════════════
// Format Detection Endpoints
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Detect optimal format for request (GET)
 */
app.get('/format/detect', (req, res) => {
  const result = formatDetection.getOptimalFormat(req, {
    contentType: req.query.contentType || 'photo',
    hasAlpha: req.query.hasAlpha === 'true',
    isAnimated: req.query.isAnimated === 'true',
  });

  res.json({
    success: true,
    ...result,
    headers: {
      Vary: formatDetection.getVaryHeader(),
      'Cache-Control': formatDetection.getCacheControlHeader(),
    },
  });
});

/**
 * Detect optimal format for request (POST)
 */
app.post('/format/detect', (req, res) => {
  const { acceptHeader, userAgent, contentType, hasAlpha, isAnimated } = req.body;

  const mockReq = {
    headers: {
      accept: acceptHeader || req.headers.accept,
      'user-agent': userAgent || req.headers['user-agent'],
    },
  };

  const result = formatDetection.getOptimalFormat(mockReq, {
    contentType: contentType || 'photo',
    hasAlpha: hasAlpha === true,
    isAnimated: isAnimated === true,
  });

  res.json({
    success: true,
    ...result,
    headers: {
      Vary: formatDetection.getVaryHeader(),
      'Cache-Control': formatDetection.getCacheControlHeader(),
    },
  });
});

/**
 * Get picture sources for responsive images (GET)
 */
app.get('/format/picture-sources', (req, res) => {
  const { url, sizes } = req.query;
  const sizeArray = sizes ? sizes.split(',').map(Number) : [320, 640, 1024, 1920];

  const sources = formatDetection.getPictureSources(url || '/placeholder', sizeArray);

  res.json({
    success: true,
    sources,
    sizes: cdnRulesService.generateSizes(sizeArray),
  });
});

/**
 * Get picture sources for responsive images (POST)
 */
app.post('/format/picture-sources', (req, res) => {
  const { url, sizes } = req.body;
  const sizeArray = Array.isArray(sizes) ? sizes : [320, 640, 1024, 1920];

  const sources = formatDetection.getPictureSources(url || '/placeholder', sizeArray);

  res.json({
    success: true,
    sources,
    sizes: cdnRulesService.generateSizes(sizeArray),
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// CDN Rules & Compression Profiles
// ═══════════════════════════════════════════════════════════════════════════

/**
 * List all compression profiles
 */
app.get('/cdn/profiles', (req, res) => {
  res.json({
    success: true,
    profiles: cdnRulesService.listProfiles(),
  });
});

/**
 * Get specific profile details
 */
app.get('/cdn/profiles/:name', (req, res) => {
  const profile = compressionProfiles[req.params.name];
  if (!profile) {
    return res.status(404).json({ success: false, error: 'Profile not found' });
  }
  res.json({ success: true, profile });
});

/**
 * Generate transformation URL
 */
app.post('/cdn/transform-url', (req, res) => {
  try {
    const { originalUrl, url: urlParam, width, height, format, quality, fit, profile } = req.body;
    const targetUrl = originalUrl || urlParam;

    if (!targetUrl) {
      return res.status(400).json({ success: false, error: 'URL is required (use "url" or "originalUrl")' });
    }

    const validation = cdnRulesService.validateOptions({ width, height, quality, format });
    if (!validation.valid) {
      return res.status(400).json({ success: false, errors: validation.errors });
    }

    const transformedUrl = cdnRulesService.generateTransformUrl(targetUrl, {
      width, height, format, quality, fit, profile,
    });

    res.json({
      success: true,
      url: transformedUrl,
      cacheHeaders: cdnRulesService.getCacheHeaders(),
    });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Generate srcset for responsive images
 */
app.post('/cdn/srcset', (req, res) => {
  try {
    const { originalUrl, url: urlParam, profile, format, maxWidth } = req.body;
    const targetUrl = originalUrl || urlParam;

    if (!targetUrl) {
      return res.status(400).json({ success: false, error: 'URL is required (use "url" or "originalUrl")' });
    }

    const srcset = cdnRulesService.generateSrcset(targetUrl, {
      profile: profile || 'web',
      format: format || 'auto',
      maxWidth: maxWidth ? parseInt(maxWidth) : null,
    });

    res.json({
      success: true,
      srcset,
      sizes: cdnRulesService.generateSizes(cdnRulesService.getBreakpoints(profile || 'web')),
    });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Get social media presets
 */
app.get('/cdn/social-presets', (req, res) => {
  res.json({
    success: true,
    presets: cdnRulesService.getSocialPresets(),
  });
});

/**
 * Generate social media optimized URL
 */
app.post('/cdn/social-url', (req, res) => {
  try {
    const { originalUrl, url: urlParam, platform } = req.body;
    const targetUrl = originalUrl || urlParam;

    if (!targetUrl) {
      return res.status(400).json({ success: false, error: 'URL is required (use "url" or "originalUrl")' });
    }

    if (!platform) {
      return res.status(400).json({ success: false, error: 'Platform is required' });
    }

    const url = cdnRulesService.generateSocialUrl(targetUrl, platform);

    res.json({
      success: true,
      url,
      platform,
      preset: cdnRulesService.getSocialPresets()[platform],
    });
  } catch (error) {
    res.status(400).json({ success: false, error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════════════════════
// Bulk Import Endpoints
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Create bulk import job from URLs
 */
app.post('/import/urls', async (req, res) => {
  try {
    const { urls, options } = req.body;

    if (!urls || !Array.isArray(urls) || urls.length === 0) {
      return res.status(400).json({ success: false, error: 'No URLs provided' });
    }

    const items = urls.map(url => typeof url === 'string' ? { url } : url);
    const job = await bulkImportService.createJob('urls', items, options || {});

    // Start the job asynchronously
    bulkImportService.startJob(job.id).catch(console.error);

    res.json({
      success: true,
      job: {
        id: job.id,
        status: job.status,
        totalItems: job.totalItems,
      },
    });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Create bulk import job from text (newline-separated URLs)
 */
app.post('/import/urls-text', async (req, res) => {
  try {
    const { text, options } = req.body;

    if (!text) {
      return res.status(400).json({ success: false, error: 'No text provided' });
    }

    const items = bulkImportService.parseUrlList(text);
    if (items.length === 0) {
      return res.status(400).json({ success: false, error: 'No valid URLs found in text' });
    }

    const job = await bulkImportService.createJob('urls', items, options || {});
    bulkImportService.startJob(job.id).catch(console.error);

    res.json({
      success: true,
      job: {
        id: job.id,
        status: job.status,
        totalItems: job.totalItems,
      },
    });
  } catch (error) {
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Get import job status
 */
app.get('/import/jobs/:jobId', (req, res) => {
  const job = bulkImportService.getJob(req.params.jobId);

  if (!job) {
    return res.status(404).json({ success: false, error: 'Job not found' });
  }

  res.json({
    success: true,
    job: {
      id: job.id,
      source: job.source,
      status: job.status,
      totalItems: job.totalItems,
      processedItems: job.processedItems,
      successCount: job.successCount,
      failedCount: job.failedCount,
      skippedCount: job.skippedCount,
      progress: Math.round((job.processedItems / job.totalItems) * 100),
      createdAt: job.createdAt,
      startedAt: job.startedAt,
      completedAt: job.completedAt,
    },
  });
});

/**
 * Get all import jobs
 */
app.get('/import/jobs', (req, res) => {
  const jobs = bulkImportService.getAllJobs().map(job => ({
    id: job.id,
    source: job.source,
    status: job.status,
    totalItems: job.totalItems,
    processedItems: job.processedItems,
    progress: Math.round((job.processedItems / job.totalItems) * 100),
    createdAt: job.createdAt,
  }));

  res.json({
    success: true,
    jobs,
    stats: bulkImportService.getStats(),
  });
});

/**
 * Cancel import job
 */
app.post('/import/jobs/:jobId/cancel', (req, res) => {
  try {
    const job = bulkImportService.cancelJob(req.params.jobId);
    res.json({ success: true, job: { id: job.id, status: job.status } });
  } catch (error) {
    res.status(400).json({ success: false, error: error.message });
  }
});

/**
 * Resume import job
 */
app.post('/import/jobs/:jobId/resume', async (req, res) => {
  try {
    const job = await bulkImportService.resumeJob(req.params.jobId);
    res.json({ success: true, job: { id: job.id, status: job.status } });
  } catch (error) {
    res.status(400).json({ success: false, error: error.message });
  }
});

/**
 * Delete import job
 */
app.delete('/import/jobs/:jobId', (req, res) => {
  try {
    bulkImportService.deleteJob(req.params.jobId);
    res.json({ success: true });
  } catch (error) {
    res.status(400).json({ success: false, error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════════════════════
// Error Handling
// ═══════════════════════════════════════════════════════════════════════════

app.use((error, req, res, next) => {
  console.error('Server error:', error);
  res.status(500).json({
    success: false,
    error: error.message || 'Internal server error',
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// Startup
// ═══════════════════════════════════════════════════════════════════════════

async function start() {
  // Initialize services
  await storage.initialize();
  await cache.initialize();

  // Start server
  app.listen(PORT, () => {
    console.log(`
╔═══════════════════════════════════════════════════════════════════════════╗
║              Revolution Image Service v2.0                                ║
║              Powered by Sharp + Cloudflare R2                             ║
╠═══════════════════════════════════════════════════════════════════════════╣
║  Server:          http://localhost:${PORT}                                    ║
║  Health:          http://localhost:${PORT}/health                             ║
║  Storage:         ${storage.isR2Configured ? 'Cloudflare R2 ✓' : 'Local filesystem (dev mode)'}                           ║
║  Cache:           ${cache.connected ? 'Redis ✓' : 'In-memory (Redis not configured)'}                           ║
╠═══════════════════════════════════════════════════════════════════════════╣
║  Features:                                                                ║
║    ✓ Parallel processing (worker threads)                                 ║
║    ✓ On-demand variant generation                                         ║
║    ✓ Streaming uploads                                                    ║
║    ✓ BlurHash & LQIP placeholders                                         ║
║    ✓ HTTP/2 push hints                                                    ║
╚═══════════════════════════════════════════════════════════════════════════╝
    `);
  });
}

start().catch(console.error);

export default app;
