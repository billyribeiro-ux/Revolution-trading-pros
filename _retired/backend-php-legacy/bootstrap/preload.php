<?php

declare(strict_types=1);

/**
 * OPcache Preloading Script (ICT9+ Enterprise Grade)
 *
 * Preloads frequently used classes into OPcache for faster boot times.
 * Configure in php.ini: opcache.preload=/path/to/preload.php
 *
 * @version 1.0.0
 */

// Only run in production
if (php_sapi_name() === 'cli' && !in_array('--preload', $_SERVER['argv'] ?? [])) {
    return;
}

require_once __DIR__ . '/../vendor/autoload.php';

/**
 * Preload helper function
 */
function preloadDirectory(string $directory, array $exclude = []): int
{
    $count = 0;

    if (!is_dir($directory)) {
        return $count;
    }

    $iterator = new RecursiveIteratorIterator(
        new RecursiveDirectoryIterator($directory, FilesystemIterator::SKIP_DOTS)
    );

    foreach ($iterator as $file) {
        if (!$file->isFile() || $file->getExtension() !== 'php') {
            continue;
        }

        $path = $file->getPathname();

        // Skip excluded paths
        foreach ($exclude as $excludePattern) {
            if (str_contains($path, $excludePattern)) {
                continue 2;
            }
        }

        try {
            opcache_compile_file($path);
            $count++;
        } catch (Throwable $e) {
            // Skip files that can't be preloaded
        }
    }

    return $count;
}

// Laravel core
$count = 0;

// Preload Laravel framework core classes
$count += preloadDirectory(__DIR__ . '/../vendor/laravel/framework/src/Illuminate/Support');
$count += preloadDirectory(__DIR__ . '/../vendor/laravel/framework/src/Illuminate/Database');
$count += preloadDirectory(__DIR__ . '/../vendor/laravel/framework/src/Illuminate/Http');
$count += preloadDirectory(__DIR__ . '/../vendor/laravel/framework/src/Illuminate/Routing');
$count += preloadDirectory(__DIR__ . '/../vendor/laravel/framework/src/Illuminate/Cache');

// Preload application code
$count += preloadDirectory(__DIR__ . '/../app/Models');
$count += preloadDirectory(__DIR__ . '/../app/Services', ['Test']);
$count += preloadDirectory(__DIR__ . '/../app/Http/Controllers');
$count += preloadDirectory(__DIR__ . '/../app/Http/Middleware');
$count += preloadDirectory(__DIR__ . '/../app/Providers');
$count += preloadDirectory(__DIR__ . '/../app/Traits');

// Log preload results
error_log("Preloaded {$count} files");
