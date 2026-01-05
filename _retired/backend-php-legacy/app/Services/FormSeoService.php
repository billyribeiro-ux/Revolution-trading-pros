<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use Illuminate\Support\Str;

/**
 * Form SEO Service - SEO optimization for form landing pages
 *
 * Features:
 * - SEO score calculation (0-100)
 * - Meta title/description analysis
 * - Schema.org form markup
 * - Open Graph support
 * - Twitter Card support
 * - Keyword optimization
 * - Readability analysis
 * - SEO recommendations
 *
 * @version 1.0.0
 */
class FormSeoService
{
    /**
     * SEO scoring weights
     */
    private const WEIGHTS = [
        'title_length' => 15,
        'title_keywords' => 10,
        'description_length' => 15,
        'description_keywords' => 10,
        'slug_quality' => 10,
        'has_og_image' => 10,
        'has_schema' => 10,
        'mobile_friendly' => 10,
        'field_labels' => 5,
        'form_length' => 5,
    ];

    /**
     * Ideal ranges
     */
    private const IDEAL_TITLE_LENGTH = [30, 60];
    private const IDEAL_DESC_LENGTH = [120, 160];
    private const IDEAL_FIELD_COUNT = [3, 10];

    /**
     * Analyze form SEO and return score with recommendations
     *
     * @param Form $form
     * @param array $options Additional options (keywords, etc.)
     * @return array SEO analysis result
     */
    public function analyze(Form $form, array $options = []): array
    {
        $keywords = $options['keywords'] ?? [];
        $scores = [];
        $recommendations = [];

        // Analyze title
        $titleAnalysis = $this->analyzeTitle($form->title, $keywords);
        $scores['title_length'] = $titleAnalysis['length_score'];
        $scores['title_keywords'] = $titleAnalysis['keyword_score'];
        $recommendations = array_merge($recommendations, $titleAnalysis['recommendations']);

        // Analyze description
        $descAnalysis = $this->analyzeDescription($form->description ?? '', $keywords);
        $scores['description_length'] = $descAnalysis['length_score'];
        $scores['description_keywords'] = $descAnalysis['keyword_score'];
        $recommendations = array_merge($recommendations, $descAnalysis['recommendations']);

        // Analyze slug
        $slugAnalysis = $this->analyzeSlug($form->slug, $keywords);
        $scores['slug_quality'] = $slugAnalysis['score'];
        $recommendations = array_merge($recommendations, $slugAnalysis['recommendations']);

        // Check OG image
        $hasOgImage = !empty($form->settings['og_image'] ?? $form->styles['og_image'] ?? null);
        $scores['has_og_image'] = $hasOgImage ? 100 : 0;
        if (!$hasOgImage) {
            $recommendations[] = [
                'type' => 'warning',
                'message' => 'Add an Open Graph image for better social sharing',
                'priority' => 'medium',
            ];
        }

        // Check schema markup
        $hasSchema = !empty($form->settings['schema_type'] ?? null);
        $scores['has_schema'] = $hasSchema ? 100 : 0;
        if (!$hasSchema) {
            $recommendations[] = [
                'type' => 'info',
                'message' => 'Add Schema.org markup for rich search results',
                'priority' => 'low',
            ];
        }

        // Mobile friendliness (based on styles)
        $mobileScore = $this->analyzeMobileFriendliness($form);
        $scores['mobile_friendly'] = $mobileScore['score'];
        $recommendations = array_merge($recommendations, $mobileScore['recommendations']);

        // Analyze field labels
        $labelAnalysis = $this->analyzeFieldLabels($form);
        $scores['field_labels'] = $labelAnalysis['score'];
        $recommendations = array_merge($recommendations, $labelAnalysis['recommendations']);

        // Analyze form length
        $lengthAnalysis = $this->analyzeFormLength($form);
        $scores['form_length'] = $lengthAnalysis['score'];
        $recommendations = array_merge($recommendations, $lengthAnalysis['recommendations']);

        // Calculate total score
        $totalScore = 0;
        foreach ($scores as $key => $score) {
            $weight = self::WEIGHTS[$key] ?? 0;
            $totalScore += ($score / 100) * $weight;
        }

        // Sort recommendations by priority
        usort($recommendations, function ($a, $b) {
            $priorityOrder = ['high' => 0, 'medium' => 1, 'low' => 2];
            return ($priorityOrder[$a['priority']] ?? 3) - ($priorityOrder[$b['priority']] ?? 3);
        });

        return [
            'score' => round($totalScore),
            'grade' => $this->getGrade($totalScore),
            'scores' => $scores,
            'recommendations' => $recommendations,
            'meta' => $this->generateMetaTags($form, $options),
            'schema' => $this->generateSchema($form),
            'og' => $this->generateOpenGraph($form, $options),
            'twitter' => $this->generateTwitterCard($form, $options),
        ];
    }

    /**
     * Analyze title for SEO
     */
    private function analyzeTitle(string $title, array $keywords): array
    {
        $length = strlen($title);
        $recommendations = [];

        // Length score
        if ($length >= self::IDEAL_TITLE_LENGTH[0] && $length <= self::IDEAL_TITLE_LENGTH[1]) {
            $lengthScore = 100;
        } elseif ($length < self::IDEAL_TITLE_LENGTH[0]) {
            $lengthScore = ($length / self::IDEAL_TITLE_LENGTH[0]) * 100;
            $recommendations[] = [
                'type' => 'warning',
                'message' => "Title is too short ({$length} chars). Aim for 30-60 characters.",
                'priority' => 'high',
            ];
        } else {
            $lengthScore = max(0, 100 - (($length - self::IDEAL_TITLE_LENGTH[1]) * 2));
            $recommendations[] = [
                'type' => 'warning',
                'message' => "Title is too long ({$length} chars). Keep it under 60 characters.",
                'priority' => 'high',
            ];
        }

        // Keyword score
        $keywordScore = 0;
        if (!empty($keywords)) {
            $titleLower = strtolower($title);
            $foundKeywords = 0;
            foreach ($keywords as $keyword) {
                if (str_contains($titleLower, strtolower($keyword))) {
                    $foundKeywords++;
                }
            }
            $keywordScore = min(100, ($foundKeywords / count($keywords)) * 100);

            if ($keywordScore < 50) {
                $recommendations[] = [
                    'type' => 'info',
                    'message' => 'Consider adding target keywords to the title',
                    'priority' => 'medium',
                ];
            }
        } else {
            $keywordScore = 50; // Neutral if no keywords provided
        }

        return [
            'length_score' => $lengthScore,
            'keyword_score' => $keywordScore,
            'recommendations' => $recommendations,
        ];
    }

    /**
     * Analyze description for SEO
     */
    private function analyzeDescription(?string $description, array $keywords): array
    {
        $recommendations = [];

        if (empty($description)) {
            return [
                'length_score' => 0,
                'keyword_score' => 0,
                'recommendations' => [[
                    'type' => 'error',
                    'message' => 'Add a meta description for better search visibility',
                    'priority' => 'high',
                ]],
            ];
        }

        $length = strlen($description);

        // Length score
        if ($length >= self::IDEAL_DESC_LENGTH[0] && $length <= self::IDEAL_DESC_LENGTH[1]) {
            $lengthScore = 100;
        } elseif ($length < self::IDEAL_DESC_LENGTH[0]) {
            $lengthScore = ($length / self::IDEAL_DESC_LENGTH[0]) * 100;
            $recommendations[] = [
                'type' => 'warning',
                'message' => "Description is too short ({$length} chars). Aim for 120-160 characters.",
                'priority' => 'medium',
            ];
        } else {
            $lengthScore = max(0, 100 - (($length - self::IDEAL_DESC_LENGTH[1]) * 1));
            $recommendations[] = [
                'type' => 'info',
                'message' => "Description may be truncated in search results ({$length} chars).",
                'priority' => 'low',
            ];
        }

        // Keyword score
        $keywordScore = 50;
        if (!empty($keywords)) {
            $descLower = strtolower($description);
            $foundKeywords = 0;
            foreach ($keywords as $keyword) {
                if (str_contains($descLower, strtolower($keyword))) {
                    $foundKeywords++;
                }
            }
            $keywordScore = min(100, ($foundKeywords / count($keywords)) * 100);
        }

        return [
            'length_score' => $lengthScore,
            'keyword_score' => $keywordScore,
            'recommendations' => $recommendations,
        ];
    }

    /**
     * Analyze URL slug
     */
    private function analyzeSlug(string $slug, array $keywords): array
    {
        $score = 100;
        $recommendations = [];

        // Check length
        if (strlen($slug) > 50) {
            $score -= 20;
            $recommendations[] = [
                'type' => 'info',
                'message' => 'Consider a shorter URL slug for better SEO',
                'priority' => 'low',
            ];
        }

        // Check for special characters
        if (preg_match('/[^a-z0-9\-]/', $slug)) {
            $score -= 30;
            $recommendations[] = [
                'type' => 'warning',
                'message' => 'URL slug should only contain lowercase letters, numbers, and hyphens',
                'priority' => 'medium',
            ];
        }

        // Check for keywords in slug
        if (!empty($keywords)) {
            $foundInSlug = false;
            foreach ($keywords as $keyword) {
                if (str_contains($slug, Str::slug($keyword))) {
                    $foundInSlug = true;
                    break;
                }
            }
            if (!$foundInSlug) {
                $score -= 10;
            }
        }

        return [
            'score' => max(0, $score),
            'recommendations' => $recommendations,
        ];
    }

    /**
     * Analyze mobile friendliness
     */
    private function analyzeMobileFriendliness(Form $form): array
    {
        $score = 100;
        $recommendations = [];

        $styles = $form->styles ?? [];

        // Check if responsive
        if (isset($styles['max_width']) && strpos($styles['max_width'], 'px') !== false) {
            $maxWidth = (int) $styles['max_width'];
            if ($maxWidth > 800) {
                $score -= 20;
                $recommendations[] = [
                    'type' => 'info',
                    'message' => 'Consider a narrower max-width for better mobile experience',
                    'priority' => 'low',
                ];
            }
        }

        // Check field widths
        $hasFixedWidths = false;
        foreach ($form->fields as $field) {
            if (isset($field->width) && $field->width < 50 && $field->width !== 100) {
                $hasFixedWidths = true;
            }
        }

        if ($hasFixedWidths) {
            $recommendations[] = [
                'type' => 'info',
                'message' => 'Ensure multi-column fields stack properly on mobile',
                'priority' => 'low',
            ];
        }

        return [
            'score' => $score,
            'recommendations' => $recommendations,
        ];
    }

    /**
     * Analyze field labels for accessibility/SEO
     */
    private function analyzeFieldLabels(Form $form): array
    {
        $score = 100;
        $recommendations = [];
        $unlabeledCount = 0;
        $shortLabels = 0;

        foreach ($form->fields as $field) {
            if (empty($field->label)) {
                $unlabeledCount++;
            } elseif (strlen($field->label) < 3) {
                $shortLabels++;
            }
        }

        if ($unlabeledCount > 0) {
            $score -= min(50, $unlabeledCount * 10);
            $recommendations[] = [
                'type' => 'error',
                'message' => "{$unlabeledCount} field(s) are missing labels - this hurts accessibility and SEO",
                'priority' => 'high',
            ];
        }

        if ($shortLabels > 0) {
            $score -= min(20, $shortLabels * 5);
            $recommendations[] = [
                'type' => 'info',
                'message' => 'Some field labels are very short - consider more descriptive labels',
                'priority' => 'low',
            ];
        }

        return [
            'score' => max(0, $score),
            'recommendations' => $recommendations,
        ];
    }

    /**
     * Analyze form length (number of fields)
     */
    private function analyzeFormLength(Form $form): array
    {
        $fieldCount = $form->fields->count();
        $recommendations = [];

        if ($fieldCount >= self::IDEAL_FIELD_COUNT[0] && $fieldCount <= self::IDEAL_FIELD_COUNT[1]) {
            $score = 100;
        } elseif ($fieldCount < self::IDEAL_FIELD_COUNT[0]) {
            $score = 70;
            $recommendations[] = [
                'type' => 'info',
                'message' => 'Very short form - consider if you need more fields to gather useful data',
                'priority' => 'low',
            ];
        } else {
            // Penalize long forms
            $excess = $fieldCount - self::IDEAL_FIELD_COUNT[1];
            $score = max(30, 100 - ($excess * 5));
            $recommendations[] = [
                'type' => 'warning',
                'message' => "Form has {$fieldCount} fields - longer forms have higher abandonment rates",
                'priority' => 'medium',
            ];
        }

        return [
            'score' => $score,
            'recommendations' => $recommendations,
        ];
    }

    /**
     * Get letter grade from score
     */
    private function getGrade(float $score): string
    {
        if ($score >= 90) return 'A';
        if ($score >= 80) return 'B';
        if ($score >= 70) return 'C';
        if ($score >= 60) return 'D';
        return 'F';
    }

    /**
     * Generate meta tags
     */
    public function generateMetaTags(Form $form, array $options = []): array
    {
        $siteUrl = config('app.url');
        $formUrl = "{$siteUrl}/forms/{$form->slug}";

        return [
            'title' => $form->settings['meta_title'] ?? $form->title,
            'description' => $form->settings['meta_description'] ?? $form->description ?? '',
            'canonical' => $form->settings['canonical_url'] ?? $formUrl,
            'robots' => $form->settings['robots'] ?? 'index, follow',
            'keywords' => implode(', ', $options['keywords'] ?? []),
        ];
    }

    /**
     * Generate Schema.org markup for form
     */
    public function generateSchema(Form $form): array
    {
        $siteUrl = config('app.url');

        return [
            '@context' => 'https://schema.org',
            '@type' => 'WebPage',
            'name' => $form->title,
            'description' => $form->description ?? '',
            'url' => "{$siteUrl}/forms/{$form->slug}",
            'mainEntity' => [
                '@type' => 'ContactPoint',
                'contactType' => 'Form Submission',
                'name' => $form->title,
            ],
            'datePublished' => $form->published_at?->toIso8601String(),
            'dateModified' => $form->updated_at->toIso8601String(),
        ];
    }

    /**
     * Generate Open Graph tags
     */
    public function generateOpenGraph(Form $form, array $options = []): array
    {
        $siteUrl = config('app.url');

        return [
            'og:type' => 'website',
            'og:title' => $form->settings['og_title'] ?? $form->title,
            'og:description' => $form->settings['og_description'] ?? $form->description ?? '',
            'og:url' => "{$siteUrl}/forms/{$form->slug}",
            'og:image' => $form->settings['og_image'] ?? $form->styles['og_image'] ?? '',
            'og:site_name' => config('app.name'),
            'og:locale' => $form->settings['locale'] ?? 'en_US',
        ];
    }

    /**
     * Generate Twitter Card tags
     */
    public function generateTwitterCard(Form $form, array $options = []): array
    {
        return [
            'twitter:card' => $form->settings['twitter_card_type'] ?? 'summary_large_image',
            'twitter:title' => $form->settings['twitter_title'] ?? $form->title,
            'twitter:description' => $form->settings['twitter_description'] ?? $form->description ?? '',
            'twitter:image' => $form->settings['twitter_image'] ?? $form->settings['og_image'] ?? '',
            'twitter:site' => $form->settings['twitter_site'] ?? config('services.twitter.handle', ''),
        ];
    }

    /**
     * Update form with SEO data
     */
    public function updateSeoData(Form $form, array $seoData): Form
    {
        $settings = $form->settings ?? [];

        $settings = array_merge($settings, [
            'meta_title' => $seoData['meta_title'] ?? $settings['meta_title'] ?? null,
            'meta_description' => $seoData['meta_description'] ?? $settings['meta_description'] ?? null,
            'keywords' => $seoData['keywords'] ?? $settings['keywords'] ?? [],
            'og_title' => $seoData['og_title'] ?? $settings['og_title'] ?? null,
            'og_description' => $seoData['og_description'] ?? $settings['og_description'] ?? null,
            'og_image' => $seoData['og_image'] ?? $settings['og_image'] ?? null,
            'twitter_card_type' => $seoData['twitter_card_type'] ?? $settings['twitter_card_type'] ?? 'summary_large_image',
            'canonical_url' => $seoData['canonical_url'] ?? $settings['canonical_url'] ?? null,
            'robots' => $seoData['robots'] ?? $settings['robots'] ?? 'index, follow',
            'schema_type' => $seoData['schema_type'] ?? $settings['schema_type'] ?? 'WebPage',
            'last_seo_check_at' => now()->toIso8601String(),
        ]);

        $form->update(['settings' => $settings]);

        return $form;
    }
}
