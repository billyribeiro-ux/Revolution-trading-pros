<?php

namespace App\Services\Seo;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;

/**
 * Schema Intelligence Service
 * 
 * Google L8 Enterprise-Grade Schema Generation
 * Implements: Auto-detection, JSON-LD Generation, Entity Enrichment, Validation
 * 
 * @author RevolutionSEO-L8-System
 * @version 1.0.0
 */
class SchemaIntelligenceService
{
    private const CACHE_PREFIX = 'seo:schema';

    /**
     * Generate schema for content.
     */
    public function generateSchema(
        string $contentType,
        int $contentId,
        array $content,
        array $entities = []
    ): array {
        $cacheKey = self::CACHE_PREFIX . ":instance:{$contentType}:{$contentId}";
        
        return Cache::remember($cacheKey, 86400, function () use ($contentType, $contentId, $content, $entities) {
            // Detect applicable schema types
            $schemaTypes = $this->detectSchemaTypes($contentType, $content);
            
            $schemas = [];
            
            foreach ($schemaTypes as $type) {
                $schema = $this->buildSchema($type, $content, $entities);
                
                if ($schema) {
                    // Validate schema
                    $validation = $this->validateSchema($schema);
                    
                    // Store schema instance
                    DB::table('seo_schema_instances')->updateOrInsert(
                        [
                            'content_type' => $contentType,
                            'content_id' => $contentId,
                            'schema_type' => $type,
                        ],
                        [
                            'schema_json' => json_encode($schema),
                            'validation_status' => $validation['status'],
                            'validation_errors' => json_encode($validation['errors'] ?? []),
                            'rich_results_eligible' => $validation['rich_results_eligible'],
                            'generated_at' => now(),
                            'updated_at' => now(),
                        ]
                    );
                    
                    $schemas[] = $schema;
                }
            }
            
            return $schemas;
        });
    }

    /**
     * Detect applicable schema types for content.
     */
    private function detectSchemaTypes(string $contentType, array $content): array
    {
        $types = [];
        
        // Always add Organization
        $types[] = 'Organization';
        
        // Content-specific schemas
        if ($contentType === 'post') {
            $types[] = 'BlogPosting';
            
            // Check for FAQ content
            if ($this->hasFaqContent($content['content'] ?? '')) {
                $types[] = 'FAQPage';
            }
            
            // Check for How-To content
            if ($this->hasHowToContent($content['content'] ?? '')) {
                $types[] = 'HowTo';
            }
        }
        
        if ($contentType === 'product') {
            $types[] = 'Product';
        }
        
        // Always add BreadcrumbList
        $types[] = 'BreadcrumbList';
        
        return $types;
    }

    /**
     * Build schema for specific type.
     */
    private function buildSchema(string $type, array $content, array $entities): ?array
    {
        switch ($type) {
            case 'BlogPosting':
                return $this->buildBlogPostingSchema($content, $entities);
            
            case 'Article':
                return $this->buildArticleSchema($content, $entities);
            
            case 'FAQPage':
                return $this->buildFaqSchema($content);
            
            case 'HowTo':
                return $this->buildHowToSchema($content);
            
            case 'Organization':
                return $this->buildOrganizationSchema();
            
            case 'BreadcrumbList':
                return $this->buildBreadcrumbSchema($content);
            
            case 'Product':
                return $this->buildProductSchema($content);
            
            default:
                return null;
        }
    }

    /**
     * Build BlogPosting schema.
     */
    private function buildBlogPostingSchema(array $content, array $entities): array
    {
        $schema = [
            '@context' => 'https://schema.org',
            '@type' => 'BlogPosting',
            'headline' => $content['title'] ?? '',
            'description' => $content['meta_description'] ?? $content['excerpt'] ?? '',
            'datePublished' => $content['published_at'] ?? $content['created_at'] ?? now()->toIso8601String(),
            'dateModified' => $content['updated_at'] ?? now()->toIso8601String(),
            'author' => [
                '@type' => 'Person',
                'name' => $content['author_name'] ?? 'Admin',
            ],
            'publisher' => [
                '@type' => 'Organization',
                'name' => config('app.name'),
                'logo' => [
                    '@type' => 'ImageObject',
                    'url' => config('app.url') . '/logo.png',
                ],
            ],
        ];
        
        // Add image if available
        if (!empty($content['featured_image'])) {
            $schema['image'] = [
                '@type' => 'ImageObject',
                'url' => $content['featured_image'],
            ];
        }
        
        // Add entities as mentions
        if (!empty($entities)) {
            $schema['mentions'] = [];
            foreach ($entities as $entity) {
                $schema['mentions'][] = [
                    '@type' => $entity['type'] ?? 'Thing',
                    'name' => $entity['name'],
                ];
            }
        }
        
        return $schema;
    }

    /**
     * Build Article schema.
     */
    private function buildArticleSchema(array $content, array $entities): array
    {
        return $this->buildBlogPostingSchema($content, $entities);
    }

    /**
     * Build FAQPage schema.
     */
    private function buildFaqSchema(array $content): array
    {
        $faqs = $this->extractFaqs($content['content'] ?? '');
        
        $schema = [
            '@context' => 'https://schema.org',
            '@type' => 'FAQPage',
            'mainEntity' => [],
        ];
        
        foreach ($faqs as $faq) {
            $schema['mainEntity'][] = [
                '@type' => 'Question',
                'name' => $faq['question'],
                'acceptedAnswer' => [
                    '@type' => 'Answer',
                    'text' => $faq['answer'],
                ],
            ];
        }
        
        return $schema;
    }

    /**
     * Build HowTo schema.
     */
    private function buildHowToSchema(array $content): array
    {
        $steps = $this->extractHowToSteps($content['content'] ?? '');
        
        $schema = [
            '@context' => 'https://schema.org',
            '@type' => 'HowTo',
            'name' => $content['title'] ?? '',
            'description' => $content['meta_description'] ?? '',
            'step' => [],
        ];
        
        foreach ($steps as $index => $step) {
            $schema['step'][] = [
                '@type' => 'HowToStep',
                'position' => $index + 1,
                'name' => $step['name'],
                'text' => $step['text'],
            ];
        }
        
        return $schema;
    }

    /**
     * Build Organization schema.
     */
    private function buildOrganizationSchema(): array
    {
        return [
            '@context' => 'https://schema.org',
            '@type' => 'Organization',
            'name' => config('app.name'),
            'url' => config('app.url'),
            'logo' => config('app.url') . '/logo.png',
            'sameAs' => [
                // Add social media URLs
            ],
        ];
    }

    /**
     * Build BreadcrumbList schema.
     */
    private function buildBreadcrumbSchema(array $content): array
    {
        return [
            '@context' => 'https://schema.org',
            '@type' => 'BreadcrumbList',
            'itemListElement' => [
                [
                    '@type' => 'ListItem',
                    'position' => 1,
                    'name' => 'Home',
                    'item' => config('app.url'),
                ],
                [
                    '@type' => 'ListItem',
                    'position' => 2,
                    'name' => $content['title'] ?? '',
                    'item' => config('app.url') . '/' . ($content['slug'] ?? ''),
                ],
            ],
        ];
    }

    /**
     * Build Product schema.
     */
    private function buildProductSchema(array $content): array
    {
        return [
            '@context' => 'https://schema.org',
            '@type' => 'Product',
            'name' => $content['name'] ?? '',
            'description' => $content['description'] ?? '',
            'image' => $content['image'] ?? '',
            'offers' => [
                '@type' => 'Offer',
                'price' => $content['price'] ?? 0,
                'priceCurrency' => 'USD',
                'availability' => 'https://schema.org/InStock',
            ],
        ];
    }

    /**
     * Validate schema.
     */
    private function validateSchema(array $schema): array
    {
        $errors = [];
        $richResultsEligible = true;
        
        // Check required fields based on type
        $type = $schema['@type'] ?? '';
        
        switch ($type) {
            case 'BlogPosting':
            case 'Article':
                if (empty($schema['headline'])) {
                    $errors[] = 'Missing required field: headline';
                    $richResultsEligible = false;
                }
                if (empty($schema['author'])) {
                    $errors[] = 'Missing required field: author';
                }
                if (empty($schema['datePublished'])) {
                    $errors[] = 'Missing required field: datePublished';
                }
                break;
            
            case 'FAQPage':
                if (empty($schema['mainEntity'])) {
                    $errors[] = 'FAQPage must have at least one question';
                    $richResultsEligible = false;
                }
                break;
            
            case 'Product':
                if (empty($schema['name'])) {
                    $errors[] = 'Missing required field: name';
                    $richResultsEligible = false;
                }
                if (empty($schema['offers'])) {
                    $errors[] = 'Missing required field: offers';
                }
                break;
        }
        
        return [
            'status' => empty($errors) ? 'valid' : 'invalid',
            'errors' => $errors,
            'rich_results_eligible' => $richResultsEligible,
        ];
    }

    /**
     * Check if content has FAQ structure.
     */
    private function hasFaqContent(string $content): bool
    {
        // Simple check for question patterns
        $questionPatterns = [
            '/\bwhat is\b/i',
            '/\bhow to\b/i',
            '/\bwhy\b/i',
            '/\?\s*$/m',
        ];
        
        $questionCount = 0;
        foreach ($questionPatterns as $pattern) {
            $questionCount += preg_match_all($pattern, $content);
        }
        
        return $questionCount >= 3;
    }

    /**
     * Check if content has How-To structure.
     */
    private function hasHowToContent(string $content): bool
    {
        $stepPatterns = [
            '/step \d+/i',
            '/\d+\.\s+/m',
            '/first,|second,|third,|finally/i',
        ];
        
        foreach ($stepPatterns as $pattern) {
            if (preg_match($pattern, $content)) {
                return true;
            }
        }
        
        return false;
    }

    /**
     * Extract FAQs from content.
     */
    private function extractFaqs(string $content): array
    {
        $faqs = [];
        
        // Simple extraction (in production, use more sophisticated parsing)
        preg_match_all('/<h[23]>(.*?\?)<\/h[23]>\s*<p>(.*?)<\/p>/is', $content, $matches, PREG_SET_ORDER);
        
        foreach ($matches as $match) {
            $faqs[] = [
                'question' => strip_tags($match[1]),
                'answer' => strip_tags($match[2]),
            ];
        }
        
        return $faqs;
    }

    /**
     * Extract How-To steps from content.
     */
    private function extractHowToSteps(string $content): array
    {
        $steps = [];
        
        // Extract numbered steps
        preg_match_all('/<h[23]>(?:step )?\d+[:.]\s*(.*?)<\/h[23]>\s*<p>(.*?)<\/p>/is', $content, $matches, PREG_SET_ORDER);
        
        foreach ($matches as $match) {
            $steps[] = [
                'name' => strip_tags($match[1]),
                'text' => strip_tags($match[2]),
            ];
        }
        
        return $steps;
    }
}
