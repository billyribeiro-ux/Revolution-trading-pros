<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;

/**
 * FormQuiz Model - FluentForm Pro Quiz & Survey Module
 *
 * Handles quiz configuration, scoring, grading, and result management.
 *
 * @property int $id
 * @property int $form_id
 * @property string $quiz_type
 * @property bool $enabled
 * @property string $scoring_type
 * @property float $passing_score
 * @property bool $show_score
 * @property bool $show_correct_answers
 * @property bool $randomize_questions
 * @property bool $randomize_options
 * @property int|null $time_limit
 * @property int|null $attempts_allowed
 * @property array|null $grade_settings
 * @property array|null $result_messages
 * @property bool $instant_feedback
 * @property bool $allow_retake
 * @property Carbon $created_at
 * @property Carbon $updated_at
 */
class FormQuiz extends Model
{
    use HasFactory;

    protected $table = 'form_quizzes';

    protected $fillable = [
        'form_id',
        'quiz_type',
        'enabled',
        'scoring_type',
        'passing_score',
        'show_score',
        'show_correct_answers',
        'randomize_questions',
        'randomize_options',
        'time_limit',
        'attempts_allowed',
        'grade_settings',
        'result_messages',
        'instant_feedback',
        'allow_retake',
    ];

    protected $casts = [
        'form_id' => 'integer',
        'enabled' => 'boolean',
        'passing_score' => 'float',
        'show_score' => 'boolean',
        'show_correct_answers' => 'boolean',
        'randomize_questions' => 'boolean',
        'randomize_options' => 'boolean',
        'time_limit' => 'integer',
        'attempts_allowed' => 'integer',
        'grade_settings' => 'array',
        'result_messages' => 'array',
        'instant_feedback' => 'boolean',
        'allow_retake' => 'boolean',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    protected $attributes = [
        'enabled' => true,
        'scoring_type' => 'points',
        'passing_score' => 70,
        'show_score' => true,
        'show_correct_answers' => false,
        'randomize_questions' => false,
        'randomize_options' => false,
        'instant_feedback' => false,
        'allow_retake' => true,
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    // Quiz Types
    public const TYPE_QUIZ = 'quiz';
    public const TYPE_SURVEY = 'survey';
    public const TYPE_ASSESSMENT = 'assessment';
    public const TYPE_POLL = 'poll';
    public const TYPE_NPS = 'nps';
    public const TYPE_PERSONALITY = 'personality';

    public const QUIZ_TYPES = [
        self::TYPE_QUIZ => 'Quiz',
        self::TYPE_SURVEY => 'Survey',
        self::TYPE_ASSESSMENT => 'Assessment',
        self::TYPE_POLL => 'Poll',
        self::TYPE_NPS => 'Net Promoter Score',
        self::TYPE_PERSONALITY => 'Personality Test',
    ];

    // Scoring Types
    public const SCORING_POINTS = 'points';
    public const SCORING_PERCENTAGE = 'percentage';
    public const SCORING_LETTER_GRADE = 'letter_grade';
    public const SCORING_PASS_FAIL = 'pass_fail';
    public const SCORING_CUSTOM = 'custom';

    public const SCORING_TYPES = [
        self::SCORING_POINTS => 'Points',
        self::SCORING_PERCENTAGE => 'Percentage',
        self::SCORING_LETTER_GRADE => 'Letter Grade (A-F)',
        self::SCORING_PASS_FAIL => 'Pass/Fail',
        self::SCORING_CUSTOM => 'Custom Scoring',
    ];

    // Default Grade Settings
    public const DEFAULT_GRADES = [
        ['min' => 90, 'max' => 100, 'grade' => 'A', 'label' => 'Excellent'],
        ['min' => 80, 'max' => 89, 'grade' => 'B', 'label' => 'Good'],
        ['min' => 70, 'max' => 79, 'grade' => 'C', 'label' => 'Average'],
        ['min' => 60, 'max' => 69, 'grade' => 'D', 'label' => 'Below Average'],
        ['min' => 0, 'max' => 59, 'grade' => 'F', 'label' => 'Fail'],
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class);
    }

    public function results(): HasMany
    {
        return $this->hasMany(FormQuizResult::class, 'quiz_id');
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    public function scopeEnabled(Builder $query): Builder
    {
        return $query->where('enabled', true);
    }

    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('quiz_type', $type);
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    public function getTypeLabelAttribute(): string
    {
        return self::QUIZ_TYPES[$this->quiz_type] ?? ucfirst($this->quiz_type);
    }

    public function getScoringLabelAttribute(): string
    {
        return self::SCORING_TYPES[$this->scoring_type] ?? ucfirst($this->scoring_type);
    }

    public function getTimeLimitFormattedAttribute(): ?string
    {
        if (!$this->time_limit) {
            return null;
        }
        $minutes = floor($this->time_limit / 60);
        $seconds = $this->time_limit % 60;
        return $minutes > 0 ? "{$minutes}m {$seconds}s" : "{$seconds}s";
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    /**
     * Calculate score from answers.
     */
    public function calculateScore(array $answers, array $questions): array
    {
        $totalPoints = 0;
        $earnedPoints = 0;
        $correctAnswers = 0;
        $totalQuestions = count($questions);
        $details = [];

        foreach ($questions as $question) {
            $fieldName = $question['name'];
            $correctAnswer = $question['correct_answer'] ?? null;
            $points = $question['points'] ?? 1;
            $userAnswer = $answers[$fieldName] ?? null;

            $totalPoints += $points;
            $isCorrect = $this->isAnswerCorrect($userAnswer, $correctAnswer, $question['type'] ?? 'single');

            if ($isCorrect) {
                $earnedPoints += $points;
                $correctAnswers++;
            }

            $details[$fieldName] = [
                'user_answer' => $userAnswer,
                'correct_answer' => $correctAnswer,
                'is_correct' => $isCorrect,
                'points_earned' => $isCorrect ? $points : 0,
                'points_possible' => $points,
            ];
        }

        $percentage = $totalPoints > 0 ? ($earnedPoints / $totalPoints) * 100 : 0;
        $grade = $this->calculateGrade($percentage);
        $passed = $percentage >= $this->passing_score;

        return [
            'total_points' => $totalPoints,
            'earned_points' => $earnedPoints,
            'percentage' => round($percentage, 2),
            'correct_answers' => $correctAnswers,
            'total_questions' => $totalQuestions,
            'grade' => $grade,
            'passed' => $passed,
            'details' => $details,
            'result_message' => $this->getResultMessage($percentage, $passed),
        ];
    }

    /**
     * Check if answer is correct.
     */
    protected function isAnswerCorrect(mixed $userAnswer, mixed $correctAnswer, string $type): bool
    {
        if ($correctAnswer === null) {
            return true; // No correct answer defined
        }

        if ($type === 'multiple') {
            // Multiple correct answers
            $userAnswers = is_array($userAnswer) ? $userAnswer : [$userAnswer];
            $correctAnswers = is_array($correctAnswer) ? $correctAnswer : [$correctAnswer];
            sort($userAnswers);
            sort($correctAnswers);
            return $userAnswers == $correctAnswers;
        }

        // Single answer comparison
        return strtolower(trim((string) $userAnswer)) === strtolower(trim((string) $correctAnswer));
    }

    /**
     * Calculate letter grade from percentage.
     */
    public function calculateGrade(float $percentage): string
    {
        $grades = $this->grade_settings ?? self::DEFAULT_GRADES;

        foreach ($grades as $grade) {
            if ($percentage >= $grade['min'] && $percentage <= $grade['max']) {
                return $grade['grade'];
            }
        }

        return 'F';
    }

    /**
     * Get result message based on score.
     */
    public function getResultMessage(float $percentage, bool $passed): string
    {
        $messages = $this->result_messages ?? [];

        if ($passed && isset($messages['pass'])) {
            return $this->interpolateMessage($messages['pass'], $percentage);
        }

        if (!$passed && isset($messages['fail'])) {
            return $this->interpolateMessage($messages['fail'], $percentage);
        }

        // Check custom range messages
        if (isset($messages['ranges'])) {
            foreach ($messages['ranges'] as $range) {
                if ($percentage >= $range['min'] && $percentage <= $range['max']) {
                    return $this->interpolateMessage($range['message'], $percentage);
                }
            }
        }

        return $passed
            ? 'Congratulations! You passed with ' . round($percentage, 1) . '%'
            : 'You scored ' . round($percentage, 1) . '%. You need ' . $this->passing_score . '% to pass.';
    }

    /**
     * Interpolate variables in message.
     */
    protected function interpolateMessage(string $message, float $percentage): string
    {
        return str_replace(
            ['{{percentage}}', '{{passing_score}}', '{{grade}}'],
            [round($percentage, 1) . '%', $this->passing_score . '%', $this->calculateGrade($percentage)],
            $message
        );
    }

    /**
     * Check if user can retake quiz.
     */
    public function canRetake(int $userId): bool
    {
        if (!$this->allow_retake) {
            return false;
        }

        if (!$this->attempts_allowed) {
            return true; // Unlimited attempts
        }

        $attemptCount = $this->results()
            ->where('user_id', $userId)
            ->count();

        return $attemptCount < $this->attempts_allowed;
    }

    /**
     * Get quiz statistics.
     */
    public function getStatistics(): array
    {
        $results = $this->results()->get();

        if ($results->isEmpty()) {
            return [
                'total_attempts' => 0,
                'unique_users' => 0,
                'average_score' => 0,
                'pass_rate' => 0,
                'highest_score' => 0,
                'lowest_score' => 0,
            ];
        }

        return [
            'total_attempts' => $results->count(),
            'unique_users' => $results->unique('user_id')->count(),
            'average_score' => round($results->avg('percentage'), 2),
            'pass_rate' => round(($results->where('passed', true)->count() / $results->count()) * 100, 2),
            'highest_score' => $results->max('percentage'),
            'lowest_score' => $results->min('percentage'),
            'score_distribution' => $this->getScoreDistribution($results),
        ];
    }

    /**
     * Get score distribution.
     */
    protected function getScoreDistribution($results): array
    {
        $ranges = [
            '0-20' => 0,
            '21-40' => 0,
            '41-60' => 0,
            '61-80' => 0,
            '81-100' => 0,
        ];

        foreach ($results as $result) {
            $score = $result->percentage;
            if ($score <= 20) $ranges['0-20']++;
            elseif ($score <= 40) $ranges['21-40']++;
            elseif ($score <= 60) $ranges['41-60']++;
            elseif ($score <= 80) $ranges['61-80']++;
            else $ranges['81-100']++;
        }

        return $ranges;
    }
}
