<?php

declare(strict_types=1);

namespace App\Contracts;

use App\Models\Post;
use Illuminate\Support\Collection;

interface PostServiceInterface
{
    public function getRelatedPosts(Post $post, int $limit = 5, string $algorithm = 'hybrid'): Collection;
    public function getRelatedPostsWithScores(Post $post, int $limit = 5, string $algorithm = 'hybrid'): Collection;
    public function warmRelatedPostsCache(Post $post, array $algorithms = ['hybrid']): void;
    public function invalidateRelatedPostsCache(?Post $post = null): void;
}
