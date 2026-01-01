<?php

namespace App\Services;

use App\Models\Contact;
use App\Models\CrmActivity;
use Illuminate\Support\Collection;

class ContactTimelineService
{
    public function getTimeline(Contact $contact, int $limit = 50): Collection
    {
        $timeline = collect();

        // Get CRM activities
        $activities = $contact->activities()
            ->orderBy('occurred_at', 'desc')
            ->limit($limit)
            ->get();

        foreach ($activities as $activity) {
            $timeline->push($this->formatActivity($activity));
        }

        // Get behavior events if linked
        if ($contact->user_id) {
            $behaviorEvents = $this->getBehaviorEvents($contact);
            $timeline = $timeline->merge($behaviorEvents);
        }

        // Get deal events
        $dealEvents = $this->getDealEvents($contact);
        $timeline = $timeline->merge($dealEvents);

        // Get subscription events
        $subscriptionEvents = $this->getSubscriptionEvents($contact);
        $timeline = $timeline->merge($subscriptionEvents);

        // Sort by occurred_at and limit
        return $timeline->sortByDesc('occurred_at')->take($limit)->values();
    }

    private function formatActivity(CrmActivity $activity): array
    {
        return [
            'id' => $activity->id,
            'type' => $activity->type,
            'category' => $this->getCategoryForType($activity->type),
            'title' => $activity->title ?? $this->getTitleForType($activity->type),
            'description' => $activity->description,
            'metadata' => $activity->metadata,
            'occurred_at' => $activity->occurred_at,
            'created_by' => $activity->createdBy ? [
                'id' => $activity->createdBy->id,
                'name' => $activity->createdBy->name,
            ] : null,
        ];
    }

    private function getBehaviorEvents(Contact $contact): Collection
    {
        // This would integrate with BehaviorSession model
        // For now, return empty collection
        return collect();
    }

    private function getDealEvents(Contact $contact): Collection
    {
        $events = collect();

        // Eager load deals with pipeline to prevent N+1 queries
        $deals = $contact->deals()->with(['pipeline', 'stage'])->get();

        foreach ($deals as $deal) {
            $events->push([
                'id' => $deal->id,
                'type' => 'deal_created',
                'category' => 'deal',
                'title' => "Deal created: {$deal->name}",
                'description' => "Amount: \${$deal->amount}",
                'metadata' => [
                    'deal_id' => $deal->id,
                    'amount' => $deal->amount,
                    'pipeline' => $deal->pipeline->name,
                ],
                'occurred_at' => $deal->created_at,
                'created_by' => null,
            ]);

            if ($deal->status === 'won') {
                $events->push([
                    'id' => $deal->id . '_won',
                    'type' => 'deal_won',
                    'category' => 'deal',
                    'title' => "Deal won: {$deal->name}",
                    'description' => "Won \${$deal->amount}",
                    'metadata' => ['deal_id' => $deal->id],
                    'occurred_at' => $deal->closed_at,
                    'created_by' => null,
                ]);
            }
        }

        return $events;
    }

    private function getSubscriptionEvents(Contact $contact): Collection
    {
        // This would integrate with Subscription model
        return collect();
    }

    private function getCategoryForType(string $type): string
    {
        $categoryMap = [
            'note' => 'activity',
            'email_sent' => 'email',
            'email_opened' => 'email',
            'email_clicked' => 'email',
            'call' => 'activity',
            'meeting' => 'activity',
            'task_completed' => 'activity',
            'form_submitted' => 'form',
            'popup_converted' => 'popup',
            'deal_created' => 'deal',
            'deal_stage_changed' => 'deal',
            'deal_won' => 'deal',
            'deal_lost' => 'deal',
            'subscription_started' => 'subscription',
            'subscription_cancelled' => 'subscription',
            'behavior_event' => 'behavior',
            'page_view' => 'behavior',
            'friction_detected' => 'behavior',
        ];

        return $categoryMap[$type] ?? 'system';
    }

    private function getTitleForType(string $type): string
    {
        $titleMap = [
            'note' => 'Note added',
            'email_sent' => 'Email sent',
            'email_opened' => 'Email opened',
            'email_clicked' => 'Email clicked',
            'call' => 'Call made',
            'meeting' => 'Meeting held',
            'task_completed' => 'Task completed',
            'form_submitted' => 'Form submitted',
            'popup_converted' => 'Popup converted',
            'deal_created' => 'Deal created',
            'deal_stage_changed' => 'Deal stage changed',
            'deal_won' => 'Deal won',
            'deal_lost' => 'Deal lost',
            'subscription_started' => 'Subscription started',
            'subscription_cancelled' => 'Subscription cancelled',
            'behavior_event' => 'Behavior event',
            'page_view' => 'Page viewed',
            'friction_detected' => 'Friction detected',
        ];

        return $titleMap[$type] ?? ucfirst(str_replace('_', ' ', $type));
    }
}
