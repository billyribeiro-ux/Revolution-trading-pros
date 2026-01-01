<?php

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

/**
 * Middleware to ensure the user is an admin (or superadmin)
 * 
 * Usage in routes:
 * Route::middleware('admin')->group(function () {
 *     // Admin-only routes
 * });
 */
class EnsureAdmin
{
    /**
     * Handle an incoming request.
     */
    public function handle(Request $request, Closure $next): Response
    {
        $user = $request->user();

        if (!$user) {
            return response()->json([
                'message' => 'Unauthenticated.',
                'code' => 'UNAUTHENTICATED'
            ], 401);
        }

        // Check superadmin by email first
        $superadminEmails = config('superadmin.emails', []);
        $isSuperadmin = in_array(strtolower($user->email), array_map('strtolower', $superadminEmails));

        if ($isSuperadmin) {
            return $next($request);
        }

        // Check is_admin flag
        if ($user->is_admin ?? false) {
            return $next($request);
        }

        // Check roles via Spatie
        if (method_exists($user, 'hasAnyRole')) {
            if ($user->hasAnyRole(['admin', 'super-admin', 'super_admin', 'administrator'])) {
                return $next($request);
            }
        }

        return response()->json([
            'message' => 'This action requires admin privileges.',
            'code' => 'FORBIDDEN_ADMIN_REQUIRED'
        ], 403);
    }
}
