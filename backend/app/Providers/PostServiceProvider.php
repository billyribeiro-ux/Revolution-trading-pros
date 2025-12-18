<?php

declare(strict_types=1);

namespace App\Providers;

use App\Contracts\PostServiceInterface;
use App\Services\Post\FeedService;
use App\Services\Post\PostService;
use Illuminate\Contracts\Support\DeferrableProvider;
use Illuminate\Support\ServiceProvider;

class PostServiceProvider extends ServiceProvider implements DeferrableProvider
{
    public function register(): void
    {
        $this->app->singleton(PostServiceInterface::class, function ($app) {
            return new PostService(
                $app['cache.store'],
                $app['log']
            );
        });

        $this->app->alias(PostServiceInterface::class, 'post.service');

        $this->app->singleton(FeedService::class, function ($app) {
            return new FeedService($app['cache.store']);
        });

        $this->app->alias(FeedService::class, 'post.feed');
    }

    public function boot(): void
    {
        $this->registerEventListeners();
    }

    public function provides(): array
    {
        return [
            PostServiceInterface::class,
            'post.service',
            FeedService::class,
            'post.feed',
        ];
    }

    private function registerEventListeners(): void
    {
        \App\Models\Post::saved(function ($post) {
            $this->app[PostServiceInterface::class]->invalidateRelatedPostsCache($post);
            $this->app[FeedService::class]->invalidateCache();
        });

        \App\Models\Post::deleted(function ($post) {
            $this->app[PostServiceInterface::class]->invalidateRelatedPostsCache();
            $this->app[FeedService::class]->invalidateCache();
        });
    }
}
