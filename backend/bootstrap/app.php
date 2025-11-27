<?php

use App\Http\Middleware\ApiVersion;
use App\Http\Middleware\ValidateSession;
use App\Http\Middleware\SecurityHeaders;
use App\Http\Middleware\SanitizeInput;
use Illuminate\Foundation\Application;
use Illuminate\Foundation\Configuration\Exceptions;
use Illuminate\Foundation\Configuration\Middleware;
use Illuminate\Support\Facades\Route;
use Spatie\Permission\Middleware\RoleMiddleware;
use Spatie\Permission\Middleware\PermissionMiddleware;
use Spatie\Permission\Middleware\RoleOrPermissionMiddleware;

return Application::configure(basePath: dirname(__DIR__))
    ->withRouting(
        web: __DIR__.'/../routes/web.php',
        api: __DIR__.'/../routes/api.php',
        commands: __DIR__.'/../routes/console.php',
        health: '/up',
        then: function () {
            // Load health check routes (L7+ Enterprise)
            Route::middleware('api')
                ->group(base_path('routes/health.php'));
        }
    )
    ->withMiddleware(function (Middleware $middleware): void {
        // Global middleware (applies to all requests)
        $middleware->prepend([
            SecurityHeaders::class,
        ]);

        // Route middleware aliases
        $middleware->alias([
            'role' => RoleMiddleware::class,
            'permission' => PermissionMiddleware::class,
            'role_or_permission' => RoleOrPermissionMiddleware::class,
            'session.validate' => ValidateSession::class,
            'security.headers' => SecurityHeaders::class,
            'sanitize.input' => SanitizeInput::class,
            'api.version' => ApiVersion::class,
        ]);

        // API middleware group
        $middleware->appendToGroup('api', [
            ApiVersion::class,
            SanitizeInput::class,
            ValidateSession::class,
        ]);
    })
    ->withExceptions(function (Exceptions $exceptions): void {
        // Custom exception handling with structured logging
        $exceptions->report(function (\Throwable $e) {
            // Log structured exception data for observability
            // Only log if the application is fully booted
            try {
                if (app()->hasBeenBootstrapped() && app()->bound('log')) {
                    \Illuminate\Support\Facades\Log::error('Application exception', [
                        'exception' => get_class($e),
                        'message' => $e->getMessage(),
                        'file' => $e->getFile(),
                        'line' => $e->getLine(),
                        'trace' => $e->getTraceAsString(),
                        'url' => request()?->fullUrl(),
                        'method' => request()?->method(),
                        'user_id' => auth()->id(),
                        'ip' => request()?->ip(),
                    ]);
                }
            } catch (\Throwable) {
                // Silently fail if logging isn't available during bootstrap
            }
        });
    })->create();
