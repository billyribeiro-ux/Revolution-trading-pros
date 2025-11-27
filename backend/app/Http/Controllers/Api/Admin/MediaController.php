<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api\Admin;

use App\Http\Controllers\Controller;
use App\Models\Media;
use App\Models\MediaVariant;
use App\Models\ImageOptimizationJob;
use App\Models\ImageOptimizationPreset;
use App\Services\ImageOptimizationService;
use App\Services\QueryCacheService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\Validator;

/**
 * MediaController
 *
 * Google Enterprise Grade API endpoints with:
 * - Redis query caching for read operations
 * - Automatic cache invalidation on writes
 * - Optimized pagination with caching
 *
 * @version 2.0.0
 * @level L8 Principal Engineer
 */
class MediaController extends Controller
{
    protected const CACHE_TTL = 300; // 5 minutes
    protected const CACHE_TAG = 'media';

    public function __construct(
        protected ImageOptimizationService $optimizationService,
        protected QueryCacheService $cacheService
    ) {}

    /**
     * List all media with filtering, pagination, and Redis caching
     */
    public function index(Request $request): JsonResponse
    {
        // Generate cache key based on all filter parameters
        $cacheKey = $this->generateListCacheKey($request);

        // Try to get from cache first
        $cachedResult = $this->cacheService->rememberValue(
            $cacheKey,
            function () use ($request) {
                return $this->executeListQuery($request);
            },
            [self::CACHE_TAG, 'media_list'],
            self::CACHE_TTL
        );

        return response()->json($cachedResult);
    }

    /**
     * Execute the list query (extracted for caching)
     */
    protected function executeListQuery(Request $request): array
    {
        $query = Media::query()
            ->with(['uploader:id,name'])
            ->latest();

        // Filters
        if ($type = $request->get('type')) {
            $query->ofType($type);
        }

        if ($request->boolean('images_only')) {
            $query->images();
        }

        if ($collection = $request->get('collection')) {
            $query->inCollection($collection);
        }

        if ($request->boolean('needs_optimization')) {
            $query->needsOptimization();
        }

        if ($request->boolean('optimized')) {
            $query->where('is_optimized', true);
        }

        if ($search = $request->get('search')) {
            $query->where(function ($q) use ($search) {
                $q->where('filename', 'like', "%{$search}%")
                    ->orWhere('title', 'like', "%{$search}%")
                    ->orWhere('alt_text', 'like', "%{$search}%");
            });
        }

        // Sorting
        $sortBy = $request->get('sort_by', 'created_at');
        $sortDir = $request->get('sort_dir', 'desc');
        $query->orderBy($sortBy, $sortDir);

        $perPage = min($request->integer('per_page', 24), 100);
        $media = $query->paginate($perPage);

        return [
            'success' => true,
            'data' => $media->items(),
            'meta' => [
                'current_page' => $media->currentPage(),
                'last_page' => $media->lastPage(),
                'per_page' => $media->perPage(),
                'total' => $media->total(),
            ],
        ];
    }

    /**
     * Generate unique cache key for list queries
     */
    protected function generateListCacheKey(Request $request): string
    {
        $params = [
            'type' => $request->get('type'),
            'images_only' => $request->boolean('images_only'),
            'collection' => $request->get('collection'),
            'needs_optimization' => $request->boolean('needs_optimization'),
            'optimized' => $request->boolean('optimized'),
            'search' => $request->get('search'),
            'sort_by' => $request->get('sort_by', 'created_at'),
            'sort_dir' => $request->get('sort_dir', 'desc'),
            'per_page' => $request->integer('per_page', 24),
            'page' => $request->integer('page', 1),
        ];

        return 'media_list:' . md5(json_encode($params));
    }

    /**
     * Invalidate media cache after modifications
     */
    protected function invalidateMediaCache(): void
    {
        $this->cacheService->invalidateTags([self::CACHE_TAG, 'media_list']);
    }

    /**
     * Upload new media
     */
    public function upload(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'file' => 'required|file|max:51200', // 50MB
            'collection' => 'nullable|string|max:100',
            'preset' => 'nullable|string|exists:image_optimization_presets,slug',
            'alt_text' => 'nullable|string|max:500',
            'title' => 'nullable|string|max:255',
            'process_immediately' => 'nullable|boolean',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        try {
            $media = $this->optimizationService->upload(
                $request->file('file'),
                $request->get('collection'),
                $request->get('preset'),
                [
                    'alt_text' => $request->get('alt_text'),
                    'title' => $request->get('title'),
                    'process_immediately' => $request->boolean('process_immediately'),
                    'priority' => $request->integer('priority', 5),
                ]
            );

            // Invalidate cache after upload
            $this->invalidateMediaCache();

            return response()->json([
                'success' => true,
                'data' => $media->toMediaArray(),
                'message' => 'File uploaded successfully',
            ], 201);

        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => $e->getMessage(),
            ], 400);
        }
    }

    /**
     * Bulk upload multiple files
     */
    public function bulkUpload(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'files' => 'required|array|max:20',
            'files.*' => 'file|max:51200',
            'collection' => 'nullable|string|max:100',
            'preset' => 'nullable|string|exists:image_optimization_presets,slug',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $uploaded = [];
        $failed = [];

        foreach ($request->file('files') as $file) {
            try {
                $media = $this->optimizationService->upload(
                    $file,
                    $request->get('collection'),
                    $request->get('preset')
                );
                $uploaded[] = $media->toMediaArray();
            } catch (\Exception $e) {
                $failed[] = [
                    'filename' => $file->getClientOriginalName(),
                    'error' => $e->getMessage(),
                ];
            }
        }

        return response()->json([
            'success' => true,
            'data' => [
                'uploaded' => $uploaded,
                'failed' => $failed,
            ],
            'message' => count($uploaded) . ' files uploaded, ' . count($failed) . ' failed',
        ]);
    }

    /**
     * Get single media details
     */
    public function show(string|int $id): JsonResponse
    {
        $id = (int) $id;
        $media = Media::with(['uploader:id,name'])->findOrFail($id);

        // Get variants
        $variants = MediaVariant::where('media_id', $id)->get();

        return response()->json([
            'success' => true,
            'data' => array_merge($media->toMediaArray(), [
                'variants' => $variants->map->toVariantArray(),
                'exif' => $media->exif,
                'metadata' => $media->metadata,
                'processing_status' => $media->processing_status,
                'processing_log' => $media->processing_log,
            ]),
        ]);
    }

    /**
     * Update media metadata
     */
    public function update(Request $request, string|int $id): JsonResponse
    {
        $media = Media::findOrFail((int) $id);

        $validator = Validator::make($request->all(), [
            'alt_text' => 'nullable|string|max:500',
            'title' => 'nullable|string|max:255',
            'caption' => 'nullable|string|max:1000',
            'description' => 'nullable|string|max:5000',
            'tags' => 'nullable|array',
            'tags.*' => 'string|max:50',
            'collection' => 'nullable|string|max:100',
            'is_public' => 'nullable|boolean',
            'is_featured' => 'nullable|boolean',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $media->update($request->only([
            'alt_text', 'title', 'caption', 'description',
            'tags', 'collection', 'is_public', 'is_featured',
        ]));

        // Invalidate cache after update
        $this->invalidateMediaCache();

        return response()->json([
            'success' => true,
            'data' => $media->fresh()->toMediaArray(),
            'message' => 'Media updated successfully',
        ]);
    }

    /**
     * Delete media
     */
    public function destroy(string|int $id): JsonResponse
    {
        $id = (int) $id;
        $media = Media::findOrFail($id);

        // Delete all variants
        MediaVariant::where('media_id', $id)->each(function ($variant) {
            $variant->deleteFile();
            $variant->delete();
        });

        $media->delete();

        // Invalidate cache after delete
        $this->invalidateMediaCache();

        return response()->json([
            'success' => true,
            'message' => 'Media deleted successfully',
        ]);
    }

    /**
     * Bulk delete media
     */
    public function bulkDestroy(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'ids' => 'required|array|min:1',
            'ids.*' => 'integer|exists:media,id',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $deleted = 0;
        foreach ($request->get('ids') as $id) {
            $media = Media::find($id);
            if ($media) {
                MediaVariant::where('media_id', $id)->each(function ($variant) {
                    $variant->deleteFile();
                    $variant->delete();
                });
                $media->delete();
                $deleted++;
            }
        }

        // Invalidate cache after bulk delete
        $this->invalidateMediaCache();

        return response()->json([
            'success' => true,
            'message' => "{$deleted} media items deleted",
        ]);
    }

    /**
     * Optimize single media
     */
    public function optimize(Request $request, string|int $id): JsonResponse
    {
        $media = Media::findOrFail((int) $id);

        if (!$media->isImage()) {
            return response()->json([
                'success' => false,
                'message' => 'Only images can be optimized',
            ], 400);
        }

        $preset = $request->get('preset');
        $processImmediately = $request->boolean('immediate', false);

        if ($processImmediately) {
            try {
                $media = $this->optimizationService->processImage($media, $preset);
                return response()->json([
                    'success' => true,
                    'data' => $media->toMediaArray(),
                    'message' => 'Image optimized successfully',
                ]);
            } catch (\Exception $e) {
                return response()->json([
                    'success' => false,
                    'message' => $e->getMessage(),
                ], 500);
            }
        }

        $job = $this->optimizationService->queueOptimization(
            $media,
            $preset,
            $request->integer('priority', 5)
        );

        return response()->json([
            'success' => true,
            'data' => [
                'job_id' => $job->id,
                'status' => $job->status,
            ],
            'message' => 'Optimization job queued',
        ]);
    }

    /**
     * Bulk optimize multiple images
     */
    public function bulkOptimize(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'ids' => 'required|array|min:1|max:100',
            'ids.*' => 'integer|exists:media,id',
            'preset' => 'nullable|string|exists:image_optimization_presets,slug',
            'priority' => 'nullable|integer|min:1|max:10',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $jobs = $this->optimizationService->bulkOptimize(
            $request->get('ids'),
            $request->get('preset'),
            $request->integer('priority', 5)
        );

        return response()->json([
            'success' => true,
            'data' => [
                'jobs_queued' => count($jobs),
            ],
            'message' => count($jobs) . ' optimization jobs queued',
        ]);
    }

    /**
     * Optimize all unoptimized images
     */
    public function optimizeAll(Request $request): JsonResponse
    {
        $limit = min($request->integer('limit', 100), 500);
        $preset = $request->get('preset');
        $priority = $request->integer('priority', 8); // Lower priority for bulk

        $mediaIds = Media::needsOptimization()
            ->limit($limit)
            ->pluck('id')
            ->toArray();

        $jobs = $this->optimizationService->bulkOptimize($mediaIds, $preset, $priority);

        return response()->json([
            'success' => true,
            'data' => [
                'total_pending' => Media::needsOptimization()->count(),
                'jobs_queued' => count($jobs),
            ],
            'message' => count($jobs) . ' optimization jobs queued',
        ]);
    }

    /**
     * Regenerate variants for media
     */
    public function regenerate(Request $request, string|int $id): JsonResponse
    {
        $media = Media::findOrFail((int) $id);

        if (!$media->isImage()) {
            return response()->json([
                'success' => false,
                'message' => 'Only images can be regenerated',
            ], 400);
        }

        try {
            $media = $this->optimizationService->regenerateVariants(
                $media,
                $request->get('preset')
            );

            return response()->json([
                'success' => true,
                'data' => $media->toMediaArray(),
                'message' => 'Variants regenerated successfully',
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get optimization job status
     */
    public function jobStatus(string|int $jobId): JsonResponse
    {
        $jobId = (int) $jobId;
        $job = ImageOptimizationJob::with(['media:id,filename,url'])->findOrFail($jobId);

        return response()->json([
            'success' => true,
            'data' => $job->toJobArray(),
        ]);
    }

    /**
     * Get optimization queue status
     */
    public function queueStatus(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => ImageOptimizationJob::getStats(),
        ]);
    }

    /**
     * Get optimization statistics
     */
    public function statistics(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => array_merge(
                $this->optimizationService->getStatistics(),
                Media::getDashboardStats(),
                ['storage_by_type' => Media::getStorageUsageByType()]
            ),
        ]);
    }

    /**
     * Get available presets
     */
    public function presets(): JsonResponse
    {
        $presets = ImageOptimizationPreset::active()
            ->orderBy('name')
            ->get()
            ->map
            ->toPresetArray();

        return response()->json([
            'success' => true,
            'data' => $presets,
        ]);
    }

    /**
     * Get collections
     */
    public function collections(): JsonResponse
    {
        $collections = Media::whereNotNull('collection')
            ->distinct()
            ->pluck('collection')
            ->sort()
            ->values();

        return response()->json([
            'success' => true,
            'data' => $collections,
        ]);
    }

    /**
     * Replace media file
     */
    public function replace(Request $request, string|int $id): JsonResponse
    {
        $media = Media::findOrFail((int) $id);

        $validator = Validator::make($request->all(), [
            'file' => 'required|file|max:51200',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        try {
            // Delete old variants
            MediaVariant::where('media_id', $id)->each(function ($variant) {
                $variant->deleteFile();
                $variant->delete();
            });

            // Delete old file
            $media->deleteFile();

            // Upload new file to same path
            $file = $request->file('file');
            Storage::disk($media->disk)->putFileAs(
                dirname($media->path),
                $file,
                basename($media->path)
            );

            // Update media record
            $media->update([
                'mime_type' => $file->getMimeType(),
                'size' => $file->getSize(),
                'is_optimized' => false,
                'is_processed' => false,
                'processing_status' => Media::STATUS_PENDING,
                'variants' => [],
            ]);

            // Re-queue optimization
            $this->optimizationService->queueOptimization($media);

            return response()->json([
                'success' => true,
                'data' => $media->fresh()->toMediaArray(),
                'message' => 'File replaced and queued for optimization',
            ]);

        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Download original file
     */
    public function download(string|int $id): mixed
    {
        $media = Media::findOrFail((int) $id);
        $media->recordDownload();

        return Storage::disk($media->disk)->download($media->path, $media->filename);
    }
}
