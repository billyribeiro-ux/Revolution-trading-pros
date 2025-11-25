<?php

namespace App\Http\Middleware;

use App\Models\AuditLog;
use App\Services\SessionService;
use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

/**
 * Revolution Trading Pros - ValidateSession Middleware
 * Validates user session for single-session authentication
 * Returns SESSION_INVALIDATED code when session is no longer valid
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */
class ValidateSession
{
    /**
     * Session service instance
     */
    private SessionService $sessionService;

    /**
     * Routes that don't require session validation
     */
    private array $excludedRoutes = [
        'api/login',
        'api/register',
        'api/forgot-password',
        'api/reset-password',
        'api/auth/refresh',
    ];

    public function __construct(SessionService $sessionService)
    {
        $this->sessionService = $sessionService;
    }

    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        // Skip validation for excluded routes
        if ($this->isExcludedRoute($request)) {
            return $next($request);
        }

        // Skip if user is not authenticated
        if (!$request->user()) {
            return $next($request);
        }

        // Get session ID from header
        $sessionId = $request->header('X-Session-ID');

        if (!$sessionId) {
            // No session ID provided - for backwards compatibility,
            // allow requests without session ID but log it
            // In strict mode, you would return 401 here
            return $next($request);
        }

        // Validate the session
        $session = $this->sessionService->validateSession($sessionId);

        if (!$session) {
            // Session is invalid or expired
            AuditLog::logAuth('session_invalid_access', $request->user(), [
                'session_id' => $sessionId,
                'ip' => $request->ip(),
            ]);

            return response()->json([
                'message' => 'Your session has been invalidated. You may have been signed out because you signed in from another device.',
                'code' => 'SESSION_INVALIDATED',
                'requires_reauth' => true,
            ], 401);
        }

        // Verify session belongs to current user
        if ($session->user_id !== $request->user()->id) {
            AuditLog::logAuth('session_user_mismatch', $request->user(), [
                'session_id' => $sessionId,
                'session_user_id' => $session->user_id,
            ]);

            return response()->json([
                'message' => 'Session does not belong to authenticated user.',
                'code' => 'SESSION_USER_MISMATCH',
                'requires_reauth' => true,
            ], 401);
        }

        // Update last activity (throttled to prevent excessive DB writes)
        $this->throttledActivityUpdate($session);

        // Add session to request for downstream use
        $request->attributes->set('user_session', $session);

        return $next($request);
    }

    /**
     * Check if the route is excluded from session validation.
     */
    private function isExcludedRoute(Request $request): bool
    {
        $path = $request->path();

        foreach ($this->excludedRoutes as $route) {
            if (str_starts_with($path, $route)) {
                return true;
            }
        }

        return false;
    }

    /**
     * Update session activity with throttling to reduce DB load.
     */
    private function throttledActivityUpdate($session): void
    {
        // Only update if last activity was more than 1 minute ago
        if ($session->last_activity_at->diffInMinutes(now()) >= 1) {
            $this->sessionService->touchSession($session->session_id);
        }
    }
}
