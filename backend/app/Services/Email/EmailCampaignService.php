<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Models\EmailCampaign;
use App\Models\EmailTemplate;
use App\Models\User;
use App\Models\Analytics\AnalyticsSegment;
use App\Services\EmailService;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Carbon\Carbon;

/**
 * Email Campaign Service
 * RevolutionEmailTemplates-L8-System
 * 
 * Manages email campaign creation, scheduling, and sending
 */
class EmailCampaignService
{
    public function __construct(
        private EmailService $emailService,
        private EmailTemplateRenderService $renderService
    ) {}

    /**
     * Create new campaign
     */
    public function createCampaign(array $data): EmailCampaign
    {
        return DB::transaction(function () use ($data) {
            $campaign = EmailCampaign::create([
                'name' => $data['name'],
                'template_id' => $data['template_id'],
                'segment_id' => $data['segment_id'] ?? null,
                'subject' => $data['subject'],
                'from_name' => $data['from_name'],
                'from_email' => $data['from_email'],
                'reply_to' => $data['reply_to'] ?? null,
                'scheduled_at' => isset($data['scheduled_at']) ? Carbon::parse($data['scheduled_at']) : null,
                'status' => $data['status'] ?? 'draft',
                'ab_test_config' => $data['ab_test_config'] ?? null,
                'created_by' => auth()->id() ?? 1,
            ]);

            // Calculate total recipients
            $recipients = $this->getRecipients($campaign);
            $campaign->update(['total_recipients' => $recipients->count()]);

            Log::info('[EmailCampaignService] Campaign created', [
                'campaign_id' => $campaign->id,
                'name' => $campaign->name,
                'total_recipients' => $campaign->total_recipients,
            ]);

            return $campaign;
        });
    }

    /**
     * Update campaign
     */
    public function updateCampaign(EmailCampaign $campaign, array $data): EmailCampaign
    {
        // Only allow updates if campaign is draft or scheduled
        if (!in_array($campaign->status, ['draft', 'scheduled'])) {
            throw new \Exception('Cannot update campaign that is sending or has been sent');
        }

        $campaign->update($data);

        // Recalculate recipients if segment changed
        if (isset($data['segment_id'])) {
            $recipients = $this->getRecipients($campaign);
            $campaign->update(['total_recipients' => $recipients->count()]);
        }

        return $campaign->fresh();
    }

    /**
     * Schedule campaign
     */
    public function scheduleCampaign(EmailCampaign $campaign, Carbon $scheduledAt): EmailCampaign
    {
        if (!$campaign->canBeSent()) {
            throw new \Exception('Campaign cannot be sent');
        }

        $campaign->update([
            'scheduled_at' => $scheduledAt,
            'status' => 'scheduled',
        ]);

        Log::info('[EmailCampaignService] Campaign scheduled', [
            'campaign_id' => $campaign->id,
            'scheduled_at' => $scheduledAt->toDateTimeString(),
        ]);

        return $campaign;
    }

    /**
     * Send campaign immediately
     */
    public function sendCampaign(EmailCampaign $campaign): void
    {
        if (!$campaign->canBeSent()) {
            throw new \Exception('Campaign cannot be sent');
        }

        $campaign->markAsSending();

        try {
            $recipients = $this->getRecipients($campaign);
            $template = $campaign->template;

            foreach ($recipients as $recipient) {
                $this->sendToRecipient($campaign, $template, $recipient);
            }

            $campaign->markAsSent();

            Log::info('[EmailCampaignService] Campaign sent successfully', [
                'campaign_id' => $campaign->id,
                'recipients' => $campaign->sent_count,
            ]);

        } catch (\Exception $e) {
            $campaign->update(['status' => 'failed']);
            
            Log::error('[EmailCampaignService] Campaign send failed', [
                'campaign_id' => $campaign->id,
                'error' => $e->getMessage(),
            ]);

            throw $e;
        }
    }

    /**
     * Send to individual recipient
     */
    private function sendToRecipient(EmailCampaign $campaign, EmailTemplate $template, User $recipient): void
    {
        try {
            // Prepare context data
            $context = [
                'user' => [
                    'name' => $recipient->name,
                    'first_name' => $recipient->first_name ?? $recipient->name,
                    'email' => $recipient->email,
                ],
                'campaign' => [
                    'name' => $campaign->name,
                ],
            ];

            // Render template
            $rendered = $this->renderService->render($template, $context);

            // Send email
            $this->emailService->send(
                $recipient->email,
                $rendered['subject'],
                $rendered['html'],
                [
                    'type' => 'marketing',
                    'campaign_id' => $campaign->id,
                    'template_id' => $template->id,
                    'from_name' => $campaign->from_name,
                    'from_email' => $campaign->from_email,
                    'reply_to' => $campaign->reply_to,
                ]
            );

            $campaign->incrementSent();

        } catch (\Exception $e) {
            Log::error('[EmailCampaignService] Failed to send to recipient', [
                'campaign_id' => $campaign->id,
                'recipient' => $recipient->email,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Get campaign recipients
     */
    private function getRecipients(EmailCampaign $campaign): Collection
    {
        $query = User::query()->where('is_active', true);

        // Apply segment filters if specified
        if ($campaign->segment_id) {
            $segment = AnalyticsSegment::find($campaign->segment_id);

            if ($segment) {
                if ($segment->segment_type === AnalyticsSegment::TYPE_STATIC) {
                    // Static segment: get users from membership table
                    $segmentUserIds = $segment->users()->pluck('users.id');
                    $query->whereIn('id', $segmentUserIds);
                } else {
                    // Dynamic segment: apply rules
                    $query = $this->applySegmentRulesToQuery($query, $segment);
                }

                Log::info('[EmailCampaignService] Applied segment filter', [
                    'campaign_id' => $campaign->id,
                    'segment_id' => $segment->id,
                    'segment_name' => $segment->name,
                ]);
            }
        }

        return $query->get();
    }

    /**
     * Apply segment rules to a query builder
     */
    private function applySegmentRulesToQuery($query, AnalyticsSegment $segment)
    {
        $rules = $segment->rules ?? [];

        if (empty($rules)) {
            return $query;
        }

        $isAndOperator = $segment->rule_operator === AnalyticsSegment::OP_AND;

        foreach ($rules as $index => $rule) {
            $field = $rule['field'] ?? null;
            $operator = $rule['operator'] ?? 'eq';
            $value = $rule['value'] ?? null;

            if (!$field) {
                continue;
            }

            $callback = function ($q) use ($field, $operator, $value) {
                $this->applyRuleCondition($q, $field, $operator, $value);
            };

            if ($isAndOperator || $index === 0) {
                $query->where($callback);
            } else {
                $query->orWhere($callback);
            }
        }

        return $query;
    }

    /**
     * Apply a single rule condition to query
     */
    private function applyRuleCondition($query, string $field, string $operator, mixed $value): void
    {
        match ($operator) {
            'eq' => $query->where($field, '=', $value),
            'neq' => $query->where($field, '!=', $value),
            'gt' => $query->where($field, '>', $value),
            'gte' => $query->where($field, '>=', $value),
            'lt' => $query->where($field, '<', $value),
            'lte' => $query->where($field, '<=', $value),
            'in' => $query->whereIn($field, (array) $value),
            'not_in' => $query->whereNotIn($field, (array) $value),
            'contains' => $query->where($field, 'LIKE', "%{$value}%"),
            'starts_with' => $query->where($field, 'LIKE', "{$value}%"),
            'is_null' => $query->whereNull($field),
            'is_not_null' => $query->whereNotNull($field),
            default => $query->where($field, '=', $value),
        };
    }

    /**
     * Cancel scheduled campaign
     */
    public function cancelCampaign(EmailCampaign $campaign): EmailCampaign
    {
        if ($campaign->status !== 'scheduled') {
            throw new \Exception('Only scheduled campaigns can be cancelled');
        }

        $campaign->update(['status' => 'cancelled']);

        Log::info('[EmailCampaignService] Campaign cancelled', [
            'campaign_id' => $campaign->id,
        ]);

        return $campaign;
    }

    /**
     * Get campaign analytics
     */
    public function getCampaignAnalytics(EmailCampaign $campaign): array
    {
        return [
            'sent_count' => $campaign->sent_count,
            'opened_count' => $campaign->opened_count,
            'clicked_count' => $campaign->clicked_count,
            'bounced_count' => $campaign->bounced_count,
            'unsubscribed_count' => $campaign->unsubscribed_count,
            'open_rate' => $campaign->open_rate,
            'click_rate' => $campaign->click_rate,
            'bounce_rate' => $campaign->bounce_rate,
            'unsubscribe_rate' => $campaign->unsubscribe_rate,
            'progress_percentage' => $campaign->progress_percentage,
        ];
    }

    /**
     * Duplicate campaign
     */
    public function duplicateCampaign(EmailCampaign $campaign): EmailCampaign
    {
        $newCampaign = $campaign->replicate();
        $newCampaign->name = $campaign->name . ' (Copy)';
        $newCampaign->status = 'draft';
        $newCampaign->scheduled_at = null;
        $newCampaign->sent_at = null;
        $newCampaign->sent_count = 0;
        $newCampaign->opened_count = 0;
        $newCampaign->clicked_count = 0;
        $newCampaign->bounced_count = 0;
        $newCampaign->unsubscribed_count = 0;
        $newCampaign->created_by = auth()->id() ?? 1;
        $newCampaign->save();

        return $newCampaign;
    }
}
