<?php

namespace App\Services;

use App\Models\Contact;
use App\Models\LeadScoreLog;
use Illuminate\Support\Str;

class LeadScoringService
{
    public function calculateLeadScore(Contact $contact): int
    {
        $demographicScore = $this->calculateDemographicScore($contact);
        $behavioralScore = $this->calculateBehavioralScore($contact);
        $engagementScore = $this->calculateEngagementScore($contact);
        $intentScore = $this->calculateIntentScore($contact);
        $fitScore = $this->calculateFitScore($contact);

        $totalScore = (
            $demographicScore * 0.20 +
            $behavioralScore * 0.30 +
            $engagementScore * 0.25 +
            $intentScore * 0.15 +
            $fitScore * 0.10
        );

        return min(100, max(0, (int) round($totalScore)));
    }

    private function calculateDemographicScore(Contact $contact): int
    {
        $score = 0;

        // Job title relevance
        if ($contact->job_title) {
            if (preg_match('/(CEO|CTO|CFO|Chief|President|VP|Vice President)/i', $contact->job_title)) {
                $score += 20;
            } elseif (preg_match('/(Director|Manager|Head)/i', $contact->job_title)) {
                $score += 15;
            } else {
                $score += 10;
            }
        }

        // Email domain quality
        if ($contact->email && !preg_match('/@(gmail|yahoo|hotmail|outlook)\./i', $contact->email)) {
            $score += 10;
        }

        // Has phone
        if ($contact->phone || $contact->mobile) {
            $score += 5;
        }

        // Has LinkedIn
        if ($contact->linkedin_url) {
            $score += 10;
        }

        return min(100, $score);
    }

    private function calculateBehavioralScore(Contact $contact): int
    {
        $score = 0;

        // Session activity
        $score += min(30, $contact->total_sessions);

        // Engagement score from behavior system
        if ($contact->avg_engagement_score >= 75) {
            $score += 30;
        } elseif ($contact->avg_engagement_score >= 50) {
            $score += 20;
        } elseif ($contact->avg_engagement_score >= 25) {
            $score += 10;
        }

        // Intent score from behavior system
        if ($contact->avg_intent_score >= 75) {
            $score += 20;
        } elseif ($contact->avg_intent_score >= 50) {
            $score += 10;
        }

        // Low friction bonus
        if ($contact->friction_events_count < 3) {
            $score += 10;
        }

        // Recent activity bonus
        if ($contact->last_session_at && $contact->last_session_at->isAfter(now()->subDays(7))) {
            $score += 10;
        }

        return min(100, $score);
    }

    private function calculateEngagementScore(Contact $contact): int
    {
        $score = 0;

        // Email opens
        $score += min(30, $contact->email_opens * 2);

        // Email clicks
        $score += min(30, $contact->email_clicks * 5);

        // Recent email engagement bonus
        if ($contact->last_email_opened_at && $contact->last_email_opened_at->isAfter(now()->subDays(7))) {
            $score += 20;
        }

        // Activities
        $score += min(20, $contact->activities_count * 2);

        return min(100, $score);
    }

    private function calculateIntentScore(Contact $contact): int
    {
        $score = 0;

        // High intent score from behavior system
        if ($contact->avg_intent_score >= 75) {
            $score += 50;
        } elseif ($contact->avg_intent_score >= 50) {
            $score += 30;
        } elseif ($contact->avg_intent_score >= 25) {
            $score += 15;
        }

        // Has deals (shows buying intent)
        if ($contact->deals_count > 0) {
            $score += 30;
        }

        // Recent activity
        if ($contact->last_activity_at && $contact->last_activity_at->isAfter(now()->subDays(3))) {
            $score += 20;
        }

        return min(100, $score);
    }

    private function calculateFitScore(Contact $contact): int
    {
        $score = 50; // Base score

        // Has complete profile
        if ($contact->job_title && $contact->phone && $contact->company_id) {
            $score += 25;
        }

        // Is verified
        if ($contact->is_verified) {
            $score += 15;
        }

        // VIP status
        if ($contact->is_vip) {
            $score += 10;
        }

        return min(100, $score);
    }

    public function applyScoreDecay(Contact $contact): void
    {
        $daysSinceActivity = $contact->last_activity_at 
            ? $contact->last_activity_at->diffInDays(now())
            : 999;

        $penalty = 0;

        if ($daysSinceActivity >= 90) {
            $penalty = 20;
        } elseif ($daysSinceActivity >= 60) {
            $penalty = 10;
        } elseif ($daysSinceActivity >= 30) {
            $penalty = 5;
        }

        if ($contact->is_unsubscribed) {
            $penalty += 30;
        }

        if ($penalty > 0) {
            $this->updateScore($contact, -$penalty, 'Score decay due to inactivity');
        }
    }

    public function updateScore(Contact $contact, int $change, string $reason, array $factors = []): void
    {
        $previousScore = $contact->lead_score;
        $newScore = min(100, max(0, $previousScore + $change));

        if ($previousScore !== $newScore) {
            LeadScoreLog::create([
                'id' => Str::uuid(),
                'contact_id' => $contact->id,
                'previous_score' => $previousScore,
                'new_score' => $newScore,
                'change' => $newScore - $previousScore,
                'reason' => $reason,
                'contributing_factors' => $factors,
            ]);

            $contact->update(['lead_score' => $newScore]);
        }
    }

    public function recalculateScore(Contact $contact): void
    {
        $newScore = $this->calculateLeadScore($contact);
        $previousScore = $contact->lead_score;

        if ($previousScore !== $newScore) {
            LeadScoreLog::create([
                'id' => Str::uuid(),
                'contact_id' => $contact->id,
                'previous_score' => $previousScore,
                'new_score' => $newScore,
                'change' => $newScore - $previousScore,
                'reason' => 'Score recalculation',
                'contributing_factors' => [],
            ]);

            $contact->update(['lead_score' => $newScore]);
        }
    }
}
