<?php

namespace App\Services;

use App\Models\BehaviorSession;
use App\Models\FrictionPoint;
use App\Models\IntentSignal;
use Illuminate\Support\Str;

class BehaviorClassifierService
{
    public function detectFrictionPoints(BehaviorSession $session): void
    {
        $events = $session->events;
        
        // Detect rage clicks
        $rageClicks = $events->where('event_type', 'rage_click');
        foreach ($rageClicks as $event) {
            $this->createOrUpdateFrictionPoint($session, [
                'friction_type' => 'rage_click',
                'severity' => $this->getRageClickSeverity($event->event_metadata['click_count'] ?? 3),
                'page_url' => $event->page_url,
                'element' => $event->element,
                'element_selector' => $event->element_selector,
                'description' => "User rage-clicked {$event->event_metadata['click_count']} times",
                'device_type' => $session->device_type,
                'timestamp' => $event->timestamp,
            ]);
        }
        
        // Detect form abandonments
        $formAbandons = $events->where('event_type', 'form_abandon');
        foreach ($formAbandons as $event) {
            $this->createOrUpdateFrictionPoint($session, [
                'friction_type' => 'form_abandon',
                'severity' => $this->getFormAbandonSeverity($event->event_metadata),
                'page_url' => $event->page_url,
                'element' => $event->element,
                'element_selector' => $event->element_selector,
                'description' => "Form abandoned at {$event->event_metadata['abandonment_point']}",
                'device_type' => $session->device_type,
                'timestamp' => $event->timestamp,
            ]);
        }
        
        // Detect speed scrolls
        $speedScrolls = $events->where('event_type', 'speed_scroll');
        if ($speedScrolls->count() >= 2) {
            $this->createOrUpdateFrictionPoint($session, [
                'friction_type' => 'speed_scroll',
                'severity' => 'moderate',
                'page_url' => $speedScrolls->first()->page_url,
                'description' => "User speed-scrolled {$speedScrolls->count()} times",
                'device_type' => $session->device_type,
                'timestamp' => $speedScrolls->first()->timestamp,
            ]);
        }
    }
    
    public function detectIntentSignals(BehaviorSession $session): void
    {
        $events = $session->events;
        
        // Detect CTA hover intent
        $ctaHovers = $events->where('event_type', 'hover_intent')
            ->filter(fn($e) => str_contains($e->element_selector ?? '', 'cta') || str_contains($e->element_selector ?? '', 'btn'));
        
        foreach ($ctaHovers as $event) {
            IntentSignal::create([
                'id' => Str::uuid(),
                'session_id' => $session->id,
                'user_id' => $session->user_id,
                'signal_type' => 'cta_hover',
                'intent_strength' => $event->event_metadata['intent_strength'] ?? 'moderate',
                'element' => $event->element,
                'page_url' => $event->page_url,
                'timestamp' => $event->timestamp,
            ]);
        }
        
        // Detect form start intent
        $formStarts = $events->where('event_type', 'form_focus')->unique('element_selector');
        foreach ($formStarts as $event) {
            IntentSignal::create([
                'id' => Str::uuid(),
                'session_id' => $session->id,
                'user_id' => $session->user_id,
                'signal_type' => 'form_start',
                'intent_strength' => 'strong',
                'element' => $event->element,
                'page_url' => $event->page_url,
                'timestamp' => $event->timestamp,
            ]);
        }
    }
    
    private function createOrUpdateFrictionPoint(BehaviorSession $session, array $data): void
    {
        $existing = FrictionPoint::where('session_id', $session->id)
            ->where('friction_type', $data['friction_type'])
            ->where('page_url', $data['page_url'])
            ->where('element_selector', $data['element_selector'] ?? '')
            ->first();
        
        if ($existing) {
            $existing->update([
                'event_count' => $existing->event_count + 1,
                'last_occurred_at' => $data['timestamp'],
            ]);
        } else {
            FrictionPoint::create([
                'id' => Str::uuid(),
                'session_id' => $session->id,
                'friction_type' => $data['friction_type'],
                'severity' => $data['severity'],
                'page_url' => $data['page_url'],
                'element' => $data['element'] ?? null,
                'element_selector' => $data['element_selector'] ?? null,
                'description' => $data['description'] ?? null,
                'device_type' => $data['device_type'],
                'first_occurred_at' => $data['timestamp'],
                'last_occurred_at' => $data['timestamp'],
            ]);
        }
    }
    
    private function getRageClickSeverity(int $clickCount): string
    {
        if ($clickCount >= 8) return 'severe';
        if ($clickCount >= 5) return 'moderate';
        return 'mild';
    }
    
    private function getFormAbandonSeverity(?array $metadata): string
    {
        if (!$metadata) return 'moderate';
        
        $fieldsTotal = $metadata['fields_total'] ?? 1;
        $fieldsFilled = $metadata['fields_filled'] ?? 0;
        $completionRate = $fieldsTotal > 0 ? ($fieldsFilled / $fieldsTotal) * 100 : 0;
        
        if ($completionRate >= 76) return 'critical'; // Late abandon
        if ($completionRate >= 26) return 'moderate'; // Mid abandon
        return 'mild'; // Early abandon
    }
}
