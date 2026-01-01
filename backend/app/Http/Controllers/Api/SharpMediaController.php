<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Services\SharpImageService;
use App\Models\Media;
use App\Models\MediaVariant;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Validator;

/**
 * SharpMediaController
 *
 * API endpoints for high-performance image processing using Sharp service.
 * Provides 10-15x faster image processing compared to PHP-based solutions.
 */
class SharpMediaController extends Controller
{
    protected SharpImageService $sharpService;

    public function __construct(SharpImageService $sharpService)
    {
        $this->sharpService = $sharpService;
    }

    /**
     * Check Sharp service health
     *
     * @return JsonResponse
     */
    public function health(): JsonResponse
    {
        $isAvailable = $this->sharpService->isAvailable();
        $capabilities = $this->sharpService->getCapabilities();

        return response()->json([
            'sharp_service' => [
                'available' => $isAvailable,
                'url' => config('services.sharp.url'),
            ],
            'capabilities' => $capabilities,
            'storage' => [
                'disk' => config('filesystems.default_media_disk'),
                'r2_configured' => !empty(config('filesystems.disks.r2.key')),
            ],
        ]);
    }

    /**
     * Upload and process image
     *
     * @param Request $request
     * @return JsonResponse
     */
    public function upload(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'image' => 'required|file|image|max:51200', // 50MB
            'collection' => 'nullable|string|max:255',
            'preset' => 'nullable|string|in:maximum,balanced,performance,thumbnail',
            'alt_text' => 'nullable|string|max:500',
            'title' => 'nullable|string|max:255',
            'generate_webp' => 'nullable|boolean',
            'generate_avif' => 'nullable|boolean',
            'generate_responsive' => 'nullable|boolean',
            'generate_thumbnail' => 'nullable|boolean',
            'generate_blurhash' => 'nullable|boolean',
            'generate_retina' => 'nullable|boolean',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        // Check if Sharp service is available
        if (!$this->sharpService->isAvailable()) {
            return response()->json([
                'success' => false,
                'error' => 'Image processing service is unavailable',
            ], 503);
        }

        try {
            $result = $this->sharpService->upload(
                $request->file('image'),
                $request->input('collection'),
                $request->input('preset', 'balanced'),
                [
                    'alt_text' => $request->input('alt_text'),
                    'title' => $request->input('title'),
                    'retina' => $request->boolean('generate_retina') ? 'true' : 'false',
                ]
            );

            return response()->json([
                'success' => true,
                'media' => $this->formatMediaResponse($result['media']),
                'processing_time_ms' => $result['processing_time'],
                'variants_count' => count($result['result']['variants'] ?? []),
                'stats' => $result['result']['stats'] ?? null,
            ], 201);

        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => 'Failed to process image: ' . $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Bulk upload multiple images
     *
     * @param Request $request
     * @return JsonResponse
     */
    public function bulkUpload(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'images' => 'required|array|max:20',
            'images.*' => 'required|file|image|max:51200',
            'collection' => 'nullable|string|max:255',
            'preset' => 'nullable|string|in:maximum,balanced,performance,thumbnail',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        if (!$this->sharpService->isAvailable()) {
            return response()->json([
                'success' => false,
                'error' => 'Image processing service is unavailable',
            ], 503);
        }

        $results = [];
        $errors = [];

        foreach ($request->file('images') as $index => $file) {
            try {
                $result = $this->sharpService->upload(
                    $file,
                    $request->input('collection'),
                    $request->input('preset', 'balanced'),
                    []
                );

                $results[] = [
                    'index' => $index,
                    'success' => true,
                    'media' => $this->formatMediaResponse($result['media']),
                ];
            } catch (\Exception $e) {
                $errors[] = [
                    'index' => $index,
                    'filename' => $file->getClientOriginalName(),
                    'error' => $e->getMessage(),
                ];
            }
        }

        return response()->json([
            'success' => count($errors) === 0,
            'uploaded' => $results,
            'errors' => $errors,
            'total' => count($request->file('images')),
            'successful' => count($results),
            'failed' => count($errors),
        ]);
    }

    /**
     * Process image from URL
     *
     * @param Request $request
     * @return JsonResponse
     */
    public function processUrl(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'url' => 'required|url',
            'collection' => 'nullable|string|max:255',
            'preset' => 'nullable|string|in:maximum,balanced,performance,thumbnail',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        if (!$this->sharpService->isAvailable()) {
            return response()->json([
                'success' => false,
                'error' => 'Image processing service is unavailable',
            ], 503);
        }

        try {
            $result = $this->sharpService->processFromUrl(
                $request->input('url'),
                $request->input('collection'),
                ['preset' => $request->input('preset', 'balanced')]
            );

            return response()->json([
                'success' => true,
                'media' => $this->formatMediaResponse($result['media']),
            ], 201);

        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => 'Failed to process image from URL: ' . $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Optimize existing media
     *
     * @param Request $request
     * @param int $id
     * @return JsonResponse
     */
    public function optimize(Request $request, int $id): JsonResponse
    {
        $media = Media::find($id);

        if (!$media) {
            return response()->json([
                'success' => false,
                'error' => 'Media not found',
            ], 404);
        }

        if (!$media->isImage()) {
            return response()->json([
                'success' => false,
                'error' => 'Only images can be optimized',
            ], 400);
        }

        if (!$this->sharpService->isAvailable()) {
            return response()->json([
                'success' => false,
                'error' => 'Image processing service is unavailable',
            ], 503);
        }

        try {
            $result = $this->sharpService->optimize(
                $media,
                $request->input('preset', 'balanced')
            );

            return response()->json([
                'success' => true,
                'media' => $this->formatMediaResponse($result['media']),
                'stats' => $result['result']['stats'] ?? null,
            ]);

        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => 'Failed to optimize image: ' . $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get media with all variants
     *
     * @param int $id
     * @return JsonResponse
     */
    public function show(int $id): JsonResponse
    {
        $media = Media::with('variantsRelation')->find($id);

        if (!$media) {
            return response()->json([
                'success' => false,
                'error' => 'Media not found',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'media' => $this->formatMediaResponse($media),
            'variants' => $media->variantsRelation->map(fn($v) => [
                'id' => $v->id,
                'type' => $v->variant_type,
                'size_name' => $v->size_name,
                'format' => $v->format,
                'width' => $v->width,
                'height' => $v->height,
                'url' => $v->url,
                'size' => $v->size,
                'is_retina' => $v->is_retina,
            ]),
            'srcset' => $this->sharpService->getSrcset($media),
            'picture_sources' => $this->sharpService->getPictureSources($media),
        ]);
    }

    /**
     * List all media
     *
     * @param Request $request
     * @return JsonResponse
     */
    public function index(Request $request): JsonResponse
    {
        $query = Media::query()->orderBy('created_at', 'desc');

        // Filter by collection
        if ($request->has('collection')) {
            $query->where('collection', $request->input('collection'));
        }

        // Filter by type
        if ($request->has('type')) {
            $query->where('type', $request->input('type'));
        }

        // Filter by optimized status
        if ($request->has('optimized')) {
            $query->where('is_optimized', $request->boolean('optimized'));
        }

        // Search
        if ($request->has('search')) {
            $search = $request->input('search');
            $query->where(function ($q) use ($search) {
                $q->where('filename', 'like', "%{$search}%")
                    ->orWhere('title', 'like', "%{$search}%")
                    ->orWhere('alt_text', 'like', "%{$search}%");
            });
        }

        $perPage = min($request->input('per_page', 20), 100);
        $media = $query->paginate($perPage);

        return response()->json([
            'success' => true,
            'data' => $media->items(),
            'pagination' => [
                'current_page' => $media->currentPage(),
                'last_page' => $media->lastPage(),
                'per_page' => $media->perPage(),
                'total' => $media->total(),
            ],
        ]);
    }

    /**
     * Delete media and all variants
     *
     * @param int $id
     * @return JsonResponse
     */
    public function destroy(int $id): JsonResponse
    {
        $media = Media::find($id);

        if (!$media) {
            return response()->json([
                'success' => false,
                'error' => 'Media not found',
            ], 404);
        }

        // Delete variants from storage
        MediaVariant::where('media_id', $id)->each(function ($variant) {
            $variant->deleteFile();
            $variant->delete();
        });

        // Delete original from storage
        $media->deleteFile();
        $media->delete();

        return response()->json([
            'success' => true,
            'message' => 'Media deleted successfully',
        ]);
    }

    /**
     * Get statistics
     *
     * @return JsonResponse
     */
    public function statistics(): JsonResponse
    {
        $totalMedia = Media::count();
        $totalImages = Media::where('type', 'image')->count();
        $optimizedImages = Media::where('type', 'image')->where('is_optimized', true)->count();
        $totalVariants = MediaVariant::count();
        $totalStorage = Media::sum('size');
        $totalBytesSaved = MediaVariant::sum('bytes_saved');

        return response()->json([
            'success' => true,
            'statistics' => [
                'total_media' => $totalMedia,
                'total_images' => $totalImages,
                'optimized_images' => $optimizedImages,
                'optimization_rate' => $totalImages > 0
                    ? round(($optimizedImages / $totalImages) * 100, 1)
                    : 0,
                'total_variants' => $totalVariants,
                'total_storage_bytes' => $totalStorage,
                'total_storage_human' => $this->formatBytes($totalStorage),
                'total_bytes_saved' => $totalBytesSaved,
                'total_savings_human' => $this->formatBytes($totalBytesSaved),
                'sharp_service' => [
                    'available' => $this->sharpService->isAvailable(),
                    'url' => config('services.sharp.url'),
                ],
            ],
        ]);
    }

    /**
     * Format media for response
     */
    protected function formatMediaResponse(Media $media): array
    {
        return [
            'id' => $media->id,
            'filename' => $media->filename,
            'url' => $media->url,
            'cdn_url' => $media->cdn_url,
            'thumbnail_url' => $media->thumbnail_url,
            'width' => $media->width,
            'height' => $media->height,
            'size' => $media->size,
            'size_human' => $this->formatBytes($media->size),
            'mime_type' => $media->mime_type,
            'type' => $media->type,
            'collection' => $media->collection,
            'alt_text' => $media->alt_text,
            'title' => $media->title,
            'is_optimized' => $media->is_optimized,
            'blurhash' => $media->metadata['blurhash'] ?? null,
            'lqip' => $media->metadata['lqip'] ?? null,
            'variants' => $media->variants ?? [],
            'created_at' => $media->created_at?->toISOString(),
        ];
    }

    /**
     * Format bytes to human readable
     */
    protected function formatBytes(int $bytes): string
    {
        $units = ['B', 'KB', 'MB', 'GB', 'TB'];
        $bytes = max($bytes, 0);
        $pow = floor(($bytes ? log($bytes) : 0) / log(1024));
        $pow = min($pow, count($units) - 1);
        $bytes /= pow(1024, $pow);

        return round($bytes, 2) . ' ' . $units[$pow];
    }
}
