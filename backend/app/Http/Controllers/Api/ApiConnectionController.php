<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\ApiConnection;
use App\Models\ApiConnectionLog;
use App\Services\ApiConnectionService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Validator;

/**
 * API Connection Controller
 *
 * REST API for managing all third-party service connections.
 * Provides endpoints for connecting, disconnecting, and monitoring services.
 *
 * @level L11 Principal Engineer - Apple-grade implementation
 */
class ApiConnectionController extends Controller
{
    public function __construct(
        private ApiConnectionService $connectionService
    ) {}

    /**
     * Get all available services and their connection status
     *
     * GET /api/connections
     */
    public function index(): JsonResponse
    {
        $connections = $this->connectionService->getAllConnections();
        $summary = $this->connectionService->getStatusSummary();

        return response()->json([
            'connections' => $connections,
            'summary' => $summary,
            'categories' => $this->connectionService->getAvailableServices(),
        ]);
    }

    /**
     * Get available services grouped by category
     *
     * GET /api/connections/services
     */
    public function services(): JsonResponse
    {
        return response()->json([
            'services' => $this->connectionService->getAvailableServices(),
        ]);
    }

    /**
     * Get connection status summary
     *
     * GET /api/connections/summary
     */
    public function summary(): JsonResponse
    {
        return response()->json($this->connectionService->getStatusSummary());
    }

    /**
     * Get a specific connection
     *
     * GET /api/connections/{serviceKey}
     */
    public function show(string $serviceKey): JsonResponse
    {
        $config = $this->connectionService->getServiceConfig($serviceKey);

        if (!$config) {
            return response()->json(['error' => 'Service not found'], 404);
        }

        $connection = ApiConnection::where('service_key', $serviceKey)->first();

        return response()->json([
            'service' => array_merge(['key' => $serviceKey], $config),
            'connection' => $connection?->toPublicArray(),
            'is_connected' => $connection?->isConnected() ?? false,
        ]);
    }

    /**
     * Connect a service
     *
     * POST /api/connections/{serviceKey}/connect
     */
    public function connect(Request $request, string $serviceKey): JsonResponse
    {
        $config = $this->connectionService->getServiceConfig($serviceKey);

        if (!$config) {
            return response()->json(['error' => 'Service not found'], 404);
        }

        // Build validation rules from field config
        $rules = [];
        foreach ($config['fields'] as $field) {
            if ($field['required']) {
                $rules["credentials.{$field['key']}"] = 'required|string';
            }
        }
        $rules['environment'] = 'sometimes|in:production,sandbox,test';

        $validator = Validator::make($request->all(), $rules);

        if ($validator->fails()) {
            return response()->json([
                'error' => 'Validation failed',
                'errors' => $validator->errors(),
            ], 422);
        }

        try {
            $connection = $this->connectionService->connect(
                $serviceKey,
                $request->input('credentials', []),
                $request->input('environment', 'production')
            );

            return response()->json([
                'success' => true,
                'message' => "{$config['name']} connected successfully",
                'connection' => $connection->toPublicArray(),
            ]);

        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Disconnect a service
     *
     * POST /api/connections/{serviceKey}/disconnect
     */
    public function disconnect(string $serviceKey): JsonResponse
    {
        $config = $this->connectionService->getServiceConfig($serviceKey);

        if (!$config) {
            return response()->json(['error' => 'Service not found'], 404);
        }

        $success = $this->connectionService->disconnect($serviceKey);

        if ($success) {
            return response()->json([
                'success' => true,
                'message' => "{$config['name']} disconnected successfully",
            ]);
        }

        return response()->json([
            'success' => false,
            'error' => 'Service was not connected',
        ], 400);
    }

    /**
     * Test a connection without saving
     *
     * POST /api/connections/{serviceKey}/test
     */
    public function test(Request $request, string $serviceKey): JsonResponse
    {
        $config = $this->connectionService->getServiceConfig($serviceKey);

        if (!$config) {
            return response()->json(['error' => 'Service not found'], 404);
        }

        $result = $this->connectionService->testConnection(
            $serviceKey,
            $request->input('credentials', [])
        );

        return response()->json($result);
    }

    /**
     * Verify an existing connection
     *
     * POST /api/connections/{serviceKey}/verify
     */
    public function verify(string $serviceKey): JsonResponse
    {
        $connection = ApiConnection::where('service_key', $serviceKey)->first();

        if (!$connection) {
            return response()->json(['error' => 'Connection not found'], 404);
        }

        $verified = $this->connectionService->verify($connection);

        return response()->json([
            'success' => $verified,
            'message' => $verified ? 'Connection verified' : 'Verification failed',
            'connection' => $connection->fresh()->toPublicArray(),
        ]);
    }

    /**
     * Get connection logs
     *
     * GET /api/connections/{serviceKey}/logs
     */
    public function logs(string $serviceKey): JsonResponse
    {
        $connection = ApiConnection::where('service_key', $serviceKey)->first();

        if (!$connection) {
            return response()->json(['error' => 'Connection not found'], 404);
        }

        $logs = $connection->logs()
            ->orderBy('created_at', 'desc')
            ->limit(50)
            ->get();

        return response()->json(['logs' => $logs]);
    }

    /**
     * Get health report for all connections
     *
     * GET /api/connections/health
     */
    public function health(): JsonResponse
    {
        return response()->json([
            'report' => $this->connectionService->getHealthReport(),
        ]);
    }

    /**
     * Update connection configuration
     *
     * PATCH /api/connections/{serviceKey}
     */
    public function update(Request $request, string $serviceKey): JsonResponse
    {
        $connection = ApiConnection::where('service_key', $serviceKey)->first();

        if (!$connection) {
            return response()->json(['error' => 'Connection not found'], 404);
        }

        $validator = Validator::make($request->all(), [
            'config' => 'sometimes|array',
            'metadata' => 'sometimes|array',
            'webhooks_enabled' => 'sometimes|boolean',
            'webhook_url' => 'sometimes|nullable|url',
            'environment' => 'sometimes|in:production,sandbox,test',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'error' => 'Validation failed',
                'errors' => $validator->errors(),
            ], 422);
        }

        $connection->update($request->only([
            'config',
            'metadata',
            'webhooks_enabled',
            'webhook_url',
            'environment',
        ]));

        ApiConnectionLog::log($connection->id, 'updated', 'success', [
            'fields_updated' => array_keys($request->all()),
        ]);

        return response()->json([
            'success' => true,
            'connection' => $connection->fresh()->toPublicArray(),
        ]);
    }

    /**
     * Get connections by category
     *
     * GET /api/connections/category/{category}
     */
    public function byCategory(string $category): JsonResponse
    {
        $connections = $this->connectionService->getConnectionsByCategory($category);

        return response()->json([
            'category' => $category,
            'connections' => $connections->values(),
        ]);
    }

    /**
     * Refresh OAuth token
     *
     * POST /api/connections/{serviceKey}/refresh
     */
    public function refreshToken(string $serviceKey): JsonResponse
    {
        $connection = ApiConnection::where('service_key', $serviceKey)->first();

        if (!$connection) {
            return response()->json(['error' => 'Connection not found'], 404);
        }

        if (!$connection->is_oauth) {
            return response()->json(['error' => 'Service does not use OAuth'], 400);
        }

        $success = $this->connectionService->refreshToken($serviceKey);

        return response()->json([
            'success' => $success,
            'message' => $success ? 'Token refreshed' : 'Token refresh failed',
        ]);
    }

    /**
     * Get connections that need attention
     *
     * GET /api/connections/attention
     */
    public function needsAttention(): JsonResponse
    {
        $connections = ApiConnection::needsAttention()->get();

        return response()->json([
            'connections' => $connections->map(fn($c) => $c->toPublicArray()),
        ]);
    }
}
