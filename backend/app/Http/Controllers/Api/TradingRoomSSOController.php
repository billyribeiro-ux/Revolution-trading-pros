<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\TradingRoom;
use App\Models\User;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Firebase\JWT\JWT;

/**
 * Trading Room SSO Controller
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Handles JWT-based Single Sign-On (SSO) for trading room access.
 * Similar to Simpler Trading's JWT SSO implementation.
 *
 * Features:
 * - JWT token generation for trading room access
 * - Access validation based on user memberships
 * - Secure redirect to trading room platform
 *
 * @version 1.0.0 - December 2025
 */
class TradingRoomSSOController extends Controller
{
    /**
     * JWT secret key (should be in .env)
     */
    private string $jwtSecret;

    /**
     * JWT token expiration (in seconds)
     */
    private int $tokenExpiration = 3600; // 1 hour

    /**
     * Trading room base URL
     */
    private string $tradingRoomBaseUrl;

    public function __construct()
    {
        $this->jwtSecret = config('services.trading_room.jwt_secret', env('TRADING_ROOM_JWT_SECRET', 'your-secret-key'));
        $this->tradingRoomBaseUrl = config('services.trading_room.base_url', env('TRADING_ROOM_BASE_URL', 'https://room.revolutiontradingpros.com'));
    }

    /**
     * Generate JWT token for trading room access
     *
     * @param Request $request
     * @param string $roomSlug
     * @return JsonResponse
     */
    public function generateToken(Request $request, string $roomSlug): JsonResponse
    {
        $user = $request->user();

        // Get the trading room
        $room = TradingRoom::where('slug', $roomSlug)
            ->where('is_active', true)
            ->first();

        if (!$room) {
            return response()->json([
                'success' => false,
                'message' => 'Trading room not found',
            ], 404);
        }

        // Check if user has access to this room
        $hasAccess = $this->userHasAccess($user, $room);

        if (!$hasAccess) {
            return response()->json([
                'success' => false,
                'message' => 'You do not have access to this trading room',
                'error_code' => 'NO_ACCESS',
            ], 403);
        }

        // Generate JWT token
        $token = $this->createJwtToken($user, $room);

        // Build redirect URL
        $redirectUrl = $this->buildRedirectUrl($room, $token);

        // Log SSO access
        Log::info('Trading Room SSO Token Generated', [
            'user_id' => $user->id,
            'user_email' => $user->email,
            'room_slug' => $roomSlug,
            'room_name' => $room->name,
        ]);

        return response()->json([
            'success' => true,
            'data' => [
                'jwt' => $token,
                'redirect_url' => $redirectUrl,
                'room' => [
                    'id' => $room->id,
                    'name' => $room->name,
                    'slug' => $room->slug,
                    'type' => $room->type,
                ],
                'expires_in' => $this->tokenExpiration,
            ],
        ]);
    }

    /**
     * Get all accessible trading rooms for user with SSO URLs
     *
     * @param Request $request
     * @return JsonResponse
     */
    public function getAccessibleRooms(Request $request): JsonResponse
    {
        $user = $request->user();

        // Get user's active memberships
        $memberships = $user->memberships()
            ->where('status', 'active')
            ->where(function ($query) {
                $query->whereNull('expires_at')
                    ->orWhere('expires_at', '>', now());
            })
            ->with('plan')
            ->get();

        // Get all active trading rooms
        $rooms = TradingRoom::where('is_active', true)
            ->where('type', 'trading_room')
            ->ordered()
            ->get();

        // Build accessible rooms with SSO info
        $accessibleRooms = [];
        foreach ($rooms as $room) {
            $hasAccess = $this->userHasAccess($user, $room);
            $membership = $this->getUserMembershipForRoom($user, $room);

            $accessibleRooms[] = [
                'id' => $room->id,
                'name' => $room->name,
                'slug' => $room->slug,
                'icon' => $room->icon,
                'type' => $room->type,
                'has_access' => $hasAccess,
                'membership_type' => $membership ? $membership->plan->name ?? 'active' : null,
                'membership_status' => $membership ? $this->getMembershipStatus($membership) : null,
                'sso_url' => $hasAccess ? "/api/trading-rooms/{$room->slug}/sso" : null,
            ];
        }

        return response()->json([
            'success' => true,
            'data' => [
                'rooms' => $accessibleRooms,
                'total_access' => collect($accessibleRooms)->where('has_access', true)->count(),
            ],
        ]);
    }

    /**
     * Direct SSO redirect (generates token and redirects)
     *
     * @param Request $request
     * @param string $roomSlug
     * @return \Illuminate\Http\RedirectResponse|JsonResponse
     */
    public function redirect(Request $request, string $roomSlug)
    {
        $user = $request->user();

        $room = TradingRoom::where('slug', $roomSlug)
            ->where('is_active', true)
            ->first();

        if (!$room) {
            return response()->json([
                'success' => false,
                'message' => 'Trading room not found',
            ], 404);
        }

        if (!$this->userHasAccess($user, $room)) {
            return response()->json([
                'success' => false,
                'message' => 'Access denied',
            ], 403);
        }

        $token = $this->createJwtToken($user, $room);
        $redirectUrl = $this->buildRedirectUrl($room, $token);

        // For API calls, return JSON
        if ($request->expectsJson()) {
            return response()->json([
                'success' => true,
                'redirect_url' => $redirectUrl,
            ]);
        }

        // For browser requests, redirect directly
        return redirect()->away($redirectUrl);
    }

    /**
     * Verify a JWT token (for trading room platform to validate)
     *
     * @param Request $request
     * @return JsonResponse
     */
    public function verifyToken(Request $request): JsonResponse
    {
        $request->validate([
            'token' => 'required|string',
        ]);

        try {
            $decoded = JWT::decode(
                $request->token,
                new \Firebase\JWT\Key($this->jwtSecret, 'HS256')
            );

            return response()->json([
                'success' => true,
                'valid' => true,
                'data' => [
                    'user_id' => $decoded->sub,
                    'user_email' => $decoded->email,
                    'user_name' => $decoded->name,
                    'room_slug' => $decoded->room_slug,
                    'room_id' => $decoded->room_id,
                    'exp' => $decoded->exp,
                ],
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'valid' => false,
                'message' => 'Invalid or expired token',
            ], 401);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PRIVATE METHODS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Create JWT token for user/room
     */
    private function createJwtToken(User $user, TradingRoom $room): string
    {
        $now = time();

        $payload = [
            'iss' => config('app.url'), // Issuer
            'sub' => $user->id, // Subject (user ID)
            'aud' => $this->tradingRoomBaseUrl, // Audience
            'iat' => $now, // Issued at
            'exp' => $now + $this->tokenExpiration, // Expiration
            'nbf' => $now, // Not before

            // User info
            'email' => $user->email,
            'name' => $user->name,
            'user_id' => $user->id,

            // Room info
            'room_id' => $room->id,
            'room_slug' => $room->slug,
            'room_name' => $room->name,
            'room_type' => $room->type,

            // Additional claims
            'roles' => $user->getRoleNames()->toArray(),
            'jti' => bin2hex(random_bytes(16)), // JWT ID (unique identifier)
        ];

        return JWT::encode($payload, $this->jwtSecret, 'HS256');
    }

    /**
     * Build redirect URL with JWT token
     */
    private function buildRedirectUrl(TradingRoom $room, string $token): string
    {
        // Check if room has custom URL
        $baseUrl = $room->metadata['room_url'] ?? $this->tradingRoomBaseUrl;
        $path = $room->metadata['room_path'] ?? "/{$room->slug}";

        return "{$baseUrl}/sso?jwt={$token}&redirect_to={$path}";
    }

    /**
     * Check if user has access to trading room
     */
    private function userHasAccess(User $user, TradingRoom $room): bool
    {
        // Cache key for quick lookup
        $cacheKey = "user_room_access:{$user->id}:{$room->id}";

        return Cache::remember($cacheKey, 300, function () use ($user, $room) {
            // Check if user has any active membership that grants access to this room
            // This checks based on the room slug matching a plan feature or product

            // Method 1: Check direct product ownership
            $hasProduct = $user->products()
                ->where('slug', $room->slug)
                ->wherePivot('revoked_at', null)
                ->exists();

            if ($hasProduct) {
                return true;
            }

            // Method 2: Check memberships with matching plan
            // Use database-agnostic JSON matching (compatible with both MySQL and SQLite)
            $hasMembership = $user->memberships()
                ->where('status', 'active')
                ->where(function ($query) {
                    $query->whereNull('expires_at')
                        ->orWhere('expires_at', '>', now());
                })
                ->whereHas('plan', function ($query) use ($room) {
                    // Check if plan slug matches room slug or plan includes room access
                    // Use LIKE for SQLite compatibility instead of whereJsonContains
                    $query->where('slug', $room->slug)
                        ->orWhere('features', 'like', '%"' . $room->slug . '"%')
                        ->orWhere('metadata', 'like', '%"rooms"%:%"' . $room->slug . '"%');
                })
                ->exists();

            if ($hasMembership) {
                return true;
            }

            // Method 3: Check if user has any active membership for trading rooms
            // (For demo/testing - in production, be more specific)
            $hasAnyTradingRoomMembership = $user->memberships()
                ->where('status', 'active')
                ->where(function ($query) {
                    $query->whereNull('expires_at')
                        ->orWhere('expires_at', '>', now());
                })
                ->whereHas('plan', function ($query) {
                    $query->where('slug', 'like', '%trading%')
                        ->orWhere('slug', 'like', '%room%')
                        ->orWhere('slug', 'like', '%mastering%')
                        ->orWhere('slug', 'like', '%showcase%');
                })
                ->exists();

            return $hasAnyTradingRoomMembership;
        });
    }

    /**
     * Get user's membership for a specific room
     */
    private function getUserMembershipForRoom(User $user, TradingRoom $room)
    {
        return $user->memberships()
            ->where('status', 'active')
            ->where(function ($query) {
                $query->whereNull('expires_at')
                    ->orWhere('expires_at', '>', now());
            })
            ->whereHas('plan', function ($query) use ($room) {
                // Use LIKE for SQLite compatibility instead of whereJsonContains
                $query->where('slug', $room->slug)
                    ->orWhere('features', 'like', '%"' . $room->slug . '"%')
                    ->orWhere('metadata', 'like', '%"rooms"%:%"' . $room->slug . '"%')
                    ->orWhere('slug', 'like', '%trading%');
            })
            ->with('plan')
            ->first();
    }

    /**
     * Get membership status label
     */
    private function getMembershipStatus($membership): string
    {
        if (!$membership) {
            return 'none';
        }

        // Check for trial
        if ($membership->trial_ends_at && $membership->trial_ends_at > now()) {
            return 'trial';
        }

        // Check if expiring soon (within 14 days)
        if ($membership->expires_at && $membership->expires_at->diffInDays(now()) <= 14) {
            return 'expiring';
        }

        // Check for paused
        if ($membership->status === 'paused') {
            return 'paused';
        }

        // Check for complimentary
        if ($membership->payment_provider === 'complimentary' || $membership->plan?->price == 0) {
            return 'complimentary';
        }

        return 'active';
    }
}
