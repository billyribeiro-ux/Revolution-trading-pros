<?php

namespace App\Services;

use App\Models\BehaviorSession;
use App\Models\BehaviorEvent;
use App\Models\FrictionPoint;
use App\Models\IntentSignal;
use Illuminate\Support\Str;

class BehaviorProcessorService
{
    public function __construct(
        private BehaviorScoringService $scoringService,
        private BehaviorClassifierService $classifierService
    ) {}
    
    public function processEventBatch(array $batch): BehaviorSession
    {
        $sessionId = $batch['session_id'];
        $visitorId = $batch['visitor_id'];
        $userId = $batch['user_id'] ?? null;
        $events = $batch['events'];
        
        // Find or create session
        $session = BehaviorSession::firstOrCreate(
            ['id' => $sessionId],
            [
                'visitor_id' => $visitorId,
                'user_id' => $userId,
                'session_fingerprint' => Str::random(32),
                'started_at' => now(),
            ]
        );
        
        // Process each event
        foreach ($events as $eventData) {
            $this->processEvent($session, $eventData);
        }
        
        // Update session metadata
        $this->updateSessionMetadata($session, $events);
        
        // Recalculate scores
        $this->scoringService->updateSessionScores($session);
        
        // Detect friction points
        $this->classifierService->detectFrictionPoints($session);
        
        // Detect intent signals
        $this->classifierService->detectIntentSignals($session);
        
        return $session->fresh();
    }
    
    private function processEvent(BehaviorSession $session, array $eventData): BehaviorEvent
    {
        $event = BehaviorEvent::create([
            'id' => Str::uuid(),
            'session_id' => $session->id,
            'event_type' => $eventData['event_type'],
            'timestamp' => now()->setTimestamp($eventData['timestamp'] / 1000),
            'page_url' => $eventData['page_url'],
            'element' => $eventData['element'] ?? null,
            'element_selector' => $eventData['element_selector'] ?? null,
            'coordinates_x' => $eventData['coordinates']['x'] ?? null,
            'coordinates_y' => $eventData['coordinates']['y'] ?? null,
            'event_value' => $eventData['event_value'] ?? null,
            'event_metadata' => $eventData['event_metadata'] ?? null,
            'sequence_number' => $session->event_count + 1,
            'time_since_session_start' => $eventData['timestamp'] - ($session->started_at->timestamp * 1000),
            'time_since_last_event' => 0, // Calculate if needed
        ]);
        
        $session->increment('event_count');
        
        // Update session flags
        $this->updateSessionFlags($session, $eventData['event_type']);
        
        return $event;
    }
    
    private function updateSessionMetadata(BehaviorSession $session, array $events): void
    {
        $pageViewEvent = collect($events)->firstWhere('event_type', 'page_view');
        
        if ($pageViewEvent && isset($pageViewEvent['event_metadata'])) {
            $metadata = $pageViewEvent['event_metadata'];
            
            $session->update([
                'device_type' => $this->getDeviceType($metadata['viewport']['width'] ?? 0),
                'browser' => $this->getBrowser(),
                'viewport_width' => $metadata['viewport']['width'] ?? null,
                'viewport_height' => $metadata['viewport']['height'] ?? null,
                'entry_url' => $pageViewEvent['page_url'],
                'referrer' => $metadata['referrer'] ?? null,
                'utm_source' => $metadata['utm_source'] ?? null,
                'utm_campaign' => $metadata['utm_campaign'] ?? null,
                'utm_medium' => $metadata['utm_medium'] ?? null,
            ]);
        }
        
        // Update duration
        $lastEvent = collect($events)->last();
        if ($lastEvent) {
            $duration = ($lastEvent['timestamp'] - ($session->started_at->timestamp * 1000)) / 1000;
            $session->update(['duration_seconds' => (int) $duration]);
        }
    }
    
    private function updateSessionFlags(BehaviorSession $session, string $eventType): void
    {
        $flagMap = [
            'rage_click' => 'has_rage_clicks',
            'form_abandon' => 'has_form_abandonment',
            'speed_scroll' => 'has_speed_scrolls',
            'exit_intent' => 'has_exit_intent',
            'dead_click' => 'has_dead_clicks',
        ];
        
        if (isset($flagMap[$eventType])) {
            $session->update([$flagMap[$eventType] => true]);
        }
    }
    
    private function getDeviceType(int $width): string
    {
        if ($width < 768) return 'mobile';
        if ($width < 1024) return 'tablet';
        return 'desktop';
    }
    
    private function getBrowser(): string
    {
        $userAgent = request()->userAgent();
        
        if (str_contains($userAgent, 'Firefox')) return 'Firefox';
        if (str_contains($userAgent, 'Chrome')) return 'Chrome';
        if (str_contains($userAgent, 'Safari')) return 'Safari';
        if (str_contains($userAgent, 'Edge')) return 'Edge';
        
        return 'Unknown';
    }
}
