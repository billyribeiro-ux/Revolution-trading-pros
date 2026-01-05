<?php

namespace App\Services;

use App\Models\Contact;

class HealthScoringService
{
    public function calculateHealthScore(Contact $contact): int
    {
        // Only calculate for customers
        if ($contact->status !== 'customer') {
            return 0;
        }

        $engagementHealth = $this->calculateEngagementHealth($contact);
        $productUsageHealth = $this->calculateProductUsageHealth($contact);
        $supportHealth = $this->calculateSupportHealth($contact);
        $paymentHealth = $this->calculatePaymentHealth($contact);
        $relationshipHealth = $this->calculateRelationshipHealth($contact);

        $totalScore = (
            $engagementHealth * 0.30 +
            $productUsageHealth * 0.25 +
            $supportHealth * 0.20 +
            $paymentHealth * 0.15 +
            $relationshipHealth * 0.10
        );

        return min(100, max(0, (int) round($totalScore)));
    }

    private function calculateEngagementHealth(Contact $contact): int
    {
        $score = 0;

        // Email engagement rate
        $totalEmails = $contact->email_opens + $contact->email_clicks;
        if ($totalEmails > 0) {
            $engagementRate = ($contact->email_clicks / $totalEmails) * 100;
            $score += min(30, $engagementRate);
        }

        // Login frequency (from behavior system)
        if ($contact->last_session_at) {
            $daysSinceLogin = $contact->last_session_at->diffInDays(now());
            if ($daysSinceLogin <= 7) {
                $score += 25;
            } elseif ($daysSinceLogin <= 14) {
                $score += 15;
            } elseif ($daysSinceLogin <= 30) {
                $score += 5;
            } else {
                $score -= 40; // Red flag
            }
        }

        // Behavior engagement score
        $score += min(25, $contact->avg_engagement_score / 4);

        // Recent activity
        if ($contact->last_activity_at && $contact->last_activity_at->isAfter(now()->subDays(7))) {
            $score += 20;
        }

        return min(100, max(0, $score));
    }

    private function calculateProductUsageHealth(Contact $contact): int
    {
        $score = 50; // Base score

        // Active subscription
        if (in_array($contact->subscription_status, ['active', 'trial'])) {
            $score += 40;
        } elseif ($contact->subscription_status === 'paused') {
            $score += 10;
        } else {
            $score -= 50;
        }

        // Usage metrics from behavior system
        if ($contact->total_sessions >= 10) {
            $score += 10;
        }

        return min(100, max(0, $score));
    }

    private function calculateSupportHealth(Contact $contact): int
    {
        // This would integrate with a support ticket system
        // For now, use friction events as proxy
        $score = 100;

        if ($contact->friction_events_count >= 6) {
            $score = 40;
        } elseif ($contact->friction_events_count >= 3) {
            $score = 60;
        } elseif ($contact->friction_events_count >= 1) {
            $score = 80;
        }

        return $score;
    }

    private function calculatePaymentHealth(Contact $contact): int
    {
        $score = 0;

        // Payment status from subscription
        if ($contact->subscription_status === 'active') {
            $score = 100;
        } elseif ($contact->subscription_status === 'trial') {
            $score = 80;
        } elseif ($contact->subscription_status === 'paused') {
            $score = 50;
        } elseif (in_array($contact->subscription_status, ['cancelled', 'expired'])) {
            $score = 0;
        }

        return $score;
    }

    private function calculateRelationshipHealth(Contact $contact): int
    {
        $score = 50; // Base score

        // Has assigned owner
        if ($contact->owner_id) {
            $score += 20;
        }

        // Recent contact from owner
        if ($contact->last_contacted_at && $contact->last_contacted_at->isAfter(now()->subDays(30))) {
            $score += 30;
        }

        return min(100, $score);
    }

    public function recalculateHealthScore(Contact $contact): void
    {
        $newScore = $this->calculateHealthScore($contact);
        $contact->update(['health_score' => $newScore]);
    }
}
