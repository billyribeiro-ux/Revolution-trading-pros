<?php

declare(strict_types=1);

namespace App\Console\Commands;

use App\Contracts\PostServiceInterface;
use App\Models\Post;
use Illuminate\Console\Command;
use Illuminate\Support\Facades\DB;

class WarmRelatedPostsCache extends Command
{
    protected $signature = 'posts:warm-cache
                            {--algorithm=* : Algorithms to warm (default: hybrid)}
                            {--limit= : Maximum number of posts to process}
                            {--chunk=100 : Number of posts to process per chunk}
                            {--force : Force cache warming even if cache exists}';

    protected $description = 'Warm the related posts cache for all published posts';

    public function handle(PostServiceInterface $postService): int
    {
        $algorithms = $this->option('algorithm') ?: ['hybrid'];
        $limit = $this->option('limit') ? (int) $this->option('limit') : null;
        $chunkSize = (int) $this->option('chunk');
        $force = $this->option('force');

        $this->info('Starting related posts cache warming...');
        $this->info(sprintf('Algorithms: %s', implode(', ', $algorithms)));

        $query = Post::query()
            ->where('status', 'published')
            ->whereNotNull('published_at')
            ->orderByDesc('published_at');

        if ($limit) {
            $query->limit($limit);
            $this->info(sprintf('Processing up to %d posts', $limit));
        }

        $totalPosts = $query->count();
        $this->info(sprintf('Total posts to process: %d', $totalPosts));

        if ($force) {
            $this->warn('Force flag detected - invalidating existing cache...');
            $postService->invalidateRelatedPostsCache();
        }

        $processed = 0;
        $errors = 0;
        $bar = $this->output->createProgressBar($totalPosts);
        $bar->start();

        $query->chunk($chunkSize, function ($posts) use (
            $postService,
            $algorithms,
            &$processed,
            &$errors,
            $bar
        ) {
            foreach ($posts as $post) {
                try {
                    $postService->warmRelatedPostsCache($post, $algorithms);
                    $processed++;
                } catch (\Throwable $e) {
                    $errors++;
                    $this->error(sprintf(
                        "\nError processing post ID %d: %s",
                        $post->id,
                        $e->getMessage()
                    ));
                }
                
                $bar->advance();
                
                if ($processed % 100 === 0) {
                    DB::reconnect();
                }
            }
        });

        $bar->finish();
        $this->newLine(2);

        $this->info('Cache warming completed!');
        $this->table(
            ['Metric', 'Value'],
            [
                ['Posts processed', $processed],
                ['Errors', $errors],
                ['Success rate', sprintf('%.2f%%', ($processed / max($totalPosts, 1)) * 100)],
                ['Algorithms', implode(', ', $algorithms)],
            ]
        );

        return $errors > 0 ? self::FAILURE : self::SUCCESS;
    }
}
