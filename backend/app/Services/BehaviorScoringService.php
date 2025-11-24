<?php

namespace App\Services;

use App\Models\BehaviorSession;
use App\Models\BehaviorEvent;

class BehaviorScoringService
{
    public function calculateEngagementScore(BehaviorSession $session): float
    {
        $events = $session->events;
        
        $scrollDepth = $this->getMaxScrollDepth($events);
        $timeOnPage = $session->duration_seconds;
        $interactionCount = $events->whereIn('event_type', ['click', 'form_focus', 'hover_intent'])->count();
        $contentConsumption = $events->whereIn('event_type', ['copy_text', 'video_play', 'scroll_pause'])->count();
        
        $weights = [
            'scroll_depth' => 0.25,
            'time_on_page' => 0.30,
            'interaction' => 0.30,
            'content' => 0.15,
        ];
        
        $scrollScore = $scrollDepth;
        $timeScore = min(($timeOnPage / 120) * 100, 100); // 2 min = 100%
        $interactionScore = min($interactionCount * 10, 100);
        $contentScore = min($contentConsumption * 20, 100);
        
        return round(
            $scrollScore * $weights['scroll_depth'] +
            $timeScore * $weights['time_on_page'] +
            $interactionScore * $weights['interaction'] +
            $contentScore * $weights['content'],
            2
        );
    }
    
    public function calculateIntentScore(BehaviorSession $session): float
    {
        $events = $session->events;
        
        $ctaInteractions = $events->whereIn('event_type', ['cta_click', 'cta_hesitation'])->count();
        $hoverIntents = $events->where('event_type', 'hover_intent')->count();
        $formEngagements = $events->whereIn('event_type', ['form_focus', 'form_input'])->count();
        $goalOrientedActions = $events->where('event_type', 'navigation_click')->count();
        
        $weights = [
            'cta' => 0.40,
            'hover' => 0.25,
            'form' => 0.20,
            'navigation' => 0.15,
        ];
        
        $ctaScore = min($ctaInteractions * 25, 100);
        $hoverScore = min($hoverIntents * 20, 100);
        $formScore = min($formEngagements * 15, 100);
        $navScore = min($goalOrientedActions * 10, 100);
        
        return round(
            $ctaScore * $weights['cta'] +
            $hoverScore * $weights['hover'] +
            $formScore * $weights['form'] +
            $navScore * $weights['navigation'],
            2
        );
    }
    
    public function calculateFrictionScore(BehaviorSession $session): float
    {
        $events = $session->events;
        
        $rageClicks = $events->where('event_type', 'rage_click')->count();
        $formAbandonments = $events->where('event_type', 'form_abandon')->count();
        $deadClicks = $events->where('event_type', 'dead_click')->count();
        $speedScrolls = $events->where('event_type', 'speed_scroll')->count();
        $backtracks = $events->where('event_type', 'scroll_backtrack')->count();
        $errors = $events->where('event_type', 'form_error')->count();
        
        $penalties = [
            'rage_click' => 30,
            'form_abandon' => 25,
            'dead_click' => 15,
            'speed_scroll' => 10,
            'backtrack' => 10,
            'error' => 10,
        ];
        
        $rageScore = min($rageClicks * 10, 100) * ($penalties['rage_click'] / 100);
        $abandonScore = min($formAbandonments * 25, 100) * ($penalties['form_abandon'] / 100);
        $deadScore = min($deadClicks * 15, 100) * ($penalties['dead_click'] / 100);
        $speedScore = min($speedScrolls * 5, 100) * ($penalties['speed_scroll'] / 100);
        $backtrackScore = min($backtracks * 3, 100) * ($penalties['backtrack'] / 100);
        $errorScore = min($errors * 5, 100) * ($penalties['error'] / 100);
        
        return round(
            min($rageScore + $abandonScore + $deadScore + $speedScore + $backtrackScore + $errorScore, 100),
            2
        );
    }
    
    public function calculateChurnRisk(BehaviorSession $session, float $engagementScore, float $frictionScore): float
    {
        $events = $session->events;
        
        $exitIntentDetected = $events->where('event_type', 'exit_intent')->count() > 0;
        $abandonmentSignals = $events->whereIn('event_type', ['form_abandon', 'cart_abandon'])->count();
        
        $idleTime = $events->where('event_type', 'idle_start')->sum(function ($event) {
            return $event->event_metadata['duration'] ?? 0;
        });
        $idleTimeRatio = $session->duration_seconds > 0 ? ($idleTime / $session->duration_seconds) : 0;
        
        $weights = [
            'exit_intent' => 0.30,
            'low_engagement' => 0.25,
            'high_friction' => 0.20,
            'abandonment' => 0.15,
            'idle' => 0.10,
        ];
        
        $exitScore = $exitIntentDetected ? 100 : 0;
        $lowEngagementScore = 100 - $engagementScore;
        $abandonmentScore = min($abandonmentSignals * 20, 100);
        $idleScore = $idleTimeRatio * 100;
        
        return round(
            $exitScore * $weights['exit_intent'] +
            $lowEngagementScore * $weights['low_engagement'] +
            $frictionScore * $weights['high_friction'] +
            $abandonmentScore * $weights['abandonment'] +
            $idleScore * $weights['idle'],
            2
        );
    }
    
    public function updateSessionScores(BehaviorSession $session): void
    {
        $engagementScore = $this->calculateEngagementScore($session);
        $intentScore = $this->calculateIntentScore($session);
        $frictionScore = $this->calculateFrictionScore($session);
        $churnRiskScore = $this->calculateChurnRisk($session, $engagementScore, $frictionScore);
        
        $session->update([
            'engagement_score' => $engagementScore,
            'intent_score' => $intentScore,
            'friction_score' => $frictionScore,
            'churn_risk_score' => $churnRiskScore,
        ]);
    }
    
    private function getMaxScrollDepth($events): float
    {
        $scrollEvents = $events->where('event_type', 'scroll_depth');
        if ($scrollEvents->isEmpty()) {
            return 0;
        }
        
        return $scrollEvents->max('event_value') ?? 0;
    }
}
