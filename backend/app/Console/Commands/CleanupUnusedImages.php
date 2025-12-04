<?php

declare(strict_types=1);

namespace App\Console\Commands;

use App\Models\Media;
use App\Services\ImageOptimizationServiceEnhanced;
use Illuminate\Console\Command;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Notification;
use Illuminate\Support\Str;

/**
 * Cleanup unused and orphaned images
 *
 * This command identifies and optionally removes images that:
 * - Have zero usage count
 * - Haven't been accessed in X days
 * - Are orphaned (no database record)
 */
class CleanupUnusedImages extends Command
{
    protected $signature = 'media:cleanup
                            {--days=30 : Days of inactivity before marking as unused}
                            {--delete : Actually delete unused images (otherwise just report)}
                            {--include-orphans : Also cleanup orphaned files}
                            {--dry-run : Show what would be deleted without deleting}';

    protected $description = 'Cleanup unused and orphaned images from storage';

    public function handle(ImageOptimizationServiceEnhanced $service): int
    {
        $days = (int) $this->option('days');
        $shouldDelete = $this->option('delete') && !$this->option('dry-run');
        $includeOrphans = $this->option('include-orphans');
        $dryRun = $this->option('dry-run');

        $this->info('Image Cleanup Starting...');
        $this->newLine();

        // 1. Find unused images
        $this->info("Finding images unused for more than {$days} days...");
        $unused = $service->findUnusedImages($days);

        $this->table(
            ['ID', 'Filename', 'Size', 'Days Unused'],
            array_map(fn($item) => [
                $item['id'],
                Str::limit($item['filename'], 40),
                $this->formatBytes($item['size']),
                $item['days_unused'],
            ], array_slice($unused, 0, 20))
        );

        $unusedCount = count($unused);
        $unusedSize = array_sum(array_column($unused, 'size'));

        $this->info("Found {$unusedCount} unused images ({$this->formatBytes($unusedSize)})");
        $this->newLine();

        // 2. Find duplicates
        $this->info('Finding duplicate images...');
        $duplicates = $service->findDuplicates();

        $duplicateCount = 0;
        $duplicateSavings = 0;

        foreach ($duplicates as $group) {
            $duplicateCount += $group['count'] - 1;
            $duplicateSavings += $group['potential_savings'];
        }

        $this->info("Found {$duplicateCount} duplicate images (potential savings: {$this->formatBytes($duplicateSavings)})");
        $this->newLine();

        // 3. Find orphaned files (files on disk without database record)
        $orphanedFiles = [];
        $orphanedSize = 0;

        if ($includeOrphans) {
            $this->info('Scanning for orphaned files...');
            $orphanedFiles = $this->findOrphanedFiles();
            $orphanedSize = array_sum(array_column($orphanedFiles, 'size'));
            $this->info("Found " . count($orphanedFiles) . " orphaned files ({$this->formatBytes($orphanedSize)})");
            $this->newLine();
        }

        // Summary
        $totalPotentialSavings = $unusedSize + $duplicateSavings + $orphanedSize;

        $this->info('=== CLEANUP SUMMARY ===');
        $this->table(
            ['Category', 'Count', 'Size'],
            [
                ['Unused Images', $unusedCount, $this->formatBytes($unusedSize)],
                ['Duplicate Images', $duplicateCount, $this->formatBytes($duplicateSavings)],
                ['Orphaned Files', count($orphanedFiles), $this->formatBytes($orphanedSize)],
                ['Total Potential Savings', $unusedCount + $duplicateCount + count($orphanedFiles), $this->formatBytes($totalPotentialSavings)],
            ]
        );

        // Delete if requested
        if ($shouldDelete) {
            if (!$this->confirm('Are you sure you want to delete these files? This cannot be undone.')) {
                $this->info('Cleanup cancelled.');
                return Command::SUCCESS;
            }

            $this->info('Deleting unused images...');
            $bar = $this->output->createProgressBar($unusedCount);

            $deletedCount = 0;
            $deletedSize = 0;

            foreach ($unused as $item) {
                try {
                    $media = Media::find($item['id']);
                    if ($media) {
                        $deletedSize += $media->size;
                        $media->deleteFile();
                        $media->delete();
                        $deletedCount++;
                    }
                } catch (\Exception $e) {
                    Log::error('Failed to delete media', [
                        'id' => $item['id'],
                        'error' => $e->getMessage(),
                    ]);
                }
                $bar->advance();
            }

            $bar->finish();
            $this->newLine(2);

            $this->info("Deleted {$deletedCount} unused images ({$this->formatBytes($deletedSize)})");

            // Delete orphaned files
            if ($includeOrphans && count($orphanedFiles) > 0) {
                $this->info('Deleting orphaned files...');
                foreach ($orphanedFiles as $file) {
                    @unlink($file['path']);
                }
                $this->info('Deleted ' . count($orphanedFiles) . ' orphaned files');
            }

            Log::info('Image cleanup completed', [
                'deleted_count' => $deletedCount,
                'deleted_size' => $deletedSize,
                'orphaned_deleted' => count($orphanedFiles),
            ]);
        } elseif ($dryRun) {
            $this->warn('DRY RUN - No files were deleted. Run with --delete to actually remove files.');
        } else {
            $this->info('Run with --delete flag to remove unused images.');
        }

        return Command::SUCCESS;
    }

    /**
     * Find orphaned files (files without database records)
     */
    protected function findOrphanedFiles(): array
    {
        $orphaned = [];
        $mediaPath = storage_path('app/public/media');

        if (!is_dir($mediaPath)) {
            return [];
        }

        $iterator = new \RecursiveIteratorIterator(
            new \RecursiveDirectoryIterator($mediaPath, \RecursiveDirectoryIterator::SKIP_DOTS)
        );

        foreach ($iterator as $file) {
            if ($file->isFile()) {
                $relativePath = str_replace(storage_path('app/public/'), '', $file->getPathname());

                // Check if file exists in database
                $exists = Media::where('path', $relativePath)->exists();

                if (!$exists) {
                    $orphaned[] = [
                        'path' => $file->getPathname(),
                        'relative_path' => $relativePath,
                        'size' => $file->getSize(),
                        'modified' => date('Y-m-d H:i:s', $file->getMTime()),
                    ];
                }
            }
        }

        return $orphaned;
    }

    /**
     * Format bytes to human readable
     */
    protected function formatBytes(int $bytes): string
    {
        $units = ['B', 'KB', 'MB', 'GB'];
        $bytes = max($bytes, 0);
        $pow = floor(($bytes ? log($bytes) : 0) / log(1024));
        $pow = min($pow, count($units) - 1);
        return round($bytes / pow(1024, $pow), 2) . ' ' . $units[$pow];
    }
}
