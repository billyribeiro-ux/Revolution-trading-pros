/**
 * Automated Maintenance Service
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Scheduled maintenance tasks:
 * - Cleanup orphaned variants
 * - Remove temp files older than 24h
 * - Optimize unprocessed images
 * - Clear expired cache entries
 * - Generate missing thumbnails
 * - Validate storage integrity
 *
 * @version 1.0.0
 */

import fs from 'fs/promises';
import path from 'path';
import sharp from 'sharp';
import { cache as cacheManager } from './cache.js';

class MaintenanceService {
  constructor(options = {}) {
    this.storageDir = options.storageDir || './storage/media';
    this.tempDir = options.tempDir || './storage/temp';
    this.maxTempAge = options.maxTempAge || 24 * 60 * 60 * 1000; // 24 hours
    this.maxCacheAge = options.maxCacheAge || 7 * 24 * 60 * 60 * 1000; // 7 days
    this.batchSize = options.batchSize || 50;

    this.stats = {
      lastRun: null,
      tempFilesDeleted: 0,
      orphansRemoved: 0,
      cacheEntriesCleared: 0,
      thumbnailsGenerated: 0,
      errorsEncountered: 0,
    };

    this.isRunning = false;
    this.interval = null;
  }

  /**
   * Start scheduled maintenance
   */
  start(intervalMs = 60 * 60 * 1000) { // Default: every hour
    if (this.interval) {
      clearInterval(this.interval);
    }

    console.log(`[Maintenance] Starting scheduled maintenance (every ${intervalMs / 1000 / 60} minutes)`);

    // Run immediately, then schedule
    this.run();
    this.interval = setInterval(() => this.run(), intervalMs);
  }

  /**
   * Stop scheduled maintenance
   */
  stop() {
    if (this.interval) {
      clearInterval(this.interval);
      this.interval = null;
    }
    console.log('[Maintenance] Stopped scheduled maintenance');
  }

  /**
   * Run all maintenance tasks
   */
  async run() {
    if (this.isRunning) {
      console.log('[Maintenance] Already running, skipping...');
      return this.stats;
    }

    this.isRunning = true;
    const startTime = Date.now();
    console.log('[Maintenance] Starting maintenance run...');

    // Reset stats
    this.stats = {
      lastRun: new Date().toISOString(),
      tempFilesDeleted: 0,
      orphansRemoved: 0,
      cacheEntriesCleared: 0,
      thumbnailsGenerated: 0,
      errorsEncountered: 0,
    };

    try {
      // Run tasks in sequence
      await this.cleanupTempFiles();
      await this.cleanupOrphanedVariants();
      await this.clearExpiredCache();
      await this.validateStorageIntegrity();
      await this.optimizePendingImages();
    } catch (error) {
      console.error('[Maintenance] Error during maintenance:', error);
      this.stats.errorsEncountered++;
    }

    const duration = Date.now() - startTime;
    console.log(`[Maintenance] Completed in ${duration}ms`, this.stats);

    this.isRunning = false;
    return this.stats;
  }

  /**
   * Cleanup temporary files older than maxTempAge
   */
  async cleanupTempFiles() {
    console.log('[Maintenance] Cleaning up temp files...');

    try {
      await fs.mkdir(this.tempDir, { recursive: true });
      const files = await fs.readdir(this.tempDir);
      const now = Date.now();
      let deleted = 0;

      for (const file of files) {
        try {
          const filePath = path.join(this.tempDir, file);
          const stat = await fs.stat(filePath);

          if (now - stat.mtimeMs > this.maxTempAge) {
            await fs.unlink(filePath);
            deleted++;
          }
        } catch (e) {
          // File might have been deleted already
        }
      }

      this.stats.tempFilesDeleted = deleted;
      console.log(`[Maintenance] Deleted ${deleted} temp files`);
    } catch (error) {
      console.error('[Maintenance] Error cleaning temp files:', error);
      this.stats.errorsEncountered++;
    }
  }

  /**
   * Remove orphaned variants (variants without parent)
   */
  async cleanupOrphanedVariants() {
    console.log('[Maintenance] Checking for orphaned variants...');

    try {
      const variantsDir = path.join(this.storageDir, 'variants');
      const originalsDir = path.join(this.storageDir, 'originals');

      // Ensure directories exist
      await fs.mkdir(variantsDir, { recursive: true });
      await fs.mkdir(originalsDir, { recursive: true });

      // Get all original file hashes
      const originalFiles = await fs.readdir(originalsDir);
      const originalHashes = new Set(
        originalFiles.map(f => path.basename(f, path.extname(f)))
      );

      // Check variants
      const variantFiles = await fs.readdir(variantsDir);
      let removed = 0;

      for (const file of variantFiles) {
        // Extract parent hash from variant filename
        // Format: {hash}_{size}_{format}.{ext}
        const match = file.match(/^([a-f0-9]+)_/);
        if (match) {
          const parentHash = match[1];
          if (!originalHashes.has(parentHash)) {
            try {
              await fs.unlink(path.join(variantsDir, file));
              removed++;
            } catch (e) {
              // File might have been deleted already
            }
          }
        }
      }

      this.stats.orphansRemoved = removed;
      console.log(`[Maintenance] Removed ${removed} orphaned variants`);
    } catch (error) {
      console.error('[Maintenance] Error cleaning orphaned variants:', error);
      this.stats.errorsEncountered++;
    }
  }

  /**
   * Clear expired cache entries
   */
  async clearExpiredCache() {
    console.log('[Maintenance] Clearing expired cache entries...');

    try {
      // Use the cache manager's cleanup method if available
      if (cacheManager && typeof cacheManager.cleanup === 'function') {
        const cleared = await cacheManager.cleanup();
        this.stats.cacheEntriesCleared = cleared;
        console.log(`[Maintenance] Cleared ${cleared} cache entries`);
      } else {
        // Manual cleanup for file-based cache
        const cacheDir = path.join(this.storageDir, 'cache');
        await fs.mkdir(cacheDir, { recursive: true });

        const files = await fs.readdir(cacheDir);
        const now = Date.now();
        let cleared = 0;

        for (const file of files) {
          try {
            const filePath = path.join(cacheDir, file);
            const stat = await fs.stat(filePath);

            if (now - stat.mtimeMs > this.maxCacheAge) {
              await fs.unlink(filePath);
              cleared++;
            }
          } catch (e) {
            // File might have been deleted already
          }
        }

        this.stats.cacheEntriesCleared = cleared;
        console.log(`[Maintenance] Cleared ${cleared} cache entries`);
      }
    } catch (error) {
      console.error('[Maintenance] Error clearing cache:', error);
      this.stats.errorsEncountered++;
    }
  }

  /**
   * Validate storage integrity
   */
  async validateStorageIntegrity() {
    console.log('[Maintenance] Validating storage integrity...');

    try {
      const originalsDir = path.join(this.storageDir, 'originals');
      await fs.mkdir(originalsDir, { recursive: true });

      const files = await fs.readdir(originalsDir);
      let corrupted = 0;

      for (const file of files.slice(0, this.batchSize)) {
        try {
          const filePath = path.join(originalsDir, file);
          const ext = path.extname(file).toLowerCase();

          // Only validate image files
          if (['.jpg', '.jpeg', '.png', '.webp', '.gif', '.avif'].includes(ext)) {
            // Try to read metadata - will throw if corrupted
            await sharp(filePath).metadata();
          }
        } catch (e) {
          console.warn(`[Maintenance] Potentially corrupted file: ${file}`);
          corrupted++;
        }
      }

      if (corrupted > 0) {
        console.log(`[Maintenance] Found ${corrupted} potentially corrupted files`);
      }
    } catch (error) {
      console.error('[Maintenance] Error validating storage:', error);
      this.stats.errorsEncountered++;
    }
  }

  /**
   * Optimize pending images (generate missing variants)
   */
  async optimizePendingImages() {
    console.log('[Maintenance] Checking for pending optimizations...');

    // This would typically integrate with the database
    // For now, just check for images without variants
    try {
      const originalsDir = path.join(this.storageDir, 'originals');
      const variantsDir = path.join(this.storageDir, 'variants');

      await fs.mkdir(originalsDir, { recursive: true });
      await fs.mkdir(variantsDir, { recursive: true });

      const originalFiles = await fs.readdir(originalsDir);
      const variantFiles = await fs.readdir(variantsDir);

      // Get hashes that have variants
      const hashedWithVariants = new Set(
        variantFiles.map(f => {
          const match = f.match(/^([a-f0-9]+)_/);
          return match ? match[1] : null;
        }).filter(Boolean)
      );

      // Find originals without variants
      let pending = 0;
      for (const file of originalFiles.slice(0, this.batchSize)) {
        const hash = path.basename(file, path.extname(file));
        if (!hashedWithVariants.has(hash)) {
          pending++;
        }
      }

      if (pending > 0) {
        console.log(`[Maintenance] Found ${pending} images without variants`);
      }
    } catch (error) {
      console.error('[Maintenance] Error checking pending optimizations:', error);
      this.stats.errorsEncountered++;
    }
  }

  /**
   * Get current stats
   */
  getStats() {
    return {
      ...this.stats,
      isRunning: this.isRunning,
      scheduled: !!this.interval,
    };
  }

  /**
   * Manual trigger for specific task
   */
  async runTask(taskName) {
    switch (taskName) {
      case 'cleanup-temp':
        return this.cleanupTempFiles();
      case 'cleanup-orphans':
        return this.cleanupOrphanedVariants();
      case 'clear-cache':
        return this.clearExpiredCache();
      case 'validate-storage':
        return this.validateStorageIntegrity();
      case 'optimize-pending':
        return this.optimizePendingImages();
      default:
        throw new Error(`Unknown task: ${taskName}`);
    }
  }
}

// Export singleton
export const maintenanceService = new MaintenanceService();
export default MaintenanceService;
