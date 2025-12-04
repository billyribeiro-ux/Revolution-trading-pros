/**
 * Storage Manager
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Handles file storage with support for:
 * - Cloudflare R2 (S3-compatible)
 * - Streaming uploads (no memory buffering limits)
 * - Local filesystem fallback for development
 */

import { S3Client, PutObjectCommand, GetObjectCommand, DeleteObjectCommand, HeadObjectCommand } from '@aws-sdk/client-s3';
import { Upload } from '@aws-sdk/lib-storage';
import { getSignedUrl } from '@aws-sdk/s3-request-presigner';
import fs from 'fs/promises';
import { createWriteStream, createReadStream } from 'fs';
import path from 'path';
import { pipeline } from 'stream/promises';

class StorageManager {
  constructor() {
    this.s3Client = null;
    this.bucket = process.env.R2_BUCKET || 'revolution-trading-media';
    this.publicUrl = process.env.R2_PUBLIC_URL || '';
    this.localPath = path.join(process.cwd(), 'uploads');
    this.isR2Configured = false;
  }

  /**
   * Initialize storage
   */
  async initialize() {
    const hasR2Config = process.env.R2_ACCESS_KEY_ID && process.env.R2_SECRET_ACCESS_KEY && process.env.R2_ENDPOINT;

    if (hasR2Config) {
      this.s3Client = new S3Client({
        region: 'auto',
        endpoint: process.env.R2_ENDPOINT,
        credentials: {
          accessKeyId: process.env.R2_ACCESS_KEY_ID,
          secretAccessKey: process.env.R2_SECRET_ACCESS_KEY,
        },
      });
      this.isR2Configured = true;
      console.log('Storage: R2 configured');
    } else {
      console.log('Storage: Using local filesystem (R2 not configured)');
      await fs.mkdir(this.localPath, { recursive: true });
    }
  }

  /**
   * Upload buffer to storage
   */
  async upload(key, buffer, contentType, options = {}) {
    if (this.isR2Configured) {
      return this.uploadToR2(key, buffer, contentType, options);
    }
    return this.uploadToLocal(key, buffer, contentType);
  }

  /**
   * Stream upload to R2 (for large files)
   */
  async streamUpload(key, stream, contentType, options = {}) {
    if (!this.isR2Configured) {
      return this.streamToLocal(key, stream);
    }

    const upload = new Upload({
      client: this.s3Client,
      params: {
        Bucket: this.bucket,
        Key: key,
        Body: stream,
        ContentType: contentType,
        CacheControl: options.cacheControl || 'public, max-age=31536000, immutable',
        ...options.metadata && { Metadata: options.metadata },
      },
      queueSize: 4, // Concurrent part uploads
      partSize: 1024 * 1024 * 5, // 5MB parts
      leavePartsOnError: false,
    });

    upload.on('httpUploadProgress', (progress) => {
      if (options.onProgress) {
        options.onProgress(progress);
      }
    });

    await upload.done();

    return {
      key,
      url: this.getPublicUrl(key),
      bucket: this.bucket,
    };
  }

  /**
   * Upload to R2
   */
  async uploadToR2(key, buffer, contentType, options = {}) {
    const upload = new Upload({
      client: this.s3Client,
      params: {
        Bucket: this.bucket,
        Key: key,
        Body: buffer,
        ContentType: contentType,
        CacheControl: options.cacheControl || 'public, max-age=31536000, immutable',
        ...options.metadata && { Metadata: options.metadata },
      },
    });

    await upload.done();

    console.log(`[R2] Uploaded: ${key}`);

    return {
      key,
      url: this.getPublicUrl(key),
      bucket: this.bucket,
    };
  }

  /**
   * Upload to local filesystem
   */
  async uploadToLocal(key, buffer, contentType) {
    const localFilePath = path.join(this.localPath, key);
    await fs.mkdir(path.dirname(localFilePath), { recursive: true });
    await fs.writeFile(localFilePath, buffer);

    console.log(`[LOCAL] Saved: ${localFilePath}`);

    return {
      key,
      url: `/uploads/${key}`,
      localPath: localFilePath,
    };
  }

  /**
   * Stream to local filesystem
   */
  async streamToLocal(key, stream) {
    const localFilePath = path.join(this.localPath, key);
    await fs.mkdir(path.dirname(localFilePath), { recursive: true });

    const writeStream = createWriteStream(localFilePath);
    await pipeline(stream, writeStream);

    return {
      key,
      url: `/uploads/${key}`,
      localPath: localFilePath,
    };
  }

  /**
   * Get file from storage
   */
  async get(key) {
    if (this.isR2Configured) {
      return this.getFromR2(key);
    }
    return this.getFromLocal(key);
  }

  /**
   * Get file from R2
   */
  async getFromR2(key) {
    const command = new GetObjectCommand({
      Bucket: this.bucket,
      Key: key,
    });

    const response = await this.s3Client.send(command);
    return {
      body: response.Body,
      contentType: response.ContentType,
      contentLength: response.ContentLength,
      metadata: response.Metadata,
    };
  }

  /**
   * Get file from local filesystem
   */
  async getFromLocal(key) {
    const localFilePath = path.join(this.localPath, key);
    const buffer = await fs.readFile(localFilePath);
    const stats = await fs.stat(localFilePath);

    return {
      body: buffer,
      contentLength: stats.size,
    };
  }

  /**
   * Check if file exists
   */
  async exists(key) {
    if (this.isR2Configured) {
      try {
        await this.s3Client.send(new HeadObjectCommand({
          Bucket: this.bucket,
          Key: key,
        }));
        return true;
      } catch {
        return false;
      }
    }

    try {
      await fs.access(path.join(this.localPath, key));
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Delete file from storage
   */
  async delete(key) {
    if (this.isR2Configured) {
      await this.s3Client.send(new DeleteObjectCommand({
        Bucket: this.bucket,
        Key: key,
      }));
      console.log(`[R2] Deleted: ${key}`);
    } else {
      const localFilePath = path.join(this.localPath, key);
      await fs.unlink(localFilePath).catch(() => {});
      console.log(`[LOCAL] Deleted: ${localFilePath}`);
    }
  }

  /**
   * Delete all files with prefix
   */
  async deletePrefix(prefix) {
    // For local storage
    if (!this.isR2Configured) {
      const dirPath = path.join(this.localPath, prefix);
      await fs.rm(dirPath, { recursive: true, force: true }).catch(() => {});
      return;
    }

    // For R2, would need to list and delete
    // This is a simplified version - production would use batch delete
    console.log(`[R2] Delete prefix not fully implemented: ${prefix}`);
  }

  /**
   * Get public URL for a key
   */
  getPublicUrl(key) {
    if (this.isR2Configured && this.publicUrl) {
      return `${this.publicUrl}/${key}`;
    }
    return `/uploads/${key}`;
  }

  /**
   * Generate presigned URL for direct upload
   */
  async getPresignedUploadUrl(key, contentType, expiresIn = 3600) {
    if (!this.isR2Configured) {
      throw new Error('Presigned URLs require R2 configuration');
    }

    const command = new PutObjectCommand({
      Bucket: this.bucket,
      Key: key,
      ContentType: contentType,
    });

    return getSignedUrl(this.s3Client, command, { expiresIn });
  }

  /**
   * Generate presigned URL for download
   */
  async getPresignedDownloadUrl(key, expiresIn = 3600) {
    if (!this.isR2Configured) {
      throw new Error('Presigned URLs require R2 configuration');
    }

    const command = new GetObjectCommand({
      Bucket: this.bucket,
      Key: key,
    });

    return getSignedUrl(this.s3Client, command, { expiresIn });
  }

  /**
   * Get storage statistics
   */
  getStats() {
    return {
      isR2Configured: this.isR2Configured,
      bucket: this.bucket,
      publicUrl: this.publicUrl,
      localPath: this.localPath,
    };
  }
}

// Export singleton instance
export const storage = new StorageManager();
export default StorageManager;
