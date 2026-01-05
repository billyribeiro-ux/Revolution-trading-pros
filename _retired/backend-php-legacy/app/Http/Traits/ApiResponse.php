<?php

declare(strict_types=1);

namespace App\Http\Traits;

use Illuminate\Http\JsonResponse;
use Illuminate\Http\Resources\Json\JsonResource;
use Illuminate\Http\Resources\Json\ResourceCollection;
use Illuminate\Pagination\LengthAwarePaginator;
use Illuminate\Support\Facades\Log;

/**
 * ApiResponse Trait - Standardized API Response Format
 *
 * Provides consistent JSON response structure across all API controllers.
 * Follows JSON:API specification patterns with enterprise-grade error handling.
 *
 * Response Format:
 * {
 *   "success": boolean,
 *   "message": string (optional),
 *   "data": mixed (optional),
 *   "meta": object (optional),
 *   "errors": array (optional)
 * }
 *
 * @version 1.0.0
 * @level L11 Principal Engineer
 */
trait ApiResponse
{
    /**
     * Success response with data
     */
    protected function successResponse(
        mixed $data = null,
        string $message = null,
        int $statusCode = 200,
        array $meta = []
    ): JsonResponse {
        $response = [
            'success' => true,
        ];

        if ($message) {
            $response['message'] = $message;
        }

        if ($data !== null) {
            if ($data instanceof LengthAwarePaginator) {
                $response['data'] = $data->items();
                $response['meta'] = [
                    'pagination' => [
                        'current_page' => $data->currentPage(),
                        'last_page' => $data->lastPage(),
                        'per_page' => $data->perPage(),
                        'total' => $data->total(),
                        'from' => $data->firstItem(),
                        'to' => $data->lastItem(),
                    ],
                ];
            } elseif ($data instanceof JsonResource || $data instanceof ResourceCollection) {
                $response['data'] = $data;
            } else {
                $response['data'] = $data;
            }
        }

        if (!empty($meta)) {
            $response['meta'] = array_merge($response['meta'] ?? [], $meta);
        }

        return response()->json($response, $statusCode);
    }

    /**
     * Created response (201)
     */
    protected function createdResponse(
        mixed $data = null,
        string $message = 'Resource created successfully'
    ): JsonResponse {
        return $this->successResponse($data, $message, 201);
    }

    /**
     * No content response (204)
     */
    protected function noContentResponse(): JsonResponse
    {
        return response()->json(null, 204);
    }

    /**
     * Error response
     */
    protected function errorResponse(
        string $message,
        int $statusCode = 400,
        array $errors = [],
        string $errorCode = null
    ): JsonResponse {
        $response = [
            'success' => false,
            'message' => $message,
        ];

        if ($errorCode) {
            $response['error_code'] = $errorCode;
        }

        if (!empty($errors)) {
            $response['errors'] = $errors;
        }

        return response()->json($response, $statusCode);
    }

    /**
     * Validation error response (422)
     */
    protected function validationErrorResponse(
        array $errors,
        string $message = 'Validation failed'
    ): JsonResponse {
        return $this->errorResponse($message, 422, $errors, 'VALIDATION_ERROR');
    }

    /**
     * Not found response (404)
     */
    protected function notFoundResponse(string $resource = 'Resource'): JsonResponse
    {
        return $this->errorResponse(
            "{$resource} not found",
            404,
            [],
            'NOT_FOUND'
        );
    }

    /**
     * Unauthorized response (401)
     */
    protected function unauthorizedResponse(
        string $message = 'Unauthorized'
    ): JsonResponse {
        return $this->errorResponse($message, 401, [], 'UNAUTHORIZED');
    }

    /**
     * Forbidden response (403)
     */
    protected function forbiddenResponse(
        string $message = 'You do not have permission to perform this action'
    ): JsonResponse {
        return $this->errorResponse($message, 403, [], 'FORBIDDEN');
    }

    /**
     * Internal server error response (500)
     * Logs the actual error but returns sanitized message
     */
    protected function serverErrorResponse(
        \Throwable $exception,
        string $publicMessage = 'An unexpected error occurred. Please try again later.',
        array $context = []
    ): JsonResponse {
        // Log the actual error for debugging
        Log::error($publicMessage, array_merge([
            'exception' => get_class($exception),
            'message' => $exception->getMessage(),
            'file' => $exception->getFile(),
            'line' => $exception->getLine(),
            'trace' => $exception->getTraceAsString(),
            'user_id' => auth()->id(),
        ], $context));

        // Return sanitized error to client
        $response = [
            'success' => false,
            'message' => $publicMessage,
            'error_code' => 'INTERNAL_ERROR',
        ];

        // Include request ID for support reference
        if ($requestId = request()->header('X-Request-ID')) {
            $response['request_id'] = $requestId;
        }

        // In non-production, include more details for debugging
        if (config('app.debug') && config('app.env') !== 'production') {
            $response['debug'] = [
                'exception' => get_class($exception),
                'message' => $exception->getMessage(),
                'file' => $exception->getFile(),
                'line' => $exception->getLine(),
            ];
        }

        return response()->json($response, 500);
    }

    /**
     * Rate limit exceeded response (429)
     */
    protected function rateLimitResponse(
        int $retryAfter = 60,
        string $message = 'Too many requests. Please try again later.'
    ): JsonResponse {
        return response()->json([
            'success' => false,
            'message' => $message,
            'error_code' => 'RATE_LIMIT_EXCEEDED',
            'retry_after' => $retryAfter,
        ], 429)->header('Retry-After', (string) $retryAfter);
    }

    /**
     * Service unavailable response (503)
     */
    protected function serviceUnavailableResponse(
        string $message = 'Service temporarily unavailable. Please try again later.'
    ): JsonResponse {
        return $this->errorResponse($message, 503, [], 'SERVICE_UNAVAILABLE');
    }

    /**
     * Conflict response (409)
     */
    protected function conflictResponse(
        string $message = 'Resource conflict'
    ): JsonResponse {
        return $this->errorResponse($message, 409, [], 'CONFLICT');
    }

    /**
     * Accepted response for async operations (202)
     */
    protected function acceptedResponse(
        string $message = 'Request accepted for processing',
        array $meta = []
    ): JsonResponse {
        $response = [
            'success' => true,
            'message' => $message,
        ];

        if (!empty($meta)) {
            $response['meta'] = $meta;
        }

        return response()->json($response, 202);
    }
}
