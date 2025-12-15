<?php

namespace App\Providers;

use Illuminate\Http\Request;
use Illuminate\Routing\UrlGenerator;
use Illuminate\Support\ServiceProvider;

/**
 * Custom URL Generator Service Provider
 * 
 * Fixes Laravel 12 + Octane compatibility issue where UrlGenerator
 * gets created with null request during certain lifecycle events.
 */
class UrlGeneratorServiceProvider extends ServiceProvider
{
    /**
     * Register the URL generator service.
     */
    public function register(): void
    {
        $this->app->singleton('url', function ($app) {
            $routes = $app['router']->getRoutes();

            // Get request or create a synthetic one
            $request = $app->bound('request') 
                ? $app['request'] 
                : $this->createSyntheticRequest($app);

            $url = new UrlGenerator(
                $routes,
                $request,
                $app['config']['app.asset_url']
            );

            // Force root URL from config
            $url->forceRootUrl($app['config']['app.url'] ?? 'http://localhost');
            
            if (str_starts_with($app['config']['app.url'] ?? '', 'https://')) {
                $url->forceScheme('https');
            }

            // Set session resolver
            $url->setSessionResolver(function () use ($app) {
                return $app['session'] ?? null;
            });

            // Set key resolver for signed URLs
            $url->setKeyResolver(function () use ($app) {
                return $app['config']['app.key'];
            });

            return $url;
        });
    }

    /**
     * Create a synthetic request when none exists.
     */
    protected function createSyntheticRequest($app): Request
    {
        $appUrl = $app['config']['app.url'] ?? 'http://localhost';
        $parsed = parse_url($appUrl);
        
        $request = Request::create(
            $appUrl,
            'GET',
            [],
            [],
            [],
            [
                'HTTP_HOST' => $parsed['host'] ?? 'localhost',
                'HTTPS' => ($parsed['scheme'] ?? 'http') === 'https' ? 'on' : 'off',
            ]
        );

        return $request;
    }
}
