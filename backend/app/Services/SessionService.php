<?php

namespace App\Services;

use App\Models\AuditLog;
use App\Models\User;
use App\Models\UserSession;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Jenssegers\Agent\Agent;

/**
 * Revolution Trading Pros - SessionService
 * Enterprise-grade session management with Redis caching
 * Implements Microsoft-style single-session authentication
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */
class SessionService
{
    /**
     * Cache key prefixes
     */
    const CACHE_SESSION_PREFIX = 'session:';
    const CACHE_USER_SESSIONS_PREFIX = 'user_sessions:';
    const CACHE_TTL = 300; // 5 minutes

    /**
     * Session expiry times
     */
    const ACCESS_TOKEN_TTL = 3600; // 1 hour
    const REFRESH_TOKEN_TTL = 2592000; // 30 days
    const SESSION_TTL = 86400; // 24 hours default

    /**
     * User agent parser
     */
    private Agent $agent;

    public function __construct()
    {
        $this->agent = new Agent();
    }

    /**
     * Create a new session for a user, revoking all previous sessions.
     * This implements Microsoft-style single-session authentication.
     */
    public function createSession(User $user, Request $request, bool $revokePrevious = true): UserSession
    {
        // Revoke all previous active sessions for single-session enforcement
        if ($revokePrevious) {
            $this->revokeAllUserSessions($user, 'new_login');
        }

        // Parse user agent for device info
        $this->agent->setUserAgent($request->userAgent());

        // Generate device fingerprint
        $deviceId = $this->generateDeviceFingerprint($request);

        // Detect device info
        $deviceInfo = $this->detectDeviceInfo($request);

        // Get geolocation from IP (could integrate with MaxMind or similar)
        $location = $this->getLocationFromIP($request->ip());

        // Create new session
        $session = UserSession::create([
            'user_id' => $user->id,
            'session_id' => (string) Str::uuid(),
            'device_id' => $deviceId,
            'device_name' => $deviceInfo['device_name'],
            'device_type' => $deviceInfo['device_type'],
            'os_name' => $deviceInfo['os_name'],
            'os_version' => $deviceInfo['os_version'],
            'browser_name' => $deviceInfo['browser_name'],
            'browser_version' => $deviceInfo['browser_version'],
            'ip_address' => $request->ip(),
            'user_agent' => $request->userAgent() ?? 'Unknown',
            'location_country' => $location['country'] ?? null,
            'location_city' => $location['city'] ?? null,
            'location_lat' => $location['lat'] ?? null,
            'location_lng' => $location['lng'] ?? null,
            'last_activity_at' => now(),
            'expires_at' => now()->addSeconds(self::SESSION_TTL),
            'is_active' => true,
            'metadata' => [
                'login_method' => $request->input('login_method', 'password'),
                'mfa_verified' => $request->input('mfa_verified', false),
            ],
        ]);

        // Cache the session for fast validation
        $this->cacheSession($session);

        // Log the login
        AuditLog::logAuth('login', $user, [
            'session_id' => $session->session_id,
            'device' => $deviceInfo['device_name'],
            'location' => $location['country'] ?? 'Unknown',
        ]);

        Log::info('New session created', [
            'user_id' => $user->id,
            'session_id' => $session->session_id,
            'device' => $deviceInfo['device_name'],
        ]);

        return $session;
    }

    /**
     * Validate a session by ID with Redis caching.
     */
    public function validateSession(string $sessionId): ?UserSession
    {
        $cacheKey = self::CACHE_SESSION_PREFIX . $sessionId;

        // Try to get from cache first
        $cached = Cache::get($cacheKey);

        if ($cached === false) {
            // Session was explicitly marked as invalid
            return null;
        }

        if ($cached instanceof UserSession) {
            // Verify it's still valid
            if ($cached->isValid()) {
                return $cached;
            }
            // Cached session expired, clear it
            Cache::forget($cacheKey);
            return null;
        }

        // Not in cache, query database
        $session = UserSession::where('session_id', $sessionId)
            ->where('is_active', true)
            ->where('expires_at', '>', now())
            ->first();

        if ($session) {
            // Cache the valid session
            $this->cacheSession($session);
            return $session;
        }

        // Mark as invalid in cache to prevent repeated DB queries
        Cache::put($cacheKey, false, self::CACHE_TTL);
        return null;
    }

    /**
     * Validate session for a specific user.
     */
    public function validateSessionForUser(User $user, string $sessionId): bool
    {
        $session = $this->validateSession($sessionId);

        return $session !== null && $session->user_id === $user->id;
    }

    /**
     * Revoke a specific session.
     */
    public function revokeSession(string $sessionId, string $reason = 'manual'): bool
    {
        $session = UserSession::where('session_id', $sessionId)->first();

        if (!$session) {
            return false;
        }

        $session->invalidate($reason);

        // Log the revocation
        AuditLog::logAuth('session_revoked', $session->user, [
            'session_id' => $sessionId,
            'reason' => $reason,
        ]);

        Log::info('Session revoked', [
            'session_id' => $sessionId,
            'user_id' => $session->user_id,
            'reason' => $reason,
        ]);

        return true;
    }

    /**
     * Revoke all sessions for a user.
     */
    public function revokeAllUserSessions(User $user, string $reason = 'manual', ?string $exceptSessionId = null): int
    {
        $query = UserSession::where('user_id', $user->id)
            ->where('is_active', true);

        if ($exceptSessionId) {
            $query->where('session_id', '!=', $exceptSessionId);
        }

        $sessions = $query->get();
        $count = 0;

        foreach ($sessions as $session) {
            $session->invalidate($reason);
            $count++;
        }

        // Clear user sessions cache
        $this->clearUserSessionsCache($user->id);

        if ($count > 0) {
            AuditLog::logAuth('all_sessions_revoked', $user, [
                'count' => $count,
                'reason' => $reason,
                'except' => $exceptSessionId,
            ]);

            Log::info('All user sessions revoked', [
                'user_id' => $user->id,
                'count' => $count,
                'reason' => $reason,
            ]);
        }

        return $count;
    }

    /**
     * Get all active sessions for a user.
     */
    public function getUserActiveSessions(User $user): array
    {
        $cacheKey = self::CACHE_USER_SESSIONS_PREFIX . $user->id;

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($user) {
            return UserSession::where('user_id', $user->id)
                ->where('is_active', true)
                ->where('expires_at', '>', now())
                ->orderBy('last_activity_at', 'desc')
                ->get()
                ->map(fn($s) => $s->toApiArray())
                ->toArray();
        });
    }

    /**
     * Update session last activity.
     */
    public function touchSession(string $sessionId): bool
    {
        $session = UserSession::where('session_id', $sessionId)->first();

        if (!$session || !$session->isValid()) {
            return false;
        }

        $session->last_activity_at = now();
        $session->save();

        // Update cache
        $this->cacheSession($session);

        return true;
    }

    /**
     * Extend session expiry.
     */
    public function extendSession(string $sessionId, int $seconds = null): bool
    {
        $session = UserSession::where('session_id', $sessionId)->first();

        if (!$session || !$session->is_active) {
            return false;
        }

        $seconds = $seconds ?? self::SESSION_TTL;
        $session->expires_at = now()->addSeconds($seconds);
        $session->save();

        $this->cacheSession($session);

        return true;
    }

    /**
     * Clean up expired sessions (for cron job).
     */
    public function cleanupExpiredSessions(): int
    {
        $expiredSessions = UserSession::where('is_active', true)
            ->where('expires_at', '<', now())
            ->get();

        $count = 0;
        foreach ($expiredSessions as $session) {
            $session->invalidate('expired');
            $count++;
        }

        Log::info('Expired sessions cleaned up', ['count' => $count]);

        return $count;
    }

    /**
     * Generate a device fingerprint from request data.
     */
    private function generateDeviceFingerprint(Request $request): string
    {
        $components = [
            $request->userAgent() ?? '',
            $request->header('Accept-Language', ''),
            $request->header('Accept-Encoding', ''),
        ];

        return hash('sha256', implode('|', $components));
    }

    /**
     * Detect device information from request.
     */
    private function detectDeviceInfo(Request $request): array
    {
        $this->agent->setUserAgent($request->userAgent());

        // Determine device type
        $deviceType = 'unknown';
        if ($this->agent->isDesktop()) {
            $deviceType = 'desktop';
        } elseif ($this->agent->isTablet()) {
            $deviceType = 'tablet';
        } elseif ($this->agent->isMobile()) {
            $deviceType = 'mobile';
        }

        // Get browser info
        $browser = $this->agent->browser();
        $browserVersion = $this->agent->version($browser);

        // Get OS info
        $platform = $this->agent->platform();
        $platformVersion = $this->agent->version($platform);

        // Generate device name
        $deviceName = $this->agent->device();
        if ($deviceName === false || $deviceName === 'WebKit') {
            $deviceName = $browser . ' on ' . $platform;
        }

        return [
            'device_name' => $deviceName ?: 'Unknown Device',
            'device_type' => $deviceType,
            'os_name' => $platform ?: null,
            'os_version' => $platformVersion ?: null,
            'browser_name' => $browser ?: null,
            'browser_version' => $browserVersion ?: null,
        ];
    }

    /**
     * Get location from IP address.
     * TODO: Integrate with MaxMind GeoIP2 or similar service
     */
    private function getLocationFromIP(string $ip): array
    {
        // Skip for localhost/private IPs
        if (in_array($ip, ['127.0.0.1', '::1']) || $this->isPrivateIP($ip)) {
            return [
                'country' => 'Local',
                'city' => null,
                'lat' => null,
                'lng' => null,
            ];
        }

        // TODO: Implement actual geolocation lookup
        // For now, return empty to avoid external dependencies
        return [
            'country' => null,
            'city' => null,
            'lat' => null,
            'lng' => null,
        ];
    }

    /**
     * Check if IP is private.
     */
    private function isPrivateIP(string $ip): bool
    {
        return !filter_var(
            $ip,
            FILTER_VALIDATE_IP,
            FILTER_FLAG_NO_PRIV_RANGE | FILTER_FLAG_NO_RES_RANGE
        );
    }

    /**
     * Cache a session for fast validation.
     */
    private function cacheSession(UserSession $session): void
    {
        $cacheKey = self::CACHE_SESSION_PREFIX . $session->session_id;
        Cache::put($cacheKey, $session, self::CACHE_TTL);
    }

    /**
     * Clear user sessions cache.
     */
    private function clearUserSessionsCache(int $userId): void
    {
        Cache::forget(self::CACHE_USER_SESSIONS_PREFIX . $userId);
    }

    /**
     * Get session statistics for admin dashboard.
     */
    public function getSessionStats(): array
    {
        return Cache::remember('session_stats', 60, function () {
            $activeSessions = UserSession::where('is_active', true)
                ->where('expires_at', '>', now())
                ->count();

            $deviceBreakdown = UserSession::where('is_active', true)
                ->where('expires_at', '>', now())
                ->selectRaw('device_type, COUNT(*) as count')
                ->groupBy('device_type')
                ->pluck('count', 'device_type')
                ->toArray();

            $recentLogins = UserSession::where('created_at', '>', now()->subHours(24))
                ->count();

            return [
                'active_sessions' => $activeSessions,
                'device_breakdown' => $deviceBreakdown,
                'logins_24h' => $recentLogins,
            ];
        });
    }
}
