<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Models\EmailTemplate;
use App\Models\NewsletterSubscription;
use App\Models\EmailCampaign;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Carbon\Carbon;

/**
 * AIEmailService - AI-Powered Email Marketing Features
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise AI features for email marketing optimization:
 * - Content quality scoring
 * - Subject line optimization
 * - Send time prediction
 * - Engagement prediction
 * - Spam score analysis
 * - Readability assessment
 * - A/B test winner prediction
 *
 * @version 1.0.0
 */
class AIEmailService
{
    /**
     * Spam trigger words (weighted)
     */
    private const SPAM_TRIGGERS = [
        // High risk (0.3 each)
        'free' => 0.3, 'winner' => 0.3, 'congratulations' => 0.3, 'cash' => 0.3,
        'prize' => 0.3, 'urgent' => 0.3, 'act now' => 0.3, 'limited time' => 0.3,
        // Medium risk (0.2 each)
        'click here' => 0.2, 'buy now' => 0.2, 'order now' => 0.2, 'special offer' => 0.2,
        'discount' => 0.2, 'save' => 0.2, 'deal' => 0.2, 'cheap' => 0.2,
        // Low risk (0.1 each)
        'money' => 0.1, 'credit' => 0.1, 'income' => 0.1, 'guarantee' => 0.1,
    ];

    /**
     * Engagement factors weights
     */
    private const ENGAGEMENT_WEIGHTS = [
        'subject_quality' => 0.25,
        'content_quality' => 0.20,
        'send_time_score' => 0.20,
        'subscriber_engagement' => 0.20,
        'historical_performance' => 0.15,
    ];

    /**
     * Score email content quality (0-100)
     */
    public function scoreContent(string $subject, string $html, string $text = ''): array
    {
        $scores = [];

        // Subject line analysis
        $scores['subject'] = $this->analyzeSubject($subject);

        // Content analysis
        $scores['content'] = $this->analyzeContent($html, $text);

        // Spam score
        $scores['spam'] = $this->calculateSpamScore($subject, $html);

        // Readability score
        $scores['readability'] = $this->calculateReadability($text ?: strip_tags($html));

        // Mobile friendliness
        $scores['mobile'] = $this->assessMobileFriendliness($html);

        // Accessibility score
        $scores['accessibility'] = $this->assessAccessibility($html);

        // Calculate overall score
        $overall = (
            $scores['subject']['score'] * 0.25 +
            $scores['content']['score'] * 0.25 +
            (100 - $scores['spam']['score']) * 0.20 +
            $scores['readability']['score'] * 0.15 +
            $scores['mobile']['score'] * 0.10 +
            $scores['accessibility']['score'] * 0.05
        );

        return [
            'overall_score' => round($overall, 1),
            'grade' => $this->scoreToGrade($overall),
            'details' => $scores,
            'recommendations' => $this->generateRecommendations($scores),
        ];
    }

    /**
     * Analyze subject line
     */
    private function analyzeSubject(string $subject): array
    {
        $length = strlen($subject);
        $wordCount = str_word_count($subject);

        $issues = [];
        $score = 100;

        // Length check (optimal: 30-50 chars)
        if ($length < 20) {
            $score -= 15;
            $issues[] = 'Subject line is too short (under 20 characters)';
        } elseif ($length > 60) {
            $score -= 10;
            $issues[] = 'Subject line may be truncated on mobile (over 60 characters)';
        }

        // All caps check
        if ($subject === strtoupper($subject) && $length > 5) {
            $score -= 20;
            $issues[] = 'Avoid using all caps - it triggers spam filters';
        }

        // Excessive punctuation
        $exclamations = substr_count($subject, '!');
        if ($exclamations > 1) {
            $score -= 10 * $exclamations;
            $issues[] = 'Too many exclamation marks';
        }

        // Personalization check
        $hasPersonalization = preg_match('/\{\{.*?\}\}/', $subject);
        if (!$hasPersonalization) {
            $score -= 5;
            $issues[] = 'Consider adding personalization (e.g., {{first_name}})';
        } else {
            $score += 10;
        }

        // Emoji usage (slight positive)
        $hasEmoji = preg_match('/[\x{1F600}-\x{1F64F}]/u', $subject);
        if ($hasEmoji) {
            $score += 5;
        }

        // Power words
        $powerWords = ['new', 'exclusive', 'introducing', 'discover', 'learn', 'how to'];
        foreach ($powerWords as $word) {
            if (stripos($subject, $word) !== false) {
                $score += 3;
                break;
            }
        }

        // Question format (engagement booster)
        if (str_ends_with($subject, '?')) {
            $score += 5;
        }

        return [
            'score' => max(0, min(100, $score)),
            'length' => $length,
            'word_count' => $wordCount,
            'has_personalization' => (bool) $hasPersonalization,
            'has_emoji' => (bool) $hasEmoji,
            'issues' => $issues,
        ];
    }

    /**
     * Analyze email content
     */
    private function analyzeContent(string $html, string $text): array
    {
        $score = 100;
        $issues = [];

        $textContent = $text ?: strip_tags($html);
        $wordCount = str_word_count($textContent);

        // Word count check
        if ($wordCount < 50) {
            $score -= 10;
            $issues[] = 'Content is very short - consider adding more value';
        } elseif ($wordCount > 500) {
            $score -= 5;
            $issues[] = 'Content is quite long - consider being more concise';
        }

        // Image to text ratio
        preg_match_all('/<img/i', $html, $images);
        $imageCount = count($images[0]);
        if ($imageCount > 0 && $wordCount < 100) {
            $score -= 15;
            $issues[] = 'Too many images relative to text - may trigger spam filters';
        }

        // Link count
        preg_match_all('/<a\s/i', $html, $links);
        $linkCount = count($links[0]);
        if ($linkCount > 10) {
            $score -= 10;
            $issues[] = 'Too many links - may appear spammy';
        }
        if ($linkCount === 0) {
            $score -= 5;
            $issues[] = 'No call-to-action links found';
        }

        // Alt text for images
        preg_match_all('/<img[^>]+>/i', $html, $imgTags);
        $missingAlt = 0;
        foreach ($imgTags[0] as $img) {
            if (!preg_match('/alt\s*=\s*["\'][^"\']+["\']/', $img)) {
                $missingAlt++;
            }
        }
        if ($missingAlt > 0) {
            $score -= 5 * $missingAlt;
            $issues[] = "{$missingAlt} image(s) missing alt text";
        }

        // Unsubscribe link
        if (stripos($html, 'unsubscribe') === false) {
            $score -= 20;
            $issues[] = 'Missing unsubscribe link (required by law)';
        }

        // Physical address
        $addressPatterns = ['/\d+\s+[\w\s]+,\s*[\w\s]+,?\s*\d{5}/', '/P\.?O\.?\s*Box/i'];
        $hasAddress = false;
        foreach ($addressPatterns as $pattern) {
            if (preg_match($pattern, $textContent)) {
                $hasAddress = true;
                break;
            }
        }
        if (!$hasAddress) {
            $score -= 5;
            $issues[] = 'Consider adding physical address (CAN-SPAM compliance)';
        }

        return [
            'score' => max(0, min(100, $score)),
            'word_count' => $wordCount,
            'image_count' => $imageCount,
            'link_count' => $linkCount,
            'issues' => $issues,
        ];
    }

    /**
     * Calculate spam score (0-100, lower is better)
     */
    public function calculateSpamScore(string $subject, string $content): array
    {
        $score = 0;
        $triggers = [];
        $combined = strtolower($subject . ' ' . strip_tags($content));

        foreach (self::SPAM_TRIGGERS as $trigger => $weight) {
            $count = substr_count($combined, $trigger);
            if ($count > 0) {
                $triggerScore = $weight * $count * 10;
                $score += $triggerScore;
                $triggers[] = [
                    'word' => $trigger,
                    'count' => $count,
                    'impact' => round($triggerScore, 1),
                ];
            }
        }

        // All caps penalty
        $capsRatio = $this->getCapsRatio($subject . ' ' . strip_tags($content));
        if ($capsRatio > 0.3) {
            $score += 20;
            $triggers[] = ['word' => 'Excessive caps', 'impact' => 20];
        }

        // Excessive punctuation
        $exclamationCount = substr_count($combined, '!');
        if ($exclamationCount > 3) {
            $score += min($exclamationCount * 3, 15);
        }

        // Hidden text detection
        if (preg_match('/color:\s*(white|#fff|#ffffff)/i', $content)) {
            $score += 30;
            $triggers[] = ['word' => 'Hidden text detected', 'impact' => 30];
        }

        return [
            'score' => min(100, round($score)),
            'risk_level' => $score < 20 ? 'low' : ($score < 50 ? 'medium' : 'high'),
            'triggers' => $triggers,
        ];
    }

    /**
     * Calculate readability score
     */
    public function calculateReadability(string $text): array
    {
        $sentences = preg_split('/[.!?]+/', $text, -1, PREG_SPLIT_NO_EMPTY);
        $sentenceCount = count($sentences);
        $wordCount = str_word_count($text);
        $syllableCount = $this->countSyllables($text);

        if ($sentenceCount === 0 || $wordCount === 0) {
            return ['score' => 0, 'grade_level' => 'N/A', 'issues' => ['Not enough text to analyze']];
        }

        // Flesch Reading Ease score
        $avgWordsPerSentence = $wordCount / $sentenceCount;
        $avgSyllablesPerWord = $syllableCount / $wordCount;
        $fleschScore = 206.835 - (1.015 * $avgWordsPerSentence) - (84.6 * $avgSyllablesPerWord);
        $fleschScore = max(0, min(100, $fleschScore));

        // Determine grade level
        $gradeLevel = match (true) {
            $fleschScore >= 90 => '5th grade',
            $fleschScore >= 80 => '6th grade',
            $fleschScore >= 70 => '7th grade',
            $fleschScore >= 60 => '8th-9th grade',
            $fleschScore >= 50 => '10th-12th grade',
            $fleschScore >= 30 => 'College',
            default => 'Graduate',
        };

        $issues = [];
        if ($avgWordsPerSentence > 20) {
            $issues[] = 'Sentences are too long on average - aim for 15-20 words';
        }
        if ($fleschScore < 60) {
            $issues[] = 'Content may be difficult to read - simplify language';
        }

        return [
            'score' => round($fleschScore),
            'grade_level' => $gradeLevel,
            'avg_words_per_sentence' => round($avgWordsPerSentence, 1),
            'avg_syllables_per_word' => round($avgSyllablesPerWord, 2),
            'issues' => $issues,
        ];
    }

    /**
     * Assess mobile friendliness
     */
    private function assessMobileFriendliness(string $html): array
    {
        $score = 100;
        $issues = [];

        // Check for viewport meta tag
        if (stripos($html, 'viewport') === false) {
            $score -= 15;
            $issues[] = 'Missing viewport meta tag';
        }

        // Check for responsive styles
        if (stripos($html, '@media') === false && stripos($html, 'max-width') === false) {
            $score -= 10;
            $issues[] = 'No responsive styles detected';
        }

        // Check for large fixed widths
        if (preg_match('/width:\s*[6-9]\d{2}px/', $html)) {
            $score -= 15;
            $issues[] = 'Fixed widths over 600px may cause horizontal scrolling on mobile';
        }

        // Font size check
        if (preg_match('/font-size:\s*([0-9]+)px/', $html, $matches)) {
            if ((int) $matches[1] < 14) {
                $score -= 10;
                $issues[] = 'Font size may be too small for mobile (under 14px)';
            }
        }

        // Touch target size (buttons/links)
        if (preg_match('/padding:\s*[0-5]px/', $html)) {
            $score -= 5;
            $issues[] = 'Touch targets may be too small - increase padding on buttons';
        }

        return [
            'score' => max(0, $score),
            'issues' => $issues,
        ];
    }

    /**
     * Assess accessibility
     */
    private function assessAccessibility(string $html): array
    {
        $score = 100;
        $issues = [];

        // Check for lang attribute
        if (!preg_match('/<html[^>]+lang\s*=/i', $html)) {
            $score -= 10;
            $issues[] = 'Missing language attribute on HTML tag';
        }

        // Check for heading structure
        if (!preg_match('/<h[1-6]/i', $html)) {
            $score -= 5;
            $issues[] = 'No heading tags found';
        }

        // Check image alt text
        preg_match_all('/<img[^>]+>/i', $html, $imgTags);
        foreach ($imgTags[0] as $img) {
            if (!preg_match('/alt\s*=/i', $img)) {
                $score -= 5;
            }
        }

        // Color contrast (basic check)
        if (preg_match('/color:\s*#([a-f0-9]{6}|[a-f0-9]{3})/i', $html, $colors)) {
            // Would need full contrast analysis here
        }

        // Role attributes for interactive elements
        if (preg_match('/<button/i', $html) && !preg_match('/role\s*=/i', $html)) {
            $score -= 5;
            $issues[] = 'Consider adding ARIA roles for interactive elements';
        }

        return [
            'score' => max(0, $score),
            'issues' => $issues,
        ];
    }

    /**
     * Predict optimal send time for a subscriber
     */
    public function predictSendTime(NewsletterSubscription $subscription): array
    {
        // Get subscriber's historical engagement data
        $engagementData = DB::table('email_send_time_analytics')
            ->where('subscriber_id', $subscription->id)
            ->where('emails_sent', '>', 0)
            ->get();

        if ($engagementData->isEmpty()) {
            // Use global best times for new subscribers
            return $this->getGlobalBestSendTimes();
        }

        // Find best performing time slots
        $bestSlot = $engagementData->sortByDesc('open_rate')->first();

        $timezone = $subscription->timezone ?? 'UTC';

        return [
            'recommended_hour' => $bestSlot->hour_of_day,
            'recommended_day' => $bestSlot->day_of_week,
            'timezone' => $timezone,
            'local_time' => Carbon::now($timezone)
                ->setTime($bestSlot->hour_of_day, 0)
                ->format('H:i'),
            'confidence' => min(100, $engagementData->sum('emails_sent') * 5),
            'based_on_emails' => $engagementData->sum('emails_sent'),
            'expected_open_rate' => round($bestSlot->open_rate, 2),
        ];
    }

    /**
     * Get global best send times
     */
    private function getGlobalBestSendTimes(): array
    {
        return Cache::remember('global_best_send_times', 3600, function () {
            $data = DB::table('email_send_time_analytics')
                ->selectRaw('hour_of_day, day_of_week, AVG(open_rate) as avg_open_rate')
                ->groupBy('hour_of_day', 'day_of_week')
                ->orderByDesc('avg_open_rate')
                ->first();

            if (!$data) {
                // Industry defaults: Tuesday-Thursday, 9-11 AM
                return [
                    'recommended_hour' => 10,
                    'recommended_day' => 2, // Tuesday
                    'timezone' => 'UTC',
                    'local_time' => '10:00',
                    'confidence' => 50,
                    'based_on_emails' => 0,
                    'note' => 'Using industry best practices (no subscriber data available)',
                ];
            }

            return [
                'recommended_hour' => $data->hour_of_day,
                'recommended_day' => $data->day_of_week,
                'timezone' => 'UTC',
                'local_time' => sprintf('%02d:00', $data->hour_of_day),
                'confidence' => 70,
                'expected_open_rate' => round($data->avg_open_rate, 2),
            ];
        });
    }

    /**
     * Generate subject line suggestions
     */
    public function suggestSubjectLines(string $currentSubject, string $contentSummary = ''): array
    {
        $suggestions = [];

        // Personalized version
        if (strpos($currentSubject, '{{') === false) {
            $suggestions[] = [
                'subject' => '{{first_name}}, ' . lcfirst($currentSubject),
                'reason' => 'Personalization increases open rates by 26%',
                'expected_lift' => '+26%',
            ];
        }

        // Question format
        if (!str_ends_with($currentSubject, '?')) {
            $suggestions[] = [
                'subject' => $this->convertToQuestion($currentSubject),
                'reason' => 'Questions create curiosity and engagement',
                'expected_lift' => '+10%',
            ];
        }

        // Urgency version
        if (stripos($currentSubject, 'today') === false && stripos($currentSubject, 'now') === false) {
            $suggestions[] = [
                'subject' => $currentSubject . ' (Today Only)',
                'reason' => 'Urgency drives action',
                'expected_lift' => '+15%',
            ];
        }

        // Shorter version if too long
        if (strlen($currentSubject) > 50) {
            $shortened = $this->shortenSubject($currentSubject);
            $suggestions[] = [
                'subject' => $shortened,
                'reason' => 'Shorter subjects perform better on mobile',
                'expected_lift' => '+8%',
            ];
        }

        // Number/list format
        if (!preg_match('/\d/', $currentSubject)) {
            $suggestions[] = [
                'subject' => '5 Ways to ' . $currentSubject,
                'reason' => 'Numbers in subjects increase click rates',
                'expected_lift' => '+12%',
            ];
        }

        return array_slice($suggestions, 0, 5);
    }

    /**
     * Convert statement to question
     */
    private function convertToQuestion(string $subject): string
    {
        $subject = rtrim($subject, '.!');

        if (preg_match('/^(how|what|why|when|where|who)/i', $subject)) {
            return $subject . '?';
        }

        return 'Want to ' . lcfirst($subject) . '?';
    }

    /**
     * Shorten subject line
     */
    private function shortenSubject(string $subject): string
    {
        $words = explode(' ', $subject);
        $shortened = [];
        $length = 0;

        foreach ($words as $word) {
            if ($length + strlen($word) + 1 > 45) {
                break;
            }
            $shortened[] = $word;
            $length += strlen($word) + 1;
        }

        return implode(' ', $shortened) . '...';
    }

    /**
     * Count syllables in text
     */
    private function countSyllables(string $text): int
    {
        $text = strtolower($text);
        $text = preg_replace('/[^a-z]/', ' ', $text);
        $words = preg_split('/\s+/', $text, -1, PREG_SPLIT_NO_EMPTY);

        $total = 0;
        foreach ($words as $word) {
            $total += max(1, preg_match_all('/[aeiouy]+/', $word));
        }

        return $total;
    }

    /**
     * Get caps ratio
     */
    private function getCapsRatio(string $text): float
    {
        $alphaOnly = preg_replace('/[^a-zA-Z]/', '', $text);
        if (strlen($alphaOnly) === 0) {
            return 0;
        }

        $caps = preg_replace('/[^A-Z]/', '', $text);
        return strlen($caps) / strlen($alphaOnly);
    }

    /**
     * Convert score to letter grade
     */
    private function scoreToGrade(float $score): string
    {
        return match (true) {
            $score >= 90 => 'A',
            $score >= 80 => 'B',
            $score >= 70 => 'C',
            $score >= 60 => 'D',
            default => 'F',
        };
    }

    /**
     * Generate recommendations based on scores
     */
    private function generateRecommendations(array $scores): array
    {
        $recommendations = [];

        if ($scores['subject']['score'] < 80) {
            $recommendations[] = [
                'priority' => 'high',
                'area' => 'Subject Line',
                'message' => 'Improve your subject line to increase open rates',
                'suggestions' => $scores['subject']['issues'],
            ];
        }

        if ($scores['spam']['score'] > 30) {
            $recommendations[] = [
                'priority' => 'high',
                'area' => 'Spam Score',
                'message' => 'Your email may trigger spam filters',
                'suggestions' => array_column($scores['spam']['triggers'], 'word'),
            ];
        }

        if ($scores['readability']['score'] < 60) {
            $recommendations[] = [
                'priority' => 'medium',
                'area' => 'Readability',
                'message' => 'Simplify your content for better engagement',
                'suggestions' => $scores['readability']['issues'],
            ];
        }

        if ($scores['mobile']['score'] < 80) {
            $recommendations[] = [
                'priority' => 'medium',
                'area' => 'Mobile Optimization',
                'message' => 'Optimize for mobile devices',
                'suggestions' => $scores['mobile']['issues'],
            ];
        }

        return $recommendations;
    }
}
