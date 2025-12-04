/**
 * Image Processing Worker
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Worker thread for parallel image processing using Sharp.
 * Handles WebP, AVIF, responsive sizes, and thumbnail generation.
 */

import { parentPort, workerData } from 'worker_threads';
import sharp from 'sharp';

const QUALITY_PRESETS = {
  maximum: { webp: 95, avif: 90, jpeg: 95 },
  balanced: { webp: 85, avif: 80, jpeg: 85 },
  performance: { webp: 75, avif: 70, jpeg: 75 },
  thumbnail: { webp: 70, avif: 65, jpeg: 70 },
};

/**
 * Process image based on task type
 */
async function processTask(task) {
  const { type, buffer, options } = task;
  const imageBuffer = Buffer.from(buffer);
  const quality = QUALITY_PRESETS[options.preset] || QUALITY_PRESETS.balanced;

  switch (type) {
    case 'webp':
      return generateWebp(imageBuffer, quality, options);
    case 'avif':
      return generateAvif(imageBuffer, quality, options);
    case 'responsive':
      return generateResponsive(imageBuffer, quality, options);
    case 'thumbnail':
      return generateThumbnail(imageBuffer, quality, options);
    case 'blurhash':
      return generateBlurhashData(imageBuffer);
    case 'lqip':
      return generateLqip(imageBuffer);
    case 'metadata':
      return getMetadata(imageBuffer);
    default:
      throw new Error(`Unknown task type: ${type}`);
  }
}

/**
 * Generate WebP variant
 */
async function generateWebp(buffer, quality, options) {
  const webpBuffer = await sharp(buffer)
    .webp({ quality: quality.webp, effort: 4 })
    .toBuffer();

  const metadata = await sharp(webpBuffer).metadata();

  return {
    type: 'webp',
    buffer: webpBuffer,
    width: metadata.width,
    height: metadata.height,
    size: webpBuffer.length,
  };
}

/**
 * Generate AVIF variant
 */
async function generateAvif(buffer, quality, options) {
  try {
    const avifBuffer = await sharp(buffer)
      .avif({ quality: quality.avif, effort: 4 })
      .toBuffer();

    const metadata = await sharp(avifBuffer).metadata();

    return {
      type: 'avif',
      buffer: avifBuffer,
      width: metadata.width,
      height: metadata.height,
      size: avifBuffer.length,
    };
  } catch (error) {
    console.warn('AVIF generation failed:', error.message);
    return { type: 'avif', error: error.message };
  }
}

/**
 * Generate all responsive sizes
 */
async function generateResponsive(buffer, quality, options) {
  const sizes = options.sizes || {
    xs: 320,
    sm: 640,
    md: 768,
    lg: 1024,
    xl: 1280,
    '2xl': 1920,
  };

  const originalMeta = await sharp(buffer).metadata();
  const results = [];

  // Process all sizes in parallel within the worker
  const sizePromises = Object.entries(sizes).map(async ([sizeName, targetWidth]) => {
    // Skip if larger than original
    if (targetWidth >= originalMeta.width) return null;

    const resizedBuffer = await sharp(buffer)
      .resize(targetWidth, null, { withoutEnlargement: true })
      .webp({ quality: quality.webp, effort: 4 })
      .toBuffer();

    const metadata = await sharp(resizedBuffer).metadata();

    return {
      sizeName,
      buffer: resizedBuffer,
      width: metadata.width,
      height: metadata.height,
      size: resizedBuffer.length,
    };
  });

  const sizeResults = await Promise.all(sizePromises);
  return sizeResults.filter(Boolean);
}

/**
 * Generate thumbnail
 */
async function generateThumbnail(buffer, quality, options) {
  const size = options.thumbnailSize || 300;

  const thumbBuffer = await sharp(buffer)
    .resize(size, size, { fit: 'cover', position: 'center' })
    .webp({ quality: quality.thumbnail || 80 })
    .toBuffer();

  return {
    type: 'thumbnail',
    buffer: thumbBuffer,
    width: size,
    height: size,
    size: thumbBuffer.length,
  };
}

/**
 * Generate BlurHash data (raw pixel data for encoding)
 */
async function generateBlurhashData(buffer) {
  const { data, info } = await sharp(buffer)
    .resize(32, 32, { fit: 'inside' })
    .ensureAlpha()
    .raw()
    .toBuffer({ resolveWithObject: true });

  return {
    type: 'blurhash',
    data: Array.from(data),
    width: info.width,
    height: info.height,
  };
}

/**
 * Generate LQIP (Low Quality Image Placeholder)
 */
async function generateLqip(buffer) {
  const lqipBuffer = await sharp(buffer)
    .resize(32, null, { withoutEnlargement: true })
    .blur(10)
    .jpeg({ quality: 20 })
    .toBuffer();

  return {
    type: 'lqip',
    base64: `data:image/jpeg;base64,${lqipBuffer.toString('base64')}`,
  };
}

/**
 * Get image metadata
 */
async function getMetadata(buffer) {
  const metadata = await sharp(buffer).metadata();
  return {
    type: 'metadata',
    width: metadata.width,
    height: metadata.height,
    format: metadata.format,
    size: buffer.length,
    hasAlpha: metadata.hasAlpha,
    orientation: metadata.orientation,
  };
}

// Handle messages from parent
parentPort.on('message', async (task) => {
  try {
    const result = await processTask(task);
    parentPort.postMessage({ success: true, taskId: task.taskId, result });
  } catch (error) {
    parentPort.postMessage({ success: false, taskId: task.taskId, error: error.message });
  }
});
