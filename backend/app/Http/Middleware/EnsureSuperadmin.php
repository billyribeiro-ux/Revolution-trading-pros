<?php

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

/**
 * Middleware to ensure the user is a superadmin
 * 
 * Usage in routes:
 * Route::middleware('superadmin')->group(function () {
 *     // Superadmin-only routes
 * });
 */
class EnsureSuperadmin
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

        $superadminEmails = config('superadmin.emails', []);
        $isSuperadmin = in_array(strtolower($user->email), array_map('strtolower', $superadminEmails));

        if (!$isSuperadmin) {
            return response()->json([
                'message' => 'This action requires superadmin privileges.',
                'code' => 'FORBIDDEN_SUPERADMIN_REQUIRED'
            ], 403);
        }

        return $next($request);
    }
}
