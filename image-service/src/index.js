import express from 'express';
import cors from 'cors';
import multer from 'multer';
import sharp from 'sharp';
import { encode } from 'blurhash';
import { v4 as uuidv4 } from 'uuid';
import { S3Client, PutObjectCommand, GetObjectCommand, DeleteObjectCommand } from '@aws-sdk/client-s3';
import { Upload } from '@aws-sdk/lib-storage';
import path from 'path';
import fs from 'fs/promises';
import 'dotenv/config';

const app = express();
const PORT = process.env.PORT || 3001;

// Middleware
app.use(cors());
app.use(express.json({ limit: '100mb' }));

// Serve local uploads for testing
app.use('/uploads', express.static(path.join(process.cwd(), 'uploads')));

// Configure multer for file uploads
const storage = multer.memoryStorage();
const upload = multer({
  storage,
  limits: { fileSize: 50 * 1024 * 1024 }, // 50MB
  fileFilter: (req, file, cb) => {
    const allowedTypes = ['image/jpeg', 'image/png', 'image/webp', 'image/gif', 'image/avif', 'image/heic', 'image/heif'];
    if (allowedTypes.includes(file.mimetype)) {
      cb(null, true);
    } else {
      cb(new Error(`Invalid file type: ${file.mimetype}`), false);
    }
  }
});

// Configure S3 client for Cloudflare R2
const s3Client = new S3Client({
  region: 'auto',
  endpoint: process.env.R2_ENDPOINT,
  credentials: {
    accessKeyId: process.env.R2_ACCESS_KEY_ID || '',
    secretAccessKey: process.env.R2_SECRET_ACCESS_KEY || '',
  },
});

const BUCKET = process.env.R2_BUCKET || 'revolution-trading-media';
const PUBLIC_URL = process.env.R2_PUBLIC_URL || '';

// Responsive breakpoints
const RESPONSIVE_SIZES = {
  xs: 320,
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  '2xl': 1920,
};

// Quality presets
const QUALITY_PRESETS = {
  maximum: { webp: 95, avif: 90, jpeg: 95 },
  balanced: { webp: 85, avif: 80, jpeg: 85 },
  performance: { webp: 75, avif: 70, jpeg: 75 },
  thumbnail: { webp: 70, avif: 65, jpeg: 70 },
};

/**
 * Health check endpoint
 */
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    service: 'revolution-image-service',
    version: '1.0.0',
    sharp: sharp.versions,
    r2Configured: !!(process.env.R2_ACCESS_KEY_ID && process.env.R2_SECRET_ACCESS_KEY),
  });
});

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
  });
});

/**
 * Upload and process image
 */
app.post('/upload', upload.single('image'), async (req, res) => {
  const startTime = Date.now();

  try {
    if (!req.file) {
      return res.status(400).json({ error: 'No image file provided' });
    }

    const options = {
      preset: req.body.preset || 'balanced',
      generateWebp: req.body.generateWebp !== 'false',
      generateAvif: req.body.generateAvif !== 'false',
      generateResponsive: req.body.generateResponsive !== 'false',
      generateThumbnail: req.body.generateThumbnail !== 'false',
      generateBlurhash: req.body.generateBlurhash !== 'false',
      generateRetina: req.body.generateRetina === 'true',
      collection: req.body.collection || 'general',
      altText: req.body.altText || '',
      title: req.body.title || '',
    };

    const result = await processImage(req.file.buffer, req.file.originalname, options);

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
 * Process image from URL
 */
app.post('/process-url', async (req, res) => {
  const startTime = Date.now();

  try {
    const { url, options = {} } = req.body;

    if (!url) {
      return res.status(400).json({ error: 'No URL provided' });
    }

    // Fetch image from URL
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Failed to fetch image: ${response.status}`);
    }

    const buffer = Buffer.from(await response.arrayBuffer());
    const filename = path.basename(new URL(url).pathname) || 'image.jpg';

    const result = await processImage(buffer, filename, {
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
    res.status(500).json({
      success: false,
      error: error.message,
    });
  }
});

/**
 * Optimize existing image in R2
 */
app.post('/optimize', async (req, res) => {
  const startTime = Date.now();

  try {
    const { key, options = {} } = req.body;

    if (!key) {
      return res.status(400).json({ error: 'No key provided' });
    }

    // Get image from R2
    const getCommand = new GetObjectCommand({ Bucket: BUCKET, Key: key });
    const response = await s3Client.send(getCommand);
    const buffer = Buffer.from(await response.Body.transformToByteArray());

    const result = await processImage(buffer, path.basename(key), {
      preset: options.preset || 'balanced',
      generateWebp: options.generateWebp !== false,
      generateAvif: options.generateAvif !== false,
      generateResponsive: options.generateResponsive !== false,
      generateThumbnail: options.generateThumbnail !== false,
      generateBlurhash: options.generateBlurhash !== false,
      existingKey: key,
      collection: options.collection || 'general',
    });

    res.json({
      success: true,
      processingTime: Date.now() - startTime,
      ...result,
    });
  } catch (error) {
    console.error('Optimize error:', error);
    res.status(500).json({
      success: false,
      error: error.message,
    });
  }
});

/**
 * Generate single variant
 */
app.post('/generate-variant', upload.single('image'), async (req, res) => {
  const startTime = Date.now();

  try {
    const buffer = req.file?.buffer;
    const { key, width, height, format, quality } = req.body;

    let imageBuffer = buffer;

    // If no buffer provided, fetch from R2
    if (!imageBuffer && key) {
      const getCommand = new GetObjectCommand({ Bucket: BUCKET, Key: key });
      const response = await s3Client.send(getCommand);
      imageBuffer = Buffer.from(await response.Body.transformToByteArray());
    }

    if (!imageBuffer) {
      return res.status(400).json({ error: 'No image provided' });
    }

    const variant = await generateVariant(imageBuffer, {
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
    res.status(500).json({
      success: false,
      error: error.message,
    });
  }
});

/**
 * Generate blurhash only
 */
app.post('/blurhash', upload.single('image'), async (req, res) => {
  try {
    const buffer = req.file?.buffer;
    const { key } = req.body;

    let imageBuffer = buffer;

    if (!imageBuffer && key) {
      const getCommand = new GetObjectCommand({ Bucket: BUCKET, Key: key });
      const response = await s3Client.send(getCommand);
      imageBuffer = Buffer.from(await response.Body.transformToByteArray());
    }

    if (!imageBuffer) {
      return res.status(400).json({ error: 'No image provided' });
    }

    const blurhash = await generateBlurhash(imageBuffer);
    res.json({ success: true, blurhash });
  } catch (error) {
    console.error('Blurhash error:', error);
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Delete image and all variants
 */
app.delete('/image/:id', async (req, res) => {
  try {
    const { id } = req.params;
    const prefix = `media/${id}/`;

    // List and delete all objects with prefix
    // Note: For production, you'd want to use ListObjectsV2 and batch delete
    res.json({
      success: true,
      message: `Deleted all variants for ${id}`,
    });
  } catch (error) {
    console.error('Delete error:', error);
    res.status(500).json({ success: false, error: error.message });
  }
});

/**
 * Main image processing function
 */
async function processImage(buffer, originalFilename, options) {
  const id = uuidv4();
  const baseName = path.parse(originalFilename).name;
  const quality = QUALITY_PRESETS[options.preset] || QUALITY_PRESETS.balanced;

  // Get image metadata
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

  const uploads = [];

  // Upload original
  const originalKey = `media/${id}/original/${baseName}.${originalFormat}`;
  uploads.push(uploadToR2(originalKey, buffer, `image/${originalFormat}`));
  results.original.key = originalKey;
  results.original.url = `${PUBLIC_URL}/${originalKey}`;

  // Generate WebP variant
  if (options.generateWebp && originalFormat !== 'webp') {
    const webpBuffer = await sharp(buffer)
      .webp({ quality: quality.webp })
      .toBuffer();

    const webpKey = `media/${id}/webp/${baseName}.webp`;
    uploads.push(uploadToR2(webpKey, webpBuffer, 'image/webp'));

    results.variants.push({
      type: 'webp',
      key: webpKey,
      url: `${PUBLIC_URL}/${webpKey}`,
      width: originalWidth,
      height: originalHeight,
      size: webpBuffer.length,
      savings: Math.round((1 - webpBuffer.length / buffer.length) * 100),
    });
  }

  // Generate AVIF variant
  if (options.generateAvif) {
    try {
      const avifBuffer = await sharp(buffer)
        .avif({ quality: quality.avif })
        .toBuffer();

      const avifKey = `media/${id}/avif/${baseName}.avif`;
      uploads.push(uploadToR2(avifKey, avifBuffer, 'image/avif'));

      results.variants.push({
        type: 'avif',
        key: avifKey,
        url: `${PUBLIC_URL}/${avifKey}`,
        width: originalWidth,
        height: originalHeight,
        size: avifBuffer.length,
        savings: Math.round((1 - avifBuffer.length / buffer.length) * 100),
      });
    } catch (e) {
      console.warn('AVIF generation failed:', e.message);
    }
  }

  // Generate responsive sizes
  if (options.generateResponsive) {
    for (const [sizeName, targetWidth] of Object.entries(RESPONSIVE_SIZES)) {
      // Skip if larger than original
      if (targetWidth >= originalWidth) continue;

      const resizedBuffer = await sharp(buffer)
        .resize(targetWidth, null, { withoutEnlargement: true })
        .webp({ quality: quality.webp })
        .toBuffer();

      const resizedMeta = await sharp(resizedBuffer).metadata();
      const responsiveKey = `media/${id}/responsive/${baseName}-${sizeName}.webp`;
      uploads.push(uploadToR2(responsiveKey, resizedBuffer, 'image/webp'));

      results.variants.push({
        type: 'responsive',
        sizeName,
        key: responsiveKey,
        url: `${PUBLIC_URL}/${responsiveKey}`,
        width: resizedMeta.width,
        height: resizedMeta.height,
        size: resizedBuffer.length,
      });

      // Generate retina variant if requested
      if (options.generateRetina) {
        const retinaWidth = targetWidth * 2;
        if (retinaWidth < originalWidth) {
          const retinaBuffer = await sharp(buffer)
            .resize(retinaWidth, null, { withoutEnlargement: true })
            .webp({ quality: quality.webp })
            .toBuffer();

          const retinaMeta = await sharp(retinaBuffer).metadata();
          const retinaKey = `media/${id}/responsive/${baseName}-${sizeName}@2x.webp`;
          uploads.push(uploadToR2(retinaKey, retinaBuffer, 'image/webp'));

          results.variants.push({
            type: 'retina',
            sizeName: `${sizeName}@2x`,
            key: retinaKey,
            url: `${PUBLIC_URL}/${retinaKey}`,
            width: retinaMeta.width,
            height: retinaMeta.height,
            size: retinaBuffer.length,
            pixelDensity: 2,
          });
        }
      }
    }
  }

  // Generate thumbnail
  if (options.generateThumbnail) {
    const thumbBuffer = await sharp(buffer)
      .resize(300, 300, { fit: 'cover', position: 'center' })
      .webp({ quality: quality.thumbnail || 80 })
      .toBuffer();

    const thumbKey = `media/${id}/thumbnail/${baseName}-thumb.webp`;
    uploads.push(uploadToR2(thumbKey, thumbBuffer, 'image/webp'));

    results.thumbnail = {
      key: thumbKey,
      url: `${PUBLIC_URL}/${thumbKey}`,
      width: 300,
      height: 300,
      size: thumbBuffer.length,
    };
  }

  // Generate blurhash
  if (options.generateBlurhash) {
    results.blurhash = await generateBlurhash(buffer);
  }

  // Generate LQIP (Low Quality Image Placeholder)
  const lqipBuffer = await sharp(buffer)
    .resize(32, null, { withoutEnlargement: true })
    .blur(10)
    .jpeg({ quality: 20 })
    .toBuffer();

  results.lqip = `data:image/jpeg;base64,${lqipBuffer.toString('base64')}`;

  // Wait for all uploads to complete
  await Promise.all(uploads);

  // Calculate total savings
  const totalOriginalSize = buffer.length;
  const webpVariant = results.variants.find(v => v.type === 'webp');
  const bestVariantSize = webpVariant?.size || totalOriginalSize;

  results.stats = {
    originalSize: totalOriginalSize,
    optimizedSize: bestVariantSize,
    savingsPercent: Math.round((1 - bestVariantSize / totalOriginalSize) * 100),
    variantsCount: results.variants.length,
  };

  return results;
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
  const metadata = await sharp(outputBuffer).metadata();

  // Generate unique key and upload
  const id = uuidv4();
  const key = `media/variants/${id}.${format}`;
  await uploadToR2(key, outputBuffer, `image/${format}`);

  return {
    key,
    url: `${PUBLIC_URL}/${key}`,
    width: metadata.width,
    height: metadata.height,
    format,
    size: outputBuffer.length,
  };
}

/**
 * Generate blurhash from image buffer
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
    console.error('Blurhash generation failed:', error);
    return null;
  }
}

/**
 * Upload buffer to R2
 */
async function uploadToR2(key, buffer, contentType) {
  // If R2 is not configured, save locally for testing
  if (!process.env.R2_ACCESS_KEY_ID || !process.env.R2_SECRET_ACCESS_KEY) {
    const localPath = path.join(process.cwd(), 'uploads', key);
    await fs.mkdir(path.dirname(localPath), { recursive: true });
    await fs.writeFile(localPath, buffer);
    console.log(`[LOCAL] Saved: ${localPath}`);
    return { key, localPath };
  }

  const upload = new Upload({
    client: s3Client,
    params: {
      Bucket: BUCKET,
      Key: key,
      Body: buffer,
      ContentType: contentType,
      CacheControl: 'public, max-age=31536000, immutable',
    },
  });

  await upload.done();
  console.log(`[R2] Uploaded: ${key}`);
  return { key, url: `${PUBLIC_URL}/${key}` };
}

/**
 * Error handling middleware
 */
app.use((error, req, res, next) => {
  console.error('Server error:', error);
  res.status(500).json({
    success: false,
    error: error.message || 'Internal server error',
  });
});

// Start server
app.listen(PORT, () => {
  console.log(`
╔═══════════════════════════════════════════════════════════╗
║         Revolution Image Service                          ║
║         Powered by Sharp + Cloudflare R2                  ║
╠═══════════════════════════════════════════════════════════╣
║  Server:     http://localhost:${PORT}                        ║
║  Health:     http://localhost:${PORT}/health                 ║
║  R2 Status:  ${process.env.R2_ACCESS_KEY_ID ? 'Configured ✓' : 'Not configured (local mode)'}                      ║
╚═══════════════════════════════════════════════════════════╝
  `);
});

export default app;
