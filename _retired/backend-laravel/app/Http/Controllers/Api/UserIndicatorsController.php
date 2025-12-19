<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Product;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;

/**
 * User Indicators Controller
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Handles user's purchased indicators - retrieves indicator products
 * owned by the authenticated user with download and license information.
 *
 * @version 1.0.0 - December 2025
 */
class UserIndicatorsController extends Controller
{
    /**
     * Get all indicators owned by the authenticated user
     *
     * @param Request $request
     * @return JsonResponse
     */
    public function index(Request $request): JsonResponse
    {
        $user = $request->user();

        // Get user's indicator products
        $indicators = $user->products()
            ->where('type', 'indicator')
            ->wherePivot('revoked_at', null)
            ->withPivot(['purchased_at', 'license_key', 'expires_at', 'download_count', 'last_downloaded_at'])
            ->get();

        // Transform to response format
        $formattedIndicators = $indicators->map(function ($indicator) {
            $pivot = $indicator->pivot;
            $expiresAt = $pivot->expires_at ? new \DateTime($pivot->expires_at) : null;
            $now = new \DateTime();

            // Calculate status
            $status = 'active';
            $daysUntilExpiry = null;

            if ($expiresAt) {
                $diff = $now->diff($expiresAt);
                $daysUntilExpiry = $diff->invert ? -$diff->days : $diff->days;

                if ($daysUntilExpiry <= 0) {
                    $status = 'expired';
                } elseif ($daysUntilExpiry <= 14) {
                    $status = 'expiring';
                }
            }

            return [
                'id' => $indicator->id,
                'name' => $indicator->name,
                'description' => $indicator->short_description ?? $indicator->description,
                'platform' => $indicator->metadata['platform'] ?? 'TradingView',
                'platformSlug' => strtolower(str_replace(' ', '', $indicator->metadata['platform'] ?? 'tradingview')),
                'status' => $status,
                'expiresAt' => $expiresAt ? $expiresAt->format('M j, Y') : null,
                'daysUntilExpiry' => $daysUntilExpiry,
                'downloadUrl' => $indicator->download_url ?? "/api/user/indicators/{$indicator->id}/download",
                'version' => $indicator->version ?? '1.0.0',
                'slug' => $indicator->slug,
                'lastUpdated' => $indicator->updated_at?->format('M j, Y'),
                'purchasedAt' => $pivot->purchased_at ? (new \DateTime($pivot->purchased_at))->format('M j, Y') : null,
                'licenseKey' => $pivot->license_key,
                'downloadCount' => $pivot->download_count ?? 0,
                'documentationUrl' => $indicator->documentation_url ?? "/dashboard/indicators/{$indicator->slug}/docs",
                'features' => $indicator->features ?? [],
                'thumbnail' => $indicator->thumbnail_url,
            ];
        });

        // Calculate stats
        $stats = [
            'total' => $formattedIndicators->count(),
            'active' => $formattedIndicators->where('status', 'active')->count(),
            'expiring' => $formattedIndicators->where('status', 'expiring')->count(),
            'expired' => $formattedIndicators->where('status', 'expired')->count(),
        ];

        return response()->json([
            'success' => true,
            'data' => [
                'indicators' => $formattedIndicators->values(),
                'stats' => $stats,
            ],
        ]);
    }

    /**
     * Get a specific indicator details for the user
     *
     * @param Request $request
     * @param int $id
     * @return JsonResponse
     */
    public function show(Request $request, int $id): JsonResponse
    {
        $user = $request->user();

        $indicator = $user->products()
            ->where('products.id', $id)
            ->where('type', 'indicator')
            ->wherePivot('revoked_at', null)
            ->withPivot(['purchased_at', 'license_key', 'expires_at', 'download_count', 'last_downloaded_at'])
            ->first();

        if (!$indicator) {
            return response()->json([
                'success' => false,
                'message' => 'Indicator not found or you do not have access',
            ], 404);
        }

        $pivot = $indicator->pivot;
        $expiresAt = $pivot->expires_at ? new \DateTime($pivot->expires_at) : null;

        return response()->json([
            'success' => true,
            'data' => [
                'id' => $indicator->id,
                'name' => $indicator->name,
                'description' => $indicator->description,
                'platform' => $indicator->metadata['platform'] ?? 'TradingView',
                'version' => $indicator->version ?? '1.0.0',
                'slug' => $indicator->slug,
                'expiresAt' => $expiresAt?->format('M j, Y'),
                'licenseKey' => $pivot->license_key,
                'downloadUrl' => "/api/user/indicators/{$indicator->id}/download",
                'documentationUrl' => $indicator->documentation_url,
                'installationGuide' => $indicator->installation_guide,
                'features' => $indicator->features ?? [],
                'changelog' => $indicator->changelog ?? [],
                'supportEmail' => config('mail.from.address'),
            ],
        ]);
    }

    /**
     * Download an indicator (track downloads and return download URL)
     *
     * @param Request $request
     * @param int $id
     * @return JsonResponse
     */
    public function download(Request $request, int $id): JsonResponse
    {
        $user = $request->user();

        $indicator = $user->products()
            ->where('products.id', $id)
            ->where('type', 'indicator')
            ->wherePivot('revoked_at', null)
            ->first();

        if (!$indicator) {
            return response()->json([
                'success' => false,
                'message' => 'Indicator not found or you do not have access',
            ], 404);
        }

        // Check if license has expired
        $pivot = $indicator->pivot;
        if ($pivot->expires_at && new \DateTime($pivot->expires_at) < new \DateTime()) {
            return response()->json([
                'success' => false,
                'message' => 'Your license has expired. Please renew to continue downloading.',
                'error_code' => 'LICENSE_EXPIRED',
            ], 403);
        }

        // Increment download count
        $user->products()->updateExistingPivot($id, [
            'download_count' => ($pivot->download_count ?? 0) + 1,
            'last_downloaded_at' => now(),
        ]);

        // Return download URL (could be a signed URL for S3, etc.)
        $downloadUrl = $indicator->download_url ?? $indicator->metadata['download_url'] ?? null;

        if (!$downloadUrl) {
            return response()->json([
                'success' => false,
                'message' => 'Download not available. Please contact support.',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => [
                'download_url' => $downloadUrl,
                'filename' => $indicator->metadata['filename'] ?? "{$indicator->slug}-v{$indicator->version}.zip",
                'version' => $indicator->version ?? '1.0.0',
            ],
        ]);
    }

    /**
     * Get indicator documentation
     *
     * @param Request $request
     * @param int $id
     * @return JsonResponse
     */
    public function documentation(Request $request, int $id): JsonResponse
    {
        $user = $request->user();

        $indicator = $user->products()
            ->where('products.id', $id)
            ->where('type', 'indicator')
            ->wherePivot('revoked_at', null)
            ->first();

        if (!$indicator) {
            return response()->json([
                'success' => false,
                'message' => 'Indicator not found or you do not have access',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => [
                'name' => $indicator->name,
                'documentation' => $indicator->documentation ?? '',
                'installation_guide' => $indicator->installation_guide ?? '',
                'faq' => $indicator->metadata['faq'] ?? [],
                'video_tutorials' => $indicator->metadata['video_tutorials'] ?? [],
                'changelog' => $indicator->changelog ?? [],
            ],
        ]);
    }
}
