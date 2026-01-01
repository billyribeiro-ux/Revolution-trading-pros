<?php

declare(strict_types=1);

namespace Tests\Unit\ContentLake;

use App\Services\ContentLake\AIContentAnalysisService;
use App\Services\ContentLake\PortableTextService;
use PHPUnit\Framework\TestCase;

/**
 * Unit tests for AIContentAnalysisService
 */
class AIContentAnalysisServiceTest extends TestCase
{
    private AIContentAnalysisService $service;

    protected function setUp(): void
    {
        parent::setUp();
        $this->service = new AIContentAnalysisService(new PortableTextService());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // READABILITY TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_analyzes_readability_of_simple_text(): void
    {
        $text = 'The cat sat on the mat. It was a nice day. The sun was shining.';

        $result = $this->service->analyzeReadability($text);

        $this->assertArrayHasKey('score', $result);
        $this->assertArrayHasKey('grade', $result);
        $this->assertArrayHasKey('metrics', $result);
        $this->assertArrayHasKey('stats', $result);
        $this->assertGreaterThan(0, $result['score']);
    }

    public function test_readability_score_varies_with_complexity(): void
    {
        $simpleText = 'The dog ran. It was fast. The cat sat.';
        $complexText = 'The phenomenological implications of epistemological considerations within the framework of contemporary philosophical discourse necessitate a comprehensive understanding of metaphysical paradigms.';

        $simpleResult = $this->service->analyzeReadability($simpleText);
        $complexResult = $this->service->analyzeReadability($complexText);

        $this->assertGreaterThan($complexResult['score'], $simpleResult['score']);
    }

    public function test_readability_counts_sentences_correctly(): void
    {
        $text = 'First sentence. Second sentence! Third sentence?';

        $result = $this->service->analyzeReadability($text);

        $this->assertEquals(3, $result['stats']['sentences']);
    }

    public function test_readability_counts_words_correctly(): void
    {
        $text = 'One two three four five.';

        $result = $this->service->analyzeReadability($text);

        $this->assertEquals(5, $result['stats']['words']);
    }

    public function test_readability_handles_empty_text(): void
    {
        $result = $this->service->analyzeReadability('');

        $this->assertEquals(0, $result['score']);
        $this->assertEquals('N/A', $result['grade']);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SENTIMENT TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_detects_positive_sentiment(): void
    {
        $text = 'This is amazing! I love it. Everything is wonderful and fantastic.';

        $result = $this->service->analyzeSentiment($text);

        $this->assertEquals('positive', $result['overall']);
        $this->assertGreaterThan(0, $result['score']);
    }

    public function test_detects_negative_sentiment(): void
    {
        $text = 'This is terrible. I hate it. Everything is awful and horrible.';

        $result = $this->service->analyzeSentiment($text);

        $this->assertEquals('negative', $result['overall']);
        $this->assertLessThan(0, $result['score']);
    }

    public function test_detects_neutral_sentiment(): void
    {
        $text = 'The document contains information. It has several paragraphs.';

        $result = $this->service->analyzeSentiment($text);

        $this->assertEquals('neutral', $result['overall']);
    }

    public function test_sentiment_includes_breakdown(): void
    {
        $text = 'This is good but also bad.';

        $result = $this->service->analyzeSentiment($text);

        $this->assertArrayHasKey('breakdown', $result);
        $this->assertArrayHasKey('positive', $result['breakdown']);
        $this->assertArrayHasKey('negative', $result['breakdown']);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // KEYWORD EXTRACTION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_extracts_keywords(): void
    {
        $text = 'Machine learning and artificial intelligence are transforming technology. Machine learning enables computers to learn from data. Artificial intelligence powers many modern applications.';

        $result = $this->service->extractKeywords($text, 5);

        $this->assertArrayHasKey('words', $result);
        $this->assertNotEmpty($result['words']);
        $this->assertLessThanOrEqual(5, count($result['words']));
    }

    public function test_keywords_include_frequency(): void
    {
        $text = 'Testing testing one two three testing.';

        $result = $this->service->extractKeywords($text, 3);

        foreach ($result['words'] as $keyword) {
            $this->assertArrayHasKey('word', $keyword);
            $this->assertArrayHasKey('frequency', $keyword);
            $this->assertArrayHasKey('score', $keyword);
        }
    }

    public function test_extracts_key_phrases(): void
    {
        $text = 'Machine learning is important. Machine learning helps businesses. Using machine learning for predictions.';

        $result = $this->service->extractKeywords($text, 5);

        $this->assertArrayHasKey('phrases', $result);
    }

    public function test_filters_stop_words(): void
    {
        $text = 'The and or but in on at to for of the a an';

        $result = $this->service->extractKeywords($text, 10);

        $keywords = array_column($result['words'], 'word');
        $this->assertNotContains('the', $keywords);
        $this->assertNotContains('and', $keywords);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ENTITY EXTRACTION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_extracts_emails(): void
    {
        $text = 'Contact us at info@example.com or support@company.org for help.';

        $result = $this->service->extractEntities($text);

        $this->assertArrayHasKey('emails', $result);
        $this->assertContains('info@example.com', $result['emails']);
        $this->assertContains('support@company.org', $result['emails']);
    }

    public function test_extracts_urls(): void
    {
        $text = 'Visit https://example.com or http://test.org for more information.';

        $result = $this->service->extractEntities($text);

        $this->assertArrayHasKey('urls', $result);
        $this->assertContains('https://example.com', $result['urls']);
    }

    public function test_extracts_dates(): void
    {
        $text = 'The meeting is on 12/25/2024 and the deadline is January 1, 2025.';

        $result = $this->service->extractEntities($text);

        $this->assertArrayHasKey('dates', $result);
        $this->assertNotEmpty($result['dates']);
    }

    public function test_extracts_money(): void
    {
        $text = 'The product costs $99.99 or 50 dollars.';

        $result = $this->service->extractEntities($text);

        $this->assertArrayHasKey('money', $result);
        $this->assertNotEmpty($result['money']);
    }

    public function test_extracts_percentages(): void
    {
        $text = 'Sales increased by 25% and revenue grew 10.5%.';

        $result = $this->service->extractEntities($text);

        $this->assertArrayHasKey('percentages', $result);
        $this->assertContains('25%', $result['percentages']);
    }

    public function test_extracts_hashtags(): void
    {
        $text = 'Check out #technology and #innovation for updates.';

        $result = $this->service->extractEntities($text);

        $this->assertArrayHasKey('hashtags', $result);
        $this->assertContains('#technology', $result['hashtags']);
    }

    public function test_extracts_mentions(): void
    {
        $text = 'Thanks to @johndoe and @janedoe for their help.';

        $result = $this->service->extractEntities($text);

        $this->assertArrayHasKey('mentions', $result);
        $this->assertContains('@johndoe', $result['mentions']);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // LANGUAGE DETECTION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_detects_english(): void
    {
        $text = 'The quick brown fox jumps over the lazy dog. This is a sample English text.';

        $result = $this->service->detectLanguage($text);

        $this->assertEquals('en', $result['detected']);
    }

    public function test_language_detection_includes_confidence(): void
    {
        $text = 'Hello world, this is a test.';

        $result = $this->service->detectLanguage($text);

        $this->assertArrayHasKey('confidence', $result);
        $this->assertGreaterThan(0, $result['confidence']);
    }

    public function test_language_detection_includes_alternatives(): void
    {
        $text = 'Some text to analyze.';

        $result = $this->service->detectLanguage($text);

        $this->assertArrayHasKey('alternatives', $result);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SUMMARY GENERATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_generates_summary(): void
    {
        $text = 'Machine learning is a subset of artificial intelligence. It enables computers to learn from data without being explicitly programmed. Machine learning algorithms can identify patterns and make decisions. This technology is used in many applications today.';

        $result = $this->service->generateSummary($text, 100);

        $this->assertNotEmpty($result);
        $this->assertLessThanOrEqual(150, strlen($result)); // Allow some flexibility
    }

    public function test_summary_respects_max_length(): void
    {
        $text = str_repeat('This is a long sentence that repeats many times. ', 20);

        $result = $this->service->generateSummary($text, 50);

        $this->assertLessThanOrEqual(100, strlen($result)); // Allow buffer for word boundaries
    }

    public function test_summary_handles_short_text(): void
    {
        $text = 'Short text.';

        $result = $this->service->generateSummary($text, 150);

        $this->assertNotEmpty($result);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // TAG SUGGESTION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_suggests_tags(): void
    {
        $text = 'Artificial intelligence and machine learning are revolutionizing the technology industry.';
        $keywords = $this->service->extractKeywords($text, 10);

        $result = $this->service->suggestTags($text, $keywords);

        $this->assertIsArray($result);
    }

    public function test_tags_include_confidence(): void
    {
        $text = 'Technology innovation digital transformation.';
        $keywords = $this->service->extractKeywords($text, 5);

        $result = $this->service->suggestTags($text, $keywords);

        if (!empty($result)) {
            $this->assertArrayHasKey('tag', $result[0]);
            $this->assertArrayHasKey('confidence', $result[0]);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CATEGORY SUGGESTION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_suggests_technology_category(): void
    {
        $text = 'Software development and technology trends. Building applications with AI and machine learning.';
        $entities = $this->service->extractEntities($text);

        $result = $this->service->suggestCategories($text, $entities);

        $categories = array_column($result, 'category');
        $this->assertContains('Technology', $categories);
    }

    public function test_suggests_business_category(): void
    {
        $text = 'Company revenue and profit margins. Market analysis and investment opportunities. Business growth strategies.';
        $entities = $this->service->extractEntities($text);

        $result = $this->service->suggestCategories($text, $entities);

        $categories = array_column($result, 'category');
        $this->assertContains('Business', $categories);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MODERATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_safe_content_passes_moderation(): void
    {
        $text = 'This is a perfectly normal and appropriate piece of content about technology.';

        $result = $this->service->moderateContent($text);

        $this->assertTrue($result['safe']);
        $this->assertEquals('none', $result['severity']);
    }

    public function test_moderation_includes_recommendation(): void
    {
        $text = 'Normal content here.';

        $result = $this->service->moderateContent($text);

        $this->assertArrayHasKey('recommendation', $result);
    }
}
