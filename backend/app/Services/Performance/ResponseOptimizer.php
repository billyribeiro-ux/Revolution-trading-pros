<?php

declare(strict_types=1);

namespace App\Services\Performance;

use Illuminate\Http\JsonResponse;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Http\Request;

/**
 * Response Optimizer Service (ICT9+ Enterprise Grade)
 *
 * Optimizes API responses through:
 * - Sparse fieldsets (only return requested fields)
 * - Response compression
 * - Cursor-based pagination for large datasets
 * - Field-level caching
 * - Response streaming for large payloads
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class ResponseOptimizer
{
    /**
     * Apply sparse fieldsets to model/collection
     */
    public function sparseFieldsets(
        Model|Collection|array $data,
        ?string $fields = null,
        ?string $includes = null
    ): array {
        $requestedFields = $fields ? explode(',', $fields) : null;
        $requestedIncludes = $includes ? explode(',', $includes) : [];

        if ($data instanceof Collection) {
            return $data->map(fn($item) => $this->filterFields($item, $requestedFields, $requestedIncludes))->toArray();
        }

        if ($data instanceof Model) {
            return $this->filterFields($data, $requestedFields, $requestedIncludes);
        }

        return $data;
    }

    /**
     * Filter model fields based on request
     */
    private function filterFields(Model $model, ?array $fields, array $includes): array
    {
        $data = $model->toArray();

        // Filter to only requested fields
        if ($fields !== null) {
            $data = array_intersect_key($data, array_flip($fields));
        }

        // Include requested relationships
        foreach ($includes as $include) {
            if ($model->relationLoaded($include)) {
                $data[$include] = $model->getRelation($include)?->toArray();
            }
        }

        return $data;
    }

    /**
     * Create cursor-based pagination response
     */
    public function cursorPaginate(
        \Illuminate\Database\Eloquent\Builder $query,
        int $limit = 50,
        ?string $cursor = null,
        string $cursorColumn = 'id'
    ): array {
        // Decode cursor
        $afterId = $cursor ? base64_decode($cursor) : null;

        // Apply cursor
        if ($afterId) {
            $query->where($cursorColumn, '>', $afterId);
        }

        // Get one more than limit to check for next page
        $items = $query->orderBy($cursorColumn)->limit($limit + 1)->get();

        $hasMore = $items->count() > $limit;

        if ($hasMore) {
            $items = $items->take($limit);
        }

        $lastItem = $items->last();
        $nextCursor = $hasMore && $lastItem
            ? base64_encode($lastItem->{$cursorColumn})
            : null;

        return [
            'data' => $items,
            'meta' => [
                'has_more' => $hasMore,
                'next_cursor' => $nextCursor,
                'count' => $items->count(),
            ],
        ];
    }

    /**
     * Create optimized JSON response
     */
    public function jsonResponse(
        mixed $data,
        int $status = 200,
        array $headers = [],
        bool $enableCache = true
    ): JsonResponse {
        $response = new JsonResponse($data, $status, $headers);

        // Use faster JSON encoding
        $response->setEncodingOptions(
            JSON_UNESCAPED_UNICODE |
            JSON_UNESCAPED_SLASHES |
            JSON_NUMERIC_CHECK
        );

        // Add cache headers for GET requests
        if ($enableCache && request()->isMethod('GET')) {
            $response->headers->set('Cache-Control', 'public, max-age=60, stale-while-revalidate=300');

            // Add ETag
            $etag = '"' . md5($response->getContent()) . '"';
            $response->headers->set('ETag', $etag);

            // Check If-None-Match
            if (request()->headers->get('If-None-Match') === $etag) {
                return new JsonResponse(null, 304);
            }
        }

        return $response;
    }

    /**
     * Stream large response as JSON lines
     */
    public function streamResponse(\Generator $data, int $chunkSize = 100): \Symfony\Component\HttpFoundation\StreamedResponse
    {
        return response()->stream(function () use ($data, $chunkSize) {
            $buffer = [];
            $count = 0;

            echo "[\n";

            foreach ($data as $item) {
                $buffer[] = json_encode($item);
                $count++;

                if (count($buffer) >= $chunkSize) {
                    echo implode(",\n", $buffer);
                    $buffer = [];
                    flush();
                }
            }

            if (!empty($buffer)) {
                if ($count > count($buffer)) {
                    echo ",\n";
                }
                echo implode(",\n", $buffer);
            }

            echo "\n]";
        }, 200, [
            'Content-Type' => 'application/json',
            'X-Accel-Buffering' => 'no', // Disable nginx buffering
        ]);
    }

    /**
     * Build optimized response with metadata
     */
    public function buildResponse(Request $request, mixed $data, array $meta = []): array
    {
        $fields = $request->query('fields');
        $includes = $request->query('include');

        $optimizedData = $this->sparseFieldsets($data, $fields, $includes);

        $response = [
            'data' => $optimizedData,
            'meta' => array_merge($meta, [
                'timestamp' => now()->toIso8601String(),
            ]),
        ];

        // Add request timing if in debug mode
        if (config('app.debug')) {
            $response['meta']['response_time_ms'] = round((microtime(true) - LARAVEL_START) * 1000, 2);
        }

        return $response;
    }
}
