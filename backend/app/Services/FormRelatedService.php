<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;

/**
 * Form Related Service - Smart form recommendations
 *
 * Features:
 * - Similar forms by content
 * - Category-based suggestions
 * - Popularity-based ranking
 * - User behavior analysis
 * - A/B test recommendations
 * - Sequential form chains
 * - Tag-based matching
 * - Collaborative filtering
 *
 * @version 1.0.0
 */
class FormRelatedService
{
    /**
     * Similarity weights
     */
    private const WEIGHTS = [
        'title' => 3.0,
        'description' => 2.0,
        'category' => 2.5,
        'tags' => 2.0,
        'field_types' => 1.5,
        'field_count' => 0.5,
    ];

    /**
     * Get related forms
     */
    public function getRelatedForms(Form $form, int $limit = 5): array
    {
        $cacheKey = "related_forms:{$form->id}:{$limit}";

        return Cache::remember($cacheKey, now()->addHours(6), function () use ($form, $limit) {
            // First check manual relations
            $manualRelations = $this->getManualRelations($form->id, $limit);

            if (count($manualRelations) >= $limit) {
                return $manualRelations;
            }

            // Calculate similarity scores
            $remaining = $limit - count($manualRelations);
            $excludeIds = array_merge([$form->id], array_column($manualRelations, 'id'));

            $similarForms = $this->findSimilarForms($form, $remaining, $excludeIds);

            return array_merge($manualRelations, $similarForms);
        });
    }

    /**
     * Get manual relations
     */
    private function getManualRelations(int $formId, int $limit): array
    {
        return DB::table('form_relations')
            ->join('forms', 'form_relations.related_form_id', '=', 'forms.id')
            ->where('form_relations.form_id', $formId)
            ->where('form_relations.manual', true)
            ->where('forms.status', 'published')
            ->orderByDesc('form_relations.score')
            ->limit($limit)
            ->select([
                'forms.id',
                'forms.title',
                'forms.slug',
                'forms.description',
                'forms.category',
                'form_relations.relation_type',
            ])
            ->get()
            ->toArray();
    }

    /**
     * Find similar forms using content analysis
     */
    private function findSimilarForms(Form $form, int $limit, array $excludeIds): array
    {
        $candidates = Form::where('status', 'published')
            ->whereNotIn('id', $excludeIds)
            ->limit(100)
            ->get();

        $scored = [];

        foreach ($candidates as $candidate) {
            $score = $this->calculateSimilarityScore($form, $candidate);
            if ($score > 0.1) {
                $scored[] = [
                    'id' => $candidate->id,
                    'title' => $candidate->title,
                    'slug' => $candidate->slug,
                    'description' => $candidate->description,
                    'category' => $candidate->category,
                    'score' => $score,
                    'relation_type' => 'similar',
                ];
            }
        }

        // Sort by score descending
        usort($scored, fn($a, $b) => $b['score'] <=> $a['score']);

        return array_slice($scored, 0, $limit);
    }

    /**
     * Calculate similarity score between two forms
     */
    private function calculateSimilarityScore(Form $source, Form $target): float
    {
        $score = 0;
        $maxScore = array_sum(self::WEIGHTS);

        // Title similarity
        $titleSimilarity = $this->textSimilarity($source->title, $target->title);
        $score += $titleSimilarity * self::WEIGHTS['title'];

        // Description similarity
        if ($source->description && $target->description) {
            $descSimilarity = $this->textSimilarity($source->description, $target->description);
            $score += $descSimilarity * self::WEIGHTS['description'];
        }

        // Category match
        if ($source->category && $target->category && $source->category === $target->category) {
            $score += self::WEIGHTS['category'];
        }

        // Tag overlap
        $sourceTags = $source->tags ?? [];
        $targetTags = $target->tags ?? [];
        if (!empty($sourceTags) && !empty($targetTags)) {
            $tagOverlap = count(array_intersect($sourceTags, $targetTags)) /
                max(count(array_union($sourceTags, $targetTags)), 1);
            $score += $tagOverlap * self::WEIGHTS['tags'];
        }

        // Field type similarity
        $sourceFieldTypes = $source->fields()->pluck('type')->toArray();
        $targetFieldTypes = $target->fields()->pluck('type')->toArray();

        if (!empty($sourceFieldTypes) && !empty($targetFieldTypes)) {
            $fieldTypeOverlap = count(array_intersect($sourceFieldTypes, $targetFieldTypes)) /
                max(count(array_union($sourceFieldTypes, $targetFieldTypes)), 1);
            $score += $fieldTypeOverlap * self::WEIGHTS['field_types'];
        }

        // Field count similarity (penalize big differences)
        $sourceCount = count($sourceFieldTypes);
        $targetCount = count($targetFieldTypes);
        if ($sourceCount > 0 && $targetCount > 0) {
            $countSimilarity = 1 - abs($sourceCount - $targetCount) / max($sourceCount, $targetCount);
            $score += $countSimilarity * self::WEIGHTS['field_count'];
        }

        return $score / $maxScore;
    }

    /**
     * Calculate text similarity using word overlap
     */
    private function textSimilarity(string $text1, string $text2): float
    {
        $words1 = $this->tokenize($text1);
        $words2 = $this->tokenize($text2);

        if (empty($words1) || empty($words2)) {
            return 0;
        }

        $intersection = count(array_intersect($words1, $words2));
        $union = count(array_unique(array_merge($words1, $words2)));

        return $union > 0 ? $intersection / $union : 0;
    }

    /**
     * Tokenize text into words
     */
    private function tokenize(string $text): array
    {
        $text = strtolower($text);
        $words = preg_split('/\W+/', $text, -1, PREG_SPLIT_NO_EMPTY);

        // Remove stop words
        $stopWords = ['the', 'a', 'an', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with', 'by', 'from', 'is', 'are', 'was', 'were', 'be', 'been', 'being', 'have', 'has', 'had', 'do', 'does', 'did', 'will', 'would', 'could', 'should', 'may', 'might', 'must', 'shall', 'form', 'your'];

        return array_diff($words, $stopWords);
    }

    /**
     * Get forms by category
     */
    public function getByCategory(string $category, int $limit = 10, ?int $excludeId = null): array
    {
        $query = Form::where('status', 'published')
            ->where('category', $category);

        if ($excludeId) {
            $query->where('id', '!=', $excludeId);
        }

        return $query->orderByDesc('submission_count')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get popular forms
     */
    public function getPopular(int $limit = 10, ?string $category = null): array
    {
        $query = Form::where('status', 'published');

        if ($category) {
            $query->where('category', $category);
        }

        return $query->orderByDesc('submission_count')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get trending forms (recent high activity)
     */
    public function getTrending(int $days = 7, int $limit = 10): array
    {
        return DB::table('forms')
            ->join('form_submissions', 'forms.id', '=', 'form_submissions.form_id')
            ->where('forms.status', 'published')
            ->where('form_submissions.created_at', '>=', now()->subDays($days))
            ->select([
                'forms.id',
                'forms.title',
                'forms.slug',
                'forms.description',
                'forms.category',
            ])
            ->selectRaw('COUNT(form_submissions.id) as recent_submissions')
            ->groupBy('forms.id', 'forms.title', 'forms.slug', 'forms.description', 'forms.category')
            ->orderByDesc('recent_submissions')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get forms by tag
     */
    public function getByTag(string $tag, int $limit = 10): array
    {
        return Form::where('status', 'published')
            ->whereJsonContains('tags', $tag)
            ->orderByDesc('submission_count')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Add manual relation
     */
    public function addRelation(int $formId, int $relatedFormId, string $type = 'similar'): bool
    {
        return DB::table('form_relations')->updateOrInsert(
            ['form_id' => $formId, 'related_form_id' => $relatedFormId],
            [
                'relation_type' => $type,
                'manual' => true,
                'score' => 1.0,
                'updated_at' => now(),
            ]
        );
    }

    /**
     * Remove relation
     */
    public function removeRelation(int $formId, int $relatedFormId): bool
    {
        return DB::table('form_relations')
            ->where('form_id', $formId)
            ->where('related_form_id', $relatedFormId)
            ->delete() > 0;
    }

    /**
     * Recalculate all relations for a form
     */
    public function recalculateRelations(int $formId): int
    {
        $form = Form::find($formId);
        if (!$form) {
            return 0;
        }

        // Clear cache
        Cache::forget("related_forms:{$formId}:5");

        // Get similar forms
        $similar = $this->findSimilarForms($form, 10, [$formId]);

        // Store calculated relations
        $count = 0;
        foreach ($similar as $related) {
            DB::table('form_relations')->updateOrInsert(
                ['form_id' => $formId, 'related_form_id' => $related['id']],
                [
                    'relation_type' => 'similar',
                    'manual' => false,
                    'score' => $related['score'],
                    'updated_at' => now(),
                ]
            );
            $count++;
        }

        return $count;
    }

    /**
     * Get "You might also like" suggestions for a user
     */
    public function getPersonalizedSuggestions(int $userId, int $limit = 5): array
    {
        // Get forms the user has submitted to
        $submittedFormIds = DB::table('form_submissions')
            ->where('user_id', $userId)
            ->distinct()
            ->pluck('form_id')
            ->toArray();

        if (empty($submittedFormIds)) {
            // Return popular forms if no history
            return $this->getPopular($limit);
        }

        // Get categories/tags from submitted forms
        $categories = Form::whereIn('id', $submittedFormIds)
            ->whereNotNull('category')
            ->pluck('category')
            ->unique()
            ->toArray();

        // Find similar forms
        return Form::where('status', 'published')
            ->whereNotIn('id', $submittedFormIds)
            ->where(function ($query) use ($categories) {
                if (!empty($categories)) {
                    $query->whereIn('category', $categories);
                }
            })
            ->orderByDesc('submission_count')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get form chain (sequential forms)
     */
    public function getFormChain(int $formId): array
    {
        // Get forms marked as sequels
        $next = DB::table('form_relations')
            ->join('forms', 'form_relations.related_form_id', '=', 'forms.id')
            ->where('form_relations.form_id', $formId)
            ->where('form_relations.relation_type', 'sequel')
            ->where('forms.status', 'published')
            ->orderBy('form_relations.score')
            ->select(['forms.id', 'forms.title', 'forms.slug'])
            ->get()
            ->toArray();

        // Get forms that have this form as sequel (previous)
        $previous = DB::table('form_relations')
            ->join('forms', 'form_relations.form_id', '=', 'forms.id')
            ->where('form_relations.related_form_id', $formId)
            ->where('form_relations.relation_type', 'sequel')
            ->where('forms.status', 'published')
            ->orderByDesc('form_relations.score')
            ->select(['forms.id', 'forms.title', 'forms.slug'])
            ->get()
            ->toArray();

        return [
            'previous' => $previous,
            'next' => $next,
        ];
    }
}

/**
 * Helper function for array_union (PHP doesn't have it built-in)
 */
function array_union(array $a, array $b): array
{
    return array_unique(array_merge($a, $b));
}
