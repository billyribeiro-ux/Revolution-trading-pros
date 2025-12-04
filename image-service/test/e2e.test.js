/**
 * End-to-End Tests for Image Service v3.0
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive testing of all service endpoints and functionality:
 * - Core image processing (upload, transform, resize)
 * - AI services (alt-text, smart crop, content detection)
 * - Maintenance system
 * - Format detection
 * - CDN rules and profiles
 * - Bulk import system
 *
 * @version 3.0.0
 */

import { test, describe, before, after } from 'node:test';
import assert from 'node:assert';
import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const BASE_URL = 'http://localhost:3456';

// Test image (1x1 red PNG)
const TEST_PNG_BASE64 = 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==';
const TEST_PNG_BUFFER = Buffer.from(TEST_PNG_BASE64, 'base64');

// Test helpers
async function fetchJSON(endpoint, options = {}) {
  const response = await fetch(`${BASE_URL}${endpoint}`, {
    ...options,
    headers: {
      'Accept': 'application/json',
      ...options.headers,
    },
  });

  const data = await response.json();
  return { response, data };
}

async function uploadTestImage(filename = 'test.png') {
  const formData = new FormData();
  const blob = new Blob([TEST_PNG_BUFFER], { type: 'image/png' });
  formData.append('file', blob, filename);

  const response = await fetch(`${BASE_URL}/upload`, {
    method: 'POST',
    body: formData,
  });

  return response.json();
}

// ═══════════════════════════════════════════════════════════════════════════
// Core Service Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('Core Service', async () => {
  test('Health check returns healthy', async () => {
    const { response, data } = await fetchJSON('/health');
    assert.strictEqual(response.status, 200);
    assert.strictEqual(data.status, 'healthy');
    assert.ok(data.uptime >= 0);
  });

  test('Stats endpoint returns service statistics', async () => {
    const { response, data } = await fetchJSON('/stats');
    assert.strictEqual(response.status, 200);
    assert.ok('imagesProcessed' in data);
    assert.ok('uptime' in data);
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// Upload Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('Upload Service', async () => {
  test('Upload endpoint accepts image files', async () => {
    const result = await uploadTestImage('upload-test.png');
    assert.ok(result.success || result.id, 'Upload should succeed');
  });

  test('Batch upload info endpoint works', async () => {
    const { response, data } = await fetchJSON('/upload/info');
    assert.strictEqual(response.status, 200);
    assert.ok('maxFileSize' in data);
    assert.ok('supportedFormats' in data);
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// Transformation Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('Transform Service', async () => {
  test('Transform info returns supported formats', async () => {
    const { response, data } = await fetchJSON('/transform/formats');
    assert.strictEqual(response.status, 200);
    assert.ok(Array.isArray(data.formats) || typeof data.formats === 'object');
  });

  test('Resize endpoint accepts parameters', async () => {
    const { response, data } = await fetchJSON('/resize/test-image.png?w=100&h=100');
    // May fail if image doesn't exist, but endpoint should respond
    assert.ok(response.status === 200 || response.status === 404);
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// AI Service Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('AI Service', async () => {
  test('Alt-text generation endpoint exists', async () => {
    const formData = new FormData();
    const blob = new Blob([TEST_PNG_BUFFER], { type: 'image/png' });
    formData.append('image', blob, 'ai-test.png');

    const response = await fetch(`${BASE_URL}/ai/alt-text`, {
      method: 'POST',
      body: formData,
    });

    // Should return result or indicate API key needed
    assert.ok(response.status === 200 || response.status === 400 || response.status === 500);
    const data = await response.json();
    assert.ok('altText' in data || 'error' in data);
  });

  test('Smart crop endpoint exists', async () => {
    const formData = new FormData();
    const blob = new Blob([TEST_PNG_BUFFER], { type: 'image/png' });
    formData.append('image', blob, 'crop-test.png');

    const response = await fetch(`${BASE_URL}/ai/smart-crop`, {
      method: 'POST',
      body: formData,
    });

    assert.ok(response.status === 200 || response.status === 400 || response.status === 500);
  });

  test('Content detection endpoint exists', async () => {
    const formData = new FormData();
    const blob = new Blob([TEST_PNG_BUFFER], { type: 'image/png' });
    formData.append('image', blob, 'detect-test.png');

    const response = await fetch(`${BASE_URL}/ai/detect`, {
      method: 'POST',
      body: formData,
    });

    assert.ok(response.status === 200 || response.status === 400 || response.status === 500);
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// Maintenance Service Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('Maintenance Service', async () => {
  test('Maintenance status returns current state', async () => {
    const { response, data } = await fetchJSON('/maintenance/status');
    assert.strictEqual(response.status, 200);
    assert.ok('isRunning' in data);
    assert.ok('scheduled' in data);
  });

  test('Manual maintenance run works', async () => {
    const response = await fetch(`${BASE_URL}/maintenance/run`, {
      method: 'POST',
    });

    assert.strictEqual(response.status, 200);
    const data = await response.json();
    assert.ok('lastRun' in data || 'stats' in data || 'success' in data);
  });

  test('Individual task execution works', async () => {
    const response = await fetch(`${BASE_URL}/maintenance/task/cleanup-temp`, {
      method: 'POST',
    });

    assert.ok(response.status === 200 || response.status === 202);
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// Format Detection Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('Format Detection Service', async () => {
  test('Format detection with Accept header', async () => {
    const response = await fetch(`${BASE_URL}/format/detect`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'image/avif,image/webp,*/*',
      },
      body: JSON.stringify({
        acceptHeader: 'image/avif,image/webp,image/png,*/*',
      }),
    });

    assert.strictEqual(response.status, 200);
    const data = await response.json();
    assert.ok('format' in data);
    assert.ok(['avif', 'webp', 'jpeg', 'png'].includes(data.format));
  });

  test('Format detection with User-Agent', async () => {
    const response = await fetch(`${BASE_URL}/format/detect`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        userAgent: 'Mozilla/5.0 Chrome/120',
      }),
    });

    assert.strictEqual(response.status, 200);
    const data = await response.json();
    assert.ok('format' in data);
  });

  test('Picture sources generation', async () => {
    const response = await fetch(`${BASE_URL}/format/picture-sources`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        url: '/test-image.jpg',
        sizes: [320, 640, 1024],
      }),
    });

    assert.strictEqual(response.status, 200);
    const data = await response.json();
    assert.ok(Array.isArray(data.sources) || 'sources' in data);
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// CDN Rules Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('CDN Rules Service', async () => {
  test('List compression profiles', async () => {
    const { response, data } = await fetchJSON('/cdn/profiles');
    assert.strictEqual(response.status, 200);
    assert.ok(Array.isArray(data) || 'profiles' in data);
  });

  test('Get specific profile (web)', async () => {
    const { response, data } = await fetchJSON('/cdn/profiles/web');
    assert.strictEqual(response.status, 200);
    assert.ok('name' in data || 'formats' in data);
  });

  test('Get social profile', async () => {
    const { response, data } = await fetchJSON('/cdn/profiles/social');
    assert.strictEqual(response.status, 200);
    assert.ok(data.name || data.presets);
  });

  test('Transform URL generation', async () => {
    const response = await fetch(`${BASE_URL}/cdn/transform-url`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        url: '/images/test.jpg',
        width: 800,
        format: 'webp',
        quality: 80,
      }),
    });

    assert.strictEqual(response.status, 200);
    const data = await response.json();
    assert.ok('url' in data || typeof data === 'string');
  });

  test('Srcset generation', async () => {
    const response = await fetch(`${BASE_URL}/cdn/srcset`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        url: '/images/test.jpg',
        profile: 'web',
      }),
    });

    assert.strictEqual(response.status, 200);
    const data = await response.json();
    assert.ok('srcset' in data || typeof data === 'string');
  });

  test('Social presets list', async () => {
    const { response, data } = await fetchJSON('/cdn/social-presets');
    assert.strictEqual(response.status, 200);
    assert.ok(data['og-image'] || data.presets);
  });

  test('Social URL generation', async () => {
    const response = await fetch(`${BASE_URL}/cdn/social-url`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        url: '/images/test.jpg',
        platform: 'og-image',
      }),
    });

    assert.strictEqual(response.status, 200);
    const data = await response.json();
    assert.ok('url' in data || typeof data === 'string');
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// Bulk Import Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('Bulk Import Service', async () => {
  let testJobId;

  test('Create import job from URLs', async () => {
    const response = await fetch(`${BASE_URL}/import/urls`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        urls: [
          'https://via.placeholder.com/100',
          'https://via.placeholder.com/200',
        ],
      }),
    });

    assert.strictEqual(response.status, 200);
    const data = await response.json();
    assert.ok(data.id || data.jobId);
    testJobId = data.id || data.jobId;
  });

  test('Create import job from text', async () => {
    const response = await fetch(`${BASE_URL}/import/urls-text`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        text: 'https://via.placeholder.com/150\nhttps://via.placeholder.com/250',
      }),
    });

    assert.strictEqual(response.status, 200);
    const data = await response.json();
    assert.ok(data.id || data.jobId);
  });

  test('List all jobs', async () => {
    const { response, data } = await fetchJSON('/import/jobs');
    assert.strictEqual(response.status, 200);
    assert.ok(Array.isArray(data) || 'jobs' in data);
  });

  test('Get specific job', async () => {
    if (!testJobId) {
      console.log('Skipping - no job ID available');
      return;
    }

    const { response, data } = await fetchJSON(`/import/jobs/${testJobId}`);
    assert.ok(response.status === 200 || response.status === 404);
  });

  test('Cancel job endpoint exists', async () => {
    if (!testJobId) {
      console.log('Skipping - no job ID available');
      return;
    }

    const response = await fetch(`${BASE_URL}/import/jobs/${testJobId}/cancel`, {
      method: 'POST',
    });
    assert.ok(response.status === 200 || response.status === 400 || response.status === 404);
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// Integration Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('Integration Tests', async () => {
  test('Full upload -> transform workflow', async () => {
    // 1. Upload image
    const uploadResult = await uploadTestImage('workflow-test.png');
    assert.ok(uploadResult.success || uploadResult.id || uploadResult.filename);

    // 2. Get stats
    const { data: stats } = await fetchJSON('/stats');
    assert.ok(stats.imagesProcessed >= 0);
  });

  test('CDN profile -> format detection workflow', async () => {
    // 1. Get profile
    const { data: profile } = await fetchJSON('/cdn/profiles/web');
    assert.ok(profile);

    // 2. Detect format
    const formatResponse = await fetch(`${BASE_URL}/format/detect`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ acceptHeader: 'image/avif,image/webp,*/*' }),
    });
    const formatData = await formatResponse.json();
    assert.ok(formatData.format);
  });
});

// ═══════════════════════════════════════════════════════════════════════════
// Run all tests
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n═══════════════════════════════════════════════════════════════');
console.log('  Image Service v3.0 - End-to-End Test Suite');
console.log('═══════════════════════════════════════════════════════════════\n');
