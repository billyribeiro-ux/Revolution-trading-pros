<?php

namespace App\Services\Seo;

use App\Models\SeoAnalysis;
use App\Enums\SeoAnalysisStatus;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Str;
use DOMDocument;
use DOMXPath;

class SeoAnalyzerService
{
    private array $config;
    private array $weights = [
        'title' => 0.15,
        'meta_description' => 0.10,
        'headings' => 0.10,
        'content_quality' => 0.20,
        'keyword_usage' => 0.15,
        'internal_links' => 0.05,
        'external_links' => 0.05,
        'images' => 0.10,
        'readability' => 0.10,
    ];

    public function __construct()
    {
        $this->config = config('seo.analyzer', []);
    }

    /**
     * Perform comprehensive SEO analysis.
     */
    public function analyze(
        string $contentType,
        int $contentId,
        ?string $focusKeyword,
        array $secondaryKeywords = [],
        string $depth = 'standard'
    ): SeoAnalysis {
        $content = $this->fetchContent($contentType, $contentId);
        
        if (!$content) {
            throw new \RuntimeException("Content not found: {$contentType}:{$contentId}");
        }

        $analysis = SeoAnalysis::updateOrCreate(
            [
                'analyzable_type' => $contentType,
                'analyzable_id' => $contentId,
            ],
            [
                'focus_keyword' => $focusKeyword,
                'secondary_keywords' => $secondaryKeywords,
                'status' => SeoAnalysisStatus::IN_PROGRESS,
                'depth' => $depth,
            ]
        );

        try {
            // Parse HTML content
            $dom = $this->parseHtml($content['html']);
            $textContent = $this->extractTextContent($dom);
            
            // Perform various analyses
            $scores = [];
            $issues = [];
            $suggestions = [];

            // Title analysis
            $titleAnalysis = $this->analyzeTitle($dom, $focusKeyword);
            $scores['title'] = $titleAnalysis['score'];
            $issues = array_merge($issues, $titleAnalysis['issues']);
            $suggestions = array_merge($suggestions, $titleAnalysis['suggestions']);

            // Meta description analysis
            $metaAnalysis = $this->analyzeMetaDescription($dom, $focusKeyword);
            $scores['meta_description'] = $metaAnalysis['score'];
            $issues = array_merge($issues, $metaAnalysis['issues']);
            $suggestions = array_merge($suggestions, $metaAnalysis['suggestions']);

            // Headings analysis
            $headingsAnalysis = $this->analyzeHeadings($dom, $focusKeyword);
            $scores['headings'] = $headingsAnalysis['score'];
            $issues = array_merge($issues, $headingsAnalysis['issues']);
            $suggestions = array_merge($suggestions, $headingsAnalysis['suggestions']);

            // Content quality analysis
            $contentAnalysis = $this->analyzeContentQuality($textContent, $focusKeyword, $secondaryKeywords);
            $scores['content_quality'] = $contentAnalysis['score'];
            $issues = array_merge($issues, $contentAnalysis['issues']);
            $suggestions = array_merge($suggestions, $contentAnalysis['suggestions']);

            // Keyword analysis
            $keywordAnalysis = $this->analyzeKeywordUsage($textContent, $focusKeyword, $secondaryKeywords);
            $scores['keyword_usage'] = $keywordAnalysis['score'];
            $analysis->keyword_density = $keywordAnalysis['density'];
            $issues = array_merge($issues, $keywordAnalysis['issues']);
            $suggestions = array_merge($suggestions, $keywordAnalysis['suggestions']);

            // Links analysis
            $linksAnalysis = $this->analyzeLinks($dom);
            $scores['internal_links'] = $linksAnalysis['internal_score'];
            $scores['external_links'] = $linksAnalysis['external_score'];
            $issues = array_merge($issues, $linksAnalysis['issues']);
            $suggestions = array_merge($suggestions, $linksAnalysis['suggestions']);

            // Images analysis
            $imagesAnalysis = $this->analyzeImages($dom, $focusKeyword);
            $scores['images'] = $imagesAnalysis['score'];
            $issues = array_merge($issues, $imagesAnalysis['issues']);
            $suggestions = array_merge($suggestions, $imagesAnalysis['suggestions']);

            // Readability analysis
            $readabilityAnalysis = $this->analyzeReadability($textContent);
            $scores['readability'] = $readabilityAnalysis['score'];
            $analysis->readability_score = $readabilityAnalysis['flesch_score'];
            $issues = array_merge($issues, $readabilityAnalysis['issues']);
            $suggestions = array_merge($suggestions, $readabilityAnalysis['suggestions']);

            // Advanced analysis for comprehensive depth
            if ($depth === 'comprehensive') {
                $advancedAnalysis = $this->performAdvancedAnalysis($dom, $textContent, $content);
                $issues = array_merge($issues, $advancedAnalysis['issues']);
                $suggestions = array_merge($suggestions, $advancedAnalysis['suggestions']);
            }

            // Calculate overall score
            $overallScore = $this->calculateWeightedScore($scores);

            // Update analysis with results
            $analysis->update([
                'seo_score' => round($overallScore),
                'has_meta_title' => !empty($titleAnalysis['title']),
                'has_meta_description' => !empty($metaAnalysis['description']),
                'analysis_results' => $issues,
                'suggestions' => $this->prioritizeSuggestions($suggestions),
                'scores_breakdown' => $scores,
                'status' => SeoAnalysisStatus::COMPLETED,
                'completed_at' => now(),
                'metadata' => [
                    'word_count' => str_word_count($textContent),
                    'reading_time' => $this->calculateReadingTime($textContent),
                    'links_count' => $linksAnalysis['total_links'],
                    'images_count' => $imagesAnalysis['total_images'],
                    'headings_structure' => $headingsAnalysis['structure'],
                ],
            ]);

            return $analysis;

        } catch (\Exception $e) {
            $analysis->update([
                'status' => SeoAnalysisStatus::FAILED,
                'error_message' => $e->getMessage(),
            ]);
            
            throw $e;
        }
    }

    /**
     * Analyze title tag.
     */
    private function analyzeTitle(DOMDocument $dom, ?string $focusKeyword): array
    {
        $xpath = new DOMXPath($dom);
        $titleNodes = $xpath->query('//title');
        $title = $titleNodes->length > 0 ? trim($titleNodes->item(0)->textContent) : '';
        
        $score = 100;
        $issues = [];
        $suggestions = [];

        if (empty($title)) {
            $score = 0;
            $issues[] = [
                'type' => 'missing_title',
                'severity' => 'critical',
                'message' => 'Page is missing a title tag',
            ];
            $suggestions[] = [
                'priority' => 'critical',
                'category' => 'title',
                'title' => 'Add a title tag',
                'description' => 'Every page needs a unique, descriptive title tag',
                'estimated_impact' => 30,
                'effort_hours' => 0.5,
            ];
        } else {
            $titleLength = strlen($title);
            
            // Check length
            if ($titleLength < 30) {
                $score -= 20;
                $issues[] = [
                    'type' => 'title_too_short',
                    'severity' => 'medium',
                    'message' => "Title is too short ({$titleLength} characters)",
                ];
                $suggestions[] = [
                    'priority' => 'medium',
                    'category' => 'title',
                    'title' => 'Expand your title',
                    'description' => 'Aim for 30-60 characters to maximize visibility',
                    'estimated_impact' => 10,
                    'effort_hours' => 0.25,
                ];
            } elseif ($titleLength > 60) {
                $score -= 15;
                $issues[] = [
                    'type' => 'title_too_long',
                    'severity' => 'low',
                    'message' => "Title may be truncated ({$titleLength} characters)",
                ];
            }

            // Check for focus keyword
            if ($focusKeyword && stripos($title, $focusKeyword) === false) {
                $score -= 25;
                $issues[] = [
                    'type' => 'keyword_missing_title',
                    'severity' => 'high',
                    'message' => 'Focus keyword not found in title',
                ];
                $suggestions[] = [
                    'priority' => 'high',
                    'category' => 'title',
                    'title' => 'Include focus keyword in title',
                    'description' => "Add '{$focusKeyword}' to your title, preferably near the beginning",
                    'estimated_impact' => 20,
                    'effort_hours' => 0.25,
                ];
            } elseif ($focusKeyword && stripos($title, $focusKeyword) > 30) {
                $score -= 10;
                $suggestions[] = [
                    'priority' => 'low',
                    'category' => 'title',
                    'title' => 'Move keyword closer to beginning',
                    'description' => 'Keywords at the start of titles carry more weight',
                    'estimated_impact' => 5,
                    'effort_hours' => 0.25,
                ];
            }

            // Check for clickbait elements
            if ($this->hasClickbaitElements($title)) {
                $score -= 5;
            }
        }

        return [
            'score' => max(0, $score),
            'title' => $title,
            'issues' => $issues,
            'suggestions' => $suggestions,
        ];
    }

    /**
     * Analyze meta description.
     */
    private function analyzeMetaDescription(DOMDocument $dom, ?string $focusKeyword): array
    {
        $xpath = new DOMXPath($dom);
        $metaNodes = $xpath->query('//meta[@name="description"]');
        $description = $metaNodes->length > 0 
            ? trim($metaNodes->item(0)->getAttribute('content')) 
            : '';
        
        $score = 100;
        $issues = [];
        $suggestions = [];

        if (empty($description)) {
            $score = 0;
            $issues[] = [
                'type' => 'missing_meta_description',
                'severity' => 'high',
                'message' => 'Page is missing a meta description',
            ];
            $suggestions[] = [
                'priority' => 'high',
                'category' => 'meta',
                'title' => 'Add a meta description',
                'description' => 'Write a compelling 120-160 character description',
                'estimated_impact' => 20,
                'effort_hours' => 0.5,
            ];
        } else {
            $descLength = strlen($description);
            
            // Check length
            if ($descLength < 120) {
                $score -= 15;
                $issues[] = [
                    'type' => 'meta_description_too_short',
                    'severity' => 'medium',
                    'message' => "Meta description is too short ({$descLength} characters)",
                ];
            } elseif ($descLength > 160) {
                $score -= 10;
                $issues[] = [
                    'type' => 'meta_description_too_long',
                    'severity' => 'low',
                    'message' => "Meta description may be truncated ({$descLength} characters)",
                ];
            }

            // Check for focus keyword
            if ($focusKeyword && stripos($description, $focusKeyword) === false) {
                $score -= 20;
                $issues[] = [
                    'type' => 'keyword_missing_meta',
                    'severity' => 'medium',
                    'message' => 'Focus keyword not found in meta description',
                ];
                $suggestions[] = [
                    'priority' => 'medium',
                    'category' => 'meta',
                    'title' => 'Include focus keyword in meta description',
                    'description' => "Naturally incorporate '{$focusKeyword}' in your description",
                    'estimated_impact' => 10,
                    'effort_hours' => 0.25,
                ];
            }

            // Check for call-to-action
            if (!$this->hasCallToAction($description)) {
                $score -= 5;
                $suggestions[] = [
                    'priority' => 'low',
                    'category' => 'meta',
                    'title' => 'Add a call-to-action',
                    'description' => 'Include action words to improve click-through rate',
                    'estimated_impact' => 5,
                    'effort_hours' => 0.25,
                ];
            }
        }

        return [
            'score' => max(0, $score),
            'description' => $description,
            'issues' => $issues,
            'suggestions' => $suggestions,
        ];
    }

    /**
     * Analyze heading structure.
     */
    private function analyzeHeadings(DOMDocument $dom, ?string $focusKeyword): array
    {
        $xpath = new DOMXPath($dom);
        $score = 100;
        $issues = [];
        $suggestions = [];
        $structure = [];

        // Check for H1
        $h1Nodes = $xpath->query('//h1');
        $h1Count = $h1Nodes->length;
        
        if ($h1Count === 0) {
            $score -= 30;
            $issues[] = [
                'type' => 'missing_h1',
                'severity' => 'critical',
                'message' => 'Page is missing an H1 heading',
            ];
            $suggestions[] = [
                'priority' => 'critical',
                'category' => 'headings',
                'title' => 'Add an H1 heading',
                'description' => 'Every page needs exactly one H1 heading',
                'estimated_impact' => 25,
                'effort_hours' => 0.5,
            ];
        } elseif ($h1Count > 1) {
            $score -= 15;
            $issues[] = [
                'type' => 'multiple_h1',
                'severity' => 'medium',
                'message' => "Page has {$h1Count} H1 headings (should have only 1)",
            ];
        } else {
            $h1Text = $h1Nodes->item(0)->textContent;
            $structure['h1'] = $h1Text;
            
            if ($focusKeyword && stripos($h1Text, $focusKeyword) === false) {
                $score -= 15;
                $issues[] = [
                    'type' => 'keyword_missing_h1',
                    'severity' => 'medium',
                    'message' => 'Focus keyword not found in H1',
                ];
            }
        }

        // Check heading hierarchy
        for ($level = 2; $level <= 6; $level++) {
            $headings = $xpath->query("//h{$level}");
            $structure["h{$level}_count"] = $headings->length;
            
            if ($level === 2 && $headings->length === 0) {
                $score -= 10;
                $suggestions[] = [
                    'priority' => 'medium',
                    'category' => 'headings',
                    'title' => 'Add H2 subheadings',
                    'description' => 'Break up content with descriptive H2 headings',
                    'estimated_impact' => 10,
                    'effort_hours' => 1,
                ];
            }
        }

        // Check for broken hierarchy
        if ($this->hasbrokenHeadingHierarchy($dom)) {
            $score -= 10;
            $issues[] = [
                'type' => 'broken_heading_hierarchy',
                'severity' => 'low',
                'message' => 'Heading hierarchy is broken (skipping levels)',
            ];
        }

        return [
            'score' => max(0, $score),
            'structure' => $structure,
            'issues' => $issues,
            'suggestions' => $suggestions,
        ];
    }

    /**
     * Analyze content quality.
     */
    private function analyzeContentQuality(string $text, ?string $focusKeyword, array $secondaryKeywords): array
    {
        $wordCount = str_word_count($text);
        $score = 100;
        $issues = [];
        $suggestions = [];

        // Check word count
        if ($wordCount < 300) {
            $score -= 30;
            $issues[] = [
                'type' => 'thin_content',
                'severity' => 'high',
                'message' => "Content is too short ({$wordCount} words)",
            ];
            $suggestions[] = [
                'priority' => 'high',
                'category' => 'content',
                'title' => 'Expand your content',
                'description' => 'Aim for at least 600 words for better SEO performance',
                'estimated_impact' => 30,
                'effort_hours' => 2,
            ];
        } elseif ($wordCount < 600) {
            $score -= 15;
            $issues[] = [
                'type' => 'short_content',
                'severity' => 'medium',
                'message' => "Content could be longer ({$wordCount} words)",
            ];
        } elseif ($wordCount > 3000) {
            // Consider breaking up very long content
            $suggestions[] = [
                'priority' => 'low',
                'category' => 'content',
                'title' => 'Consider breaking up long content',
                'description' => 'Very long articles might benefit from being split into multiple pages',
                'estimated_impact' => 5,
                'effort_hours' => 2,
            ];
        }

        // Check for duplicate content patterns
        if ($this->hasRepetitiveContent($text)) {
            $score -= 20;
            $issues[] = [
                'type' => 'repetitive_content',
                'severity' => 'medium',
                'message' => 'Content appears to have repetitive sections',
            ];
        }

        // Check topic coverage
        if ($focusKeyword) {
            $relatedTerms = $this->getRelatedTerms($focusKeyword);
            $coverageScore = $this->calculateTopicCoverage($text, $relatedTerms);
            
            if ($coverageScore < 0.3) {
                $score -= 15;
                $suggestions[] = [
                    'priority' => 'medium',
                    'category' => 'content',
                    'title' => 'Improve topic coverage',
                    'description' => 'Cover more aspects of your main topic',
                    'estimated_impact' => 15,
                    'effort_hours' => 1.5,
                ];
            }
        }

        return [
            'score' => max(0, $score),
            'word_count' => $wordCount,
            'issues' => $issues,
            'suggestions' => $suggestions,
        ];
    }

    /**
     * Analyze keyword usage.
     */
    private function analyzeKeywordUsage(string $text, ?string $focusKeyword, array $secondaryKeywords): array
    {
        $score = 100;
        $issues = [];
        $suggestions = [];
        $density = 0;

        if (!$focusKeyword) {
            $score -= 20;
            $issues[] = [
                'type' => 'no_focus_keyword',
                'severity' => 'medium',
                'message' => 'No focus keyword specified',
            ];
            return [
                'score' => $score,
                'density' => 0,
                'issues' => $issues,
                'suggestions' => $suggestions,
            ];
        }

        $wordCount = str_word_count($text);
        $keywordCount = substr_count(strtolower($text), strtolower($focusKeyword));
        $density = $wordCount > 0 ? ($keywordCount * strlen($focusKeyword)) / strlen($text) * 100 : 0;

        // Check density
        if ($density < 0.5) {
            $score -= 20;
            $issues[] = [
                'type' => 'low_keyword_density',
                'severity' => 'medium',
                'message' => sprintf('Keyword density is too low (%.1f%%)', $density),
            ];
            $suggestions[] = [
                'priority' => 'medium',
                'category' => 'keywords',
                'title' => 'Increase keyword usage',
                'description' => 'Use your focus keyword more naturally throughout the content',
                'estimated_impact' => 15,
                'effort_hours' => 1,
            ];
        } elseif ($density > 3) {
            $score -= 25;
            $issues[] = [
                'type' => 'keyword_stuffing',
                'severity' => 'high',
                'message' => sprintf('Keyword density is too high (%.1f%%)', $density),
            ];
            $suggestions[] = [
                'priority' => 'high',
                'category' => 'keywords',
                'title' => 'Reduce keyword stuffing',
                'description' => 'Use synonyms and variations to avoid over-optimization',
                'estimated_impact' => 20,
                'effort_hours' => 1,
            ];
        }

        // Check keyword placement
        $firstOccurrence = stripos($text, $focusKeyword);
        if ($firstOccurrence > 150) {
            $score -= 10;
            $suggestions[] = [
                'priority' => 'low',
                'category' => 'keywords',
                'title' => 'Use keyword earlier',
                'description' => 'Include your focus keyword in the first paragraph',
                'estimated_impact' => 5,
                'effort_hours' => 0.5,
            ];
        }

        // Check secondary keywords
        if (!empty($secondaryKeywords)) {
            $secondaryUsage = 0;
            foreach ($secondaryKeywords as $keyword) {
                if (stripos($text, $keyword) !== false) {
                    $secondaryUsage++;
                }
            }
            
            if ($secondaryUsage < count($secondaryKeywords) * 0.5) {
                $score -= 10;
                $suggestions[] = [
                    'priority' => 'low',
                    'category' => 'keywords',
                    'title' => 'Use more secondary keywords',
                    'description' => 'Include variations and related terms',
                    'estimated_impact' => 8,
                    'effort_hours' => 1,
                ];
            }
        }

        return [
            'score' => max(0, $score),
            'density' => round($density, 2),
            'issues' => $issues,
            'suggestions' => $suggestions,
        ];
    }

    /**
     * Analyze internal and external links.
     */
    private function analyzeLinks(DOMDocument $dom): array
    {
        $xpath = new DOMXPath($dom);
        $links = $xpath->query('//a[@href]');
        
        $internalLinks = [];
        $externalLinks = [];
        $issues = [];
        $suggestions = [];
        
        $domain = parse_url(config('app.url'), PHP_URL_HOST);
        
        foreach ($links as $link) {
            $href = $link->getAttribute('href');
            $linkDomain = parse_url($href, PHP_URL_HOST);
            
            if (!$linkDomain || $linkDomain === $domain || strpos($href, '/') === 0) {
                $internalLinks[] = $href;
            } else {
                $externalLinks[] = $href;
                
                // Check for nofollow on external links
                $rel = $link->getAttribute('rel');
                if (!str_contains($rel, 'nofollow') && !$this->isTrustedDomain($linkDomain)) {
                    $issues[] = [
                        'type' => 'missing_nofollow',
                        'severity' => 'low',
                        'message' => "External link to {$linkDomain} missing nofollow",
                    ];
                }
            }
            
            // Check for broken anchor text
            $anchorText = trim($link->textContent);
            if (empty($anchorText) && !$link->getElementsByTagName('img')->length) {
                $issues[] = [
                    'type' => 'empty_anchor_text',
                    'severity' => 'medium',
                    'message' => 'Link with empty anchor text found',
                ];
            } elseif (in_array(strtolower($anchorText), ['click here', 'read more', 'here'])) {
                $issues[] = [
                    'type' => 'generic_anchor_text',
                    'severity' => 'low',
                    'message' => "Generic anchor text: '{$anchorText}'",
                ];
            }
        }
        
        // Calculate scores
        $internalScore = 100;
        $externalScore = 100;
        
        if (count($internalLinks) < 2) {
            $internalScore -= 30;
            $suggestions[] = [
                'priority' => 'medium',
                'category' => 'links',
                'title' => 'Add more internal links',
                'description' => 'Link to related content on your site',
                'estimated_impact' => 10,
                'effort_hours' => 0.5,
            ];
        }
        
        if (count($externalLinks) === 0) {
            $externalScore -= 20;
            $suggestions[] = [
                'priority' => 'low',
                'category' => 'links',
                'title' => 'Consider adding external references',
                'description' => 'Link to authoritative external sources',
                'estimated_impact' => 5,
                'effort_hours' => 0.5,
            ];
        } elseif (count($externalLinks) > 10) {
            $externalScore -= 15;
            $issues[] = [
                'type' => 'too_many_external_links',
                'severity' => 'low',
                'message' => 'High number of external links may dilute page authority',
            ];
        }
        
        return [
            'internal_score' => max(0, $internalScore),
            'external_score' => max(0, $externalScore),
            'internal_links' => count($internalLinks),
            'external_links' => count($externalLinks),
            'total_links' => count($internalLinks) + count($externalLinks),
            'issues' => $issues,
            'suggestions' => $suggestions,
        ];
    }

    /**
     * Analyze images.
     */
    private function analyzeImages(DOMDocument $dom, ?string $focusKeyword): array
    {
        $xpath = new DOMXPath($dom);
        $images = $xpath->query('//img');
        
        $score = 100;
        $issues = [];
        $suggestions = [];
        $totalImages = $images->length;
        
        if ($totalImages === 0) {
            $score -= 20;
            $suggestions[] = [
                'priority' => 'medium',
                'category' => 'images',
                'title' => 'Add relevant images',
                'description' => 'Images improve engagement and SEO',
                'estimated_impact' => 15,
                'effort_hours' => 1,
            ];
        } else {
            $missingAlt = 0;
            $largeImages = 0;
            $keywordInAlt = false;
            
            foreach ($images as $img) {
                $alt = $img->getAttribute('alt');
                
                if (empty($alt)) {
                    $missingAlt++;
                }
                
                if ($focusKeyword && stripos($alt, $focusKeyword) !== false) {
                    $keywordInAlt = true;
                }
                
                // Check for lazy loading
                $loading = $img->getAttribute('loading');
                if ($loading !== 'lazy' && $img->getAttribute('data-lazy') === null) {
                    // First image doesn't need lazy loading
                    static $firstImage = true;
                    if (!$firstImage) {
                        $issues[] = [
                            'type' => 'missing_lazy_loading',
                            'severity' => 'low',
                            'message' => 'Image missing lazy loading attribute',
                        ];
                    }
                    $firstImage = false;
                }
            }
            
            if ($missingAlt > 0) {
                $score -= min(30, $missingAlt * 10);
                $issues[] = [
                    'type' => 'missing_alt_text',
                    'severity' => 'high',
                    'message' => "{$missingAlt} images missing alt text",
                ];
                $suggestions[] = [
                    'priority' => 'high',
                    'category' => 'images',
                    'title' => 'Add alt text to images',
                    'description' => 'Alt text improves accessibility and SEO',
                    'estimated_impact' => 20,
                    'effort_hours' => 0.5,
                ];
            }
            
            if ($focusKeyword && !$keywordInAlt && $totalImages > 0) {
                $score -= 10;
                $suggestions[] = [
                    'priority' => 'low',
                    'category' => 'images',
                    'title' => 'Use keyword in image alt text',
                    'description' => 'Include your focus keyword in at least one image alt text',
                    'estimated_impact' => 5,
                    'effort_hours' => 0.25,
                ];
            }
        }
        
        return [
            'score' => max(0, $score),
            'total_images' => $totalImages,
            'issues' => $issues,
            'suggestions' => $suggestions,
        ];
    }

    /**
     * Analyze readability.
     */
    private function analyzeReadability(string $text): array
    {
        $score = 100;
        $issues = [];
        $suggestions = [];
        
        // Calculate Flesch Reading Ease
        $sentences = preg_split('/[.!?]+/', $text, -1, PREG_SPLIT_NO_EMPTY);
        $sentenceCount = count($sentences);
        $wordCount = str_word_count($text);
        $syllableCount = $this->countSyllables($text);
        
        if ($sentenceCount > 0 && $wordCount > 0) {
            $avgWordsPerSentence = $wordCount / $sentenceCount;
            $avgSyllablesPerWord = $syllableCount / $wordCount;
            
            $fleschScore = 206.835 - 1.015 * $avgWordsPerSentence - 84.6 * $avgSyllablesPerWord;
            $fleschScore = max(0, min(100, $fleschScore));
            
            if ($fleschScore < 30) {
                $score -= 25;
                $issues[] = [
                    'type' => 'very_difficult_reading',
                    'severity' => 'high',
                    'message' => 'Content is very difficult to read',
                ];
                $suggestions[] = [
                    'priority' => 'high',
                    'category' => 'readability',
                    'title' => 'Simplify your writing',
                    'description' => 'Use shorter sentences and simpler words',
                    'estimated_impact' => 20,
                    'effort_hours' => 2,
                ];
            } elseif ($fleschScore < 60) {
                $score -= 10;
                $issues[] = [
                    'type' => 'difficult_reading',
                    'severity' => 'medium',
                    'message' => 'Content may be difficult for average readers',
                ];
            }
            
            // Check sentence length
            if ($avgWordsPerSentence > 20) {
                $score -= 15;
                $issues[] = [
                    'type' => 'long_sentences',
                    'severity' => 'medium',
                    'message' => 'Sentences are too long on average',
                ];
                $suggestions[] = [
                    'priority' => 'medium',
                    'category' => 'readability',
                    'title' => 'Shorten your sentences',
                    'description' => 'Aim for 15-20 words per sentence',
                    'estimated_impact' => 10,
                    'effort_hours' => 1,
                ];
            }
            
            // Check paragraph length
            $paragraphs = preg_split('/\n\n+/', $text, -1, PREG_SPLIT_NO_EMPTY);
            $avgWordsPerParagraph = $wordCount / max(1, count($paragraphs));
            
            if ($avgWordsPerParagraph > 150) {
                $score -= 10;
                $issues[] = [
                    'type' => 'long_paragraphs',
                    'severity' => 'low',
                    'message' => 'Paragraphs are too long',
                ];
                $suggestions[] = [
                    'priority' => 'low',
                    'category' => 'readability',
                    'title' => 'Break up paragraphs',
                    'description' => 'Keep paragraphs under 150 words',
                    'estimated_impact' => 5,
                    'effort_hours' => 0.5,
                ];
            }
            
            // Check for passive voice (simplified check)
            $passivePatterns = [
                '/\b(was|were|been|being)\s+\w+ed\b/i',
                '/\b(is|are|was|were)\s+being\s+\w+ed\b/i',
            ];
            
            $passiveCount = 0;
            foreach ($passivePatterns as $pattern) {
                $passiveCount += preg_match_all($pattern, $text);
            }
            
            $passivePercentage = ($passiveCount / max(1, $sentenceCount)) * 100;
            
            if ($passivePercentage > 20) {
                $score -= 10;
                $suggestions[] = [
                    'priority' => 'low',
                    'category' => 'readability',
                    'title' => 'Reduce passive voice',
                    'description' => 'Use more active voice for engaging content',
                    'estimated_impact' => 5,
                    'effort_hours' => 1,
                ];
            }
        } else {
            $fleschScore = 0;
        }
        
        return [
            'score' => max(0, $score),
            'flesch_score' => round($fleschScore),
            'issues' => $issues,
            'suggestions' => $suggestions,
        ];
    }

    /**
     * Perform advanced analysis for comprehensive depth.
     */
    private function performAdvancedAnalysis(DOMDocument $dom, string $text, array $content): array
    {
        $issues = [];
        $suggestions = [];
        
        // Schema markup check
        $xpath = new DOMXPath($dom);
        $schemaScripts = $xpath->query('//script[@type="application/ld+json"]');
        
        if ($schemaScripts->length === 0) {
            $issues[] = [
                'type' => 'missing_schema',
                'severity' => 'medium',
                'message' => 'No structured data (Schema.org) found',
            ];
            $suggestions[] = [
                'priority' => 'medium',
                'category' => 'technical',
                'title' => 'Add structured data',
                'description' => 'Implement Schema.org markup for rich snippets',
                'estimated_impact' => 15,
                'effort_hours' => 2,
            ];
        }
        
        // Open Graph tags
        $ogTags = $xpath->query('//meta[starts-with(@property, "og:")]');
        if ($ogTags->length < 4) {
            $issues[] = [
                'type' => 'incomplete_og_tags',
                'severity' => 'low',
                'message' => 'Incomplete Open Graph tags for social sharing',
            ];
            $suggestions[] = [
                'priority' => 'low',
                'category' => 'social',
                'title' => 'Complete Open Graph tags',
                'description' => 'Add og:title, og:description, og:image, and og:url',
                'estimated_impact' => 5,
                'effort_hours' => 0.5,
            ];
        }
        
        // Canonical URL
        $canonical = $xpath->query('//link[@rel="canonical"]');
        if ($canonical->length === 0) {
            $issues[] = [
                'type' => 'missing_canonical',
                'severity' => 'medium',
                'message' => 'No canonical URL specified',
            ];
        }
        
        // Check for table of contents for long content
        $wordCount = str_word_count($text);
        if ($wordCount > 1500) {
            $tocFound = $xpath->query('//nav[contains(@class, "toc")] | //div[contains(@class, "table-of-contents")]');
            if ($tocFound->length === 0) {
                $suggestions[] = [
                    'priority' => 'low',
                    'category' => 'usability',
                    'title' => 'Add table of contents',
                    'description' => 'Long content benefits from a table of contents',
                    'estimated_impact' => 8,
                    'effort_hours' => 1,
                ];
            }
        }
        
        return [
            'issues' => $issues,
            'suggestions' => $suggestions,
        ];
    }

    /**
     * Calculate weighted overall score.
     */
    private function calculateWeightedScore(array $scores): float
    {
        $totalScore = 0;
        $totalWeight = 0;
        
        foreach ($scores as $key => $score) {
            if (isset($this->weights[$key])) {
                $totalScore += $score * $this->weights[$key];
                $totalWeight += $this->weights[$key];
            }
        }
        
        return $totalWeight > 0 ? $totalScore / $totalWeight : 0;
    }

    /**
     * Prioritize suggestions by impact and effort.
     */
    private function prioritizeSuggestions(array $suggestions): array
    {
        // Calculate ROI (impact / effort) for each suggestion
        foreach ($suggestions as &$suggestion) {
            $suggestion['roi'] = $suggestion['estimated_impact'] / max(0.1, $suggestion['effort_hours']);
        }
        
        // Sort by ROI descending
        usort($suggestions, function ($a, $b) {
            return $b['roi'] <=> $a['roi'];
        });
        
        // Remove ROI from output
        foreach ($suggestions as &$suggestion) {
            unset($suggestion['roi']);
        }
        
        return array_slice($suggestions, 0, 10); // Return top 10
    }

    /**
     * Generate prioritized recommendations from analysis.
     */
    public function generatePrioritizedRecommendations(SeoAnalysis $analysis): array
    {
        $suggestions = $analysis->suggestions ?? [];
        
        // Group by priority
        $grouped = [
            'critical' => [],
            'high' => [],
            'medium' => [],
            'low' => [],
        ];
        
        foreach ($suggestions as $suggestion) {
            $priority = $suggestion['priority'] ?? 'low';
            $grouped[$priority][] = $suggestion;
        }
        
        // Flatten and limit
        $recommendations = [];
        foreach (['critical', 'high', 'medium', 'low'] as $priority) {
            foreach ($grouped[$priority] as $suggestion) {
                $recommendations[] = $suggestion;
                if (count($recommendations) >= 10) {
                    break 2;
                }
            }
        }
        
        return $recommendations;
    }

    // Helper methods
    
    private function fetchContent(string $type, int $id): ?array
    {
        // Implementation depends on your content structure
        // This is a simplified example
        $model = match($type) {
            'post' => \App\Models\Post::class,
            'page' => \App\Models\Page::class,
            'product' => \App\Models\Product::class,
            default => null,
        };
        
        if (!$model) {
            return null;
        }
        
        $content = $model::find($id);
        
        if (!$content) {
            return null;
        }
        
        return [
            'html' => $content->content ?? '',
            'title' => $content->title ?? '',
            'meta_description' => $content->meta_description ?? '',
            'url' => $content->url ?? '',
        ];
    }
    
    private function parseHtml(string $html): DOMDocument
    {
        $dom = new DOMDocument();
        @$dom->loadHTML($html, LIBXML_HTML_NOIMPLIED | LIBXML_HTML_NODEFDTD);
        return $dom;
    }
    
    private function extractTextContent(DOMDocument $dom): string
    {
        $xpath = new DOMXPath($dom);
        $textNodes = $xpath->query('//text()[not(parent::script) and not(parent::style)]');
        
        $text = '';
        foreach ($textNodes as $node) {
            $text .= ' ' . $node->textContent;
        }
        
        return trim(preg_replace('/\s+/', ' ', $text));
    }
    
    private function hasClickbaitElements(string $title): bool
    {
        $clickbaitPatterns = [
            '/you won\'t believe/i',
            '/this one trick/i',
            '/doctors hate/i',
            '/shocking/i',
            '/number \d+ will/i',
        ];
        
        foreach ($clickbaitPatterns as $pattern) {
            if (preg_match($pattern, $title)) {
                return true;
            }
        }
        
        return false;
    }
    
    private function hasCallToAction(string $text): bool
    {
        $ctaWords = ['learn', 'discover', 'find out', 'get', 'try', 'start', 'join', 'download', 'buy', 'shop'];
        
        foreach ($ctaWords as $word) {
            if (stripos($text, $word) !== false) {
                return true;
            }
        }
        
        return false;
    }
    
    private function hasbrokenHeadingHierarchy(DOMDocument $dom): bool
    {
        $xpath = new DOMXPath($dom);
        $previousLevel = 0;
        
        for ($level = 1; $level <= 6; $level++) {
            $headings = $xpath->query("//h{$level}");
            if ($headings->length > 0) {
                if ($previousLevel > 0 && $level - $previousLevel > 1) {
                    return true;
                }
                $previousLevel = $level;
            }
        }
        
        return false;
    }
    
    private function hasRepetitiveContent(string $text): bool
    {
        $sentences = preg_split('/[.!?]+/', $text, -1, PREG_SPLIT_NO_EMPTY);
        $uniqueSentences = array_unique(array_map('trim', $sentences));
        
        return count($uniqueSentences) < count($sentences) * 0.9;
    }
    
    private function getRelatedTerms(string $keyword): array
    {
        // In production, this would use an API or database
        // This is a simplified example
        $relatedTerms = [
            'seo' => ['search engine', 'optimization', 'ranking', 'google', 'keywords'],
            'marketing' => ['promotion', 'advertising', 'campaign', 'strategy', 'brand'],
            // Add more mappings
        ];
        
        $normalizedKeyword = strtolower($keyword);
        
        return $relatedTerms[$normalizedKeyword] ?? [];
    }
    
    private function calculateTopicCoverage(string $text, array $terms): float
    {
        if (empty($terms)) {
            return 1.0;
        }
        
        $foundTerms = 0;
        $lowerText = strtolower($text);
        
        foreach ($terms as $term) {
            if (stripos($lowerText, $term) !== false) {
                $foundTerms++;
            }
        }
        
        return $foundTerms / count($terms);
    }
    
    private function isTrustedDomain(string $domain): bool
    {
        $trustedDomains = [
            'wikipedia.org',
            'google.com',
            'github.com',
            'stackoverflow.com',
            // Add more trusted domains
        ];
        
        foreach ($trustedDomains as $trusted) {
            if (str_ends_with($domain, $trusted)) {
                return true;
            }
        }
        
        return false;
    }
    
    private function countSyllables(string $text): int
    {
        $text = strtolower($text);
        $text = preg_replace('/[^a-z\s]/', '', $text);
        $words = str_word_count($text, 1);
        
        $totalSyllables = 0;
        
        foreach ($words as $word) {
            // Simplified syllable counting
            $syllables = max(1, preg_match_all('/[aeiou]+/', $word));
            $totalSyllables += $syllables;
        }
        
        return $totalSyllables;
    }
    
    private function calculateReadingTime(string $text): int
    {
        $wordsPerMinute = 200;
        $wordCount = str_word_count($text);
        
        return max(1, ceil($wordCount / $wordsPerMinute));
    }
}