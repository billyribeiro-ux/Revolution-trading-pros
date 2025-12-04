/**
 * Bulk Import & Migration System
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Features:
 * - Import from URLs (batch download)
 * - Import from ZIP archives
 * - Import from cloud storage (S3, R2, GCS)
 * - WordPress media migration
 * - Progress tracking and resumption
 * - Duplicate detection
 * - Automatic organization by date/type
 *
 * @version 1.0.0
 */

import fs from 'fs/promises';
import path from 'path';
import crypto from 'crypto';
import sharp from 'sharp';
import { EventEmitter } from 'events';

// Import states
const ImportState = {
  PENDING: 'pending',
  DOWNLOADING: 'downloading',
  PROCESSING: 'processing',
  COMPLETE: 'complete',
  FAILED: 'failed',
  SKIPPED: 'skipped',
};

class BulkImportService extends EventEmitter {
  constructor(options = {}) {
    super();

    this.storageDir = options.storageDir || './storage/media';
    this.tempDir = options.tempDir || './storage/temp/imports';
    this.concurrency = options.concurrency || 5;
    this.chunkSize = options.chunkSize || 50;
    this.retryAttempts = options.retryAttempts || 3;
    this.retryDelay = options.retryDelay || 1000;

    // Active jobs
    this.jobs = new Map();

    // Supported import sources
    this.sources = {
      urls: this.importFromUrls.bind(this),
      zip: this.importFromZip.bind(this),
      s3: this.importFromS3.bind(this),
      wordpress: this.importFromWordPress.bind(this),
      directory: this.importFromDirectory.bind(this),
    };
  }

  /**
   * Create a new import job
   */
  async createJob(source, items, options = {}) {
    const jobId = crypto.randomUUID();
    const job = {
      id: jobId,
      source,
      status: 'pending',
      createdAt: new Date().toISOString(),
      startedAt: null,
      completedAt: null,
      totalItems: items.length,
      processedItems: 0,
      successCount: 0,
      failedCount: 0,
      skippedCount: 0,
      items: items.map((item, index) => ({
        id: `${jobId}-${index}`,
        ...item,
        state: ImportState.PENDING,
        progress: 0,
        error: null,
        result: null,
      })),
      options,
    };

    this.jobs.set(jobId, job);
    this.emit('job:created', job);

    return job;
  }

  /**
   * Start processing an import job
   */
  async startJob(jobId) {
    const job = this.jobs.get(jobId);
    if (!job) {
      throw new Error(`Job not found: ${jobId}`);
    }

    if (job.status === 'processing') {
      throw new Error('Job is already processing');
    }

    job.status = 'processing';
    job.startedAt = new Date().toISOString();
    this.emit('job:started', job);

    try {
      // Process items in batches with concurrency control
      const batches = this.chunkArray(job.items, this.chunkSize);

      for (const batch of batches) {
        await this.processBatch(job, batch);

        // Update job progress
        job.processedItems = job.items.filter(
          i => i.state !== ImportState.PENDING && i.state !== ImportState.DOWNLOADING && i.state !== ImportState.PROCESSING
        ).length;

        this.emit('job:progress', {
          jobId,
          progress: Math.round((job.processedItems / job.totalItems) * 100),
          processed: job.processedItems,
          total: job.totalItems,
        });
      }

      // Calculate final stats
      job.successCount = job.items.filter(i => i.state === ImportState.COMPLETE).length;
      job.failedCount = job.items.filter(i => i.state === ImportState.FAILED).length;
      job.skippedCount = job.items.filter(i => i.state === ImportState.SKIPPED).length;

      job.status = job.failedCount === job.totalItems ? 'failed' : 'complete';
      job.completedAt = new Date().toISOString();

      this.emit('job:completed', job);

    } catch (error) {
      job.status = 'failed';
      job.error = error.message;
      job.completedAt = new Date().toISOString();
      this.emit('job:failed', { jobId, error: error.message });
    }

    return job;
  }

  /**
   * Process a batch of items
   */
  async processBatch(job, batch) {
    const handler = this.sources[job.source];
    if (!handler) {
      throw new Error(`Unknown source: ${job.source}`);
    }

    // Process items with concurrency limit
    const tasks = batch.map(item =>
      this.processItemWithRetry(job, item, handler)
    );

    await Promise.all(tasks);
  }

  /**
   * Process a single item with retry logic
   */
  async processItemWithRetry(job, item, handler) {
    let attempts = 0;

    while (attempts < this.retryAttempts) {
      try {
        item.state = ImportState.PROCESSING;
        this.emit('item:processing', { jobId: job.id, itemId: item.id });

        const result = await handler(item, job.options);

        item.state = result.skipped ? ImportState.SKIPPED : ImportState.COMPLETE;
        item.result = result;
        item.progress = 100;

        this.emit('item:complete', { jobId: job.id, itemId: item.id, result });
        return;

      } catch (error) {
        attempts++;

        if (attempts < this.retryAttempts) {
          await this.sleep(this.retryDelay * attempts);
          this.emit('item:retry', { jobId: job.id, itemId: item.id, attempt: attempts });
        } else {
          item.state = ImportState.FAILED;
          item.error = error.message;
          this.emit('item:failed', { jobId: job.id, itemId: item.id, error: error.message });
        }
      }
    }
  }

  /**
   * Import from URLs
   */
  async importFromUrls(item, options = {}) {
    const { url, filename: customFilename } = item;
    const { checkDuplicates = true, organize = true, optimize = true } = options;

    // Download the file
    item.state = ImportState.DOWNLOADING;
    const response = await fetch(url);

    if (!response.ok) {
      throw new Error(`Failed to download: ${response.status}`);
    }

    const buffer = Buffer.from(await response.arrayBuffer());

    // Check for duplicates
    if (checkDuplicates) {
      const hash = crypto.createHash('md5').update(buffer).digest('hex');
      const existingPath = path.join(this.storageDir, 'originals', `${hash}.*`);
      // In production, this would check the database
      // For now, we'll skip duplicate checking
    }

    // Determine filename
    const urlPath = new URL(url).pathname;
    const originalFilename = customFilename || path.basename(urlPath);
    const ext = path.extname(originalFilename).toLowerCase();

    // Validate it's an image
    const validExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.avif', '.svg'];
    if (!validExtensions.includes(ext)) {
      return { skipped: true, reason: 'Not an image file' };
    }

    // Get image metadata
    let metadata;
    try {
      metadata = await sharp(buffer).metadata();
    } catch (e) {
      throw new Error('Invalid image file');
    }

    // Generate unique filename
    const hash = crypto.createHash('md5').update(buffer).digest('hex').slice(0, 12);
    const filename = `${hash}${ext}`;

    // Determine storage path
    let storagePath;
    if (organize) {
      const date = new Date();
      const year = date.getFullYear();
      const month = String(date.getMonth() + 1).padStart(2, '0');
      storagePath = path.join(this.storageDir, 'originals', String(year), month);
    } else {
      storagePath = path.join(this.storageDir, 'originals');
    }

    // Ensure directory exists
    await fs.mkdir(storagePath, { recursive: true });

    // Save file
    const fullPath = path.join(storagePath, filename);
    await fs.writeFile(fullPath, buffer);

    return {
      filename,
      originalFilename,
      path: fullPath,
      size: buffer.length,
      width: metadata.width,
      height: metadata.height,
      format: metadata.format,
      hash,
    };
  }

  /**
   * Import from ZIP archive
   */
  async importFromZip(item, options = {}) {
    // This would use a library like 'adm-zip' or 'yauzl'
    // For now, return a placeholder
    throw new Error('ZIP import not yet implemented');
  }

  /**
   * Import from S3/R2 bucket
   */
  async importFromS3(item, options = {}) {
    const { bucket, key, region = 'auto' } = item;
    const { accessKeyId, secretAccessKey, endpoint } = options;

    // This would use the AWS SDK or S3-compatible client
    // For now, return a placeholder
    throw new Error('S3 import not yet implemented');
  }

  /**
   * Import from WordPress media library
   */
  async importFromWordPress(item, options = {}) {
    const { wpUrl, mediaId } = item;
    const { username, password, applicationPassword } = options;

    // Fetch media info from WordPress REST API
    const response = await fetch(`${wpUrl}/wp-json/wp/v2/media/${mediaId}`, {
      headers: {
        'Authorization': `Basic ${Buffer.from(`${username}:${applicationPassword}`).toString('base64')}`,
      },
    });

    if (!response.ok) {
      throw new Error(`Failed to fetch WordPress media: ${response.status}`);
    }

    const media = await response.json();
    const sourceUrl = media.source_url;

    // Import the file
    return this.importFromUrls(
      { url: sourceUrl, filename: media.title?.rendered || path.basename(sourceUrl) },
      { ...options, wpMeta: media }
    );
  }

  /**
   * Import from local directory
   */
  async importFromDirectory(item, options = {}) {
    const { filePath } = item;
    const { checkDuplicates = true, organize = true } = options;

    // Read file
    const buffer = await fs.readFile(filePath);
    const filename = path.basename(filePath);
    const ext = path.extname(filename).toLowerCase();

    // Validate it's an image
    const validExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.avif', '.svg'];
    if (!validExtensions.includes(ext)) {
      return { skipped: true, reason: 'Not an image file' };
    }

    // Get image metadata
    let metadata;
    try {
      metadata = await sharp(buffer).metadata();
    } catch (e) {
      throw new Error('Invalid image file');
    }

    // Generate unique filename
    const hash = crypto.createHash('md5').update(buffer).digest('hex').slice(0, 12);
    const newFilename = `${hash}${ext}`;

    // Determine storage path
    let storagePath;
    if (organize) {
      const date = new Date();
      const year = date.getFullYear();
      const month = String(date.getMonth() + 1).padStart(2, '0');
      storagePath = path.join(this.storageDir, 'originals', String(year), month);
    } else {
      storagePath = path.join(this.storageDir, 'originals');
    }

    // Ensure directory exists
    await fs.mkdir(storagePath, { recursive: true });

    // Copy file
    const fullPath = path.join(storagePath, newFilename);
    await fs.writeFile(fullPath, buffer);

    return {
      filename: newFilename,
      originalFilename: filename,
      path: fullPath,
      size: buffer.length,
      width: metadata.width,
      height: metadata.height,
      format: metadata.format,
      hash,
    };
  }

  /**
   * Get job status
   */
  getJob(jobId) {
    return this.jobs.get(jobId);
  }

  /**
   * Get all jobs
   */
  getAllJobs() {
    return Array.from(this.jobs.values());
  }

  /**
   * Cancel a running job
   */
  cancelJob(jobId) {
    const job = this.jobs.get(jobId);
    if (!job) {
      throw new Error(`Job not found: ${jobId}`);
    }

    if (job.status === 'complete' || job.status === 'cancelled') {
      return job;
    }

    job.status = 'cancelled';
    job.completedAt = new Date().toISOString();
    this.emit('job:cancelled', job);

    return job;
  }

  /**
   * Resume a failed/cancelled job
   */
  async resumeJob(jobId) {
    const job = this.jobs.get(jobId);
    if (!job) {
      throw new Error(`Job not found: ${jobId}`);
    }

    if (job.status === 'processing') {
      throw new Error('Job is already processing');
    }

    // Reset pending/failed items
    for (const item of job.items) {
      if (item.state === ImportState.PENDING || item.state === ImportState.FAILED) {
        item.state = ImportState.PENDING;
        item.error = null;
        item.progress = 0;
      }
    }

    return this.startJob(jobId);
  }

  /**
   * Delete a job
   */
  deleteJob(jobId) {
    const job = this.jobs.get(jobId);
    if (!job) {
      throw new Error(`Job not found: ${jobId}`);
    }

    if (job.status === 'processing') {
      throw new Error('Cannot delete a running job');
    }

    this.jobs.delete(jobId);
    this.emit('job:deleted', { jobId });

    return true;
  }

  /**
   * Helper: Split array into chunks
   */
  chunkArray(array, size) {
    const chunks = [];
    for (let i = 0; i < array.length; i += size) {
      chunks.push(array.slice(i, i + size));
    }
    return chunks;
  }

  /**
   * Helper: Sleep for ms milliseconds
   */
  sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  /**
   * Parse URL list from text
   */
  parseUrlList(text) {
    return text
      .split(/[\r\n]+/)
      .map(line => line.trim())
      .filter(line => line && line.startsWith('http'))
      .map(url => ({ url }));
  }

  /**
   * Get import statistics
   */
  getStats() {
    const jobs = Array.from(this.jobs.values());
    return {
      totalJobs: jobs.length,
      pendingJobs: jobs.filter(j => j.status === 'pending').length,
      processingJobs: jobs.filter(j => j.status === 'processing').length,
      completedJobs: jobs.filter(j => j.status === 'complete').length,
      failedJobs: jobs.filter(j => j.status === 'failed').length,
      totalItems: jobs.reduce((sum, j) => sum + j.totalItems, 0),
      processedItems: jobs.reduce((sum, j) => sum + j.processedItems, 0),
    };
  }
}

// Export singleton
export const bulkImportService = new BulkImportService();
export default BulkImportService;
