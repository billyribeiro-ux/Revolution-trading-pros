<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Media;
use App\Models\MediaVariant;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Collection;

/**
 * ImageIntelligenceService
 *
 * AI-powered image analysis and optimization intelligence:
 * - Content type detection (photo, illustration, graphic, text)
 * - Auto-compression level recommendation
 * - SEO alt/title suggestions
 * - Duplicate detection
 * - Storage optimization recommendations
 * - Brand consistency checking
 */
class ImageIntelligenceService
{
    protected array $config;

    public function __construct()
    {
        $this->config = config('image-optimization.content_detection', []);
    }

    /**
     * Analyze image and return comprehensive insights
     */
    public function analyzeImage(Media $media): array
    {
        if (!$media->isImage()) {
            return ['error' => 'Not an image'];
        }

        $analysis = [
            'media_id' => $media->id,
            'filename' => $media->filename,
            'analyzed_at' => now()->toIso8601String(),
        ];

        try {
            // Content type detection
            $analysis['content_type'] = $this->detectContentType($media);

            // Compression recommendation
            $analysis['compression'] = $this->getCompressionRecommendation($media, $analysis['content_type']);

            // SEO suggestions
            $analysis['seo'] = $this->getSeoSuggestions($media);

            // Optimization status
            $analysis['optimization'] = $this->getOptimizationStatus($media);

            // Storage recommendations
            $analysis['storage'] = $this->getStorageRecommendations($media);

            // Similar images (potential duplicates)
            $analysis['similar_images'] = $this->findSimilarImages($media);

            // Quality score
            $analysis['quality_score'] = $this->calculateQualityScore($media, $analysis);

        } catch (\Exception $e) {
            Log::error('Image analysis failed', [
                'media_id' => $media->id,
                'error' => $e->getMessage(),
            ]);
            $analysis['error'] = $e->getMessage();
        }

        return $analysis;
    }

    /**
     * Detect content type using image analysis
     */
    public function detectContentType(Media $media): array
    {
        $sourcePath = Storage::disk($media->disk)->path($media->path);

        if (!file_exists($sourcePath)) {
            return ['type' => 'unknown', 'confidence' => 0];
        }

        try {
            // Analyze image characteristics
            $characteristics = $this->analyzeImageCharacteristics($sourcePath);

            // Determine content type based on characteristics
            $type = $this->classifyContentType($characteristics);

            return [
                'type' => $type['type'],
                'confidence' => $type['confidence'],
                'characteristics' => $characteristics,
            ];
        } catch (\Exception $e) {
            return ['type' => 'unknown', 'confidence' => 0, 'error' => $e->getMessage()];
        }
    }

    /**
     * Analyze image characteristics for content type detection
     */
    protected function analyzeImageCharacteristics(string $path): array
    {
        $image = imagecreatefromstring(file_get_contents($path));
        if (!$image) {
            return [];
        }

        $width = imagesx($image);
        $height = imagesy($image);

        // Sample colors from the image
        $colors = $this->sampleColors($image, $width, $height);

        // Calculate color statistics
        $colorStats = $this->calculateColorStatistics($colors);

        // Detect edges (indicates detail/sharpness)
        $edgeScore = $this->calculateEdgeScore($image, $width, $height);

        // Detect gradients (photos have smooth gradients, graphics have sharp edges)
        $gradientScore = $this->calculateGradientScore($colors);

        // Detect transparency
        $hasTransparency = $this->detectTransparency($image, $width, $height);

        imagedestroy($image);

        return [
            'dimensions' => ['width' => $width, 'height' => $height],
            'color_count' => $colorStats['unique_colors'],
            'color_variance' => $colorStats['variance'],
            'dominant_colors' => $colorStats['dominant'],
            'edge_score' => $edgeScore,
            'gradient_score' => $gradientScore,
            'has_transparency' => $hasTransparency,
            'aspect_ratio' => $width / max($height, 1),
        ];
    }

    /**
     * Sample colors from image
     */
    protected function sampleColors($image, int $width, int $height, int $samples = 1000): array
    {
        $colors = [];
        $stepX = max(1, (int)($width / sqrt($samples)));
        $stepY = max(1, (int)($height / sqrt($samples)));

        for ($x = 0; $x < $width; $x += $stepX) {
            for ($y = 0; $y < $height; $y += $stepY) {
                $rgb = imagecolorat($image, $x, $y);
                $colors[] = [
                    'r' => ($rgb >> 16) & 0xFF,
                    'g' => ($rgb >> 8) & 0xFF,
                    'b' => $rgb & 0xFF,
                ];
            }
        }

        return $colors;
    }

    /**
     * Calculate color statistics
     */
    protected function calculateColorStatistics(array $colors): array
    {
        $uniqueColors = [];
        $rSum = $gSum = $bSum = 0;
        $count = count($colors);

        foreach ($colors as $color) {
            $key = "{$color['r']},{$color['g']},{$color['b']}";
            $uniqueColors[$key] = ($uniqueColors[$key] ?? 0) + 1;
            $rSum += $color['r'];
            $gSum += $color['g'];
            $bSum += $color['b'];
        }

        $rMean = $rSum / max($count, 1);
        $gMean = $gSum / max($count, 1);
        $bMean = $bSum / max($count, 1);

        // Calculate variance
        $variance = 0;
        foreach ($colors as $color) {
            $variance += pow($color['r'] - $rMean, 2) + pow($color['g'] - $gMean, 2) + pow($color['b'] - $bMean, 2);
        }
        $variance = $variance / max($count, 1);

        // Get dominant colors
        arsort($uniqueColors);
        $dominant = array_slice(array_keys($uniqueColors), 0, 5);

        return [
            'unique_colors' => count($uniqueColors),
            'variance' => $variance,
            'dominant' => $dominant,
            'mean' => ['r' => $rMean, 'g' => $gMean, 'b' => $bMean],
        ];
    }

    /**
     * Calculate edge score (higher = more detailed/sharp)
     */
    protected function calculateEdgeScore($image, int $width, int $height): float
    {
        $edgeSum = 0;
        $samples = 0;

        for ($x = 1; $x < $width - 1; $x += 5) {
            for ($y = 1; $y < $height - 1; $y += 5) {
                $center = imagecolorat($image, $x, $y) & 0xFF;
                $left = imagecolorat($image, $x - 1, $y) & 0xFF;
                $right = imagecolorat($image, $x + 1, $y) & 0xFF;
                $top = imagecolorat($image, $x, $y - 1) & 0xFF;
                $bottom = imagecolorat($image, $x, $y + 1) & 0xFF;

                $edgeSum += abs($center - $left) + abs($center - $right) + abs($center - $top) + abs($center - $bottom);
                $samples++;
            }
        }

        return $samples > 0 ? $edgeSum / ($samples * 4 * 255) : 0;
    }

    /**
     * Calculate gradient score (higher = smoother gradients)
     */
    protected function calculateGradientScore(array $colors): float
    {
        if (count($colors) < 2) {
            return 0;
        }

        $gradientSum = 0;
        for ($i = 1; $i < count($colors); $i++) {
            $diff = abs($colors[$i]['r'] - $colors[$i-1]['r']) +
                    abs($colors[$i]['g'] - $colors[$i-1]['g']) +
                    abs($colors[$i]['b'] - $colors[$i-1]['b']);
            $gradientSum += $diff < 30 ? 1 : 0; // Smooth transition
        }

        return $gradientSum / (count($colors) - 1);
    }

    /**
     * Detect if image has transparency
     */
    protected function detectTransparency($image, int $width, int $height): bool
    {
        // Check for alpha channel
        $hasAlpha = imagecolortransparent($image) !== -1;

        if (!$hasAlpha && imageistruecolor($image)) {
            // Sample some pixels for alpha
            for ($x = 0; $x < $width; $x += max(1, $width / 10)) {
                for ($y = 0; $y < $height; $y += max(1, $height / 10)) {
                    $rgba = imagecolorat($image, (int)$x, (int)$y);
                    $alpha = ($rgba >> 24) & 0x7F;
                    if ($alpha > 0) {
                        return true;
                    }
                }
            }
        }

        return $hasAlpha;
    }

    /**
     * Classify content type based on characteristics
     */
    protected function classifyContentType(array $chars): array
    {
        $scores = [
            'photo' => 0,
            'illustration' => 0,
            'graphic' => 0,
            'text' => 0,
            'screenshot' => 0,
        ];

        // Photos: High color variance, many unique colors, smooth gradients
        if (($chars['color_count'] ?? 0) > 10000) {
            $scores['photo'] += 3;
        }
        if (($chars['color_variance'] ?? 0) > 5000) {
            $scores['photo'] += 2;
        }
        if (($chars['gradient_score'] ?? 0) > 0.6) {
            $scores['photo'] += 2;
        }

        // Illustrations: Medium color count, moderate variance
        if (($chars['color_count'] ?? 0) > 1000 && ($chars['color_count'] ?? 0) < 10000) {
            $scores['illustration'] += 2;
        }
        if (($chars['edge_score'] ?? 0) > 0.1 && ($chars['edge_score'] ?? 0) < 0.3) {
            $scores['illustration'] += 2;
        }

        // Graphics: Low color count, high edge score, possibly transparent
        if (($chars['color_count'] ?? 0) < 1000) {
            $scores['graphic'] += 3;
        }
        if ($chars['has_transparency'] ?? false) {
            $scores['graphic'] += 2;
        }

        // Text/Screenshots: Very high edge score, specific aspect ratios
        if (($chars['edge_score'] ?? 0) > 0.3) {
            $scores['text'] += 2;
            $scores['screenshot'] += 1;
        }
        if (($chars['aspect_ratio'] ?? 1) > 1.3 && ($chars['aspect_ratio'] ?? 1) < 2.0) {
            $scores['screenshot'] += 2;
        }

        // Find winner
        arsort($scores);
        $winner = array_key_first($scores);
        $maxScore = $scores[$winner];
        $totalScore = array_sum($scores) ?: 1;

        return [
            'type' => $winner,
            'confidence' => round($maxScore / $totalScore, 2),
            'scores' => $scores,
        ];
    }

    /**
     * Get compression recommendation based on content type
     */
    public function getCompressionRecommendation(Media $media, array $contentType): array
    {
        $type = $contentType['type'] ?? 'photo';
        $confidence = $contentType['confidence'] ?? 0.5;

        $recommendations = [
            'photo' => [
                'format' => 'webp',
                'quality' => 85,
                'mode' => 'lossy',
                'rationale' => 'Photos compress well with lossy WebP while maintaining visual quality.',
            ],
            'illustration' => [
                'format' => 'webp',
                'quality' => 90,
                'mode' => 'lossy',
                'rationale' => 'Illustrations benefit from higher quality to preserve details.',
            ],
            'graphic' => [
                'format' => 'png',
                'quality' => 95,
                'mode' => 'lossless',
                'rationale' => 'Graphics with few colors and transparency should use lossless compression.',
            ],
            'text' => [
                'format' => 'png',
                'quality' => 95,
                'mode' => 'lossless',
                'rationale' => 'Text images require lossless compression to maintain readability.',
            ],
            'screenshot' => [
                'format' => 'webp',
                'quality' => 90,
                'mode' => 'lossy',
                'rationale' => 'Screenshots balance quality and size with high-quality lossy compression.',
            ],
        ];

        $rec = $recommendations[$type] ?? $recommendations['photo'];

        // Adjust quality based on file size
        if ($media->size > 5 * 1024 * 1024) { // > 5MB
            $rec['quality'] = max(70, $rec['quality'] - 10);
            $rec['rationale'] .= ' Quality reduced due to large file size.';
        }

        return array_merge($rec, [
            'content_type' => $type,
            'confidence' => $confidence,
        ]);
    }

    /**
     * Get SEO suggestions for media
     */
    public function getSeoSuggestions(Media $media): array
    {
        $suggestions = [
            'alt_text' => null,
            'title' => null,
            'description' => null,
            'issues' => [],
            'score' => 100,
        ];

        // Check alt text
        if (empty($media->alt_text)) {
            $suggestions['issues'][] = 'Missing alt text - important for accessibility and SEO';
            $suggestions['score'] -= 30;

            // Generate suggestion from filename
            $suggestions['alt_text'] = $this->generateAltTextSuggestion($media->filename);
        }

        // Check title
        if (empty($media->title)) {
            $suggestions['issues'][] = 'Missing title attribute';
            $suggestions['score'] -= 10;
            $suggestions['title'] = $this->generateTitleSuggestion($media->filename);
        }

        // Check filename
        if ($this->hasUnfriendlyFilename($media->filename)) {
            $suggestions['issues'][] = 'Filename could be more SEO-friendly';
            $suggestions['score'] -= 10;
        }

        // Check file size for web
        if ($media->size > 500 * 1024) { // > 500KB
            $suggestions['issues'][] = 'Large file size may affect page load speed';
            $suggestions['score'] -= 20;
        }

        // Check dimensions
        if ($media->width && $media->width > 2560) {
            $suggestions['issues'][] = 'Image dimensions larger than typically needed for web';
            $suggestions['score'] -= 10;
        }

        return $suggestions;
    }

    /**
     * Generate alt text suggestion from filename
     */
    protected function generateAltTextSuggestion(string $filename): string
    {
        $name = pathinfo($filename, PATHINFO_FILENAME);

        // Remove common patterns
        $name = preg_replace('/[-_]/', ' ', $name);
        $name = preg_replace('/\d+/', '', $name);
        $name = preg_replace('/\s+/', ' ', $name);
        $name = trim($name);

        // Capitalize words
        $name = ucwords(strtolower($name));

        return $name ?: 'Image';
    }

    /**
     * Generate title suggestion from filename
     */
    protected function generateTitleSuggestion(string $filename): string
    {
        return $this->generateAltTextSuggestion($filename);
    }

    /**
     * Check if filename is SEO-unfriendly
     */
    protected function hasUnfriendlyFilename(string $filename): bool
    {
        $name = pathinfo($filename, PATHINFO_FILENAME);

        // Check for UUID patterns
        if (preg_match('/^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$/i', $name)) {
            return true;
        }

        // Check for random character strings
        if (preg_match('/^[a-z0-9]{20,}$/i', $name)) {
            return true;
        }

        // Check for generic names
        $genericNames = ['image', 'photo', 'img', 'picture', 'screenshot', 'untitled'];
        if (in_array(strtolower($name), $genericNames)) {
            return true;
        }

        return false;
    }

    /**
     * Get optimization status and potential savings
     */
    public function getOptimizationStatus(Media $media): array
    {
        $status = [
            'is_optimized' => $media->is_optimized,
            'has_webp' => false,
            'has_avif' => false,
            'has_responsive' => false,
            'has_retina' => false,
            'potential_savings' => 0,
            'current_savings' => 0,
        ];

        // Check existing variants
        $variants = MediaVariant::where('media_id', $media->id)->get();

        $status['has_webp'] = $variants->contains('format', 'webp');
        $status['has_avif'] = $variants->contains('format', 'avif');
        $status['has_responsive'] = $variants->where('variant_type', 'responsive')->isNotEmpty();
        $status['has_retina'] = $variants->where('is_retina', true)->isNotEmpty();

        // Calculate current savings
        $totalSaved = $variants->sum('bytes_saved');
        $status['current_savings'] = $totalSaved;

        // Estimate potential savings if not optimized
        if (!$media->is_optimized) {
            // Estimate ~40% savings with WebP conversion
            $status['potential_savings'] = (int)($media->size * 0.4);
        }

        return $status;
    }

    /**
     * Get storage recommendations
     */
    public function getStorageRecommendations(Media $media): array
    {
        $recommendations = [];

        // Check for unused media
        if ($media->usage_count === 0 && $media->created_at < now()->subDays(30)) {
            $recommendations[] = [
                'type' => 'cleanup',
                'message' => 'This media has not been used in 30+ days. Consider removing if not needed.',
                'priority' => 'low',
            ];
        }

        // Check for very large files
        if ($media->size > 10 * 1024 * 1024) { // > 10MB
            $recommendations[] = [
                'type' => 'optimize',
                'message' => 'Very large file size. Consider optimizing or resizing.',
                'priority' => 'high',
            ];
        }

        // Check for multiple versions
        $variantCount = MediaVariant::where('media_id', $media->id)->count();
        if ($variantCount > 20) {
            $recommendations[] = [
                'type' => 'cleanup',
                'message' => 'Many variants generated. Consider cleaning up unused sizes.',
                'priority' => 'medium',
            ];
        }

        return $recommendations;
    }

    /**
     * Find similar images (potential duplicates)
     */
    public function findSimilarImages(Media $media, int $limit = 5): array
    {
        if (!$media->hash) {
            return [];
        }

        // Find by exact hash (duplicates)
        $duplicates = Media::where('id', '!=', $media->id)
            ->where('hash', $media->hash)
            ->limit($limit)
            ->get(['id', 'filename', 'url', 'size'])
            ->toArray();

        // Find by similar dimensions
        $similar = [];
        if ($media->width && $media->height) {
            $similar = Media::where('id', '!=', $media->id)
                ->where('width', $media->width)
                ->where('height', $media->height)
                ->where('hash', '!=', $media->hash)
                ->limit($limit)
                ->get(['id', 'filename', 'url', 'size'])
                ->toArray();
        }

        return [
            'duplicates' => $duplicates,
            'similar_dimensions' => $similar,
        ];
    }

    /**
     * Calculate overall quality score
     */
    protected function calculateQualityScore(Media $media, array $analysis): int
    {
        $score = 100;

        // Deduct for missing optimization
        if (!$media->is_optimized) {
            $score -= 20;
        }

        // Add SEO score contribution
        $seoScore = $analysis['seo']['score'] ?? 100;
        $score = (int)(($score + $seoScore) / 2);

        // Deduct for very large files
        if ($media->size > 2 * 1024 * 1024) {
            $score -= 10;
        }

        // Deduct for low confidence content detection
        $confidence = $analysis['content_type']['confidence'] ?? 0;
        if ($confidence < 0.5) {
            $score -= 5;
        }

        return max(0, min(100, $score));
    }

    /**
     * Get bulk analysis for multiple media items
     */
    public function bulkAnalyze(array $mediaIds): array
    {
        $results = [];

        foreach ($mediaIds as $mediaId) {
            $media = Media::find($mediaId);
            if ($media) {
                $results[$mediaId] = $this->analyzeImage($media);
            }
        }

        return $results;
    }

    /**
     * Get optimization recommendations for the entire library
     */
    public function getLibraryInsights(): array
    {
        $stats = [
            'total_images' => Media::images()->count(),
            'optimized_count' => Media::images()->where('is_optimized', true)->count(),
            'unoptimized_count' => Media::needsOptimization()->count(),
            'total_size' => Media::images()->sum('size'),
            'total_savings' => MediaVariant::sum('bytes_saved'),
            'largest_files' => Media::images()->orderBy('size', 'desc')->limit(10)->get(['id', 'filename', 'size', 'is_optimized']),
            'unused_files' => Media::images()->where('usage_count', 0)->where('created_at', '<', now()->subDays(30))->count(),
            'potential_savings' => Media::needsOptimization()->sum('size') * 0.4,
        ];

        $recommendations = [];

        if ($stats['unoptimized_count'] > 0) {
            $recommendations[] = [
                'type' => 'optimize',
                'priority' => 'high',
                'message' => "{$stats['unoptimized_count']} images need optimization. Potential savings: " . $this->formatBytes((int)$stats['potential_savings']),
                'action' => 'Run bulk optimization',
            ];
        }

        if ($stats['unused_files'] > 0) {
            $recommendations[] = [
                'type' => 'cleanup',
                'priority' => 'medium',
                'message' => "{$stats['unused_files']} unused images from 30+ days ago could be cleaned up.",
                'action' => 'Review and delete unused files',
            ];
        }

        return [
            'statistics' => $stats,
            'recommendations' => $recommendations,
            'optimization_rate' => $stats['total_images'] > 0
                ? round(($stats['optimized_count'] / $stats['total_images']) * 100, 1)
                : 0,
        ];
    }

    /**
     * Format bytes helper
     */
    protected function formatBytes(int $bytes): string
    {
        if ($bytes < 1024) return $bytes . ' B';
        if ($bytes < 1024 * 1024) return round($bytes / 1024, 1) . ' KB';
        if ($bytes < 1024 * 1024 * 1024) return round($bytes / 1024 / 1024, 2) . ' MB';
        return round($bytes / 1024 / 1024 / 1024, 2) . ' GB';
    }
}
